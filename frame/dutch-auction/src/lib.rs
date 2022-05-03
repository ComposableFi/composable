//! Dutch Auction
//!
//! Ask to sell on auction.
//! Initial price can start from price above market.
//! Diminishes with time.
//! Takers can take for price same or higher.
//! Higher takers take first.
//! Sell(ask) orders stored on chain. Sell takes deposit from seller, returned during take or
//! liquidation. Takes live only one block.
//!
//! # Take Sell Order
//! Allows for best price to win during auction take. as takes are not executed immediately.
//! When auction steps onto new value, several people will decide it worth it.
//! They will know that highest price wins, so will try to overbid other, hopefully driving price to
//! more optimal. So takers appropriate tip to auction, not via transaction tip(not proportional to
//! price) to parachain. Allows to win bids not by closes to parachain host machine.
//!
//! # Sell Order deposit
//! Sell takes deposit (as for accounts), to store sells for some time.
//! We have to store lock deposit value with ask as it can change within time.
//! Later deposit is used by pallet as initiative to liquidate garbage.
//!
//! # Price prediction
//! Dutch action starts with configured price and than and other price value is f(t).
//! So any external observer can predict what price will be on specified block.
//!
//! # DEX
//! Currently this dutch auction does not tries to sell on external DEX.
//!
//! # XCMP
//!
//! Auction provides cross chain API. Alternative

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
	unused_extern_crates
)]
pub use pallet::*;

pub mod math;
#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
mod mock;

