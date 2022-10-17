//! Less general more targeted solution before XCM gets general transactions support and discovery,
//! so at same time it can be more robust for Substrate based sibling chains. Also is more
//! specialized form of `xcm::latest::Junction`.
//!
//! API allow for selling (partial exchange via DEX, OB or Auction) via `engines` on other
//! parachains.
//!
//! Main use case is automatic liquidations of assets.

pub mod assets;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use xcm::latest::QueryId;

use crate::defi::Sell;
type AccountId = [u8; 32];
type PalletInstance = u8;
type AssetId = u128;
/// as of now it is what is handled by XCM decoders of all networks, so let try it for now
pub type Balance = u128;
pub type ConfigurationId = u128;
pub type OrderId = QueryId;

/// Strongly typed sibling location to `Transact` via XCM.
/// Shortcut to more complicate way to set it via `xcm::latest::Junction`.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmTransactConfiguration {
	pub parachain_id: polkadot_parachain::primitives::Id,
	pub method_id: CumulusMethodId,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct CumulusMethodId {
	pub pallet_instance: PalletInstance,
	pub method_id: u8,
}

impl XcmTransactConfiguration {
	pub fn new(
		parachain_id: polkadot_parachain::primitives::Id,
		pallet_instance: PalletInstance,
		method_id: u8,
	) -> Self {
		Self { parachain_id, method_id: CumulusMethodId { pallet_instance, method_id } }
	}
}
/// must be stored on side of protocol which will be asked to sell
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellRequestTransactConfiguration {
	/// Used to route XCM message dispatch into right target
	pub location: XcmTransactConfiguration,
	/// Taken from storage and put as parameter into call.
	/// Some preconfigured way of sell on other chain.
	/// Example, some specific slippage or amount limits, or number of blocks it should take before
	/// cancellation. Must be set by owners of engine and chose by those who governs caller side
	pub configuration_id: u128,
	/// native token fee to pay on `engine` chain
	pub fee: Balance,
}

#[derive(Encode)]
pub struct XcmCumulusDispatch<Parameters>
where
	Parameters: Encode,
{
	pallet: PalletInstance,
	method: u8,
	parameters: Parameters,
}

impl<Parameters: Encode> XcmCumulusDispatch<Parameters> {
	pub fn new(pallet: PalletInstance, method: u8, parameters: Parameters) -> Self {
		Self { pallet, method, parameters }
	}
}

/// The actual binary data dispatched into `Call`.
/// Assets to be liquidated was moved with `xcm::latest::Instruction::TransferReserveAsset` before
/// in same XCM message.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellRequest {
	/// Created on sender side and used to correlate callbacks.
	/// Receiver tracks origin and `order_id`.
	pub order_id: OrderId,
	/// Sovereign account of sender parachain on remote parachain.
	/// It was transferred amount to sell before, and
	/// Notes:
	/// This may be parachain id to get account, but than all protocols parachain will share same
	/// account and will be harder to debug. May consider using indices pallet or some sub account
	/// for protocol too.
	pub from_to: AccountId,
	/// Target network asset id, so sender must be aware of remote encoding.
	/// If order cannot be  filled to some amount,  the description of that is sent in
	/// `XcmSellResponseTransact`
	pub order: Sell<AssetId, Balance>,
	pub configuration: ConfigurationId,
}

/// Optional response if engine did not sold all on first requests
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellInitialResponseTransact {
	/// Amount of `base` token which was taken by engine to be sold.
	/// Amount remaining was transferred with `xcm::latest::Instruction::TransferReserveAsset` in
	/// same XCM message. Must be more than `Balance::zero()`
	pub total_amount_taken: Balance,
	/// Minimal price in `quote` amount  of `amount_taken`
	pub minimal_price: Balance,
	pub order_id: OrderId,
}

/// Response from engine, either be first and final, or can be after
/// `XcmSellInitialResponseTransact`
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellFinalResponseTransact {
	/// may be less than `XcmSellInitialResponseTransact::total_amount_taken`.
	/// Would be `Balance::zero()` if cannot sell anything. So sender can switch to other engine.
	pub total_amount_taken: Balance,
	/// Price `total_amount_taken` in `quote`. Must be larger than or equal to
	/// `XcmSellInitialResponseTransact::minimal_price`
	pub price: Balance,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct SellOrderResponse {
	/// sender order_id, way to correlate XCM message (like
	/// `xcm::latest::Instruction::QueryResponse`)
	pub order_id: OrderId,
	pub body: SellResponse,
}

// Next relation must hold:
// Sell minimal price  <= initial price <= final price
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum SellResponse {
	Initial(XcmSellInitialResponseTransact),
	Final(XcmSellInitialResponseTransact),
}
