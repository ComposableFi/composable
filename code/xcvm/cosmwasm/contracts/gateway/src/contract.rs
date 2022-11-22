extern crate alloc;

use crate::{
	common::ensure_admin,
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{CONFIG, IBC_NETWORK_CHANNEL},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;

pub const CONTRACT_NAME: &str = "composable:xcvm-gateway";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const XCVM_GATEWAY_EVENT_PREFIX: &str = "xcvm.gateway";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	mut msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	msg.config.admin = deps.api.addr_validate(msg.config.admin.as_ref())?;
	msg.config.router_address = deps.api.addr_validate(msg.config.router_address.as_ref())?;
	CONFIG.save(deps.storage, &msg.config)?;
	Ok(Response::default()
		.add_event(Event::new(XCVM_GATEWAY_EVENT_PREFIX).add_attribute("action", "instantiated")))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	_env: Env,
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
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
	Err(StdError::generic_err("not implemented"))
}
