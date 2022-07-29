//! Default interface is used to interact with financial NFTs through a NFT provider.
//!
//! Allows to enter positions and sell it before position is left as NFT.
//! Allows to split financial positions.
//!
//! # Design
//! There are 2 ways to integrate NFT and financial positions.
//!
//! Store all metadata of position in NFT BLOB or store only reference in NFT.
//!
//! What we have in case of storing reference:
//! - NFT account produced from identity is set as owner of position
//! - protocol providing position implements trait which allow to work with it through NFT API
//! - position is stored in any way efficient for that
//! - NFT stores only reference to position in protocol,
//! For example, pallet identifier and some strict monotonic a counter/sequence identifier of
//! position combined.
//! - allows to build NFTs from typed positions(primitives)
//! - during XCMP transfer, owner of NFT become target parachain
//! - position reference work well with reserver transfer XCMP approach
//!
//! So NFT is (class, id, position reference)
//!
//!
//! In case of storing all data in NFT(as serde binary):
//! - protocol calls NFT to read state
//! - storage is not maximally efficient
//! - more risks of having serde issue and types
//!
//! In both cases can XCMPing NFT state as whole requires protocol constraints, like immutability or
//! protocol implementation on other side. In both cases RPC or shared library is required to
//! interpret state offchain.
//!
//! Problem with traditional NFTs - these are isolated from each other while burn and mint, while
//! our goal is to make burn/mint(and split) to influence some total/shared.  For this case we go
//! with option to have a proxy which ensure that NFT part of state and protocol state are
//! synchronized during such calls.
pub mod protocol;

use codec::{Decode, Encode, MaxEncodedLen};
use composable_support::collections::vec::bounded::BiBoundedVec;
use core::fmt::Debug;
use frame_support::{
	dispatch::DispatchResult,
	traits::{
		tokens::nonfungibles::{Create, Inspect, Mutate},
		Get,
	},
	BoundedBTreeMap,
};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

pub type Key = BiBoundedVec<u8, 1, 64>;
pub type Value = BiBoundedVec<u8, 1, 256>;

pub type Properties<MaxProperties> = BoundedBTreeMap<Key, Value, MaxProperties>;

/// depending on `ClassId` this can mean typed position or complex storage in NFT
pub type Reference = BiBoundedVec<u8, 1, 64>;

/// allow to wrap any position into ownership of fNFT
pub trait ReferenceNft<AccountId>:
	Create<AccountId> + Mutate<AccountId> + Inspect<AccountId>
{
	type MaxProperties: Get<u32>;
	// `who` must be owner of original reference. after NFTing position, NFT instance is owner, but
	// `who` is owner of NFT in case of reference NFT is reported burn (so owner is lost), it is
	// auctioned
	fn reference_mint_into<NFTProvider, NFT>(
		_class: &Self::CollectionId,
		_instance: &Self::ItemId,
		_who: &AccountId,
		reference: Reference,
	) -> DispatchResult;

	fn mint_new_into(
		class: &Self::CollectionId,
		who: &AccountId,
		properties: Properties<Self::MaxProperties>,
	) -> Result<Self::ItemId, DispatchError>;
}

/// Default ClassId type used for NFTs.
#[derive(
	Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, MaxEncodedLen, TypeInfo,
)]
#[repr(transparent)]
pub struct NftClass(u8);

#[cfg(feature = "test-utils")]
impl NftClass {
	/// Create a new [`NftClass`].
	///
	/// Will not necessarilly be a well-known class; only for use in testing.
	pub fn new(inner: u8) -> Self {
		NftClass(inner)
	}
}

impl NftClass {
	pub const STAKING: NftClass = NftClass(1);
}

/// Default Version type used for NFTs.
#[derive(
	Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, MaxEncodedLen, TypeInfo,
)]
#[repr(transparent)]
pub struct NftVersion(u8);

impl NftVersion {
	pub const VERSION_1: NftVersion = NftVersion(1);
}
