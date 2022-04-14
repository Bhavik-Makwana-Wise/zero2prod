use once_cell::sync::Lazy;
use reqwest::Url;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings, Settings};
use zero2prod::email_client::EmailClient;
use zero2prod::startup::{Application, get_connection_pool};
use zero2prod::telemetry::{get_subscriber, init_subscriber};


static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    if (std::env::var("TEST_LOG").is_ok()) {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::sink);
        init_subscriber(subscriber);
    }
});



pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };
    configure_database(&configuration.database).await;
    // let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    // let mut configuration = get_configuration().expect("Failed to read configuration");
    // configuration.database.database_name = Uuid::new_v4().to_string();
    // let db_pool = configure_database(&configuration.database).await;
    // let sender_email = configuration
    //     .email_client
    //     .sender()
    //     .expect("Invalid sender email address");
    // let base_url =
    //     Url::parse(configuration.email_client.base_url.as_str()).expect("Invalid base url");
    // let timeout = configuration.email_client.timeout();
    // let email_client = EmailClient::new(
    //     base_url,
    //     sender_email,
    //     configuration.email_client.authorization_token,
    //     timeout,
    // );
    // retrieve OS assigned port
    // let port = listener.local_addr().unwrap().port();
    // let address = format!("http://127.0.0.1:{}", port);

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    // let port = application.port();
    let address = format!("http://127.0.0.1:{}", application.port());
       // let server = zero2prod::startup::run(listener, db_pool.clone(), email_client)
       //  .expect("Failed to bind address");

    // launch server as background task
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // Migrate database
    let db_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to postgres");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to mgirate the database");
    db_pool
}
