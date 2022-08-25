use crate::{
	types::{MarketConfigOf, PaymentOutcome, Timestamp},
	Config, Pallet,
};
use composable_traits::vault::{FundsAvailability, StrategicVault, Vault};
use frame_support::{storage::with_transaction, traits::fungibles::Inspect};
use sp_runtime::{DispatchError, TransactionOutcome};
use sp_std::collections::btree_set::BTreeSet;

impl<T: Config> Pallet<T> {
	//===========================================================================================================================
	//                                                   Vault balancing
	//===========================================================================================================================

	// TODO: @mikolaichuk: Add weights calculation
	// #generalization
	pub(crate) fn treat_vaults_balance(block_number: T::BlockNumber) -> () {
		let _ = with_transaction(|| {
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

	//===========================================================================================================================
	//                                         EOD last chance payments processing
	//===========================================================================================================================

	// Collects accounts ids of unprocessed loans and process them.
	// Used at the beginning of the day to proccess loans which
	// were not processed yesterday for some reason.
	pub(crate) fn last_chance_processing(date: Timestamp) {
		let set_of_loans_accounts_ids: BTreeSet<T::AccountId> =
			crate::ScheduleStorage::<T>::get(date).keys().cloned().collect();
		let unprocessed_loans_accounts_ids: Vec<_> = set_of_loans_accounts_ids
			.difference(&crate::ProcessedLoansStorage::<T>::get())
			.cloned()
			.collect();
		if !unprocessed_loans_accounts_ids.is_empty() {
			Self::process_unchecked_payments(unprocessed_loans_accounts_ids, date);
		};
	}

	// Process payments which for some reason were not processed EOD.
	pub(crate) fn process_unchecked_payments(
		loans_accounts_ids: Vec<T::AccountId>,
		date: Timestamp,
	) {
		for loan_account_id in loans_accounts_ids {
			let loan_info = match Self::get_loan_info_via_account_id(&loan_account_id) {
				Ok(loan_info) => loan_info,
				Err(error) => {
					log::error!("Error: {:?}", error);
					continue
				},
			};
			let payment = match Self::treat_payment(&loan_info, date) {
				Some(payment) => payment,
				None => continue,
			};
			match payment {
				PaymentOutcome::RegularPaymentSucceed(_) => (),
				PaymentOutcome::LastPaymentSucceed(payment) =>
					Self::close_loan_contract(payment.loan_info.config()),
				PaymentOutcome::PaymentDelayed(payment) =>
					Self::process_delayed_payment_logged(&payment),
			}
		}
	}
}
