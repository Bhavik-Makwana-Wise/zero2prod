
use crate::domain;
use domain::SubscriberEmail;
use reqwest::Client;

#[derive(Clone)]
pub struct EmailClient {
    pub sender: SubscriberEmail,
    pub http_client: Client,
    base_url: String,
}

impl EmailClient {

    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            base_url
        }
    }
    pub async fn send_email(&self,
                            recipient: SubscriberEmail,
                            subject: &str,
                            html_content: &str,
                            text_content: &str) -> Result<(), String> {
        todo!()
    }
}