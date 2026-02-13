use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use derive_new::new;
use crate::application::config::ClientConfigUseCase;
use crate::domain::config::{ClientConfig, Config};
use crate::domain::file_accessor::FileAccessor;

#[derive(new)]
pub struct ClientConfigAdapter {
    config_file_accessor: Arc<dyn FileAccessor<Config> + Send + Sync>
}

#[async_trait]
impl ClientConfigUseCase for ClientConfigAdapter {
    async fn add_client(&self, client_config: ClientConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut config = self.config_file_accessor.read().await?;
        config.clients.push(client_config);
        self.config_file_accessor.write(&config).await?;
        Ok(())
    }

    async fn list_client(&self) -> Result<Vec<ClientConfig>, Box<dyn Error + Send + Sync>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.clients)
    }
}