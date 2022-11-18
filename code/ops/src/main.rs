#![feature(const_trait_impl)]

mod common;
mod crowdloan;
mod operational;

use std::fmt::Debug;

use clap::Command;
use sp_core::Pair;
use substrate_api_client::{rpc::WsRpcClient, Api};

const COMMAND_CROWDLOAN: &str = "crowdloan";
const COMMAND_CROWDLOAN_SEED: &str = "seed";
const COMMAND_OPERATIONAL: &str = "operational";
const COMMAND_OPERATIONAL_FUND_INVESTORS: &str = "fund-investors";
const COMMAND_OPERATIONAL_FUND_MULTISIGS: &str = "fund-multisigs";
const COMMAND_OPERATIONAL_FUND_DUST: &str = "fund-dust";
const COMMAND_OPERATIONAL_SETUP_NATIVE_COUNCIL: &str = "setup-native-council";
const COMMAND_OPERATIONAL_SETUP_TECHNICAL_COUNCIL: &str = "setup-technical-council";

fn wrap_cmd<E: Debug>(command: &str, cmd: Result<(), E>) -> Result<(), std::io::Error> {
	match cmd {
		Ok(_) => {
			log::info!("Successfully executed {}", command);
			Ok(())
		},
		Err(e) => Err(std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("Failed to execute {}, reason={:?}", command, e),
		)),
	}
}

fn main() -> Result<(), std::io::Error> {
	env_logger::init();

	let cli = Command::new("composable-cli")
		.about("In the beginning, there was nothing but darkness.")
		.subcommand_required(true)
		.arg(clap::arg!(--"node-url" <NODE_URL> "The websocket URL of the node." ))
		.arg(clap::arg!(--"sudo-key" <SUDO_KEY> "The SUDO key in prefixed 0x hex format."))
		.subcommand(
			Command::new(COMMAND_CROWDLOAN)
				.subcommand_required(true)
				.subcommand(Command::new(COMMAND_CROWDLOAN_SEED)),
		)
		.subcommand(
			Command::new(COMMAND_OPERATIONAL)
				.subcommand_required(true)
				.subcommand(Command::new(COMMAND_OPERATIONAL_FUND_INVESTORS))
				.subcommand(Command::new(COMMAND_OPERATIONAL_FUND_MULTISIGS))
				.subcommand(Command::new(COMMAND_OPERATIONAL_FUND_DUST))
				.subcommand(Command::new(COMMAND_OPERATIONAL_SETUP_NATIVE_COUNCIL))
				.subcommand(Command::new(COMMAND_OPERATIONAL_SETUP_TECHNICAL_COUNCIL)),
		);

	let args = cli.get_matches();
	let node_url = args.get_one::<String>("node-url").expect("required");
	let sudo_key = args.get_one::<String>("sudo-key").expect("required");

	let sudo_account = sp_core::sr25519::Pair::from_string(&sudo_key, None).unwrap();
	let client = WsRpcClient::new(&node_url);
	let api = Api::<_, _, _>::new(client)
		.map(|api| api.set_signer(sudo_account.clone()))
		.unwrap();

	match args.subcommand() {
		Some((cmd, sub_args)) if cmd == COMMAND_CROWDLOAN => match sub_args.subcommand() {
			Some((sub_cmd, _)) if sub_cmd == COMMAND_CROWDLOAN_SEED =>
				wrap_cmd(sub_cmd, crowdloan::crowdloan_seed(api)),

			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown subcommand")),
		},

		Some((cmd, sub_args)) if cmd == COMMAND_OPERATIONAL => match sub_args.subcommand() {
			Some((sub_cmd, _)) if sub_cmd == COMMAND_OPERATIONAL_FUND_INVESTORS =>
				wrap_cmd(sub_cmd, operational::fund_investors(api)),

			Some((sub_cmd, _)) if sub_cmd == COMMAND_OPERATIONAL_FUND_MULTISIGS =>
				wrap_cmd(sub_cmd, operational::fund_multisigs(api)),

			Some((sub_cmd, _)) if sub_cmd == COMMAND_OPERATIONAL_FUND_DUST =>
				wrap_cmd(sub_cmd, operational::fund_dust(api)),

			Some((sub_cmd, _)) if sub_cmd == COMMAND_OPERATIONAL_SETUP_NATIVE_COUNCIL =>
				wrap_cmd(sub_cmd, operational::setup_native_council(api)),

			Some((sub_cmd, _)) if sub_cmd == COMMAND_OPERATIONAL_SETUP_TECHNICAL_COUNCIL =>
				wrap_cmd(sub_cmd, operational::setup_technical_council(api)),

			_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown subcommand")),
		},

		_ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown command")),
	}
}
