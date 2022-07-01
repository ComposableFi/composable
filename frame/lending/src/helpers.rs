use crate::{models::borrower_data::BorrowerData, types::AccruedInterest, weights::WeightInfo, *};

use crate::{
	types::InitializeBlockCallCounters,
	validation::{
		AssetIsSupportedByOracle, BalanceGreaterThenZero, CurrencyPairIsNotSame, MarketModelValid,
		UpdateInputValid,
	},
};
use composable_support::{
	math::safe::{SafeAdd, SafeDiv, SafeMul},
	validation::{TryIntoValidated, Validated},
};
use composable_traits::{
	currency::CurrencyFactory,
	defi::{DeFiComposableConfig, *},
	lending::{math::*, BorrowAmountOf, CollateralLpAmountOf, Lending, MarketConfig, UpdateInput},
	liquidation::Liquidation,
	oracle::Oracle,
	time::{DurationSeconds, SECONDS_PER_YEAR_NAIVE},
	vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig},
};
use frame_support::{
	pallet_prelude::*,
	storage::{with_transaction, TransactionOutcome},
	traits::{
		fungible::{Inspect as NativeInspect, Transfer as NativeTransfer},
		fungibles::{Inspect, Transfer},
		UnixTime,
	},
	weights::WeightToFeePolynomial,
};
use sp_runtime::{
	traits::{One, Saturating, Zero},
	ArithmeticError, DispatchError, FixedPointNumber, FixedU128, Percent, Perquintill,
};
use sp_std::vec::Vec;

