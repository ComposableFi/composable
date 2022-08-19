//! # DEX Router Pallet
//!
//! Is used to add route to DEX for given asset_id's pair.
//! It is required to have permissioned approval of routes.

#![cfg_attr(not(test), warn(clippy::disallowed_methods, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock_fnft;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};

	use composable_support::math::safe::SafeArithmetic;
	use composable_traits::{
		defi::CurrencyPair,
		dex::{Amm, DexRoute, DexRouter, RedeemableAssets, RemoveLiquiditySimulationResult},
	};
	use core::fmt::Debug;
	use frame_support::{pallet_prelude::*, transactional, PalletId};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{CheckedAdd, One, Zero},
		DispatchResult,
	};
	use sp_std::{
		collections::{btree_map::BTreeMap, btree_set::BTreeSet},
		vec::Vec,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ Ord;
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Zero
			+ Ord
			+ SafeArithmetic;
		/// The maximum hops in the route.
		#[pallet::constant]
		type MaxHopsInRoute: Get<u32> + MaxEncodedLen + TypeInfo;
		type PoolId: FullCodec
			+ MaxEncodedLen
			+ Default
			+ TypeInfo
			+ Eq
			+ PartialEq
			+ Ord
			+ Copy
			+ Debug
			+ CheckedAdd
			+ Zero
			+ One;
		type Pablo: Amm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolId = Self::PoolId,
		>;

		/// Required origin to update route operations.
		type UpdateRouteOrigin: EnsureOrigin<Self::Origin>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type DexRoutes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Blake2_128Concat,
		T::AssetId,
		DexRoute<T::PoolId, T::MaxHopsInRoute>,
		OptionQuery,
	>;

	#[pallet::error]
	pub enum Error<T> {
		/// Number of hops in route exceeded maximum limit.
		MaxHopsExceeded,
		/// For given asset pair no route found.
		NoRouteFound,
		/// Unexpected node found while route validation.
		UnexpectedNodeFoundWhileValidation,
		/// Can not respect minimum amount requested.
		CanNotRespectMinAmountRequested,
		/// Unsupported operation.
		UnsupportedOperation,
		/// Route with possible loop is not allowed.
		LoopSuspectedInRouteUpdate,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RouteAdded {
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			route: Vec<T::PoolId>,
		},
		RouteDeleted {
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			route: Vec<T::PoolId>,
		},
		RouteUpdated {
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			old_route: Vec<T::PoolId>,
			updated_route: Vec<T::PoolId>,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create, update or remove route.
		/// On successful emits one of `RouteAdded`, `RouteUpdated` or `RouteDeleted`.
		#[pallet::weight(T::WeightInfo::update_route())]
		pub fn update_route(
			origin: OriginFor<T>,
			asset_pair: CurrencyPair<T::AssetId>,
			route: Option<BoundedVec<T::PoolId, T::MaxHopsInRoute>>,
		) -> DispatchResult {
			T::UpdateRouteOrigin::ensure_origin(origin)?;
			<Self as DexRouter<
				T::AssetId,
				T::PoolId,
				T::Balance,
				T::MaxHopsInRoute,
			>>::update_route(asset_pair, route)?;
			Ok(())
		}

		/// Exchange `amount` of quote asset for `asset_pair` via route found in router.
		/// On successful underlying DEX pallets will emit appropriate event
		#[pallet::weight(T::WeightInfo::exchange())]
		pub fn exchange(
			origin: OriginFor<T>,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
			min_receive: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::exchange(&who, asset_pair, asset_pair, amount, min_receive, false)?;
			Ok(())
		}

		/// Sell `amount` of quote asset for `asset_pair` via route found in router.
		/// On successful underlying DEX pallets will emit appropriate event.
		#[pallet::weight(T::WeightInfo::sell())]
		pub fn sell(
			origin: OriginFor<T>,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
			min_receive: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::sell(
				&who,
				asset_pair,
				asset_pair.base, /* will be ignored */
				amount,
				min_receive,
				false,
			)?;
			Ok(())
		}

		/// Buy `amount` of quote asset for `asset_pair` via route found in router.
		/// On successful underlying DEX pallets will emit appropriate event.
		#[pallet::weight(T::WeightInfo::buy())]
		pub fn buy(
			origin: OriginFor<T>,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
			min_receive: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::buy(
				&who,
				asset_pair,
				asset_pair.base, /* will be ignored */
				amount,
				min_receive,
				false,
			)?;
			Ok(())
		}

		/// Add liquidity to the underlying pablo pool.
		/// Works only for single pool route.
		#[pallet::weight(T::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			asset_pair: CurrencyPair<T::AssetId>,
			base_amount: T::Balance,
			quote_amount: T::Balance,
			min_mint_amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::add_liquidity(
				&who,
				asset_pair,
				base_amount,
				quote_amount,
				min_mint_amount,
				keep_alive,
			)?;
			Ok(())
		}

		/// Remove liquidity from the underlying pablo pool.
		/// Works only for single pool route.
		#[pallet::weight(T::WeightInfo::remove_liquidity())]
		pub fn remove_liquidity(
			origin: OriginFor<T>,
			asset_pair: CurrencyPair<T::AssetId>,
			lp_amount: T::Balance,
			min_base_amount: T::Balance,
			min_quote_amount: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::remove_liquidity(
				&who,
				asset_pair,
				lp_amount,
				min_base_amount,
				min_quote_amount,
			)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn validate_route(
			asset_pair: CurrencyPair<T::AssetId>,
			route: &BoundedVec<T::PoolId, T::MaxHopsInRoute>,
		) -> Result<(), DispatchError> {
			let mut pair_set = BTreeSet::<CurrencyPair<T::AssetId>>::new();
			route
				.iter()
				// starting with asset_pair.quote, make sure current node's quote
				// matches with previous node's base
				.try_fold(asset_pair.quote, |val, iter| {
					T::Pablo::currency_pair(*iter).and_then(
						|pair| -> Result<T::AssetId, DispatchError> {
							if !pair_set.insert(pair) {
								return Err(Error::<T>::LoopSuspectedInRouteUpdate.into())
							}
							if pair.quote == val {
								Ok(pair.base)
							} else {
								Err(Error::<T>::UnexpectedNodeFoundWhileValidation.into())
							}
						},
					)
				})
				// last node's base asset matches asset_pair's base
				.and_then(|val| -> Result<(), DispatchError> {
					if val == asset_pair.base {
						Ok(())
					} else {
						Err(Error::<T>::UnexpectedNodeFoundWhileValidation.into())
					}
				})
		}

		fn do_update_route(
			asset_pair: CurrencyPair<T::AssetId>,
			route: BoundedVec<T::PoolId, T::MaxHopsInRoute>,
		) -> Result<(), DispatchError> {
			Self::validate_route(asset_pair, &route)?;
			let existing_route = Self::get_route(asset_pair);
			let event = match existing_route {
				Some((old_route, reverse)) => {
					if reverse {
						DexRoutes::<T>::remove(asset_pair.quote, asset_pair.base);
					} else {
						DexRoutes::<T>::remove(asset_pair.base, asset_pair.quote);
					}
					Event::RouteUpdated {
						x_asset_id: asset_pair.base,
						y_asset_id: asset_pair.quote,
						old_route,
						updated_route: route.to_vec(),
					}
				},
				None => Event::RouteAdded {
					x_asset_id: asset_pair.base,
					y_asset_id: asset_pair.quote,
					route: route.to_vec(),
				},
			};
			DexRoutes::<T>::insert(asset_pair.base, asset_pair.quote, DexRoute::Direct(route));
			Self::deposit_event(event);
			Ok(())
		}

		fn do_delete_route(asset_pair: CurrencyPair<T::AssetId>) -> Result<(), DispatchError> {
			let mut base_asset = asset_pair.base;
			let mut quote_asset = asset_pair.quote;
			if let Some((to_be_deleted_route, reverse)) = Self::get_route(asset_pair) {
				if reverse {
					quote_asset = asset_pair.quote;
					base_asset = asset_pair.base;
				}
				DexRoutes::<T>::remove(base_asset, quote_asset);
				Self::deposit_event(Event::RouteDeleted {
					x_asset_id: base_asset,
					y_asset_id: quote_asset,
					route: to_be_deleted_route,
				});
			}
			Ok(())
		}
	}

	impl<T: Config> DexRouter<T::AssetId, T::PoolId, T::Balance, T::MaxHopsInRoute> for Pallet<T> {
		#[transactional]
		fn update_route(
			asset_pair: CurrencyPair<T::AssetId>,
			route: Option<BoundedVec<T::PoolId, T::MaxHopsInRoute>>,
		) -> Result<(), DispatchError> {
			match route {
				Some(bounded_route) => Self::do_update_route(asset_pair, bounded_route)?,
				None => Self::do_delete_route(asset_pair)?,
			}
			Ok(())
		}

		/// Returns pair of route and bool to indicate if route should be used in reverse direction
		/// with assets swapped.
		fn get_route(asset_pair: CurrencyPair<T::AssetId>) -> Option<(Vec<T::PoolId>, bool)> {
			DexRoutes::<T>::get(asset_pair.base, asset_pair.quote).map_or_else(
				|| {
					DexRoutes::<T>::get(asset_pair.quote, asset_pair.base)
						.map(|DexRoute::Direct(route)| (route.into_inner(), true))
				},
				|DexRoute::Direct(route)| Some((route.into_inner(), false)),
			)
		}
	}

	impl<T: Config> Amm for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		type AccountId = T::AccountId;
		type PoolId = CurrencyPair<T::AssetId>;

		fn pool_exists(pool_id: Self::PoolId) -> bool {
			DexRoutes::<T>::contains_key(pool_id.base, pool_id.quote) ||
				DexRoutes::<T>::contains_key(pool_id.quote, pool_id.base)
		}

		fn currency_pair(
			pool_id: Self::PoolId,
		) -> Result<CurrencyPair<Self::AssetId>, DispatchError> {
			if Self::pool_exists(pool_id) {
				Ok(pool_id)
			} else {
				Err(Error::<T>::NoRouteFound.into())
			}
		}

		fn lp_token(pool_id: Self::PoolId) -> Result<Self::AssetId, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::lp_token(pool_id),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		fn get_exchange_value(
			pool_id: Self::PoolId,
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::get_exchange_value(pool_id, asset_id, amount),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		fn simulate_add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<Self::Balance, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::simulate_add_liquidity(who, pool_id, amounts),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		fn redeemable_assets_for_lp_tokens(
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_expected_amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<RedeemableAssets<Self::AssetId, Self::Balance>, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::redeemable_assets_for_lp_tokens(
					pool_id,
					lp_amount,
					min_expected_amounts,
				),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		fn simulate_remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_expected_amounts: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<RemoveLiquiditySimulationResult<Self::AssetId, Self::Balance>, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::simulate_remove_liquidity(
					who,
					pool_id,
					lp_amount,
					min_expected_amounts,
				),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		#[transactional]
		fn exchange(
			who: &Self::AccountId,
			_pool_id: Self::PoolId,
			asset_pair: CurrencyPair<Self::AssetId>,
			quote_amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let (route, reverse) = Self::get_route(asset_pair).ok_or(Error::<T>::NoRouteFound)?;
			let mut dx_t = quote_amount;
			let mut dy_t = T::Balance::zero();
			let mut forward_iter;
			let mut backward_iter;
			let route_iter: &mut dyn Iterator<Item = &T::PoolId> = if !reverse {
				forward_iter = route.iter();
				&mut forward_iter
			} else {
				backward_iter = route.iter().rev();
				&mut backward_iter
			};
			for pool_id in route_iter {
				let mut currency_pair = T::Pablo::currency_pair(*pool_id)?;
				if reverse {
					currency_pair = currency_pair.swap();
				}
				dy_t = T::Pablo::exchange(
					who,
					*pool_id,
					currency_pair,
					dx_t,
					T::Balance::zero(),
					keep_alive,
				)?;
				dx_t = dy_t;
			}
			ensure!(dy_t >= min_receive, Error::<T>::CanNotRespectMinAmountRequested);
			Ok(dy_t)
		}

		#[transactional]
		fn sell(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			_asset_id: Self::AssetId,
			amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			<Self as Amm>::exchange(who, pool_id, pool_id, amount, min_receive, keep_alive)
		}

		#[transactional]
		fn buy(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			_asset_id: Self::AssetId,
			amount: Self::Balance,
			min_receive: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let (route, reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			let mut dy_t = amount;
			let mut dx_t = T::Balance::zero();
			let mut forward_iter;
			let mut backward_iter;
			let route_iter: &mut dyn Iterator<Item = &T::PoolId> = if !reverse {
				backward_iter = route.iter().rev();
				&mut backward_iter
			} else {
				forward_iter = route.iter();
				&mut forward_iter
			};
			for pool_id in route_iter {
				let mut currency_pair = T::Pablo::currency_pair(*pool_id)?;
				if reverse {
					currency_pair = currency_pair.swap();
				}
				dx_t = T::Pablo::get_exchange_value(*pool_id, currency_pair.base, dy_t)?;
				dy_t = dx_t;
			}
			let route_iter: &mut dyn Iterator<Item = &T::PoolId> = if !reverse {
				forward_iter = route.iter();
				&mut forward_iter
			} else {
				backward_iter = route.iter().rev();
				&mut backward_iter
			};
			for pool_id in route_iter {
				let mut currency_pair = T::Pablo::currency_pair(*pool_id)?;
				if reverse {
					currency_pair = currency_pair.swap();
				}
				let dy_t = T::Pablo::exchange(
					who,
					*pool_id,
					currency_pair,
					dx_t,
					T::Balance::zero(),
					keep_alive,
				)?;
				dx_t = dy_t;
			}
			ensure!(dx_t >= min_receive, Error::<T>::CanNotRespectMinAmountRequested);
			Ok(dx_t)
		}

		#[transactional]
		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			base_amount: Self::Balance,
			quote_amount: Self::Balance,
			min_mint_amount: Self::Balance,
			keep_alive: bool,
		) -> Result<(), DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::add_liquidity(
					who,
					pool_id,
					base_amount,
					quote_amount,
					min_mint_amount,
					keep_alive,
				),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		#[transactional]
		fn remove_liquidity(
			who: &T::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_base_amount: Self::Balance,
			min_quote_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::remove_liquidity(
					who,
					pool_id,
					lp_amount,
					min_base_amount,
					min_quote_amount,
				),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}
	}
}
