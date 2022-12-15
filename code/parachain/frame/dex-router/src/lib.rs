//! # DEX Router Pallet
//!
//! Is used to add route to DEX for given asset_id's pair.
//! It is required to have permissioned approval of routes.

#![cfg_attr(not(test), warn(clippy::disallowed_methods, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]

use composable_traits::defi::CurrencyPair;
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod mock_fnft;
#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};

	use crate::pool_id_pair;
	use composable_support::math::safe::SafeArithmetic;
	use composable_traits::{
		defi::CurrencyPair,
		dex::{Amm, AssetAmount, DexRoute, DexRouter, RedeemableAssets, SwapResult},
	};
	use core::fmt::Debug;
	use frame_support::{pallet_prelude::*, transactional, PalletId};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_arithmetic::Permill;
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
		/// Only dual asset pools supported
		OnlyDualAssetPoolsSupported,
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
		#[pallet::weight(T::WeightInfo::swap())]
		pub fn swap(
			origin: OriginFor<T>,
			in_asset: AssetAmount<T::AssetId, T::Balance>,
			min_receive: AssetAmount<T::AssetId, T::Balance>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::do_swap(
				&who,
				pool_id_pair::<T>(in_asset.asset_id, min_receive.asset_id),
				in_asset,
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
			in_asset_id: T::AssetId,
			out_asset: AssetAmount<T::AssetId, T::Balance>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as Amm>::do_buy(
				&who,
				pool_id_pair::<T>(in_asset_id, out_asset.asset_id),
				in_asset_id,
				out_asset,
				false,
			)?;
			Ok(())
		}

		/// Add liquidity to the underlying pablo pool.
		/// Works only for single pool route.
		#[pallet::weight(T::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			assets: BTreeMap<T::AssetId, T::Balance>,
			min_mint_amount: T::Balance,
			keep_alive: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(assets.len() == 2, Error::<T>::OnlyDualAssetPoolsSupported);
			let assets_vec = assets.keys().copied().collect::<Vec<_>>();
			let asset_pair = pool_id_pair::<T>(
				*assets_vec.get(0).expect("Must exist"),
				*assets_vec.get(1).expect("Must exist"),
			);
			<Self as Amm>::add_liquidity(&who, asset_pair, assets, min_mint_amount, keep_alive)?;
			Ok(())
		}

		/// Remove liquidity from the underlying pablo pool.
		/// Works only for single pool route.
		#[pallet::weight(T::WeightInfo::remove_liquidity())]
		pub fn remove_liquidity(
			origin: OriginFor<T>,
			lp_amount: T::Balance,
			min_receive: BTreeMap<T::AssetId, T::Balance>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(min_receive.len() == 2, Error::<T>::OnlyDualAssetPoolsSupported);
			let assets_vec = min_receive.keys().copied().collect::<Vec<_>>();
			let asset_pair = pool_id_pair::<T>(
				*assets_vec.get(0).expect("Must exist"),
				*assets_vec.get(1).expect("Must exist"),
			);

			<Self as Amm>::remove_liquidity(&who, asset_pair, lp_amount, min_receive)?;
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
				.try_fold(asset_pair.quote, |val, pool| {
					T::Pablo::assets(*pool).and_then(
						|assets| -> Result<T::AssetId, DispatchError> {
							ensure!(assets.len() == 2, Error::<T>::OnlyDualAssetPoolsSupported);
							let keys = assets.into_keys().collect::<Vec<T::AssetId>>();
							let first_asset = keys.get(0).copied().expect("Must exist");
							let second_asset = keys.get(1).copied().expect("Must exist");
							let pair = CurrencyPair::new(second_asset, first_asset);
							if !pair_set.insert(pair) {
								return Err(Error::<T>::LoopSuspectedInRouteUpdate.into())
							}
							if pair.quote == val {
								Ok(pair.base)
							} else if pair.base == val {
								Ok(pair.quote)
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

		fn assets(
			pool_id: Self::PoolId,
		) -> Result<BTreeMap<Self::AssetId, Permill>, DispatchError> {
			if Self::pool_exists(pool_id) {
				Ok([(pool_id.base, Permill::zero()), (pool_id.quote, Permill::zero())].into())
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

		fn spot_price(
			pool_id: Self::PoolId,
			base_asset: AssetAmount<Self::AssetId, Self::Balance>,
			quote_asset_id: Self::AssetId,
			calculate_with_fees: bool,
		) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] =>
					T::Pablo::spot_price(pool_id, base_asset, quote_asset_id, calculate_with_fees),
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
		) -> Result<RedeemableAssets<Self::AssetId, Self::Balance>, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::redeemable_assets_for_lp_tokens(pool_id, lp_amount),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		fn simulate_remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
		) -> Result<BTreeMap<Self::AssetId, Self::Balance>, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::simulate_remove_liquidity(who, pool_id, lp_amount),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		#[transactional]
		fn do_swap(
			who: &Self::AccountId,
			_pool_id: Self::PoolId,
			in_asset: AssetAmount<Self::AssetId, Self::Balance>,
			min_receive: AssetAmount<Self::AssetId, Self::Balance>,
			keep_alive: bool,
		) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError> {
			let currency_pair = CurrencyPair::new(min_receive.asset_id, in_asset.asset_id);
			let (route, reverse) =
				Self::get_route(currency_pair).ok_or(Error::<T>::NoRouteFound)?;
			let mut forward_iter;
			let mut backward_iter;
			// Iterate forward or backward depending on the reverse flag to find the pools to swap
			// with.
			let route_iter: &mut dyn Iterator<Item = &T::PoolId> = if !reverse {
				forward_iter = route.iter();
				&mut forward_iter
			} else {
				backward_iter = route.iter().rev();
				&mut backward_iter
			};
			// Iterate and swap until we obtain the required asset in the `min_receive.asset_id`
			let mut in_asset_itr = in_asset;
			let mut swap_result: SwapResult<T::AssetId, T::Balance> = SwapResult {
				value: in_asset_itr,
				fee: AssetAmount { asset_id: in_asset_itr.asset_id, amount: T::Balance::zero() },
			};
			for pool_id in route_iter {
				let assets = T::Pablo::assets(*pool_id)?;
				// We only allow dual asset pools in routes, therefore taking the remaining asset
				// other than `in_asset_itr.asset_id` gives us the out_asset_id
				let out_asset_id = assets
					.keys()
					.copied()
					.find(|a| *a != in_asset_itr.asset_id)
					.ok_or(Error::<T>::NoRouteFound)?;
				swap_result = T::Pablo::do_swap(
					who,
					*pool_id,
					in_asset_itr,
					AssetAmount::new(out_asset_id, T::Balance::zero()),
					keep_alive,
				)?;
				in_asset_itr = swap_result.value;
			}
			ensure!(
				swap_result.value.amount >= min_receive.amount,
				Error::<T>::CanNotRespectMinAmountRequested
			);
			// TODO (vim): Final fee amount is not correct as the fee need to be incremented with
			// each swap fee when iterating.
			Ok(swap_result)
		}

		#[transactional]
		fn do_buy(
			who: &Self::AccountId,
			_pool_id: Self::PoolId,
			in_asset_id: Self::AssetId,
			out_asset: AssetAmount<Self::AssetId, Self::Balance>,
			keep_alive: bool,
		) -> Result<SwapResult<Self::AssetId, Self::Balance>, DispatchError> {
			let currency_pair = CurrencyPair::new(out_asset.asset_id, in_asset_id);
			let (route, reverse) =
				Self::get_route(currency_pair).ok_or(Error::<T>::NoRouteFound)?;

			// Iterate and calculate spot price until we reach the `in_asset` amount required
			let mut forward_iter;
			let mut backward_iter;
			let route_iter: &mut dyn Iterator<Item = &T::PoolId> = if !reverse {
				backward_iter = route.iter().rev();
				&mut backward_iter
			} else {
				forward_iter = route.iter();
				&mut forward_iter
			};
			let mut in_asset_itr = out_asset;
			let mut in_asset: SwapResult<T::AssetId, T::Balance> = SwapResult {
				value: in_asset_itr,
				fee: AssetAmount { asset_id: in_asset_itr.asset_id, amount: T::Balance::zero() },
			};
			for pool_id in route_iter {
				let assets = T::Pablo::assets(*pool_id)?;
				// We only allow dual asset pools in routes, therefore taking the remaining asset
				// other than `in_asset_itr.asset_id` gives us the out_asset_id
				let quote_asset_id = assets
					.keys()
					.copied()
					.find(|a| *a != in_asset_itr.asset_id)
					.ok_or(Error::<T>::NoRouteFound)?;
				in_asset = T::Pablo::spot_price(*pool_id, in_asset_itr, quote_asset_id, false)?;
				in_asset_itr = in_asset.value;
			}

			// Iterate and swap until we reach the out_asset amount required
			let route_iter: &mut dyn Iterator<Item = &T::PoolId> = if !reverse {
				forward_iter = route.iter();
				&mut forward_iter
			} else {
				backward_iter = route.iter().rev();
				&mut backward_iter
			};
			let mut out_asset_itr = in_asset;
			for pool_id in route_iter {
				let assets = T::Pablo::assets(*pool_id)?;
				let out_asset_id = assets
					.keys()
					.copied()
					.find(|a| *a != out_asset_itr.value.asset_id)
					.ok_or(Error::<T>::NoRouteFound)?;
				out_asset_itr = T::Pablo::do_swap(
					who,
					*pool_id,
					out_asset_itr.value,
					AssetAmount::new(out_asset_id, T::Balance::zero()),
					keep_alive,
				)?;
			}
			// TODO (vim): Final fee amount is not correct as the fee need to be incremented with
			//  each swap fee when iterating.
			Ok(out_asset_itr)
		}

		#[transactional]
		fn add_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			assets: BTreeMap<Self::AssetId, Self::Balance>,
			min_mint_amount: Self::Balance,
			keep_alive: bool,
		) -> Result<Self::Balance, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] =>
					T::Pablo::add_liquidity(who, pool_id, assets, min_mint_amount, keep_alive),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}

		#[transactional]
		fn remove_liquidity(
			who: &Self::AccountId,
			pool_id: Self::PoolId,
			lp_amount: Self::Balance,
			min_receive: BTreeMap<Self::AssetId, Self::Balance>,
		) -> Result<BTreeMap<Self::AssetId, Self::Balance>, DispatchError> {
			let (route, _reverse) = Self::get_route(pool_id).ok_or(Error::<T>::NoRouteFound)?;
			match route[..] {
				[pool_id] => T::Pablo::remove_liquidity(who, pool_id, lp_amount, min_receive),
				_ => Err(Error::<T>::UnsupportedOperation.into()),
			}
		}
	}
}

/// Create a pool_id pair with convention of ordering base/quote by assetId.
pub(crate) fn pool_id_pair<T: Config>(
	first_asset_id: T::AssetId,
	second_asset_id: T::AssetId,
) -> CurrencyPair<T::AssetId> {
	if first_asset_id < second_asset_id {
		CurrencyPair::new(first_asset_id, second_asset_id)
	} else {
		CurrencyPair::new(second_asset_id, first_asset_id)
	}
}
