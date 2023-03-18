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
	/// add schedule from table
	Add(AddCommand),
	/// list all existing schedules
	List(ListCommand),
	/// Unlock schedule, so really to all vesting now
	Unlock(UnlockCommand),
	/// All not yet vested amount to be unlocked and transferred back to wallet
	Delete(DeleteCommand),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct UnlockCommand {
	/// Link to CSV file with schedule
	#[arg(long)]
	pub schedule: String,

	/// `VestedTransferOrigin`
	#[arg(long)]
	pub key: String,
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ListCommand {
	/// Link to CSV file with schedule
	#[arg(long)]
	pub out: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct DeleteCommand {
	/// Link to CSV file with schedule
	#[arg(long)]
	pub schedule: String,

	/// `VestedTransferOrigin`
	#[arg(long)]
	pub key: String,

	/// Where to transfer unlocked amount
	#[arg(long)]	
	pub to: String,
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
pub struct UnlockRecord {
	pub account: String,
	pub vesting_schedule_id: u128,
}

#[derive(Debug, serde::Deserialize)]
pub struct DeleteRecord {
	pub account: String,
	pub vesting_schedule_id: u128,
	pub total: u128,
	pub already_claimed : u128,	
}