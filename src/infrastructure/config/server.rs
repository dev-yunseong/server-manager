use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use crate::application::config::ServerConfigUseCase;
use crate::domain::config::{Config, ServerConfig};
use crate::domain::file_accessor::FileAccessor;

#[derive(new)]
pub struct ServerConfigAdapter {
    config_file_accessor: Arc<dyn FileAccessor<Config> + Send + Sync>
}

#[async_trait]
impl ServerConfigUseCase for ServerConfigAdapter {

    async fn add_server(&self, server_config: ServerConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut config = self.config_file_accessor.read().await?;
        config.servers.push(server_config);
        self.config_file_accessor.write(&config).await?;
        Ok(())
    }

    async fn list_server(&self) -> Result<Vec<ServerConfig>, Box<dyn Error + Send + Sync>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.servers)
    }
}
