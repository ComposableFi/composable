use crate::{
	strategies::repayment_strategies::{
		interest_periodically_principal_when_mature, principal_only, RepaymentResult,
		RepaymentStrategy,
	},
	types::{LoanConfigOf, LoanInputOf, MarketConfigOf, MarketInfoOf, MarketInputOf, TimeMeasure},
	validation::{AssetIsSupportedByOracle, CurrencyPairIsNotSame, LoanInputIsValid},
	Config, DebtTokenForMarketStorage, Error, MarketsStorage, Pallet,
};
use composable_support::validation::Validated;
use composable_traits::{
	currency::CurrencyFactory,
	defi::DeFiComposableConfig,
	oracle::Oracle,
	undercollateralized_loans::{LoanConfig, MarketConfig, MarketInfo, UndercollateralizedLoans},
	vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig},
};
use frame_support::{
	ensure,
	storage::with_transaction,
	traits::{
		fungibles::{Inspect, Mutate, Transfer},
		Get, UnixTime,
	},
};
use sp_runtime::{
	traits::{One, Saturating, Zero},
	DispatchError, Percent, Perquintill, TransactionOutcome,
};

use sp_std::vec::Vec;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

// #generalization
impl<T: Config> Pallet<T> {
	pub(crate) fn do_create_market(
		manager: T::AccountId,
		input: Validated<
			MarketInputOf<T>,
			(CurrencyPairIsNotSame, AssetIsSupportedByOracle<T::Oracle>),
		>,
		keep_alive: bool,
	) -> Result<MarketInfoOf<T>, DispatchError> {
		let config_input = input.value();
		crate::MarketsCounterStorage::<T>::try_mutate(|counter| {
			*counter += T::Counter::one();
			ensure!(
				*counter <= T::MaxMarketsCounterValue::get(),
				Error::<T>::ExceedMaxMarketsCounterValue
			);
			let market_account_id = Self::market_account_id(*counter);
			let borrow_asset_vault = T::Vault::create(
				Deposit::Existential,
				VaultConfig {
					asset_id: config_input.borrow_asset(),
					reserved: config_input.reserved_factor(),
					manager: manager.clone(),
					strategies: [(
						market_account_id.clone(),
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

			T::MultiCurrency::transfer(
				config_input.borrow_asset(),
				&manager,
				&market_account_id,
				initial_pool_size,
				keep_alive,
			)?;

			let market_config = MarketConfig::new(
				market_account_id.clone(),
				manager,
				borrow_asset_vault,
				config_input.borrow_asset(),
				config_input.collateral_asset(),
				config_input.max_price_age,
				config_input.whitelist,
			);
			let market_info = MarketInfo::new(market_config, config_input.liquidation_strategies);
			let debt_token_id = T::CurrencyFactory::reserve_lp_token_id(T::Balance::default())?;

			DebtTokenForMarketStorage::<T>::insert(market_account_id.clone(), debt_token_id);
			MarketsStorage::<T>::insert(market_account_id, market_info.clone());
			Ok(market_info)
		})
	}
	// Create non-active loan, which should be activated via borrower.
	// TODO: @mikolaichuk: check why LoanInputOf does not work here
	pub(crate) fn do_create_loan(
		input: Validated<LoanInputOf<T>, LoanInputIsValid<crate::Pallet<T>>>,
	) -> Result<LoanConfigOf<T>, DispatchError> {
		let config_input = input.value();
		// Convert schedule timestamps from string to seconds have passed from the beginning of UNIX
		// epoche.
		let schedule = Self::convert_schedule_timestamps(&config_input.payment_schedule)?;
		// Create non-activated loan and increment loans' counter.
		// This loan have to be activated by borrower further.
		crate::LoansCounterStorage::<T>::try_mutate(|counter| {
			*counter += T::Counter::one();
			let loan_account_id = Self::loan_account_id(*counter);
			let loan_config = LoanConfig::new(
				loan_account_id.clone(),
				config_input.market_account_id,
				config_input.borrower_account_id,
				config_input.principal,
				config_input.collateral,
				schedule,
				config_input.repayment_strategy,
			);
			crate::LoansStorage::<T>::insert(loan_account_id.clone(), loan_config.clone());
			crate::NonActiveLoansStorage::<T>::insert(loan_account_id, ());
			Ok(loan_config)
		})
	}

	/// Borrow assets as per loan_account loan terms.
	/// Activates the loan.
	/// Supposed to be called from transactional dispatchable function.
	pub(crate) fn do_borrow(
		borrower_account_id: T::AccountId,
		loan_account_id: T::AccountId,
		keep_alive: bool,
	) -> Result<LoanConfigOf<T>, DispatchError> {
		// Check if loan's account id is in non-activated loans list.
		// If it is not, loan does not exist or was already activated.
		ensure!(
			crate::NonActiveLoansStorage::<T>::contains_key(loan_account_id.clone()),
			Error::<T>::LoanDoesNotExistOrWasActivated
		);
		let loan_config = Self::get_loan_config_via_account_id(&loan_account_id)?;
		// Check if borrower is authorized to execute this loan agreement.
		ensure!(
			*loan_config.borrower_account_id() == borrower_account_id,
			Error::<T>::ThisUserIsNotAllowedToExecuteThisContract
		);
		// Check if borrower tries to activate expired loan.
		let today = Utc::today().naive_utc();
		let first_payment_date =
			NaiveDateTime::from_timestamp_opt(*loan_config.first_payment_moment(), 0)
				.ok_or(Error::<T>::OutOfRangeNumberSecondInTimestamp)?
				.date();
		// Loan should be activated before the first payment date.
		if today >= first_payment_date {
			crate::NonActiveLoansStorage::<T>::remove(loan_account_id.clone());
			crate::LoansStorage::<T>::remove(loan_account_id.clone());
			Err(Error::<T>::TheLoanContractIsExpired)?;
		}
		// Obtain market's configuration.
		let market_config =
			Self::get_market_config_via_account_id(loan_config.market_account_id())?;
		// Transfer collateral from the borrower's account to the loan's account.
		let collateral_asset_id = *market_config.collateral_asset();
		let source = &borrower_account_id;
		let destination = &loan_account_id;
		let amount = *loan_config.collateral();
		T::MultiCurrency::transfer(collateral_asset_id, source, destination, amount, keep_alive)?;
		// Transfer borrowed assets from market's account to the borrower's account.
		let borrow_asset_id = *market_config.borrow_asset();
		let source = market_config.account_id();
		let destination = &borrower_account_id;
		let amount = *loan_config.principal();
		T::MultiCurrency::transfer(borrow_asset_id, source, destination, amount, keep_alive)?;
		// Mint 'principal' amount of debt tokens to the loan's account.
		// We use these tokens to indicate which part of principal is not refunded yet.
		let debt_token_id =
			crate::DebtTokenForMarketStorage::<T>::get(loan_config.market_account_id())
				.ok_or(Error::<T>::MarketDoesNotExist)?;
		<T as Config>::MultiCurrency::mint_into(debt_token_id, &loan_account_id, amount)?;
		// Loan is active now.
		// Remove loan configuration from the non-activated loans accounts ids storage.
		crate::NonActiveLoansStorage::<T>::remove(loan_account_id.clone());
		// Build payment schedule.
		for timestamp in loan_config.schedule().keys() {
			crate::ScheduleStorage::<T>::mutate(timestamp, |loans_accounts_ids| {
				loans_accounts_ids.insert(loan_account_id.clone())
			});
		}
		Ok(loan_config)
	}

	// Repay any amount of money
	pub(crate) fn do_repay(
		payer_account_id: T::AccountId,
		loan_account_id: T::AccountId,
		repay_amount: T::Balance,
		keep_alive: bool,
	) -> Result<T::Balance, DispatchError> {
		// Get loan's info.
		let loan_config = Self::get_loan_config_via_account_id(&loan_account_id)?;
		// Get account id of market which holds this loan.
		let market_account_id = loan_config.market_account_id();
		let market_info = Self::get_market_info_via_account_id(market_account_id)?;
		let borrow_asset_id = market_info.config().borrow_asset();
		// Transfer 'amount' of assets from the payer account to the loan account
		T::MultiCurrency::transfer(
			*borrow_asset_id,
			&payer_account_id,
			&loan_account_id,
			repay_amount,
			keep_alive,
		)
	}

	pub(crate) fn do_liquidate() -> () {}

	// Close loan contract since loan is paid.
	pub(crate) fn close_loan_contract(
		loan_account_id_ref: &T::AccountId,
		keep_alive: bool,
	) -> Result<(T::AccountId, Vec<TimeMeasure>), crate::Error<T>> {
		let loan_config = Self::get_loan_config_via_account_id(loan_account_id_ref)?;
		let borrower_account_id = loan_config.borrower_account_id();
		let collateral_amount = loan_config.collateral();
		let market_account_id = loan_config.market_account_id();
		let market_config = Self::get_market_config_via_account_id(market_account_id)?;
		let collateral_asset_id = market_config.collateral_asset();
		// Transfer collateral to borrower's account.
		T::MultiCurrency::transfer(
			*collateral_asset_id,
			loan_account_id_ref,
			borrower_account_id,
			*collateral_amount,
			keep_alive,
		)
		.map_err(|_| crate::Error::<T>::CollateralCanNotBeTransferedBackToTheBorrowersAccount)?;
		// Remove all information about the loan.
		let payments_moments = Self::terminate_activated_loan(loan_account_id_ref)?;
		Ok((loan_account_id_ref.clone(), payments_moments))
	}

	// Check if borrower's account is whitelisted for particular market.
	pub(crate) fn is_borrower_account_whitelisted(
		borrower_account_id_ref: &T::AccountId,
		market_account_id_ref: &T::AccountId,
	) -> Result<bool, DispatchError> {
		let market_config = Self::get_market_config_via_account_id(market_account_id_ref)?;
		Ok(market_config.whitelist().contains(borrower_account_id_ref))
	}

	pub(crate) fn calculate_initial_pool_size(
		borrow_asset: <T::Oracle as composable_traits::oracle::Oracle>::AssetId,
	) -> Result<<T as DeFiComposableConfig>::Balance, DispatchError> {
		T::Oracle::get_price_inverse(borrow_asset, T::OracleMarketCreationStake::get())
	}

	// Check if provided account id belongs to the market manager.
	pub(crate) fn is_market_manager_account(
		account_id: &T::AccountId,
		market_account_id_ref: &T::AccountId,
	) -> Result<bool, DispatchError> {
		let market_config = Self::get_market_config_via_account_id(market_account_id_ref)?;
		Ok(market_config.manager() == account_id)
	}

	// TODO: @mikolaichuk: Add weights calculation
	// #generalization
	pub(crate) fn treat_vaults_balance(block_number: T::BlockNumber) -> () {
		let _ = with_transaction(|| {
			let now = Self::now();
			let mut errors = crate::MarketsStorage::<T>::iter()
				.map(|(market_account_id, market_info)| {
					match Self::available_funds(&market_info.config(), &market_account_id)? {
						FundsAvailability::Withdrawable(balance) => {
							Self::handle_withdrawable(
								&market_info.config(),
								&market_account_id,
								balance,
							)?;
						},
						FundsAvailability::Depositable(balance) => {
							Self::handle_depositable(
								&market_info.config(),
								&market_account_id,
								balance,
							)?;
						},
						FundsAvailability::MustLiquidate => {
							Self::handle_must_liquidate(&market_info.config(), market_account_id)?;
						},
					}
					Result::<(), DispatchError>::Ok(())
				})
				.filter_map(|r| match r {
					Ok(_) => None,
					Err(err) => Some(err),
				})
				.peekable();

			if errors.peek().is_none() {
				crate::LastBlockTimestamp::<T>::put(now);
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
	}

	pub(crate) fn check_payments(keep_alive: bool) -> () {
		let today = Self::today();
		// Retrive collection of loans(loans' accounts ids) which have to be paid now.
		// If nothing found we will get empty set.
		let loans_accounts_ids = crate::ScheduleStorage::<T>::try_get(today).unwrap_or_default();

		let mut garbage_loans_ids = vec![];
		for loan_account_id in loans_accounts_ids {
			// Treat situation when non-existend loan's id is in global schedule.
			let loan_config = match Self::get_loan_config_via_account_id(&loan_account_id) {
				Ok(loan_config) => loan_config,
				Err(_) => {
					garbage_loans_ids.push(loan_account_id);
					continue
				},
			};
			let repayment_result: RepaymentResult<T> = match loan_config.repayment_strategy() {
				RepaymentStrategy::InterestPeriodicallyPrincipalWhenMature =>
					interest_periodically_principal_when_mature::apply(
						loan_config,
						&today,
						keep_alive,
					),
				RepaymentStrategy::PrincipalOnlyWhenMature =>
					principal_only::apply(loan_config, &today, keep_alive),
			};

			match repayment_result {
				// Failed not beacuse of user fault.
				// Should not happens at all.
				RepaymentResult::Failed(_) => (),
				RepaymentResult::InterestIsPaidInTime(_) => (),
				RepaymentResult::InterestIsNotPaidInTime(_) => Self::do_liquidate(),
				RepaymentResult::PrincipalAndLastInterestPaymentArePaidBackInTime(_) => {
					Self::close_loan_contract(&loan_account_id, keep_alive);
					()
				},
				RepaymentResult::PrincipalAndLastInterestPaymentAreNotPaidBackInTime(_) =>
					Self::do_liquidate(),
			}
			// We do not need information regarding this date anymore.
			crate::ScheduleStorage::<T>::remove(today);
		}
	}

	// Check if vault balanced or we have to deposit money to the vault or withdraw money from it.
	// If vault is balanced we will do nothing.
	// #generalization
	fn available_funds(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
	) -> Result<FundsAvailability<T::Balance>, DispatchError> {
		<T::Vault as StrategicVault>::available_funds(&config.borrow_asset_vault(), market_account)
	}

	// If we can withdraw from the vault
	// #generalization
	fn handle_withdrawable(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
		balance: T::Balance,
	) -> Result<(), DispatchError> {
		<T::Vault as StrategicVault>::withdraw(
			&config.borrow_asset_vault(),
			market_account,
			balance,
		)
	}

	// If vault is unblanced and we have to deposit some assets to the vault.
	// #generalization
	fn handle_depositable(
		config: &MarketConfigOf<T>,
		market_account: &T::AccountId,
		balance: T::Balance,
	) -> Result<(), DispatchError> {
		let asset_id = <T::Vault as Vault>::asset_id(&config.borrow_asset_vault())?;
		let balance =
			<T as Config>::MultiCurrency::reducible_balance(asset_id, market_account, false)
				.min(balance);
		<T::Vault as StrategicVault>::deposit(&config.borrow_asset_vault(), market_account, balance)
	}

	// TODO: @mikolaichuk: Implement logic when vault is stopped, tombstoned or does not exist.
	// #generalization
	fn handle_must_liquidate(
		_market_config: &MarketConfigOf<T>,
		_market_account_id: T::AccountId,
	) -> Result<(), DispatchError> {
		todo!()
	}

	// #generalization
	pub(crate) fn now() -> u64 {
		T::UnixTime::now().as_secs()
	}

	// #generalization
	pub(crate) fn get_market_info_via_account_id(
		market_account_id_ref: &T::AccountId,
	) -> Result<MarketInfoOf<T>, crate::Error<T>> {
		crate::MarketsStorage::<T>::try_get(market_account_id_ref)
			.map_err(|_| crate::Error::<T>::MarketDoesNotExist)
	}

	pub(crate) fn get_market_config_via_account_id(
		market_account_id_ref: &T::AccountId,
	) -> Result<MarketConfigOf<T>, crate::Error<T>> {
		let market_info = Self::get_market_info_via_account_id(market_account_id_ref)?;
		Ok(market_info.config().clone())
	}

	pub(crate) fn get_loan_config_via_account_id(
		loan_account_id_ref: &T::AccountId,
	) -> Result<LoanConfigOf<T>, crate::Error<T>> {
		crate::LoansStorage::<T>::try_get(loan_account_id_ref)
			.map_err(|_| crate::Error::<T>::ThereIsNoSuchLoan)
	}

	// Convert timestamps from strings to seconds have passed from 01.01.1970.
	pub(crate) fn convert_schedule_timestamps(
		schedule: &Vec<(String, Percent)>,
	) -> Result<Vec<(TimeMeasure, Percent)>, crate::Error<T>> {
		let mut output = vec![];
		for (timestamp, percent) in schedule {
			output.push((
				NaiveDate::parse_from_str(&timestamp, "%d.%m.%Y")
					.map_err(|_| crate::Error::<T>::IncorrectTimestampFormat)?
					.and_time(NaiveTime::default())
					.timestamp(),
				*percent,
			));
		}
		Ok(output)
	}

	// Removes expired non-active loans.
	// Expired non-active loans are loans which were not activated by borrower.
	// First payment date of such loans is less or equal to the current date.
	pub(crate) fn terminate_non_active_loans() -> Result<Vec<T::AccountId>, crate::Error<T>> {
		let mut removed_non_active_accounts_ids = vec![];
		for non_active_loan_account_id in crate::NonActiveLoansStorage::<T>::iter_keys() {
			let loan_config = Self::get_loan_config_via_account_id(&non_active_loan_account_id)?;
			if Self::today() >= *loan_config.first_payment_moment() {
				crate::LoansStorage::<T>::remove(non_active_loan_account_id.clone());
				removed_non_active_accounts_ids.push(non_active_loan_account_id);
			}
		}
		removed_non_active_accounts_ids
			.iter()
			.for_each(|account_id| crate::NonActiveLoansStorage::<T>::remove(account_id));
		Ok(removed_non_active_accounts_ids)
	}

	// Remove all information about activated loan.
	pub(crate) fn terminate_activated_loan(
		loan_account_id_ref: &T::AccountId,
	) -> Result<Vec<TimeMeasure>, crate::Error<T>> {
		// Get payment moments for the loan.
		let payment_moments: Vec<TimeMeasure> =
			Self::get_loan_config_via_account_id(loan_account_id_ref)?
				.schedule()
				.keys()
				.map(|&payment_moment| payment_moment)
				.collect();
		// Remove account id from global payment schedule for each payment date.
		payment_moments.iter().for_each(|payment_moment| {
			crate::ScheduleStorage::<T>::mutate(payment_moment, |loans_accounts_ids| {
				loans_accounts_ids.remove(loan_account_id_ref)
			});
		});
		Ok(payment_moments)
	}
	pub(crate) fn today() -> TimeMeasure {
		Utc::today().naive_utc().and_time(NaiveTime::default()).timestamp()
	}
}
