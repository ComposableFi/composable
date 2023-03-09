use crate::prelude::*;
/// maintains high fidelity with extrinsic
#[derive(Debug, serde::Serialize)]
pub struct OutputRecord {
	pub to: String,
	pub vesting_schedule_added: String,
	pub total: u128,
	pub window_start: String,
	pub window_period: String,
}
