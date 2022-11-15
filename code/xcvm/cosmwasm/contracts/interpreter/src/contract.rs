extern crate alloc;

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{Config, CONFIG, IP_REGISTER, OWNERS, RELAYER_REGISTER, RESULT_REGISTER},
};
use alloc::{borrow::Cow, collections::VecDeque};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, wasm_execute, Addr, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Deps, DepsMut,
	Env, Event, MessageInfo, QueryRequest, Reply, Response, StdError, StdResult, SubMsg, WasmQuery,
};
use cw2::set_contract_version;
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use cw_utils::ensure_from_older_version;
use cw_xcvm_asset_registry::{contract::external_query_lookup_asset, msg::AssetReference};
use num::Zero;
use proto::Encodable;
use xcvm_core::{
	cosmwasm::*, Amount, BindingValue, BridgeSecurity, Destination, Displayed, Funds, NetworkId,
	Register, SpawnEvent,
};
use xcvm_proto as proto;

type XCVMInstruction = xcvm_core::Instruction<NetworkId, Vec<u8>, CanonicalAddr, Funds>;
type XCVMProgram = xcvm_core::Program<VecDeque<XCVMInstruction>>;

const CONTRACT_NAME: &str = "composable:xcvm-interpreter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CALL_ID: u64 = 1;
const SELF_CALL_ID: u64 = 2;
pub const XCVM_INTERPRETER_EVENT_PREFIX: &str = "xcvm.interpreter";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

	let registry_address = deps.api.addr_validate(&msg.registry_address)?;
	let gateway_address = deps.api.addr_validate(&msg.gateway_address)?;
	let router_address = deps.api.addr_validate(&msg.router_address)?;
	let config =
		Config { registry_address, router_address, gateway_address, user_origin: msg.user_origin };
	CONFIG.save(deps.storage, &config)?;
	// Save the caller as owner, in most cases, it is the `XCVM router`
	OWNERS.save(deps.storage, info.sender, &())?;

	Ok(Response::new().add_event(
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX)
			.add_attribute("data", cw_xcvm_utils::encode_origin_data(config.user_origin)?.as_str()),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	// Only owners can execute entrypoints of the interpreter
	assert_owner(deps.as_ref(), &env.contract.address, &info.sender)?;
	match msg {
		ExecuteMsg::Execute { relayer, program } => initiate_execution(deps, env, relayer, program),

		// ExecuteStep should be called by interpreter itself
		ExecuteMsg::ExecuteStep { relayer, program } =>
			if env.contract.address != info.sender {
				Err(ContractError::NotAuthorized)
			} else {
				let program =
					proto::decode(&program[..]).map_err(|_| ContractError::InvalidProgram)?;
				interpret_program(deps, env, info, relayer, program)
			},

		ExecuteMsg::AddOwners { owners } => add_owners(deps, owners),

		ExecuteMsg::RemoveOwners { owners } => Ok(remove_owners(deps, owners)),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
	// Already only callable by the admin of the contract, so no need to `assert_owner`
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	let _ = add_owners(deps, msg.owners)?;
	Ok(Response::default())
}

/// Check if the caller is the interpreter itself, or one of the owners
fn assert_owner(deps: Deps, self_addr: &Addr, owner: &Addr) -> Result<(), ContractError> {
	if owner == self_addr || OWNERS.has(deps.storage, owner.clone()) {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}

/// Initiate an execution by adding a `ExecuteStep` callback. This is used to be able to prepare an
/// execution by resetting the necessary registers as well as being able to catch any failures and
/// store it in the `RESULT_REGISTER`
fn initiate_execution(
	deps: DepsMut,
	env: Env,
	relayer: Addr,
	program: Vec<u8>,
) -> Result<Response, ContractError> {
  // Reset instruction pointer to zero.
	IP_REGISTER.save(deps.storage, &0)?;

  // Set the new relayer, note that the relayer that is in the register is always the last relayer that executed a program.
	RELAYER_REGISTER.save(deps.storage, &relayer)?;

	Ok(Response::default().add_submessage(SubMsg::reply_on_error(
		wasm_execute(
			env.contract.address,
			&ExecuteMsg::ExecuteStep { relayer, program },
			Default::default(),
		)?,
		SELF_CALL_ID,
	)))
}

/// Add owners who can execute entrypoints other than `ExecuteStep`
fn add_owners(deps: DepsMut, owners: Vec<Addr>) -> Result<Response, ContractError> {
	let mut event =
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "owners.added");
	for owner in owners {
		event = event.add_attribute("owner", format!("{}", owner));
		OWNERS.save(deps.storage, owner, &())?;
	}
	Ok(Response::default().add_event(event))
}

fn remove_owners(deps: DepsMut, owners: Vec<Addr>) -> Response {
	let mut event =
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "owners.removed");
	for owner in owners {
		event = event.add_attribute("owner", format!("{}", owner));
		OWNERS.remove(deps.storage, owner);
	}
	Response::default().add_event(event)
}

