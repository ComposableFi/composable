//! we do not care modifying assets nor metadata, it is up to source chain to handle

use ibc_rs_scale::core::ics24_host::identifier::ChannelId;

use crate::{location::ForeignAssetId, prelude::*, IbcIcs20Sender};

use crate::{
	ibc::Ics20MessageHook, AssetId, CallOrigin, Displayed, Funds, InterpreterOrigin, NetworkId,
};

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.gateway";

/// Version of IBC channels used by the gateway.
pub const IBC_VERSION: &str = "xcvm-v0";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct InstantiateMsg {
	/// Address of the XCVM interpreter contract code
	pub interpreter_code_id: u64,
	/// Network ID of this network
	pub network_id: NetworkId,
	/// The admin which is allowed to update the bridge list.
	pub admin: String,
	pub ibc_ics_20_sender: Option<IbcIcs20Sender>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	IbcSetNetworkChannel {
		from: NetworkId,
		to: NetworkId,
		channel_id: ChannelId,
		/// on `to` chain
		gateway: Option<String>,
	},

	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram {
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
	},

	/// Request to execute a program on behalf of given user.
	///
	/// This can only be sent by trusted contract.  The message is
	ExecuteProgramPrivileged {
		/// The origin of the call.
		call_origin: CallOrigin,
		/// Program to execute.
		execute_program: ExecuteProgramMsg,
	},

	/// Message sent from interpreter trying to spawn program on another
	/// network.
	Bridge(BridgeMsg),

	/// Message sent by an admin to register a new asset.
	RegisterAsset(RegisterAssetMsg),

	/// Message sent by an admin to remove an asset from registry.
	UnregisterAsset {
		asset_id: AssetId,
	},

	Wasm(Ics20MessageHook),
}

/// when message is sent to other side, we should identify receiver of some kind
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum GatewayId {
	CosmWasm(Addr),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]

pub struct Asset {
	pub network_id: NetworkId,
	pub local: AssetReference,
	pub bridged: Option<BridgeAsset>,
}

impl Asset {
	pub fn denom(&self) -> String {
		self.local.denom()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct BridgeAsset {
	pub gateway: Option<GatewayId>,
	pub location_on_network: ForeignAssetId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct RegisterAssetMsg {
	pub id: AssetId,
	pub asset: Asset,
}
/// Definition of a program to be executed including its context.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct ExecuteProgramMsg {
	/// The program salt.
	pub salt: Vec<u8>,
	/// The program.
	pub program: crate::shared::DefaultXCVMProgram,
	/// Assets to fund the XCVM interpreter instance
	/// The interpreter is funded prior to execution
	pub assets: Funds<Displayed<u128>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct BridgeMsg {
	pub interpreter_origin: InterpreterOrigin,
	/// target network
	pub network_id: NetworkId,
	pub execute_program: ExecuteProgramMsg,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema, QueryResponses))]
pub enum QueryMsg {
	/// Returns [`AssetReference`] for an asset with given id.
	#[cfg_attr(feature = "std", returns(LookupResponse))]
	LookupAsset { asset_id: AssetId },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct LookupResponse {
	pub reference: Asset,
}
