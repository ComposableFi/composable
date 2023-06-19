extern crate alloc;

use crate::{
	common,
	contract::INSTANTIATE_INTERPRETER_REPLY_ID,
	error::{ContractError, ContractResult},
	msg, state,
	state::Config,
};

use cosmwasm_std::{
	to_binary, wasm_execute, Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
	Reply, Response, StdError, StdResult, SubMsg, WasmMsg,
};
use cw20::{Cw20Contract, Cw20ExecuteMsg};
use cw_xc_asset_registry::{contract::external_query_lookup_asset, msg::AssetReference};
use cw_xc_interpreter::contract::{
	XCVM_INTERPRETER_EVENT_DATA_ORIGIN, XCVM_INTERPRETER_EVENT_PREFIX,
};
use cw_xc_utils::DefaultXCVMProgram;
use xc_core::{CallOrigin, Displayed, Funds, InterpreterOrigin};

fn transfer_from_user(
	deps: &DepsMut,
	self_address: Addr,
	user: Addr,
	funds: Vec<Coin>,
	assets: &Funds<Displayed<u128>>,
) -> ContractResult<Vec<CosmosMsg>> {
	let config = Config::load(deps.storage)?;
	let mut transfers = Vec::with_capacity(assets.0.len());
	for (asset, Displayed(amount)) in assets.0.iter() {
		let reference =
			external_query_lookup_asset(deps.querier, config.registry_address.to_string(), *asset)?;
		match reference {
			AssetReference::Native { denom } => {
				let Coin { amount: provided_amount, .. } = funds
					.iter()
					.find(|c| c.denom == denom)
					.ok_or(ContractError::InsufficientFunds)?;
				if u128::from(*provided_amount) != *amount {
					return Err(ContractError::InsufficientFunds)?
				}
			},
			AssetReference::Virtual { cw20_address } =>
				transfers.push(Cw20Contract(cw20_address).call(Cw20ExecuteMsg::TransferFrom {
					owner: user.to_string(),
					recipient: self_address.to_string(),
					amount: amount.clone().into(),
				})?),
		}
	}
	Ok(transfers)
}

/// Handles request to execute an [`XCVMProgram`].
///
/// This is the entry point for executing a program from a user.  Handling
pub(crate) fn handle_execute_program(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	salt: Vec<u8>,
	program: DefaultXCVMProgram,
	assets: Funds<Displayed<u128>>,
) -> ContractResult<Response> {
	let self_address = env.contract.address;
	let call_origin = CallOrigin::Local { user: info.sender.clone() };
	let transfers =
		transfer_from_user(&deps, self_address.clone(), info.sender, info.funds, &assets)?;
	let msg = wasm_execute(
		self_address,
		&msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, salt, program, assets },
		Default::default(),
	)?;
	Ok(Response::default().add_messages(transfers).add_message(msg))
}

/// Handle a request to execute a [`XCVMProgram`].
/// Only the gateway is allowed to dispatch such operation.
/// The gateway must ensure that the `CallOrigin` is valid as the router does not do further
/// checking on it.
pub(crate) fn handle_execute_program_privilleged(
	_: common::auth::Contract,
	deps: DepsMut,
	env: Env,
	call_origin: CallOrigin,
	salt: Vec<u8>,
	program: DefaultXCVMProgram,
	assets: Funds<Displayed<u128>>,
) -> ContractResult<Response> {
	let config = Config::load(deps.storage)?;
	let interpreter_origin =
		InterpreterOrigin { user_origin: call_origin.user(config.network_id), salt };
	let interpreter = state::INTERPRETERS.may_load(deps.storage, interpreter_origin.clone())?;
	if let Some(state::Interpreter { address }) = interpreter {
		// There is already an interpreter instance, so all we do is fund the interpreter, then
		// add a callback to it
		let response = send_funds_to_interpreter(deps.as_ref(), address.clone(), assets)?;
		let wasm_msg = wasm_execute(
			address.clone(),
			&cw_xc_interpreter::msg::ExecuteMsg::Execute {
				relayer: call_origin.relayer().clone(),
				program,
			},
			vec![],
		)?;
		Ok(response
			.add_event(
				common::make_event("route.execute")
					.add_attribute("interpreter", address.into_string()),
			)
			.add_message(wasm_msg))
	} else {
		// First, add a callback to instantiate an interpreter (which we later get the result
		// and save it)
		let instantiate_msg: CosmosMsg = WasmMsg::Instantiate {
			// router is the default admin of a contract
			admin: Some(env.contract.address.clone().into_string()),
			code_id: config.interpreter_code_id,
			msg: to_binary(&cw_xc_interpreter::msg::InstantiateMsg {
				gateway_address: env.contract.address.clone().into_string(),
				registry_address: config.registry_address.into(),
				interpreter_origin: interpreter_origin.clone(),
			})?,
			funds: vec![],
			label: format!(
				"xcvm-interpreter-{}-{}-{}",
				u32::from(interpreter_origin.user_origin.network_id),
				hex::encode::<Vec<u8>>(interpreter_origin.user_origin.user_id.into()),
				hex::encode(&interpreter_origin.salt)
			),
		}
		.into();

		let interpreter_instantiate_submessage =
			SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_INTERPRETER_REPLY_ID);
		// Secondly, call itself again with the same parameters, so that this functions goes
		// into `Ok` state and properly executes the interpreter
		let self_call_message: CosmosMsg = wasm_execute(
			env.contract.address,
			&cw_xc_common::gateway::ExecuteMsg::ExecuteProgramPrivileged {
				call_origin: call_origin.clone(),
				salt: interpreter_origin.salt,
				program,
				assets,
			},
			vec![],
		)?
		.into();
		Ok(Response::new()
			.add_event(common::make_event("route.create"))
			.add_submessage(interpreter_instantiate_submessage)
			.add_message(self_call_message))
	}
}

/// Transfer funds attached to a [`XCVMProgram`] before dispatching the program to the interpreter.
fn send_funds_to_interpreter(
	deps: Deps,
	interpreter_address: Addr,
	funds: Funds<Displayed<u128>>,
) -> StdResult<Response> {
	let mut response = Response::new();
	let registry_address = state::Config::load(deps.storage)?.registry_address.into_string();
	let interpreter_address = interpreter_address.into_string();
	for (asset_id, Displayed(amount)) in funds.0 {
		// We ignore zero amounts
		if amount == 0 {
			continue
		}

		let reference =
			external_query_lookup_asset(deps.querier, registry_address.clone(), asset_id)?;
		response = match reference {
			AssetReference::Native { denom } => response.add_message(BankMsg::Send {
				to_address: interpreter_address.clone(),
				amount: vec![Coin::new(amount, denom)],
			}),
			AssetReference::Virtual { cw20_address } => {
				let contract = Cw20Contract(cw20_address);
				response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
					recipient: interpreter_address.clone(),
					amount: amount.into(),
				})?)
			},
		};
	}
	Ok(response)
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
	let interpreter_address = deps.api.addr_validate(&address)?;

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
		cw_xc_common::shared::decode_base64::<_, InterpreterOrigin>(interpreter_origin.as_str())?;

	let interpreter = state::Interpreter { address: interpreter_address };
	state::INTERPRETERS.save(deps.storage, interpreter_origin, &interpreter)?;

	Ok(Response::new())
}
