use clap::Subcommand;
use log::{debug, trace};
use struct_input::StructInputTrait;
use crate::application::config::ServerConfigUseCase;
use crate::domain::config::ServerConfig;
use crate::domain::server::Server;
use crate::infrastructure::cli::util::{read_string, read_string_option, FormatChecker};

#[derive(Subcommand)]
#[derive(Debug)]
pub enum ServerCommands {
    Add,
    List
}

impl ServerCommands {
    pub async fn run(&self, server_config_adapter: Box<dyn ServerConfigUseCase>) {
        trace!("server command start: {:?}", &self);
        match self {
            ServerCommands::Add => {
                debug!("add server");
                println!("--- Add Server ---");
                let config = ServerConfig::from_input().await;
                debug!("new server config: {:?}", &config);
                let _ = server_config_adapter.add_server(config).await;
            },
            ServerCommands::List => {
                debug!("list server");

                let servers: Vec<ServerConfig> = server_config_adapter.list_server().await
                    .unwrap();
                debug!("servers: {:?}", &servers);

                println!("--- Server List ---");

                if servers.is_empty() {
                    println!("Empty Server");
                } else {
                    for server in servers {
                        let server = Server::from(server);

                        let command = match server.log_command.as_ref() {
                            Some(command) => command.join(" "),
                            None => "None".to_string()
                        };

                        println!(
                            "=========\nName: {}\nBASE URL: {}\nDocker Container Name: {}\nKill URL: {}\nHealth Check URL: {}\nLog command: {}\n\n",
                            server.name,
                            server.base_url.as_deref().unwrap_or("None"),
                            server.docker_container_name.as_deref().unwrap_or("None"),
                            server.get_kill_url().as_deref().unwrap_or("None"),
                            server.get_health_check_url().as_deref().unwrap_or("None"),
                            command
                        );
                    }
                }
            }
        }
        trace!("server command end");
    }
}