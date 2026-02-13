use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub base_url: Option<String>,
    pub docker_container_name: Option<String>,
    pub health_check_path: Option<String>,
    pub kill_path: Option<String>,
    pub log_command: Option<String>,
}

impl ServerConfig {
    pub fn new(name: String, base_url: Option<String>, docker_container_name: Option<String>, health_check_path: Option<String>, kill_path: Option<String>, log_command: Option<String>,) -> Self {
        Self {
            name: String::from(name),
            base_url,
            docker_container_name,
            health_check_path,
            kill_path,
            log_command
        }
    }
}
