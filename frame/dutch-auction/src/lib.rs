//! Dutch Auction
//! Run thorough all asks, and reduces these in price as time goes. Initial price can start from
//! price above market.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod math;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Decode, Encode};
	use composable_traits::{
		auction::{AuctionStepFunction, DutchAuction},
		defi::{DeFiComposableConfig, DeFiEngine, OrderIdLike, Sell, SellEngine, Take},
		loans::DurationSeconds,
		math::{SafeArithmetic, WrappingNext},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{fungibles::Transfer, IsType, UnixTime},
	};
	use frame_system::pallet_prelude::BlockNumberFor;
use num_traits::Zero;
	use scale_info::TypeInfo;

	use crate::math::*;
	use orml_traits::{MultiCurrency, MultiCurrencyExtended, MultiReservableCurrency};
	use sp_runtime::DispatchError;
	use sp_std::vec::Vec;

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type UnixTime: UnixTime;
		type OrderId: OrderIdLike + WrappingNext + Zero;
		type Order;
		type MultiCurrency: MultiCurrencyExtended<
				Self::AccountId,
				CurrencyId = Self::AssetId,
				Amount = <Self as DeFiComposableConfig>::Balance,
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

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage]
	#[pallet::getter(fn orders_index)]
	pub type OrdersIndex<T: Config> = StorageValue<_, T::OrderId, ValueQuery, OrderIdOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn buys)]
	pub type SellOrders<T: Config> =
		StorageMap<_, Twox64Concat, OrderIdOf<T>, SellOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn takes)]
	pub type Takes<T: Config> =
		// Vec is  here, unbounded, adding payed via weigh, cleared without being stored into block 
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
			Self::deposit_event(Event::OrderAdded { order_id: <_>::default() });
			Ok(order_id)
		}

		fn take(
			from_to: &Self::AccountId,
			order_id: Self::OrderId,
			take: Take<Self::Balance>,
		) -> Result<(), DispatchError> {
			ensure!(take.is_valid(), Error::<T>::TakeParametersIsInvalid,);
			let order = <SellOrders<T>>::try_get(order_id)
				.map_err(|x| Error::<T>::RequestedOrderDoesNotExists)?;
			ensure!(
				order.order.take.limit <= take.limit,
				Error::<T>::TakeLimitDoesNotSatisfiesOrder,
			);
			let limit = order.order.take.limit.into();
			let passed = T::UnixTime::now().as_secs() - order.added_at;
			let limit = order.configuration.price(limit, passed)?;
			let quote = take.amount.safe_mul(&take.limit)?;

			T::MultiCurrency::reserve(order.order.pair.quote, from_to, quote)?;
			<Takes<T>>::append(order_id, TakeOf::<T> { from_to: from_to.clone(), take });

			Ok(())
		}
	}

	impl AuctionTimeCurveModel for AuctionStepFunction {
		fn price(
			&self,
			initial_price: composable_traits::math::LiftedFixedBalance,
			duration_since_start: composable_traits::loans::DurationSeconds,
		) -> Result<composable_traits::math::LiftedFixedBalance, sp_runtime::ArithmeticError> {
			todo!()
		}
	}

	impl<T: Config + DeFiComposableConfig> DutchAuction for Pallet<T> {
		type Order = T::Order;

		fn get_order(_order: &Self::OrderId) -> Option<Self::Order> {
			todo!()
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: T::BlockNumber) {
			for (order_id, mut takes) in <Takes<T>>::iter() {
				takes.sort_by(|a, b| b.take.limit.cmp(&a.take.limit));
				let mut order = <SellOrders<T>>::get(order_id).expect("takes are added only onto existing orders");
				// calculate real price
				for take in takes {
					// 1. cap take part which can eat order
					// 2. take from order
					// 4. unlock amount from seller
					// 5. unlock amount for taker
					// 6. transfer amount base to taker
					// 6. transfer amount quate to seller
					// if order is zero , remove order
				}																	
			}
			<Takes<T>>::remove_all(None);
		}
		fn on_initialize(_n: T::BlockNumber) -> Weight {	
			todo!("T::WeightInfo::known_overhead_for_on_finalize()");
		}
	}
}
