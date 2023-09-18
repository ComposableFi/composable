use crate::error::ContractError;
use cosmwasm_std::{entry_point, wasm_execute, DepsMut, Env, Event, Response};
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
			},
		SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { channel, sequence }) =>
			handle_transport_failure(deps, env, channel, sequence, "timeout".to_string()),
	}
}

/// return funds to interpreter and sets final error
fn handle_transport_failure(
	deps: DepsMut,
	_env: Env,
	channel: ChannelId,
	sequence: u64,
	reason: String,
) -> Result<cosmwasm_std::Response, ContractError> {
	deps.api.debug(
		format!("cvm::gateway::handle::transport_failure {} {} {}", &channel, sequence, &reason)
			.as_str(),
	);
	let msg = cw_xc_interpreter::msg::ExecuteMsg::SetErr { reason };
	let (interpreter_origin, tracked_state) =
		crate::state::tracking::get_interpreter_track(deps.storage, channel.as_str(), sequence)?;
	let interpreter = crate::state::interpreter::get_by_origin(deps.as_ref(), interpreter_origin)?;
	let mut response = Response::new();
	response = response.add_message(wasm_execute(interpreter.address, &msg, tracked_state.assets)?);
	Ok(response.add_event(Event::new("cvm::gateway::handle::transport_failure")))
}
