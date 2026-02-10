use clap::Subcommand;
use crate::application::config::ClientConfigUseCase;
use crate::domain::config::ClientConfig;
use crate::infrastructure::cli::util::{read_string, read_string_option};

#[derive(Subcommand)]
pub enum ClientCommands {
    Add,
    List
}

impl ClientCommands {
    pub async fn run(&self, client_config_use_case: Box<dyn ClientConfigUseCase>) {
        match self {
            ClientCommands::Add => {
                let name = read_string("Name").await;
                let kind = read_string("kind (ex: telegram)").await;
                let token = read_string_option("Token").await;

                let client = match kind.as_str() {
                    "telegram" => ClientConfig::new_telegram(name.as_str(), token.unwrap().as_str()),
                    _ => { 
                        println!("kind({kind}) is not available");
                        return;
                    }
                };
                
                client_config_use_case.add_client(client).await;
            },
            ClientCommands::List => {
                let clients = client_config_use_case.list_client().await;
                println!("--- Client List ---");
                if clients.is_empty() {
                    println!("Empty Client");
                } else {
                    for client in clients {
                        println!(
                            "=========\nName: {}\nKind: {}\nToken: {}\n\n",
                            client.name,
                            client.kind,
                            client.token.unwrap_or("None".to_string())
                        );
                    }
                }
            }
        }
    }
}