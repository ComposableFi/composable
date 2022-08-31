use crate::{helpers::general::accrue_interest_internal, *};

use composable_support::{
	math::safe::{SafeDiv, SafeMul, SafeSub},
	validation::TryIntoValidated,
};
use composable_traits::{
	defi::*,
	lending::{
		math::{self, *},
		BorrowAmountOf, CollateralLpAmountOf, Lending, RepayStrategy, TotalDebtWithInterest,
		UpdateInput,
	},
	time::Timestamp,
	vault::Vault,
};
use frame_support::{
	pallet_prelude::*,
	traits::{
		fungibles::{Inspect, InspectHold, Mutate},
	},
};
use sp_runtime::{
	traits::{AccountIdConversion, Zero},
	ArithmeticError, DispatchError, FixedPointNumber, Percent,
};
use sp_std::vec::Vec;

impl<T: Config> Lending for Pallet<T> {
	type VaultId = <T::Vault as Vault>::VaultId;
	type MarketId = MarketId;
	type BlockNumber = T::BlockNumber;
	type LiquidationStrategyId = <T as Config>::LiquidationStrategyId;
	type Oracle = T::Oracle;

	fn create_market(
		manager: Self::AccountId,
		input: CreateInputOf<T>,
		keep_alive: bool,
	) -> Result<(Self::MarketId, Self::VaultId), DispatchError> {
		Self::do_create_market(manager, input.try_into_validated()?, keep_alive)
	}

	fn update_market(
		manager: Self::AccountId,
		market_id: Self::MarketId,
		input: UpdateInput<Self::LiquidationStrategyId, Self::BlockNumber>,
	) -> DispatchResultWithPostInfo {
		Self::do_update_market(manager, market_id, input.try_into_validated()?)
	}

	fn account_id(market_id: &Self::MarketId) -> Self::AccountId {
		T::PalletId::get().into_sub_account_truncating(market_id)
	}

	fn deposit_collateral(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
		amount: CollateralLpAmountOf<Self>,
		keep_alive: bool,
	) -> Result<(), DispatchError> {
		Self::do_deposit_collateral(market_id, account, amount.try_into_validated()?, keep_alive)
	}

	fn withdraw_collateral(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
		amount: CollateralLpAmountOf<Self>,
	) -> Result<(), DispatchError> {
		Self::do_withdraw_collateral(market_id, account, amount.try_into_validated()?)
	}

	fn get_markets_for_borrow(borrow: Self::VaultId) -> Vec<Self::MarketId> {
		Markets::<T>::iter()
			.filter_map(|(index, market)| market.borrow_asset_vault.eq(&borrow).then_some(index))
			.collect()
	}

	fn borrow(
		market_id: &Self::MarketId,
		borrowing_account: &Self::AccountId,
		amount_to_borrow: BorrowAmountOf<Self>,
	) -> Result<(), DispatchError> {
        Self::do_borrow(market_id, borrowing_account, amount_to_borrow)	
    }

	/// NOTE: Must be called in transaction!
	fn repay_borrow(
		market_id: &Self::MarketId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		total_repay_amount: RepayStrategy<BorrowAmountOf<Self>>,
		keep_alive: bool,
	) -> Result<BorrowAmountOf<Self>, DispatchError> {
        Self::do_repay_borrow(market_id, from, beneficiary, total_repay_amount, keep_alive)
    }

	fn total_borrowed_from_market_excluding_interest(
		market_id: &Self::MarketId,
	) -> Result<Self::Balance, DispatchError> {
		let debt_token =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		// total amount of debt *interest* owned by the market
		let total_debt_interest =
			<T as Config>::MultiCurrency::balance(debt_token, &Self::account_id(market_id));

		let total_issued = <T as Config>::MultiCurrency::total_issuance(debt_token);
		let total_amount_borrowed_from_market = total_issued.safe_sub(&total_debt_interest)?;

		Ok(total_amount_borrowed_from_market)
	}

	fn total_interest(market_id: &Self::MarketId) -> Result<Self::Balance, DispatchError> {
		let debt_token =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		// total amount of debt *interest* owned by the market
		let total_debt_interest =
			<T as Config>::MultiCurrency::balance(debt_token, &Self::account_id(market_id));

		Ok(total_debt_interest)
	}

