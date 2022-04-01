use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;
    let configuration = get_configuration().expect("Failed to get configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres");
    let client = reqwest::Client::new();

    let body = "name=Johnny%20Bravo&email=Johnny_Bravo%40proton.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "Johnny_Bravo@proton.com");
    assert_eq!(saved.name, "Johnyn Bravo");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_valid_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=Oscar_Isaac%40@gmail.com", "missing the name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 bad request when payload was {}.",
            error_message
        );
    }
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    let configuration = get_configuration().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await.expect("");
    // retrieve OS assigned port
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener, db_pool).expect("Failed to bind address");
    // launch server as background task
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
