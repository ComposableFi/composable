use cosmwasm_std::{Addr, IbcEndpoint};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeId, BridgeSecurity, NetworkId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
	/// Address of the XCVM registry contract
	pub registry_address: String,
	/// Address of the XCVM router.
	pub router_code_id: u64,
	/// Address of the XCVM interpreter contract code
	pub interpreter_code_id: u64,
	/// Network ID of this network
	pub network_id: NetworkId,
  /// The admin which is allowed to update the bridge list.
  pub admin: String,
}

/// Bridge following the OTP specs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Bridge {
	pub security: BridgeSecurity,
	pub address: Addr,
}

/// Information associated with an IBC channel.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ChannelInfo {
	/// id of this channel
	pub id: String,
	/// the remote channel/port we connect to
	pub counterparty_endpoint: IbcEndpoint,
	/// the connection this exists on (you can use to query client/consensus info)
	pub connection_id: String,
}

pub const ROUTER: Item<Addr> = Item::new("router");
pub const CONFIG: Item<Config> = Item::new("config");
pub const BRIDGES: Map<BridgeId, Bridge> = Map::new("bridges");

pub const IBC_CHANNEL_INFO: Map<String, ChannelInfo> = Map::new("ibc_channel_info");
pub const IBC_NETWORK_CHANNEL: Map<NetworkId, String> = Map::new("ibc_network_channel");
pub const IBC_CHANNEL_NETWORK: Map<String, NetworkId> = Map::new("ibc_channel_network");
