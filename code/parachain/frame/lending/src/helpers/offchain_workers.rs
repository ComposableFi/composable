pub use crate::types::{MarketId, MarketIdInner};
use crate::*;
use frame_support::pallet_prelude::*;
use frame_system::offchain::{SendSignedTransaction, Signer};
use sp_std::vec;

impl<T: Config> Pallet<T> {
	pub(crate) fn do_offchain_worker(_block_number: T::BlockNumber) {
		let signer = Signer::<T, <T as Config>::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			log::warn!("No signer");
			return
		}
		for (market_id, account, _) in DebtIndex::<T>::iter() {
			//Check that it should liquidate before liquidations
			let should_be_liquidated = match Self::should_liquidate(&market_id, &account) {
				Ok(status) => status,
				Err(error) => {
					log::error!(
						"Liquidation necessity check failed, market_id: {:?}, account: {:?},
									error: {:?}",
						market_id,
						account,
						error
					);
					false
				},
			};
			if !should_be_liquidated {
				continue
			}
			let results = signer.send_signed_transaction(|_account| Call::liquidate {
				market_id,
				// Unwrapped since we push only one borrower in the vector
				borrowers: BoundedVec::<_, T::MaxLiquidationBatchSize>::try_from(vec![
					account.clone()
				])
				.expect("This function never panics"),
			});

			for (_acc, res) in &results {
				match res {
					Ok(()) => log::info!(
						"Liquidation succeed, market_id: {:?}, account: {:?}",
						market_id,
						account
					),
					Err(e) => log::error!(
						"Liquidation failed, market_id: {:?}, account: {:?}, error: {:?}",
						market_id,
						account,
						e
					),
				}
			}
		}
	}
}