/// Interpret an XCVM program
pub fn interpret_program(
	mut deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	relayer: Addr,
	program: XCVMProgram,
) -> Result<Response, ContractError> {
	let mut response = Response::new();
	let instruction_len = program.instructions.len();
	let mut instruction_iter = program.instructions.into_iter().enumerate();
	let mut ip = IP_REGISTER.load(deps.storage)?;
	while let Some((index, instruction)) = instruction_iter.next() {
		response = match instruction {
			XCVMInstruction::Call { bindings, encoded } => {
				if index >= instruction_len - 1 {
					// If the call is the final instruction, do not yield execution
					interpret_call(deps.as_ref(), &env, bindings, encoded, ip as usize, response)?
				} else {
					// If the call is not the final instruction:
					// 1. interpret the call: this will add the call to the response's
					//    submessages.
					// 2. yield the execution by adding a call to the interpreter with the
					//    rest of the instructions as XCVM program. This will make sure that
					//    previous call instruction will run first, then the rest of the program
					//    will run.
					let response =
						interpret_call(deps.as_ref(), &env, bindings, encoded, index, response)?;
					let instructions: VecDeque<XCVMInstruction> =
						instruction_iter.map(|(_, instr)| instr).collect();
					let program = XCVMProgram { tag: program.tag, instructions };
					IP_REGISTER.save(deps.storage, &ip)?;
					return Ok(response.add_message(wasm_execute(
						env.contract.address,
						&ExecuteMsg::ExecuteStep {
							relayer: relayer.clone(),
							program: program.encode(),
						},
						vec![],
					)?))
				}
			},
			XCVMInstruction::Spawn { network, bridge_security, salt, assets, program } =>
				interpret_spawn(
					&deps,
					&env,
					network,
					bridge_security,
					salt,
					assets,
					program,
					response,
				)?,
			XCVMInstruction::Transfer { to, assets } =>
				interpret_transfer(&mut deps, &env, relayer.clone(), to, assets, response)?,
			instr => return Err(ContractError::InstructionNotSupported(format!("{:?}", instr))),
		};
		ip += 1;
	}

	IP_REGISTER.save(deps.storage, &ip)?;

	Ok(response.add_event(
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX)
			.add_attribute("action", "executed")
			.add_attribute(
				"program",
				core::str::from_utf8(&program.tag).map_err(|_| ContractError::InvalidProgramTag)?,
			),
	))
}

