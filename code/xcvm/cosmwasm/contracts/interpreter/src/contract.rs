extern crate alloc;

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{Config, CONFIG, IP_REGISTER, OWNERS, RESULT_REGISTER},
};
use alloc::borrow::Cow;
use core::cmp::max;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, wasm_execute, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, Event, MessageInfo,
	QueryRequest, Reply, Response, StdError, StdResult, SubMsg, WasmQuery,
};
use cw2::set_contract_version;
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use cw_utils::ensure_from_older_version;
use num::Zero;
use prost::Message;
use serde::Serialize;
use std::collections::VecDeque;
use xcvm_asset_registry::msg::{GetAssetContractResponse, QueryMsg as AssetRegistryQueryMsg};
use xcvm_core::{cosmwasm::*, Amount, Displayed, Funds, Register};
use xcvm_proto as proto;

const CONTRACT_NAME: &str = "composable:xcvm-interpreter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CALL_ID: u64 = 1;
const SELF_CALL_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

	let registry_address = deps.api.addr_validate(&msg.registry_address)?;
	let config =
		Config { registry_address, network_id: msg.network_id, user_id: msg.user_id.clone() };
	CONFIG.save(deps.storage, &config)?;
	// Save the caller as owner, in this case it is `router`
	OWNERS.save(deps.storage, info.sender, &())?;

	Ok(Response::new().add_event(
		Event::new("xcvm.interpreter.instantiated").add_attribute(
			"data",
			to_binary(&(msg.network_id.0, msg.user_id))?.to_base64().as_str(),
		),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	assert_owner(deps.as_ref(), &info.sender)?;
	match msg {
		ExecuteMsg::Execute { program } => initiate_execution(deps, env, program),
		ExecuteMsg::_SelfExecute { program } =>
		// _SelfExecute should be called by interpreter itself
			if env.contract.address != info.sender {
				Err(ContractError::NotAuthorized)
			} else {
				let program =
					proto::Program::decode(program).map_err(|_| ContractError::InvalidProgram)?;
				interpret_program(deps, env, info, program)
			},
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
	// Already only callable by the admin of the contract, so no need to `assert_owner`
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

fn assert_owner(deps: Deps, owner: &Addr) -> Result<(), ContractError> {
	if OWNERS.has(deps.storage, owner.clone()) {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}

pub fn initiate_execution(
	deps: DepsMut,
	env: Env,
	program: VecDeque<u8>,
) -> Result<Response, ContractError> {
	IP_REGISTER.save(deps.storage, &0)?;
	Ok(Response::default().add_submessage(SubMsg::reply_on_error(
		wasm_execute(env.contract.address, &ExecuteMsg::_SelfExecute { program }, Vec::new())?,
		SELF_CALL_ID,
	)))
}

/// Interpret an XCVM program
pub fn interpret_program(
	mut deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	program: proto::Program,
) -> Result<Response, ContractError> {
	let mut response = Response::new();
	let instructions = program.instructions.ok_or(ContractError::InvalidProgram)?.instructions;
	let instruction_len = instructions.len();
	let mut instruction_iter = instructions.into_iter().enumerate();
	let mut ip = IP_REGISTER.load(deps.storage)?;
	while let Some((index, instruction)) = instruction_iter.next() {
		let instruction = instruction.instruction.ok_or(ContractError::InvalidProgram)?;
		response = match instruction {
			proto::instruction::Instruction::Call(proto::Call { payload, bindings }) => {
				let bindings = bindings.ok_or(ContractError::InvalidProgram)?;
				if index >= instruction_len - 1 {
					// If the call is the final instruction, do not yield execution
					interpret_call(
						deps.as_ref(),
						&env,
						bindings.bindings,
						payload,
						ip as usize,
						response,
					)?
				} else {
					// If the call is not the final instruction:
					// 1. interpret the call: this will add the call to the response's
					//    submessages.
					// 2. yield the execution by adding a call to the interpreter with the
					//    rest of the instructions as XCVM program. This will make sure that
					//    previous call instruction will run first, then the rest of the program
					//    will run.
					let _response = interpret_call(
						deps.as_ref(),
						&env,
						bindings.bindings,
						payload,
						index,
						response,
					)?;

					// TODO(aeryz): Build proto here
					/*
					let instructions: VecDeque<XCVMInstruction> =
						instruction_iter.map(|(_, instr)| instr).collect();
					let program = XCVMProgram { tag: program.tag, instructions };
					return Ok(response.add_message(wasm_execute(
						env.contract.address,
						&ExecuteMsg::Execute { program },
						vec![],
					)?))
					*/
					todo!()
				}
			},
			proto::instruction::Instruction::Spawn(ctx) =>
				interpret_spawn(&deps, &env, ctx, response)?,
			proto::instruction::Instruction::Transfer(proto::Transfer { assets, account_type }) => {
				let account_type = account_type.ok_or(ContractError::InvalidProgram)?;
				interpret_transfer(&mut deps, &env, account_type, assets, response)?
			},
			instr => return Err(ContractError::InstructionNotSupported(format!("{:?}", instr))),
		};
		ip += 1;
	}

	IP_REGISTER.save(deps.storage, &ip)?;

	Ok(response.add_event(Event::new("xcvm.interpreter.executed").add_attribute(
		"program",
		"tag", /* TODO(aeryz): tag
		       * core::str::from_utf8(&program.tag).map_err(|_|
		       * ContractError::InvalidProgramTag)?, */
	)))
}

/// Interpret the `Call` instruction
/// * `encoded`: JSON-encoded `LateCall` as bytes
///
/// Late-bindings are actually done in this function. If our XCVM SDK is not used,
/// make sure that indices in the `LateCall` is sorted in an ascending order.
pub fn interpret_call(
	deps: Deps,
	env: &Env,
	bindings: Vec<proto::Binding>,
	payload: Vec<u8>,
	_ip: usize,
	response: Response,
) -> Result<Response, ContractError> {
	// We don't know the type of the payload, so we use `serde_json::Value`
	let cosmwasm_msg: FlatCosmosMsg<serde_json::Value> = if !bindings.is_empty() {
		let Config { user_id, registry_address, .. } = CONFIG.load(deps.storage)?;
		// `user_id` is the ID that comes from the origin chain. We don't know the length
		// of this. Hence, the maximum length of the output call, will be:
		// `max(len(user_id), len(contract_address)) * len(bindings)`
		let new_len = {
			let max_size = max(user_id.len(), env.contract.address.as_bytes().len());
			max_size * bindings.len() + payload.len()
		};

		let mut formatted_call = vec![0; new_len];
		// Current index of the unformatted call
		let mut original_index: usize = 0;
		// This stores the amount of shifting we caused because of the data insertion. For example,
		// inserting a contract address "addr1234" causes 8 chars of shift. Which means index 'X' in
		// the unformatted call, will be equal to 'X + 8' in the output call.
		let mut offset: usize = 0;
		for binding in bindings {
			let binding_index = binding.position as usize;
			let binding = binding
				.binding_value
				.ok_or(ContractError::InvalidProgram)?
				.r#type
				.ok_or(ContractError::InvalidProgram)?;
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
				proto::binding_value::Type::Relayer(_) => Cow::Borrowed(&user_id),
				proto::binding_value::Type::Self_(_) =>
					Cow::Borrowed(env.contract.address.as_bytes()),
				proto::binding_value::Type::AssetId(proto::AssetId { asset_id }) => {
					let query_msg = AssetRegistryQueryMsg::GetAssetContract(asset_id.into());

					let response: GetAssetContractResponse = deps.querier.query(
						&WasmQuery::Smart {
							contract_addr: registry_address.clone().into_string(),
							msg: to_binary(&query_msg)?,
						}
						.into(),
					)?;

					Cow::Owned(response.addr.into_string().into())
				},
				proto::binding_value::Type::Result(_) => Cow::Owned(
					serde_json_wasm::to_vec(&RESULT_REGISTER.load(deps.storage)?)
						.map_err(|_| ContractError::DataSerializationError)?,
				),
				_ => return Err(ContractError::InvalidBindings),
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

	let cosmos_msg: CosmosMsg = cosmwasm_msg.try_into().unwrap();
	Ok(response.add_submessage(SubMsg::reply_on_success(cosmos_msg, CALL_ID)))
}

pub fn interpret_spawn(
	deps: &DepsMut,
	env: &Env,
	ctx: proto::Spawn,
	mut response: Response,
) -> Result<Response, ContractError> {
	#[derive(Serialize)]
	struct SpawnEvent {
		network: u32,
		salt: u64,
		assets: Funds<Displayed<u128>>,
		program: Vec<u8>,
	}

	let config = CONFIG.load(deps.storage)?;
	let registry_addr = config.registry_address.into_string();
	let mut normalized_funds = Funds::<Displayed<u128>>::empty();

	for asset in ctx.assets {
		let asset_id = asset.asset_id.ok_or(ContractError::InvalidProgram)?.asset_id;
		let amount: Amount = asset
			.balance
			.ok_or(ContractError::InvalidProgram)?
			.try_into()
			.map_err(|_| ContractError::InvalidProgram)?;
		if amount.is_zero() {
			// We ignore zero amounts
			continue
		}

		let amount = if amount.slope.0 == 0 {
			// No need to get balance from cw20 contract
			amount.intercept
		} else {
			let query_msg = AssetRegistryQueryMsg::GetAssetContract(asset_id.into());

			let cw20_address: GetAssetContractResponse = deps.querier.query(
				&WasmQuery::Smart {
					contract_addr: registry_addr.clone(),
					msg: to_binary(&query_msg)?,
				}
				.into(),
			)?;
			let response =
				deps.querier.query::<BalanceResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
					contract_addr: cw20_address.addr.clone().into_string(),
					msg: to_binary(&Cw20QueryMsg::Balance {
						address: env.contract.address.clone().into_string(),
					})?,
				}))?;
			amount.apply(response.balance.into()).into()
		};

		if amount.0 > 0 {
			normalized_funds.0.push((asset_id.into(), amount.into()));
		}
	}

	// TODO(probably call the router via a Cw20 `send` to spawn the program and do w/e required with
	// the funds)
	for (asset_id, amount) in normalized_funds.clone().0 {
		let query_msg = AssetRegistryQueryMsg::GetAssetContract(asset_id.into());
		let cw20_address: GetAssetContractResponse = deps.querier.query(
			&WasmQuery::Smart { contract_addr: registry_addr.clone(), msg: to_binary(&query_msg)? }
				.into(),
		)?;
		let contract = Cw20Contract(cw20_address.addr);
		response =
			response.add_message(contract.call(Cw20ExecuteMsg::Burn { amount: amount.0.into() })?);
	}

	let serialized_program = {
		let mut buf = Vec::new();
		let program = ctx.program.ok_or(ContractError::InvalidProgram)?;
		buf.reserve(program.encoded_len());
		program.encode(&mut buf).unwrap();
		buf
	};

	let data = SpawnEvent {
		network: ctx.network.ok_or(ContractError::InvalidProgram)?.network_id,
		salt: ctx.salt.ok_or(ContractError::InvalidProgram)?.salt,
		assets: normalized_funds,
		program: serialized_program,
	};

	Ok(response.add_event(
		Event::new("xcvm.interpreter.spawn")
			.add_attribute(
				"origin_network_id",
				serde_json_wasm::to_string(&config.network_id.0)
					.map_err(|_| ContractError::DataSerializationError)?,
			)
			.add_attribute(
				"origin_user_id",
				serde_json_wasm::to_string(&config.user_id)
					.map_err(|_| ContractError::DataSerializationError)?,
			)
			.add_attribute(
				"program",
				serde_json_wasm::to_string(&data)
					.map_err(|_| ContractError::DataSerializationError)?,
			),
	))
}

pub fn interpret_transfer(
	deps: &mut DepsMut,
	env: &Env,
	to: proto::transfer::AccountType,
	assets: Vec<proto::Asset>,
	mut response: Response,
) -> Result<Response, ContractError> {
	let config = CONFIG.load(deps.storage)?;
	let registry_addr = config.registry_address.into_string();

	let recipient = match to {
		proto::transfer::AccountType::Account(proto::Account { account }) =>
			String::from_utf8(account).map_err(|_| ContractError::InvalidAddress)?,
		proto::transfer::AccountType::Relayer(_) => todo!(),
	};

	for asset in assets {
		let asset_id = asset.asset_id.ok_or(ContractError::InvalidProgram)?.asset_id;
		let amount: Amount = asset
			.balance
			.ok_or(ContractError::InvalidProgram)?
			.try_into()
			.map_err(|_| ContractError::InvalidProgram)?;

		let query_msg = AssetRegistryQueryMsg::GetAssetContract(asset_id.into());

		let cw20_address: GetAssetContractResponse = deps.querier.query(
			&WasmQuery::Smart { contract_addr: registry_addr.clone(), msg: to_binary(&query_msg)? }
				.into(),
		)?;
		let contract = Cw20Contract(cw20_address.addr.clone());

		if amount.is_zero() {
			continue
		}

		let transfer_amount = {
			let response =
				deps.querier.query::<BalanceResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
					contract_addr: cw20_address.addr.clone().into_string(),
					msg: to_binary(&Cw20QueryMsg::Balance {
						address: env.contract.address.clone().into_string(),
					})?,
				}))?;
			amount.apply(response.balance.into())
		};

		response = response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
			recipient: recipient.clone(),
			amount: transfer_amount.into(),
		})?);
	}

	Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::Register(Register::Ip) => Ok(to_binary(&IP_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::Result) =>
			Ok(to_binary(&RESULT_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::This) => Ok(to_binary(&env.contract.address)?),
		QueryMsg::Register(Register::Relayer) => {
			let Config { user_id, .. } = CONFIG.load(deps.storage)?;
			Ok(to_binary(&user_id)?)
		},
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
	match msg.result.into_result() {
		Ok(_) => Err(StdError::generic_err("Returned OK from a reply that is called with `reply_on_error`. This should never happen")),
		Err(e) => {
			// Save the result that is returned from the sub-interpreter
			// this way, only the `RESULT_REGISTER` is persisted. All 
			// other state changes are reverted.
			RESULT_REGISTER.save(deps.storage, &Err(e))?;
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
	use crate::msg::XCVMInstruction;
	use cosmwasm_std::{
		testing::{mock_dependencies, mock_env, mock_info, MockQuerier},
		Addr, ContractResult, QuerierResult, SystemResult, WasmMsg,
	};
	use serde::Deserialize;
	use std::collections::BTreeMap;
	use xcvm_core::{Amount, AssetId, Picasso, ETH, PICA};

	const CW20_ADDR: &str = "cw20addr";
	const REGISTRY_ADDR: &str = "registryaddr";

	#[test]
	fn proper_instantiation() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {
			registry_address: "addr".to_string(),
			network_id: Picasso.into(),
			user_id: vec![],
		};
		let info = mock_info("sender", &vec![]);

		let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
		assert_eq!(0, res.messages.len());

		// Make sure that the storage is empty
		assert_eq!(
			CONFIG.load(&deps.storage).unwrap(),
			Config {
				registry_address: Addr::unchecked("addr"),
				network_id: Picasso.into(),
				user_id: vec![]
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
					to_binary(&xcvm_asset_registry::msg::GetAssetContractResponse {
						addr: Addr::unchecked(CW20_ADDR),
					})
					.unwrap(),
				))
				.into(),
			_ => panic!("Unhandled query"),
		}
	}

	#[test]
	fn execute_transfer() {
		let mut deps = mock_dependencies();
		let mut querier = MockQuerier::default();
		querier.update_wasm(wasm_querier);
		deps.querier = querier;

		let info = mock_info("sender", &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				registry_address: REGISTRY_ADDR.into(),
				network_id: Picasso.into(),
				user_id: vec![],
			},
		)
		.unwrap();

		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Transfer {
				to: "asset".into(),
				assets: Funds::from([
					(Into::<AssetId>::into(PICA), Amount::absolute(1)),
					(ETH.into(), Amount::absolute(2)),
				]),
			}]
			.into(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
			.unwrap();
		let contract = Cw20Contract(Addr::unchecked(CW20_ADDR));
		let messages = vec![
			contract
				.call(Cw20ExecuteMsg::Transfer { recipient: "asset".into(), amount: 1_u128.into() })
				.unwrap(),
			contract
				.call(Cw20ExecuteMsg::Transfer { recipient: "asset".into(), amount: 2_u128.into() })
				.unwrap(),
		];

		assert_eq!(res.messages.iter().map(|msg| msg.msg.clone()).collect::<Vec<_>>(), messages);
	}

	#[test]
	fn execute_call() {
		let mut deps = mock_dependencies();

		let info = mock_info("sender", &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				registry_address: "addr".into(),
				network_id: Picasso.into(),
				user_id: vec![],
			},
		)
		.unwrap();

		let out_msg_1 = LateCall::wasm_execute(
			StaticBinding::None(String::from("1234")),
			IndexedBinding::None(&"hello world".to_string()),
			vec![],
		)
		.unwrap();
		let msg = serde_json_wasm::to_string(&out_msg_1).unwrap();
		let instructions = vec![
			XCVMInstruction::Call { encoded: msg.as_bytes().into() },
			XCVMInstruction::Transfer { to: "1234".into(), assets: Funds::empty() },
			XCVMInstruction::Call { encoded: msg.as_bytes().into() },
			XCVMInstruction::Spawn {
				network: Picasso.into(),
				salt: vec![],
				assets: Funds::empty(),
				program: XCVMProgram { tag: vec![], instructions: vec![].into() },
			},
		];

		let program = XCVMProgram { tag: vec![], instructions: instructions.clone().into() };
		let execute_msg = ExecuteMsg::Execute {
			program: XCVMProgram { tag: vec![], instructions: instructions[1..].to_owned().into() },
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
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
				contract_addr: "cosmos2contract".into(),
				msg: to_binary(&execute_msg).unwrap(),
				funds: Vec::new(),
			})
		);
		assert_eq!(res.messages.len(), 2);
	}

	#[test]
	fn execute_spawn() {
		let mut deps = mock_dependencies();

		let info = mock_info("sender", &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				registry_address: "addr".into(),
				network_id: Picasso.into(),
				user_id: vec![],
			},
		)
		.unwrap();

		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Spawn {
				network: Picasso.into(),
				salt: vec![],
				assets: Funds(BTreeMap::new()),
				program: XCVMProgram {
					tag: vec![],
					instructions: vec![XCVMInstruction::Call { encoded: vec![] }].into(),
				},
			}]
			.into(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
			.unwrap();
		assert_eq!(res.events[0], Event::new("xcvm.interpreter.spawn").add_attribute("origin_network_id", "1").add_attribute("origin_user_id", "[]").add_attribute("program", r#"{"network":1,"salt":[],"assets":{},"program":{"tag":[],"instructions":[{"call":{"encoded":[]}}]}}"#.to_string()));
	}

	#[test]
	fn late_bindings() {
		let mut deps = mock_dependencies();

		let info = mock_info("sender", &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				registry_address: "addr".into(),
				network_id: Picasso.into(),
				user_id: vec![65, 65],
			},
		)
		.unwrap();

		#[derive(Debug, Clone, Serialize, Deserialize, Default)]
		struct TestMsg {
			part1: String,
			part2: String,
			part3: String,
		}

		let msg = LateCall::wasm_execute(
			StaticBinding::Some(BindingValue::Register(Register::This)),
			IndexedBinding::Some((
				[(9, BindingValue::This), (36, BindingValue::Register(Register::Relayer))].into(),
				TestMsg {
					part1: String::new(),
					part2: String::from("hello"),
					part3: String::new(),
				},
			)),
			Vec::new(),
		)
		.unwrap();

		let msg = serde_json_wasm::to_string(&msg).unwrap();
		let instructions = vec![XCVMInstruction::Call { encoded: msg.as_bytes().into() }];
		let program = XCVMProgram { tag: vec![], instructions: instructions.clone().into() };
		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
			.unwrap();
		let final_test_msg = TestMsg {
			part1: String::from("cosmos2contract"),
			part2: String::from("hello"),
			part3: String::from("AA"),
		};
		assert_eq!(
			CosmosMsg::Wasm(WasmMsg::Execute {
				contract_addr: String::from("cosmos2contract"),
				msg: cosmwasm_std::Binary(serde_json::to_vec(&final_test_msg).unwrap()),
				funds: Vec::new()
			}),
			res.messages[0].msg
		);
	}
}
