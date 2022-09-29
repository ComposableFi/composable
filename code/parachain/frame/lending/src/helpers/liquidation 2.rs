use crate::*;
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, DeFiEngine, Sell},
	lending::Lending,
	liquidation::Liquidation,
	oracle::Oracle,
	vault::Vault,
};
use frame_support::{
	pallet_prelude::*,
	storage::{with_transaction, TransactionOutcome},
	traits::fungible::Transfer as NativeTransfer,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

impl<T: Config> Pallet<T> {
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
	pub fn do_liquidate(
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
