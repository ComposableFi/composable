// #![cfg_attr(
// 	not(test),
// 	deny(
// 		clippy::disallowed_methods,
// 		clippy::disallowed_types,
// 		clippy::indexing_slicing,
// 		clippy::todo,
// 		clippy::unwrap_used,
// 		clippy::panic
// 	)
// )]
// #![deny(clippy::unseparated_literal_suffix, unused_imports, non_snake_case, dead_code)]

#![cfg_attr(not(feature = "std"), no_std)]

pub mod governance;
pub mod impls;
pub mod topology;
pub mod xcmp;
pub mod fees;
mod prelude;

use core::marker::PhantomData;

#[cfg(not(feature = "runtime-benchmarks"))]
use composable_traits::currency::AssetExistentialDepositInspect;
use composable_traits::{defi::Ratio, };
pub use constants::*;
use frame_support::{parameter_types, weights::Weight};
use num_traits::CheckedMul;
use primitives::currency::CurrencyId;
use scale_info::TypeInfo;
use sp_runtime::DispatchError;
pub use types::*;

/// Common types of statemint and statemine and dali and picasso and composable.
mod types {
	use codec::{Decode, Encode, MaxEncodedLen};
	use core::fmt::Debug;
	use scale_info::TypeInfo;
	use sp_runtime::traits::{IdentifyAccount, Verify};

	// todo move it into more shared directory so it can be shared with
	// tests, integration, benchmark, (simnode?)

	pub type BondOfferId = u128;

	/// Pablo pool ID
	pub type PoolId = u128;

	/// Timestamp implementation.
	pub type Moment = u64;

	/// An index to a block.
	pub type BlockNumber = u32;

	/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
	pub type Signature = sp_runtime::MultiSignature;

	/// Some way of identifying an account on the chain. We intentionally make it equivalent
	/// to the public key of our transaction signing scheme.
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

	/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
	/// never know...
	pub type AccountIndex = u32;

	/// Balance of an account.
	pub type Balance = u128;

	/// Identifier for a fNFT
	pub type FinancialNftInstanceId = u64;

	/// An amount
	pub type Amount = i128;

	/// Index of a transaction in the chain.
	pub type Index = u32;

	/// The address format for describing accounts.
	pub type Address = sp_runtime::MultiAddress<AccountId, AccountIndex>;

	/// A hash of some data used by the chain.
	pub type Hash = sp_core::H256;

	/// Digest item type.
	pub type DigestItem = sp_runtime::generic::DigestItem;

	// Aura consensus authority.
	pub type AuraId = sp_consensus_aura::sr25519::AuthorityId;

	/// Concrete header
	pub type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;

	/// Opaque block
	pub type OpaqueBlock = sp_runtime::generic::Block<Header, sp_runtime::OpaqueExtrinsic>;

	#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum MosaicRemoteAssetId {
		EthereumTokenAddress([u8; 20]),
	}

	impl From<[u8; 20]> for MosaicRemoteAssetId {
		fn from(x: [u8; 20]) -> Self {
			MosaicRemoteAssetId::EthereumTokenAddress(x)
		}
	}

	pub type NftInstanceId = u128;

	pub type PositionId = u128;

	pub type ForeignAssetId = composable_traits::xcm::assets::XcmAssetLocation;
}

/// Common constants of statemint and statemine
mod constants {
	use super::types::BlockNumber;
	use frame_support::weights::{constants::WEIGHT_PER_SECOND, Weight};
	use sp_runtime::Perbill;

	/// This determines the average expected block time that we are targeting. Blocks will be
	/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
	/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
	/// slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: u32 = 12000;
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK as u64;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
	/// used to limit the maximal weight of a single extrinsic.
	// TODO changed to be more in line with statemine
	pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);
	/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
	/// Operational  extrinsics.
	pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND / 2;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
}


parameter_types! {
	/// NOTE: do not reduce, as it will tell that some already stored vectors has smaller range of values
	#[derive(PartialEq, Eq, Copy, Clone, codec::Encode, codec::Decode, codec::MaxEncodedLen, Debug, TypeInfo)]
	pub const MaxStringSize: u32 = 100;
}
