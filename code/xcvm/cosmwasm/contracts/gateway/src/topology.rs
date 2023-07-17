//! Helps connecting identifiers into networks.
//! Allows to map asset identifiers, contracts, networks, channels, denominations from, to and on
//! each chain via contract storage, precompiles, host extensions.

use cosmwasm_std::Storage;
use xc_core::{
	gateway::{Asset, GatewayId},
	ibc::IbcRoute,
	AssetId,
};

use crate::{
	error::ContractError,
	state,
	state::{NetworkItem, OtherNetworkItem},
};

pub fn get_route(
	storage: &mut dyn Storage,
	to: xc_core::NetworkId,
	asset_id: AssetId,
) -> Result<IbcRoute, ContractError> {
	let this = state::Config::load(storage)?;
	let other: NetworkItem = state::NETWORK.load(storage, to)?;
	let this_to_other: OtherNetworkItem =
		state::NETWORK_TO_NETWORK.load(storage, (this.network_id, to))?;
	let asset: Asset = state::ASSETS.load(storage, asset_id)?;
	let to_asset: AssetId = state::NETWORK_ASSET.load(storage, (asset_id, to))?;
	let gateway_to_send_to = other.gateway_to_send_to.ok_or(ContractError::UnsupportedNetwork)?;
	let gateway_to_send_to = match gateway_to_send_to {
		GatewayId::CosmWasm(addr) => addr.to_string(),
	};
	Ok(IbcRoute {
		from_network: this.network_id,
		local_native_denom: asset.local.denom(),
		channel_to_send_to: this_to_other.ics_20_channel,
		gateway_to_send_to,
		counterparty_timeout: this_to_other.counterparty_timeout,
		ibc_ics_20_sender: this.ibc_ics_20_sender.ok_or(ContractError::UnsupportedNetwork)?,
		on_remote_asset: to_asset,
	})
}
