use crate::{
	contract::INSTANTIATE_INTERPRETER_REPLY_ID,
	error::{Result, ContractError},
	events::make_event,
	network::load_this,
	state::{self, interpreter},
};
use cosmwasm_std::{to_binary, DepsMut, Reply, Response, StdError, StdResult, SubMsg, WasmMsg, Deps, Storage};
use cw_xc_interpreter::contract::{XCVM_INTERPRETER_EVENT_PREFIX, XCVM_INTERPRETER_EVENT_DATA_ORIGIN};
use xc_core::{CallOrigin, InterpreterOrigin, NetworkId};

use crate::{auth, prelude::*};

pub(crate) fn force_instantiate(
	auth: auth::Admin,
	gateway: Addr,
	deps: DepsMut,
	user_origin: Addr,
) -> Result {
	let config = load_this(deps.storage)?;
	let interpreter_code_id = match config.gateway.expect("expected setup") {
		GatewayId::CosmWasm { interpreter_code_id, .. } => interpreter_code_id,
	};

	let call_origin = CallOrigin::Local { user: user_origin };
	let interpreter_origin = InterpreterOrigin {
		user_origin: call_origin.user(config.network_id),
		salt: b"default".to_vec(),
	};
	let msg = instantiate(deps.as_ref(), gateway, interpreter_code_id, &interpreter_origin)?;
	Ok(Response::new().add_submessage(msg).add_event(make_event("interpreter.forced")))
}

pub fn instantiate(deps: Deps, admin: Addr, interpreter_code_id: u64, interpreter_origin: &InterpreterOrigin) -> Result<SubMsg, ContractError> {
	let next_interpreter_id: u128 = state::interpreter::INTERPRETERS_COUNT.get(deps.storage)? + 1;
    let instantiate_msg = WasmMsg::Instantiate2 {
			    admin: Some(admin.clone().into_string()),
			    code_id: interpreter_code_id,
			    msg: to_binary(&cw_xc_interpreter::msg::InstantiateMsg {
				    gateway_address: admin.clone().into_string(),
				    interpreter_origin: interpreter_origin.clone(),
			    })?,
			    funds: vec![],
				label: ["xcvm_interpreter", &next_interpreter_id.to_string()].join("_"),
			    salt: to_binary(&interpreter_origin.to_string().as_bytes())?,
		    };
    let interpreter_instantiate_submessage =
			    SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_INTERPRETER_REPLY_ID);
    Ok(interpreter_instantiate_submessage)
}


pub(crate) fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;

	// Catch the default `instantiate` event which contains `_contract_address` attribute that
	// has the instantiated contract's address
	let address = &response
		.events
		.iter()
		.find(|event| event.ty == "instantiate")
		.ok_or_else(|| StdError::not_found("instantiate event not found"))?
		.attributes
		.iter()
		.find(|attr| &attr.key == "_contract_address")
		.ok_or_else(|| StdError::not_found("_contract_address attribute not found"))?
		.value;
	let interpreter_address = deps.api.addr_validate(address)?;

	// Interpreter provides `network_id, user_id` pair as an event for the router to know which
	// pair is instantiated
	let event_name = format!("wasm-{}", XCVM_INTERPRETER_EVENT_PREFIX);
	let interpreter_origin = &response
		.events
		.iter()
		.find(|event| event.ty.starts_with(&event_name))
		.ok_or_else(|| StdError::not_found("interpreter event not found"))?
		.attributes
		.iter()
		.find(|attr| &attr.key == XCVM_INTERPRETER_EVENT_DATA_ORIGIN)
		.ok_or_else(|| StdError::not_found("no data is returned from 'xcvm_interpreter'"))?
		.value;
	let interpreter_origin =
		xc_core::shared::decode_base64::<_, InterpreterOrigin>(interpreter_origin.as_str())?;

	let interpreter_id = state::interpreter::INTERPRETERS_COUNT.get(deps.storage)? + 1;
	let interpreter =
		state::interpreter::Interpreter { address: interpreter_address, interpreter_id };

	state::interpreter::INTERPRETERS_COUNT.save(deps.storage, &interpreter_id)?;
	state::interpreter::INTERPRETERS.save(deps.storage, interpreter_id.0, &interpreter)?;
	state::interpreter::INTERPRETERS_ORIGIN_TO_ID.save(
		deps.storage,
		interpreter_origin,
		&interpreter_id,
	)?;

	Ok(Response::new().add_event(
		make_event("xcvm.interpreter.instantiated")
			.add_attribute("interpreter_id", interpreter_id.to_string()),
	))
}
