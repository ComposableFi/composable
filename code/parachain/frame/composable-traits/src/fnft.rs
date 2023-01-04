//! Defines the interfaces of interacting with financial NFTs.
//!
//! A financial NFT Allows management of financial positions as represented by a NFT.

use crate::account_proxy::ProxyType;
use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::collections::vec::bounded::BiBoundedVec;
use core::fmt::Debug;
use frame_support::traits::tokens::nonfungibles::Inspect;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

pub type Key = BiBoundedVec<u8, 1, 64>;
pub type Value = BiBoundedVec<u8, 1, 256>;

/// Wrap any financial position into the ownership of an NFT
pub trait FinancialNft<AccountId>: Inspect<AccountId> {
	/// ID of the Account which holds the assets owned by a financial NFT. The value of the
	/// financial NFT is the sum total of balances of all asset types in this account plus the
	/// future returns minus any liabilities. Future returns and liabilities should be queried
	/// through the originating financial NFT protocol.
	fn asset_account(collection: &Self::CollectionId, instance: &Self::ItemId) -> AccountId;

	/// Retrieve the next valid financial NFT ID for the given collection in order to
	/// mint a new NFT.
	// TODO(benluelo): Remove this, it should be handled internally and doesn't need to be exposed.
	fn get_next_nft_id(collection: &Self::CollectionId) -> Result<Self::ItemId, DispatchError>;
}

/// Trait to be implemented by protocol supporting financial NFTs.
pub trait FinancialNftProtocol {
	/// Type for identifying an item.
	type ItemId;

	/// Asset ID type. This is the type used for financial NFT collection IDs. Following
	/// https://github.com/paritytech/xcm-format#6-universal-asset-identifiers setting collection
	/// IDs as asset IDs (asset class), allows universal identifiers for all asset classes
	/// across eco system projects. Refer xcm::..::MultiLocation
	type AssetId;

	/// Balance type.
	type Balance;

	/// Returns the set of Asset IDs mapping the originated financial NFT collections to
	/// the financial NFT protocol. Used to identify the financial NFT protocol to route operations
	/// related to a given financial NFT.
	///
	/// Eg: for staking rewards if
	///  the fNFT collectionId(assetId) of issued fNFTs for staking positions of a particular reward
	///  pool a is x and for another b is y. Then this function returns vec![x, y].
	fn collection_asset_ids() -> Vec<Self::AssetId>;

	/// The value of the financial NFT is the sum total of balances of all asset types in its
	/// account plus the future returns minus any liabilities.
	///
	/// - collection: id of the financial NFT collection issued/used by the protocol.
	/// TODO (vim): Think how to represent the difference between assets and liabilities
	fn value_of(
		collection: &Self::AssetId,
		instance: &Self::ItemId,
	) -> Result<Vec<(Self::AssetId, Self::Balance)>, DispatchError>;
}

/// Default Version type used for NFTs.
#[derive(
	Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, MaxEncodedLen, TypeInfo,
)]
#[repr(transparent)]
pub struct FinancialNftVersion(u8);

impl FinancialNftVersion {
	pub const VERSION_1: FinancialNftVersion = FinancialNftVersion(1);
}

pub trait FnftAccountProxyTypeSelector<T> {
	/// Return the selected account proxy types
	fn get_proxy_types() -> Vec<T>;
}

pub struct FnftAccountProxyType;
impl FnftAccountProxyTypeSelector<ProxyType> for FnftAccountProxyType {
	fn get_proxy_types() -> Vec<ProxyType> {
		[ProxyType::Governance, ProxyType::CancelProxy].into()
	}
}
