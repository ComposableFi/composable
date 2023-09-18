use crate::error::ContractError;
use cosmwasm_std::{entry_point, DepsMut, Env, Response};
use ibc_rs_scale::core::ics24_host::identifier::ChannelId;
use xc_core::transport::ibc::{ics20::hook::IBCLifecycleComplete, SudoMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> crate::error::Result {
	deps.api
		.debug(&format!("cvm::gateway::sudo {}", serde_json_wasm::to_string(&msg)?));
	match msg {
		SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
			channel,
			sequence,
			ack,
			success,
		}) =>
			if !success {
				handle_transport_failure(deps, env, channel, sequence, ack)
			} else {
				Ok(Response::new())
			}
		SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { channel, sequence }) =>
			handle_transport_failure(deps, env, channel, sequence, "timeout".to_string()),
	}
}

fn handle_transport_failure(deps: DepsMut, env: Env, channel: ChannelId, sequence: u64, reason: String) -> Result<cosmwasm_std::Response, ContractError> {
    
}
