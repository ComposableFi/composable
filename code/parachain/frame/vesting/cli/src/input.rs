use clap::Parser;

#[derive(clap::Parser, Debug)]
pub struct Args {
	/// WS url to node
	#[arg(long)]
	pub client: String,

	#[command(subcommand)]
	pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
	Add(AddCommand),
	List,
	Clean(CleanCommand),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CleanCommand {
	/// Link to CSV file with schedule
	#[arg(long)]
	pub schedule: String,

	/// `VestedTransferOrigin`
	#[arg(long)]
	pub key: String,
}

/// So it validates all vesting parameters and dry-runs on RPC node.
/// Outputs hex encoded extrinsic to call
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AddCommand {
	/// Link to CSV file with schedule
	#[arg(long)]
	pub schedule: String,

	/// `VestedTransferOrigin`
	#[arg(long)]
	pub key: String,

	/// From
	#[arg(long)]
	pub from: String,
}

/// maintains high fidelity with extrinsic
#[derive(Debug, serde::Deserialize)]
pub struct AddRecord {
	pub account: String,
	/// unix timestamp
	pub window_moment_start: u64,
	/// unix time
	pub window_moment_period: u64,
	pub period_count: u32,
	/// amount
	pub per_period: u128,
}

#[derive(Debug, serde::Deserialize)]
pub struct CleanRecord {
	pub account: String,
	pub vesting_schedule_id: u128,
}