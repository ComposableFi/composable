//! Dutch Auction
//! Ask to sell on auction.
//! Initial price can start from price above market.
//! Diminishes with time.
//! Takers can take for price same or higher.
//! Higher takers take first.
//! Sell orders stored on chain.
//! Takes live only one block.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod math;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Decode, Encode};
	use composable_traits::{
		auction::AuctionStepFunction,
		defi::{DeFiComposableConfig, DeFiEngine, OrderIdLike, Sell, SellEngine, Take},
		loans::DurationSeconds,
		math::{SafeArithmetic, WrappingNext},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{IsType, UnixTime},
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use num_traits::Zero;
	use scale_info::TypeInfo;

	use crate::math::*;
	use orml_traits::{MultiCurrency, MultiReservableCurrency};
	use sp_runtime::{traits::Saturating, DispatchError};
	use sp_std::vec::Vec;

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type UnixTime: UnixTime;
		type OrderId: OrderIdLike + WrappingNext + Zero;
		type MultiCurrency: MultiCurrency<
				Self::AccountId,
				CurrencyId = Self::AssetId,
				Balance = <Self as DeFiComposableConfig>::Balance,
			> + MultiReservableCurrency<
				Self::AccountId,
				CurrencyId = Self::AssetId,
				Balance = <Self as DeFiComposableConfig>::Balance,
			>;
	}

	#[derive(Encode, Decode, Default, TypeInfo)]
	pub struct SellOrder<AssetId, Balance, AccountId, Moment> {
		pub from_to: AccountId,
		pub order: Sell<AssetId, Balance>,
		pub configuration: AuctionStepFunction,
		pub added_at: Moment,
	}

	#[derive(Encode, Decode, Default, TypeInfo)]
	pub struct TakeOrder<Balance, AccountId> {
		pub from_to: AccountId,
		pub take: Take<Balance>,
	}

	// type aliases
	pub type OrderIdOf<T> = <T as Config>::OrderId;
	pub type SellOf<T> = SellOrder<
		<T as DeFiComposableConfig>::AssetId,
		<T as DeFiComposableConfig>::Balance,
		<T as frame_system::Config>::AccountId,
		DurationSeconds,
	>;

	pub type TakeOf<T> =
		TakeOrder<<T as DeFiComposableConfig>::Balance, <T as frame_system::Config>::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		OrderAdded { order_id: OrderIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		RequestedOrderDoesNotExists,
		TakeParametersIsInvalid,
		TakeLimitDoesNotSatisfiesOrder,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn orders_index)]
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
		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;
	}

	#[pallet::type_value]
	pub fn OrderIdOnEmpty<T: Config>() -> T::OrderId {
		T::OrderId::zero()
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn ask(
			origin: OriginFor<T>,
			order: Sell<T::AssetId, T::Balance>,
			configuration: AuctionStepFunction,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let _order_id =
				<Self as SellEngine<AuctionStepFunction>>::ask(&who, order, configuration)?;
			Self::deposit_event(Event::OrderAdded { order_id: <_>::default() });
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn take(
			origin: OriginFor<T>,
			order_id: T::OrderId,
			take: Take<T::Balance>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			<Self as SellEngine<AuctionStepFunction>>::take(&who, order_id, take)?;
			Ok(().into())
		}
	}

	impl<T: Config + DeFiComposableConfig> SellEngine<AuctionStepFunction> for Pallet<T> {
		type OrderId = T::OrderId;
		fn ask(
			from_to: &Self::AccountId,
			order: Sell<Self::AssetId, Self::Balance>,
			configuration: AuctionStepFunction,
		) -> Result<Self::OrderId, DispatchError> {
			ensure!(order.is_valid(), Error::<T>::TakeParametersIsInvalid,);
			let order_id = <OrdersIndex<T>>::mutate(|x| {
				*x = x.next();
				// in case of wrapping, will need to check existence of order/takes
				*x
			});
			let order = SellOf::<T> {
				from_to: from_to.clone(),
				configuration,
				order,
				added_at: T::UnixTime::now().as_secs(),
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
			let limit = order.order.take.limit.into();
			// may consider storing calculation results within single block, so that finalize does
			// not recalculates
			let passed = T::UnixTime::now().as_secs() - order.added_at;
			let _limit = order.configuration.price(limit, passed)?;
			let quote = take.amount.safe_mul(&take.limit)?;

			T::MultiCurrency::reserve(order.order.pair.quote, from_to, quote)?;
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
				// users payed N * WEIGHT before, we here pay N * (log N - 1) * Weight. We can
				// retain pure N by first served principle so, not highest price.
				takes.sort_by(|a, b| b.take.limit.cmp(&a.take.limit));
				let SellOrder { mut order, added_at: _, from_to: ref seller, configuration: _ } =
					<SellOrders<T>>::get(order_id)
						.expect("takes are added only onto existing orders");

				// calculate real price
				for take in takes {
					let take_amount = take.take.amount.min(order.take.amount);
					order.take.amount -= take_amount;
					let quote_amount = take_amount.saturating_mul(take.take.limit);

					exchange_reserved::<T>(
						order.pair.base,
						seller,
						take_amount,
						order.pair.quote,
						&take.from_to,
						quote_amount,
					)
					.expect("we forced locks beforehand");

					// what to do with orders which nobody ever takes? some kind of dust orders with
					// 1 token
					if order.take.amount == T::Balance::zero() {
						break
					}
				}

				if order.take.amount == T::Balance::zero() {
					<SellOrders<T>>::remove(order_id);
				}
			}
			<Takes<T>>::remove_all(None);
		}
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			todo!("T::WeightInfo::known_overhead_for_on_finalize()");
		}
	}

	/// feesless exchange of reserved currencies
	fn exchange_reserved<T: Config>(
		base: <T as DeFiComposableConfig>::AssetId,
		seller: &<T as frame_system::Config>::AccountId,
		take_amount: <T as DeFiComposableConfig>::Balance,
		quote: <T as DeFiComposableConfig>::AssetId,
		taker: &<T as frame_system::Config>::AccountId,
		quote_amount: <T as DeFiComposableConfig>::Balance,
	) -> Result<(), DispatchError> {
		T::MultiCurrency::unreserve(base, seller, take_amount);
		T::MultiCurrency::unreserve(quote, taker, quote_amount);
		T::MultiCurrency::transfer(base, seller, taker, take_amount)?;
		T::MultiCurrency::transfer(quote, taker, seller, take_amount)?;
		Ok(())
	}
}
