mod alarm;

use std::error::Error;
use anyhow::anyhow;
use async_trait::async_trait;
use log::{debug, trace};
use crate::application::handler::command::alarm::AlarmCommand;
use crate::application::handler::command::Command::{Alarm, EventList, HealthCheck, HealthCheckAll, Help, Logs, Nothing};
use crate::application::handler::GeneralHandler;
use crate::domain::client::Message;

pub struct CommandMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub examples: &'static [&'static str],
}

impl CommandMeta {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        usage: &'static str,
        examples: &'static [&'static str],
    ) -> Self {
        Self { name, description, usage, examples }
    }
}

#[async_trait]
pub trait Run: Send + Sync {
    async fn run(&self, handler: &mut GeneralHandler, id: String, message: &Message) -> Result<String, Box<dyn Error + Send + Sync>>;
}

#[derive(Debug)]
pub enum Command {
    Logs(String, i32),
    HealthCheckAll,
    HealthCheck(String),
    Nothing,
    Alarm(AlarmCommand),
    EventList,
    Help(Option<String>),
}

impl Command {
    const HELP: CommandMeta = CommandMeta::new(
        "/help",
        "커맨드 사용법을 표시합니다",
        "/help [command]",
        &["/help", "/help logs"],
    );
    const LOGS: CommandMeta = CommandMeta::new(
        "/logs",
        "서버의 최근 로그를 가져옵니다",
        "/logs <server_name> <lines>",
        &["/logs main 100", "/logs api 50"],
    );
    const HEALTH: CommandMeta = CommandMeta::new(
        "/health",
        "서버 헬스체크 결과를 확인합니다",
        "/health [server_name]",
        &["/health", "/health main"],
    );
    const ALARM: CommandMeta = CommandMeta::new(
        "/alarm",
        "이벤트 알람을 구독 또는 해제합니다",
        "/alarm <add|remove|list> [event_name]",
        &["/alarm list", "/alarm add cpu-high", "/alarm remove cpu-high"],
    );
    const EVENT: CommandMeta = CommandMeta::new(
        "/event",
        "설정된 이벤트 목록을 표시합니다",
        "/event [list]",
        &["/event", "/event list"],
    );

    pub fn meta(&self) -> Option<&'static CommandMeta> {
        match self {
            Help(_)                            => Some(&Self::HELP),
            Logs(_, _)                         => Some(&Self::LOGS),
            HealthCheck(_) | HealthCheckAll    => Some(&Self::HEALTH),
            Alarm(_)                           => Some(&Self::ALARM),
            EventList                          => Some(&Self::EVENT),
            Nothing                            => None,
        }
    }

    pub fn all_docs() -> &'static [&'static CommandMeta] {
        &[&Self::HELP, &Self::LOGS, &Self::HEALTH, &Self::ALARM, &Self::EVENT]
    }

    pub fn render_help_all() -> String {
        let list = Self::all_docs()
            .iter()
            .map(|m| format!("{} — {}", m.name, m.description))
            .collect::<Vec<_>>()
            .join("\n");
        format!("사용 가능한 커맨드:\n\n{list}\n\n자세한 사용법: /help <command>\n예시: /help logs")
    }

    pub fn render_help_one(name: &str) -> Option<String> {
        let meta = Self::all_docs()
            .iter()
            .find(|m| m.name.trim_start_matches('/') == name)?;
        let examples = meta.examples
            .iter()
            .map(|e| format!("  {e}"))
            .collect::<Vec<_>>()
            .join("\n");
        Some(format!(
            "{} — {}\n\n사용법:\n  {}\n\n예시:\n{examples}",
            meta.name, meta.description, meta.usage
        ))
    }
}

#[async_trait]
impl Run for Command {
    async fn run(&self, handler: &mut GeneralHandler, id: String, message: &Message) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self {
            Command::Logs(name, n) => {
                handler.server_manager.logs(name.as_str(), *n).await
                    .ok_or_else(|| anyhow!("Logs are not available."))
                    .map_err(Into::into)
            },
            Command::HealthCheck(name) => {
                let health = handler.server_manager.healthcheck(name.as_str()).await;
                let response = format!("===\nServer: {name}\n Health: {health}");
                Ok(response)
            },
            Command::HealthCheckAll => {
                let response = handler.server_manager.healthcheck_all()
                    .await
                    .iter().map(|result|{format!("===\nServer: {}\nHealth: {}", result.0, result.1)})
                    .collect::<Vec<String>>()
                    .join("\n");
                Ok(response)
            },
            Command::Alarm(command) => {
                command.run(handler, id, message).await
            },
            Command::EventList => {
                let events = handler.event_config_use_case.list_event().await?;
                let event_names = events.iter().map(|e| e.name.clone()).collect::<Vec<String>>().join("\n");
                Ok(format!("Available events:\n{}", event_names))
            },
            Command::Help(name) => match name {
                None => Ok(Command::render_help_all()),
                Some(name) => Command::render_help_one(name)
                    .ok_or_else(|| anyhow!("알 수 없는 커맨드: /{name}\n\n{}", Command::render_help_all()).into()),
            },
            Command::Nothing => Ok(Command::render_help_all()),
        }
    }
}

impl Command {
    pub fn parse(text: &str) -> Self {
        trace!("Command::parse(text: {})", &text);
        let command = match text.split_whitespace().collect::<Vec<_>>()[..] {
            ["/help"] => Help(None),
            ["/help", name] => Help(Some(name.trim_start_matches('/').to_string())),
            ["/health", name] => HealthCheck(name.to_string()),
            ["/health"] => HealthCheckAll,
            ["/logs", name, n] => {
                match n.parse() {
                    Ok(n) => Logs(name.to_string(), n),
                    Err(_) => Nothing
                }
            },
            ["/alarm", "add", name] => {
                Alarm(AlarmCommand::Add(String::from(name)))
            },
            ["/alarm", "remove", name] => {
                Alarm(AlarmCommand::Remove(String::from(name)))
            },
            ["/alarm", "list"] => {
                Alarm(AlarmCommand::List)
            },
            ["/alarm"] => {
                Alarm(AlarmCommand::List)
            },
            ["/event", "list"] => EventList,
            ["/event"] => EventList,
            _ => Nothing
        };
        debug!("parsed command: {:?}", &command);
        command
    }
}
