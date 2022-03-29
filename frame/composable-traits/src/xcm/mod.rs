//! Less general more targeted solution before XCM gets general transactions support and discovery,
//! so at same time it can be more robust for Substate based sibling chains. Also is mor specialised
//! form of `xcm::latest::Junction`.
//!
//! API allow for selling (partial exchange via DEX, OB or Auction) via `engines` on other
//! parachains.
//!
//! Main use case is automatic liquidations of assets.

use xcm::latest::QueryId;

use crate::defi::Sell;
type AccountId = [u8; 32];
type PalletInstance = u8;
type AssetId = u128;
type Balance = u128;
type OrderId = QueryId;

/// Strongly typed sibling location to `Transact` via XCM.
/// Shortcut to more complicate way to set it via `xcm::latest::Junction`.
#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmTransactConfiguration {
	pub parachain_id: polkadot_parachain::primitives::Id,
	pub pallet_instance: PalletInstance,
	pub method_id: u8,
}
/// must be stored on side of protocol which will be asked to sell
#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellRequestTransactConfiguration {
	/// Used to route XCM message dispatch into right target
	pub location: XcmTransactConfiguration,
	/// Taken from storage and put as paramter into call.
	/// Some preconfigured way of sell on other chain.
	/// Example, some specific slippage or amount limits, or number of blocks it should take before
	/// cancelation. Must be set by owners of engine and chose by thoose who governs caller side
	pub configuraition_id: u128,
	/// native token fee to pay on `engine` chain
	pub fee: Balance,
}

/// The actualy binary data dispatched into `Call`.
/// Assets to be liqudatied was moved with `xcm::latest::Instruction::TransferReserveAsset` before
/// in same XCM message.
#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellRequest {
	/// Created on sender side and used to correlate callbacks.
	/// Receiver tracks origin and `order_id`.
	pub order_id: OrderId,
	/// Sovereign account of sender parachain on remote parachain.
	/// It was transfered amount to sell before, and
	/// Notes:
	/// This may be parachain id to get account, but than all protocols parchain will share same
	/// account and will be harder to debug. May consider using indices pallet or some sub account
	/// for protocol too.
	pub from_to: AccountId,
	/// Target network asset id, so sender must be aware of remote encoding.
	/// If order cannot be  filled to some amount,  the description of that is sent in
	/// `XcmSellResponseTransact`
	pub order: Sell<AssetId, Balance>,
	pub configuration: u128,
}

/// Optional response if engine did not sold all on first requests
#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellInitialResponseTransact {
	/// Amount of `base` token which was taken by engine to be sold..
	/// Amount remaining was transfered with `xcm::latest::Instruction::TransferReserveAsset` in
	/// same XCM message. Must be more than `Balance::zero()`
	pub total_amount_taken: Balance,
	/// Minimal price in `quote` amount  of `amount_taken`
	pub minimal_price: Balance,
}

/// Response from enigne, either be first and final, or can be after
/// `XcmSellInitialResponseTransact`
#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct XcmSellFinalResponseTransact {
	/// may be less than `XcmSellInitialResponseTransact::total_amount_taken`.
	/// Would be `Balance::zero()` if cannot sell anything. So sender can switch to other engine.
	pub total_amount_taken: Balance,
	/// Price `total_amount_taken` in `quote`. Must be larger or eququal than
	/// `XcmSellInitialResponseTransact::minimal_price`
	pub price: Balance,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct SellOrderResponse {
	/// sender order_id, way to corellate XCM message (like
	/// `xcm::latest::Instruction::QueryResponse`)
	pub order_id: OrderId,
	pub body: SellResponse,
}

// Nex relation must hold:
// Sell minimal price  <= initial price <= final price
#[derive(Clone, Debug, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum SellResponse {
	Initial(XcmSellInitialResponseTransact),
	Final(XcmSellInitialResponseTransact),
}
