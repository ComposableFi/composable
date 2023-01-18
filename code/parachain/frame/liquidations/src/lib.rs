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
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	bad_style,
	bare_trait_objects,
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
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;

pub use crate::weights::WeightInfo;

pub use pallet::*;

/// TODO: add here backward mapping registry assets interface
#[frame_support::pallet]
pub mod pallet {
	use codec::{Decode, Encode, FullCodec, MaxEncodedLen};
	use composable_support::{
		abstractions::{
			nonce::Nonce,
			utils::{
				increment::{Increment, WrappingIncrement},
				start_at::DefaultInit,
				ValueQuery,
			},
		},
		math::wrapping_next::WrappingNext,
	};
	use composable_traits::{
		defi::{DeFiComposableConfig, DeFiEngine, Sell, SellEngine},
		liquidation::Liquidation,
		time::{LinearDecrease, StairstepExponentialDecrease, TimeReleaseFunction},
	};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::{OptionQuery, StorageMap, StorageValue},
		traits::{EnsureOrigin, Get, IsType, UnixTime},
		BoundedVec, PalletId, Parameter, Twox64Concat,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use scale_info::TypeInfo;
	use sp_runtime::{DispatchError, Permill, Perquintill};
	use sp_std::vec::Vec;

	#[cfg(feature = "std")]
	use frame_support::traits::GenesisBuild;

	use crate::weights::WeightInfo;

	#[pallet::config]

	pub trait Config: frame_system::Config + DeFiComposableConfig {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type UnixTime: UnixTime;

		type DutchAuction: SellEngine<
			TimeReleaseFunction,
			OrderId = Self::OrderId,
			MayBeAssetId = <Self as DeFiComposableConfig>::MayBeAssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type LiquidationStrategyId: Default
			+ FullCodec
			+ MaxEncodedLen
			+ WrappingNext
			+ Parameter
			+ Copy
			+ From<u32>;

		type OrderId: Default + FullCodec + MaxEncodedLen + sp_std::fmt::Debug;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		// /// when called, engine pops latest order to liquidate and pushes back result
		// type Liquidate: Parameter + Dispatchable<Origin = Self::RuntimeOrigin> +
		// From<Call<Self>>;
		type WeightInfo: WeightInfo;

		/// is used to talk to external liquidation engines
		type XcmSender: xcm::latest::SendXcm;

		type CanModifyStrategies: EnsureOrigin<Self::RuntimeOrigin>;
		type MaxLiquidationStrategiesAmount: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		PositionWasSentToLiquidation {},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoLiquidationEngineFound,
		InvalidLiquidationStrategiesVector,
		OnlyDutchAuctionStrategyIsImplemented,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_liquidation_strategy())]
		pub fn add_liquidation_strategy(
			origin: OriginFor<T>,
			// TODO: make it validated
			// TODO: User parachains pallet to validate parachain is connected
			// TODO: use hardcoded swap interface to validate native token is supported
			configuration: LiquidationStrategyConfiguration,
		) -> DispatchResultWithPostInfo {
			T::CanModifyStrategies::ensure_origin(origin)?;
			let index = StrategyIndex::<T>::increment();
			Strategies::<T>::insert(index, configuration);
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::sell(configuration.len().max(1) as u32))]
		pub fn sell(
			origin: OriginFor<T>,
			order: Sell<T::MayBeAssetId, T::Balance>,
			configuration: Vec<T::LiquidationStrategyId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			Self::liquidate(&who, order, configuration)?;
			Ok(().into())
		}

		// TODO: Add API to manage callback from liquidation engine and managing it state
		// TODO: each step from request to have its slots so can tackle
		// TODO: add incentivised API to allow "progress" finalization if it stalled (or OCW)
	}

	#[pallet::storage]
	#[pallet::getter(fn strategies)]
	pub type Strategies<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::LiquidationStrategyId,
		LiquidationStrategyConfiguration,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn strategy_index)]
	#[allow(clippy::disallowed_types)]
	pub type StrategyIndex<T: Config> = StorageValue<
		_,
		T::LiquidationStrategyId,
		ValueQuery,
		Nonce<DefaultInit, WrappingIncrement>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn default_strategy_index)]
	#[allow(clippy::disallowed_types)]
	pub type DefaultStrategyIndex<T: Config> =
		StorageValue<_, T::LiquidationStrategyId, ValueQuery>;

	impl<T: Config> DeFiEngine for Pallet<T> {
		type MayBeAssetId = T::MayBeAssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;
	}

	#[cfg(feature = "std")]
	#[derive(Default)]
	#[pallet::genesis_config]
	pub struct GenesisConfig;

	impl<T: Config> Pallet<T> {
		pub fn create_strategy_id() -> T::LiquidationStrategyId {
			StrategyIndex::<T>::increment()
		}
	}

	#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum LiquidationStrategyConfiguration {
		DutchAuction(TimeReleaseFunction),
		Pablo { slippage: Perquintill },
		Xcm(composable_traits::xcm::XcmSellRequestTransactConfiguration),
	}

	#[cfg(feature = "std")]
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			// DutchAction
			let index = StrategyIndex::<T>::increment();
			DefaultStrategyIndex::<T>::set(index);
			let linear_ten_minutes = LiquidationStrategyConfiguration::DutchAuction(
				TimeReleaseFunction::LinearDecrease(LinearDecrease { total: 10 * 60 }),
			);
			Strategies::<T>::insert(index, linear_ten_minutes);

			let index = StrategyIndex::<T>::increment();
			let exponential =
				StairstepExponentialDecrease { step: 10, cut: Permill::from_rational(95_u32, 100) };
			let exponential = LiquidationStrategyConfiguration::DutchAuction(
				TimeReleaseFunction::StairstepExponentialDecrease(exponential),
			);
			Strategies::<T>::insert(index, exponential);
		}
	}

	impl<T: Config> Liquidation for Pallet<T> {
		type LiquidationStrategyId = T::LiquidationStrategyId;
		type OrderId = T::OrderId;

		fn liquidate(
			from_to: &Self::AccountId,
			order: Sell<Self::MayBeAssetId, Self::Balance>,
			configuration: Vec<Self::LiquidationStrategyId>,
		) -> Result<T::OrderId, DispatchError> {
			let configuration = BoundedVec::try_from(configuration)
				.map_err(|()| Error::<T>::InvalidLiquidationStrategiesVector)?;
			Self::do_liquidate(from_to, order, configuration)
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_liquidate(
			from_to: &T::AccountId,
			order: Sell<T::MayBeAssetId, T::Balance>,
			configuration: BoundedVec<T::LiquidationStrategyId, T::MaxLiquidationStrategiesAmount>,
		) -> Result<T::OrderId, DispatchError> {
			let mut configuration = configuration;
			if configuration.is_empty() {
				configuration
					.try_push(DefaultStrategyIndex::<T>::get())
					.map_err(|()| Error::<T>::InvalidLiquidationStrategiesVector)?;
			};
			for id in configuration {
				let configuration = Strategies::<T>::get(id);
				if let Some(configuration) = configuration {
					let result = match configuration {
						LiquidationStrategyConfiguration::DutchAuction(configuration) =>
							T::DutchAuction::ask(from_to, order.clone(), configuration),
						_ => return Err(Error::<T>::OnlyDutchAuctionStrategyIsImplemented.into()),
					};
					if let Ok(order_id) = result {
						Self::deposit_event(Event::<T>::PositionWasSentToLiquidation {});
						return Ok(order_id)
					}
				}
			}

			Err(Error::<T>::NoLiquidationEngineFound.into())
		}
	}
}
