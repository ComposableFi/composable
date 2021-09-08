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

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, FullCodec};
	use composable_traits::{
		currency::CurrencyFactory,
		lending::{
			BorrowAmountOf, CollateralLpAmountOf, Lending, MarketConfig, MarketConfigInput,
			NormalizedCollateralFactor, Timestamp,
		},
		oracle::Oracle,
		rate_model::*,
		vault::{FundsAvailability, StrategicVault, Vault},
	};
	use frame_support::{
		pallet_prelude::*,
		storage::{with_transaction, TransactionOutcome},
		traits::{
			fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer},
			tokens::{DepositConsequence, WithdrawConsequence},
			GenesisBuild, UnixTime,
		},
		PalletId,
	};
	use num_traits::{CheckedDiv, SaturatingSub};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, One,
			Zero,
		},
		ArithmeticError, FixedPointNumber, FixedPointOperand, FixedU128,
	};
	use sp_std::fmt::Debug;

	#[derive(Default, Debug, Copy, Clone, Encode, Decode)]
	#[repr(transparent)]
	pub struct MarketIndex(u32);

	impl MarketIndex {
		pub fn new(i: u32) -> Self {
			Self(i)
		}
	}

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	/// number like of hi bits, so that amount and balance calculations are don it it with high
	/// precision via fixed point
	/// while this is 128 bit, cannot support u128
	/// can support it if to use FixedU256
	type LiftedFixedBalance = FixedU128;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Oracle: Oracle<AssetId = Self::AssetId, Balance = Self::Balance>;
		type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;
		type Vault: StrategicVault<
			VaultId = Self::VaultId,
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
		>;

		type CurrencyFactory: CurrencyFactory<Self::AssetId>;
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
			+ From<u64> // at least 64 bit
			+ Zero
			+ FixedPointOperand
			+ Into<LiftedFixedBalance> // integer part not more than bits in this
			+ Into<u128>; // cannot do From<u128>, until LiftedFixedBalance integer part is larger than 128
			  // bit

		// actually there are 2 types of currencies:
		// 1. vault owned - can transfer, cannot mint
		// 2. market owned - debt token can be minted
		type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ MutateHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>
			+ InspectHold<Self::AccountId, Balance = Self::Balance, AssetId = Self::AssetId>;

		type UnixTime: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let now = T::UnixTime::now().as_secs();
			if LastBlockTimestamp::<T>::get().is_zero() {
				LastBlockTimestamp::<T>::put(now);
			}
			with_transaction(|| {
				let res = Markets::<T>::iter()
					.map(|(index, _)| <Pallet<T>>::accrue_interest(&index))
					.collect();
				match res {
					Ok(()) => {
						LastBlockTimestamp::<T>::put(now);
						TransactionOutcome::Commit(1000)
					},
					Err(err) => {
						log::error!("This should never happen, could not initialize block!!! {:#?} {:#?}", block_number, err);
						TransactionOutcome::Rollback(0)
					},
				}
			});
			0
		}
	}

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
		MarketAndAccountPairNotFound,
		NotEnoughCollateralToBorrowAmount,
		CannotBorrowInCurrentLendingState,
		NotEnoughBorrowAsset,
		NotEnoughCollateral,
		TransferFailed,
		CannotWithdrawFromProvidedBorrowAccount,
		CannotRepayMoreThanBorrowAmount,
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

	/// Original debt values are on balances.
	/// Debt token allows to simplify some debt management and implementation of features
	#[pallet::storage]
	#[pallet::getter(fn debt_currencies)]

	pub type DebtMarkets<T: Config> =
		StorageMap<_, Twox64Concat, MarketIndex, T::AssetId, ValueQuery>;

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

	/// market borrow index
	#[pallet::storage]
	#[pallet::getter(fn borrow_index)]
	pub type BorrowIndex<T: Config> = StorageMap<_, Twox64Concat, MarketIndex, Ratio, ValueQuery>;

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

	/// The timestamp of the previous block or defaults to timestamp at genesis.
	#[pallet::storage]
	#[pallet::getter(fn last_block_timestamp)]
	pub type LastBlockTimestamp<T: Config> = StorageValue<_, Timestamp, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub last_block_timestamp: Timestamp,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			GenesisConfig { last_block_timestamp: 0 }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			LastBlockTimestamp::<T>::put(self.last_block_timestamp.clone());
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn account_id(market_id: &<Self as Lending>::MarketId) -> <Self as Lending>::AccountId {
			<Self as Lending>::account_id(market_id)
		}
		pub fn calc_utilization_ratio(
			cash: &<Self as Lending>::Balance,
			borrows: &<Self as Lending>::Balance,
			reserves: &<Self as Lending>::Balance,
		) -> Result<Ratio, DispatchError> {
			<Self as Lending>::calc_utilization_ratio(cash, borrows, reserves)
		}
		pub fn create(
			borrow_asset_vault: <T::Vault as Vault>::VaultId,
			collateral_asset_vault: <T::Vault as Vault>::VaultId,
			config_input: MarketConfigInput<<Self as Lending>::AccountId>,
		) -> Result<<Self as Lending>::MarketId, DispatchError> {
			<Self as Lending>::create(borrow_asset_vault, collateral_asset_vault, config_input)
		}
	}

	impl<T: Config> Lending for Pallet<T> {
		/// we are operating only on vault types, so restricted by these
		type VaultId = <T::Vault as Vault>::VaultId;
		type AccountId = <T::Vault as Vault>::AccountId;
		type Balance = T::Balance;

		type MarketId = MarketIndex;

		type BlockNumber = T::BlockNumber;

		fn create(
			borrow_asset_vault: <T::Vault as Vault>::VaultId,
			collateral_asset_vault: <T::Vault as Vault>::VaultId,
			config_input: MarketConfigInput<Self::AccountId>,
		) -> Result<Self::MarketId, DispatchError> {
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

			LendingCount::<T>::try_mutate(|MarketIndex(previous_market_index)| {
				let market_index = {
					*previous_market_index += 1;
					MarketIndex(*previous_market_index)
				};

				let config = MarketConfig {
					borrow: borrow_asset_vault,
					collateral: collateral_asset_vault,
					reserve_factor: config_input.reserve_factor,
					collateral_factor: config_input.collateral_factor,
					interest_rate: InterestRateModel::default(),
				};

				let debt_asset_id = T::CurrencyFactory::create()?;
				DebtMarkets::<T>::insert(market_index, debt_asset_id);
				Markets::<T>::insert(market_index, config);

				Ok(market_index)
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
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let borrower_balance_with_interest =
				Self::borrow_balance_current(market_id, debt_owner)?.unwrap_or_default();
			let asset_id = T::Vault::asset_id(&market.borrow)?;
			let borrow_asset_price = T::Oracle::get_price(&asset_id)?.0;
			let borrowed_normalized = borrower_balance_with_interest
				.checked_mul(&borrow_asset_price)
				.ok_or(Error::<T>::Overflow)?;

			let borrow_limit = Self::get_borrow_limit(&market_id, debt_owner)?;

			let possible_borrow = borrow_limit
				.checked_sub(&borrowed_normalized)
				.ok_or(Error::<T>::NotEnoughCollateralToBorrowAmount)?;

			let account_id = Self::account_id(market_id);
			let can_withdraw = T::Currency::reducible_balance(asset_id.clone(), &account_id, true);
			match T::Vault::available_funds(&market.borrow, &Self::account_id(&market_id))? {
				FundsAvailability::Withdrawable(balance) => {
					<T::Vault as StrategicVault>::withdraw(
						&market.borrow,
						&account_id,
						possible_borrow,
					)?;
					ensure!(
						can_withdraw + balance >= possible_borrow,
						Error::<T>::NotEnoughBorrowAsset
					)
				}
				FundsAvailability::Depositable(_) => (),
				// TODO: decide when react and how to return fees back
				// https://mlabs-corp.slack.com/archives/C02CRQ9KW04/p1630662664380600?thread_ts=1630658877.363600&cid=C02CRQ9KW04
				FundsAvailability::MustLiquidate => {
					return Err(Error::<T>::CannotBorrowInCurrentLendingState.into())
				}
			}

			T::Currency::transfer(
				asset_id.clone(),
				&Self::account_id(market_id),
				debt_owner,
				possible_borrow,
				true,
			)?;

			let market_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			//ASK: storage or currency?
			//<T::DebtCurrency as Mutate<T::AccountId>>::mint_into(market_id.clone(), debt_owner,
			//<T::DebtCurrency possible_borrow)?;
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			T::Currency::mint_into(debt_asset_id, debt_owner, possible_borrow)?;
			T::Currency::hold(debt_asset_id, debt_owner, possible_borrow)?;

			// TODO: decide what todo do with reborrow  https://mlabs-corp.slack.com/archives/C02CRQ9KW04/p1631005365082200
			DebtIndex::<T>::insert(market_id, debt_owner, market_index);

			Ok(())
		}

		fn repay_borrow(
			market_id: &Self::MarketId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Option<BorrowAmountOf<Self>>,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			if let Some(owed) = Self::borrow_balance_current(market_id, beneficiary)? {
				let repay_amount = repay_amount.unwrap_or(owed);
				let borrow_id = T::Vault::asset_id(&market.borrow)?;
				ensure!(repay_amount <= owed, Error::<T>::CannotRepayMoreThanBorrowAmount);
				ensure!(
					T::Currency::can_withdraw(borrow_id, &from, repay_amount).into_result().is_ok(),
					Error::<T>::CannotWithdrawFromProvidedBorrowAccount
				);
				let market_account = Self::account_id(market_id);
				ensure!(
					T::Currency::can_deposit(borrow_id, &market_account, repay_amount)
						.into_result()
						.is_ok(),
					Error::<T>::TransferFailed
				);
				let debt_asset_id = DebtMarkets::<T>::get(market_id);

				let burn_amount = T::Currency::balance(debt_asset_id, beneficiary);

				// TODO: fuzzing is must to uncover cases when sum != total
				let market_debt_reduction = T::Currency::balance(debt_asset_id, &market_account)
					.checked_sub(&burn_amount)
					.expect("debt balance of market must be of parts of debts of borrowers");
				T::Currency::burn_from(debt_asset_id, &market_account, market_debt_reduction).expect(
					"debt balance of market must be of parts of debts of borrowers and can reduce it",
				);
				T::Currency::burn_from(debt_asset_id, beneficiary, burn_amount)
					.expect("can always burn current balance");
				T::Currency::transfer(borrow_id, from, &market_account, repay_amount, false)
					.expect("must be able to transfer because of above checks");

				// TODO: not sure why Warp V2 (Blacksmith) does that, but seems will need to revise it later with some strategy
				let interest_index = BorrowIndex::<T>::get(market_id);
				DebtIndex::<T>::insert(market_id, beneficiary, interest_index);

				// we do not optimize vault here, will do it on finalize after all repays
			}

			Ok(())
		}

		fn total_borrows(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let accrued_debt = T::Currency::balance(debt_asset_id, &Self::account_id(market_id));
			let total_issued = T::Currency::total_issuance(debt_asset_id);
			Ok(total_issued - accrued_debt)
		}

		fn total_cash(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			Ok(T::Currency::total_issuance(debt_asset_id))
		}

		fn total_reserves(_pair: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
			Ok(Self::Balance::zero())
		}

		fn update_borrows(
			market_id: &Self::MarketId,
			delta_interest_rate: Rate,
		) -> Result<(), DispatchError> {
			// we maintain original borrow principals intact on hold,
			// but accrue total borrow balance by adding to market debt balance
			// when user pays loan back, we reduce marked accrued debt
			// so no need to loop over each account -> scales to millions of users
			let debt_asset_id = DebtMarkets::<T>::get(market_id);
			let total_debt = Self::total_borrows(market_id)?;
			let accrued_interest =
				T::Currency::balance(debt_asset_id, &Self::account_id(market_id));
			let total_borrows: FixedU128 = total_debt
				.checked_sub(&accrued_interest)
				.ok_or(ArithmeticError::Overflow)?
				.into();
			let accrued = total_borrows
				.checked_mul(&delta_interest_rate)
				.and_then(|x| x.checked_mul_int(1u64))
				.ok_or(ArithmeticError::Overflow)?
				.into();
			T::Currency::mint_into(debt_asset_id, &Self::account_id(market_id), accrued)
		}

		fn calc_utilization_ratio(
			cash: &Self::Balance,
			borrows: &Self::Balance,
			reserves: &Self::Balance,
		) -> Result<Ratio, DispatchError> {
			// utilization ratio is 0 when there are no borrows
			if borrows.is_zero() {
				return Ok(Ratio::zero());
			}
			// utilizationRatio = totalBorrows / (totalCash + totalBorrows − totalReserves)
			let total: u128 = cash
				.checked_add(borrows)
				.and_then(|r| r.checked_sub(reserves))
				.ok_or(Error::<T>::Overflow)?
				.into();
			let borrows: u128 = (*borrows).into();
			let util_ratio = Ratio::saturating_from_rational(borrows, total);
			Ok(util_ratio.clamp(0.into(), 1.into()))
		}

		fn accrue_interest(market_id: &Self::MarketId) -> Result<(), DispatchError> {
			let total_borrows = Self::total_borrows(market_id)?;
			let total_cash = Self::total_cash(market_id)?;
			let total_reserves = Self::total_reserves(market_id)?;
			let utilization_ratio =
				Self::calc_utilization_ratio(&total_cash, &total_borrows, &total_reserves)?;
			let delta_time = T::UnixTime::now()
				.as_secs()
				.checked_sub(Self::last_block_timestamp())
				.ok_or(Error::<T>::Underflow)?;
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_rate = market.interest_rate.get_borrow_rate(utilization_ratio).unwrap();

			//update borrow_index
			let borrow_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let borrow_index_new = increment_index(borrow_rate, borrow_index, delta_time)
				.and_then(|r| r.checked_add(&borrow_index))
				.ok_or(Error::<T>::Overflow)?;
			// overwrite value
			BorrowIndex::<T>::insert(market_id, borrow_index_new);

			let delta_interest_rate =
				increment_borrow_rate(borrow_rate, delta_time).ok_or(Error::<T>::Overflow)?;
			Self::update_borrows(market_id, delta_interest_rate)?;

			//TODO: update_reserves
			Ok(())
		}

		fn borrow_balance_current(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Option<BorrowAmountOf<Self>>, DispatchError> {
			let debt_asset_id = DebtMarkets::<T>::try_get(market_id)
				.map_err(|_| Error::<T>::MarketAndAccountPairNotFound)?;
			let principal = T::Currency::balance_on_hold(debt_asset_id, account);
			let account_interest_index = DebtIndex::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketAndAccountPairNotFound)?;

			let market_interest_index =
				BorrowIndex::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let balance = borrow_from_principal::<T>(
				principal,
				market_interest_index,
				account_interest_index,
			)?;

			Ok(balance.map(Into::into))
		}

		fn collateral_of_account(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			AccountCollateral::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketCollateralWasNotDepositedByAccount.into())
		}

		fn collateral_required(
			_market_id: &Self::MarketId,
			_borrow_amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			todo!()
		}

		fn get_borrow_limit(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, DispatchError> {
			let config =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let collateral_balance = AccountCollateral::<T>::try_get(market_id, account)
				.map_err(|_| Error::<T>::MarketCollateralWasNotDepositedByAccount)?;
			let underlying_collateral_balance: LiftedFixedBalance =
				<T::Vault as Vault>::to_underlying_value(&config.collateral, collateral_balance)?
					.into();
			let collateral_asset_id = <T::Vault as Vault>::asset_id(&config.collateral)?;
			let collateral_price: LiftedFixedBalance =
				<T::Oracle as Oracle>::get_price(&collateral_asset_id)?.0.into();

			let normalized_limit = underlying_collateral_balance
				.checked_mul(&collateral_price)
				.and_then(|collateral_normalized| {
					collateral_normalized.checked_div(&config.collateral_factor)
				})
				.and_then(|borrow_normalized| borrow_normalized.checked_mul_int(1u64))
				.ok_or(Error::<T>::Overflow)?;

			Ok(normalized_limit.into())
		}

		fn deposit_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;
			let collateral_lp_id = T::Vault::lp_asset_id(&market.collateral)?;
			let market_account = Self::account_id(&market_id);
			ensure!(
				T::Currency::can_withdraw(collateral_lp_id, &account, amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFailed
			);
			ensure!(
				T::Currency::can_deposit(collateral_lp_id, &market_account, amount)
					== DepositConsequence::Success,
				Error::<T>::TransferFailed
			);

			AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
				let new_collateral_balance = (*collateral_balance)
					.checked_add(&amount)
					.ok_or(Error::<T>::Overflow.into())?;
				*collateral_balance = new_collateral_balance;
				Result::<(), Error<T>>::Ok(())
			})?;
			T::Currency::transfer(collateral_lp_id, account, &market_account, amount, true)
				.expect("we'd better exit; qed;");
			Ok(())
		}

		fn withdraw_collateral(
			market_id: &Self::MarketId,
			account: &Self::AccountId,
			amount: CollateralLpAmountOf<Self>,
		) -> Result<(), DispatchError> {
			let market =
				Markets::<T>::try_get(market_id).map_err(|_| Error::<T>::MarketDoesNotExist)?;

			let borrow_asset = T::Vault::asset_id(&market.borrow)?;
			let collateral_asset = T::Vault::asset_id(&market.collateral)?;

			let (collateral_price, _) = T::Oracle::get_price(&collateral_asset)?;

			let current_borrow =
				Self::borrow_balance_current(market_id, account)?.unwrap_or_default();
			let (borrow_price, _) = T::Oracle::get_price(&borrow_asset)?;
			let current_borrow_value =
				current_borrow.checked_mul(&borrow_price).ok_or(Error::<T>::Overflow)?;

			let max_borrowable_value = Self::get_borrow_limit(market_id, account)?;

			/*
				v = (max - current) * factor

				In practice a user will not withdraw v
				because it would probably result in quick liquidation?
			*/
			let withdrawable_collateral_value = max_borrowable_value
				.checked_sub(&current_borrow_value)
				.and_then(|x| market.collateral_factor.checked_mul_int(x))
				.ok_or(Error::<T>::Overflow)?;

			let collateral_from_collateral_lp =
				<T::Vault as Vault>::to_underlying_value(&market.collateral, amount)?;
			let collateral_value = collateral_from_collateral_lp
				.checked_mul(&collateral_price)
				.ok_or(Error::<T>::Overflow)?;

			ensure!(
				collateral_value > withdrawable_collateral_value,
				Error::<T>::NotEnoughCollateral
			);

			let market_account = Self::account_id(&market_id);
			ensure!(
				T::Currency::can_deposit(collateral_asset, &account, amount)
					== DepositConsequence::Success,
				Error::<T>::TransferFailed
			);
			ensure!(
				T::Currency::can_withdraw(collateral_asset, &market_account, amount)
					.into_result()
					.is_ok(),
				Error::<T>::TransferFailed
			);

			AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
				let new_collateral_balance =
					(*collateral_balance).checked_sub(&amount).ok_or(Error::<T>::Underflow)?;
				*collateral_balance = new_collateral_balance;
				Result::<(), Error<T>>::Ok(())
			})?;
			T::Currency::transfer(collateral_asset, &market_account, account, amount, true)
				.expect("we'd better exit; qed;");
			Ok(())
		}
	}

	// If borrowBalance = 0 then borrow index is likely also 0.
	// Rather than failing the calculation with a division by 0, we immediately return 0 in
	// this case.
	fn borrow_from_principal<T: Config>(
		principal: <T as Config>::Balance,
		market_interest_index: Ratio,
		account_interest_index: Ratio,
	) -> Result<Option<u64>, DispatchError> {
		if principal.is_zero() {
			return Ok(None);
		}
		let principal: LiftedFixedBalance = principal.into();
		let balance = principal
			.checked_mul(&market_interest_index)
			.and_then(|from_start_total| from_start_total.checked_div(&account_interest_index))
			.and_then(|x| x.checked_mul_int(1u64))
			.ok_or(ArithmeticError::Overflow)?;
		Ok(Some(balance))
	}
}
