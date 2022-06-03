#![allow(unused_imports)]
use composable_support::validation2::{TryIntoValidated, Validate};
use composable_traits::{
	defi::{CurrencyPair, DeFiComposableConfig, DeFiEngine, MoreThanOneFixedU128},
	lending::{
		math::{
			CurveModel, DoubleExponentModel, DynamicPIDControllerModel, InterestRateModel,
			JumpModel,
		},
		CreateInput, UpdateInput,
	},
	oracle::Oracle as OracleTrait,
	time::Timestamp,
};
use frame_support::{pallet_prelude::*, sp_runtime::Perquintill, sp_std::vec::Vec};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{One, Zero},
	DispatchError, Percent,
};

use crate::{Config, Error};

// Here we use crate::Error
pub struct InterestRateModelIsValid<T>(PhantomData<T>);
impl<T: Config> Validate<InterestRateModel, InterestRateModelIsValid<T>, Error<T>>
	for InterestRateModelIsValid<T>
{
	fn validate(interest_rate_model: InterestRateModel) -> Result<InterestRateModel, Error<T>> {
		match interest_rate_model {
			InterestRateModel::Jump(x) =>
				JumpModel::new(x.base_rate, x.jump_rate, x.full_rate, x.target_utilization)
					.ok_or(Error::InterestRateModelIsNotValid)
					.map(InterestRateModel::Jump),
			InterestRateModel::Curve(x) => CurveModel::new(x.base_rate)
				.ok_or(Error::InterestRateModelIsNotValid)
				.map(InterestRateModel::Curve),
			InterestRateModel::DynamicPIDController(x) => DynamicPIDControllerModel::new(
				x.proportional_parameter,
				x.integral_parameter,
				x.derivative_parameter,
				x.previous_interest_rate,
				x.target_utilization,
			)
			.ok_or(Error::InterestRateModelIsNotValid)
			.map(InterestRateModel::DynamicPIDController),
			InterestRateModel::DoubleExponent(x) => DoubleExponentModel::new(x.coefficients)
				.ok_or(Error::InterestRateModelIsNotValid)
				.map(InterestRateModel::DoubleExponent),
		}
	}
}

// Here we use DispatchError since Oracle::is_supported throws this type of errors
#[derive(Clone, Copy, RuntimeDebug, PartialEq, TypeInfo, Default)]
pub struct AssetIsSupportedByOracle<T>(PhantomData<T>);
impl<LiquidationStrategyId, Asset: Copy, BlockNumber, T: Config + DeFiComposableConfig>
	Validate<
		CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
		AssetIsSupportedByOracle<T>,
		DispatchError,
	> for AssetIsSupportedByOracle<T>
where
	T: DeFiComposableConfig<MayBeAssetId = Asset>,
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, DispatchError> {
		ensure!(
			<T::Oracle as OracleTrait>::is_supported(create_input.borrow_asset())?,
			Error::<T>::BorrowAssetNotSupportedByOracle
		);
		ensure!(
			<T::Oracle as OracleTrait>::is_supported(create_input.collateral_asset())?,
			Error::<T>::CollateralAssetNotSupportedByOracle
		);
		Ok(create_input)
	}
}

// Here we can use crate::Error
#[derive(Clone, Copy, RuntimeDebug, PartialEq, TypeInfo, Default)]
pub struct UpdateInputValid<T>(PhantomData<T>);
impl<LiquidationStrategyId, BlockNumber, T: Config>
	Validate<UpdateInput<LiquidationStrategyId, BlockNumber>, UpdateInputValid<T>, Error<T>>
	for UpdateInputValid<T>
{
	fn validate(
		update_input: UpdateInput<LiquidationStrategyId, BlockNumber>,
	) -> Result<UpdateInput<LiquidationStrategyId, BlockNumber>, Error<T>> {
		if update_input.collateral_factor < MoreThanOneFixedU128::one() {
			return Err(Error::<T>::CollateralFactorMustBeMoreThanOne)
		}
		let interest_rate_model = update_input
			.interest_rate_model
			.try_into_validated::<InterestRateModelIsValid<T>>()?
			.value();
		Ok(UpdateInput { interest_rate_model, ..update_input })
	}
}

#[derive(Clone, Copy, RuntimeDebug, PartialEq, TypeInfo, Default)]
pub struct MarketModelValid<T>(PhantomData<T>);
#[derive(Clone, Copy, RuntimeDebug, PartialEq, TypeInfo, Default)]
pub struct CurrencyPairIsNotSame<T>(PhantomData<T>);
// DispatchError is used here since  AssetIsSupportedByOracl and MarketModeValid are going
// to be used together.
impl<LiquidationStrategyId, Asset: Eq, BlockNumber, T: Config>
	Validate<
		CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
		MarketModelValid<T>,
		DispatchError,
	> for MarketModelValid<T>
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, DispatchError> {
		let updatable = create_input.updatable.try_into_validated::<UpdateInputValid<T>>()?.value();

		Ok(CreateInput { updatable, ..create_input })
	}
}
// DispatchError is used here since  AssetIsSupportedByOracl and  CurrencyPairIsNotSame are going
// to be used together.
impl<LiquidationStrategyId, Asset: Eq, BlockNumber, T: Config>
	Validate<
		CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
		CurrencyPairIsNotSame<T>,
		DispatchError,
	> for CurrencyPairIsNotSame<T>
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, DispatchError> {
		if create_input.currency_pair.base == create_input.currency_pair.quote {
			Err(Error::<T>::BaseAndQuoteCurrenciesInPairAreSame.into())
		} else {
			Ok(create_input)
		}
	}
}