// private helper functions
impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_market(
		manager: T::AccountId,
		input: Validated<
			CreateInputOf<T>,
			(MarketModelValid, CurrencyPairIsNotSame, AssetIsSupportedByOracle<T::Oracle>),
		>,
		keep_alive: bool,
	) -> Result<(<Self as Lending>::MarketId, T::VaultId), DispatchError> {
		let config_input = input.value();
		LendingCount::<T>::try_mutate(|MarketIndex(previous_market_index)| {
			let market_id = {
				// TODO: early mutation of `previous_market_index` value before check.
				*previous_market_index += 1;
				ensure!(
					*previous_market_index <= T::MaxMarketCount::get(),
					Error::<T>::ExceedLendingCount
				);
				MarketIndex(*previous_market_index)
			};

			let borrow_asset_vault = T::Vault::create(
				Deposit::Existential,
				VaultConfig {
					asset_id: config_input.borrow_asset(),
					reserved: config_input.reserved_factor(),
					manager: manager.clone(),
					strategies: [(
						Self::account_id(&market_id),
						// Borrowable = 100% - reserved
						// REVIEW: Review use of `saturating_sub` here - I'm pretty sure this
						// can never error, but if `Perquintill` can be `>`
						// `Perquintill::one()` then we might want to re-evaluate the logic
						// here.
						Perquintill::one().saturating_sub(config_input.reserved_factor()),
					)]
					.into_iter()
					.collect(),
				},
			)?;

			let initial_pool_size = Self::calculate_initial_pool_size(config_input.borrow_asset())?;

			ensure!(
				initial_pool_size > T::Balance::zero(),
				Error::<T>::PriceOfInitialBorrowVaultShouldBeGreaterThanZero
			);

			// transfer `initial_pool_size` worth of borrow asset from the manager to the market
			T::MultiCurrency::transfer(
				config_input.borrow_asset(),
				&manager,
				&Self::account_id(&market_id),
				initial_pool_size,
				keep_alive,
			)?;

			let market_config = MarketConfig {
				manager,
				max_price_age: config_input.updatable.max_price_age,
				borrow_asset_vault: borrow_asset_vault.clone(),
				collateral_asset: config_input.collateral_asset(),
				collateral_factor: config_input.updatable.collateral_factor,
				interest_rate_model: config_input.interest_rate_model,
				under_collateralized_warn_percent: config_input
					.updatable
					.under_collateralized_warn_percent,
				liquidators: config_input.updatable.liquidators,
			};
			// TODO: pass ED from API,
			let debt_token_id = T::CurrencyFactory::reserve_lp_token_id(T::Balance::default())?;

			DebtTokenForMarket::<T>::insert(market_id, debt_token_id);
			Markets::<T>::insert(market_id, market_config);
			BorrowIndex::<T>::insert(market_id, FixedU128::one());

			Ok((market_id, borrow_asset_vault))
		})
	}
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

	pub(crate) fn do_update_market(
		manager: T::AccountId,
		market_id: MarketIndex,
		input: Validated<
			UpdateInput<T::LiquidationStrategyId, <T as frame_system::Config>::BlockNumber>,
			UpdateInputValid,
		>,
	) -> DispatchResultWithPostInfo {
		let input = input.value();
		Markets::<T>::mutate(&market_id, |market| {
			if let Some(market) = market {
				ensure!(manager == market.manager, Error::<T>::Unauthorized);

				ensure!(
					market.collateral_factor >= input.collateral_factor,
					Error::<T>::CannotIncreaseCollateralFactorOfOpenMarket
				);
				market.collateral_factor = input.collateral_factor;
				market.under_collateralized_warn_percent = input.under_collateralized_warn_percent;
				market.liquidators = input.liquidators.clone();
				Ok(())
			} else {
				Err(Error::<T>::MarketDoesNotExist)
			}
		})?;
		Self::deposit_event(Event::<T>::MarketUpdated { market_id, input });
		Ok(().into())
	}

	/// Returns pair of market's id and market (as 'MarketConfing') via market's id
	/// - `market_id` : Market index as a key in 'Markets' storage
	pub(crate) fn get_market(
		market_id: &MarketIndex,
	) -> Result<(&MarketIndex, MarketConfigOf<T>), DispatchError> {
		Markets::<T>::get(market_id)
			.map(|market| (market_id, market))
			.ok_or_else(|| Error::<T>::MarketDoesNotExist.into())
	}

	/// Get TWAP from oracle. If history of prices is empty then return latest price.
	pub(crate) fn get_price(
		asset_id: <T as DeFiComposableConfig>::MayBeAssetId,
		amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		<T::Oracle as Oracle>::get_twap_for_amount(asset_id, amount)
	}

	/// Some of these checks remain to provide better errors. See [this clickup task](task) for
	/// more information.
	///
	/// [task]: <https://sharing.clickup.com/20465559/t/h/27yd3wt/7IB0QYYHXP0TZZT>
	pub(crate) fn can_borrow(
		market_id: &MarketIndex,
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

		ensure!(
			<T as Config>::MultiCurrency::can_withdraw(
				borrow_asset,
				market_account,
				amount_to_borrow
			)
			.into_result()
			.is_ok(),
			Error::<T>::NotEnoughBorrowAsset,
		);

		if !BorrowRent::<T>::contains_key(market_id, debt_owner) {
			let deposit = T::WeightToFee::calc(&T::WeightInfo::liquidate(1));
			// See note 1
			ensure!(
				<T as Config>::NativeCurrency::can_withdraw(debt_owner, deposit)
					.into_result()
					.is_ok(),
				Error::<T>::NotEnoughRent,
			);
		}

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
				.then(|| ())
				.ok_or(Error::<T>::CannotBorrowFromMarketWithUnbalancedVault),
			FundsAvailability::MustLiquidate => Err(Error::<T>::MarketIsClosing),
			FundsAvailability::Withdrawable(_) => Ok(()),
		}?;
		Ok(())
	}

	/// Check is price actual yet
	pub(crate) fn ensure_price_is_recent(market: &MarketConfigOf<T>) -> Result<(), DispatchError> {
		use sp_runtime::traits::CheckedSub as _;

		let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;

		let current_block = frame_system::Pallet::<T>::block_number();
		let blocks_count = market.max_price_age;
		let edge_block = current_block.checked_sub(&blocks_count).unwrap_or_default();

		// check borrow asset
		let price_block =
			<T::Oracle as Oracle>::get_price(borrow_asset, BorrowAmountOf::<Self>::default())?
				.block;
		ensure!(price_block >= edge_block, Error::<T>::PriceTooOld);

		// check collateral asset
		let collateral_asset = market.collateral_asset;
		let price_block =
			<T::Oracle as Oracle>::get_price(collateral_asset, BorrowAmountOf::<Self>::default())?
				.block;
		ensure!(price_block >= edge_block, Error::<T>::PriceTooOld);

		Ok(())
	}
}

