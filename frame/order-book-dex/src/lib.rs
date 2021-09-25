#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
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
	trivial_numeric_casts,
	unused_extern_crates
)]

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{dex::{Orderbook, SimpleExchange}, rate_model::LiftedFixedBalance};
	use frame_support::{Parameter, pallet_prelude::MaybeSerializeDeserialize, traits::UnixTime};
	use frame_system::{Account, pallet_prelude::*};
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Perquintill,
	};
	use sp_std::{fmt::Debug, vec::Vec};
	pub trait DeFiComposablePallet {
		type AssetId: FullCodec
		+ Eq
		+ PartialEq
		+ Copy
		+ MaybeSerializeDeserialize
		+ From<u128>
		+ Default;
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + DeFiComposablePallet {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ SaturatingSub
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ FixedPointOperand
			+ Into<LiftedFixedBalance> // integer part not more than bits in this
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit
		type UnixTime: UnixTime;
	}

	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	pub struct DexInitialization {
		// fee for executing swap
	}

	/// allows order to be diminished in requested price
	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	pub struct DutchAuctionsConfig {

	}

	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	pub enum OrderPrice<T: Config> {
		ExactPrice(T::Balance),
		/// allows to buy/sell little off requested
		AllowSlipagePrice(T::Balance, Perquintill),
		/// allows to change sell/bid price with time up to some limits
		Dutch(T::Balance, DutchAuctionsConfig),
	}

	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	pub enum OrderStatus {}

	/// Store on chain multi dictionary key (from, to, account) , dictionary per buy and sell
	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	pub struct Order<T: Config> {
		pub amount: T::Balance,
		pub price: OrderPrice<T>,
		pub time_stamp: T::UnixTime,
		pub trader: T::AccountId,
		pub status: OrderStatus,
		/// allow for Multi-specialist book
		/// if i want to trade A for B, and there is A -> C -> B, than I can do it.
		pub multi_book: bool,
		pub from : T::AssetId,
		pub to: T::AssetId,
	}
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		BuyAdded {
			order_time_stamp : T::UnixTime,
			/// at specific unit of time can do limited amount of orders on behalf single trade
			/// so (trader, order_time_stamp, counter per block) is natural  order key
			/// ASK: Hussein, Andrei?
			counter: u8,
			amt : T::Balance,
			price : T::Balance,
			trader : T::AccountId
		},


		SellAdded{
			order_time_stamp : T::UnixTime,
			counter: u8,
			amt : T::Balance,
			price : T::Balance,
			trader : T::AccountId
		},

		TradeAdd{
			order_time_stamp : T::UnixTime,
			counter: u8,
			amt : T::Balance,
			price : T::Balance,
			maker : T::AccountId,
			taker : T::AccountId
		}
	}

	/// certain limited number of traders
	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	#[repr(transparent)]
	pub struct TraderIndex(u32);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// must allow O(1) status changes on total orders in system
	/// may allow O(n) on total orders of from single address, with some order limits per address
	/// assuming that creating address is seldom, dictionary can be stable enough
	/// churn with creating new address of any count may be protected by forcing locking some trader fee

	/// alternative desing having random index or growin index dictionary . Ask, what  is pefromance of it?
	///  or alterantive design would be ring, so that old order are killed within time buy new orders
	/// so if need to hold longer possition, can hold it off chain
	#[pallet::storage]
	#[pallet::getter(fn sell)]
	pub type Sell<T: Config> = StorageMap<
		_,
		Twox64Concat,
		TraderIndex,
		Vec<Order<T>>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn buy)]
	pub type Buy<T: Config> = StorageMap<
		_,
		Twox64Concat,
		TraderIndex,
		Vec<Order<T>>,
		OptionQuery,
	>;

	/// trader index to signing address
	#[pallet::storage]
	#[pallet::getter(fn traders)]
	pub type Traders<T: Config> = StorageMap<
		_,
		Twox64Concat,
		TraderIndex,
		Vec<Order<T>>,
		T::AccountId,
	>;

	/// locked currencies owned by dex
	#[pallet::storage]
	#[pallet::getter(fn traders_accounts)]
	pub type TraderAccounts<T: Config> = StorageMap<
		_,
		Twox64Concat,
		TraderIndex,
		Vec<Order<T>>,
		T::AccountId,
	>;

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		/// validate trader limits
		/// add buy order
		/// send event
		/// transfers
		pub fn buy() {

		}

		/// validate trader limits
		/// add sell order
		/// send event
		/// locks(transfers amount from trader to )
		pub fn sell() {

		}

		/// swap 2 assets
		pub fn trade() {

		}

		/// decomissi
		pub fn decomission() {

		}
	}


	impl<T: Config> Orderbook for Pallet<T> {
		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type AccountId = T::Balance;

		type Error = Error<T>;

		type OrderId = u32;

		fn post(
				account: &Self::AccountId,
				asset: &Self::AssetId,
				want: &Self::AssetId,
				amount: &Self::Balance,
				price: &Self::Balance,
			) -> Result<Self::OrderId, Self::Error> {
				todo!()
			}

		fn market_sell(
				account: &Self::AccountId,
				asset: &Self::AssetId,
				want: &Self::AssetId,
				amount: &Self::Balance,
			) -> Result<Self::OrderId, Self::Error> {
				todo!()
			}

		fn take(
				account: &Self::AccountId,
				orders: impl Iterator<Item=Self::OrderId>,
				up_to: Self::Balance,
			) -> Result<composable_traits::dex::TakeResult<Self::Balance>, Self::Error> {
				todo!()
			}

	}
}
