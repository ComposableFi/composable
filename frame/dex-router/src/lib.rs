//! # DEX Router Pallet
//!
//! Is used to add route to DEX for given asset_id's pair.
//! It is required to have permissioned approval of routes.

#![cfg_attr(not(test), warn(clippy::disallowed_method, clippy::indexing_slicing))] // allow in tests
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

pub mod weights;
pub use crate::weights::WeightInfo;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use crate::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_traits::{
		defi::CurrencyPair,
		dex::{Amm, DexRoute, DexRouteNode, DexRouter},
		math::SafeArithmetic,
	};
	use core::fmt::Debug;
	use frame_support::{pallet_prelude::*, transactional};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{CheckedAdd, One, Zero},
		DispatchResult,
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
		type StableSwapDex: Amm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolId = Self::PoolId,
		>;
		type ConstantProductDex: Amm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolId = Self::PoolId,
		>;

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
		/// A Pool provided as part of route does not exist.
		PoolDoesNotExist,
		/// For given asset pair no route found.
		NoRouteFound,
		/// Exchanged amount is less than expected minimum.
		CannotRespectMinimumRequested,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RouteAdded {
			who: T::AccountId,
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			route: Vec<DexRouteNode<T::PoolId>>,
		},
		RouteDeleted {
			who: T::AccountId,
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			route: Vec<DexRouteNode<T::PoolId>>,
		},
		RouteUpdated {
			who: T::AccountId,
			x_asset_id: T::AssetId,
			y_asset_id: T::AssetId,
			old_route: Vec<DexRouteNode<T::PoolId>>,
			updated_route: Vec<DexRouteNode<T::PoolId>>,
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
			route: Option<BoundedVec<DexRouteNode<T::PoolId>, T::MaxHopsInRoute>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as DexRouter<
				T::AccountId,
				T::AssetId,
				T::PoolId,
				T::Balance,
				T::MaxHopsInRoute,
			>>::update_route(&who, asset_pair, route)?;
			Ok(())
		}

		/// Exchange `amount` of quote asset for `asset_pair` via route found in router.
		/// On successful underlying DEX pallets will emit appropriate event.
		#[pallet::weight(T::WeightInfo::exchange())]
		pub fn exchange(
			origin: OriginFor<T>,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
			min_receive: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _ = <Self as DexRouter<
				T::AccountId,
				T::AssetId,
				T::PoolId,
				T::Balance,
				T::MaxHopsInRoute,
			>>::exchange(&who, asset_pair, amount, min_receive)?;
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
			let _ = <Self as DexRouter<
				T::AccountId,
				T::AssetId,
				T::PoolId,
				T::Balance,
				T::MaxHopsInRoute,
			>>::sell(&who, asset_pair, amount, min_receive)?;
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
			let _ = <Self as DexRouter<
				T::AccountId,
				T::AssetId,
				T::PoolId,
				T::Balance,
				T::MaxHopsInRoute,
			>>::buy(&who, asset_pair, amount, min_receive)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_update_route(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			route: BoundedVec<DexRouteNode<T::PoolId>, T::MaxHopsInRoute>,
		) -> Result<(), DispatchError> {
			let k1 = asset_pair.base;
			let k2 = asset_pair.quote;
			for r in route.as_slice() {
				match r {
					DexRouteNode::Curve(pool_id) => {
						ensure!(
							T::StableSwapDex::pool_exists(*pool_id),
							Error::<T>::PoolDoesNotExist
						)
					},
					DexRouteNode::Uniswap(pool_id) => {
						ensure!(
							T::ConstantProductDex::pool_exists(*pool_id),
							Error::<T>::PoolDoesNotExist
						)
					},
				}
			}
			let existing_route = DexRoutes::<T>::get(k1, k2);

			DexRoutes::<T>::insert(k1, k2, DexRoute::Direct(route.clone()));
			let event = match existing_route {
				Some(DexRoute::Direct(old_route)) => Event::RouteUpdated {
					who: who.clone(),
					x_asset_id: k1,
					y_asset_id: k2,
					old_route: old_route.into_inner(),
					updated_route: route.to_vec(),
				},
				None => Event::RouteAdded {
					who: who.clone(),
					x_asset_id: k1,
					y_asset_id: k2,
					route: route.to_vec(),
				},
			};
			Self::deposit_event(event);
			Ok(())
		}

		fn do_delete_route(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
		) -> Result<(), DispatchError> {
			let k1 = asset_pair.base;
			let k2 = asset_pair.quote;
			if let Some(DexRoute::Direct(deleted_route)) = DexRoutes::<T>::take(k1, k2) {
				Self::deposit_event(Event::RouteDeleted {
					who: who.clone(),
					x_asset_id: k1,
					y_asset_id: k2,
					route: deleted_route.into_inner(),
				});
			}

			Ok(())
		}
	}

	impl<T: Config> DexRouter<T::AccountId, T::AssetId, T::PoolId, T::Balance, T::MaxHopsInRoute>
		for Pallet<T>
	{
		#[transactional]
		fn update_route(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			route: Option<BoundedVec<DexRouteNode<T::PoolId>, T::MaxHopsInRoute>>,
		) -> Result<(), DispatchError> {
			match route {
				Some(bounded_route) => Self::do_update_route(who, asset_pair, bounded_route)?,
				None => Self::do_delete_route(who, asset_pair)?,
			}
			Ok(())
		}

		fn get_route(asset_pair: CurrencyPair<T::AssetId>) -> Option<Vec<DexRouteNode<T::PoolId>>> {
			DexRoutes::<T>::get(asset_pair.base, asset_pair.quote)
				.map(|DexRoute::Direct(route)| route.into_inner())
		}

		#[transactional]
		fn exchange(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
			min_receive: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let route;
			let reverse;
			let res = Self::get_route(asset_pair);
			if res.is_some() {
				reverse = false;
				route = Self::get_route(asset_pair).ok_or(Error::<T>::NoRouteFound)?;
			} else {
				route = Self::get_route(asset_pair.swap()).ok_or(Error::<T>::NoRouteFound)?;
				reverse = true;
			}
			// let route =
			// Self::get_route(asset_pair).ok_or(Self::get_route(asset_pair.swap()).ok_or(Error::
			// <T>::NoRouteFound))?;
			let mut dx_t = amount;
			let mut dy_t = T::Balance::zero();
			let mut forward_iter;
			let mut backward_iter;
			let route_iter: &mut dyn Iterator<Item = &DexRouteNode<_>> = if !reverse {
				forward_iter = route.iter();
				&mut forward_iter
			} else {
				backward_iter = route.iter().rev();
				&mut backward_iter
			};
			for route_node in route_iter {
				match route_node {
					DexRouteNode::Curve(pool_id) => {
						let mut currency_pair = T::StableSwapDex::currency_pair(*pool_id)?;
						if reverse {
							currency_pair = currency_pair.swap();
						}
						dy_t = T::StableSwapDex::exchange(
							who,
							*pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)?;
						dx_t = dy_t;
					},
					DexRouteNode::Uniswap(pool_id) => {
						let mut currency_pair = T::ConstantProductDex::currency_pair(*pool_id)?;
						if reverse {
							currency_pair = currency_pair.swap();
						}
						dy_t = T::ConstantProductDex::exchange(
							who,
							*pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)?;
						dx_t = dy_t;
					},
				}
			}
			ensure!(dy_t >= min_receive, Error::<T>::CannotRespectMinimumRequested);
			Ok(dy_t)
		}

		#[transactional]
		fn sell(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
			min_receive: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			<Self as DexRouter<
				T::AccountId,
				T::AssetId,
				T::PoolId,
				T::Balance,
				T::MaxHopsInRoute,
			>>::exchange(who, asset_pair, amount, min_receive)
		}

		#[transactional]
		fn buy(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
			min_receive: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			// let route = Self::get_route(asset_pair).ok_or(Error::<T>::NoRouteFound)?;
			let route;
			let reverse;
			let res = Self::get_route(asset_pair);
			if res.is_some() {
				reverse = false;
				route = Self::get_route(asset_pair).ok_or(Error::<T>::NoRouteFound)?;
			} else {
				route = Self::get_route(asset_pair.swap()).ok_or(Error::<T>::NoRouteFound)?;
				reverse = true;
			}
			let mut forward_iter;
			let mut backward_iter;
			let route_iter: &mut dyn Iterator<Item = &DexRouteNode<_>> = if !reverse {
				backward_iter = route.iter().rev();
				&mut backward_iter
			} else {
				forward_iter = route.iter();
				&mut forward_iter
			};
			let mut dy_t = amount;
			let mut dx_t = T::Balance::zero();
			for route_node in route_iter {
				match route_node {
					DexRouteNode::Curve(pool_id) => {
						// let currency_pair = T::StableSwapDex::currency_pair(*pool_id)?;
						let mut currency_pair = T::StableSwapDex::currency_pair(*pool_id)?;
						if reverse {
							currency_pair = currency_pair.swap();
						}
						dx_t = T::StableSwapDex::get_exchange_value(
							*pool_id,
							currency_pair.base,
							dy_t,
						)?;
						dy_t = dx_t;
					},
					DexRouteNode::Uniswap(pool_id) => {
						let mut currency_pair = T::ConstantProductDex::currency_pair(*pool_id)?;
						if reverse {
							currency_pair = currency_pair.swap();
						}
						// let currency_pair = T::ConstantProductDex::currency_pair(*pool_id)?;
						dx_t = T::ConstantProductDex::get_exchange_value(
							*pool_id,
							currency_pair.base,
							dy_t,
						)?;
						dy_t = dx_t;
					},
				}
			}
			let route_iter: &mut dyn Iterator<Item = &DexRouteNode<_>> = if !reverse {
				forward_iter = route.iter();
				&mut forward_iter
			} else {
				backward_iter = route.iter().rev();
				&mut backward_iter
			};
			for route_node in route_iter {
				match route_node {
					DexRouteNode::Curve(pool_id) => {
						// let currency_pair = T::StableSwapDex::currency_pair(pool_id)?;
						let mut currency_pair = T::StableSwapDex::currency_pair(*pool_id)?;
						if reverse {
							currency_pair = currency_pair.swap();
						}
						let dy_t = T::StableSwapDex::exchange(
							who,
							*pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)?;
						dx_t = dy_t;
					},
					DexRouteNode::Uniswap(pool_id) => {
						// let currency_pair = T::ConstantProductDex::currency_pair(pool_id)?;
						let mut currency_pair = T::ConstantProductDex::currency_pair(*pool_id)?;
						if reverse {
							currency_pair = currency_pair.swap();
						}
						let dy_t = T::ConstantProductDex::exchange(
							who,
							*pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)?;
						dx_t = dy_t;
					},
				}
			}
			ensure!(dx_t >= min_receive, Error::<T>::CannotRespectMinimumRequested);
			Ok(dx_t)
		}
	}
}
