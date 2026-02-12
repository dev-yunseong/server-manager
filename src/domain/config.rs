mod event;
mod client;
mod server;

use derive_new::new;
use serde::{Deserialize, Serialize};
pub use event::*;
pub use client::*;
pub use server::*;

#[derive(Serialize, Deserialize, Debug, new)]
pub struct Config {
    pub password: Option<String>,
    #[new(default)]
    pub clients: Vec<ClientConfig>,
    #[new(default)]
    pub servers: Vec<ServerConfig>,
    #[new(default)]
    pub events: Vec<EventConfig>
}