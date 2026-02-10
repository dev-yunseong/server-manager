use async_trait::async_trait;
use crate::domain::config::{ClientConfig, ServerConfig};

#[async_trait]
pub trait ServerConfigUseCase {
    async fn add_server(&self, server_config: ServerConfig);
    async fn list_server(&self) -> Vec<ServerConfig>;
}

#[async_trait]
pub trait ClientConfigUseCase {
    async fn add_client(&self, client_config: ClientConfig);
    async fn list_client(&self) -> Vec<ClientConfig>;
}