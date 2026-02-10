use log::{debug, trace};
use crate::infrastructure::handler::command::Command::{Logs, Nothing};

#[derive(Debug)]
pub enum Command {
    Logs(String, i32),
    Nothing
}

impl Command {
    pub fn parse(text: String) -> Self {
        trace!("Command::parse(text: {})", &text);
        let command = match text.split_whitespace().collect::<Vec<_>>()[..] {
            ["/logs", name, n] => {
                match n.parse() {
                    Ok(n) => Logs(name.to_string(), n),
                    Err(_) => Nothing
                }
            },
            _ => Nothing
        };
        debug!("parsed command: {:?}", &command);
        command
    }
}