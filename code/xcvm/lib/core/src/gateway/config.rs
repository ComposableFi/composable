use cosmwasm_std::IbcTimeout;
use ibc_rs_scale::core::ics24_host::identifier::ChannelId;

use crate::{
	prelude::*,
	transport::ibc::{ChannelInfo, IbcIcs20Sender},
	AssetId, NetworkId,
};

/// Version of IBC channels used by the gateway.
pub const IBC_VERSION: &str = "xcvm-v0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct WasmHooks {
	pub callback: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct PFM {}

/// what features/modules/version enabled/installed/configured
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct Ics20Features {
	/// if it is exists, chain has that enabled
	pub wasm_hooks: Option<WasmHooks>,
	pub pfm: Option<PFM>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(JsonSchema))]
pub enum ForeignAssetId {
	IbcIcs20(PrefixedDenom),
}

impl From<PrefixedDenom> for ForeignAssetId {
	fn from(this: PrefixedDenom) -> Self {
		Self::IbcIcs20(this)
	}
}

/// given prefix you may form accounts from 32 bit addresses or partially identify chains
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum Prefix {
	SS58(u16),
	Bech(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct ForceNetworkToNetworkMsg {
	pub from: NetworkId,
	pub to: NetworkId,

	/// on `to` chain
	pub other: OtherNetworkItem,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct NetworkItem {
	pub network_id: NetworkId,
	/// something which will be receiver on other side
	/// case of network has XCVM deployed as contract, account address is stored here
	pub gateway: Option<GatewayId>,
	/// Account encoding type
	pub accounts: Option<Prefix>,
	pub ibc: Option<IbcEnabled>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct Ics20Channel {
	/// specific per chain way to send IBC ICS 20 assets
	pub sender: IbcIcs20Sender,
	pub features: Option<Ics20Features>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct IbcChannels {
	pub ics20: Option<Ics20Channel>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct IbcEnabled {
	pub channels: Option<IbcChannels>,
}

/// we need both, so we can unwrap
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct IcsPair {
	pub source: ChannelId,
	pub sink: ChannelId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct OtherNetworkItem {
	pub ics_20: Option<IcsPair>,
	/// default timeout to use for direct send
	pub counterparty_timeout: IbcTimeout,
	/// if there is custom IBC channel opened
	pub xcvm_channel: Option<ChannelInfo>,
}

/// cross cross chain routing requires a lot of configuration,
/// about chain executing this contract,
/// about connectivity to and of other chains (even if not connected directly)
/// and about assets and services on these chains
/// (in future block hooks and some set of host extensions/precompiles would help to get some info
/// automatically)
/// `Force` message sets the data unconditionally.  
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ConfigSubMsg {
	/// Permissioned message (gov or admin) to force set information about network contract is
	/// executed. Network can be any network or this network (so it overrides some this network
	/// parameters too)
	ForceNetwork(NetworkItem),
	/// Sets network to network connectivity/routing information
	ForceNetworkToNetwork(ForceNetworkToNetworkMsg),

	/// Permissioned message (gov or admin) to force set asset information.
	ForceAsset(AssetItem),

	/// Message sent by an admin to remove an asset from registry.
	ForceRemoveAsset { asset_id: AssetId },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct InstantiateMsg(pub HereItem);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct HereItem {
	/// Network ID of this network
	pub here_id: NetworkId,
	/// The admin which is allowed to update the bridge list.
	pub admin: Addr,
}

/// when message is sent to other side, we should identify receiver of some kind
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum GatewayId {
	CosmWasm {
		contract: Addr,
		/// Address of the XCVM interpreter contract code
		interpreter_code_id: u64,
		/// admin of everything
		admin: Addr,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]

pub struct AssetItem {
	pub asset_id: AssetId,
	pub from_network_id: NetworkId,
	pub local: AssetReference,
	pub bridged: Option<BridgeAsset>,
}

impl AssetItem {
	pub fn denom(&self) -> String {
		self.local.denom()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct BridgeAsset {
	pub location_on_network: ForeignAssetId,
}

/// Definition of an asset on this local chain to operate with
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum AssetReference {
	Native { denom: String },
	Virtual { cw20_address: Addr },
}

impl AssetReference {
	pub fn denom(&self) -> String {
		match self {
			AssetReference::Native { denom } => denom.clone(),
			AssetReference::Virtual { cw20_address } => ["cw20:", cw20_address.as_str()].concat(),
		}
	}
}
