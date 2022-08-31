use crate::{models::borrower_data::BorrowerData, weights::WeightInfo, types::MarketId, *};
use composable_support::{validation::TryIntoValidated, math::safe::{SafeAdd, SafeMul}};
use composable_traits::{
	defi::{
		DeFiEngine, ZeroToOneFixedU128,
	},
	lending::{
		BorrowAmountOf, Lending,
	},
	vault::{FundsAvailability, StrategicVault, Vault},
};
use frame_support::{
	pallet_prelude::*,
	traits::fungibles::{Inspect as NativeInspect, Mutate, MutateHold, Transfer as NativeTransfer},
    traits::fungible::Transfer,
	weights::WeightToFee,
};
use sp_runtime::{
	traits::Zero, DispatchError, Percent,
};

// private helper functions
impl<T: Config> Pallet<T> {
	/// Some of these checks remain to provide better errors. See [this clickup task](task) for
	/// more information.
	///
	/// [task]: <https://sharing.clickup.com/20465559/t/h/27yd3wt/7IB0QYYHXP0TZZT>
	pub(crate) fn can_borrow(
		market_id: &MarketId,
		debt_owner: &T::AccountId,
		amount_to_borrow: BorrowAmountOf<Self>,
		market: MarketConfigOf<T>,
		market_account: &T::AccountId,
	) -> Result<(), DispatchError> {
		// this check prevents free flash loans
		if let Some(latest_borrow_timestamp) = BorrowTimestamp::<T>::get(market_id, debt_owner) {
			if latest_borrow_timestamp >= LastBlockTimestamp::<T>::get() {
				return Err(Error::<T>::InvalidTimestampOnBorrowRequest.into())
			}
		}

		let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
		let borrow_limit = Self::get_borrow_limit(market_id, debt_owner)?;
		let borrow_amount_value = Self::get_price(borrow_asset, amount_to_borrow)?;
		ensure!(borrow_limit >= borrow_amount_value, Error::<T>::NotEnoughCollateralToBorrow);

		Self::ensure_can_borrow_from_vault(&market.borrow_asset_vault, market_account)?;

		Ok(())
	}

	// Checks if we can borrow from the vault.
	// If available_funds() returns FundsAvailability::Depositable then vault is unbalanced,
	// and we can not borrow, except the case when returned balances equals zero.
	// In the case of FundsAvailability::MustLiquidate we obviously can not borrow, since the market
	// is going to be closed. If FundsAvailability::Withdrawable is return, we can borrow, since
	// vault has extra money that will be used for balancing in the next block. So, if we even
	// borrow all assets from the market, vault has posibity for rebalancing.
	pub(crate) fn ensure_can_borrow_from_vault(
		vault_id: &T::VaultId,
		account_id: &T::AccountId,
	) -> Result<(), DispatchError> {
		match <T::Vault as StrategicVault>::available_funds(vault_id, account_id)? {
			FundsAvailability::Depositable(balance) => balance
				.is_zero()
				.then_some(())
				.ok_or(Error::<T>::CannotBorrowFromMarketWithUnbalancedVault),
			FundsAvailability::MustLiquidate => Err(Error::<T>::MarketIsClosing),
			FundsAvailability::Withdrawable(_) => Ok(()),
		}?;
		Ok(())
	}
}

// public helper functions
impl<T: Config> Pallet<T> {
		pub fn do_borrow(
		market_id: &MarketId,
		borrowing_account: &T::AccountId,
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
			let deposit = T::WeightToFee::weight_to_fee(&T::WeightInfo::liquidate(2));
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


    /// Creates a new [`BorrowerData`] for the given market and account. See [`BorrowerData`]
	/// for more information.
	pub fn create_borrower_data(
		market_id: &<Self as Lending>::MarketId,
		account: &<Self as DeFiEngine>::AccountId,
	) -> Result<BorrowerData, DispatchError> {
		let (_, market) = Self::get_market(market_id)?;

		let collateral_balance_value = Self::get_price(
			market.collateral_asset,
			Self::collateral_of_account(market_id, account)?,
		)?;

		let account_total_debt_with_interest =
			Self::total_debt_with_interest(market_id, account)?.unwrap_or_zero();
		let borrow_balance_value = Self::get_price(
			T::Vault::asset_id(&market.borrow_asset_vault)?,
			account_total_debt_with_interest,
		)?;

		let borrower = BorrowerData::new(
			collateral_balance_value,
			borrow_balance_value,
			market
				.collateral_factor
				.try_into_validated()
				.map_err(|_| Error::<T>::CollateralFactorMustBeMoreThanOne)?, /* TODO: Use a proper
			                                                                * error mesage */
			market.under_collateralized_warn_percent,
		);

		Ok(borrower)
	}
}

