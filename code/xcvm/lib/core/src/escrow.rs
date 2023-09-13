use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::NetworkId;

type Uint128 = crate::shared::Displayed<u128>;

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.escrow";

/// Kinds of events escrow contract can generate.
#[derive(Clone, Copy, Debug, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Action {
	/// Contract has been instantiated.
	Instantiated,
	/// User made a new deposit.  It’s now waiting for accounts contract’s
	/// acknowledgement.
	PendingDeposit,
	/// A pending deposit has been acknowledgement by accounts contract or timed
	/// out.
	DepositDone,
	/// New IBC channel has been opened.
	IbcConnect,
	/// An IBC channel has been closed.
	IbcClose,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct InstantiateMsg {
	/// Network ID of this network
	pub network_id: NetworkId,
	/// Address of a local XCVM gateway contract.
	pub gateway_address: String,
	/// Location of the accounts contract.
	pub accounts_contract: AccountsContract,
	/// Admins which are allowed to use the break glass feature.
	pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum AccountsContract {
	/// Address of the accounts contract on the local network.
	Local(String),
	/// IBC channel name with accounts contract on the other side.
	Remote(String),
	/// No accounts contract configured yet.
	None,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	/// Request to deposit any attached coins to the account.
	DepositAssets(DepositAssetsRequest),

	/// Tokens received from CW-20 contract.
	///
	/// Just like [`ExecuteMsg::DepositAssets`] this triggers a message to the
	/// accounts contract.  The account to credit is specified in the messages
	/// `msg` which is JSON-serialised [`ReceiveMsgBody`].
	#[cfg(feature = "cw20")]
	Receive(cw20::Cw20ReceiveMsg),

	/// Sets accounts contract address to the one given.
	SetAccountsContract(AccountsContract),

	Relay(RelayRequest),
	BreakGlass,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum QueryMsg {}

/// Message attached to [`::cw20::Cw20ReceiveMsg`] sent when receiving CW20
/// funds.
///
/// If the `account` doesn’t exist on the accounts contract, the deposit is
/// aborted and assets are returned to the sender of the message.
#[cfg(feature = "cw20")]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct ReceiveMsgBody {
	/// Name of the account in the virtual wallet to deposit funds to.
	pub account: String,
}

/// Request to deposit assets to the account.
///
/// This triggers a message to the accounts contract which updates user
/// balances.  Because this is a cross-chain operation, there is a delay between
/// this operation succeeding and funds showing up on user account.
///
/// Responses with [`DepositAssetsResponse`] message.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct DepositAssetsRequest {
	/// Name of the account in the virtual wallet to deposit funds to.
	pub account: String,

	/// List of CW20 tokens with amount to include in the deposit.
	///
	/// The contract must have allowance from the sender on the CW20 contract
	/// for given amount of the token.  The contract will attempt to transfer
	/// the funds to its account to get the funds.
	#[cfg(feature = "cw20")]
	pub tokens: Vec<(String, Uint128)>,
}

/// Response to asset deposit request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct DepositAssetsResponse {
	/// Identifier of the deposit unique on given chain.
	pub deposit_id: Uint128,
}

/// Relies a problem to the accounts contract on the Centauri chain.
///
/// If sender of the message has been added as a recovery address of the
/// `account` the request behaves as if it was submitted on the Centauri
/// chain.  If the `account` doesn’t exist or sender of this message isn’t
/// designed recovery address of the account, the call will eventually fail.
///
/// Because this is a cross-chain request, its failure or success is delayed
/// (by having to propagate the message) and user has to monitor the wallet
/// contract to see the result.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct RelayRequest {
	/// Wallet account this request affects.
	pub account: String,
	/// The actual request to relay.
	pub request: crate::accounts::RelayedRequest,
}

impl core::fmt::Display for Action {
	#[inline]
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		fmtr.write_str(self.as_ref())
	}
}