	fn accrue_interest(market_id: &Self::MarketId, now: Timestamp) -> Result<(), DispatchError> {
		// we maintain original borrow principals intact on hold,
		// but accrue total borrow balance by adding to market debt balance
		// when user pays loan back, we reduce marked accrued debt
		// so no need to loop over each account -> scales to millions of users

		let total_borrowed_from_market_excluding_interest =
			Self::total_borrowed_from_market_excluding_interest(market_id)?;
		let total_available_to_be_borrowed = Self::total_available_to_be_borrowed(market_id)?;

		let utilization_ratio = Self::calculate_utilization_ratio(
			total_available_to_be_borrowed,
			total_borrowed_from_market_excluding_interest,
		)?;

		let delta_time = now.checked_sub(LastBlockTimestamp::<T>::get()).ok_or(
			ArithmeticError::Underflow,
		)?;

		let borrow_index =
			BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;
		let debt_asset_id =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		let accrued_interest = Markets::<T>::try_mutate(market_id, |market_config| {
			let market_config = market_config.as_mut().ok_or(Error::<T>::MarketDoesNotExist)?;

			accrue_interest_internal::<T, InterestRateModel>(
				utilization_ratio,
				&mut market_config.interest_rate_model,
				borrow_index,
				delta_time,
				total_borrowed_from_market_excluding_interest,
			)
		})?;

		// overwrites
		BorrowIndex::<T>::insert(market_id, accrued_interest.new_borrow_index);
		<T as Config>::MultiCurrency::mint_into(
			debt_asset_id,
			&Self::account_id(market_id),
			accrued_interest.accrued_increment,
		)?;

		Ok(())
	}

	fn total_available_to_be_borrowed(
		market_id: &Self::MarketId,
	) -> Result<Self::Balance, DispatchError> {
		let (_, market) = Self::get_market(market_id)?;
		let borrow_asset_id = T::Vault::asset_id(&market.borrow_asset_vault)?;
		Ok(<T as Config>::MultiCurrency::balance(borrow_asset_id, &Self::account_id(market_id)))
	}

	fn calculate_utilization_ratio(
		cash: Self::Balance,
		borrows: Self::Balance,
	) -> Result<Percent, DispatchError> {
		Ok(math::calculate_utilization_ratio(
			LiftedFixedBalance::saturating_from_integer(cash.into()),
			LiftedFixedBalance::saturating_from_integer(borrows.into()),
		)?)
	}

	// previously 'borrow_balance_current'
	fn total_debt_with_interest(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<TotalDebtWithInterest<BorrowAmountOf<Self>>, DispatchError> {
		let debt_token =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		// Self::get_assets_for_market()?;
		match DebtIndex::<T>::get(market_id, account) {
			Some(account_interest_index) => {
				let market_interest_index =
					BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

				let account_principal =
					<T as Config>::MultiCurrency::balance_on_hold(debt_token, account);

				if account_principal.is_zero() {
					Ok(TotalDebtWithInterest::NoDebt)
				} else {
					// REVIEW
					let account_principal =
						LiftedFixedBalance::saturating_from_integer(account_principal.into());
					// principal * (market index / debt index)
					let index_ratio = market_interest_index.safe_div(&account_interest_index)?;

					let balance = account_principal
						.safe_mul(&index_ratio)?
						// TODO: Balance should be u128 eventually
						.checked_mul_int(1_u64)
						.ok_or(ArithmeticError::Overflow)?;
					Ok(TotalDebtWithInterest::Amount(balance.into()))
				}
			},
			None => Ok(TotalDebtWithInterest::NoDebt),
		}
	}

	fn collateral_of_account(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<CollateralLpAmountOf<Self>, DispatchError> {
		AccountCollateral::<T>::get(market_id, account)
			.ok_or_else(|| Error::<T>::MarketCollateralWasNotDepositedByAccount.into())
	}

	fn collateral_required(
		market_id: &Self::MarketId,
		borrow_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		let (_, market) = Self::get_market(market_id)?;
		let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
		let borrow_amount_value = Self::get_price(borrow_asset, borrow_amount)?;

		Ok(LiftedFixedBalance::saturating_from_integer(borrow_amount_value.into())
			.safe_mul(&market.collateral_factor)?
			.checked_mul_int(1_u64)
			.ok_or(ArithmeticError::Overflow)?
			.into())
	}

	fn get_borrow_limit(
		market_id: &Self::MarketId,
		account: &Self::AccountId,
	) -> Result<Self::Balance, DispatchError> {
		let collateral_balance = AccountCollateral::<T>::get(market_id, account)
			// REVIEW: I don't think this should default to zero, only to check against zero
			// afterwards.
			.unwrap_or_else(CollateralLpAmountOf::<Self>::zero);

		if collateral_balance > T::Balance::zero() {
			let borrower = Self::create_borrower_data(market_id, account)?;
			let balance = borrower
				.get_borrow_limit()
				.map_err(|_| Error::<T>::BorrowerDataCalculationFailed)?
				.checked_mul_int(1_u64)
				.ok_or(ArithmeticError::Overflow)?;
			Ok(balance.into())
		} else {
			Ok(Self::Balance::zero())
		}
	}
}
