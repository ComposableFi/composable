use crate::prelude::*;
/// maintains high fidelity with extrinsic
#[derive(RuntimeDebug, Serialize)]
pub struct OutputRecord {
	pub to: String,
	pub vesting_schedule_added: String,
	pub total: u128,
	pub window_start: String,
	pub window_period: String,
}

/// that is what stored on chain, human friendly
#[derive(RuntimeDebug, Serialize)]
pub struct ListRecord {
	pub pubkey: String,
	pub account: String,
	pub vesting_schedule_id: u128,
	pub total : u128,
	pub per_period : u128,
	pub period_count: u32,
	pub window_start: String,
	pub window_period: String,
	pub already_claimed : u128,
}
