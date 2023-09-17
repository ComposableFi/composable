//! is used to track cross chain programs execution statuses (non atomic execution)

use crate::prelude::*;
use cosmwasm_std::{Coin, StdResult, Storage};
use cw_storage_plus::Map;
use xc_core::{transport::ibc::TransportTrackerId, InterpreterOrigin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct TrackedState {
	/// funds sent to the other network, tracked on side of gateway,
	/// so it is easy to refund them if needed to interpreter
	pub assets: Vec<Coin>,
}

pub fn track(
	storage: &mut dyn Storage,
	interpreter_origin: InterpreterOrigin,
	tracker_id: TransportTrackerId,
	state: TrackedState,
) -> StdResult<()> {
	let (channel_id, sequence) = match tracker_id {
		TransportTrackerId::Ibc { channel_id, sequence } => (channel_id, sequence),
	};
	let key = (interpreter_origin, channel_id.to_string(), sequence);
	INTERPRETER_CHANNEL_SEQUENCE_TO_TRACKED.save(storage, key, &state)
}

pub(crate) const INTERPRETER_CHANNEL_SEQUENCE_TO_TRACKED: Map<
	(InterpreterOrigin, String, u64),
	TrackedState,
> = Map::new("interpreter_channel_sequence_to_tracked");
