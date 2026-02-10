use clap::Parser;
use log::{debug, trace};
use server_watchdog::infrastructure::cli::Cli;

#[tokio::main]
async fn main() {
    env_logger::init();
    trace!("main start");

    let cli = Cli::parse();
    debug!("command: {:?}", &cli.command);

    cli.command.run().await;
    trace!("main end");
}
