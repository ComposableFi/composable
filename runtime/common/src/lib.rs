#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)]
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod impls;
pub mod xcmp;
use composable_traits::oracle::MinimalOracle;
pub use constants::*;
use frame_support::parameter_types;
use orml_traits::parameter_type_with_key;
use primitives::currency::CurrencyId;
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

	/// Council Instance
	pub type CouncilInstance = collective::Instance1;

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
}

/// Common constants of statemint and statemine
mod constants {
	use super::types::{AccountId, BlockNumber, CouncilInstance};
	use frame_support::{
		traits::EnsureOneOf,
		weights::{constants::WEIGHT_PER_SECOND, Weight},
	};
	use frame_system::EnsureRoot;
	use sp_core::u32_trait::{_1, _2};
	use sp_runtime::Perbill;

	/// This determines the average expected block time that we are targeting. Blocks will be
	/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
	/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
	/// slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: u64 = 12000;
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

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

	/// Origin for either root or half of general council
	pub type EnsureRootOrHalfCouncil = EnsureOneOf<
		EnsureRoot<AccountId>,
		collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilInstance>,
	>;
}

pub struct PriceConverter;

impl MinimalOracle for PriceConverter {
	type AssetId = CurrencyId;

	type Balance = Balance;

	fn get_price_inverse(
		asset_id: Self::AssetId,
		amount: Self::Balance,
	) -> Result<Self::Balance, sp_runtime::DispatchError> {
		match asset_id {
			CurrencyId::PICA => Ok(amount),
			CurrencyId::KSM => Ok(amount / 10),
			CurrencyId::kUSD => Ok(amount / 10),
			_ => Err(DispatchError::Other("cannot pay with given weight")),
		}
	}
}

//  cannot be zero as in benches it fails Invalid input: InsufficientBalance
fn native_existential_deposit() -> Balance {
	100 * CurrencyId::milli::<Balance>()
}

parameter_types! {
	/// Existential deposit (ED for short) is minimum amount an account has to hold to stay in state.
	pub NativeExistentialDeposit: Balance = native_existential_deposit();
}

#[cfg(feature = "runtime-benchmarks")]
pub fn multi_existential_deposits(_currency_id: &CurrencyId) -> Balance {
	// ISSUE:
	// Running benchmarks with non zero multideopist leads to fail in 3rd party pallet.
	// It is not cleary why it happens.pub const BaseXcmWeight: Weight = 100_000_000;
	// 2022-03-14 20:50:19 Running Benchmark: collective.set_members 2/1 1/1
	// Error:
	//   0: Invalid input: Account cannot exist with the funds that would be given
	use num_traits::Zero;
	Balance::zero()
}

#[cfg(not(feature = "runtime-benchmarks"))]
pub fn multi_existential_deposits(currency_id: &CurrencyId) -> Balance {
	PriceConverter::get_price_inverse(*currency_id, NativeExistentialDeposit::get())
		.unwrap_or(Balance::MAX) // TODO: here DEX call to pemissioned markets should come
}

parameter_type_with_key! {
	// Minimum amount an account has to hold to stay in state
	pub MultiExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		multi_existential_deposits(currency_id)
	};
}
