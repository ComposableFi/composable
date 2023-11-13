use crate::{
	batch::BatchResponse,
	contract::ReplyId,
	error::{ContractError, Result},
	events::make_event,
	network::load_this,
	state,
};
use cosmwasm_std::{
	to_binary, Deps, DepsMut, Reply, Response, StdError, StdResult, SubMsg, WasmMsg,
};

use cw_xc_executor::events::CvmInterpreterInstantiated;
use xc_core::{CallOrigin, InterpreterOrigin};

use crate::{auth, prelude::*};

pub(crate) fn force_instantiate(
	_: auth::Admin,
	gateway: Addr,
	deps: DepsMut,
	user_origin: Addr,
	salt: String,
) -> Result<BatchResponse> {
	let config = load_this(deps.storage)?;
	let interpreter_code_id = match config.gateway.expect("expected setup") {
		GatewayId::CosmWasm { interpreter_code_id, .. } => interpreter_code_id,
		GatewayId::Evm { .. } => Err(ContractError::RuntimeUnsupportedOnNetwork)?,
	};
	let salt = salt.into_bytes();

	let call_origin = CallOrigin::Local { user: user_origin };
	let interpreter_origin =
		InterpreterOrigin { user_origin: call_origin.user(config.network_id), salt: salt.clone() };
	let msg = instantiate(deps.as_ref(), gateway, interpreter_code_id, &interpreter_origin, salt)?;
	Ok(BatchResponse::new().add_submessage(msg).add_event(
		make_event("interpreter.forced")
			.add_attribute("interpreter_origin", interpreter_origin.to_string()),
	))
}

pub fn instantiate(
	deps: Deps,
	admin: Addr,
	interpreter_code_id: u64,
	interpreter_origin: &InterpreterOrigin,
	salt: Vec<u8>,
) -> Result<SubMsg, ContractError> {
	let next_interpreter_id: u128 =
		state::interpreter::INTERPRETERS_COUNT.load(deps.storage).unwrap_or_default() + 1;

	let instantiate_msg = WasmMsg::Instantiate2 {
		admin: Some(admin.clone().into_string()),
		code_id: interpreter_code_id,
		msg: to_binary(&cw_xc_executor::msg::InstantiateMsg {
			gateway_address: admin.into_string(),
			interpreter_origin: interpreter_origin.clone(),
		})?,
		funds: vec![],
		// and label has some unknown limits  (including usage of special characters)
		label: format!("cvm_executor_{}", &next_interpreter_id),
		// salt limit is 64 characters
		salt: to_binary(&salt)?,
	};
	let interpreter_instantiate_submessage =
		SubMsg::reply_on_success(instantiate_msg, ReplyId::InstantiateInterpreter.into());
	Ok(interpreter_instantiate_submessage)
}

pub(crate) fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	deps.api.debug(&format!(
		"cvm:: {}",
		serde_json_wasm::to_string(&msg).map_err(|e| StdError::generic_err(e.to_string()))?
	));
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

	let event_name = format!("wasm-{}", CvmInterpreterInstantiated::NAME);
	let interpreter_origin = &response
		.events
		.iter()
		.find(|event| event.ty.starts_with(&event_name))
		.ok_or_else(|| StdError::not_found("interpreter event not found"))?
		.attributes
		.iter()
		.find(|attr| attr.key == CvmInterpreterInstantiated::INTERPRETER_ORIGIN)
		.ok_or_else(|| StdError::not_found("no data is returned from 'cvm_executor'"))?
		.value;
	let interpreter_origin =
		xc_core::shared::decode_base64::<_, InterpreterOrigin>(interpreter_origin.as_str())?;

	let interpreter_id: u128 =
		state::interpreter::INTERPRETERS_COUNT.load(deps.storage).unwrap_or_default() + 1;
	let interpreter = state::interpreter::Interpreter {
		address: interpreter_address,
		interpreter_id: interpreter_id.into(),
	};

	state::interpreter::INTERPRETERS_COUNT.save(deps.storage, &interpreter_id)?;
	state::interpreter::INTERPRETERS.save(deps.storage, interpreter_id, &interpreter)?;
	state::interpreter::INTERPRETERS_ORIGIN_TO_ID.save(
		deps.storage,
		interpreter_origin,
		&interpreter_id,
	)?;

	deps.api.debug("cvm:: saved interpreter");

	Ok(Response::new().add_event(
		make_event("cvm.executor.instantiated")
			.add_attribute("interpreter_id", interpreter_id.to_string()),
	))
}
