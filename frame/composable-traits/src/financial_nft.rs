use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
use composable_support::collections::vec::bounded::BiBoundedVec;
use core::fmt::Debug;
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{
		tokens::nonfungibles::{Create, Inspect, Mutate},
		Get,
	},
};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, TokenError};
use sp_std::{collections::btree_map::BTreeMap, vec::Vec};

// TODO: use BiBoundedVec to avoid for safety - it is easy to grow, but not easy to shrink (needs
// migration)
pub type AttributeKey = Vec<u8>; //TODO: len 1 to 64
pub type AttributeValue = Vec<u8>; //TODO: len from 0 to 2048

pub trait FinancialNftProvider<AccountId>: Create<AccountId> + Mutate<AccountId> {
	/// Mint an NFT instance with initial (key, value) attribute in the given account.
	///
	/// Arguments
	///
	/// * `class` the NFT class id.
	/// * `who` the owner of the minted NFT.
	/// * `key` the key of the initial attribute.
	/// * `value` the value of the initial attribute.
	///
	/// Note: we store the NFT scale encoded struct under a single attribute key.
	///
	/// Returns the unique instance id.
	fn mint_nft<K: Encode, V: Encode>(
		class: &Self::ClassId,
		who: &AccountId,
		version: &K,
		nft_data: &V,
	) -> Result<Self::InstanceId, DispatchError>;

	fn split(
		instance: &Self::InstanceId,
		overrides: BiBoundedVec<BTreeMap<AttributeKey, AttributeValue>, 1, 16>,
	) -> Result<BiBoundedVec<Self::InstanceId, 1, 16>, DispatchError>;
}

/// Default interface used to interact with financial NFTs through a NFT provider.
///
/// The interface will always fully serialize/deserialize the NFT type with the NFT::Version as
/// single attribute key.
pub trait FinancialNftProtocol<AccountId: Eq> {
	/// Abstract type of a class id.
	type ClassId: FullCodec + TypeInfo;

	/// Abstract type of an instance id. Used to uniquely identify NFTs.
	type InstanceId: Copy + Eq + Debug + FullCodec + TypeInfo;

	/// Abstract type of a version. Used to migrate NFT when updating their content.
	/// Migration must be done by the protocol operating on the NFT type.
	type Version: FullCodec + TypeInfo;

	/// NFT provider from which we load/store NFT's.
	type NFTProvider: FinancialNftProvider<
		AccountId,
		ClassId = Self::ClassId,
		InstanceId = Self::InstanceId,
	>;

	/// Mint a new NFT into an account.
	///
	/// Arguments
	///
	/// * `owner` the owner of the minted NFT.
	/// * `nft` the initial value of the minted NFT.
	///
	/// Return the NFT instance id if successfull, otherwise the underlying NFT provider error.
	fn mint_protocol_nft<NFT>(
		owner: &AccountId,
		nft: &NFT,
	) -> Result<Self::InstanceId, DispatchError>
	where
		NFT: Get<Self::ClassId> + Get<Self::Version> + Encode,
	{
		Self::NFTProvider::mint_nft(&NFT::get(), owner, &<NFT as Get<Self::Version>>::get(), &nft)
	}

	/// Retrieve the _possible_ owner of the NFT identified by `instance_id`.
	///
	/// Arguments
	///
	/// * `instance_id` the ID that uniquely identify the NFT.
	fn get_protocol_nft_owner<NFT>(
		instance_id: &Self::InstanceId,
	) -> Result<AccountId, DispatchError>
	where
		NFT: Get<Self::ClassId>,
	{
		Self::NFTProvider::owner(&NFT::get(), instance_id).ok_or(DispatchError::CannotLookup)
	}

	/// Ensure that the owner of the identifier NFT is `account_id`.
	///
	/// Arguments
	///
	/// * `owner` the account id that should own the NFT.
	/// * `instance_id` the NFT instance id.
	///
	/// Returns `Ok(())` if `owner` is the owner of the NFT identified by `instance_id`.
	fn ensure_protocol_nft_owner<NFT>(
		owner: &AccountId,
		instance_id: &Self::InstanceId,
	) -> Result<(), DispatchError>
	where
		NFT: Get<Self::ClassId>,
	{
		let nft_owner = Self::get_protocol_nft_owner::<NFT>(instance_id)?;
		ensure!(nft_owner == *owner, DispatchError::BadOrigin);
		Ok(())
	}

	/// Return an NFT identified by its instance id.
	///
	/// Arguments
	///
	/// * `instance_id` the NFT instance id.
	fn get_protocol_nft<NFT>(instance_id: &Self::InstanceId) -> Result<NFT, DispatchError>
	where
		NFT: Get<Self::ClassId> + Get<Self::Version> + Decode,
	{
		Self::NFTProvider::typed_attribute(
			&NFT::get(),
			instance_id,
			&<NFT as Get<Self::Version>>::get(),
		)
		.ok_or(DispatchError::Token(TokenError::UnknownAsset))
	}

	/// Mutate the NFT identified by `instance_id`.
	///
	/// Arguments
	///
	/// * `T` the callback return value.
	/// * `E` the callback error value.
	///
	/// * `instance_id` the NFT instance id.
	/// * `f` the callback in charge of mutating the NFT.
	///
	/// Returns the result of the callback, either `T` or `E`.
	fn try_mutate_protocol_nft<NFT, T, E>(
		instance_id: &Self::InstanceId,
		f: impl FnOnce(&mut NFT) -> Result<T, E>,
	) -> Result<T, E>
	where
		NFT: Get<Self::ClassId> + Get<Self::Version> + Encode + Decode,
		E: From<DispatchError>,
	{
		let mut nft = Self::get_protocol_nft(instance_id)?;
		let r = f(&mut nft)?;
		Self::NFTProvider::set_typed_attribute(
			&NFT::get(),
			instance_id,
			&<NFT as Get<Self::Version>>::get(),
			&nft,
		)?;
		Ok(r)
	}

	/// Destroy the given NFT. Irreversible operation.
	///
	/// Arguments
	///
	/// * `instance_id` the NFT instance to destroy.
	fn burn_protocol_nft<NFT>(instance_id: &Self::InstanceId) -> DispatchResult
	where
		NFT: Get<Self::ClassId>,
	{
		Self::NFTProvider::burn_from(&NFT::get(), instance_id)
	}
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
