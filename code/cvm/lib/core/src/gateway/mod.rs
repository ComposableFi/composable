pub mod config;
mod query;

pub use config::*;
pub use query::*;

use crate::{
	prelude::*, transport::ibc::XcMessageData, AssetId, CallOrigin, Funds, InterpreterOrigin,
	NetworkId,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	Config(ConfigSubMsg),

	/// Sent by the user to execute a program on their behalf.
	ExecuteProgram(ExecuteProgramMsg),

	/// Request to execute a program on behalf of given user.
	///
	/// This can only be sent by trusted contract.  The message is
	ExecuteProgramPrivileged {
		/// The origin of the call.
		call_origin: CallOrigin,
		/// Program to execute.
		execute_program: BridgeExecuteProgramMsg,
	},

	/// Message sent from interpreter trying to spawn program on another
	/// network.
	BridgeForward(BridgeForwardMsg),

	/// simple permissionless message which produce xcvm program to test routes
	Shortcut(ShortcutSubMsg),

	/// executed by host as part of memo handling
	MessageHook(XcMessageData),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ShortcutSubMsg {
	Transfer {
		/// assets from there
		asset_id: AssetId,
		amount: Uint128,
		/// target network, can hope over several networks
		/// if route is stored in state
		network: NetworkId,
		/// by default receiver is this
		receiver: Option<String>,
	},
}

/// Definition of a program to be executed including its context.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct ExecuteProgramMsg<Assets = Option<Funds<crate::shared::Displayed<u128>>>> {
	/// The program salt.
	/// If JSON, than hex encoded non prefixed lower case string.
	/// If not specified, uses no salt.
	#[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
	#[cfg_attr(feature = "std", schemars(schema_with = "String::json_schema"))]
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub salt: Vec<u8>,
	/// The program.
	pub program: crate::shared::XcProgram,
	/// Assets to fund the CVM interpreter instance.
	/// The interpreter is funded prior to execution.
	/// If None, 100% of received funds go to interpreter.
	pub assets: Assets,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub tip: Option<String>,
}

/// message sent within CVM must have assets defined
pub type BridgeExecuteProgramMsg = ExecuteProgramMsg<Funds<crate::shared::Displayed<u128>>>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct BridgeForwardMsg {
	pub executor_origin: InterpreterOrigin,
	/// target network
	pub to: NetworkId,
	pub msg: BridgeExecuteProgramMsg,
}

/// Wrapper for interfacing with a gateway contract.
///
/// Provides convenience methods for querying the gateway and sending execute
/// messages to it.  Queries use [`cosmwasm_std::QuerierWrapper`] to make the
/// request and return immediately.  Execute requests on the other hand are
/// asynchronous and done by returning a [`cosmwasm_std::CosmosMsg`] which needs
/// to be added to a [`cosmwasm_std::Response`] object.
///
/// The object can be JSON-serialised as the address of the gateway.  Note that
/// since it’s serialised as [`cosmwasm_std::Addr`] it should not be part of
/// public API and only serialised in trusted objects where addresses don’t need
/// to be validated.
#[cfg(feature = "cosmwasm")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Gateway {
	address: cosmwasm_std::Addr,
}

#[cfg(feature = "cosmwasm")]
impl Gateway {
	pub fn new(address: cosmwasm_std::Addr) -> Self {
		Self { address }
	}

	/// Validates gateway address and if it’s correct constructs a new object.
	///
	/// This is mostly a wrapper around CosmWasm address validation API.
	pub fn addr_validate(
		api: &dyn cosmwasm_std::Api,
		address: &str,
	) -> cosmwasm_std::StdResult<Self> {
		api.addr_validate(address).map(Self::new)
	}

	/// Returns gateway contract’s address as a String.
	pub fn address(&self) -> cosmwasm_std::Addr {
		self.address.clone()
	}

	/// Creates a CosmWasm message executing given message on the gateway.
	///
	/// The returned message must be added to Response to take effect.
	pub fn execute(
		&self,
		msg: impl Into<ExecuteMsg>,
	) -> cosmwasm_std::StdResult<cosmwasm_std::CosmosMsg> {
		self.execute_with_funds(msg, Vec::new())
	}

	/// Creates a CosmWasm message executing given message on the gateway with
	/// given funds attached.
	///
	/// The returned message must be added to Response to take effect.
	pub fn execute_with_funds(
		&self,
		msg: impl Into<ExecuteMsg>,
		funds: Vec<cosmwasm_std::Coin>,
	) -> cosmwasm_std::StdResult<cosmwasm_std::CosmosMsg> {
		cosmwasm_std::wasm_execute(self.address(), &msg.into(), funds)
			.map(cosmwasm_std::CosmosMsg::from)
	}

	/// Queries the gateway for definition of an asset with given id.
	pub fn get_asset_by_id(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		asset_id: AssetId,
	) -> cosmwasm_std::StdResult<AssetItem> {
		let query = QueryMsg::GetAssetById { asset_id };
		self.do_query::<GetAssetResponse>(querier, query).map(|response| response.asset)
	}

	pub fn get_exchange_by_id(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		exchange_id: crate::service::dex::ExchangeId,
	) -> cosmwasm_std::StdResult<crate::service::dex::ExchangeItem> {
		let query = QueryMsg::GetExchangeById { exchange_id };
		self.do_query::<GetExchangeResponse>(querier, query)
			.map(|response| response.exchange)
	}

	/// Queries the gateway for definition of an asset with given local
	/// reference.
	pub fn get_local_asset_by_reference(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		reference: AssetReference,
	) -> cosmwasm_std::StdResult<AssetItem> {
		let query = QueryMsg::GetLocalAssetByReference { reference };
		self.do_query::<GetAssetResponse>(querier, query).map(|response| response.asset)
	}

	/// Sends a query to the gateway contract.
	fn do_query<R: serde::de::DeserializeOwned>(
		&self,
		querier: cosmwasm_std::QuerierWrapper,
		query: QueryMsg,
	) -> cosmwasm_std::StdResult<R> {
		let query = cosmwasm_std::WasmQuery::Smart {
			contract_addr: self.address().into(),
			msg: cosmwasm_std::to_binary(&query)?,
		};
		querier.query(&query.into())
	}
}
