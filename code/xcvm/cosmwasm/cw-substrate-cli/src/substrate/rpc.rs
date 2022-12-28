use clap::Args;

#[derive(Args, Debug)]
pub struct Command {
    #[command(subcommand)]
    pub subcommands: Subcommands,
}
