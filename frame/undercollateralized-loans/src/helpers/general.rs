use crate::{
	types::{
		Counter, LoanConfigOf, LoanInputOf, MarketInfoOf, MarketInputOf, Payment, PaymentOf,
		PaymentOutcome, PaymentOutcomeOf, PaymentsOutcomes, Timestamp,
	},
	validation::{MarketInputIsValid, LoanInputIsValid},
	Config, DebtTokenForMarketStorage, Error, MarketsStorage, Pallet,
};
use composable_support::validation::Validated;
use composable_traits::{
	currency::CurrencyFactory,
	defi::{CurrencyPair, DeFiComposableConfig, Sell},
	liquidation::Liquidation,
	oracle::Oracle,
	undercollateralized_loans::{LoanConfig, MarketConfig, MarketInfo, UndercollateralizedLoans},
	vault::{Deposit, Vault, VaultConfig},
};
use frame_support::{
	ensure,
	traits::{
		fungible::{Inspect as NativeInspect, Transfer as NativeTransfer},
		fungibles::{Mutate, Transfer},
		Get, UnixTime,
	},
};
use sp_runtime::{
	traits::{One, Saturating, Zero},
	DispatchError, Perquintill,
};
use sp_std::vec::Vec;

impl<T: Config> Pallet<T> {
	//===========================================================================================================================
	//                                                   "Do" methods
	//===========================================================================================================================

