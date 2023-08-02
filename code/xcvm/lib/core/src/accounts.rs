use alloc::{string::String, vec::Vec};

use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::{AssetId, NetworkId};

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.accounts";

/// Kinds of events escrow contract can generate.
#[derive(Clone, Copy, Debug, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Action {
	/// Contract has been instantiated.
	Instantiated,
	/// Funds have been deposited to an account.
	Deposit,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct InstantiateMsg {
	/// Identifier of the network this contract is running on.
	pub network_id: NetworkId,

	/// Address of an escrow account running locally.
	///
	/// If specified, the contract with this address may execute
	/// [`ExecuteMsg::LocalPacket`] messages on the accounts contract and they
	/// will be interpreted like cross-chain messages from `network_id`.
	#[serde(skip_serializing_if = "Option::is_none", default)]
	pub local_escrow: Option<String>,

	/// Admins which are allowed to use the break glass feature.
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MigrateMsg {}

// TODO(mina86): Add messages for managing recovery addresses.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum ExecuteMsg {
	CreateAccount(CreateAccountRequest),
	DropAccount(DropAccountRequest),
	SubmitProblem(SubmitProblemRequest),
	/// A normally cross-chain packet sent from a contract on local chain.
	LocalPacket(Packet),
	BreakGlass,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum QueryMsg {}

/// Requests creation of a new account.
///
/// The account will have the same name as the sender of the message.
/// Request fails if the account already exists.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct CreateAccountRequest {
	/// List of addresses on remote chains which will have access to
	/// the account.
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub recovery_addresses: Vec<RemoteAddress>,
}

/// An address of a remote chain.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct RemoteAddress {
	/// Network identifier of the chain.
	pub network_id: NetworkId,
	/// Chain-specific representation of the address on that chain.
	pub address: String,
}

/// Deletes the account transferring all remaining funds to another account.
///
/// The rquest fails if the account has any pending problems or locked
/// assets, account holds funds and the beneficiary account doesn’t exist.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Encode, Decode)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct DropAccountRequest {
	/// Account to transfer all funds remaining on the current account.
	pub beneficiary_account: String,
}

/// Balance of a single asset.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct AssetBalance {
	/// Identifier of the asset.
	pub asset_id: AssetId,
	/// Available unlocked balance.  This is the amount user can access at
	/// the moment.
	pub unlocked_amount: u128,
	/// Available locked balance.  This is the amount that is being used
	/// in processing of a problem and cannot be used until execution of
	/// the problem terminates.
	pub locked_amount: u128,
}

/// Sends a new problem for the system to solve.
///
/// The problem is added to set of active problems so that solvers can start
/// working on it and figure out the best solution.  Submitting of a problem
/// may fail if user has insufficient funds.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Encode, Decode)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct SubmitProblemRequest {
	/// The problem to solve; TODO: refer to problem specification
	// TODO(mina86): Switch to Binary.  Currently issue is this conflicts with
	// Encode and Decode derives.
	pub problem: Vec<u8>,
}

/// Response to submisison of a new problem.
///
/// The problem is assigned a unique identifier which can be used to query
/// state of the problem.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct SubmitProblemResponse {
	/// Globally unique identifier of the problem.
	pub problem_id: u128,
}

/// Message from escrow contract to wallet contact updating balances for
/// given `account`.
///
/// In acknowledgement, the contract responses with a single boolean value
/// indicating whether the deposit was accepted.  If it wasn’t, escrow contract
/// must refund funds to the sender.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Encode, Decode)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct DepositNotificationPacket {
	/// Identifier of the deposist assigned by the escrow contract.  It’s
	/// not globally unique and is used to confirm or deecline a deposit.
	pub deposit_id: u128,
	/// The account whose balances are affected.
	pub account: String,
	/// List of credits to balances.
	pub deposits: Vec<(AssetId, u128)>,
}

/// Message from escrow contract to accounts contract relaying user request.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Encode, Decode)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct RelayedRequestPacket {
	/// Address of the user initiating the message on the local chain.
	pub address: String,
	/// Account to execute the request for.  The request will fail if
	/// account doesn’t exist or `(network_id, address)` isn’t its recovery
	/// address.
	pub account: String,
	/// The request to relay.
	pub request: RelayedRequest,
}

/// Request which can be relayed to the accounts contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Encode, Decode)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum RelayedRequest {
	DropAccount(DropAccountRequest),
	SubmitProblem(SubmitProblemRequest),
}

/// A cross-chain packet that the contract accepts.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Encode, Decode, derive_more::From)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum Packet {
	/// Message from escrow contract informing of funds being deposited to an
	/// account.
	DepositNotification(DepositNotificationPacket),
	/// Message from escrow contract with a relayed request from user on that
	/// contract’s chain.
	RelayedRequest(RelayedRequestPacket),
}
