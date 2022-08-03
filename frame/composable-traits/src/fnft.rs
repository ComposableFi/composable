//! Defines the interfaces of interacting with financial NFTs.
//!
//! A financial NFT Allows management of financial positions as represented by a NFT.

use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::collections::vec::bounded::BiBoundedVec;
use core::fmt::Debug;
use frame_support::traits::tokens::nonfungibles::Inspect;
use scale_info::TypeInfo;

pub type Key = BiBoundedVec<u8, 1, 64>;
pub type Value = BiBoundedVec<u8, 1, 256>;

/// Wrap any financial position into the ownership of an NFT
pub trait FinancialNFT<AccountId>: Inspect<AccountId> {
	/// ID of the Account which holds the assets owned by a financial NFT. The value of the
	/// financial NFT is the sum total of balances of all asset types in this account plus the
	/// future returns minus any liabilities. Future returns and liabilities should be queried
	/// through the originating financial NFT protocol.
	fn asset_account(collection: &Self::CollectionId, instance: &Self::ItemId) -> AccountId;
}

/// Trait to be implemented by protocol supporting financial NFTs.
pub trait FinancialNFTProtocol {
	/// Type for identifying an item.
	type ItemId;

	/// Asset ID type.
	type AssetId;

	/// Balance type.
	type Balance;

	/// Asset ID mapping the financial NFT to the financial NFT protocol. This is generally the
	/// asset type that is locked into the financial NFT account at creation.
	fn protocol_asset_id() -> Self::AssetId;

	/// The value of the financial NFT is the sum total of balances of all asset types in its
	/// account plus the future returns minus any liabilities.
	///
	/// - collection: id of the financial NFT collection issued/used by the protocol.
	/// TODO (vim): Think how to represent the difference between assets and liabilities
	fn value_of(
		collection: &Self::AssetId,
		instance: &Self::ItemId,
	) -> Vec<(Self::AssetId, Self::Balance)>;
}

/// Default Version type used for NFTs.
#[derive(
	Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, MaxEncodedLen, TypeInfo,
)]
#[repr(transparent)]
pub struct FinancialNFTVersion(u8);

impl FinancialNFTVersion {
	pub const VERSION_1: FinancialNFTVersion = FinancialNFTVersion(1);
}
