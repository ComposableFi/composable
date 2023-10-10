//! is used to track cross chain programs execution statuses (non atomic execution)
//! useful for timeouts, message acknowledgements failures, spawn concurrency handling  

use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::{Item, Map};
use xc_core::{
	gateway::BridgeForwardMsg,
	transport::ibc::{IbcIcs20ProgramRoute, TransportTrackerId},
};

pub(crate) const CURRENT_BRIDGE: Item<(BridgeForwardMsg, IbcIcs20ProgramRoute)> =
	Item::new("current_bridge");

pub(crate) const CHANNEL_SEQUENCE_TO_BRIDGE_MSG: Map<(String, u64), BridgeForwardMsg> =
	Map::new("channel_sequence_to_bridge_msg");

pub fn track(
	storage: &mut dyn Storage,
	tracker_id: TransportTrackerId,
	msg: BridgeForwardMsg,
) -> StdResult<()> {
	let (channel_id, sequence) = match tracker_id {
		TransportTrackerId::Ibc { channel_id, sequence } => (channel_id, sequence),
	};
	let key = (channel_id.to_string(), sequence);

	CHANNEL_SEQUENCE_TO_BRIDGE_MSG.save(storage, key, &msg)
}

pub fn bridge_lock(
	storage: &mut dyn Storage,
	lock: (BridgeForwardMsg, IbcIcs20ProgramRoute),
) -> StdResult<()> {
	if CURRENT_BRIDGE.load(storage).is_ok() {
		return Err(cosmwasm_std::StdError::GenericErr { msg: "bridge is locked".to_string() })
	}

	CURRENT_BRIDGE.save(storage, &lock)?;
	Ok(())
}

pub fn bridge_unlock(
	storage: &mut dyn Storage,
) -> StdResult<(BridgeForwardMsg, IbcIcs20ProgramRoute)> {
	let item = CURRENT_BRIDGE.load(storage)?;
	CURRENT_BRIDGE.remove(storage);
	Ok(item)
}

/// Gets interpreter and gateway owned state on behalf interpreter (coins)
pub fn get_interpreter_track(
	storage: &dyn Storage,
	channel_id: &str,
	sequence: u64,
) -> StdResult<BridgeForwardMsg> {
	let key = (channel_id.to_string(), sequence);
	let msg = CHANNEL_SEQUENCE_TO_BRIDGE_MSG.load(storage, key)?;
	Ok(msg)
}
