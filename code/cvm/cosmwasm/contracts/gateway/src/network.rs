use crate::{
	batch::BatchResponse, events::make_event, prelude::*, state::xcvm::IBC_CHANNEL_NETWORK,
};
use cosmwasm_std::{DepsMut, Storage};
use xc_core::{gateway::NetworkItem, NetworkId};

use crate::state::{self, NETWORK, NETWORK_TO_NETWORK};

use crate::error::{ContractError, Result};

pub fn load_this(storage: &dyn Storage) -> Result<NetworkItem> {
	state::load(storage)
		.and_then(|this| NETWORK.load(storage, this.network_id))
		.map_err(|_| ContractError::NetworkConfig)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct OtherNetwork {
	pub network: NetworkItem,
	pub connection: OtherNetworkItem,
}

pub fn load_other(storage: &dyn Storage, other: NetworkId) -> Result<OtherNetwork> {
	let this = state::load(storage)?;
	let other = NETWORK.load(storage, other)?;
	let connection = NETWORK_TO_NETWORK.load(storage, (this.network_id, other.network_id))?;
	Ok(OtherNetwork { network: other, connection })
}

pub(crate) fn force_network_to_network(
	_: crate::auth::Auth<crate::auth::policy::Admin>,
	deps: DepsMut,
	msg: xc_core::gateway::ForceNetworkToNetworkMsg,
) -> std::result::Result<BatchResponse, crate::error::ContractError> {
	NETWORK_TO_NETWORK.save(deps.storage, (msg.from, msg.to), &msg.other)?;
	if let Some(ibc) = msg.other.ics27_channel {
		IBC_CHANNEL_NETWORK.save(deps.storage, ibc.id.to_string(), &msg.to)?;
	}
	Ok(BatchResponse::new().add_event(
		make_event("network_to_network.forced")
			.add_attribute("to", msg.to.to_string())
			.add_attribute("from", msg.from.to_string())
			.add_attribute("ics_20", msg.other.ics_20.is_some().to_string()),
	))
}

pub(crate) fn force_network(
	_auth: crate::auth::Auth<crate::auth::policy::Admin>,
	deps: DepsMut,
	msg: NetworkItem,
) -> crate::error::Result<BatchResponse> {
	NETWORK.save(deps.storage, msg.network_id, &msg)?;
	Ok(BatchResponse::new().add_event(
		make_event("network.forced").add_attribute("network_id", msg.network_id.to_string()),
	))
}
