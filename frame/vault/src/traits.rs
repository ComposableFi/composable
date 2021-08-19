//! Traits on which this pallet relies

use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Debug, PartialEq)]
pub enum FundsAvailability<Balance> {
    Withdrawable(Balance),
    MustLiquidate(Balance),
}

pub trait CurrencyFactory<CurrencyId> {
    fn create() -> Result<CurrencyId, DispatchError>;
}

pub trait StrategicVault {
    type AccountId;
    type Balance;
    type Error;

    fn available_funds(
        account: &Self::AccountId,
    ) -> Result<FundsAvailability<Self::Balance>, Self::Error>;

    fn withdraw(
        account: &Self::AccountId,
        value: Self::Balance,
    ) -> Result<Self::Balance, Self::Error>;

    fn deposit(
        account: &Self::AccountId,
        value: Self::Balance,
    ) -> Result<Self::Balance, Self::Error>;
}

pub trait ReportableStrategicVault: StrategicVault {
    type StrategyReport;

    fn update_strategy_report(strategy_report: &Self::StrategyReport) -> Result<(), Self::Error>;
}
