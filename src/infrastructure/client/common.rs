
use dyn_clone::{clone_trait_object, DynClone};
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use crate::application::worker::Worker;
use crate::domain::client::Message;
use crate::domain::config::ClientConfig;
use crate::infrastructure::client::telegram::TelegramClient;

#[async_trait]
pub trait Client : Worker + DynClone {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool;
    fn subscribe(&mut self, tx: Sender<Message>);
}

pub fn from(config: ClientConfig) -> Option<Box<dyn Client>> {
    match config.kind.as_str() {
        "telegram" => {
            let token = config.token?;
            Some(Box::new(TelegramClient::new(config.name, token)))
        }
        _ => None,
    }
}

clone_trait_object!(Client);