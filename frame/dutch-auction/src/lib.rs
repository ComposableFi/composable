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
		math::WrappingNext,
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{IsType, UnixTime, fungibles::Transfer},
	};
	use scale_info::TypeInfo;

	use sp_runtime::DispatchError;
	use sp_std::vec::Vec;
	use orml_traits::{MultiReservableCurrency, MultiCurrencyExtended, MultiCurrency};

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type UnixTime: UnixTime;
		type OrderId: OrderIdLike + WrappingNext;
		type Order;
		type MultiCurrency: MultiCurrencyExtended<Self::AccountId, CurrencyId = Self::AssetId, Amount = Self::Balance>
		+ MultiReservableCurrency<Self::AccountId>;
	}

	// type aliases
	pub type OrderIdOf<T> = <T as Config>::OrderId;
	pub type SellOf<T> = Sell<
		<T as DeFiComposableConfig>::AssetId,
		<T as DeFiComposableConfig>::Balance,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		OrderAdded { order_id: OrderIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		RequestedOrderDoesNotExists
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[derive(Encode, Decode, Default, TypeInfo)]
	pub struct TakeBy<AccountId, Balance> {
		pub from_to: AccountId,
		pub take: Take<Balance>,
	}

	#[pallet::storage]
	#[pallet::getter(fn orders_index)]
	pub type OrdersIndex<T: Config> = StorageValue<_, T::OrderId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn buys)]
	pub type SellOrders<T: Config> =
		StorageMap<_, Twox64Concat, OrderIdOf<T>, SellOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn takes)]
	pub type Takes<T: Config> =
		StorageMap<_, Twox64Concat, OrderIdOf<T>, Vec<TakeBy<T::AccountId, T::Balance>>, OptionQuery>;		

	impl<T: Config + DeFiComposableConfig> DeFiEngine for Pallet<T> {
		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type AccountId = T::AccountId;		
	}

	impl<T: Config + DeFiComposableConfig> SellEngine<AuctionStepFunction> for Pallet<T> {
		type OrderId = T::OrderId;

		fn ask(
			_from_to: &Self::AccountId,
			_order: Sell<Self::AssetId, Self::Balance>,
			_base_amount: Self::Balance,
			_configuration: AuctionStepFunction,
		) -> Result<Self::OrderId, DispatchError> {
			
			Self::deposit_event(Event::OrderAdded { order_id: <_>::default() });
			todo!()
		}

		fn take(
			from_to: &Self::AccountId,
			order: Self::OrderId,
			take: Take<Self::Balance>,
		) -> Result<(), DispatchError> {
			let order = SellOrders::<T>::try_get(order)
			.map_err(|x| Error::<T>::RequestedOrderDoesNotExists)?;
			 
			let order = order.limit;
			
			T::MultiCurrency::reserve(order.pair.quote, from_to, take.amount)?;

			// ensure!(
			// 	SellOrders::<T>::contains_key(order),
				
			// );
			//ensure!(
				//can_reserve(ta
			//)

			//Takes::<T>::insert()
			Ok(())
		}
	}

	impl<T: Config + DeFiComposableConfig> DutchAuction for Pallet<T> {
		type Order = T::Order;

		fn get_order(_order: &Self::OrderId) -> Option<Self::Order> {
			todo!()
		}
	}
}
