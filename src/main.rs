use clap::Parser;
use server_manager::core::cli::Cli;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = Cli::parse();

    cli.command.run().await;
}
