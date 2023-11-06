use crate::{assets, error::Result, exchange, msg, prelude::*};

use cosmwasm_std::{Binary, Deps, Env};

use super::ibc::ics20::get_this_route;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> Result<Binary> {
	match msg {
		msg::QueryMsg::GetAssetById { asset_id } => assets::get_asset_by_id(deps, asset_id)
			.and_then(|asset| Ok(to_binary(&msg::GetAssetResponse { asset })?)),
		msg::QueryMsg::GetLocalAssetByReference { reference } =>
			assets::get_local_asset_by_reference(deps, reference)
				.and_then(|asset| Ok(to_binary(&msg::GetAssetResponse { asset })?)),
		msg::QueryMsg::GetIbcIcs20Route { to_network, for_asset } =>
			get_this_route(deps.storage, to_network, for_asset)
				.and_then(|route| Ok(to_binary(&msg::GetIbcIcs20RouteResponse { route })?)),
		msg::QueryMsg::GetExchangeById { exchange_id } => exchange::get_by_id(deps, exchange_id)
			.and_then(|exchange| Ok(to_binary(&msg::GetExchangeResponse { exchange })?)),
	}
}
