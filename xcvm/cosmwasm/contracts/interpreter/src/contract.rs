#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, Event, MessageInfo, QueryRequest, Response,
	StdError, StdResult, WasmQuery,
};
use serde::Serialize;

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, QueryMsg, XCVMProgram},
	state::{Config, CONFIG},
};
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use num::Zero;
use xcvm_asset_registry::msg::{GetAssetContractResponse, QueryMsg as AssetRegistryQueryMsg};
use xcvm_core::{Funds, Instruction, NetworkId};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, StdError> {
	let registry_address = deps.api.addr_validate(&msg.registry_address)?;
	let config = Config { registry_address };
	CONFIG.save(deps.storage, &config)?;

	Ok(Response::new().add_event(
		Event::new("xcvm.interpreter.instantiated")
			.add_attribute(
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
	match msg {
		ExecuteMsg::Execute { program } => interpret_program(deps, env, info, program),
	}
}

pub fn interpret_program(
	mut deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	program: XCVMProgram,
) -> Result<Response, ContractError> {
	let mut response = Response::new();

	for instruction in program.instructions {
		response = match instruction {
			Instruction::Call { encoded } => interpret_call(encoded, response),
			Instruction::Spawn { network, salt, assets, program } =>
				interpret_spawn(network, salt, assets, program, response),
			Instruction::Transfer { to, assets } =>
				interpret_transfer(&mut deps, to, assets, response),
		}?;
	}

	Ok(response.add_event(
		Event::new("xcvm.interpreter.executed")
			.add_attribute(
				"program",
				core::str::from_utf8(&program.tag).map_err(|_| ContractError::InvalidProgramTag)?,
			),
	))
}

pub fn interpret_call(encoded: Vec<u8>, response: Response) -> Result<Response, ContractError> {
	let cosmos_msg: CosmosMsg =
		serde_json_wasm::from_slice(&encoded).map_err(|_| ContractError::InvalidCallPayload)?;

	Ok(response.add_message(cosmos_msg))
}

pub fn interpret_spawn(
	network: NetworkId,
	salt: Vec<u8>,
	assets: Funds,
	program: XCVMProgram,
	response: Response,
) -> Result<Response, ContractError> {
	#[derive(Serialize)]
	struct SpawnEvent {
		network: NetworkId,
		salt: Vec<u8>,
		assets: Funds,
		program: XCVMProgram,
	}

	let data = SpawnEvent { network, salt, assets, program };

	Ok(response.add_event(
		Event::new("xcvm.interpreter.spawn").add_attribute(
			"program",
			serde_json_wasm::to_string(&data).map_err(|_| ContractError::DataSerializationError)?,
		),
	))
}

pub fn interpret_transfer(
	deps: &mut DepsMut,
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
					msg: to_binary(&Cw20QueryMsg::Balance { address: to.clone() })?,
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
	use std::collections::BTreeMap;

	use crate::msg::XCVMInstruction;

	use super::*;
	use cosmwasm_std::{
		testing::{mock_dependencies, mock_env, mock_info, MockQuerier},
		wasm_execute, Addr, ContractResult, QuerierResult, SystemResult,
	};
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
			Config { registry_address: Addr::unchecked("addr") }
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

		let cosmos_msg: CosmosMsg =
			wasm_execute("1234", &"hello world".to_string(), vec![]).unwrap().into();
		let msg = serde_json_wasm::to_string(&cosmos_msg).unwrap();

		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Call { encoded: msg.as_bytes().into() }].into(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
			.unwrap();
		assert_eq!(res.messages[0].msg, cosmos_msg);
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
		assert_eq!(res.events[0], Event::new("xcvm.interpreter.spawn").add_attribute("program", r#"{"network":1,"salt":[],"assets":{},"program":{"tag":[],"instructions":[{"call":{"encoded":[]}}]}}"#.to_string()));
	}
}
