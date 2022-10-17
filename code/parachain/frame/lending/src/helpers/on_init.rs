use crate::{types::InitializeBlockCallCounters, *};
use composable_traits::{
	lending::Lending,
	vault::{FundsAvailability, StrategicVault, Vault},
};
use frame_support::{
	storage::{with_transaction, TransactionOutcome},
	traits::{fungibles::Inspect, UnixTime},
};
use sp_runtime::DispatchError;

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
					// It would probably be more performant to handle theses
					// case while borrowing/repaying.
					//
					// I don't know whether we would face any issue by doing that.
					//
					// borrow:
					//  - withdrawable = transfer(vault->market) + transfer(market->user)
					//  - depositable = error(not enough borrow asset) // vault asking for reserve
					//    to be fulfilled
					//  - must liquidate = error(market is closing)
					// repay:
					// 	- (withdrawable || depositable || must liquidate) = transfer(user->market) +
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
						FundsAvailability::None => {},
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
}
