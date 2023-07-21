use crate::msg;

use cosmwasm_std::{Addr, IbcEndpoint, IbcTimeout, StdResult, Storage};
use cw_storage_plus::{Item, Map, PrimaryKey, UniqueIndex};
use ibc_rs_scale::{
	applications::transfer::TracePrefix,
	core::ics24_host::identifier::{ChannelId, ConnectionId},
};
use serde::{Deserialize, Serialize};
use xc_core::{
	gateway::{Asset, GatewayId},
	location::ForeignAssetId,
	AssetId, Funds, IbcIcs20Sender, InterpreterOrigin, NetworkId,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub(crate) struct Config {
	/// Address of the XCVM interpreter contract code
	pub interpreter_code_id: u64,
	/// Network ID of this network
	pub network_id: NetworkId,
	/// The admin which is allowed to update the bridge list.
	pub admin: Addr,
	/// specific per chain way to send IBC ICS 20 assets
	pub ibc_ics_20_sender: Option<IbcIcs20Sender>,
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub(crate) struct ChannelInfo {
	/// id of this channel
	pub id: String,
	/// the remote channel/port we connect to
	pub counterparty_endpoint: IbcEndpoint,
	/// the connection this exists on (you can use to query client/consensus info)
	pub connection_id: String,
}

pub(crate) const IBC_CHANNEL_INFO: Map<String, ChannelInfo> = Map::new("ibc_channel_info");
pub(crate) const IBC_CHANNEL_NETWORK: Map<String, NetworkId> = Map::new("ibc_channel_network");
pub(crate) const IBC_NETWORK_CHANNEL: Map<NetworkId, String> = Map::new("ibc_network_channel");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct OtherNetworkItem {
	/// channel to use to send ics 20 tokens
	pub ics_20_channel: ChannelId,
	/// default timeout to use for direct send
	pub counterparty_timeout: IbcTimeout,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct NetworkItem {
	/// something which will be receiver on this network
	/// case of network has XCVM deployed as contract, account address is stored here
	pub gateway_to_send_to: Option<GatewayId>,
	/// Cosmos bech32 prefix per network,
	/// if there is prefix chain accounts are Cosmos SDK compatible chain
	pub cosmos_prefix: Option<String>,

	/// https://twitter.com/scvsecurity/status/1682329758020022272?s=46&t=seqlmFXCNZ42xN1cSXpKdQ
	/// what we can do to protect agains?
	pub one_hop : bool,
}

/// the connection description from first network to second
pub(crate) const NETWORK_TO_NETWORK: Map<(NetworkId, NetworkId), OtherNetworkItem> =
	Map::new("network_to_network");

/// network state shared among all networks about it
pub(crate) const NETWORK: Map<NetworkId, NetworkItem> = Map::new("network");

/// when assets to be sent to other network it should be mapped before sent
pub(crate) const NETWORK_ASSET: Map<(AssetId, NetworkId), AssetId> = Map::new("network_asset");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub(crate) struct Interpreter {
	pub address: Addr,
}

pub(crate) const INTERPRETERS: Map<InterpreterOrigin, Interpreter> = Map::new("interpreters");

pub(crate) const ASSETS: Map<AssetId, msg::Asset> = Map::new("assets");
