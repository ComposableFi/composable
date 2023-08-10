use crate::prelude::*;

use crate::{
	assets,
	error::{ContractError, Result},
	events::make_event,
	msg, state,
};

use cosmwasm_std::{
	to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, SubMsgResult,
};

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> Result<Binary> {
	match msg {
		msg::QueryMsg::GetAssetById { asset_id } => assets::get_asset_by_id(deps, asset_id),
		msg::QueryMsg::GetLocalAssetByReference { reference } =>
			assets::get_local_asset_by_reference(deps, reference),
	}
	.and_then(|asset| Ok(to_binary(&msg::GetAssetResponse { asset })?))
}