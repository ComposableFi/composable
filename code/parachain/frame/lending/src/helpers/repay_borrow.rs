use crate::{types::MarketId, *};

use crate::Config;
use composable_support::math::safe::SafeSub;
use composable_traits::{
	defi::DeFiComposableConfig,
	lending::{BorrowAmountOf, Lending, RepayStrategy, TotalDebtWithInterest},
};
use frame_support::{
	ensure,
	traits::{
		fungible::Transfer as NativeTransfer,
		fungibles::{Inspect, Mutate, MutateHold, Transfer},
	},
};
use sp_runtime::{traits::Zero, ArithmeticError, DispatchError, FixedPointNumber, FixedU128};

impl<T: Config> Pallet<T> {
	/// NOTE: Must be called in transaction!
	pub fn do_repay_borrow(
		market_id: &MarketId,
		from: &T::AccountId,
		beneficiary: &T::AccountId,
		total_repay_amount: RepayStrategy<BorrowAmountOf<Self>>,
		keep_alive: bool,
	) -> Result<BorrowAmountOf<Self>, DispatchError> {
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
				Self::pay_interest(
					borrow_asset,
					debt_asset,
					from,
					&market_account,
					beneficiary_interest_on_market,
					keep_alive,
				)?;

				// release and burn debt token from beneficiary and transfer borrow asset to
				// market, paid by `from`
				Self::repay_principal(
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
				Self::pay_interest(
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
				Self::repay_principal(
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
}

// private helper functions
impl<T: Config> Pallet<T> {
	/// Repay `amount` of `beneficiary_account`'s `borrow_asset` debt principal.
	///
	/// Release given `amount` of `debt_token` from `beneficiary_account`, transfer `amount` from
	/// `payer_account` to `market_account`, and then burn `amount` of `debt_token` from
	/// `beneficiary_account`.
	fn repay_principal<'a>(
		// The borrowed asset being repaid.
		borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,

		// The debt token to burn from `beneficiary_account`.
		debt_token: <T as DeFiComposableConfig>::MayBeAssetId,

		// The account repaying `beneficiary_account`'s debt.
		payer_account: &'a T::AccountId,

		// The market account that will be repaid.
		market_account: &'a T::AccountId,

		// The account that took out the borrow and who's debt is being repaid, i.e. the
		// beneficiary.
		beneficiary_account: &'a T::AccountId,

		// The amount of `beneficiary_account`'s debt to be repaid by `payer_account`.
		//
		// NOTE: This is assumed to be `<=` the total principal amount.
		amount_of_debt_to_repay: <T as DeFiComposableConfig>::Balance,

		// Whether or not to keep `from_account` alive.
		keep_alive: bool,
	) -> Result<(), DispatchError> {
		// release and burn debt token from beneficiary
		<T as Config>::MultiCurrency::release(
			debt_token,
			beneficiary_account,
			amount_of_debt_to_repay,
			false, // <- we don't want best_effort, all of it must be released
		)?;
		<T as Config>::MultiCurrency::burn_from(
			debt_token,
			beneficiary_account,
			amount_of_debt_to_repay,
		)?;

		// transfer from payer -> market
		// payer repays the debt
		<T as Config>::MultiCurrency::transfer(
			borrow_asset,
			payer_account,
			market_account,
			amount_of_debt_to_repay,
			keep_alive,
		)?;

		Ok(())
	}

	/// Pays off the interest accrued in a market.
	///
	/// Transfers `amount` of `borrow_asset` from `payer_account` to `market_account`,
	/// and then burns the same `amount` of `debt_asset` from `market_account`.
	fn pay_interest<'a>(
		// The borrowed asset.
		//
		// This is the asset that was originally borrowed, and is the same asset used to pay the
		// interest on the borrow (loan).
		borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,

		// The debt asset.
		//
		// This is the asset the market accrues interest into.
		debt_asset: <T as DeFiComposableConfig>::MayBeAssetId,

		// The account that is paying off the interest.
		payer_account: &'a T::AccountId,

		// The market account that owns the interest being paid off.
		market_account: &'a T::AccountId,

		// How much interest is being paid off.
		//
		// NOTE: This is assumed to be `<=` the total interest amount.
		amount_of_interest_to_repay: <T as DeFiComposableConfig>::Balance,

		// Whether or not to keep `from_account` alive.
		keep_alive: bool,
	) -> Result<(), DispatchError> {
		<T as Config>::MultiCurrency::transfer(
			borrow_asset,
			payer_account,
			market_account,
			amount_of_interest_to_repay,
			keep_alive,
		)?;

		let market_debt_asset_balance =
			<T as Config>::MultiCurrency::balance(debt_asset, market_account);

		<T as Config>::MultiCurrency::burn_from(
			debt_asset,
			market_account,
			// NOTE(benluelo):
			//
			// Due to precision errors, the actual interest balance may be *slightly* less
			// than the amount requested to repay. If that's the case, burn the amount
			// actually on the account. See the documentation on `DebtTokenForMarket` for more
			// information.
			if market_debt_asset_balance < amount_of_interest_to_repay {
				market_debt_asset_balance
			} else {
				amount_of_interest_to_repay
			},
		)?;

		Ok(())
	}
}
