//!

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

	use codec::{Codec, EncodeLike, FullCodec};
	use composable_traits::{
		lending::{
			Lending, MarketConfig, MarketConfigInput, NormalizedCollateralFactor, Timestamp,
		},
		oracle::Oracle,
		rate_model::*,
		vault::{Deposit, Vault, VaultConfig},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{fungibles::MutateHold, DepositConsequence},
			Backing, UnixTime,
		},
		PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use num_traits::{Bounded, CheckedDiv, SaturatingSub};
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedConversion, CheckedMul,
			CheckedSub, Convert, Hash, One, Zero,
		},
		FixedPointNumber, FixedPointOperand, FixedU128, PerThing, Permill, Perquintill,
	};
	use sp_std::{convert::TryInto, fmt::Debug, ops::Mul};

	#[derive(Default, Copy, Clone, Encode, Decode)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Oracle: Oracle<AssetId = Self::AssetId, Balance = Self::Balance>;
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
		type Vault: Vault<VaultId = Self::VaultId, AssetId = Self::AssetId, Balance = Self::Balance>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default;
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
			+ Zero
			+ From<u128>
			+ Into<u128>;

		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type UnixTime: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
		Underflow,
		/// vault provided does not exist
		VaultNotFound,
		/// Only assets for which we can track price are supported
		AssetWithoutPrice,
		/// The market could not be found
		MarketDoesNotExist,
		CollateralDepositFailed,
		MarketCollateralWasNotDepositedByAccount,
		CollateralFactorIsLessOrEqualOne,
	}

	/// Lending instances counter
	#[pallet::storage]
	#[pallet::getter(fn lending_count)]
	pub type LendingCount<T: Config> = StorageValue<_, MarketIndex, ValueQuery>;

	/// Indexed lending instances
	#[pallet::storage]
	#[pallet::getter(fn markets)]
	pub type Markets<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, MarketConfig<T::VaultId>, ValueQuery>;

	/// original debt values
	#[pallet::storage]
	#[pallet::getter(fn debt_principals)]
	pub type DebtPrincipals<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketIndex,
		Twox64Concat,
		T::AccountId,
		T::Balance,
		ValueQuery,
	>;

	/// at which lending index account did borrowed.
	#[pallet::storage]
	#[pallet::getter(fn debt_index)]
	pub type DebtIndex<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		MarketIndex,
		Twox64Concat,
		T::AccountId,
		Ratio,
		ValueQuery,
	>;

	/// (Market, Account) -> Collateral
	#[pallet::storage]
	#[pallet::getter(fn account_collateral)]
	pub type AccountCollateral<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MarketIndex,
		Blake2_128Concat,
		T::AccountId,
		T::Balance,
		ValueQuery,
	>;

	impl<T: Config> Pallet<T> {
		///Accrue interest to updated borrow index
		/// and then calculate account's borrow balance using the updated borrow index
		pub fn borrow_balance_current(
			market_id: &<Self as Lending>::MarketId,
		) -> Result<T::Balance, DispatchError> {
			<Self as Lending>::accrue_interest(market_id)?;
			todo!()
		}
	}

	/// The timestamp of the previous block or defaults to timestamp at genesis.
	/// TODO: should be updated in on_finalize() hook.
	#[pallet::storage]
	#[pallet::getter(fn last_block_timestamp)]
	pub type LastBlockTimestamp<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	impl<T: Config> Lending for Pallet<T> {
		type VaultId = <T::Vault as Vault>::VaultId;

		type AccountId = T::AccountId;

		type MarketId = MarketIndex;

		type Balance = T::Balance;

		type BlockNumber = T::BlockNumber;

		fn create_or_update(
			borrow_asset_vault: <T::Vault as Vault>::VaultId,
			collateral_asset_vault: <T::Vault as Vault>::VaultId,
			config_input: MarketConfigInput<Self::AccountId>,
		) -> Result<(), DispatchError> {
			ensure!(
				config_input.collateral_factor > 1.into(),
				Error::<T>::CollateralFactorIsLessOrEqualOne
			);
			let collateral_asset = T::Vault::asset_id(&collateral_asset_vault)?;
			let borrow_asset = T::Vault::asset_id(&borrow_asset_vault)?;

			<T::Oracle as Oracle>::get_price(&collateral_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;
			<T::Oracle as Oracle>::get_price(&borrow_asset)
				.map_err(|_| Error::<T>::AssetWithoutPrice)?;

			LendingCount::<T>::try_mutate(|MarketIndex(previous_lending_index)| {
				let lending_index = {
					*previous_lending_index += 1;
					MarketIndex(*previous_lending_index)
				};

				let config = MarketConfig {
					borrow: borrow_asset_vault,
					collateral: collateral_asset_vault,
					reserve_factor: config_input.reserve_factor,
					collateral_factor: config_input.collateral_factor,
					interest_rate: InterestRateModel::default(),
				};

				Markets::<T>::insert(lending_index, config);

				Ok(())
			})
		}

		fn account_id(market_id: &Self::MarketId) -> Self::AccountId {
			PALLET_ID.into_sub_account(market_id)
		}

		fn get_markets_for_borrow(borrow: Self::VaultId) -> Vec<Self::MarketId> {
			// allow to be slow until it becomes write transaction (not the case now and then)
			let mut markets = sp_std::vec![];
			for (index, market) in Markets::<T>::iter() {
				if market.borrow == borrow {
					markets.push(index);
				}
			}

			markets
		}

		fn get_all_markets() -> Vec<(Self::MarketId, MarketConfig<<T::Vault as Vault>::VaultId>)> {
			Markets::<T>::iter().map(|(index, config)| (index, config)).collect()
		}

		fn borrow(
			market_id: &Self::MarketId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn repay_borrow(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn total_cash(pair: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn total_reserves(pair: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn borrow_index(
			market_id: &Self::MarketId,
		) -> Result<sp_runtime::FixedU128, DispatchError> {
			todo!()
		}

		fn update_borrows(
			market_id: &Self::MarketId,
			borrows: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn update_reserves(
			market_id: &Self::MarketId,
			reserves: Self::Balance,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn update_borrow_index(
			market_id: &Self::MarketId,
			borrow_index: sp_runtime::FixedU128,
		) -> Result<(), DispatchError> {
			todo!()
		}

		fn calc_utilization_ratio(
			cash: &Self::Balance,
			borrows: &Self::Balance,
			reserves: &Self::Balance,
		) -> Result<Ratio, DispatchError> {
			// utilization ratio is 0 when there are no borrows
			if borrows.is_zero() {
				return Ok(Ratio::zero())
			}
			// utilizationRatio = totalBorrows / (totalCash + totalBorrows − totalReserves)
			let total = cash
				.checked_add(borrows)
				.and_then(|r| r.checked_sub(reserves))
				.ok_or(Error::<T>::Overflow)?;
			let mut util_ratio = Ratio::saturating_from_rational((*borrows).into(), total.into());
			if util_ratio > Ratio::one() {
				util_ratio = Ratio::one();
			}
			Ok(util_ratio)
		}

		fn accrue_interest(market_id: &Self::MarketId) -> Result<(), DispatchError> {
			let total_borrows = Self::total_borrows(market_id)?;
			let total_cash = Self::total_cash(market_id)?;
			let total_reserves = Self::total_reserves(market_id)?;
			let util = Self::calc_utilization_ratio(&total_cash, &total_borrows, &total_reserves)?;
			let delta_time = T::UnixTime::now()
				.as_secs()
				.checked_sub(Self::last_block_timestamp())
				.ok_or(Error::<T>::Underflow)?;
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_rate = market.interest_rate.get_borrow_rate(util).unwrap();
			let interest_accumulated = composable_traits::rate_model::accrued_interest(
				borrow_rate,
				total_borrows.into(),
				delta_time,
			)
			.unwrap();
			let total_borrows_new = interest_accumulated
				.checked_add(total_borrows.into())
				.ok_or(Error::<T>::Overflow)?;
			let total_reserves_new = market
				.reserve_factor
				.mul_floor(interest_accumulated)
				.checked_add(total_reserves.into())
				.ok_or(Error::<T>::Overflow)?;
			let borrow_index = Self::borrow_index(market_id)?;
			let borrow_index_new = increment_index(borrow_rate, borrow_index, delta_time)
				.and_then(|r| r.checked_add(&borrow_index))
				.ok_or(Error::<T>::Overflow)?;
			Self::update_borrows(market_id, Self::Balance::from(total_borrows_new))?;
			Self::update_reserves(market_id, Self::Balance::from(total_reserves_new))?;
			Self::update_borrow_index(market_id, borrow_index_new)?;
			Ok(())
		}

		fn borrow_balance_current(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn collateral_of_account(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn collateral_required(
			market_id: &Self::MarketId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			let config =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral = AccountCollateral::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketCollateralWasNotDepositedByAccount)?;
			let asset = <T::Vault as Vault>::asset_id(&config.collateral)?;
			let collateral_price = <T::Oracle as Oracle>::get_price(&asset)?;
			let balance: u128 = collateral_price.0.into();

			let limit: u128 = NormalizedCollateralFactor::from(balance)
				.checked_div(&config.collateral_factor)
				.ok_or(Error::<T>::Overflow)?
				.checked_mul_int(1)
				.ok_or(Error::<T>::Overflow)?;
			Ok(limit.try_into().map_err(|_| Error::<T>::Overflow)?)
		}

		fn deposit_collateral(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral_lp_id = T::Vault::lp_asset_id(&market.collateral)?;
			T::Currency::transfer(
				collateral_lp_id,
				from,
				&Self::account_id(market_id),
				amount,
				true,
			)
			.map_err(|_| Error::<T>::CollateralDepositFailed)?;
			AccountCollateral::<T>::try_mutate(market_id, from, |collateral_balance| {
				let new_collateral_balance = *collateral_balance + amount;
				*collateral_balance = new_collateral_balance;
				Ok(())
			})
		}
	}
}
