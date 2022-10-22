extern crate alloc;

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, XCVMInstruction, XCVMProgram},
	state::{Config, CONFIG, OWNERS},
};
use alloc::borrow::Cow;
use core::cmp::max;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, wasm_execute, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, Event, MessageInfo,
	QueryRequest, Response, StdError, StdResult, WasmQuery,
};
use cw2::set_contract_version;
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use cw_utils::ensure_from_older_version;
use num::Zero;
use serde::Serialize;
use std::collections::VecDeque;
use xcvm_asset_registry::msg::{GetAssetContractResponse, QueryMsg as AssetRegistryQueryMsg};
use xcvm_core::{cosmwasm::*, BindingValue, Displayed, Funds, Instruction, NetworkId};

const CONTRACT_NAME: &str = "composable:xcvm-interpreter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
		ExecuteMsg::Execute { program } => interpret_program(deps, env, info, program),
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

/// Interpret an XCVM program
pub fn interpret_program(
	mut deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	program: XCVMProgram,
) -> Result<Response, ContractError> {
	let mut response = Response::new();
	let instruction_len = program.instructions.len();
	let mut instruction_iter = program.instructions.into_iter().enumerate();
	while let Some((index, instruction)) = instruction_iter.next() {
		response = match instruction {
			Instruction::Call { encoded } => {
				if index >= instruction_len - 1 {
					// If the call is the final instruction, do not yield execution
					interpret_call(deps.as_ref(), &env, encoded, index, response)?
				} else {
					// If the call is not the final instruction:
					// 1. interpret the call: this will add the call to the response's
					//    submessages.
					// 2. yield the execution by adding a call to the interpreter with the
					//    rest of the instructions as XCVM program. This will make sure that
					//    previous call instruction will run first, then the rest of the program
					//    will run.
					let response = interpret_call(deps.as_ref(), &env, encoded, index, response)?;
					let instructions: VecDeque<XCVMInstruction> =
						instruction_iter.map(|(_, instr)| instr).collect();
					let program = XCVMProgram { tag: program.tag, instructions };
					return Ok(response.add_message(wasm_execute(
						env.contract.address,
						&ExecuteMsg::Execute { program },
						vec![],
					)?))
				}
			},
			Instruction::Spawn { network, salt, assets, program } =>
				interpret_spawn(&deps, &env, network, salt, assets, program, response)?,
			Instruction::Transfer { to, assets } =>
				interpret_transfer(&mut deps, &env, to, assets, response)?,
		};
	}

	Ok(response.add_event(Event::new("xcvm.interpreter.executed").add_attribute(
		"program",
		core::str::from_utf8(&program.tag).map_err(|_| ContractError::InvalidProgramTag)?,
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
	encoded: Vec<u8>,
	ip: usize,
	response: Response,
) -> Result<Response, ContractError> {
	let LateCall { bindings, encoded_call } =
		serde_json_wasm::from_slice(&encoded).map_err(|_| ContractError::InvalidCallPayload)?;
	// We don't know the type of the payload, so we use `serde_json::Value`
	let cosmwasm_msg: FlatCosmosMsg<serde_json::Value> = if !bindings.is_empty() {
		let Config { user_id, registry_address, .. } = CONFIG.load(deps.storage)?;
		// `user_id` is the ID that comes from the origin chain. We don't know the length
		// of this. Hence, the maximum length of the output call, will be:
		// `max(len(user_id), len(contract_address)) * len(bindings)`
		let new_len = {
			let max_size = max(user_id.len(), env.contract.address.as_bytes().len());
			max_size * bindings.len() + encoded_call.len()
		};

		let mut formatted_call = vec![0; new_len];
		// Current index of the unformatted call
		let mut original_index: usize = 0;
		// This stores the amount of shifting we caused because of the data insertion. For example,
		// inserting a contract address "addr1234" causes 8 chars of shift. Which means index 'X' in
		// the unformatted call, will be equal to 'X + 8' in the output call.
		let mut offset: usize = 0;
		for binding in bindings {
			let (binding_index, binding) = (binding.0 as usize, binding.1);
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
			if original_index > binding_index || binding_index + 1 >= encoded_call.len() {
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
				.copy_from_slice(&encoded_call[original_index..=binding_index]);

			let data: Cow<[u8]> = match binding {
				BindingValue::Relayer => Cow::Borrowed(&user_id),
				BindingValue::This => Cow::Borrowed(env.contract.address.as_bytes()),
				BindingValue::Asset(asset_id) => {
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
				BindingValue::Ip => Cow::Owned(format!("{}", ip).into()),
			};

			formatted_call[binding_index + offset + 1..=binding_index + offset + data.len()]
				.copy_from_slice(&data);
			offset += data.len();
			original_index = binding_index + 1;
		}
		// Copy the rest of the data to the output data
		if original_index < encoded_call.len() {
			formatted_call[original_index + offset..encoded_call.len() + offset]
				.copy_from_slice(&encoded_call[original_index..]);
		}
		// Get rid of the final 0's.
		formatted_call.truncate(encoded_call.len() + offset);
		serde_json_wasm::from_slice(&formatted_call)
			.map_err(|_| ContractError::InvalidCallPayload)?
	} else {
		// We don't have any binding, just deserialize the data
		serde_json_wasm::from_slice(&encoded_call).map_err(|_| ContractError::InvalidCallPayload)?
	};

	let cosmos_msg: CosmosMsg = cosmwasm_msg.try_into().unwrap();
	Ok(response.add_message(cosmos_msg))
}

pub fn interpret_spawn(
	deps: &DepsMut,
	env: &Env,
	network: NetworkId,
	salt: Vec<u8>,
	assets: Funds,
	program: XCVMProgram,
	mut response: Response,
) -> Result<Response, ContractError> {
	#[derive(Serialize)]
	struct SpawnEvent {
		network: NetworkId,
		salt: Vec<u8>,
		assets: Funds<Displayed<u128>>,
		program: XCVMProgram,
	}

	let config = CONFIG.load(deps.storage)?;
	let registry_addr = config.registry_address.into_string();
	let mut normalized_funds = Funds::<Displayed<u128>>::empty();

	for (asset_id, amount) in assets.0 {
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
			normalized_funds.0.insert(asset_id, amount.into());
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

	let data = SpawnEvent { network, salt, assets: normalized_funds, program };

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
	to: String,
	assets: Funds,
	mut response: Response,
) -> Result<Response, ContractError> {
	let config = CONFIG.load(deps.storage)?;
	let registry_addr = config.registry_address.into_string();

	for (asset_id, amount) in assets.0 {
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
			recipient: to.clone(),
			amount: transfer_amount.into(),
		})?);
	}

	Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
	Err(StdError::generic_err("not implemented"))
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
			StaticBinding::Some(BindingValue::This),
			IndexedBinding::Some((
				[(9, BindingValue::This), (36, BindingValue::Relayer)].into(),
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
