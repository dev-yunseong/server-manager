use clap::Subcommand;
use crate::application::config::AuthUseCase;

#[derive(Subcommand)]
#[derive(Debug)]
pub enum PasswordCommands {
    Set {
        password: String
    },
}

impl PasswordCommands {

    pub async fn run(&self, auth_use_case: Box<dyn  AuthUseCase>) {
        match self {
            PasswordCommands::Set { password } => {
                println!("Entered password: {}", password);
                auth_use_case.set_password(password.clone()).await;
            }
        }
    }
}
