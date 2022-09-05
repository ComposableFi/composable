use composable_support::validation::{TryIntoValidated, Validate};
use composable_traits::{
	defi::MoreThanOneFixedU128,
	lending::{math::InterestRateModelIsValid, CreateInput, UpdateInput},
	oracle::Oracle as OracleTrait,
};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::{One, Zero};

#[derive(Clone, Copy, RuntimeDebug, PartialEq, Eq, TypeInfo, Default)]
pub struct UpdateInputValid;

impl<LiquidationStrategyId, BlockNumber>
	Validate<UpdateInput<LiquidationStrategyId, BlockNumber>, UpdateInputValid> for UpdateInputValid
{
	fn validate(
		update_input: UpdateInput<LiquidationStrategyId, BlockNumber>,
	) -> Result<UpdateInput<LiquidationStrategyId, BlockNumber>, &'static str> {
		if update_input.collateral_factor < MoreThanOneFixedU128::one() {
			return Err("Collateral factor must be more than one.")
		}

		Ok(update_input)
	}
}

#[derive(Clone, Copy, RuntimeDebug, PartialEq, Eq, TypeInfo, Default)]
pub struct MarketModelValid;
#[derive(Clone, Copy, RuntimeDebug, PartialEq, Eq, TypeInfo, Default)]
pub struct CurrencyPairIsNotSame;

impl<LiquidationStrategyId, Asset: Eq, BlockNumber>
	Validate<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, MarketModelValid>
	for MarketModelValid
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, &'static str> {
		let updatable = create_input.updatable.try_into_validated::<UpdateInputValid>()?.value();
		let interest_rate_model = create_input
			.interest_rate_model
			.try_into_validated::<InterestRateModelIsValid>()?
			.value();

		Ok(CreateInput { updatable, interest_rate_model, ..create_input })
	}
}

impl<LiquidationStrategyId, Asset: Eq, BlockNumber>
	Validate<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, CurrencyPairIsNotSame>
	for CurrencyPairIsNotSame
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, &'static str> {
		if create_input.currency_pair.base == create_input.currency_pair.quote {
			Err("Base and quote currencies supposed to be different in currency pair")
		} else {
			Ok(create_input)
		}
	}
}

#[derive(RuntimeDebug, PartialEq, Eq, TypeInfo, Default, Clone, Copy)]
pub struct AssetIsSupportedByOracle<Oracle: OracleTrait>(PhantomData<Oracle>);

impl<LiquidationStrategyId, Asset: Copy, BlockNumber, Oracle: OracleTrait<AssetId = Asset>>
	Validate<
		CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
		AssetIsSupportedByOracle<Oracle>,
	> for AssetIsSupportedByOracle<Oracle>
{
	fn validate(
		create_input: CreateInput<LiquidationStrategyId, Asset, BlockNumber>,
	) -> Result<CreateInput<LiquidationStrategyId, Asset, BlockNumber>, &'static str> {
		ensure!(
			Oracle::is_supported(create_input.borrow_asset())?,
			"Borrow asset is not supported by oracle"
		);
		ensure!(
			Oracle::is_supported(create_input.collateral_asset())?,
			"Collateral asset is not supported by oracle"
		);
		Ok(create_input)
	}
}

#[derive(RuntimeDebug, PartialEq, Eq, TypeInfo, Default, Copy, Clone)]
pub struct BalanceGreaterThenZero;
impl<B> Validate<B, BalanceGreaterThenZero> for BalanceGreaterThenZero
where
	B: Zero + PartialOrd,
{
	fn validate(balance: B) -> Result<B, &'static str> {
		ensure!(balance > B::zero(), "Can not deposit or withdraw zero collateral");

		Ok(balance)
	}
}
