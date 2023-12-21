use cosmwasm_std::{BlockInfo, IbcTimeout};
use ibc_rs_scale::core::ics24_host::identifier::ChannelId;

use crate::{
	prelude::*,
	service::dex::ExchangeItem,
	transport::ibc::{ChannelInfo, IbcIcs20Sender},
	AssetId, NetworkId,
};

type EthAddress = eth_primitive_types::H160;

/// Version of IBC channels used by the gateway.
pub const IBC_VERSION: &str = "xcvm-v0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct OsmosisIbcHooks {
	pub callback: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct PFM {}

/// if chain has IBC SDK callbacks enabled
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct Adr08IbcCallbacks {}

/// what features/modules/version enabled/installed/configured
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct Ics20Features {
	/// if it is exists, chain has that enabled
	pub wasm_hooks: Option<OsmosisIbcHooks>,
	pub ibc_callbacks: Option<Adr08IbcCallbacks>,
	pub pfm: Option<PFM>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(all(feature = "std", not(feature = "substrate")), derive(JsonSchema))]
pub enum ForeignAssetId {
	IbcIcs20(PrefixedDenom),
	#[cfg(feature = "substrate")]
	Xcm(xcm::VersionedMultiLocation),
}

#[cfg(feature = "substrate")]
impl parity_scale_codec::MaxEncodedLen for ForeignAssetId {
	fn max_encoded_len() -> usize {
		2048
	}
}

#[cfg(feature = "substrate")]
impl From<xcm::VersionedMultiLocation> for ForeignAssetId {
	fn from(this: xcm::VersionedMultiLocation) -> Self {
		Self::Xcm(this)
	}
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
	/// no prefix, pure Ethereum EVM
	EthEvm,
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

/// relative timeout to CW/IBC-rs time.
/// very small, assumed messages are arriving fast enough, like less than hours
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Encode, Decode)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum RelativeTimeout {
	/// Timeout is relative to the current block timestamp of counter party
	Seconds(u16),
}

impl RelativeTimeout {
	pub fn absolute(&self, block: BlockInfo) -> IbcTimeout {
		match self {
			RelativeTimeout::Seconds(seconds) =>
				IbcTimeout::with_timestamp(block.time.plus_seconds(*seconds as u64)),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct OtherNetworkItem {
	pub ics_20: Option<IcsPair>,
	/// default timeout to use for direct send
	pub counterparty_timeout: RelativeTimeout,
	/// if there is ICS27 IBC channel opened
	pub ics27_channel: Option<ChannelInfo>,
	/// if true, than will use shortcuts
	/// for example,
	/// if program transfer only program will just use native transfer
	/// or if connection supports exchange, it will use exchange
	/// default is false if target chain has CVM gateway
	pub use_shortcut: Option<bool>,
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

	ForceAssetToNetworkMap {
		this_asset: AssetId,
		other_network: NetworkId,
		other_asset: AssetId,
	},

	ForceExchange(ExchangeItem),

	/// Message sent by an admin to remove an asset from registry.
	ForceRemoveAsset {
		asset_id: AssetId,
	},

	// https://github.com/CosmWasm/cosmwasm/discussions/1814
	/// short cut to rollout config faster
	Force(Vec<ConfigSubMsg>),

	/// instantiates default interpreter on behalf of user
	/// `salt` - human string, converted to hex or base64 depending on implementation
	ForceInstantiate {
		user_origin: Addr,
		#[serde(skip_serializing_if = "String::is_empty", default)]
		salt: String,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct InstantiateMsg(pub HereItem);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct HereItem {
	/// Network ID of this network where contract is deployed
	pub network_id: NetworkId,
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
		/// CVM interpreter contract code
		interpreter_code_id: u64,
		/// admin of everything
		admin: Addr,
	},
	Evm {
		contract: EthAddress,
		admin: EthAddress,
	},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]

pub struct AssetItem {
	pub asset_id: AssetId,
	/// network id on which this asset id can be used locally
	pub network_id: NetworkId,
	pub local: AssetReference,
	/// if asset was bridged, it would have way to identify bridge/source/channel
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

/// Definition of an asset native to some chain to operate on.
/// For example for Cosmos CW and EVM chains both CW20 and ERC20 can be actual.
/// So if asset is local or only remote to some chain depends on context of network or connection.
/// this design leads to some dummy matches, but in general unifies code (so that if one have to
/// solve other chain route it can)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum AssetReference {
	Native { denom: String },
	Cw20 { contract: Addr },
	Erc20 { contract: EthAddress },
}

impl AssetReference {
	pub fn denom(&self) -> String {
		match self {
			AssetReference::Native { denom } => denom.clone(),
			AssetReference::Cw20 { contract } => ["cw20:", contract.as_str()].concat(),
			AssetReference::Erc20 { contract } => ["erc20:", &contract.to_string()].concat(),
		}
	}
}

#[cfg(feature = "cosmwasm")]
impl cw_storage_plus::PrimaryKey<'_> for AssetReference {
	type Prefix = ();
	type SubPrefix = ();
	type Suffix = ();
	type SuperSuffix = ();

	#[inline]
	fn key(&self) -> Vec<cw_storage_plus::Key<'_>> {
		use cw_storage_plus::Key;
		let (tag, value) = match self {
			AssetReference::Native { denom } => (0, denom.as_bytes()),
			AssetReference::Cw20 { contract } => (1, contract.as_bytes()),
			AssetReference::Erc20 { contract } => (2, contract.as_bytes()),
		};
		vec![Key::Val8([tag]), Key::Ref(value)]
	}
}