mod prelude;
mod support;
mod types;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use crate::{prelude::*, types::*};
	use xcm::latest::{prelude::*, MultiAsset, WeightLimit::Unlimited};

	use crate::{math::*, support::DefiMultiReservableCurrency};
	use composable_support::{
		abstractions::{
			nonce::Nonce,
			utils::{
				increment::{Increment, WrappingIncrement},
				start_at::ZeroInit,
			},
		},
		math::wrapping_next::WrappingNext,
	};
	use composable_traits::{
		defi::{DeFiComposableConfig, DeFiEngine, OrderIdLike, Sell, SellEngine, Take},
		time::TimeReleaseFunction,
		xcm::{ConfigurationId, CumulusMethodId, XcmSellInitialResponseTransact, XcmSellRequest},
	};
	use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		traits::{tokens::fungible::Transfer as NativeTransfer, EnsureOrigin, IsType, UnixTime},
		transactional, PalletId, Twox64Concat,
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use orml_traits::{MultiCurrency, MultiReservableCurrency};
	use sp_runtime::{traits::AccountIdConversion, DispatchError};
	use sp_std::convert::TryInto;

	pub type OrderIdOf<T> = <T as Config>::OrderId;
	pub type SellOf<T> = SellOrder<
		<T as DeFiComposableConfig>::MayBeAssetId,
		<T as DeFiComposableConfig>::Balance,
		<T as frame_system::Config>::AccountId,
		EDContext<<T as DeFiComposableConfig>::Balance>,
		TimeReleaseFunction,
	>;

	pub type TakeOf<T> =
		TakeOrder<<T as DeFiComposableConfig>::Balance, <T as frame_system::Config>::AccountId>;

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type UnixTime: UnixTime;
		type OrderId: OrderIdLike + WrappingNext + Zero + One;
		type MultiCurrency: MultiCurrency<
				Self::AccountId,
				CurrencyId = Self::MayBeAssetId,
				Balance = <Self as DeFiComposableConfig>::Balance,
			> + MultiReservableCurrency<
				Self::AccountId,
				CurrencyId = Self::MayBeAssetId,
				Balance = <Self as DeFiComposableConfig>::Balance,
			>;
		type WeightInfo: WeightInfo;
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		type NativeCurrency: NativeTransfer<Self::AccountId, Balance = Self::Balance>;

		/// ED taken to create position. Part of if returned when position is liqudated.
		#[pallet::constant]
		type PositionExistentialDeposit: Get<Self::Balance>;

		type XcmOrigin: From<<Self as frame_system::Config>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::XcmOrigin>>;
		/// origin of admin of this pallet
		type AdminOrigin: EnsureOrigin<<Self as frame_system::Config>::Origin>;

		type XcmSender: SendXcm;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		OrderAdded {
			order_id: OrderIdOf<T>,
			order: SellOf<T>,
		},
		/// raised when part or whole order was taken with mentioned balance
		OrderTaken {
			order_id: OrderIdOf<T>,
			taken: T::Balance,
		},
		OrderRemoved {
			order_id: OrderIdOf<T>,
		},
		CofigurationAdded {
			configuration_id: ConfigurationId,
			configuration: TimeReleaseFunction,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		RequestedOrderDoesNotExists,
		OrderParametersIsInvalid,
		TakeParametersIsInvalid,
		TakeLimitDoesNotSatisfyOrder,
		OrderNotFound,
		TakeOrderDidNotHappen,
		NotEnoughNativeCurrencyToPayForAuction,
		/// errors trying to decode and parse XCM input
		XcmCannotDecodeRemoteParametersToLocalRepresentations,
		XcmCannotFindLocalIdentifiersAsDecodedFromRemote,
		XcmNotFoundConfigurationById,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn orders_index)]
	#[allow(clippy::disallowed_types)] // nonce
	pub type OrdersIndex<T: Config> =
		StorageValue<_, T::OrderId, ValueQuery, Nonce<ZeroInit, WrappingIncrement>>;

	#[pallet::storage]
	#[pallet::getter(fn buys)]
	pub type SellOrders<T: Config> =
		StorageMap<_, Twox64Concat, OrderIdOf<T>, SellOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn xcm_sell_orders)]
	pub type XcmSellOrders<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		polkadot_parachain::primitives::Id,
		Twox64Concat,
		composable_traits::xcm::OrderId,
		T::OrderId,
		OptionQuery,
	>;

	/// orders are handled locally, but if these came from remote,
	/// these should be notified approtiately
	#[pallet::storage]
	#[pallet::getter(fn get_local_order_id_to_remote)]
	pub type LocalOrderIdToRemote<T: Config> = StorageMap<
		_,
		Twox64Concat,
		OrderIdOf<T>,
		(polkadot_parachain::primitives::Id, composable_traits::xcm::OrderId),
		OptionQuery,
	>;

	/// registered callback location for specific parachain
	#[pallet::storage]
	#[pallet::getter(fn get_callback_locations)]
	pub type ParachainXcmCallbackLocation<T: Config> = StorageMap<
		_,
		Twox64Concat,
		polkadot_parachain::primitives::Id,
		CumulusMethodId,
		OptionQuery,
	>;

	/// set of reusable auction configurations
	#[pallet::storage]
	#[pallet::getter(fn configuraitons)]
	pub type Configurations<T: Config> =
		StorageMap<_, Twox64Concat, ConfigurationId, TimeReleaseFunction, OptionQuery>;

	/// one block storage, users payed N * WEIGHT for this Vec, so will not put bound here (neither
	/// HydraDX does)
	#[pallet::storage]
	#[pallet::getter(fn takes)]
	pub type Takes<T: Config> =
		StorageMap<_, Twox64Concat, OrderIdOf<T>, Vec<TakeOf<T>>, OptionQuery>;

	impl<T: Config + DeFiComposableConfig> DeFiEngine for Pallet<T> {
		type MayBeAssetId = T::MayBeAssetId;
		type Balance = T::Balance;
		type AccountId = T::AccountId;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO: benchmarking
		/// Inserts or replaces auction configuration.
		/// Already running auctions are not updated.
		#[pallet::weight(10_000)]
		pub fn add_configuration(
			origin: OriginFor<T>,
			configuration_id: ConfigurationId,
			configuration: TimeReleaseFunction,
		) -> DispatchResultWithPostInfo {
			let _ = T::AdminOrigin::ensure_origin(origin)?;
			Configurations::<T>::insert(configuration_id, configuration.clone());
			Self::deposit_event(Event::CofigurationAdded { configuration_id, configuration });
			Ok(().into())
		}

		/// sell `order` in auction with `configuration`
		/// some deposit is taken for storing sell order
		#[pallet::weight(T::WeightInfo::ask())]
		pub fn ask(
			origin: OriginFor<T>,
			order: Sell<T::MayBeAssetId, T::Balance>,
			configuration: TimeReleaseFunction,
		) -> DispatchResultWithPostInfo {
			let who = &(ensure_signed(origin)?);
			let order_id =
				<Self as SellEngine<TimeReleaseFunction>>::ask(who, order, configuration)?;

			Self::deposit_event(Event::OrderAdded {
				order_id,
				order: SellOrders::<T>::get(order_id).expect("just added order exists"),
			});
			Ok(().into())
		}

		/// adds take to list, does not execute take immediately
		#[pallet::weight(T::WeightInfo::take())]
		pub fn take(
			origin: OriginFor<T>,
			order_id: T::OrderId,
			take: Take<T::Balance>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			<Self as SellEngine<TimeReleaseFunction>>::take(&who, order_id, take)?;
			Ok(().into())
		}

		/// allows to remove `order_id` from storage
		#[pallet::weight(T::WeightInfo::liquidate())]
		pub fn liquidate(origin: OriginFor<T>, order_id: T::OrderId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let order = SellOrders::<T>::get(order_id).ok_or(Error::<T>::OrderNotFound)?;
			ensure!(order.from_to == who, DispatchError::BadOrigin,);
			// weights fees are of platform spam protection, so we do not interfere with
			// this function but on pallet level, we allow "fee less" liquidation by owner
			// we can later allow liquidate old orders(or orders with some block liquidation
			// timeout set) using kind of account per order is possible, but may risk to
			// pollute account system
			let treasury = &T::PalletId::get().into_account();
			T::MultiCurrency::unreserve(order.order.pair.base, &who, order.order.take.amount);
			<T::NativeCurrency as NativeTransfer<T::AccountId>>::transfer(
				treasury,
				&order.from_to,
				order.context.deposit,
				false,
			)?;

			<SellOrders<T>>::remove(order_id);
			Self::deposit_event(Event::OrderRemoved { order_id });

			Ok(Pays::No.into())
		}

		// TODO: benchmark
		// TODO: make API for call this as liquidation engine
		// TODO: so make pallet trait for having this call
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn xcm_sell(
			origin: OriginFor<T>,
			request: XcmSellRequest,
		) -> DispatchResultWithPostInfo {
			// TODO: make events/logs from all failed liqudations

			// incoming message is generic in representations, so need to map it back to local,
			let parachain_id = ensure_sibling_para(<T as Config>::XcmOrigin::from(origin))?;
			let base = T::MayBeAssetId::decode(&mut &request.order.pair.base.encode()[..])
				.map_err(|_| Error::<T>::XcmCannotDecodeRemoteParametersToLocalRepresentations)?;
			let quote = T::MayBeAssetId::decode(&mut &request.order.pair.quote.encode()[..])
				.map_err(|_| Error::<T>::XcmCannotDecodeRemoteParametersToLocalRepresentations)?;
			let amount: T::Balance =
				request.order.take.amount.try_into().map_err(|_| {
					Error::<T>::XcmCannotDecodeRemoteParametersToLocalRepresentations
				})?;
			let order = Sell::new(base, quote, amount, request.order.take.limit);
			let configuration = Configurations::<T>::get(request.configuration)
				.ok_or(Error::<T>::XcmNotFoundConfigurationById)?;
			let who = T::AccountId::decode(&mut &request.from_to[..])
				.map_err(|_| Error::<T>::XcmCannotDecodeRemoteParametersToLocalRepresentations)?;

			let order_id =
				<Self as SellEngine<TimeReleaseFunction>>::ask(&who, order, configuration)?;
			LocalOrderIdToRemote::<T>::insert(order_id, (parachain_id, request.order_id));

			Self::deposit_event(Event::OrderAdded {
				order_id,
				order: SellOrders::<T>::get(order_id).expect("just added order exists"),
			});

			Ok(().into())
		}
	}

	impl<T: Config + DeFiComposableConfig> SellEngine<TimeReleaseFunction> for Pallet<T> {
		type OrderId = T::OrderId;
		fn ask(
			from_to: &Self::AccountId,
			order: Sell<Self::MayBeAssetId, Self::Balance>,
			configuration: TimeReleaseFunction,
		) -> Result<Self::OrderId, DispatchError> {
			ensure!(order.is_valid(), Error::<T>::OrderParametersIsInvalid,);
			let order_id = <OrdersIndex<T>>::increment();
			let treasury = &T::PalletId::get().into_account();
			let deposit = T::PositionExistentialDeposit::get();
			<T::NativeCurrency as NativeTransfer<T::AccountId>>::transfer(
				from_to, treasury, deposit, true,
			)?;

			let now = T::UnixTime::now().as_secs();
			let order = SellOf::<T> {
				from_to: from_to.clone(),
				configuration,
				order,
				context: EDContext::<Self::Balance> { added_at: now, deposit },
				total_amount_received: Self::Balance::zero(),
			};

			T::MultiCurrency::reserve(order.order.pair.base, from_to, order.order.take.amount)?;
			SellOrders::<T>::insert(order_id, order);

			Ok(order_id)
		}

		fn take(
			from_to: &Self::AccountId,
			order_id: Self::OrderId,
			take: Take<Self::Balance>,
		) -> Result<(), DispatchError> {
			ensure!(take.is_valid(), Error::<T>::TakeParametersIsInvalid,);
			let order = <SellOrders<T>>::try_get(order_id)
				.map_err(|_x| Error::<T>::RequestedOrderDoesNotExists)?;
			ensure!(order.order.take.limit <= take.limit, Error::<T>::TakeLimitDoesNotSatisfyOrder,);
			let limit = order.order.take.limit;
			// may consider storing calculation results within single block, so that finalize does
			// not recalculates
			let passed = T::UnixTime::now().as_secs() - order.context.added_at;
			let _limit = order.configuration.price(limit, passed)?;
			let quote_amount = take.quote_limit_amount()?;

			T::MultiCurrency::reserve(order.order.pair.quote, from_to, quote_amount)?;
			<Takes<T>>::append(order_id, TakeOf::<T> { from_to: from_to.clone(), take });

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// this cleanups all takes added into block, so we never store takes
		// so we stay fast and prevent attack
		fn on_finalize(_n: T::BlockNumber) {
			for (order_id, takes) in <Takes<T>>::drain() {
				if let Err(err) = Self::take_order(order_id, takes) {
					log::error!("failed to take order {:?} with {:?}", order_id, err);
				}
			}
		}

		fn on_initialize(_n: T::BlockNumber) -> Weight {
			T::WeightInfo::known_overhead_for_on_finalize()
		}
	}

	impl<T: Config> Pallet<T> {
		#[transactional]
		pub fn take_order(
			order_id: <T as Config>::OrderId,
			mut takes: Vec<TakeOf<T>>,
		) -> Result<(), DispatchError> {
			<SellOrders<T>>::try_mutate_exists(order_id, |order_item| {
				if let Some(crate::types::SellOrder {
					order,
					context: _,
					from_to: ref seller,
					configuration: _,
					total_amount_received,
				}) = order_item
				{
					let mut amount_received = T::Balance::zero();
					// users payed N * WEIGHT before, we here pay N * (log N - 1) * Weight. We can
					// retain pure N by first served principle so, not highest price.
					takes.sort_by(|a, b| b.take.limit.cmp(&a.take.limit));
					// calculate real price
					for take in takes {
						let quote_amount = take.take.quote_limit_amount()?;
						// TODO: what to do with orders which nobody ever takes? some kind of dust
						// orders
						if order.take.amount == T::Balance::zero() {
							// bidder was unlucky because order was sol out
							T::MultiCurrency::unreserve(
								order.pair.quote,
								&take.from_to,
								quote_amount,
							);
						} else {
							let take_amount = take.take.amount.min(order.take.amount);
							order.take.amount -= take_amount;
							let real_quote_amount = take.take.quote_amount(take_amount)?;

							T::MultiCurrency::exchange_reserved(
								order.pair.base,
								seller,
								take_amount,
								order.pair.quote,
								&take.from_to,
								real_quote_amount,
							)?;
							if real_quote_amount < quote_amount {
								T::MultiCurrency::unreserve(
									order.pair.quote,
									&take.from_to,
									quote_amount - real_quote_amount,
								);
							}
							amount_received += real_quote_amount;
						}
					}

					*total_amount_received += amount_received;

					if order.take.amount == T::Balance::zero() {
						Self::callback_xcm(order, seller, order_id, *total_amount_received)?;
						*order_item = None;
						Self::deposit_event(Event::OrderRemoved { order_id });
					}

					if amount_received > T::Balance::zero() {
						return Ok(())
					}
				}
				Err(Error::<T>::TakeOrderDidNotHappen.into())
			})
		}

		pub fn callback_xcm(
			order: &Sell<
				<T as DeFiComposableConfig>::MayBeAssetId,
				<T as DeFiComposableConfig>::Balance,
			>,
			seller: &<T as frame_system::Config>::AccountId,
			order_id: <T as Config>::OrderId,
			received_amount: <T as DeFiComposableConfig>::Balance,
		) -> Result<(), DispatchError> {
			LocalOrderIdToRemote::<T>::try_mutate_exists(order_id, |xcm_order_item| {
				if let Some((parachain_id, xcm_order_id)) = xcm_order_item {
					let parachain_id = *parachain_id;
					let xcm_order_id = *xcm_order_id;
					let mut account = vec![0_u8; 32];
					seller.encode_to(&mut account);
					let account: [u8; 32] =
						account.try_into().expect("cumulus runtime has no account with 33 bytes");
					// as of now we do only final sell
					// setting up XCM message
					let asset_id = MultiLocation {
						parents: 1,
						interior: X2(
							AccountId32 { network: Any, id: account },
							GeneralKey(order.pair.encode()),
						),
					};
					let asset_id = AssetId::Concrete(asset_id);
					let assets = MultiAsset { fun: Fungible(received_amount.into()), id: asset_id };
					let callback = composable_traits::xcm::SellResponse::Final(
						XcmSellInitialResponseTransact {
							total_amount_taken: received_amount.into(),
							minimal_price: composable_traits::xcm::Balance::one(), /* auction goes to
							                                                        * mimimal price, can
							                                                        * thin about
							                                                        * better
							                                                        * later */
							order_id: xcm_order_id,
						},
					);
					if let Some(method) = ParachainXcmCallbackLocation::<T>::get(parachain_id) {
						let callback = composable_traits::xcm::XcmCumulusDispatch::new(
							method.pallet_instance,
							method.method_id,
							callback,
						);
						let callback = vec![
							WithdrawAsset(assets.clone().into()),
							BuyExecution { fees: assets.clone(), weight_limit: Unlimited },
							TransferReserveAsset {
								assets: assets.into(),
								dest: (
									Parent,
									X3(
										Parachain(parachain_id.into()),
										AccountId32 { network: Any, id: account },
										GeneralKey(order.pair.encode()),
									),
								)
									.into(),
								xcm: Xcm(vec![Transact {
									origin_type: OriginKind::Native,
									require_weight_at_most: 0, /* TODO: make sure that
									                            * callbacks are free (if
									                            * correct) or specify
									                            * price */
									call: callback.encode().into(),
								}]),
							},
						];
						let msg = Xcm(callback);
						let result = T::XcmSender::send_xcm(
							(Parent, Junction::Parachain(parachain_id.into())),
							msg,
						);
						match result {
							Ok(_) => {
								// TODO: decide if need to send event about sent XCM
								*xcm_order_item = None;
							},
							Err(_) => {
								// TODO: insert here event to allow to act on failure
								return Err(Error::<T>::TakeOrderDidNotHappen.into())
							},
						}
					}
				}
				Ok(())
			})
		}
	}
}