/// Interpret the `Call` instruction
/// * `encoded`: JSON-encoded `LateCall` as bytes
///
/// Late-bindings are actually done in this function. If our XCVM SDK is not used,
/// make sure that indices in the `LateCall` is sorted in an ascending order.
pub fn interpret_call(
	deps: Deps,
	env: &Env,
	bindings: Vec<(u32, BindingValue)>,
	payload: Vec<u8>,
	_ip: usize,
	response: Response,
) -> Result<Response, ContractError> {
	// We don't know the type of the payload, so we use `serde_json::Value`
	let flat_cosmos_msg: FlatCosmosMsg<serde_json::Value> = if !bindings.is_empty() {
		let Config { registry_address, .. } = CONFIG.load(deps.storage)?;
		// Len here is the maximum possible length
		let mut formatted_call =
			vec![0; env.contract.address.as_bytes().len() * bindings.len() + payload.len()];
		// Current index of the unformatted call
		let mut original_index: usize = 0;
		// This stores the amount of shifting we caused because of the data insertion. For example,
		// inserting a contract address "addr1234" causes 8 chars of shift. Which means index 'X' in
		// the unformatted call, will be equal to 'X + 8' in the output call.
		let mut offset: usize = 0;
		for (binding_index, binding) in bindings {
			let binding_index = binding_index as usize;
			// Current index of the output call
			let shifted_index = original_index + offset;

			// Check for overflow
			// * No need to check if `shifted_index` > `binding_index + offset` because
			//   `original_index > binding_index` already guarantees that
			// * No need to check if `shifted_index < formatted_call.len()` because initial
			//   allocation of `formatted_call` guarantees that even the max length can fit in.
			// * No need to check if `original_index < encoded_call.len()` because `original_index`
			//   is already less or equals to `binding_index` and we check if `binding_index` is
			//   in-bounds.
			if original_index > binding_index || binding_index + 1 >= payload.len() {
				return Err(ContractError::InvalidBindings)
			}

			// Copy everything until the index of where binding happens from original call
			// to formatted call. Eg.
			// Formatted call: `{ "hello": "" }`
			// Output call supposed to be: `{ "hello": "contract_addr" }`
			// In the first iteration, this will copy `{ "hello": "` to the formatted call.
			// SAFETY:
			//     - Two slices are in the same size for sure because `shifted_index` is
			//		 `original_index + offset` and `binding_index + offset - (shifted_index)`
			//       equals to `binding_index - original_index`.
			//     - Index accesses should not fail because we check if all indices are inbounds and
			//       also if `shifted` and `original` indices are greater than `binding_index`
			formatted_call[shifted_index..=binding_index + offset]
				.copy_from_slice(&payload[original_index..=binding_index]);

			let data: Cow<[u8]> = match binding {
				BindingValue::Register(Register::Ip) =>
					Cow::Owned(format!("{}", IP_REGISTER.load(deps.storage)?).into()),
				BindingValue::Register(Register::Relayer) =>
					Cow::Owned(format!("{}", RELAYER_REGISTER.load(deps.storage)?).into()),
				BindingValue::Register(Register::This) =>
					Cow::Borrowed(env.contract.address.as_bytes()),
				BindingValue::Register(Register::Result) => Cow::Owned(
					serde_json_wasm::to_vec(&RESULT_REGISTER.load(deps.storage)?)
						.map_err(|_| ContractError::DataSerializationError)?,
				),
				BindingValue::Asset(asset_id) => {
					let reference = external_query_lookup_asset(
						deps.querier,
						registry_address.clone().into(),
						asset_id,
					)?;
					match reference {
						AssetReference::Virtual { cw20_address } =>
							Cow::Owned(cw20_address.into_string().into()),
						AssetReference::Native { denom } => Cow::Owned(denom.into()),
					}
				},
			};

			formatted_call[binding_index + offset + 1..=binding_index + offset + data.len()]
				.copy_from_slice(&data);
			offset += data.len();
			original_index = binding_index + 1;
		}
		// Copy the rest of the data to the output data
		if original_index < payload.len() {
			formatted_call[original_index + offset..payload.len() + offset]
				.copy_from_slice(&payload[original_index..]);
		}
		// Get rid of the final 0's.
		formatted_call.truncate(payload.len() + offset);
		serde_json_wasm::from_slice(&formatted_call)
			.map_err(|_| ContractError::InvalidCallPayload)?
	} else {
		// We don't have any binding, just deserialize the data
		serde_json_wasm::from_slice(&payload).map_err(|_| ContractError::InvalidCallPayload)?
	};

	let cosmos_msg: CosmosMsg =
		flat_cosmos_msg.try_into().map_err(|_| ContractError::DataSerializationError)?;
	Ok(response
		.add_event(Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("instruction", "call"))
		.add_submessage(SubMsg::reply_on_success(cosmos_msg, CALL_ID)))
}

