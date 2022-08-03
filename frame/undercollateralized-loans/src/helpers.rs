use crate::{
	strategies::repayment_strategies::{
		interest_periodically_principal_when_mature_strategy, principal_only_fake_strategy,
		RepaymentResult, RepaymentStrategy,
	},
	types::{LoanConfigOf, LoanInfoOf, MarketConfigOf, MarketInfoOf, MarketInputOf},
	validation::{AssetIsSupportedByOracle, CurrencyPairIsNotSame, LoanInputIsValid},
	Config, DebtTokenForMarketStorage, Error, MarketsStorage, Pallet,
};
use composable_support::{math::safe::SafeAdd, validation::Validated};
use composable_traits::{
	currency::CurrencyFactory,
	defi::DeFiComposableConfig,
	oracle::Oracle,
	undercollateralized_loans::{
		LoanConfig, LoanInfo, LoanInput, MarketConfig, MarketInfo, UndercollateralizedLoans,
	},
	vault::{Deposit, FundsAvailability, StrategicVault, Vault, VaultConfig},
};
use frame_support::{
	ensure,
	storage::with_transaction,
	traits::{
		fungibles::{Inspect, Transfer},
		Get, UnixTime,
	},
};
use sp_runtime::{
	traits::{One, Saturating, Zero},
	DispatchError, Percent, Perquintill, TransactionOutcome,
};

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
		input: Validated<
			LoanInput<T::AccountId, T::Balance, T::BlockNumber, T::TimeMeasure, Percent, RepaymentStrategy>,
			LoanInputIsValid<crate::Pallet<T>>,
		>,
	) -> Result<LoanConfigOf<T>, DispatchError> {
		let config_input = input.value();
		// Create non-activated loan and increment loans' counter.
		// This loan have to be activated by borrower further.
		// TODO: @mikolaichuk:  unactivated loans may have lifetime.
		//                      once loan's lifetime expired the loan should be terminated.
		crate::LoansCounterStorage::<T>::try_mutate(|counter| {
			*counter += T::Counter::one();
			let loan_account_id = Self::loan_account_id(*counter);
			let loan_config = LoanConfig::new(
				loan_account_id.clone(),
				config_input.market_account_id,
				config_input.borrower_account_id,
				config_input.principal,
				config_input.collateral,
				config_input.interest,
				config_input.loan_maturity,
				config_input.repayment_strategy,
			);
			crate::NonActiveLoansStorage::<T>::insert(loan_account_id, loan_config.clone());
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
	) -> Result<LoanInfoOf<T>, DispatchError> {
		let loan_config = crate::NonActiveLoansStorage::<T>::try_get(loan_account_id.clone())
			.map_err(|_| Error::<T>::LoanDoesNotExist)?;
		// Check if borrower is authorized to execute this loan agreement.
		ensure!(
			*loan_config.borrower_account_id() == borrower_account_id,
			Error::<T>::ThisUserIsNotAllowedToExecuteThisContract
		);
		let market_info = Self::get_market_info_via_account_id(loan_config.market_account_id())?;
		let market_config = market_info.config();
		// Transfer collateral from the borrower's account to the loan account.
		let collateral_asset_id = *market_config.collateral_asset();
		let source = &borrower_account_id;
		let destination = &loan_account_id;
		let amount = *loan_config.collateral();
		T::MultiCurrency::transfer(collateral_asset_id, source, destination, amount, keep_alive)?;
		// Transfer borrowed assets from market account to the borrower account.
		let borrow_asset_id = *market_config.borrow_asset();
		let source = market_config.account_id();
		let destination = &borrower_account_id;
		let amount = *loan_config.principal();
		T::MultiCurrency::transfer(borrow_asset_id, source, destination, amount, keep_alive)?;
		// Set start block number equals to the current block number.
		// Calculate end block number before which pricnipal should be returned.
		let start_block_number = frame_system::Pallet::<T>::block_number();
		let loan_info = LoanInfo::new(loan_config.clone(), start_block_number)?;
		// Register activated loan.
		crate::LoansStorage::<T>::insert(loan_account_id.clone(), loan_info.clone());
		// Remove loan configuration from the non-activated loans storage.
		crate::NonActiveLoansStorage::<T>::remove(loan_account_id.clone());
		Ok(loan_info)
	}

	// Repay any amount of money
	pub(crate) fn do_repay(
		payer_account_id: T::AccountId,
		loan_account_id: T::AccountId,
		repay_amount: T::Balance,
		keep_alive: bool,
	) -> Result<T::Balance, DispatchError> {
		// Get loan's info.
		let loan_info = Self::get_loan_info_via_account_id(&loan_account_id)?;
		// Get account id of market which holds this loan.
		let market_account_id = loan_info.config().market_account_id();
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
	// Close a loan since it is paid.
	pub(crate) fn close_loan(loan_account_id: &T::AccountId, keep_alive: bool) -> () {
		let loan_info = match Self::get_loan_info_via_account_id(loan_account_id) {
			Ok(loan_info) => loan_info,
			Err(_) => return (),
		};
		let borrower_account_id = loan_info.config().borrower_account_id();
		let collateral_amount = loan_info.config().collateral();
		let market_account_id = loan_info.config().market_account_id();
		let market_info = match Self::get_market_info_via_account_id(market_account_id) {
			Ok(market_info) => market_info,
			Err(_) => return (),
		};
		let collateral_asset_id = market_info.config().collateral_asset();
		// Transfer collateral to borrower's account.
		T::MultiCurrency::transfer(
			*collateral_asset_id,
			loan_account_id,
			borrower_account_id,
			*collateral_amount,
			keep_alive,
		);
		crate::LoansStorage::<T>::remove(loan_account_id);
	}

	// Check if borrower's account is in whitelist of the particular market.
	pub(crate) fn is_borrower_account_whitelisted(
		borrower_account_id_ref: &T::AccountId,
		market_account_id_ref: &T::AccountId,
	) -> Result<bool, DispatchError> {
		let market_info = Self::get_market_info_via_account_id(market_account_id_ref)?;
		let market_config = market_info.config();
		Ok(market_config.whitelist().contains(borrower_account_id_ref))
	}

	pub(crate) fn calculate_initial_pool_size(
		borrow_asset: <T::Oracle as composable_traits::oracle::Oracle>::AssetId,
	) -> Result<<T as DeFiComposableConfig>::Balance, DispatchError> {
		T::Oracle::get_price_inverse(borrow_asset, T::OracleMarketCreationStake::get())
	}

	// Check if provided account id belongs to market manager.
	pub(crate) fn is_market_manager_account(
		account_id: &T::AccountId,
		market_account_id_ref: &T::AccountId,
	) -> Result<bool, DispatchError> {
		let market_info = Self::get_market_info_via_account_id(market_account_id_ref)?;
		let market_config = market_info.config();
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

	pub(crate) fn check_payments(block_nubmer: T::BlockNumber, keep_alive: bool) -> () {
		// Retrive collection of loans(loans' accounts ids) which have to be paid now
		let loans_accounts_ids =
			crate::PaymentsScheduleStorage::<T>::try_get(block_nubmer).unwrap_or_default();
		for loan_account_id in loans_accounts_ids {
			let loan_info = match Self::get_loan_info_via_account_id(&loan_account_id) {
				Ok(loan_info) => loan_info,
				// TODO: @mikolaichuk: add event emmition.
				Err(_) => continue,
			};
			let repayment_result: RepaymentResult<T> = match loan_info.config().repayment_strategy()
			{
				RepaymentStrategy::InterestPeriodicallyPrincipalWhenMature =>
					interest_periodically_principal_when_mature_strategy::apply(
						loan_info,
						block_nubmer,
						keep_alive,
					),
				RepaymentStrategy::PrincipalOnlyWhenMature =>
					principal_only_fake_strategy::apply(loan_info, keep_alive),
			};

			match repayment_result {
				// Failed not beacuse of user fault.
				// Should not happend at all.
				RepaymentResult::Failed(_) => (),
				RepaymentResult::InterestIsPaidInTime(_) => (),
				RepaymentResult::InterestIsNotPaidInTime(_) => Self::do_liquidate(),
				RepaymentResult::PrincipalAndLastInterestPaymentArePaidBackInTime(_) =>
					Self::close_loan(&loan_account_id, keep_alive),
				RepaymentResult::PrincipalAndLastInterestPaymentAreNotPaidBackInTime(_) =>
					Self::do_liquidate(),
			}
			// We do not need information regarding this date anymore.
			crate::PaymentsScheduleStorage::<T>::remove(block_nubmer);
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

	// TODO: @mikolaichuk: Implements logic when vault is stopped, tombstoned or epsent.
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

	pub(crate) fn get_loan_info_via_account_id(
		loan_account_id_ref: &T::AccountId,
	) -> Result<LoanInfoOf<T>, crate::Error<T>> {
		crate::LoansStorage::<T>::try_get(loan_account_id_ref)
			.map_err(|_| crate::Error::<T>::ThereIsNoSuchActiveLoan)
	}
}
