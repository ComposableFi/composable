use crate::prelude::*;

/// maintains high fidelity with extrinsic
#[derive(RuntimeDebug, Serialize, Clone)]
pub struct OutputRecord {
	pub to: String,
	pub total: u128,
	pub window_start: String,
	pub window_period: String,
}

/// maintains high fidelity with extrinsic
#[derive(RuntimeDebug, Serialize)]
pub struct OutputRecordOne {
	// #[serde(flatten)]
	// pub base: OutputRecord,
	pub to: String,
	pub total: u128,
	pub window_start: String,
	pub window_period: String,
	pub vesting_schedule_added: String,
}

#[derive(RuntimeDebug, Serialize)]
pub struct OutputBatch {
	// #[serde(flatten)]
	// pub base: OutputRecord,
	pub to: String,
	pub total: u128,
	pub window_start: String,
	pub window_period: String,
	pub vesting_schedules_added: String,
	pub batch: u16,
}

/// that is what stored on chain, human friendly
#[derive(RuntimeDebug, Serialize)]
pub struct ListRecord {
	pub pubkey: String,
	pub account: String,
	pub vesting_schedule_id: u128,
	pub total: u128,
	pub per_period: u128,
	pub period_count: u32,
	pub window_start: String,
	pub window_period: String,
	pub already_claimed: u128,
}