// public helper functions
impl<T: Config> Pallet<T> {
	/// Returns the initial pool size for a market with `borrow_asset`. Calculated with
	/// [`Config::OracleMarketCreationStake`].
	pub fn calculate_initial_pool_size(
		borrow_asset: <T::Oracle as composable_traits::oracle::Oracle>::AssetId,
	) -> Result<<T as composable_traits::defi::DeFiComposableConfig>::Balance, DispatchError> {
		T::Oracle::get_price_inverse(borrow_asset, T::OracleMarketCreationStake::get())
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

	/// Whether or not an account should be liquidated. See [`BorrowerData::should_liquidate()`]
	/// for more information.
	pub fn should_liquidate(
		market_id: &<Self as Lending>::MarketId,
		account: &<Self as DeFiEngine>::AccountId,
	) -> Result<bool, DispatchError> {
		let borrower = Self::create_borrower_data(market_id, account)?;
		let should_liquidate = borrower.should_liquidate()?;
		Ok(should_liquidate)
	}

	pub fn soon_under_collateralized(
		market_id: &<Self as Lending>::MarketId,
		account: &<Self as DeFiEngine>::AccountId,
	) -> Result<bool, DispatchError> {
		let borrower = Self::create_borrower_data(market_id, account)?;
		let should_warn = borrower.should_warn()?;
		Ok(should_warn)
	}

	/// Initiate liquidation of individual position for particular borrower within mentioned
	/// market. Returns 'Ok(())' in the case of successful initiation, 'Err(DispatchError)' in
	/// the opposite case.
	/// - `liquidator` : Liquidator's account id.
	/// - `market_pair` : Index and configuration of the market from which tokens were borrowed.
	/// - `account` : Borrower's account id whose debt are going to be liquidated.
	fn liquidate_position(
		liquidator: &<Self as DeFiEngine>::AccountId,
		market_pair: &(&<Self as Lending>::MarketId, MarketConfigOf<T>),
		borrow_asset: <T as DeFiComposableConfig>::MayBeAssetId,
		account: &<Self as DeFiEngine>::AccountId,
	) -> Result<(), DispatchError> {
		let (market_id, market) = market_pair;
		ensure!(
			Self::should_liquidate(market_id, account)?,
			DispatchError::Other("Tried liquidate position which is not supposed to be liquidated")
		);

		let collateral_to_liquidate = Self::collateral_of_account(market_id, account)?;

		let source_target_account = Self::account_id(market_id);

		let unit_price =
			T::Oracle::get_ratio(CurrencyPair::new(market.collateral_asset, borrow_asset))?;

		let sell =
			Sell::new(market.collateral_asset, borrow_asset, collateral_to_liquidate, unit_price);
		T::Liquidation::liquidate(&source_target_account, sell, market.liquidators.clone())?;
		if let Some(deposit) = BorrowRent::<T>::get(market_id, account) {
			let market_account = Self::account_id(market_id);
			<T as Config>::NativeCurrency::transfer(&market_account, liquidator, deposit, false)?;
		}
		Ok(())
	}

	/// Liquidates debt for each borrower in the vector within mentioned market.
	/// Returns a vector of borrowers' account ids whose debts were liquidated.
	/// - `liquidator` : Liquidator's account id.
	/// - `market_id` : Market index from which `borrowers` has taken borrow.
	/// - `borrowers` : Vector of borrowers whose debts are going to be liquidated.
	pub fn liquidate_internal(
		liquidator: &<Self as DeFiEngine>::AccountId,
		market_id: &<Self as Lending>::MarketId,
		borrowers: BoundedVec<<Self as DeFiEngine>::AccountId, T::MaxLiquidationBatchSize>,
	) -> Result<Vec<<Self as DeFiEngine>::AccountId>, DispatchError> {
		// Vector of borrowers whose positions are involved in the liquidation process.
		let mut subjected_borrowers: Vec<<Self as DeFiEngine>::AccountId> = Vec::new();
		let market_pair = Self::get_market(market_id)?;
		let borrow_asset = T::Vault::asset_id(&market_pair.1.borrow_asset_vault)?;
		for account in borrowers.iter() {
			// Wrap liquidate position request in a storage transaction.
			// So, in the case of any error state's changes will not be committed
			let storage_transaction_succeeded =
				with_transaction(|| {
					let liquidation_response_result =
						Self::liquidate_position(liquidator, &market_pair, borrow_asset, account);
					if let Err(error) = liquidation_response_result {
						log::warn!("Creation of liquidation request for position {:?} {:?} was failed: {:?}",
						market_id,
						account,
						error );
						return TransactionOutcome::Rollback(liquidation_response_result)
					}
					TransactionOutcome::Commit(Ok(()))
				});

			// If storage transaction succeeded,
			// push borrower to the output vector,
			// remove debt records from storages.
			if storage_transaction_succeeded.is_ok() {
				subjected_borrowers.push(account.clone());
				BorrowTimestamp::<T>::remove(market_id, account);
				DebtIndex::<T>::remove(market_id, account);
			}
		}
		Ok(subjected_borrowers)
	}
}

// crate-public helper functions
impl<T: Config> Pallet<T> {
	pub(crate) fn initialize_block(block_number: T::BlockNumber) -> InitializeBlockCallCounters {
		let mut call_counters = InitializeBlockCallCounters::default();
		let _ = with_transaction(|| {
			let now = Self::now();
			call_counters.now += 1;

			let mut errors = Markets::<T>::iter()
				.map(|(market_id, config)| {
					call_counters.read_markets += 1;
					Self::accrue_interest(&market_id, now)?;
					call_counters.accrue_interest += 1;
					let market_account = Self::account_id(&market_id);
					call_counters.account_id += 1;
					// NOTE(hussein-aitlahcen):
					// It would probably be more perfomant to handle theses
					// case while borrowing/repaying.
					//
					// I don't know whether we would face any issue by doing that.
					//
					// borrow:
					//  - withdrawable = transfer(vault->market) + transfer(market->user)
					//  - depositable = error(not enough borrow asset) // vault asking for reserve
					//    to be fullfilled
					//  - mustliquidate = error(market is closing)
					// repay:
					// 	- (withdrawable || depositable || mustliquidate) = transfer(user->market) +
					//    transfer(market->vault)
					//
					// The intermediate transfer(vault->market) while borrowing would
					// allow the vault to update the strategy balance (market = borrow vault
					// strategy).
					match Self::available_funds(&config, &market_account)? {
						FundsAvailability::Withdrawable(balance) => {
							Self::handle_withdrawable(&config, &market_account, balance)?;
							call_counters.handle_withdrawable += 1;
						},
						FundsAvailability::Depositable(balance) => {
							Self::handle_depositable(&config, &market_account, balance)?;
							call_counters.handle_depositable += 1;
						},
						FundsAvailability::MustLiquidate => {
							Self::handle_must_liquidate(&config, &market_account)?;
							call_counters.handle_must_liquidate += 1;
						},
					}

					call_counters.available_funds += 1;

					Result::<(), DispatchError>::Ok(())
				})
				.filter_map(|r| match r {
					Ok(_) => None,
					Err(err) => Some(err),
				})
				.peekable();

			if errors.peek().is_none() {
				LastBlockTimestamp::<T>::put(now);
				TransactionOutcome::Commit(Ok(1000))
			} else {
				errors.for_each(|e| {
					log::error!(
						"This should never happen, could not initialize block!!! {:#?} {:#?}",
						block_number,
						e
					)
				});
				TransactionOutcome::Rollback(Err(DispatchError::Other(
					"failed to initialize block",
				)))
			}
		});
		call_counters
	}

