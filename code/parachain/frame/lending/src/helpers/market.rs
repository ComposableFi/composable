use crate::{
	validation::{
		AssetIsSupportedByOracle, CurrencyPairIsNotSame, MarketModelValid, UpdateInputValid,
	},
	*,
};
use composable_support::validation::Validated;
use composable_traits::{
	currency::CurrencyFactory,
	lending::{Lending, MarketConfig, UpdateInput},
	vault::{Deposit, Vault, VaultConfig},
};
use frame_support::{pallet_prelude::*, traits::fungibles::Transfer};
use sp_runtime::{
	traits::{One, Saturating, Zero},
	DispatchError, FixedU128, Perquintill,
};
use sp_std::{vec, vec::Vec};

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
		LendingCount::<T>::try_mutate(|MarketId(previous_market_index)| {
			let market_id = {
				// TODO: early mutation of `previous_market_index` value before check.
				*previous_market_index += 1;
				ensure!(
					*previous_market_index <= T::MaxMarketCount::get(),
					Error::<T>::ExceedLendingCount
				);
				MarketId(*previous_market_index)
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

			let initial_market_volume =
				Self::calculate_initial_market_volume(config_input.borrow_asset())?;

			ensure!(
				initial_market_volume > T::Balance::zero(),
				Error::<T>::InitialMarketVolumeIncorrect
			);

			// transfer `initial_market_volume` worth of borrow asset from the manager to the market
			T::MultiCurrency::transfer(
				config_input.borrow_asset(),
				&manager,
				&Self::account_id(&market_id),
				initial_market_volume,
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
				is_paused_functionalities: vec![false; 7],
			};
			let debt_token_id = T::CurrencyFactory::reserve_lp_token_id()?;

			DebtTokenForMarket::<T>::insert(market_id, debt_token_id);
			Markets::<T>::insert(market_id, market_config);
			BorrowIndex::<T>::insert(market_id, FixedU128::one());

			Ok((market_id, borrow_asset_vault))
		})
	}

	pub(crate) fn do_update_market(
		manager: T::AccountId,
		market_id: MarketId,
		input: Validated<
			UpdateInput<T::LiquidationStrategyId, <T as frame_system::Config>::BlockNumber>,
			UpdateInputValid,
		>,
	) -> Result<(), DispatchError> {
		let input = input.value();
		Markets::<T>::mutate(market_id, |market| {
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
		Ok(())
	}

	pub(crate) fn do_update_market_functionality(
		manager: T::AccountId,
		market_id: MarketId,
		changed_functionalities: Vec<(u8, bool)>,
	) -> Result<(), DispatchError> {
		Markets::<T>::mutate(market_id, |market| {
			if let Some(market) = market {
				ensure!(manager == market.manager, Error::<T>::Unauthorized);
				for (index, is_paused) in changed_functionalities {
					if (index as usize) < market.is_paused_functionalities.len() {
						if let Some(elem) = market.is_paused_functionalities.get_mut(index as usize)
						{
							*elem = is_paused
						};
					}
				}
				Ok(())
			} else {
				Err(Error::<T>::MarketDoesNotExist)
			}
		})?;
		Ok(())
	}

	pub(crate) fn get_functionality_index(functionality: Functionality) -> usize {
		match functionality {
			Functionality::DepositVault => 0,
			Functionality::WithdrawVault => 1,
			Functionality::DepositCollateral => 2,
			Functionality::WithdrawCollateral => 3,
			Functionality::Borrow => 4,
			Functionality::RepayBorrow => 5,
			Functionality::Liquidate => 6,
		}
	}

	pub(crate) fn functionality_allowed(
		market_id: &MarketId,
		functionality: Functionality,
	) -> Result<bool, DispatchError> {
		let (_, market) = Self::get_market(market_id)?;
		let index = Self::get_functionality_index(functionality);
		match market.is_paused_functionalities.get(index) {
			Some(val) => Ok(!val),
			None => Err(Error::<T>::FunctionalityNotAddedToMarket.into()),
		}
	}

	/// Returns pair of market's id and market (as 'MarketConfig') via market's id
	/// - `market_id` : Market index as a key in 'Markets' storage
	pub(crate) fn get_market(
		market_id: &MarketId,
	) -> Result<(&MarketId, MarketConfigOf<T>), DispatchError> {
		Markets::<T>::get(market_id)
			.map(|market| (market_id, market))
			.ok_or_else(|| Error::<T>::MarketDoesNotExist.into())
	}

	/// Returns the borrow and debt assets for the given market, if it exists.
	pub(crate) fn get_assets_for_market(
		market_id: &MarketId,
	) -> Result<MarketAssets<T>, DispatchError> {
		let (_, market) = Self::get_market(market_id)?;
		let borrow_asset = T::Vault::asset_id(&market.borrow_asset_vault)?;
		let debt_asset =
			DebtTokenForMarket::<T>::get(market_id).ok_or(Error::<T>::MarketDoesNotExist)?;

		Ok(MarketAssets { borrow_asset, debt_asset })
	}
}
