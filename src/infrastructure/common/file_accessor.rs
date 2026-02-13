use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::anyhow;
use derive_new::new;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use tokio::fs;
use crate::domain::chat::ChatList;
use crate::domain::config::{Config, EventSubscribeList};
use crate::domain::file_accessor::FileAccessor;
use async_trait::async_trait;

#[derive(new, Clone)]
pub struct JsonFileAccessor<T>
    where T: DeserializeOwned + Serialize + Send
{
    file_name: String,
    factory: Arc<dyn Fn() -> T + Send + Sync>,
}

#[async_trait]
impl<T> FileAccessor<T> for JsonFileAccessor<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Clone
{
    async fn read(&self)
        -> Result<T, Box<dyn Error + Send + Sync>>
    {
        let file_path = self.get_file_path()?;

        if file_path.exists() {
            let raw_string = fs::read_to_string(file_path).await?;
            Ok(serde_json::from_str(raw_string.as_str())?)
        } else {
            Ok((self.factory)())
        }
    }

    async fn write(&self, data: &T)
        -> Result<(), Box<dyn Error + Send + Sync>> {
        let raw_json = serde_json::to_string_pretty(data)?;

        let directory_path = self.get_directory_path()?;

        fs::create_dir_all(directory_path).await?;

        let file_path = self.get_file_path()?;

        let mut temp_path = file_path.clone();
        temp_path.set_extension("tmp");

        fs::write(&temp_path, &raw_json).await?;
        fs::rename(&temp_path, &file_path).await?;

        Ok(())
    }
}

impl<T> JsonFileAccessor<T> where T: Serialize + DeserializeOwned + Send {
    fn get_file_path(&self)
        -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        let mut path = self.get_directory_path()?;
        path.push(self.file_name.as_str());
        Ok(path)
    }

    fn get_directory_path(&self) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
        let mut directory_path = home::home_dir()
            .ok_or(anyhow!("Fail to find home directory"))?;
        directory_path.push(".watchdog");
        Ok(directory_path)
    }
}

pub fn get_chat_list_file_accessor() -> JsonFileAccessor<ChatList> {
    JsonFileAccessor::new(
        String::from("chat_list.json"),
        Arc::new(|| { ChatList::new() })
    )
}

pub fn get_config_file_accessor() -> JsonFileAccessor<Config> {
    JsonFileAccessor::new(
        String::from("config.json"),
        Arc::new(||{Config::new(None)})
    )
}

pub fn get_event_subscribe_file_accessor() -> JsonFileAccessor<EventSubscribeList> {
    JsonFileAccessor::new(
        String::from("subscribe.json"),
        Arc::new(||{EventSubscribeList::new()})
    )
}