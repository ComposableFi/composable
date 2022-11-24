extern crate alloc;

use crate::{
	common::ensure_admin,
	error::ContractError,
	ibc::handle_bridge,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{IBC_NETWORK_CHANNEL, ROUTER},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Reply, Response, StdError, SubMsg,
	WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;

pub const CONTRACT_NAME: &str = "composable:xcvm-gateway";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const XCVM_GATEWAY_EVENT_PREFIX: &str = "xcvm.gateway";
pub const XCVM_GATEWAY_INSTANTIATE_ROUTER_REPLY_ID: u64 = 0;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	mut msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	msg.config.registry_address =
		deps.api.addr_validate(&msg.config.registry_address)?.into_string();
	msg.config.admin = deps.api.addr_validate(&msg.config.admin)?.into_string();
	Ok(Response::default()
		.add_event(Event::new(XCVM_GATEWAY_EVENT_PREFIX).add_attribute("action", "instantiated"))
		.add_submessage(SubMsg::reply_on_success(
			WasmMsg::Instantiate {
				admin: Some(env.contract.address.to_string()),
				code_id: msg.config.router_code_id,
				msg: to_binary(&cw_xcvm_router::msg::InstantiateMsg {
					registry_address: msg.config.registry_address,
					interpreter_code_id: msg.config.interpreter_code_id,
					network_id: msg.config.network_id,
				})?,
				funds: Default::default(),
				label: "xcvm-router".into(),
			},
			XCVM_GATEWAY_INSTANTIATE_ROUTER_REPLY_ID,
		)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	match msg {
		ExecuteMsg::IbcSetNetworkChannel { network_id, channel_id } => {
			ensure_admin(deps.as_ref(), info.sender.as_ref())?;
			IBC_NETWORK_CHANNEL.save(deps.storage, network_id, &channel_id)?;
			Ok(Response::default().add_event(
				Event::new(XCVM_GATEWAY_EVENT_PREFIX)
					.add_attribute("action", "set_network_channel")
					.add_attribute("network_id", format!("{network_id}"))
					.add_attribute("channel_id", channel_id),
			))
		},
		ExecuteMsg::Bridge { network_id, security, salt, program, assets } =>
			handle_bridge(deps, env, info, network_id, security, salt, program, assets),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
	Err(StdError::generic_err("not implemented").into())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
	match msg.id {
		XCVM_GATEWAY_INSTANTIATE_ROUTER_REPLY_ID => handle_instantiate_reply(deps, msg),
		_ => panic!("impossible"),
	}
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	let router_address = {
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
	ROUTER.save(deps.storage, &router_address)?;
	Ok(Response::default())
}
