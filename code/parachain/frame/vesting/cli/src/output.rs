/// maintains high fidelity with extrinsic
#[derive(Debug, serde::Serialize)]
pub struct OutputRecord {
    pub to: String,
    pub vesting_schedule_added: String,
}