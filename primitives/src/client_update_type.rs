use crate::{Chain, UpdateType};

pub async fn detect_update_type(
	_sink: &impl Chain,
	_header_height: u64,
	_timestamp: u64,
) -> Result<UpdateType, anyhow::Error> {
	todo!()
}
