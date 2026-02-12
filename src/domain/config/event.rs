use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventConfig {
    pub r#type: String, // logs, health
    pub name: String,
    pub target: String, // target server
    pub keyword: String,
}