use std::error::Error;
use async_trait::async_trait;
use crate::domain::config::{ClientConfig, EventConfig, ServerConfig};

#[async_trait]
pub trait AuthUseCase : Send + Sync {
    async fn set_password(&self, password: Option<String>) -> Result<(), Box<dyn Error>>;
    async fn validate_password(&mut self, password: String) -> bool;
    async fn register(&mut self, client_name: String, identity: String) -> Result<(), Box<dyn Error>>;
    async fn authenticate(&mut self, client_name: String, identity: String) -> bool;
    fn password_required(&self) -> bool;
}

#[async_trait]
pub trait ServerConfigUseCase {
    async fn add_server(&self, server_config: ServerConfig) -> Result<(), Box<dyn Error>>;
    async fn list_server(&self) -> Result<Vec<ServerConfig>, Box<dyn Error>>;
}

#[async_trait]
pub trait ClientConfigUseCase {
    async fn add_client(&self, client_config: ClientConfig) -> Result<(), Box<dyn Error>>;
    async fn list_client(&self) -> Result<Vec<ClientConfig>, Box<dyn Error>>;
}

#[async_trait]
pub trait EventConfigUseCase {
    async fn add_event(&self, event_config: EventConfig) -> Result<(), Box<dyn Error>>;
    async fn list_event(&self) -> Result<Vec<EventConfig>, Box<dyn Error>>;
    async fn remove_event(&self, name: String) -> Result<(), Box<dyn Error>>;
}
