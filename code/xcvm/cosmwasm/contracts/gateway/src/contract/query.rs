use crate::prelude::*;

use crate::{
	assets,
	error::{ContractError, Result},
	events::make_event,
	msg, state,
};

use cosmwasm_std::{
	Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, SubMsgResult,
};

use super::ibc::ics20::get_route;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> Result<Binary> {
	match msg {
		msg::QueryMsg::GetAssetById { asset_id } => assets::get_asset_by_id(deps, asset_id)
			.and_then(|asset| Ok(to_binary(&msg::GetAssetResponse { asset })?)),
		msg::QueryMsg::GetLocalAssetByReference { reference } =>
			assets::get_local_asset_by_reference(deps, reference)
				.and_then(|asset| Ok(to_binary(&msg::GetAssetResponse { asset })?)),
		msg::QueryMsg::GetIbcIcs20Route { to_network, for_asset } =>
			get_route(deps.api, deps.storage, to_network, for_asset)
				.and_then(|route| Ok(to_binary(&msg::GetIbcIcs20RouteResponse { route })?)),
	}
}
