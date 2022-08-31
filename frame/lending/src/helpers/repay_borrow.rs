
use composable_traits::defi::DeFiComposableConfig;
use frame_support::traits::fungibles::{Inspect, Mutate, MutateHold, Transfer};
use sp_runtime::DispatchError;

use crate::Config;

/// Repay `amount` of `beneficiary_account`'s `borrow_asset` debt principal.
///
/// Release given `amount` of `debt_token` from `beneficiary_account`, transfer `amount` from
/// `payer_account` to `market_account`, and then burn `amount` of `debt_token` from
/// `beneficiary_account`.
pub(crate) fn repay_principal<'a, T: Config>(
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
pub(crate) fn pay_interest<'a, T: Config>(
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
