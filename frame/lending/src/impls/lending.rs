use crate::{
	helpers::accrue_interest_internal, models::borrower_data::BorrowerData, weights::WeightInfo, *,
};

use composable_support::{
	math::safe::{SafeAdd, SafeDiv, SafeMul, SafeSub},
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
		fungible::Transfer as NativeTransfer,
		fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer},
		tokens::DepositConsequence,
	},
	weights::WeightToFeePolynomial,
};
use sp_runtime::{
	traits::{AccountIdConversion, Zero},
	ArithmeticError, DispatchError, FixedPointNumber, FixedU128, Percent,
};
use sp_std::vec::Vec;

impl<T: Config> Lending for Pallet<T> {
	type VaultId = <T::Vault as Vault>::VaultId;
	type MarketId = MarketIndex;
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
		T::PalletId::get().into_sub_account(market_id)
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
		let (_, market) = Self::get_market(market_id)?;

		let collateral_balance = AccountCollateral::<T>::try_get(market_id, account)
			// REVIEW: Perhaps don't default to zero
			// REVIEW: What is expected behaviour if there is no collateral?
			.unwrap_or_else(|_| CollateralLpAmountOf::<Self>::zero());

		ensure!(amount <= collateral_balance, Error::<T>::NotEnoughCollateralToWithdraw);

		let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
		let borrower_balance_with_interest =
			Self::total_debt_with_interest(market_id, account)?.unwrap_or_zero();

		let borrow_balance_value = Self::get_price(borrow_asset, borrower_balance_with_interest)?;

		let collateral_balance_after_withdrawal_value =
			Self::get_price(market.collateral_asset, collateral_balance.safe_sub(&amount)?)?;

		let borrower_after_withdrawal = BorrowerData::new(
			collateral_balance_after_withdrawal_value,
			borrow_balance_value,
			market
				.collateral_factor
				.try_into_validated()
				.map_err(|_| Error::<T>::Overflow)?, // TODO: Use a proper error mesage?
			market.under_collateralized_warn_percent,
		);

		ensure!(
			!borrower_after_withdrawal.should_liquidate()?,
			Error::<T>::WouldGoUnderCollateralized
		);

		let market_account = Self::account_id(market_id);

		ensure!(
			<T as Config>::MultiCurrency::can_deposit(
				market.collateral_asset,
				account,
				amount,
				false
			) == DepositConsequence::Success,
			Error::<T>::TransferFailed
		);
		ensure!(
			<T as Config>::MultiCurrency::can_withdraw(
				market.collateral_asset,
				&market_account,
				amount
			)
			.into_result()
			.is_ok(),
			Error::<T>::TransferFailed
		);

		AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
			let new_collateral_balance =
				// REVIEW: Should we default if there's no collateral? Or should an error (something like "NoCollateralToWithdraw") be returned instead?
				collateral_balance.unwrap_or_default().safe_sub(&amount)?;

			collateral_balance.replace(new_collateral_balance);

