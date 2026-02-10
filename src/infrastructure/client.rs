pub mod common;
pub mod telegram;

use std::collections::HashMap;
use std::os::unix::raw::time_t;
use derive_new::new;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
pub use common::*;
use crate::application::client::{ClientLoader, MessageGateway};
use crate::application::worker::Worker;
use crate::domain::client::Message;
use crate::infrastructure::{client, config};
use crate::infrastructure::worker::WorkerRunner;

pub struct MessageAdapter {

}

impl MessageGateway for MessageAdapter {
    async fn send_message(&self, client_name: &str, chat_id: &str, message: &str) {
        todo!()
    }
}

pub struct ClientManager {
    worker_runner: WorkerRunner,
    client_map: HashMap<String, Box<dyn Client>>
}

impl ClientManager {
    pub fn new() -> Self {
        Self {
            worker_runner: WorkerRunner::new(),
            client_map: HashMap::new()
        }
    }
}


impl ClientLoader for ClientManager {
    async fn load_clients(&mut self) {
        let clients = config::read().await.clients;
        let clients: Vec<Box<dyn Client>> = clients.into_iter()
            .map(|client_config| {client::from(client_config)})
            .filter(|option|{option.is_some()})
            .map(|client| { client.unwrap() })
            .collect();

        self.client_map.clear();

        for client in clients.into_iter() {
            self.client_map.insert(client.get_name().to_string(), client);
        }
    }

    async fn find(&self, name: &str) -> Option<&Box<dyn Client>> {
        self.client_map.get(name)
    }

    async fn run(&mut self) -> Receiver<Message> {
        let (tx, rx) = mpsc::channel(16);
        let mut clients: Vec<Box<dyn Client>> = self.client_map
            .values()
            .map(|c| dyn_clone::clone_box(&**c))
            .collect();

        for client in clients.iter_mut() {
            let tx = tx.clone();
            client.subscribe(tx);
        }

        let workers: Vec<Box<dyn Worker>> = clients
            .into_iter()
            .map(|c| c as Box<dyn Worker>)
            .collect();

        self.worker_runner.run_batch(workers);

        rx
    }
}