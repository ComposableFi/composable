use codec::{Decode, Encode};
use core::fmt::Debug;
use frame_support::traits::{
	tokens::nonfungibles::{Create, Inspect, Mutate},
	Get,
};
use scale_info::TypeInfo;
use sp_runtime::{DispatchError, TokenError};

pub trait FinancialNFTProvider<AccountId>: Create<AccountId> + Mutate<AccountId> {
	///
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
		key: &K,
		value: &V,
	) -> Result<Self::InstanceId, DispatchError>;
}

/// Default interface used to interact with financial NFTs through a NFT provider.
///
/// The interface will always fully serialize/deserialize the NFT type with the NFT::Version as
/// single attribute key.
pub trait FinancialNFTProtocol {
	/// Abstract type of an account id.
	type AccountId: Copy + Eq;

	/// Abstract type of a class id.
	type ClassId: Encode + Decode + TypeInfo;

	/// Abstract type of an instance id. Used to uniquely identify NFTs.
	type InstanceId: Copy + Eq + Debug + Encode + Decode + TypeInfo;

	/// Abstract type of a version. Used to migrate NFT when updating their content.
	/// Migration must be done by the protocol operating on the NFT type.
	type Version: Encode + Decode + TypeInfo;

	/// NFT provider from which we load/store NFT's.
	type NFTProvider: FinancialNFTProvider<
		Self::AccountId,
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
		owner: &Self::AccountId,
		nft: &NFT,
	) -> Result<Self::InstanceId, DispatchError>
	where
		NFT: Get<Self::ClassId> + Get<Self::Version> + Encode,
	{
		Self::NFTProvider::mint_nft(&NFT::get(), owner, &<NFT as Get<Self::Version>>::get(), &nft)
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
		owner: &Self::AccountId,
		instance_id: &Self::InstanceId,
	) -> Result<(), DispatchError>
	where
		NFT: Get<Self::ClassId>,
	{
		match Self::NFTProvider::owner(&NFT::get(), instance_id) {
			Some(nft_owner) if nft_owner == *owner => Ok(()),
			_ => Err(DispatchError::BadOrigin),
		}
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
}

/// Default ClassId type used for NFTs.
#[repr(transparent)]
pub struct NFTClass(u8);

impl NFTClass {
	pub const CHAOS_STAKING: NFTClass = NFTClass(1);
}

/// Default Version type used for NFTs.
#[repr(transparent)]
pub struct NFTVersion(u8);

impl NFTVersion {
	pub const VERSION_1: NFTVersion = NFTVersion(1);
}

pub trait DefaultFinancialNFTProtocol:
	FinancialNFTProtocol<ClassId = NFTClass, Version = NFTVersion>
{
}
