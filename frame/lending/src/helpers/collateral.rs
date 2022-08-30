
use crate::{models::borrower_data::BorrowerData, *};

use crate::{
	validation::{
		BalanceGreaterThenZero,
	},
};
use composable_support::{
	math::safe::{SafeAdd, SafeSub},
	validation::{TryIntoValidated, Validated},
};
use composable_traits::{
	lending::{
		CollateralLpAmountOf, Lending,
	},
	vault::{Vault},
};
use frame_support::{
	pallet_prelude::*,
	traits::{
		fungibles::{Inspect, Transfer},
		tokens::DepositConsequence,
	},
};
use sp_runtime::{
	ArithmeticError,
	traits::{Zero}, DispatchError,
};


// private helper functions
impl<T: Config> Pallet<T> {
	pub(crate) fn do_deposit_collateral(
		market_id: &<Self as Lending>::MarketId,
		account: &T::AccountId,
		amount: Validated<CollateralLpAmountOf<Self>, BalanceGreaterThenZero>,
		keep_alive: bool,
	) -> Result<(), DispatchError> {
		let amount = amount.value();
		let (_, market) = Self::get_market(market_id)?;
		let market_account = Self::account_id(market_id);

		AccountCollateral::<T>::try_mutate(market_id, account, |collateral_balance| {
			let new_collateral_balance =
				collateral_balance.unwrap_or_default().safe_add(&amount)?;
			collateral_balance.replace(new_collateral_balance);
			Result::<(), DispatchError>::Ok(())
		})?;

		<T as Config>::MultiCurrency::transfer(
			market.collateral_asset,
			account,
			&market_account,
			amount,
			keep_alive,
		)?;
		Ok(())
	}

	pub(crate) fn do_withdraw_collateral(
		market_id: &<Self as Lending>::MarketId,
		account: &T::AccountId,
		amount: Validated<CollateralLpAmountOf<Self>, BalanceGreaterThenZero>,
	) -> Result<(), DispatchError> {
		let amount = amount.value();
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
				.map_err(|_| ArithmeticError::Overflow)?, // TODO: Use a proper error mesage?
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
}

