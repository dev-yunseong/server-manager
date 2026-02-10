use crate::infrastructure::handler::command::Command::{Logs, Nothing};

pub enum Command {
    Logs(String, i32),
    Nothing
}

impl Command {
    pub fn parse(text: String) -> Self {
        match text.split_whitespace().collect::<Vec<_>>()[..] {
            ["/logs", name, n] => {
                match n.parse() {
                    Ok(n) => Logs(name.to_string(), n),
                    Err(_) => Nothing   
                }
            },
            _ => Nothing
        }
    }
}