	pub(crate) fn do_create_market(
		manager: T::AccountId,
		input: Validated<
			MarketInputOf<T>,
			MarketInputIsValid<T::Oracle, crate::Pallet<T>>,
		>,
		keep_alive: bool,
	) -> Result<MarketInfoOf<T>, DispatchError> {
		let config_input = input.value();
		crate::MarketsCounterStorage::<T>::try_mutate(|counter| {
			*counter += Counter::one();
			ensure!(*counter <= T::MaxMarketsCounterValue::get(), Error::<T>::MaxMarketsReached);
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
				!initial_pool_size.is_zero(),
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
	pub(crate) fn do_create_loan(
		input: Validated<LoanInputOf<T>, LoanInputIsValid<crate::Pallet<T>>>,
	) -> Result<LoanConfigOf<T>, DispatchError> {
		let config_input = input.value();
		// Get market config. Unwrapped since we have checked market existence during input
		// validation process.
		let market_config =
			Self::get_market_config_via_account_id(&config_input.market_account_id)?;
		// Align schedule timestamps to the beginning of the day.
		// 24.08.1991 08:45:03 -> 24.08.1991 00:00:00
		let schedule = config_input
			.payment_schedule
			.into_iter()
			.map(|(timestamp, balance)| (Self::get_date_aligned_timestamp(timestamp), balance))
			.collect();
		// Create non-activated loan and increment loans' counter.
		// This loan have to be activated by borrower further.
		crate::LoansCounterStorage::<T>::try_mutate(|counter| {
			*counter += Counter::one();
			crate::LoansPerMarketCounterStorage::<T>::try_mutate(
				market_config.account_id(),
				|loans_per_market_counter| {
					*loans_per_market_counter += Counter::one();
					ensure!(
						*loans_per_market_counter <= T::MaxLoansPerMarketCounterValue::get(),
						Error::<T>::MaxLoansPerMarketReached
					);
					let loan_account_id = Self::loan_account_id(*counter);
					let loan_config = LoanConfig::new(
						loan_account_id.clone(),
						config_input.market_account_id,
						config_input.borrower_account_id,
						market_config.collateral_asset_id().clone(),
						market_config.borrow_asset_id().clone(),
						config_input.principal,
						config_input.collateral,
						schedule,
					);
					crate::LoansStorage::<T>::insert(loan_account_id.clone(), loan_config.clone());
					crate::NonActiveLoansStorage::<T>::insert(loan_account_id, ());
					Ok(loan_config)
				},
			)
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
			Error::<T>::NonAuthorizedToExecuteContract,
		);
		// Check if borrower tries to activate expired loan.
		// Need this check since we remove expired loans only once a day.
		let today = Self::get_current_date();
		let first_payment_date = Self::get_date_from_timestamp(*loan_config.first_payment_moment());
		// Loan should be activated before the first payment date.
		if today >= first_payment_date {
			crate::NonActiveLoansStorage::<T>::remove(loan_account_id.clone());
			crate::LoansStorage::<T>::remove(loan_account_id.clone());
			Err(Error::<T>::LoanContractIsExpired)?;
		}
		// Transfer minimum amount of native asset from the borrower's account to the loan's account
		// to ensure loan account existence.
		T::NativeCurrency::transfer(
			&borrower_account_id,
			&loan_account_id,
			T::NativeCurrency::minimum_balance(),
			keep_alive,
		)?;
		// Transfer collateral from the borrower's account to the loan's account.
		let collateral_asset_id = *loan_config.collateral_asset_id();
		let source = &borrower_account_id;
		let destination = &loan_account_id;
		let amount = *loan_config.collateral();
		T::MultiCurrency::transfer(collateral_asset_id, source, destination, amount, keep_alive)?;
		// Transfer borrow assets from market's account to the borrower's account.
		let borrow_asset_id = *loan_config.borrow_asset_id();
		let source = loan_config.market_account_id();
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
		let borrow_asset_id = loan_config.borrow_asset_id();
		// Transfer 'amount' of assets from the payer account to the loan account
		T::MultiCurrency::transfer(
			*borrow_asset_id,
			&payer_account_id,
			&loan_account_id,
			repay_amount,
			keep_alive,
		)
	}

	// Process payments wich correctness were checked via off-chain procedures.
	pub(crate) fn do_process_checked_payments(possible_payments_outcomes: PaymentsOutcomes<T>) {
		for outcome in possible_payments_outcomes {
			match outcome {
				PaymentOutcome::RegularPaymentSucceed(payment) =>
					Self::process_checked_payment(&payment),
				PaymentOutcome::LastPaymentSucceed(payment) => {
					Self::process_checked_payment(&payment);
					Self::close_loan_contract(&payment.loan_config);
				},
				PaymentOutcome::PaymentFailed(payment) =>
					Self::process_failed_payment(&payment.loan_config),
			}
		}
	}

	//Removes a bunch of non-activated loans.
	//TODO: @mikolaichuk: add Event;
	pub(crate) fn do_remove_non_activated_loans(loans_accounts_ids: Vec<T::AccountId>) {
		loans_accounts_ids
			.into_iter()
			//Check that loan is non-activated.
			.filter(|loan_account_id| {
				crate::NonActiveLoansStorage::<T>::contains_key(loan_account_id)
			})
			.for_each(|loan_account_id| {
				crate::LoansStorage::<T>::remove(loan_account_id.clone());
				crate::NonActiveLoansStorage::<T>::remove(loan_account_id)
			});
	}

	//===========================================================================================================================
	//                                          General payments treatment
	//===========================================================================================================================

	// In off-chain context: checks that payment will be successful.
	// In on-chain context: really transfers assets from borrower's account to
	// market account.
	pub(crate) fn treat_payment(
		loan_config: &LoanConfigOf<T>,
		today: Timestamp,
	) -> Option<PaymentOutcomeOf<T>> {
		// If there is no such date in the local loan's schedule we return None.
		let payment_amount = loan_config.get_payment_for_particular_moment(&today)?;
		let payment =
			Payment { loan_config: loan_config.clone(), amount: payment_amount, timestamp: today };
		// Use to check if payment transfer is possible if off-chain context
		// In on-chain context try to transfer borrow asset from loan's account to market's account.
		// Please note that methods called within off-chain context do not change chain's state.
		let outcome = match Self::pay_back_borrowed_asset(&loan_config, payment_amount) {
			// We have enough money on the loan's account to perform last payment.
			Ok(_) if *loan_config.last_payment_moment() == today =>
				PaymentOutcome::LastPaymentSucceed(payment),
			// We have enough money on the loan's account to perform regular payment.
			Ok(_) => PaymentOutcome::RegularPaymentSucceed(payment),
			// TODO: @mikolaichuk:  we should give to borrower
			//                      several attempts.
			// Payment is failed.
			Err(_) => PaymentOutcome::PaymentFailed(payment),
		};
		Some(outcome)
	}

	// Process loans which payments were checked off-chain.
	// We may be sure that loans' accounts have enough money to pay.
	pub fn process_checked_payment(payment: &PaymentOf<T>) {
		let loan_account_id = payment.loan_config.account_id();
		// Check if the loan was already processed.
		// Allows to avoid double processing, which cause unreasonable liquidation.
		if crate::ProcessedLoansStorage::<T>::get().contains(loan_account_id) {
			return
		};
		// Get payment amount.
		let amount = match payment.loan_config.get_payment_for_particular_moment(&payment.timestamp)
		{
			Some(amount) => amount,
			None => {
				log::error!(
					"Payment moment does not exist in the loan's local payment schedule, loan account id: {:?}, moment: {:?}.",
					loan_account_id,
					payment.timestamp
				);
				return
			},
		};
		// We are sure that payment is succeed since it has been checked off-chain.
		// Nobody except the pallet can withdraw money from the loan's account, so there is no
		// chance that somebody withdraw money from loan's account after it was checked off-chain.
		Self::pay_back_borrowed_asset(&payment.loan_config, amount).unwrap_or(T::Balance::zero());
		// Mark processed loan to avoid double processing which can cause loan liquidation.
		crate::ProcessedLoansStorage::<T>::mutate(|set| set.insert(loan_account_id.clone()));
	}

	// Tansfers borrow asset from loan's account to market's account.
	pub(crate) fn pay_back_borrowed_asset(
		loan_config: &LoanConfigOf<T>,
		payment_amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		T::MultiCurrency::transfer(
			*loan_config.borrow_asset_id(),
			loan_config.account_id(),
			loan_config.market_account_id(),
			payment_amount,
			true,
		)
	}

	//===========================================================================================================================
	//                                          Failed payments treatment
	//===========================================================================================================================

	// Logic we apply if payment is failed.
	pub(crate) fn process_failed_payment(loan_config: &LoanConfigOf<T>) {
		// TODO: @mikolaichuk:  add credit rating functionality.
		//                      add event deposit(?)
		match Self::liquidate(loan_config) {
			Ok(_) => log::info!(
				"Loan with the following account id: {:?} was successfuly send to liquidation",
				loan_config.account_id()
			),
			Err(error) => log::error!(
				"Loan with the following account id: {:?} was not send to liquidation. Error: {:?}",
				loan_config.account_id(),
				error
			),
		};
	}

	// Send position to liquidation.
	pub(crate) fn liquidate(
		loan_config: &LoanConfigOf<T>,
	) -> Result<<T::Liquidation as Liquidation>::OrderId, DispatchError> {
		let liquidation_strategies =
			Self::get_market_info_via_account_id(loan_config.market_account_id())?
				.liquidation_strategies;
		let unit_price = T::Oracle::get_ratio(CurrencyPair::new(
			*loan_config.collateral_asset_id(),
			*loan_config.borrow_asset_id(),
		))?;
		let sell = Sell::new(
			*loan_config.collateral_asset_id(),
			*loan_config.borrow_asset_id(),
			*loan_config.collateral(),
			unit_price,
		);
		T::Liquidation::liquidate(loan_config.account_id(), sell, liquidation_strategies)
	}

	//===========================================================================================================================
	//                                              Close contracts procedure
	//===========================================================================================================================

	// Close loan contract since loan is paid.
	pub(crate) fn close_loan_contract(loan_config: &LoanConfigOf<T>) {
		// Transfer collateral to borrower's account.
		T::MultiCurrency::transfer(
			*loan_config.collateral_asset_id(),
			loan_config.market_account_id(),
			loan_config.borrower_account_id(),
			*loan_config.collateral(),
			true,
		)
	    // May happens if borrower's account died for some reason.
        // TODO: @mikolaichuk: what we are going to with such situations? 
        // Can we transfer collateral at market account in this case?
        // Perhaps we have to allow manager to transfer collateral to any account in such cases? 
        .map_or_else(|error| log::error!("Collateral was not transferred back to the borrower's account due to the following error: {:?}", error), |_| ());

		// Remove all information about the loan.
		Self::terminate_activated_loan(&loan_config);
		Self::deposit_event(crate::Event::<T>::LoanClosed { loan_config: loan_config.clone() });
	}

	// Remove all information regarding activated loan.
	pub(crate) fn terminate_activated_loan(loan_config: &LoanConfigOf<T>) {
		// Get payment moments for the loan.
		let payment_moments: Vec<Timestamp> =
			loan_config.schedule().keys().map(|key| key.clone()).collect();
		// Remove account id from the global payment schedule for each payment date.
		// Go through payment moments extracted from the local loan's payment schedule.
		// Use moments from the current date only.
		// TODO: @mikolaichuk: Test this.
		payment_moments
			.into_iter()
			.filter(|&moment| moment >= Self::get_current_date_timestamp())
			.for_each(|payment_moment| {
				crate::ScheduleStorage::<T>::mutate(payment_moment, |loans_accounts_ids| {
					loans_accounts_ids.remove(loan_config.account_id())
				});
			});
		// TODO: @mikolaichuk: What we suppose to do if borrower's account is died?
		T::NativeCurrency::transfer(
			loan_config.account_id(),
			loan_config.borrower_account_id(),
			T::NativeCurrency::minimum_balance(),
			false,
		)
		.map_or_else(|error| log::error!("Fee was not transferred back to the borrower's account due to the following error: {:?}", error), |_| ());
		Self::deposit_event(crate::Event::<T>::LoanTerminated { loan_config: loan_config.clone() })
	}

	//===========================================================================================================================
	//                                              Non-organized helpers
	//===========================================================================================================================

	// Check if borrower's account is whitelisted for particular market.
	pub(crate) fn is_borrower_account_whitelisted(
		borrower_account_id_ref: &T::AccountId,
		market_account_id_ref: &T::AccountId,
	) -> Result<bool, DispatchError> {
		let market_config = Self::get_market_config_via_account_id(market_account_id_ref)?;
		Ok(market_config.whitelist().contains(borrower_account_id_ref))
	}

	// Check if provided account id belongs to the market manager.
	pub(crate) fn is_market_manager_account(
		account_id: &T::AccountId,
		market_account_id_ref: &T::AccountId,
	) -> Result<bool, DispatchError> {
		let market_config = Self::get_market_config_via_account_id(market_account_id_ref)?;
		Ok(market_config.manager() == account_id)
	}

	pub(crate) fn calculate_initial_pool_size(
		borrow_asset: <T::Oracle as composable_traits::oracle::Oracle>::AssetId,
	) -> Result<<T as DeFiComposableConfig>::Balance, DispatchError> {
		T::Oracle::get_price_inverse(borrow_asset, T::OracleMarketCreationStake::get())
	}

	pub(crate) fn now() -> Timestamp {
		T::UnixTime::now().as_secs() as Timestamp
	}
}
