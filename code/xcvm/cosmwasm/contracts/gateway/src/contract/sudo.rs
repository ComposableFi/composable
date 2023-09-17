use crate::error::ContractError;
use cosmwasm_std::{entry_point, DepsMut, Env};
use xc_core::transport::ibc::{ics20::hook::IBCLifecycleComplete, SudoMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> crate::error::Result {
	match msg {
		SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
			channel,
			sequence,
			ack,
			success,
		}) =>
			if !success {
				handle_transport_failure(deps, env, channel, sequence, ack)
			},
		SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { channel, sequence }) =>
			handle_transport_failure(deps, env, channel, sequence, "timeout".to_string()),
	}
}