			Result::<(), DispatchError>::Ok(())
		})?;
		<T as Config>::MultiCurrency::transfer(
			market.collateral_asset,
			&market_account,
			account,
			amount,
			true,
		)
		.expect("impossible; qed;");
		Ok(())
	}

	fn get_markets_for_borrow(borrow: Self::VaultId) -> Vec<Self::MarketId> {
		Markets::<T>::iter()
			.filter_map(|(index, market)| market.borrow_asset_vault.eq(&borrow).then(|| index))
			.collect()
	}

	fn borrow(
		market_id: &Self::MarketId,
		borrowing_account: &Self::AccountId,
		amount_to_borrow: BorrowAmountOf<Self>,
	) -> Result<(), DispatchError> {
		let (_, market) = Self::get_market(market_id)?;

		Self::ensure_price_is_recent(&market)?;

		let MarketAssets { borrow_asset, debt_asset: debt_asset_id } =
			Self::get_assets_for_market(market_id)?;

		let market_account = Self::account_id(market_id);

		Self::can_borrow(market_id, borrowing_account, amount_to_borrow, market, &market_account)?;

		let new_account_interest_index = {
			let market_index =
				BorrowIndex::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

			// previous account interest index
			let account_interest_index = DebtIndex::<T>::get(market_id, borrowing_account)
				.unwrap_or_else(ZeroToOneFixedU128::zero);

			// amount of debt currently
			let existing_principal_amount =
				<T as Config>::MultiCurrency::balance(debt_asset_id, borrowing_account);

			// principal_after_new_borrow
			let principal_after_new_borrow =
				existing_principal_amount.safe_add(&amount_to_borrow)?;

			// amount of principal the account already has
			let existing_borrow_share =
				Percent::from_rational(existing_principal_amount, principal_after_new_borrow);
			// amount of principal the account is adding
			let new_borrow_share =
				Percent::from_rational(amount_to_borrow, principal_after_new_borrow);

			market_index
				.safe_mul(&new_borrow_share.into())?
				.safe_add(&account_interest_index.safe_mul(&existing_borrow_share.into())?)?
		};

		// mint debt token into user and lock it (it's used as a marker of how much the account
		// has borrowed total)
		<T as Config>::MultiCurrency::mint_into(
			debt_asset_id,
			borrowing_account,
			amount_to_borrow,
		)?;
		<T as Config>::MultiCurrency::hold(debt_asset_id, borrowing_account, amount_to_borrow)?;

		// transfer borrow asset from market to the borrower
		<T as Config>::MultiCurrency::transfer(
			borrow_asset,
			&market_account,
			borrowing_account,
			amount_to_borrow,
			false,
		)?;
		DebtIndex::<T>::insert(market_id, borrowing_account, new_account_interest_index);
		BorrowTimestamp::<T>::insert(market_id, borrowing_account, LastBlockTimestamp::<T>::get());

		if !BorrowRent::<T>::contains_key(market_id, borrowing_account) {
			let deposit = T::WeightToFee::calc(&T::WeightInfo::liquidate(2));
			<T as Config>::NativeCurrency::transfer(
				borrowing_account,
				&market_account,
				deposit,
				true,
			)?;
			BorrowRent::<T>::insert(market_id, borrowing_account, deposit);
		} else {
			// REVIEW
		}

		Ok(())
	}

	/// NOTE: Must be called in transaction!
	fn repay_borrow(
		market_id: &Self::MarketId,
		from: &Self::AccountId,
		beneficiary: &Self::AccountId,
		total_repay_amount: RepayStrategy<BorrowAmountOf<Self>>,
		keep_alive: bool,
	) -> Result<BorrowAmountOf<Self>, DispatchError> {
		use crate::repay_borrow::{pay_interest, repay_principal};

		// cannot repay in the same block as the borrow
		let timestamp = BorrowTimestamp::<T>::get(market_id, beneficiary)
			.ok_or(Error::<T>::BorrowDoesNotExist)?;
		ensure!(
			timestamp != LastBlockTimestamp::<T>::get(),
			Error::<T>::BorrowAndRepayInSameBlockIsNotSupported
		);

		// principal + interest
		let beneficiary_total_debt_with_interest =
			match Self::total_debt_with_interest(market_id, beneficiary)? {
				TotalDebtWithInterest::Amount(amount) => amount,
				TotalDebtWithInterest::NoDebt =>
					return Err(Error::<T>::CannotRepayZeroBalance.into()),
			};

		let market_account = Self::account_id(market_id);

		let MarketAssets { borrow_asset, debt_asset } = Self::get_assets_for_market(market_id)?;

		// initial borrow amount
		let beneficiary_borrow_asset_principal =
			<T as Config>::MultiCurrency::balance(debt_asset, beneficiary);
		// interest accrued
		let beneficiary_interest_on_market =
			beneficiary_total_debt_with_interest.safe_sub(&beneficiary_borrow_asset_principal)?;

		ensure!(
			!beneficiary_total_debt_with_interest.is_zero(),
			Error::<T>::CannotRepayZeroBalance
		);

		let repaid_amount = match total_repay_amount {
			RepayStrategy::TotalDebt => {
				// pay interest, from -> market
				// burn debt token interest from market
				pay_interest::<T>(
					borrow_asset,
					debt_asset,
					from,
					&market_account,
					beneficiary_interest_on_market,
					keep_alive,
				)?;

				// release and burn debt token from beneficiary and transfer borrow asset to
				// market, paid by `from`
				repay_principal::<T>(
					borrow_asset,
					debt_asset,
					from,
					&market_account,
					beneficiary,
					beneficiary_borrow_asset_principal,
					keep_alive,
				)?;

				beneficiary_total_debt_with_interest
			},

			// attempt to repay a partial amount of the debt, paying off interest and principal
			// proportional to how much of each there is.
			RepayStrategy::PartialAmount(partial_repay_amount) => {
				ensure!(
					partial_repay_amount <= beneficiary_total_debt_with_interest,
					Error::<T>::CannotRepayMoreThanTotalDebt
				);

				// INVARIANT: ArithmeticError::Overflow is used as the error here as
				// beneficiary_total_debt_with_interest is known to be non-zero at this point
				// due to the check above (CannotRepayZeroBalance)

				let interest_percentage = FixedU128::checked_from_rational(
					beneficiary_interest_on_market,
					beneficiary_total_debt_with_interest,
				)
				.ok_or(ArithmeticError::Overflow)?;

				let principal_percentage = FixedU128::checked_from_rational(
					beneficiary_borrow_asset_principal,
					beneficiary_total_debt_with_interest,
				)
				.ok_or(ArithmeticError::Overflow)?;

				// pay interest, from -> market
				// burn interest (debt token) from market
				pay_interest::<T>(
					borrow_asset,
					debt_asset,
					from,
					&market_account,
					interest_percentage
						.checked_mul_int::<u128>(partial_repay_amount.into())
						.ok_or(ArithmeticError::Overflow)?
						.into(),
					keep_alive,
				)?;

				// release and burn debt token from beneficiary and transfer borrow asset to
				// market, paid by `from`
				repay_principal::<T>(
					borrow_asset,
					debt_asset,
					from,
					&market_account,
					beneficiary,
					principal_percentage
						.checked_mul_int::<u128>(partial_repay_amount.into())
						.ok_or(ArithmeticError::Overflow)?
						.into(),
					keep_alive,
				)?;

				// the above will short circuit if amount cannot be paid, so if this is reached
				// then we know `partial_repay_amount` has been repaid
				partial_repay_amount
			},
		};

		// if the borrow is completely repaid, remove the borrow information
		if repaid_amount == beneficiary_total_debt_with_interest {
			// borrow no longer exists as it has been repaid in entirety, remove the
			// timestamp & index
			BorrowTimestamp::<T>::remove(market_id, beneficiary);
			DebtIndex::<T>::remove(market_id, beneficiary);

			// give back rent (rent = deposit)
			let rent = BorrowRent::<T>::get(market_id, beneficiary)
				.ok_or(Error::<T>::BorrowRentDoesNotExist)?;

			<T as Config>::NativeCurrency::transfer(
				&market_account,
				beneficiary,
				rent,
				false, // we do not need to keep the market account alive
			)?;
		}

		Ok(repaid_amount)
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
			// REVIEW: INVARIANT: this error should never happen, `now` should always
			// be `> LastBlockTimestamp`
			Error::<T>::Underflow,
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
