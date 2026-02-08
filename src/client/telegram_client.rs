mod dto;

use anyhow::{anyhow, Result};
use crate::client::Client;
use rust_api_client::api::ApiClient;
use dto::SendMessageDto;
use log::{debug, error};
use crate::client::telegram_client::dto::{Message, TelegramResponse, Update};

pub struct TelegramClient {
    api_client: ApiClient,
    callback: Option<Box<dyn Fn(&str)>>
}

impl TelegramClient {
    pub fn new(token: &str) -> Self {
        Self {
            api_client: ApiClient::new(format!("https://api.telegram.org/bot{token}")),
            callback: None
        }
    }

    async fn get_update(&self) -> Result<Vec<Update>> {
        match self.api_client.get_json::<TelegramResponse<Vec<Update>>>("getUpdates", None).await {
            Ok(updates) => {

                debug!("[TelegramClient] Ok: Successfully get update");
                if !updates.ok {
                    return Err(anyhow!("[TelegramClient] status: {} {}", updates.error_code.unwrap(), updates.description.unwrap()));
                }

                Ok(updates.result)
            },
            Err(e) => {
                Err(anyhow!("[TelegramClient] Err: {}", e))
            }
        }
    }
}

impl Client for TelegramClient {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool {
        let response = self.api_client
            .post_json::<SendMessageDto, Message> (
                "sendMessage",
                &SendMessageDto::new(chat_id, data), None).await;

        if response.is_err() {
            error!("[Err]: {}", response.err().unwrap().to_string());
            return false
        }

        true
    }

    fn set_callback(&mut self, callback: impl Fn(&str) + 'static) {
        self.callback = Some(Box::new(callback))
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn get_update() {
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let telegram_client = TelegramClient::new(token.as_str());
        let response = telegram_client.get_update().await;

        println!("{:?}", response);
        assert!(response.is_ok());
        println!("{:?}", response.unwrap())
    }

    #[tokio::test]
    async fn send_message() {
        dotenv().ok();
        let token = env::var("TELEGRAM_TOKEN").unwrap();
        let telegram_client = TelegramClient::new(token.as_str());
         telegram_client.send_message(env::var("CHAT_ID").unwrap().as_str(), "test message").await;
    }
}
