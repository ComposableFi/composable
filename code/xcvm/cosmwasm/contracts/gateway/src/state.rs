use crate::msg;

use cosmwasm_std::{Addr, IbcEndpoint, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xc_core::{AssetId, InterpreterOrigin, NetworkId};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) struct Config {
	/// Address of the XCVM interpreter contract code
	pub interpreter_code_id: u64,
	/// Network ID of this network
	pub network_id: NetworkId,
	/// The admin which is allowed to update the bridge list.
	pub admin: Addr,
}

const CONFIG: Item<Config> = Item::new("config");

impl Config {
	pub(crate) fn load(storage: &dyn Storage) -> StdResult<Self> {
		CONFIG.load(storage)
	}

	pub(crate) fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
		CONFIG.save(storage, self)
	}
}

/// Information associated with an IBC channel.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) struct ChannelInfo {
	/// id of this channel
	pub id: ibc_rs_scale:: ,
	/// the remote channel/port we connect to
	pub counterparty_endpoint: IbcEndpoint,
	/// the connection this exists on (you can use to query client/consensus info)
	pub connection_id: String,
}

pub(crate) const IBC_CHANNEL_INFO: Map<ChannelId, ChannelInfo> = Map::new("ibc_channel_info");

/// According to XCVM protocol, it's always a 1:1 mapping between [`NetworkId`] and [`ChannelId`]
pub(crate) const IBC_NETWORK_CHANNEL: Map<NetworkId, ChannelId> = Map::new("ibc_network_channel");
pub(crate) const IBC_CHANNEL_NETWORK: Map<ChannelId, NetworkId> = Map::new("ibc_channel_network");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub(crate) struct Interpreter {
	pub address: Addr,
}

pub(crate) const INTERPRETERS: Map<InterpreterOrigin, Interpreter> = Map::new("interpreters");

pub(crate) const ASSETS: Map<AssetId, msg::AssetReference> = Map::new("assets");
