use crate::{
	auth,
	error::{ContractError, Result},
	events::make_event,
	msg,
	prelude::*,
	state,
};
use cosmwasm_std::{Deps, DepsMut, Response};
use xc_core::AssetId;

use crate::state::assets::*;

/// Adds a new asset to the registry; errors out if asset already exists.
pub(crate) fn force_asset(_: auth::Admin, deps: DepsMut, msg: AssetItem) -> Result {
	state::assets::ASSETS.save(deps.storage, msg.asset_id, &msg)?;
	Ok(Response::new().add_event(
		make_event("assets.forced")
			.add_attribute("asset_id", msg.asset_id.to_string())
			.add_attribute("denom", msg.denom()),
	))
}

/// Fetches information about given asset.
pub(crate) fn query_lookup(deps: Deps, asset_id: AssetId) -> Result<msg::LookupResponse> {
	ASSETS
		.may_load(deps.storage, asset_id)?
		.map(|reference| msg::LookupResponse { reference })
		.ok_or(ContractError::UnsupportedAsset)
}

/// Removes an existing asset from the registry; errors out if asset doesn’t
/// exist.
pub(crate) fn force_remove_asset(
	_: auth::Auth<auth::policy::Admin>,
	deps: DepsMut<'_>,
	asset_id: AssetId,
) -> std::result::Result<Response, ContractError> {
	ASSETS.remove(deps.storage, asset_id);
	Ok(Response::new()
		.add_event(make_event("assets.removed").add_attribute("asset_id", asset_id.to_string())))
}