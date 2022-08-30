use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg},
	state::{Config, UserId, CONFIG, INTERPRETERS},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	from_binary, to_binary, wasm_execute, Addr, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
	Response, StdError, StdResult, SubMsg, WasmMsg, WasmQuery,
};
use cw20::{Cw20Contract, Cw20ExecuteMsg};
use xcvm_asset_registry::msg::{GetAssetContractResponse, QueryMsg as AssetRegistryQueryMsg};
use xcvm_core::{Funds, NetworkId};
use xcvm_interpreter::msg::{
	ExecuteMsg as InterpreterExecuteMsg, InstantiateMsg as InterpreterInstantiateMsg,
};

const INSTANTIATE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	let addr = deps.api.addr_validate(&msg.registry_address)?;
	CONFIG.save(
		deps.storage,
		&Config { registry_address: addr, interpreter_code_id: msg.interpreter_code_id },
	)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	match msg {
		ExecuteMsg::Run { network_id, user_id, interpreter_execute_msg, funds } =>
			handle_run(deps, env, network_id, user_id, interpreter_execute_msg, funds),
	}
}

pub fn handle_run(
	deps: DepsMut,
	env: Env,
	network_id: NetworkId,
	user_id: UserId,
	interpreter_execute_msg: InterpreterExecuteMsg,
	funds: Funds,
) -> Result<Response, ContractError> {
	match INTERPRETERS.load(deps.storage, (network_id.0, user_id.clone())) {
		Ok(interpreter_address) => {
			let response =
				send_funds_to_interpreter(deps.as_ref(), interpreter_address.clone(), funds)?;
			let wasm_msg = wasm_execute(interpreter_address, &interpreter_execute_msg, vec![])?;
			Ok(response.add_message(wasm_msg))
		},
		Err(_) => {
			let Config { registry_address, interpreter_code_id } = CONFIG.load(deps.storage)?;
			let instantiate_msg: CosmosMsg = WasmMsg::Instantiate {
				admin: Some(env.contract.address.clone().into_string()),
				code_id: interpreter_code_id,
				msg: to_binary(&InterpreterInstantiateMsg {
					registry_address: registry_address.into_string(),
					network_id,
					user_id: user_id.clone(),
				})?,
				funds: vec![],
				label: format!("xcvm-interpreter-{}", network_id.0), /* TODO(aeryz): juno doesn't
				                                                      * allow empty label */
			}
			.into();

			let submessage = SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_REPLY_ID);
			let wasm_msg: CosmosMsg = wasm_execute(
				env.contract.address,
				&ExecuteMsg::Run { network_id, user_id, interpreter_execute_msg, funds },
				vec![],
			)?
			.into();
			Ok(Response::new().add_submessage(submessage).add_message(wasm_msg))
		},
	}
}

fn send_funds_to_interpreter(
	deps: Deps,
	interpreter_address: Addr,
	funds: Funds,
) -> StdResult<Response> {
	let mut response = Response::new();
	let registry_address = CONFIG.load(deps.storage)?.registry_address.into_string();
	let interpreter_address = interpreter_address.into_string();
	for (asset_id, amount) in funds.0 {
		let query_msg = AssetRegistryQueryMsg::GetAssetContract(asset_id.into());
		let cw20_address: GetAssetContractResponse = deps.querier.query(
			&WasmQuery::Smart {
				contract_addr: registry_address.clone(),
				msg: to_binary(&query_msg)?,
			}
			.into(),
		)?;
		let contract = Cw20Contract(cw20_address.addr.clone());

		if amount.intercept.0 == 0 {
			continue
		}

		response = response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
			recipient: interpreter_address.clone(),
			amount: amount.intercept.0.into(),
		})?);
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

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	let interpreter_address = {
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

	let router_reply: (u8, UserId) = from_binary(
		&response
			.data
			.ok_or(StdError::not_found("no data is returned from 'xcvm_interpreter'"))?,
	)?;

	INTERPRETERS.save(deps.storage, (router_reply.0, router_reply.1), &interpreter_address)?;

	Ok(Response::new())
}

#[cfg(test)]
mod tests {}
