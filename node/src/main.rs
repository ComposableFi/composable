mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;
mod runtime;

fn main() -> sc_cli::Result<()> {
	command::run()
}
