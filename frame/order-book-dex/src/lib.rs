#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
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

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{dex::SimpleExchange, rate_model::LiftedFixedBalance};
	use frame_support::{Parameter, pallet_prelude::MaybeSerializeDeserialize, traits::UnixTime};
	use frame_system::pallet_prelude::*;
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Perquintill,
	};
	pub trait DeFiComposablePallet {
		type AssetId: FullCodec
		+ Eq
		+ PartialEq
		+ Copy
		+ MaybeSerializeDeserialize
		+ From<u128>
		+ Default;
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + DeFiComposablePallet {
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ FixedPointOperand
			+ Into<LiftedFixedBalance> // integer part not more than bits in this
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
		type UnixTime: UnixTime;
	}

	pub struct DexInitialization {}

	/// allows order to be diminished in requested price
	pub struct DutchAuctionsConfig {}

	pub enum OrderPrice<T: Config> {
		ExactPrice(T::Balance),
		Dutch(T::Balance, DutchAuctionsConfig),
	}

	pub enum OrderStatus {}

	/// Store on chain multi dictionary key (from, to, account) , dictionary per buy and sell
	pub struct Order<T: Config> {
		pub amount: T::Balance,
		pub price: OrderPrice<T>,
		pub time_stamp: T::UnixTime,
		pub trader: T::AccountId,
		pub status: OrderStatus,
	}
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);



	impl<T: Config> SimpleExchange for Pallet<T> {
		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type AccountId = T::Balance;

		type Error = Error<T>;

		fn price(asset_id: Self::AssetId) -> Option<Self::Balance> {
			todo!()
		}

		fn exchange(
			from: Self::AssetId,
			from_account: Self::AccountId,
			to: Self::AssetId,
			to_account: Self::AccountId,
			from_amount: Self::Balance,
			slippage: sp_runtime::Perbill,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}
	}
}
