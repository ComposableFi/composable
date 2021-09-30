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
// TODO: allow until pallet fully implemented
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod price_function;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{auction::DutchAuction, dex::{Orderbook, SimpleExchange}, loans::DurationSeconds, math::LiftedFixedBalance};
	use frame_support::{Parameter, StorageMap, ensure, pallet_prelude::MaybeSerializeDeserialize, traits::{Currency, IsType, UnixTime, fungibles::{Inspect, Mutate, Transfer}, tokens::WithdrawConsequence}};

	use frame_system::{pallet_prelude::*, Account};
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128, Percent, Permill, Perquintill, traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		}};
	use sp_std::{fmt::Debug, vec::Vec};

	pub trait DeFiComposableConfig: frame_system::Config {
		// what.
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ From<u128>
			+ Default;

		type Balance: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ CheckedSub
			+ AtLeast32BitUnsigned
			+ From<u64> // at least 64 bit
			+ Zero
			+ FixedPointOperand
			+ Into<LiftedFixedBalance> // integer part not more than bits in this
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit

		/// bank. vault owned - can transfer, cannot mint
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			// used to check balances before any storage updates allowing acting without rollback
			+ Inspect<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;
	}

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: DeFiComposableConfig {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type UnixTime: UnixTime;
		type Orderbook: Orderbook;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		PositionWasSentToLiquidation {},
	}

	#[pallet::error]
	pub enum Error<T> {
		CannotWithdrawAmountEqualToDesiredAuction,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {}


	#[derive(Default, Debug, Copy, Clone, Encode, Decode, PartialEq)]
	#[repr(transparent)]
	pub struct OrderIndex(u64);


	/// auction can span several dex orders within its lifetime
	#[derive(Encode, Decode, Default)]
	pub struct Order<DexOrderId>
	{
		pub dex_order: DexOrderId,
		pub started: DurationSeconds,
		pub function: composable_traits::auction::AuctionStepFunction,
	}


	#[pallet::storage]
	#[pallet::getter(fn orders)]
	pub type Orders<T: Config> = StorageMap<
		_,
		Twox64Concat,
		OrderIndex,
		Order<T::Orderbook::OrderId>,
		ValueQuery,
	>;

	impl<T: Config + DeFiComposableConfig> DutchAuction for Pallet<T> {
		type AccountId = T::AccountId;

		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type Error = Error<T>;

		type OrderId = u128;

		type Orderbook = T::Orderbook;

		fn start(
			account_id: &Self::AccountId,
			source_asset_id: &Self::AssetId,
			source_account: &Self::AccountId,
			target_asset_id: &Self::AssetId,
			target_account: &Self::AccountId,
			want: &Self::AssetId,
			total_amount: &Self::Balance,
			initial_price: &Self::Balance,
			function: composable_traits::auction::AuctionStepFunction,
		) -> Result<Self::OrderId, Self::Error> {

			ensure!(
				matches!(<T::Currency as Inspect>::can_withdraw(source_asset_id, account_id, total_amount), WithdrawConsequence::Success),
				Error::<T>::CannotWithdrawAmountEqualToDesiredAuction
			);

			let dex_order = <T::Orderbook as Orderbook>::post(account_id, source_asset_id, initial_price, total_amount, initial_price, Permill::from_perthousand(10))?;
			let order_id = OrderIndex(42);
			let order = Order {
				dex_order : dex_order,
				started : T::UnixTime::now(),
				function,
			};
			Orders::<T>::insert(order_id, order);
		}

		fn run_auctions(now: composable_traits::loans::DurationSeconds) -> Result<(), Self::Error> {
			todo!()
		}

		fn get_auction_state(
			order: &Self::OrderId,
		) -> Option<composable_traits::auction::AuctionOrder<Self::OrderId>> {
			todo!()
		}
	}
}
