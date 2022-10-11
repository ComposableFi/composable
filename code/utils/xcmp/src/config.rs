use clap::{clap_derive::ArgEnum, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
	#[clap(subcommand)]
	pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	// DISCUSS: move to `subkey` if they are ok with that https://github.com/paritytech/substrate/discussions/12355
	Parachain(Address),
	// TODO: unify transfer under single command
	// DISCUSS: should we move this to `node` with support for only our chains? (in next PR)
	TransferNative(TransferNative),
	ReserveTransferNative(ReserveTransferNative),
	Sudo(Sudo),
}

#[derive(Parser, Debug)]
pub struct Sudo {
	#[clap(subcommand)]
	pub command: SudoCommand,
}

#[derive(Subcommand, Debug)]
pub enum SudoCommand {
	TransferNative(TransferNative),
	ReserveTransferNative(ReserveTransferNative),
	Execute(Execute),
}

/// tries to parse and execute extrinsic again define chain
#[derive(Parser, Debug)]
pub struct Execute {
	/// path to key
	pub suri: String, // name for parity clis

	/// hex encoded call to execute
	pub call: String,

	/// ask before
	//#[clap(default = true)]
	pub ask: Option<bool>,

	/// one of supported networks
	pub network: String,

	pub rpc: String,
}

#[derive(Parser, Debug)]
pub struct TransferNative {
	pub from_account_id: String,
	pub to_account_id: String,
	pub amount: u128,
	pub rpc: String,
}

#[derive(Parser, Debug)]
pub struct AcceptChannelOpen {
	pub para_id: u32,
	pub root: String,
	pub rpc: String,
}

#[derive(Parser, Debug)]
pub struct Address {
	pub para_id: u32,
	#[clap(arg_enum, value_parser, default_value_t = AddressFormat::Base58)]
	pub format: AddressFormat,
}

#[derive(Parser, Debug)]
pub struct ReserveTransferNative {
	pub from_account_id: String,
	pub to_para_id: u32,
	pub to_account_id: String,
	pub amount: u128,
	pub rpc: String,
}

#[derive(Parser, Debug, Clone, ArgEnum)]
pub enum AddressFormat {
	Hex,
	Base58,
}
