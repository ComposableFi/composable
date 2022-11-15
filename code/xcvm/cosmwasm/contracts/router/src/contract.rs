extern crate alloc;

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{Config, Interpreter, CONFIG, INTERPRETERS},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, wasm_execute, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, Event,
	MessageInfo, Reply, Response, StdError, StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg};
use cw_utils::ensure_from_older_version;
use cw_xcvm_asset_registry::{contract::external_query_lookup_asset, msg::AssetReference};
use cw_xcvm_interpreter::msg::{
	ExecuteMsg as InterpreterExecuteMsg, InstantiateMsg as InterpreterInstantiateMsg,
};
use xcvm_core::{BridgeSecurity, CallOrigin, Displayed, Funds, UserOrigin};

const CONTRACT_NAME: &str = "composable:xcvm-router";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_REPLY_ID: u64 = 1;
pub const XCVM_ROUTER_EVENT_PREFIX: &str = "xcvm.router";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	let gateway_address = deps.api.addr_validate(&msg.gateway_address)?;
	let registry_address = deps.api.addr_validate(&msg.registry_address)?;
	CONFIG.save(
		deps.storage,
		&Config {
			gateway_address,
			registry_address,
			interpreter_code_id: msg.interpreter_code_id,
			network_id: msg.network_id,
		},
	)?;
	Ok(Response::default()
		.add_event(Event::new(XCVM_ROUTER_EVENT_PREFIX).add_attribute("action", "instantiated")))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	match msg {
		ExecuteMsg::ExecuteProgram { call_origin, msg, funds } =>
			handle_execute_program(deps, env, info, call_origin, msg, funds),

		// Only the local user origin is able to change it's interpreter security.
		ExecuteMsg::SetInterpreterSecurity { user_origin, bridge_security } =>
			handle_set_interpreter_security(deps, info, user_origin, bridge_security),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

/// Ensure that the `sender` is the router gateway contract.
/// This is used for privileged operations such as [`ExecuteMsg::ExecuteProgram`].
fn ensure_gateway(deps: &DepsMut, sender: &Addr) -> Result<(), ContractError> {
	let config = CONFIG.load(deps.storage)?;
	if &config.gateway_address == sender {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}

/// Ensure that the `sender` is the interpreter for the provided `user_origin`.
/// This function is used whenever we want an operation to be executable by an interpreter,
/// currently [`ExecuteMsg::SetInterpreterSecurity`].
fn ensure_interpreter(
	deps: &DepsMut,
	sender: &Addr,
	user_origin: UserOrigin,
) -> Result<(), ContractError> {
	match INTERPRETERS.load(deps.storage, user_origin) {
		Ok(Interpreter { address: Some(address), .. }) if &address == sender => Ok(()),
		_ => Err(ContractError::NotAuthorized),
	}
}

/// Handle a request to change an interpreter security level.
/// Only the interpreter instance itself is allowed to change it's security level.
/// A user is able to change it's interpreter security level by provided an [`XCVMProgram`] that
/// contains a [`XCVMInstruction::Call`] to the router contract.
fn handle_set_interpreter_security(
	deps: DepsMut,
	info: MessageInfo,
	user_origin: UserOrigin,
	security: BridgeSecurity,
) -> Result<Response, ContractError> {
	// Ensure that the sender is the interpreter for the given user origin.
	// The security of an interpreter can only be altered by the interpreter itself.
	// If a user is willing to alter the default security, he must submit an XCVM program with a
	// call to the router that does it for him.
	ensure_interpreter(&deps, &info.sender, user_origin.clone())?;

	match INTERPRETERS.load(deps.storage, user_origin.clone()) {
		Ok(Interpreter { address, .. }) =>
			INTERPRETERS.save(deps.storage, user_origin.clone(), &Interpreter { address, security }),
		Err(_) => INTERPRETERS.save(
			deps.storage,
			user_origin.clone(),
			&Interpreter { address: None, security },
		),
	}?;
	Ok(Response::default().add_event(
		Event::new(XCVM_ROUTER_EVENT_PREFIX)
			.add_attribute("action", "interpreter.setSecurity")
			.add_attribute("network_id", format!("{}", u32::from(user_origin.network_id)))
			.add_attribute("user_id", hex::encode(&user_origin.user_id))
			.add_attribute("security", format!("{}", security as u8)),
	))
}

/// Handle a request to execute a [`XCVMProgram`].
/// Only the gateway is allowed to dispatch such operation.
/// The gateway must ensure that the `CallOrigin` is valid as the router does not do further
/// checking on it.
fn handle_execute_program(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	call_origin: CallOrigin,
	msg: InterpreterExecuteMsg,
	funds: Funds<Displayed<u128>>,
) -> Result<Response, ContractError> {
	// Ensure that the sender is the gateway.
	// If a user want to directly execute payload, he must send it to it's interpreter after having
	// added himself as owner.
	ensure_gateway(&deps, &info.sender)?;

	match INTERPRETERS.load(deps.storage, call_origin.user().clone()) {
		Ok(Interpreter { address: Some(interpreter_address), security }) => {
			// Ensure that the current call origin meet the user expected security.
			call_origin
				.ensure_security(security)
				.map_err(|_| ContractError::ExpectedBridgeSecurity(security))?;

			// There is already an interpreter instance, so all we do is fund the interpreter, then
			// add a callback to it
			let response =
				send_funds_to_interpreter(deps.as_ref(), interpreter_address.clone(), funds)?;
			let wasm_msg = wasm_execute(interpreter_address, &msg, vec![])?;
			Ok(response.add_message(wasm_msg))
		},
		_ => {
			let Config { gateway_address, registry_address, interpreter_code_id, .. } =
				CONFIG.load(deps.storage)?;

			// There is no interpreter, so the bridge security must be at least `Deterministic`
			// or the message should be coming from a local origin.
			call_origin.ensure_security(BridgeSecurity::Deterministic).map_err(|_| {
				ContractError::ExpectedBridgeSecurity(BridgeSecurity::Deterministic)
			})?;

			// First, add a callback to instantiate an interpreter (which we later get the result
			// and save it)
			let instantiate_msg: CosmosMsg = WasmMsg::Instantiate {
				// router is the default admin of a contract
				admin: Some(env.contract.address.clone().into_string()),
				code_id: interpreter_code_id,
				msg: to_binary(&InterpreterInstantiateMsg {
					gateway_address: gateway_address.into(),
					registry_address: registry_address.into(),
					router_address: env.contract.address.clone().into_string(),
					user_origin: call_origin.user().clone(),
				})?,
				funds: vec![],
				label: format!(
					"xcvm-interpreter-{}-{}",
					u32::from(call_origin.user().network_id),
					hex::encode(&call_origin.user().user_id)
				),
			}
			.into();

			let interpreter_instantiate_submessage =
				SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_REPLY_ID);
			// Secondly, call itself again with the same parameters, so that this functions goes
			// into `Ok` state and properly executes the interpreter
			let self_call_message: CosmosMsg = wasm_execute(
				env.contract.address,
				&ExecuteMsg::ExecuteProgram { call_origin: call_origin.clone(), msg, funds },
				vec![],
			)?
			.into();
			Ok(Response::new()
				.add_submessage(interpreter_instantiate_submessage)
				.add_message(self_call_message))
		},
	}
}

/// Transfer funds attached to a [`XCVMProgram`] before dispatching the program to the interpreter.
fn send_funds_to_interpreter(
	deps: Deps,
	interpreter_address: Addr,
	funds: Funds<Displayed<u128>>,
) -> StdResult<Response> {
	let mut response = Response::new();
	let registry_address = CONFIG.load(deps.storage)?.registry_address.into_string();
	let interpreter_address = interpreter_address.into_string();
	for (asset_id, amount) in funds.0 {
		// We ignore zero amounts
		if amount.0 == 0 {
			continue
		}

		let reference =
			external_query_lookup_asset(deps.querier, registry_address.clone(), asset_id)?;
		response = match reference {
			AssetReference::Native { denom } => response.add_message(BankMsg::Send {
				to_address: interpreter_address.clone(),
				amount: vec![Coin::new(amount.0, denom)],
			}),
			AssetReference::Virtual { cw20_address } => {
				let contract = Cw20Contract(cw20_address);
				response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
					recipient: interpreter_address.clone(),
					amount: amount.0.into(),
				})?)
			},
		};
	}
	Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
	match msg.id {
		INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
		id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
	Err(StdError::generic_err("not implemented"))
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	let interpreter_address = {
		// Catch the default `instantiate` event which contains `_contract_address` attribute that
		// has the instantiated contract's address
		let instantiate_event = response
			.events
			.iter()
			.find(|event| event.ty == "instantiate")
			.ok_or(StdError::not_found("instantiate event not found"))?;
		deps.api.addr_validate(
			&instantiate_event
				.attributes
				.iter()
				.find(|attr| &attr.key == "_contract_address")
				.ok_or(StdError::not_found("_contract_address attribute not found"))?
				.value,
		)?
	};

	let user_origin = {
		// Interpreter provides `network_id, user_id` pair as an event for the router to know which
		// pair is instantiated
		let interpreter_event = response
			.events
			.iter()
			.find(|event| event.ty == "wasm-xcvm.interpreter.instantiated")
			.ok_or(StdError::not_found("interpreter event not found"))?;

		cw_xcvm_utils::decode_origin_data(
			interpreter_event
				.attributes
				.iter()
				.find(|attr| &attr.key == "data")
				.ok_or(StdError::not_found("no data is returned from 'xcvm_interpreter'"))?
				.value
				.as_str(),
		)?
	};

	match INTERPRETERS.load(deps.storage, user_origin.clone()) {
		Ok(Interpreter { security, .. }) => INTERPRETERS.save(
			deps.storage,
			user_origin,
			&Interpreter { address: Some(interpreter_address), security },
		)?,
		Err(_) => INTERPRETERS.save(
			deps.storage,
			user_origin,
			&Interpreter {
				security: BridgeSecurity::Deterministic,
				address: Some(interpreter_address),
			},
		)?,
	}

	Ok(Response::new())
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::collections::VecDeque;
	use cosmwasm_std::{
		testing::{mock_dependencies, mock_env, mock_info, MockQuerier, MOCK_CONTRACT_ADDR},
		wasm_execute, Addr, CanonicalAddr, ContractResult, QuerierResult, SystemResult, WasmQuery,
	};
	use prost::Message;
	use xcvm_core::{Amount, AssetId, BridgeProtocol, Destination, NetworkId, Picasso, ETH, PICA};
	use xcvm_proto as proto;
	type XCVMInstruction = xcvm_core::Instruction<NetworkId, Vec<u8>, CanonicalAddr, Funds>;
	type XCVMProgram = xcvm_core::Program<VecDeque<XCVMInstruction>>;

	const CW20_ADDR: &str = "cw20addr";
	const REGISTRY_ADDR: &str = "registry_addr";
	const GATEWAY_ADDR: &str = "gateway_addr";

	#[test]
	fn proper_instantiation() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {
			gateway_address: GATEWAY_ADDR.into(),
			registry_address: REGISTRY_ADDR.into(),
			interpreter_code_id: 1,
			network_id: Picasso.into(),
		};
		let info = mock_info(GATEWAY_ADDR, &vec![]);

		let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
		assert_eq!(0, res.messages.len());

		// Make sure that the storage is empty
		assert_eq!(
			CONFIG.load(&deps.storage).unwrap(),
			Config {
				gateway_address: Addr::unchecked(GATEWAY_ADDR),
				registry_address: Addr::unchecked(REGISTRY_ADDR),
				interpreter_code_id: 1,
				network_id: Picasso.into()
			}
		);
	}

	fn wasm_querier(query: &WasmQuery) -> QuerierResult {
		match query {
			WasmQuery::Smart { contract_addr, .. } if contract_addr.as_str() == CW20_ADDR =>
				SystemResult::Ok(ContractResult::Ok(
					to_binary(&cw20::BalanceResponse { balance: 100000_u128.into() }).unwrap(),
				)),
			WasmQuery::Smart { contract_addr, .. } if contract_addr.as_str() == REGISTRY_ADDR =>
				SystemResult::Ok(ContractResult::Ok(
					to_binary(&cw_xcvm_asset_registry::msg::LookupResponse {
						reference: AssetReference::Virtual {
							cw20_address: Addr::unchecked(CW20_ADDR),
						},
					})
					.unwrap(),
				))
				.into(),
			_ => panic!("Unhandled query"),
		}
	}

	fn encode_protobuf(program: proto::Program) -> Vec<u8> {
		let mut buf = Vec::new();
		buf.reserve(program.encoded_len());
		program.encode(&mut buf).unwrap();
		buf
	}

	#[test]
	fn execute_run_phase1() {
		let mut deps = mock_dependencies();
		let mut querier = MockQuerier::default();
		querier.update_wasm(wasm_querier);
		deps.querier = querier;

		let info = mock_info(GATEWAY_ADDR, &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				gateway_address: GATEWAY_ADDR.into(),
				registry_address: REGISTRY_ADDR.into(),
				interpreter_code_id: 1,
				network_id: Picasso.into(),
			},
		)
		.unwrap();

		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Transfer {
				to: Destination::<CanonicalAddr>::Relayer,
				assets: Funds::from([
					(Into::<AssetId>::into(PICA), Amount::absolute(1)),
					(ETH.into(), Amount::absolute(2)),
				]),
			}]
			.into(),
		};

		let funds =
			Funds::<Displayed<u128>>::from([(Into::<AssetId>::into(PICA), Displayed(1000_u128))]);

		let relayer = Addr::unchecked("13245");
		let run_msg = ExecuteMsg::ExecuteProgram {
			call_origin: CallOrigin::Remote {
				protocol: BridgeProtocol::IBC,
				relayer: relayer.as_bytes().to_vec(),
				user_origin: UserOrigin { network_id: Picasso.into(), user_id: vec![1].into() },
			},
			msg: InterpreterExecuteMsg::Execute {
				relayer: relayer.clone(),
				program: encode_protobuf(program.into()),
			},
			funds: funds.clone(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), run_msg.clone()).unwrap();

		let instantiate_msg = WasmMsg::Instantiate {
			admin: Some(MOCK_CONTRACT_ADDR.into()),
			code_id: 1,
			msg: to_binary(&InterpreterInstantiateMsg {
				registry_address: REGISTRY_ADDR.into(),
				gateway_address: GATEWAY_ADDR.into(),
				router_address: MOCK_CONTRACT_ADDR.into(),
				user_origin: UserOrigin { network_id: Picasso.into(), user_id: vec![1].into() },
			})
			.unwrap(),
			funds: vec![],
			label: "xcvm-interpreter-1-01".into(),
		};

		let execute_msg = WasmMsg::Execute {
			contract_addr: MOCK_CONTRACT_ADDR.into(),
			msg: to_binary(&run_msg).unwrap(),
			funds: vec![],
		};

		assert_eq!(res.messages[0].msg, instantiate_msg.into());
		assert_eq!(res.messages[1].msg, execute_msg.into());
	}

	#[test]
	fn execute_run_phase2() {
		let mut deps = mock_dependencies();
		let mut querier = MockQuerier::default();
		querier.update_wasm(wasm_querier);
		deps.querier = querier;

		let info = mock_info(GATEWAY_ADDR, &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				gateway_address: GATEWAY_ADDR.into(),
				registry_address: REGISTRY_ADDR.into(),
				interpreter_code_id: 1,
				network_id: Picasso.into(),
			},
		)
		.unwrap();

		INTERPRETERS
			.save(
				&mut deps.storage,
				UserOrigin { network_id: Picasso.into(), user_id: vec![].into() },
				&Interpreter {
					address: Some(Addr::unchecked("interpreter")),
					security: BridgeSecurity::Deterministic,
				},
			)
			.unwrap();

		let relayer = Addr::unchecked("1337");
		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Transfer {
				to: Destination::<CanonicalAddr>::Relayer,
				assets: Funds::from([
					(Into::<AssetId>::into(PICA), Amount::absolute(1)),
					(ETH.into(), Amount::absolute(2)),
				]),
			}]
			.into(),
		};

		let funds = Funds::<Displayed<u128>>::from([
			(Into::<AssetId>::into(PICA), Displayed(1000_u128)),
			(Into::<AssetId>::into(ETH), Displayed(2000_u128)),
		]);

		let run_msg = ExecuteMsg::ExecuteProgram {
			call_origin: CallOrigin::Remote {
				protocol: BridgeProtocol::XCM,
				relayer: relayer.as_bytes().to_vec(),
				user_origin: UserOrigin { network_id: Picasso.into(), user_id: vec![].into() },
			},
			msg: InterpreterExecuteMsg::Execute {
				relayer: relayer.clone(),
				program: encode_protobuf(program.clone().into()),
			},
			funds: funds.clone(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), run_msg.clone()).unwrap();

		let cw20_contract = Cw20Contract(Addr::unchecked(CW20_ADDR));
		let messages = vec![
			cw20_contract
				.call(Cw20ExecuteMsg::Transfer {
					recipient: "interpreter".into(),
					amount: 1000_u128.into(),
				})
				.unwrap(),
			cw20_contract
				.call(Cw20ExecuteMsg::Transfer {
					recipient: "interpreter".into(),
					amount: 2000_u128.into(),
				})
				.unwrap(),
			wasm_execute(
				"interpreter",
				&InterpreterExecuteMsg::Execute {
					relayer: relayer.clone(),
					program: encode_protobuf(program.into()),
				},
				vec![],
			)
			.unwrap()
			.into(),
		];

		messages.into_iter().enumerate().for_each(|(i, msg)| {
			assert_eq!(res.messages[i].msg, msg);
		})
	}
}