	pub(crate) fn now() -> u64 {
		T::UnixTime::now().as_secs()
	}

	pub(crate) fn available_funds(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
	) -> Result<FundsAvailability<T::Balance>, DispatchError> {
		<T::Vault as StrategicVault>::available_funds(&config.borrow_asset_vault, market_account)
	}

	pub(crate) fn handle_withdrawable(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
		balance: T::Balance,
	) -> Result<(), DispatchError> {
		<T::Vault as StrategicVault>::withdraw(&config.borrow_asset_vault, market_account, balance)
	}

	pub(crate) fn handle_depositable(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
		balance: T::Balance,
	) -> Result<(), DispatchError> {
		let asset_id = <T::Vault as Vault>::asset_id(&config.borrow_asset_vault)?;
		let balance =
			<T as Config>::MultiCurrency::reducible_balance(asset_id, market_account, false)
				.min(balance);
		<T::Vault as StrategicVault>::deposit(&config.borrow_asset_vault, market_account, balance)
	}

	pub(crate) fn handle_must_liquidate(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
	) -> Result<(), DispatchError> {
		let asset_id = <T::Vault as Vault>::asset_id(&config.borrow_asset_vault)?;
		let balance =
			<T as Config>::MultiCurrency::reducible_balance(asset_id, market_account, false);
		<T::Vault as StrategicVault>::deposit(&config.borrow_asset_vault, market_account, balance)
	}

