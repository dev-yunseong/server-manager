use clap::Subcommand;
use log::{debug, trace};
use crate::application::config::EventConfigUseCase;
use crate::domain::config::EventConfig;
use crate::infrastructure::cli::util::{read_string, FormatChecker};

#[derive(Subcommand)]
#[derive(Debug)]
pub enum EventCommands {
    Add,
    List,
    Remove { name: String }
}

impl EventCommands {
    pub async fn run(&self, event_config_adapter: Box<dyn EventConfigUseCase>) {
        trace!("event command start: {:?}", &self);
        match self {
            EventCommands::Add => {
                debug!("add event");
                println!("--- Add Event ---");
                let name = read_string("name", FormatChecker::Name).await;
                let event_type = read_string("type (logs, health)", FormatChecker::Name).await;
                let target = read_string("target server name", FormatChecker::Name).await;
                let keyword = read_string("keyword", FormatChecker::None).await;

                let config = EventConfig {
                    r#type: event_type,
                    name,
                    target,
                    keyword,
                };
                debug!("new event config: {:?}", &config);
                event_config_adapter.add_event(config).await.unwrap();
            },
            EventCommands::List => {
                debug!("list event");
                let events = event_config_adapter.list_event().await.unwrap();
                debug!("events: {:?}", &events);

                println!("--- Event List ---");

                if events.is_empty() {
                    println!("Empty Event");
                } else {
                    for event in events {
                        println!(
                            "=========\nName: {}\nType: {}\nTarget: {}\nKeyword: {}\n\n",
                            event.name,
                            event.r#type,
                            event.target,
                            event.keyword
                        );
                    }
                }
            },
            EventCommands::Remove { name } => {
                debug!("remove event: {}", name);
                event_config_adapter.remove_event(name.clone()).await.unwrap();
                println!("Event '{}' removed.", name);
            }
        }
        trace!("event command end");
    }
}