use clap::Subcommand;
use log::{debug, trace};
use struct_input::StructInputTrait;
use crate::application::config::ClientConfigUseCase;
use crate::domain::config::ClientConfig;
use crate::infrastructure::cli::util::{read_string, read_string_option, FormatChecker};

#[derive(Subcommand)]
#[derive(Debug)]
pub enum ClientCommands {
    Add,
    List
}

impl ClientCommands {
    pub async fn run(&self, client_config_use_case: Box<dyn ClientConfigUseCase>) {
        trace!("client command start: {:?}", &self);
        match self {
            ClientCommands::Add => {
                debug!("add client");

                let client = ClientConfig::from_input().await;

                debug!("new client config: {:?}", &client);
                let _ = client_config_use_case.add_client(client).await;
            },
            ClientCommands::List => {
                debug!("list client");
                let clients = client_config_use_case.list_client().await
                    .unwrap();
                debug!("clients: {:?}", &clients);
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
        trace!("client command end");
    }
}