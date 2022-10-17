use crate::*;
use composable_traits::{
	defi::DeFiComposableConfig, lending::BorrowAmountOf, oracle::Oracle, vault::Vault,
};
use frame_support::pallet_prelude::*;
use sp_runtime::DispatchError;

impl<T: Config> Pallet<T> {
	/// Get TWAP from oracle. If history of prices is empty then return latest price.
	pub(crate) fn get_price(
		asset_id: <T as DeFiComposableConfig>::MayBeAssetId,
		amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		<T::Oracle as Oracle>::get_twap_for_amount(asset_id, amount)
	}

	/// Check if price actual yet
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

	/// Returns the initial pool size for a market with `borrow_asset`. Calculated with
	/// [`Config::OracleMarketCreationStake`].
	pub(crate) fn calculate_initial_market_volume(
		borrow_asset: <T::Oracle as composable_traits::oracle::Oracle>::AssetId,
	) -> Result<<T as composable_traits::defi::DeFiComposableConfig>::Balance, DispatchError> {
		T::Oracle::get_price_inverse(borrow_asset, T::OracleMarketCreationStake::get())
	}
}
