use std::error::Error;
use serde::{Serialize, de::DeserializeOwned};
use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait FileAccessor<T>: DynClone + Send + Sync
    where T: DeserializeOwned + Serialize + Send
{
    async fn read(&self) -> Result<T, Box<dyn Error + Send + Sync>>;
    async fn write(&self, data: &T) -> Result<(), Box<dyn Error + Send + Sync>>;
}

dyn_clone::clone_trait_object!(<T> FileAccessor<T> where T: DeserializeOwned + Serialize + Send);
