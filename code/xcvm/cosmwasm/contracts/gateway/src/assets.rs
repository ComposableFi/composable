use crate::{
	auth,
	batch::BatchResponse,
	error::{ContractError, Result},
	events::make_event,
	prelude::*,
	state::{
		self,
		assets::{ASSETS, LOCAL_ASSETS},
	},
};
use cosmwasm_std::{Deps, DepsMut};
use xc_core::{AssetId, NetworkId};

/// Adds a new asset to the registry; errors out if asset already exists.
pub(crate) fn force_asset(_: auth::Admin, deps: DepsMut, msg: AssetItem) -> Result<BatchResponse> {
	let config = crate::state::load(deps.storage)?;
	ASSETS.save(deps.storage, msg.asset_id, &msg)?;
	if msg.network_id == config.network_id {
		LOCAL_ASSETS.save(deps.storage, msg.local.clone(), &msg)?;
	}
	Ok(BatchResponse::new().add_event(
		make_event("assets.forced")
			.add_attribute("asset_id", msg.asset_id.to_string())
			.add_attribute("denom", msg.denom()),
	))
}

/// Fetches information about given asset.
pub(crate) fn get_asset_by_id(deps: Deps, asset_id: AssetId) -> Result<AssetItem> {
	ASSETS.may_load(deps.storage, asset_id)?.ok_or(ContractError::AssetNotFound)
}

/// Fetches information about given asset by its local reference.
pub(crate) fn get_local_asset_by_reference(
	deps: Deps,
	reference: AssetReference,
) -> Result<AssetItem> {
	LOCAL_ASSETS
		.may_load(deps.storage, reference)?
		.ok_or(ContractError::AssetNotFound)
}

/// Removes an existing asset from the registry; errors out if asset doesn’t
/// exist.
pub(crate) fn force_remove_asset(
	_: auth::Auth<auth::policy::Admin>,
	deps: DepsMut<'_>,
	asset_id: AssetId,
) -> std::result::Result<BatchResponse, ContractError> {
	let config = crate::state::load(deps.storage)?;
	let asset = ASSETS.load(deps.storage, asset_id)?;
	ASSETS.remove(deps.storage, asset_id);
	if asset.network_id == config.network_id {
		LOCAL_ASSETS.remove(deps.storage, asset.local);
	}
	Ok(BatchResponse::new()
		.add_event(make_event("assets.removed").add_attribute("asset_id", asset_id.to_string())))
}

pub(crate) fn force_asset_to_network_map(
	_: auth::Admin,
	deps: DepsMut,
	this_asset: AssetId,
	other_network: NetworkId,
	other_asset: AssetId,
) -> Result<BatchResponse> {
	state::assets::NETWORK_ASSET.save(deps.storage, (this_asset, other_network), &other_asset)?;
	Ok(BatchResponse::new().add_event(
		make_event("assets.forced_asset_to_network_map")
			.add_attribute("this_asset", this_asset.to_string())
			.add_attribute("other_asset", other_asset.to_string()),
	))
}
