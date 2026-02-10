use std::ptr::addr_of_mut;
use tokio::sync::mpsc::Receiver;
use crate::domain::client::Message;
use crate::infrastructure::client::Client;

pub trait MessageGateway {
    async fn send_message(&self, client_name: &str, chat_id: &str, message: &str);
}


pub trait ClientLoader {
    async fn load_clients(&mut self);
    async fn find(&self, name: &str) -> Option<&Box<dyn Client>>;
    async fn run(&mut self)-> Receiver<Message>;
    
}