pub fn interpret_spawn(
	deps: &DepsMut,
	env: &Env,
	network: NetworkId,
	bridge_security: BridgeSecurity,
	salt: Vec<u8>,
	assets: Funds,
	program: XCVMProgram,
	mut response: Response,
) -> Result<Response, ContractError> {
	let Config { user_origin, registry_address, router_address, .. } = CONFIG.load(deps.storage)?;

	let registry_address = registry_address.into_string();
	let mut normalized_funds: Funds<Displayed<u128>> = Funds::empty();

	for (asset_id, amount) in assets.0 {
		if amount.is_zero() {
			// We ignore zero amounts
			continue
		}

		let reference =
			external_query_lookup_asset(deps.querier, registry_address.clone(), asset_id)?;
		let amount = {
			let transfer_amount = match &reference {
				AssetReference::Native { denom } => {
					let coin =
						deps.querier.query_balance(env.contract.address.clone(), denom.clone())?;
					coin.amount
				},
				AssetReference::Virtual { cw20_address } => {
					let rsp = deps.querier.query::<BalanceResponse>(&QueryRequest::Wasm(
						WasmQuery::Smart {
							contract_addr: cw20_address.clone().into_string(),
							msg: to_binary(&Cw20QueryMsg::Balance {
								address: env.contract.address.clone().into_string(),
							})?,
						},
					))?;
					rsp.balance
				},
			};
			Amount::absolute(amount.apply(transfer_amount.into()).into())
		};

		if !amount.is_zero() {
			// Send funds to the router
			// TODO(aeryz): Router should do the IBC transfer so the interpreter should also include
			// a submessage to the router to trigger this IBC transfer
			let asset_id: u128 = asset_id.into();
			let transfer_amount = amount.intercept.0;
			normalized_funds.0.push((asset_id.into(), transfer_amount.into()));
			response = match reference {
				AssetReference::Native { denom } => response.add_message(BankMsg::Send {
					to_address: router_address.clone().into(),
					amount: vec![Coin { denom, amount: transfer_amount.into() }],
				}),
				AssetReference::Virtual { cw20_address } => response.add_message(
					Cw20Contract(cw20_address).call(Cw20ExecuteMsg::Transfer {
						recipient: router_address.clone().into(),
						amount: transfer_amount.into(),
					})?,
				),
			};
		}
	}

	let encoded_spawn =
		SpawnEvent { network, bridge_security, salt, assets: normalized_funds, program }.encode();

	Ok(response.add_event(
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX)
			.add_attribute("instruction", "spawn")
			.add_attribute(
				"origin_network_id",
				serde_json_wasm::to_string(&user_origin.network_id)
					.map_err(|_| ContractError::DataSerializationError)?,
			)
			.add_attribute(
				"origin_user_id",
				serde_json_wasm::to_string(&user_origin.user_id)
					.map_err(|_| ContractError::DataSerializationError)?,
			)
			.add_attribute("program", Binary(encoded_spawn).to_base64()),
	))
}

