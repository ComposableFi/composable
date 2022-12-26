mod args;
mod error;
mod substrate_client;
mod types;

use args::{Args, Command};
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let result = match args.main_command {
        Command::Substrate(substrate_command) => substrate_command.run().await,
        _ => todo!(),
    };

    if let Err(e) = result {
        println!("{}", e);
    }
}
