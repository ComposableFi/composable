//! Implements staking rewards protocol.
#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)]
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
mod prelude;
#[cfg(any(feature = "runtime-benchmarks", test))]
mod test;
pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use composable_traits::currency::{BalanceLike, CurrencyFactory};
	use frame_support::{traits::UnixTime, PalletId};

	use crate::{prelude::*, weights::WeightInfo};
	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {}
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The share type of pool. Is bigger than `Self::Balance`
		type Share: Parameter + Member + BalanceLike + FixedPointOperand;

		/// The reward balance type.
		type Balance: Parameter + Member + BalanceLike + FixedPointOperand;

		/// The reward pool ID type.
		type PoolId: Parameter + Member + Clone + FullCodec;

		/// The position id type.
		type PositionId: Parameter + Member + Clone + FullCodec;

		type MayBeAssetId: Parameter + Member + AssetIdLike + MaybeSerializeDeserialize + Ord;

		/// Is used to create staked asset per `Self::PoolId`
		type CurrencyFactory: CurrencyFactory<Self::MayBeAssetId, Self::Balance>;

		/// is used for rate based rewarding and position lock timing
		type UnixTime: UnixTime;

		/// the size of batch to take each time trying to release rewards
		#[pallet::constant]
		type ReleaseRewardsPoolsBatchSize: Get<u8>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);
}
