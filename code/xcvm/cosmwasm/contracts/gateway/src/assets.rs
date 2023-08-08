use crate::{
	auth,
	error::{ContractError, Result},
	events::make_event,
	prelude::*,
	state::assets::{ASSETS, LOCAL_ASSETS},
};
use cosmwasm_std::{Deps, DepsMut, Response};
use xc_core::AssetId;

/// Adds a new asset to the registry; errors out if asset already exists.
pub(crate) fn force_asset(_: auth::Admin, deps: DepsMut, msg: AssetItem) -> Result {
	ASSETS.save(deps.storage, msg.asset_id, &msg)?;
	LOCAL_ASSETS.save(deps.storage, msg.local.clone(), &msg)?;
	Ok(Response::new().add_event(
		make_event("assets.forced")
			.add_attribute("asset_id", msg.asset_id.to_string())
			.add_attribute("denom", msg.denom()),
	))
}

/// Fetches information about given asset.
pub(crate) fn get_asset_by_id(deps: Deps, asset_id: AssetId) -> Result<AssetItem> {
	ASSETS.may_load(deps.storage, asset_id)?.ok_or(ContractError::UnsupportedAsset)
}

/// Fetches information about given asset by its local reference.
pub(crate) fn get_local_asset_by_reference(
	deps: Deps,
	reference: AssetReference,
) -> Result<AssetItem> {
	LOCAL_ASSETS
		.may_load(deps.storage, reference)?
		.ok_or(ContractError::UnsupportedAsset)
}

/// Removes an existing asset from the registry; errors out if asset doesn’t
/// exist.
pub(crate) fn force_remove_asset(
	_: auth::Auth<auth::policy::Admin>,
	deps: DepsMut<'_>,
	asset_id: AssetId,
) -> std::result::Result<Response, ContractError> {
	let mut resp = Response::new();
	if let Some(asset) = ASSETS.may_load(deps.storage, asset_id)? {
		ASSETS.remove(deps.storage, asset_id);
		LOCAL_ASSETS.remove(deps.storage, asset.local);
		resp = resp
			.add_event(make_event("assets.removed").add_attribute("asset_id", asset_id.to_string()))
	}
	Ok(resp)
}
