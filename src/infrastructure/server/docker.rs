use log::error;
use crate::domain::server::health::Health;
use crate::domain::server::Server;
use crate::infrastructure::server::util::SystemCommandExecutor;

pub struct DockerHealthChecker {
    system_command_executor: SystemCommandExecutor
}

impl DockerHealthChecker {
    pub fn new() -> Self {
        Self {
            system_command_executor: SystemCommandExecutor::new()
        }
    }

    pub async fn healthcheck(&self, server: &Server) -> Health {
        if let Some(container_name) = &server.docker_container_name {
            match self.system_command_executor
                .capture_output("docker",
                         &["inspect",
                             "--format='{{json .State.Status}}'",
                             container_name.as_str()
                         ]).await {
                Ok(string) => {
                    let string = string.as_str().trim().trim_matches('\'').trim_matches('"');
                    match string {
                        "running"    => Health::Healthy,
                        "created"    => Health::Unknown(String::from("Container is creating")),
                        "restarting" => Health::Degraded,
                        "removing"   => Health::Deregistered,
                        "paused"     => Health::Down,
                        "exited"     => Health::Down,
                        "dead"       => Health::Down,
                        _            => Health::Unknown(String::from(format!("Failed to parse container status: '{}'", string)))
                    }
                },
                Err(e) => {
                    error!("Err: {e}");
                    Health::Unknown(String::from("System command is not available"))
                }
            }
        } else {
            Health::Unknown(String::from("Container name undefined"))
        }
    }
}