pub fn interpret_transfer(
	deps: &mut DepsMut,
	env: &Env,
	relayer: Addr,
	to: Destination<CanonicalAddr>,
	assets: Funds,
	mut response: Response,
) -> Result<Response, ContractError> {
	let config = CONFIG.load(deps.storage)?;
	let registry_addr = config.registry_address.into_string();

	let recipient = match to {
		Destination::Account(account) => deps.api.addr_humanize(&account)?.into_string(),
		Destination::Relayer => relayer.into(),
	};

	for (asset_id, amount) in assets.0 {
		if amount.is_zero() {
			continue
		}

		let reference = external_query_lookup_asset(deps.querier, registry_addr.clone(), asset_id)?;
		response = match reference {
			AssetReference::Native { denom } => {
				let mut coin = deps.querier.query_balance(env.contract.address.clone(), denom)?;
				coin.amount = amount.apply(coin.amount.into()).into();
				response.add_message(BankMsg::Send {
					to_address: recipient.clone(),
					amount: vec![coin],
				})
			},
			AssetReference::Virtual { cw20_address } => {
				let contract = Cw20Contract(cw20_address.clone());
				let rsp = deps.querier.query::<BalanceResponse>(&QueryRequest::Wasm(
					WasmQuery::Smart {
						contract_addr: cw20_address.into(),
						msg: to_binary(&Cw20QueryMsg::Balance {
							address: env.contract.address.clone().into_string(),
						})?,
					},
				))?;
				let transfer_amount = amount.apply(rsp.balance.into());
				response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
					recipient: recipient.clone(),
					amount: transfer_amount.into(),
				})?)
			},
		};
	}

	Ok(response.add_event(
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("instruction", "transfer"),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::Register(Register::Ip) => Ok(to_binary(&IP_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::Result) =>
			Ok(to_binary(&RESULT_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::This) => Ok(to_binary(&env.contract.address)?),
		QueryMsg::Register(Register::Relayer) =>
			Ok(to_binary(&RELAYER_REGISTER.load(deps.storage)?)?),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
	match msg.id {
		CALL_ID => handle_call_result(deps, msg),
		SELF_CALL_ID => handle_self_call_result(deps, msg),
		id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
	}
}

fn handle_self_call_result(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	// TODO(aeryz): we can have an intermediate data type to bundle all errors with the IP_REGISTER
	match msg.result.into_result() {
		Ok(_) => Err(StdError::generic_err("Returned OK from a reply that is called with `reply_on_error`. This should never happen")),
		Err(e) => {
			// Save the result that is returned from the sub-interpreter
			// this way, only the `RESULT_REGISTER` is persisted. All
			// other state changes are reverted.
			RESULT_REGISTER.save(deps.storage, &Err(e))?;
			// Ip register should be incremented by one
			let ip_register = IP_REGISTER.load(deps.storage)?;
			IP_REGISTER.save(deps.storage, &(ip_register + 1))?;
			Ok(Response::default())
		}
	}
}

fn handle_call_result(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	RESULT_REGISTER.save(deps.storage, &Ok(response.clone()))?;
	Ok(Response::default().add_events(response.events))
}

#[cfg(test)]
mod tests {
	use super::*;
	use cosmwasm_std::{
		testing::{mock_dependencies, mock_env, mock_info, MockQuerier, MOCK_CONTRACT_ADDR},
		Addr, ContractResult, Order, QuerierResult, SystemResult, WasmMsg,
	};
	use serde::{Deserialize, Serialize};
	use xcvm_core::{
		Amount, AssetId, BindingValue, BridgeSecurity, Destination, Picasso, UserOrigin, ETH,
		MAX_PARTS, PICA, USDT,
	};

	const CW20_ADDR: &str = "cw20_addr";
	const REGISTRY_ADDR: &str = "registry_addr";
	const GATEWAY_ADDR: &str = "gateway_addr";
	const ROUTER_ADDR: &str = "router_addr";
	const BALANCE: u128 = 10_000;

	fn do_instantiate(
		deps: DepsMut,
		env: Env,
		info: MessageInfo,
	) -> Result<Response, ContractError> {
		let msg = InstantiateMsg {
			gateway_address: GATEWAY_ADDR.to_string(),
			registry_address: REGISTRY_ADDR.to_string(),
			router_address: ROUTER_ADDR.to_string(),
			user_origin: UserOrigin { network_id: Picasso.into(), user_id: vec![].into() },
		};
		instantiate(deps, env, info, msg)
	}

	fn encode_protobuf<E: prost::Message>(encodable: E) -> Vec<u8> {
		let mut buf = Vec::new();
		buf.reserve(encodable.encoded_len());
		encodable.encode(&mut buf).unwrap();
		buf
	}

	fn wasm_querier(query: &WasmQuery) -> QuerierResult {
		match query {
			WasmQuery::Smart { contract_addr, .. } if contract_addr.as_str() == CW20_ADDR =>
				SystemResult::Ok(ContractResult::Ok(
					to_binary(&cw20::BalanceResponse { balance: BALANCE.into() }).unwrap(),
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

	#[test]
	fn proper_instantiation() {
		let mut deps = mock_dependencies();

		let info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let res = do_instantiate(deps.as_mut(), mock_env(), info).unwrap();
		assert_eq!(0, res.messages.len());

		// Make sure that the storage is empty
		assert_eq!(
			CONFIG.load(&deps.storage).unwrap(),
			Config {
				gateway_address: Addr::unchecked(GATEWAY_ADDR),
				registry_address: Addr::unchecked(REGISTRY_ADDR),
				router_address: Addr::unchecked(ROUTER_ADDR),
				user_origin: UserOrigin { network_id: Picasso.into(), user_id: vec![].into() }
			}
		);

		// Make sure that the sender is added as an owner
		assert!(OWNERS.has(&deps.storage, Addr::unchecked(MOCK_CONTRACT_ADDR)));
	}

	#[test]
	fn execute_initiation() {
		let mut deps = mock_dependencies();

		let info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let _ = do_instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

		let program: proto::Program =
			XCVMProgram { tag: vec![], instructions: vec![].into() }.into();
		let program = encode_protobuf(program);
		let res = execute(
			deps.as_mut(),
			mock_env(),
			info,
			ExecuteMsg::Execute { relayer: Addr::unchecked("2"), program: program.clone() },
		)
		.unwrap();

		// Make sure IP_REGISTER is reset
		assert_eq!(IP_REGISTER.load(&deps.storage).unwrap(), 0);
		// Make sure that the correct submessage is added with the correct reply handler
		assert_eq!(
			res.messages[0],
			SubMsg::reply_on_error(
				wasm_execute(
					MOCK_CONTRACT_ADDR,
					&ExecuteMsg::ExecuteStep { relayer: Addr::unchecked("2"), program },
					Vec::new()
				)
				.unwrap(),
				SELF_CALL_ID,
			)
		);
	}

	#[test]
	fn owner_controls_work() {
		let mut deps = mock_dependencies();
		const GARBAGE_SENDER: &str = "garbage_sender";

		let garbage_info = mock_info(GARBAGE_SENDER, &vec![]);
		let legit_info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let _ = do_instantiate(deps.as_mut(), mock_env(), legit_info.clone()).unwrap();

		let program: proto::Program =
			XCVMProgram { tag: vec![], instructions: vec![].into() }.into();
		let program = encode_protobuf(program);
		// This needs to fail
		let res = execute(
			deps.as_mut(),
			mock_env(),
			garbage_info.clone(),
			ExecuteMsg::Execute { relayer: Addr::unchecked("1337"), program: program.clone() },
		)
		.unwrap_err();
		// The error type needs to be authorization
		match res {
			ContractError::NotAuthorized => {},
			res => panic!("Expected ContractError::NotAuthorized, found: {:?}", res),
		}
		// We add the garbage sender to be an owner with a legit sender, this should work
		let _ = execute(
			deps.as_mut(),
			mock_env(),
			legit_info.clone(),
			ExecuteMsg::AddOwners { owners: vec![Addr::unchecked(GARBAGE_SENDER)] },
		)
		.unwrap();
		// Now this should work as well
		let _ = execute(
			deps.as_mut(),
			mock_env(),
			garbage_info.clone(),
			ExecuteMsg::Execute { relayer: Addr::unchecked("1337"), program: program.clone() },
		)
		.unwrap();
	}

	#[test]
	fn migrate_works() {
		let mut deps = mock_dependencies();

		let info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let _ = do_instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

		// Now the contract has no owner
		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::RemoveOwners { owners: vec![Addr::unchecked(MOCK_CONTRACT_ADDR)] },
		)
		.unwrap();
		// Verify there is no owner
		assert_eq!(OWNERS.keys(&deps.storage, None, None, Order::Ascending).next(), None);

		let _ = migrate(
			deps.as_mut(),
			mock_env(),
			MigrateMsg { owners: vec![Addr::unchecked(MOCK_CONTRACT_ADDR)] },
		)
		.unwrap();
		// Verify migrate adds the new owners
		assert!(OWNERS.has(&deps.storage, Addr::unchecked(MOCK_CONTRACT_ADDR)));
	}

	#[test]
	fn execute_transfer() {
		let mut deps = mock_dependencies();
		let mut querier = MockQuerier::default();
		querier.update_wasm(wasm_querier);
		deps.querier = querier;

		let info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let _ = do_instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

		IP_REGISTER.save(deps.as_mut().storage, &0).unwrap();

		let program: proto::Program = XCVMProgram {
			tag: vec![],
			instructions: vec![
				XCVMInstruction::Transfer {
					to: Destination::Relayer,
					assets: Funds::from([
						(Into::<AssetId>::into(PICA), Amount::absolute(1)),
						(ETH.into(), Amount::absolute(2)),
						(USDT.into(), Amount::ratio(MAX_PARTS / 2)),
					]),
				},
				XCVMInstruction::Transfer {
					to: Destination::Account(vec![65; 54].into()),
					assets: Funds::from([(Into::<AssetId>::into(PICA), Amount::absolute(1))]),
				},
			]
			.into(),
		}
		.into();

		let relayer = "1337".to_string();
		let program = encode_protobuf(program);
		let res = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::ExecuteStep { relayer: Addr::unchecked(relayer.clone()), program },
		)
		.unwrap();
		let contract = Cw20Contract(Addr::unchecked(CW20_ADDR));
		let messages = vec![
			contract
				.call(Cw20ExecuteMsg::Transfer {
					recipient: relayer.clone(),
					amount: 1_u128.into(),
				})
				.unwrap(),
			contract
				.call(Cw20ExecuteMsg::Transfer {
					recipient: relayer.clone(),
					amount: 2_u128.into(),
				})
				.unwrap(),
			contract
				.call(Cw20ExecuteMsg::Transfer { recipient: relayer, amount: (BALANCE / 2).into() })
				.unwrap(),
			contract
				.call(Cw20ExecuteMsg::Transfer {
					recipient: String::from_utf8_lossy(&vec![65; 54]).to_string(),
					amount: 1_u128.into(),
				})
				.unwrap(),
		];

		assert_eq!(res.messages.into_iter().map(|msg| msg.msg).collect::<Vec<_>>(), messages);
	}

	#[test]
	fn execute_call() {
		let mut deps = mock_dependencies();

		let info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let _ = do_instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

		let relayer = Addr::unchecked("1337");
		RELAYER_REGISTER.save(deps.as_mut().storage, &relayer).unwrap();

		IP_REGISTER.save(deps.as_mut().storage, &0).unwrap();

		let late_call = LateCall::wasm_execute(
			StaticBinding::None(String::from("1234")),
			IndexedBinding::None(&"hello world".to_string()),
			vec![],
		)
		.unwrap();

		let instructions = vec![
			XCVMInstruction::Call {
				bindings: late_call.bindings.clone(),
				encoded: late_call.encoded_call.clone(),
			},
			XCVMInstruction::Transfer { to: Destination::Relayer, assets: Funds::empty() },
			XCVMInstruction::Call {
				bindings: late_call.bindings.clone(),
				encoded: late_call.encoded_call.clone(),
			},
			XCVMInstruction::Spawn {
				network: Picasso.into(),
				salt: vec![],
				bridge_security: BridgeSecurity::Deterministic,
				assets: Funds::empty(),
				program: XCVMProgram { tag: vec![], instructions: vec![].into() },
			},
		];

		let program: proto::Program =
			XCVMProgram { tag: vec![], instructions: instructions.clone().into() }.into();
		let execute_msg = ExecuteMsg::ExecuteStep {
			relayer: relayer.clone(),
			program: encode_protobuf(Into::<proto::Program>::into(XCVMProgram {
				tag: vec![],
				instructions: instructions[1..].to_owned().into(),
			})),
		};

		let res = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::ExecuteStep { relayer: relayer.clone(), program: encode_protobuf(program) },
		)
		.unwrap();
		assert_eq!(
			res.messages[0].msg,
			CosmosMsg::Wasm(WasmMsg::Execute {
				contract_addr: "1234".into(),
				msg: to_binary(&"hello world").unwrap(),
				funds: Vec::new(),
			})
		);
		assert_eq!(
			res.messages[1].msg,
			CosmosMsg::Wasm(WasmMsg::Execute {
				contract_addr: MOCK_CONTRACT_ADDR.into(),
				msg: to_binary(&execute_msg).unwrap(),
				funds: Vec::new(),
			})
		);
		assert_eq!(res.messages.len(), 2);
	}

	#[test]
	fn execute_spawn() {
		let mut deps = mock_dependencies();
		let mut querier = MockQuerier::default();
		querier.update_wasm(wasm_querier);
		deps.querier = querier;

		let info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let _ = do_instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

		let relayer = Addr::unchecked("1337");
		RELAYER_REGISTER.save(deps.as_mut().storage, &relayer).unwrap();
		IP_REGISTER.save(deps.as_mut().storage, &0).unwrap();

		let inner_program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Call { bindings: vec![], encoded: vec![] }].into(),
		};

		let funds = Funds::from([(Into::<AssetId>::into(PICA), Amount::absolute(1))]);

		let xcvm_spawn = XCVMInstruction::Spawn {
			network: Picasso.into(),
			salt: vec![],
			bridge_security: BridgeSecurity::Deterministic,
			assets: funds.clone(),
			program: inner_program.clone(),
		};

		let proto_spawn = proto::Spawn {
			network: Some(Into::<xcvm_core::NetworkId>::into(Picasso).into()),
			salt: Some(xcvm_proto::Salt { salt: vec![] }),
			security: 3,
			program: Some(inner_program.into()),
			assets: funds.0.into_iter().map(Into::into).collect(),
		};

		let program: proto::Program =
			XCVMProgram { tag: vec![], instructions: vec![xcvm_spawn.clone()].into() }.into();
		let program = encode_protobuf(program);
		let res = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::ExecuteStep { relayer, program },
		)
		.unwrap();

		assert_eq!(
			res.events[0],
			Event::new(XCVM_INTERPRETER_EVENT_PREFIX)
				.add_attribute("instruction", "spawn")
				.add_attribute("origin_network_id", "1")
				.add_attribute("origin_user_id", "[]")
				.add_attribute("program", Binary(encode_protobuf(proto_spawn)).to_base64())
		);

		// Check if burn callback is added
		let contract = Cw20Contract(Addr::unchecked(CW20_ADDR));
		assert_eq!(
			res.messages[0].msg,
			contract
				.call(Cw20ExecuteMsg::Transfer {
					recipient: ROUTER_ADDR.into(),
					amount: 1_u128.into()
				})
				.unwrap()
		);
	}

	#[test]
	fn late_bindings() {
		let mut deps = mock_dependencies();

		let info = mock_info(MOCK_CONTRACT_ADDR, &vec![]);
		let _ = do_instantiate(deps.as_mut(), mock_env(), info.clone()).unwrap();

		let relayer = Addr::unchecked("1337");
		RELAYER_REGISTER.save(deps.as_mut().storage, &relayer).unwrap();

		IP_REGISTER.save(deps.as_mut().storage, &0).unwrap();

		#[derive(Debug, Clone, Serialize, Deserialize, Default)]
		struct TestMsg {
			part1: String,
			part2: String,
			part3: String,
		}

		let late_call = LateCall::wasm_execute(
			StaticBinding::Some(BindingValue::Register(Register::This)),
			IndexedBinding::Some((
				[
					(9, BindingValue::Register(Register::This)),
					(36, BindingValue::Register(Register::Relayer)),
				]
				.into(),
				TestMsg {
					part1: String::new(),
					part2: String::from("hello"),
					part3: String::new(),
				},
			)),
			Vec::new(),
		)
		.unwrap();

		let instructions = vec![XCVMInstruction::Call {
			bindings: late_call.bindings.clone(),
			encoded: late_call.encoded_call.clone(),
		}];
		let relayer = "1337".to_string();
		let program: proto::Program =
			XCVMProgram { tag: vec![], instructions: instructions.clone().into() }.into();
		let res = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::ExecuteStep {
				relayer: Addr::unchecked(relayer.clone()),
				program: encode_protobuf(program),
			},
		)
		.unwrap();
		let final_test_msg =
			TestMsg { part1: MOCK_CONTRACT_ADDR.into(), part2: "hello".into(), part3: relayer };
		assert_eq!(
			CosmosMsg::Wasm(WasmMsg::Execute {
				contract_addr: MOCK_CONTRACT_ADDR.into(),
				msg: cosmwasm_std::Binary(serde_json::to_vec(&final_test_msg).unwrap()),
				funds: Vec::new()
			}),
			res.messages[0].msg
		);
	}
}
