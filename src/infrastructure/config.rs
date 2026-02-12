pub mod auth;

use std::error::Error;
use async_trait::async_trait;
use crate::application::config::{ClientConfigUseCase, EventConfigUseCase, ServerConfigUseCase};
use crate::domain::config::{ClientConfig, Config, EventConfig, ServerConfig};
use crate::infrastructure::common::file_accessor::{get_config_file_accessor, FileAccessor};

pub struct ClientConfigAdapter {
    config_file_accessor: FileAccessor<Config>
}

impl ClientConfigAdapter {
    pub fn new() -> Self {
        Self {
            config_file_accessor: get_config_file_accessor()
        }
    }
}

#[async_trait]
impl ClientConfigUseCase for ClientConfigAdapter {
    async fn add_client(&self, client_config: ClientConfig) -> Result<(), Box<dyn Error>> {
        let mut config = self.config_file_accessor.read().await?;
        config.clients.push(client_config);
        self.config_file_accessor.write(config).await?;
        Ok(())
    }

    async fn list_client(&self) -> Result<Vec<ClientConfig>, Box<dyn Error>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.clients)
    }
}

pub struct ServerConfigAdapter {
    config_file_accessor: FileAccessor<Config>
}

impl ServerConfigAdapter {
    pub fn new() -> Self {
        Self {
            config_file_accessor: get_config_file_accessor()
        }
    }
}

#[async_trait]
impl ServerConfigUseCase for ServerConfigAdapter {

    async fn add_server(&self, server_config: ServerConfig) -> Result<(), Box<dyn Error>> {
        let mut config = self.config_file_accessor.read().await?;
        config.servers.push(server_config);
        self.config_file_accessor.write(config).await?;
        Ok(())
    }

    async fn list_server(&self) -> Result<Vec<ServerConfig>, Box<dyn Error>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.servers)
    }
}

pub struct EventConfigAdapter {
    config_file_accessor: FileAccessor<Config>
}

impl EventConfigAdapter {
    pub fn new() -> Self {
        Self {
            config_file_accessor: get_config_file_accessor()
        }
    }
}

#[async_trait]
impl EventConfigUseCase for EventConfigAdapter {
    async fn add_event(&self, event_config: EventConfig) -> Result<(), Box<dyn Error>> {
        let mut config = self.config_file_accessor.read().await?;
        config.events.push(event_config);
        self.config_file_accessor.write(config).await?;
        Ok(())
    }

    async fn list_event(&self) -> Result<Vec<EventConfig>, Box<dyn Error>> {
        let config = self.config_file_accessor.read().await?;
        Ok(config.events)
    }

    async fn remove_event(&self, name: String) -> Result<(), Box<dyn Error>> {
        let mut config = self.config_file_accessor.read().await?;
        config.events.retain(|event| event.name != name);
        self.config_file_accessor.write(config).await?;
        Ok(())
    }
}
