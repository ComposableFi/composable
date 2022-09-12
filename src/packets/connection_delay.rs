use ibc::{timestamp::Timestamp, Height};
use primitives::error::Error;
use std::time::Duration;

/// Verify the time and height delays
pub fn has_delay_elapsed(
	current_time: Timestamp,
	current_height: Height,
	client_update_time: Timestamp,
	client_update_height: Height,
	delay_period_time: Duration,
	delay_period_blocks: u64,
) -> Result<bool, anyhow::Error> {
	let earliest_time = (client_update_time + delay_period_time)
		.map_err(|_| Error::Custom("Timestamp overflow".to_string()))?;
	if !(current_time == earliest_time || current_time.after(&earliest_time)) {
		return Ok(false)
	}

	let earliest_height = client_update_height.add(delay_period_blocks);
	if current_height < earliest_height {
		return Ok(false)
	}

	Ok(true)
}
