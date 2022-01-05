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
		defi::{CurrencyPair, LiftedFixedBalance},
		dex::{CurveAmm, DexRoute, DexRouteNode, DexRouter},
	};
	use core::fmt::Debug;
	use frame_support::pallet_prelude::*;
	use sp_runtime::{
		traits::{
			AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, IntegerSquareRoot, One, Zero,
		},
		FixedPointOperand,
	};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type AssetId: FullCodec
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
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ One
			+ IntegerSquareRoot
			+ FixedPointOperand
			+ Into<LiftedFixedBalance>
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
		/// The maximum hops in the route.
		#[pallet::constant]
		type MaxHopsInRoute: Get<u32> + TypeInfo;
		type PoolId: FullCodec
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
		type PoolTokenIndex: Copy + Debug + Eq + Into<u32> + From<u8>;
		type StableSwapDex: CurveAmm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolTokenIndex = Self::PoolTokenIndex,
			PoolId = Self::PoolId,
		>;
		type ConstantProductDex: CurveAmm<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			PoolTokenIndex = Self::PoolTokenIndex,
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

	impl<T: Config> Pallet<T> {}

	impl<T: Config> DexRouter<T::AccountId, T::AssetId, T::PoolId, T::Balance> for Pallet<T> {
		fn update_route(
			who: &T::AccountId,
			asset_pair: CurrencyPair<T::AssetId>,
			route: Option<Vec<DexRouteNode<T::PoolId>>>,
		) -> Result<(), DispatchError> {
			let k1 = asset_pair.base;
			let k2 = asset_pair.quote;
			match route {
				Some(route_vec) => {
					for r in &route_vec {
						match r {
							DexRouteNode::Curve(pool_id) => {
								if !T::StableSwapDex::pool_exists(*pool_id) {
									return Err(Error::<T>::PoolDoesNotExist.into())
								}
							},
							DexRouteNode::Uniswap(pool_id) => {
								if !T::ConstantProductDex::pool_exists(*pool_id) {
									return Err(Error::<T>::PoolDoesNotExist.into())
								}
							},
						}
					}
					let bounded_route =
						route_vec.clone().try_into().map_err(|_| Error::<T>::MaxHopsExceeded)?;
					let existing_route = DexRoutes::<T>::get(k1, k2);
					DexRoutes::<T>::insert(k1, k2, DexRoute::Direct(bounded_route));
					match existing_route {
						Some(existing_route) => {
							let old_route = match existing_route {
								DexRoute::Direct(bounded_vec) => bounded_vec.into_inner(),
							};
							Self::deposit_event(Event::RouteUpdated {
								who: who.clone(),
								x_asset_id: k1,
								y_asset_id: k2,
								old_route,
								updated_route: route_vec,
							});
						},
						None => {
							Self::deposit_event(Event::RouteAdded {
								who: who.clone(),
								x_asset_id: k1,
								y_asset_id: k2,
								route: route_vec,
							});
						},
					}
				},
				None =>
					if let Some(deleted_route) = DexRoutes::<T>::take(k1, k2) {
						let deleted_route = match deleted_route {
							DexRoute::Direct(bounded_vec) => bounded_vec.into_inner(),
						};
						Self::deposit_event(Event::RouteDeleted {
							who: who.clone(),
							x_asset_id: k1,
							y_asset_id: k2,
							route: deleted_route,
						});
					},
			}
			Ok(())
		}
		fn get_route(asset_pair: CurrencyPair<T::AssetId>) -> Option<Vec<DexRouteNode<T::PoolId>>> {
			let route = DexRoutes::<T>::get(asset_pair.base, asset_pair.quote);
			if let Some(route) = route {
				match route {
					DexRoute::Direct(bounded_vec) => return Some(bounded_vec.into_inner()),
				}
			}
			None
		}
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
						dy_t = T::StableSwapDex::exchange(
							who,
							*pool_id,
							0_u8.into(),
							1_u8.into(),
							dx_t,
							T::Balance::zero(),
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						dx_t = dy_t;
					},
					DexRouteNode::Uniswap(pool_id) => {
						dy_t = T::ConstantProductDex::exchange(
							who,
							*pool_id,
							0_u8.into(),
							1_u8.into(),
							dx_t,
							T::Balance::zero(),
						)
						.map_err(|_| Error::<T>::ExchangeError)?;
						dx_t = dy_t;
					},
				}
			}
			Ok(dy_t)
		}
	}
}
