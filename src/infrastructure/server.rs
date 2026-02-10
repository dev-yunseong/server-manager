use async_trait::async_trait;
use log::{error, info};
use reqwest::Client;
use crate::application::server::{ServerManager, ServerRepository};
use crate::domain::server::Health;

struct HttpServerManager {
    server_repository: Box<dyn ServerRepository>,
    client: Client
}

impl HttpServerManager {
    pub fn new(server_repository: Box<dyn ServerRepository>) -> Self {
        Self {
            server_repository,
            client: Client::new()
        }
    }
}

#[async_trait]
impl ServerManager for HttpServerManager {
    
    async fn kill(&self, name: &str) -> bool {
        let server = match self.server_repository.find(name) {
            Some(s) => s,
            None => return false
        };
        
        let kill_url = match server.get_kill_url() {
            Some(value) => value,
            None => return false
        };

        let client = self.client.clone();

        if let Err(e) = client.get(kill_url).send().await {
            error!("[HttpWatchdog] Err: Kill request failed {}", e);
            false
        } else {
            info!("[HttpWatchdog] Info: Kill signal sent successfully");
            true
        }

    }

    async fn healthcheck(&self, name: &str) -> Health {

        let server = match self.server_repository.find(name) {
            Some(s) => s,
            None => return Health::Unknown
        };


        let health_check_url = match server.get_health_check_url() {
            Some(value) => value,
            None => return Health::Unknown
        };

        let response = self.client
            .get(health_check_url)
            .send()
            .await;

        match response {
            Ok(_) => Health::Running,
            Err(_) => Health::Dead
        }
    }
}
