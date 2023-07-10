use cosmwasm_schema::cw_serde;
use parity_scale_codec::{Decode, Encode};
use xc_core::NetworkId;

/// Prefix used for all events attached to gateway responses.
pub const EVENT_PREFIX: &str = "xcvm.escrow";

/// Kinds of events escrow contract can generate.
#[derive(Copy, strum::AsRefStr)]
#[strum(serialize_all = "lowercase")]
#[cw_serde]
pub enum Action {
	/// Contract has been instantiated.
	Instantiated,
	/// User made a new deposit.  It’s now waiting for accounts contract’s
	/// acknowledgement.
	PendingDeposit,
	/// A pending deposit has been acknowledgement by accounts contract or timed
	/// out.
	DepositDone,
}

#[cw_serde]
pub struct InstantiateMsg {
	/// Network ID of this network
	pub network_id: NetworkId,
	/// Admins which are allowed to use the break glass feature.
	pub admins: Vec<String>,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
	DepositAssets(DepositAssetsRequest),
	Relay(RelayRequest),
	BreakGlass,
}

#[cw_serde]
pub enum QueryMsg {}

/// Deposits assets onto the virtual wallet.
///
/// This triggers a message to the accounts contract on the Centauri chain
/// which updates user balances.  Because this is a cross-chain operation,
/// there is a delay between this operation succeeding and funds showing up
/// on user account.
///
/// If the `account` doesn’t exist on the accounts contract, the deposit is
/// aborted and assets are returned to the sender of the message.
#[cw_serde]
pub struct DepositAssetsRequest {
	/// Name of the account in the virtual wallet to deposit funds to.
	pub account: String,
	/// Funds attached to this message to deposit to the user.
	pub deposits: Vec<LocalAssetAmount>,
}

/// An asset with its amount.
#[cw_serde]
pub struct LocalAssetAmount {
	/// Local asset identifier.
	pub asset_id: LocalAssetId,
	/// Amount of the asset.
	pub amount: u128,
}

/// Local asset identifier.  XXX TODO
#[cw_serde]
pub enum LocalAssetId {
	Native { denom: String },
	Path { path: String },
}

impl LocalAssetId {
	/// Normalises local asset id so that equivalent asset identifiers are
	/// mapped to a single representative value.
	pub fn normalize(&mut self) {
		todo!()
	}
}

/// Response to asset deposit request.
#[cw_serde]
pub struct DepositAssetsResponse {
	/// Identifier of the deposit unique on given chain.
	pub deposit_id: u128,
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
#[cw_serde]
#[derive(Encode, Decode)]
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