	/// Returns the borrow and debt assets for the given market, if it exists.
	pub(crate) fn get_assets_for_market(
		market_id: &MarketIndex,
	) -> Result<MarketAssets<T>, DispatchError> {
		let (_, market) = Self::get_market(market_id)?;
		let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
		let debt_asset =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		Ok(MarketAssets { borrow_asset, debt_asset })
	}
}

// various helper functions

/// given collateral information, how much of borrow asset can get?
pub fn swap(
	collateral_balance: &LiftedFixedBalance,
	collateral_price: &LiftedFixedBalance,
	collateral_factor: &MoreThanOneFixedU128,
) -> Result<LiftedFixedBalance, ArithmeticError> {
	collateral_balance.safe_mul(collateral_price)?.safe_div(collateral_factor)
}

/// ```python
/// delta_interest_rate = delta_time / period_interest_rate
/// debt_delta = debt_principal * delta_interest_rate
/// new_accrued_debt = accrued_debt + debt_delta
/// total_debt = debt_principal + new_accrued_debt
/// ```
pub(crate) fn accrue_interest_internal<T: Config, I: InterestRate>(
	utilization_ratio: Percent,
	interest_rate_model: &mut I,
	borrow_index: OneOrMoreFixedU128,
	delta_time: DurationSeconds,
	total_borrows: T::Balance,
) -> Result<AccruedInterest<T>, DispatchError> {
	let total_borrows: FixedU128 =
		FixedU128::checked_from_integer(Into::<u128>::into(total_borrows))
			.ok_or(ArithmeticError::Overflow)?;

	let borrow_rate = interest_rate_model
		.get_borrow_rate(utilization_ratio)
		.ok_or(Error::<T>::BorrowRateDoesNotExist)?;

	// borrow_rate * index * delta_time / SECONDS_PER_YEAR_NAIVE + index
	let borrow_rate_delta = borrow_rate
		.safe_mul(&FixedU128::saturating_from_integer(delta_time))?
		.safe_div(&FixedU128::saturating_from_integer(SECONDS_PER_YEAR_NAIVE))?;

	let new_borrow_index = borrow_rate_delta.safe_mul(&borrow_index)?.safe_add(&borrow_index)?;

	let accrued_increment = total_borrows
		.safe_mul(&borrow_rate_delta)?
		.checked_mul_int(1_u64)
		.ok_or(ArithmeticError::Overflow)?
		.into();

	Ok(AccruedInterest { accrued_increment, new_borrow_index })
}

/// Retrieve the current interest rate for the given `market_id`.
#[cfg(test)]
pub fn current_interest_rate<T: Config>(market_id: MarketId) -> Result<Rate, DispatchError> {
	let market_id = MarketIndex::new(market_id);
	let total_borrowed_from_market_excluding_interest =
		Pallet::<T>::total_borrowed_from_market_excluding_interest(&market_id)?;
	let total_available_to_be_borrowed = Pallet::<T>::total_available_to_be_borrowed(&market_id)?;
	let utilization_ratio = Pallet::<T>::calculate_utilization_ratio(
		total_available_to_be_borrowed,
		total_borrowed_from_market_excluding_interest,
	)?;

	Markets::<T>::try_get(market_id)
		.map_err(|_| Error::<T>::MarketDoesNotExist)?
		.interest_rate_model
		.get_borrow_rate(utilization_ratio)
		.ok_or(Error::<T>::BorrowRateDoesNotExist)
		.map_err(Into::into)
}
