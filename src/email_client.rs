
use crate::domain;
use domain::SubscriberEmail;

pub struct EmailClient {
    pub sender: SubscriberEmail,
}

impl EmailClient {

    pub async fn send_email(&self,
                            recipient: SubscriberEmail,
                            subject: &str,
                            html_content: &str,
                            text_content: &str) -> Result<(), String> {
        todo!()
    }
}