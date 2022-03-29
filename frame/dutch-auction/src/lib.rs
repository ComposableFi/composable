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

#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_method,
		clippy::disallowed_type,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_type)]
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

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::{Decode, Encode};
	use composable_traits::{
		defi::{DeFiComposableConfig, DeFiEngine, OrderIdLike, Sell, SellEngine, Take},
		math::WrappingNext,
		time::{TimeReleaseFunction, Timestamp},
	};
	#[cfg(feature = "runtime-benchmarks")]
	use frame_support::traits::Currency;
	use frame_support::{
		pallet_prelude::*,
		traits::{tokens::fungible::Transfer as NativeTransfer, IsType, UnixTime},
		PalletId,
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use num_traits::Zero;
	use scale_info::TypeInfo;

	use crate::math::*;
	use orml_traits::{MultiCurrency, MultiReservableCurrency};
	use sp_runtime::{traits::AccountIdConversion, DispatchError};
	use sp_std::vec::Vec;

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type UnixTime: UnixTime;
		type OrderId: OrderIdLike + WrappingNext + Zero;
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
	}

	#[derive(Encode, Decode, MaxEncodedLen, Default, TypeInfo, Clone, Debug, PartialEq)]
	pub struct SellOrder<AssetId, Balance, AccountId, Context> {
		pub from_to: AccountId,
		pub order: Sell<AssetId, Balance>,
		pub configuration: TimeReleaseFunction,
		/// context captured when sell started
		pub context: Context,
	}

	#[derive(Encode, Decode, MaxEncodedLen, Default, TypeInfo, Clone, Debug, PartialEq)]
	pub struct Context<Balance> {
		pub added_at: Timestamp,
		pub deposit: Balance,
	}

	#[derive(Encode, Decode, MaxEncodedLen, Default, TypeInfo)]
	pub struct TakeOrder<Balance, AccountId> {
		pub from_to: AccountId,
		pub take: Take<Balance>,
	}

	// type aliases
	pub type OrderIdOf<T> = <T as Config>::OrderId;
	pub type SellOf<T> = SellOrder<
		<T as DeFiComposableConfig>::MayBeAssetId,
		<T as DeFiComposableConfig>::Balance,
		<T as frame_system::Config>::AccountId,
		Context<<T as DeFiComposableConfig>::Balance>,
	>;

	pub type TakeOf<T> =
		TakeOrder<<T as DeFiComposableConfig>::Balance, <T as frame_system::Config>::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		OrderAdded { order_id: OrderIdOf<T>, order: SellOf<T> },
		OrderRemoved { order_id: OrderIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		RequestedOrderDoesNotExists,
		OrderParametersIsInvalid,
		TakeParametersIsInvalid,
		TakeLimitDoesNotSatisfiesOrder,
		OrderNotFound,
		NotEnoughNativeCurrentyToPayForAuction,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn orders_index)]
	#[allow(clippy::disallowed_type)] // OrderIdOnEmpty provides a default value
	pub type OrdersIndex<T: Config> = StorageValue<_, T::OrderId, ValueQuery, OrderIdOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn buys)]
	pub type SellOrders<T: Config> =
		StorageMap<_, Twox64Concat, OrderIdOf<T>, SellOf<T>, OptionQuery>;

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

	#[pallet::type_value]
	pub fn OrderIdOnEmpty<T: Config>() -> T::OrderId {
		T::OrderId::zero()
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
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
	}

	impl<T: Config + DeFiComposableConfig> SellEngine<TimeReleaseFunction> for Pallet<T> {
		type OrderId = T::OrderId;
		fn ask(
			from_to: &Self::AccountId,
			order: Sell<Self::MayBeAssetId, Self::Balance>,
			configuration: TimeReleaseFunction,
		) -> Result<Self::OrderId, DispatchError> {
			ensure!(order.is_valid(), Error::<T>::OrderParametersIsInvalid,);
			let order_id = <OrdersIndex<T>>::mutate(|x| {
				*x = x.next();
				// in case of wrapping, will need to check existence of order/takes
				*x
			});
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
				context: Context::<Self::Balance> { added_at: now, deposit },
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
			ensure!(
				order.order.take.limit <= take.limit,
				Error::<T>::TakeLimitDoesNotSatisfiesOrder,
			);
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
			for (order_id, mut takes) in <Takes<T>>::iter() {
				if let Some(SellOrder {
					mut order,
					context: _,
					from_to: ref seller,
					configuration: _,
				}) = <SellOrders<T>>::get(order_id)
				{
					// users payed N * WEIGHT before, we here pay N * (log N - 1) * Weight. We can
					// retain pure N by first served principle so, not highest price.
					takes.sort_by(|a, b| b.take.limit.cmp(&a.take.limit));
					// calculate real price
					for take in takes {
						let quote_amount =
							take.take.quote_limit_amount().expect("was checked in take call");
						// what to do with orders which nobody ever takes? some kind of dust orders
						// with
						if order.take.amount == T::Balance::zero() {
							T::MultiCurrency::unreserve(
								order.pair.quote,
								&take.from_to,
								quote_amount,
							);
						} else {
							let take_amount = take.take.amount.min(order.take.amount);
							order.take.amount -= take_amount;
							let real_quote_amount =
								take.take.quote_amount(take_amount).expect("was taken via min");

							exchange_reserved::<T>(
								order.pair.base,
								seller,
								take_amount,
								order.pair.quote,
								&take.from_to,
								real_quote_amount,
							)
							.expect("we forced locks beforehand");

							if real_quote_amount < quote_amount {
								T::MultiCurrency::unreserve(
									order.pair.quote,
									&take.from_to,
									quote_amount - real_quote_amount,
								);
							}
						}
					}

					if order.take.amount == T::Balance::zero() {
						<SellOrders<T>>::remove(order_id);
						Self::deposit_event(Event::OrderRemoved { order_id });
					}
				}
			}
			<Takes<T>>::remove_all(None);
		}

		fn on_initialize(_n: T::BlockNumber) -> Weight {
			T::WeightInfo::known_overhead_for_on_finalize()
		}
	}

	/// feesless exchange of reserved currencies
	fn exchange_reserved<T: Config>(
		base: <T as DeFiComposableConfig>::MayBeAssetId,
		seller: &<T as frame_system::Config>::AccountId,
		take_amount: <T as DeFiComposableConfig>::Balance,
		quote: <T as DeFiComposableConfig>::MayBeAssetId,
		taker: &<T as frame_system::Config>::AccountId,
		quote_amount: <T as DeFiComposableConfig>::Balance,
	) -> Result<(), DispatchError> {
		T::MultiCurrency::unreserve(base, seller, take_amount);
		T::MultiCurrency::unreserve(quote, taker, quote_amount);
		T::MultiCurrency::transfer(base, seller, taker, take_amount)?;
		T::MultiCurrency::transfer(quote, taker, seller, quote_amount)?;
		Ok(())
	}
}
