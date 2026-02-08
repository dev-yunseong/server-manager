use async_trait::async_trait;
use crate::core::Worker;

mod telegram_client;

#[async_trait]
pub  trait Client : Worker {
    async fn send_message(&self, chat_id: &str, data: &str) -> bool;
    fn set_callback(&mut self, callback: impl Fn(&str, &str) + 'static + Send + Sync);
}