use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use tokio::sync::mpsc::Receiver;
use crate::application::client::MessageGateway;
use crate::application::event::dto::EventMessage;
use crate::application::worker::Worker;
use crate::domain::chat::{Chat, ChatList};
use crate::domain::config::EventSubscribeList;
use crate::domain::file_accessor::{FileAccessor};

#[derive(new)]
pub struct EventManager {
    rx: Receiver<EventMessage>,
    message_gateway: Arc<dyn MessageGateway>,
    chat_list_file_accessor: Arc<dyn FileAccessor<ChatList>>,
    subscribe_file_accessor: Arc<dyn FileAccessor<EventSubscribeList>>,
}

#[async_trait]
impl Worker for EventManager {
    async fn on_tick(&mut self) -> bool {
        if let Some(message) = self.rx.recv().await {
            let _ = self.handle(message).await;
            true
        } else {
            false
        }
    }

    fn get_name(&self) -> &str {
        "event_handler"
    }

    fn interval(&self) -> i32 {
        5
    }
}

impl EventManager {
    pub async fn handle(&self, event_message: EventMessage)
                        -> Result<(), Box<dyn Error + Send + Sync>> {
        let subscribe_list = self.subscribe_file_accessor.read().await?;
        let chat_ids = match subscribe_list
            .find_subscribe(event_message.event_name.as_str()) {
            Some(value) => &value.chat_ids,
            None => return Ok(())
        };
        let chats: Vec<Chat> = self.chat_list_file_accessor.read().await?
            .chats
            .into_iter()
            .filter(|chat| {chat_ids.contains(&chat.id)})
            .collect();
        for chat in chats {
            self.message_gateway.send_message(
                chat.client_name.as_str(),
                chat.identity.as_str(),
                event_message.text.as_str()
            ).await;
        }
        Ok(())
    }
}
