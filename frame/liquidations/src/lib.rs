#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
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
		defi::DeFiComposableConfig, lending::Lending, liquidation::Liquidation,
		loans::PriceStructure,
	};
	use frame_support::{
		traits::{IsType, UnixTime},
		PalletId,
	};

	use sp_runtime::DispatchError;

	pub const PALLET_ID: PalletId = PalletId(*b"Liqudati");

	#[pallet::config]

	pub trait Config: frame_system::Config + DeFiComposableConfig {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type UnixTime: UnixTime;

		type Lending: Lending;

		type GroupId: Default + FullCodec;
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

	impl<T: Config> Liquidation for Pallet<T> {
		type AssetId = T::MayBeAssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;

		type LiquidationId = u128;

		type GroupId = T::GroupId;

		fn liquidate(
			_source_account: &Self::AccountId,
			_source_asset_id: Self::AssetId,
			_source_asset_price: PriceStructure<Self::GroupId, Self::Balance>,
			_target_asset_id: Self::AssetId,
			_target_account: &Self::AccountId,
			_total_amount: Self::Balance,
		) -> Result<Self::LiquidationId, DispatchError> {
			Self::deposit_event(Event::<T>::PositionWasSentToLiquidation {});
			Err(DispatchError::Other("todo"))
		}
	}
}
