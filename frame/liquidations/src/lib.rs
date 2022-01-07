#![cfg_attr(not(test), warn(clippy::disallowed_method, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
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

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use codec::FullCodec;
	use composable_traits::{
		defi::{DeFiComposableConfig, DeFiEngine, SellEngine},
		lending::Lending,
		liquidation::Liquidation,
	};
	use frame_support::{
		traits::{IsType, UnixTime, Get},
		PalletId,
	};

	use sp_runtime::DispatchError;

	pub const PALLET_ID: PalletId = PalletId(*b"Liqudati");

	#[pallet::config]

	pub trait Config: frame_system::Config + DeFiComposableConfig {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type UnixTime: UnixTime;

		type Lending: Lending;

		type LiquidationStrategyId: Default + FullCodec;
		
		type OrderId: Default + FullCodec;

		type PalletId: Get<PalletId>;

	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		PositionWasSentToLiquidation {},
	}
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> DeFiEngine for Pallet<T> {
		type MayBeAssetId = T::MayBeAssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;
	}

	// #[pallet::genesis_build]

	impl<T: Config> Liquidation for Pallet<T> {
		
		type LiquidationStrategyId = T::LiquidationStrategyId;

		type OrderId = T::OrderId;

		fn liquidate(
				from_to: &Self::AccountId,
				order: composable_traits::defi::Sell<Self::MayBeAssetId, Self::Balance>,		
				configuration : Vec<Self::LiquidationStrategyId>,
			) -> Result<Self::OrderId, DispatchError> {
				todo!()
			}
	}
}
