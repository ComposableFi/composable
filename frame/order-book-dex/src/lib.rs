//! on chain state to handle state of cross chain exchanges
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
	trivial_numeric_casts
)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod mocks;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Codec, Decode, Encode, FullCodec};
	use composable_traits::{
		auction::{AuctionState, AuctionStepFunction, DutchAuction},
		dex::{Orderbook, Price, SimpleExchange},
		loans::{DeFiComposableConfig, DurationSeconds, PriceStructure, Timestamp, ONE_HOUR},
		math::{LiftedFixedBalance, SafeArithmetic, WrappingNext},
		privilege::InspectPrivilegeGroup,
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

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: DeFiComposableConfig {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type UnixTime: UnixTime;
		type Orderbook: Orderbook<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			OrderId = Self::DexOrderId,
			GroupId = Self::GroupId,
		>;
		type DexOrderId: FullCodec + Default;
		type OrderId: FullCodec + Clone + Debug + Eq + Default + WrappingNext;
		type GroupId: FullCodec + Clone + Debug + PartialEq + Default;
		type Privilege: InspectPrivilegeGroup<AccountId = Self::AccountId, GroupId = Self::GroupId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (crate) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	/// auction can span several dex orders within its lifetime
	#[derive(Encode, Decode, Default)]
	pub struct Order<OrderId> {
		pub id: OrderId,
	}

	#[pallet::storage]
	#[pallet::getter(fn orders)]
	pub type Orders<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::OrderId,
		Order<<<T as Config>::Orderbook as Orderbook>::OrderId>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn orders_index)]
	pub type OrdersIndex<T: Config> = StorageValue<_, T::OrderId, ValueQuery>;

	impl<T: Config + DeFiComposableConfig> Orderbook for Pallet<T> {
		type AccountId = T::AccountId;

		type AssetId = T::AssetId;

		type Balance = T::Balance;

		type OrderId = T::OrderId;

		type GroupId = T::GroupId;

		fn patch(
			order_id: Self::OrderId,
			price: Price<Self::GroupId, Self::Balance>,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn market_sell(
			account: &Self::AccountId,
			asset: Self::AssetId,
			want: Self::AssetId,
			amount: Self::Balance,
			amm_slippage: Permill,
		) -> Result<Self::OrderId, DispatchError> {
			todo!()
		}

		fn ask(
			account: &Self::AccountId,
			orders: impl Iterator<Item = Self::OrderId>,
			up_to: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn post(
			account_from: &Self::AccountId,
			asset: Self::AssetId,
			want: Self::AssetId,
			source_amount: Self::Balance,
			source_price: Price<Self::GroupId, Self::Balance>,
			amm_slippage: Permill,
		) -> Result<composable_traits::dex::SellOrder<Self::OrderId, Self::AccountId>, DispatchError>
		{
			todo!()
		}
	}
}
