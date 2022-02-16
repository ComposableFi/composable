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

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, FullCodec};
	use composable_traits::{
		defi::CurrencyPair,
		dex::{CurveAmm, DexRoute, DexRouteNode, DexRouter},
		math::SafeArithmetic,
	};
	use core::fmt::Debug;
	use frame_support::pallet_prelude::*;
	use sp_runtime::traits::{CheckedAdd, One, Zero};
	use std::collections::VecDeque;

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
		type StableSwapDex: CurveAmm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolId = Self::PoolId,
		>;
		type ConstantProductDex: CurveAmm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolId = Self::PoolId,
		>;
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
		/// Some error occured while performing exchange.
		ExchangeError,
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
	impl<T: Config> Pallet<T> {}

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

		// TODO: expected minimum value can be provided from input parameter.
		fn exchange(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			dx: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let route = Self::get_route(asset_pair).ok_or(Error::<T>::NoRouteFound)?;
			let mut dx_t = dx;
			let mut dy_t = T::Balance::zero();
			for route_node in &route {
				match route_node {
					DexRouteNode::Curve(pool_id) => {
						let currency_pair = T::StableSwapDex::currency_pair(*pool_id)?;
						dy_t = T::StableSwapDex::exchange(
							who,
							*pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						dx_t = dy_t;
					},
					DexRouteNode::Uniswap(pool_id) => {
						let currency_pair = T::ConstantProductDex::currency_pair(*pool_id)?;
						dy_t = T::ConstantProductDex::exchange(
							who,
							*pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						dx_t = dy_t;
					},
				}
			}
			Ok(dy_t)
		}

		fn sell(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			Self::exchange(who, asset_pair, amount)
		}

		fn buy(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			amount: T::Balance,
		) -> Result<T::Balance, DispatchError> {
			let route = Self::get_route(asset_pair).ok_or(Error::<T>::NoRouteFound)?;
			let mut stack: VecDeque<T::Balance> = VecDeque::new();
			stack.reserve(route.len());
			let mut dy_t = amount;
			let mut dx_t;
			for route_node in route.iter().rev() {
				match route_node {
					DexRouteNode::Curve(pool_id) => {
						let currency_pair = T::StableSwapDex::currency_pair(*pool_id)?;
						dx_t = T::StableSwapDex::get_exchange_value(
							*pool_id,
							currency_pair.quote,
							dy_t,
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						stack.push_front(dx_t);
						dy_t = dx_t;
					},
					DexRouteNode::Uniswap(pool_id) => {
						let currency_pair = T::ConstantProductDex::currency_pair(*pool_id)?;
						dx_t = T::ConstantProductDex::get_exchange_value(
							*pool_id,
							currency_pair.quote,
							dy_t,
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						stack.push_front(dx_t);
						dy_t = dx_t;
					},
				}
			}
			sp_std::if_std! {
				println!("stack {:?}", stack);
			}
			for route_node in route {
				match route_node {
					DexRouteNode::Curve(pool_id) => {
						let currency_pair = T::StableSwapDex::currency_pair(pool_id)?;
						let dx_t = stack
							.pop_front()
							.expect("impossible as stack has same length as route");
						let _res = T::StableSwapDex::exchange(
							who,
							pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						sp_std::if_std! {
							println!("dx_t {:?}, res {:?}", dx_t, _res);
						}
					},
					DexRouteNode::Uniswap(pool_id) => {
						let currency_pair = T::ConstantProductDex::currency_pair(pool_id)?;
						let dx_t = stack
							.pop_front()
							.expect("impossible as stack has same length as route");
						let _res = T::ConstantProductDex::exchange(
							who,
							pool_id,
							currency_pair,
							dx_t,
							T::Balance::zero(),
							true,
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						sp_std::if_std! {
							println!("dx_t {:?}, res {:?}", dx_t, _res);
						}
					},
				}
			}
			// TODO
			Ok(T::Balance::zero())
		}
	}
}
