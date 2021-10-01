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

	use codec::{Codec, Decode, Encode, FullCodec};
	use composable_traits::{
		auction::{AuctionState, AuctionStepFunction, DutchAuction},
		dex::{Orderbook, SimpleExchange},
		loans::DurationSeconds,
		math::LiftedFixedBalance,
	};
	use frame_support::{
		ensure,
		pallet_prelude::{MaybeSerializeDeserialize, ValueQuery},
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::WithdrawConsequence,
			Currency, IsType, UnixTime,
		},
		Parameter, Twox64Concat,
	};

	use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*, Account};
	use num_traits::{CheckedDiv, SaturatingAdd, SaturatingSub, WrappingAdd};

	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Saturating, Zero,
		},
		ArithmeticError, DispatchError, FixedPointNumber, FixedPointOperand, FixedU128, Percent,
		Permill, Perquintill,
	};
	use sp_std::{fmt::Debug, vec::Vec};

	use crate::price_function::AuctionTimeCurveModel;

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
		type Orderbook: Orderbook<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			Error = DispatchError,
			OrderId = Self::DexOrderId,
		>;
		type DexOrderId: FullCodec + Default;
		type OrderId: FullCodec + Clone + Debug + Eq + Default + WrappingNext;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// when auctions starts
		AuctionWasStarted {
			order_id: T::OrderId,
		},

		AuctionSuccess {
			order_id: T::OrderId,
		},

		AuctionFatalFail {
			order_id: T::OrderId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		CannotWithdrawAmountEqualToDesiredAuction,
		EitherTooMuchOfAuctions,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	/// is anybody aware of trait like Next which is semantically same as WrappingAdd, and calling
	/// wrapping_add(1) -> increment, but without knowing that it is number?
	/// - it is up to storage to clean self up preventing overwrite (clean up + next is implemented
	///   on top of ranges)
	/// - up configuration to decide cardinality
	/// - alternative - random key
	pub trait WrappingNext {
		fn next(&self) -> Self;
	}

	/// auction can span several dex orders within its lifetime
	#[derive(Encode, Decode, Default)]
	pub struct Order<DexOrderId, AccountId, AssetId, Balance> {
		pub latest_dex_order_intention: Option<DexOrderId>,
		pub started: DurationSeconds,
		pub function: AuctionStepFunction,
		pub account_id: AccountId,
		pub source_asset_id: AssetId,
		pub source_account: AccountId,
		pub target_asset_id: AssetId,
		pub target_account: AccountId,
		pub want: AssetId,
		pub total_amount: Balance,
		pub initial_price: Balance,
		pub state: AuctionState,
	}

	#[pallet::storage]
	#[pallet::getter(fn orders)]
	pub type Orders<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::OrderId,
		Order<
			<<T as Config>::Orderbook as Orderbook>::OrderId,
			T::AccountId,
			T::AssetId,
			T::Balance,
		>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn orders_index)]
	pub type OrdersIndex<T: Config> = StorageValue<_, T::OrderId, ValueQuery>;

	impl<T: Config + DeFiComposableConfig> DutchAuction for Pallet<T> {
		type AccountId = T::AccountId;

		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type Error = DispatchError;

		type OrderId = T::OrderId;

		type Orderbook = T::Orderbook;

		type Order = Order<
			<<T as Config>::Orderbook as Orderbook>::OrderId,
			T::AccountId,
			T::AssetId,
			T::Balance,
		>;

		fn start(
			account_id: &Self::AccountId,
			source_asset_id: &Self::AssetId,
			source_account: &Self::AccountId,
			target_asset_id: &Self::AssetId,
			target_account: &Self::AccountId,
			want: &Self::AssetId,
			total_amount: &Self::Balance,
			initial_price: &Self::Balance,
			function: AuctionStepFunction,
		) -> Result<Self::OrderId, Self::Error> {
			/// TODO: with remote foreign chain DEX it can pass several blocks before we get on DEX.
			/// so somehow need to lock (transfer) currency before foreign transactions settles
			ensure!(
				matches!(
					<T::Currency as Inspect<T::AccountId>>::can_withdraw(
						*source_asset_id,
						account_id,
						*total_amount
					),
					WithdrawConsequence::Success
				),
				Error::<T>::CannotWithdrawAmountEqualToDesiredAuction
			);

			/// because dex call is in "other transaction" and same block can have 2 starts each
			/// passing check, but failing during dex call.
			let order_id: T::OrderId = OrdersIndex::<T>::get();
			OrdersIndex::<T>::set(order_id.next());

			let order = Order {
				latest_dex_order_intention: None,
				started: T::UnixTime::now().as_secs(),
				function,
				account_id: account_id.clone(),
				source_asset_id: *source_asset_id,
				source_account: source_account.clone(),
				target_asset_id: *target_asset_id,
				target_account: target_account.clone(),
				want: *want,
				total_amount: *total_amount,
				initial_price: *initial_price,
				state: AuctionState::AuctionStarted,
			};
			Orders::<T>::insert(order_id.clone(), order);
			Self::deposit_event(Event::<T>::AuctionWasStarted { order_id: order_id.clone() });
			Ok(order_id)
		}

		fn run_auctions(now: DurationSeconds) -> Result<(), Self::Error> {
			for (order_id, mut order) in Orders::<T>::iter() {
				if order.latest_dex_order_intention.is_none() &&
					order.state == AuctionState::AuctionStarted
				{
					// for final protocol may be will need to transfer currency onto auction pallet
					// sub account and send dex order with idempotency tracking id final protocol seems should include multistage lock/unlock https://github.com/paritytech/xcm-format or something
					let price = match order.function {
						AuctionStepFunction::LinearDecrease(parameters) =>
							parameters.price(order.initial_price.into(), now - order.started),
						AuctionStepFunction::StairstepExponentialDecrease(parameters) =>
							parameters.price(order.initial_price.into(), now - order.started),
					}?
					.checked_mul_int(1u64)
					.ok_or(ArithmeticError::Overflow)?
					.into();
					let dex_order_intention = <T::Orderbook as Orderbook>::post(
						&order.account_id,
						&order.source_asset_id,
						&order.target_asset_id,
						&order.total_amount,
						&price,
						Permill::from_perthousand(10),
					)?;

					order.latest_dex_order_intention = Some(dex_order_intention);
				} else if order.latest_dex_order_intention.is_none() &&
					order.state != AuctionState::AuctionEndedSuccessfully &&
					order.state != AuctionState::AuctionFatalFailed
				{
					// handle here system timeout by transferring emitting event and cleaning up
					// dex/crosschain stuff
				} else if let Some(dex_order) = order.latest_dex_order_intention {
					// waiting for off chain callback about order status
				}
			}

			Ok(())
		}

		fn intention_updated(
			order_id: &Self::OrderId,
			action_event: composable_traits::auction::AuctionExchangeCallback,
		) {
			if let Ok(mut order) = Orders::<T>::try_get(order_id) {
				match order.state {
					AuctionState::AuctionStarted => match action_event {
						composable_traits::auction::AuctionExchangeCallback::Success => {
							Orders::<T>::remove(order_id);
							Self::deposit_event(Event::<T>::AuctionSuccess {
								order_id: order_id.clone(),
							});
						},
						composable_traits::auction::AuctionExchangeCallback::RetryFail => {
							order.latest_dex_order_intention = None;
						},
						composable_traits::auction::AuctionExchangeCallback::FatalFail => {
							Orders::<T>::remove(order_id);
							Self::deposit_event(Event::<T>::AuctionFatalFail {
								order_id: order_id.clone(),
							});
						},
					},
					_ => {
						// case of previous fail or success, do not care if somebody calls again
					},
				};
			}
		}

		fn get_auction_state(order: &Self::OrderId) -> Option<Self::Order> {
			todo!()
		}
	}
}
