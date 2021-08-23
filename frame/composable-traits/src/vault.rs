use frame_support::pallet_prelude::*;
use codec::Codec;
use frame_support::sp_std::fmt::Debug;

/// An indication for strategies as to how they should be rebalancing. Strategies should evaluate if
/// it is worth it to deposit or withdraw based on fees.
#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq)]
pub enum FundsAvailability<Balance> {
    /// Withdrawable balance in the vault, which the strategy may use.
    Withdrawable(Balance),
    /// Depositable balance, such as earnings from strategies, or due to rebalancing. A strategy
    /// should evaluate the magnitude of the depositable balance before returning funds to minimize
    /// losses to fees.
    Depositable(Balance),
    /// Orders the strategy to liquidate, no matter the cost or the fees associated. Usually indicates
    /// that a strategy is being terminated or a vault is being destroyed.
    MustLiquidate,
}

pub trait Vault {
    type AccountId;
    type Balance;
    type VaultId: Clone + Codec + Debug + PartialEq;
    type AssetId;

    fn asset_id(vault: &Self::VaultId) -> Self::AssetId;

    fn account_id() -> Self::AccountId;

    fn deposit(
        vault: &Self::VaultId,
        account: &Self::AccountId,
        balance: &Self::Balance,
    ) -> DispatchResult;

    fn withdraw(
        vault: &Self::VaultId,
        account: &Self::AccountId,
        balance: &Self::Balance,
    ) -> DispatchResult;
}

pub trait LpTokenVault {
    type AssetId;

    fn lp_asset_id() -> Self::AssetId;
}

/// A vault which can be used by different strategies, such as pallets and smart contracts, to
/// efficiently use capital. An example may be a vault which allocates 40% in a lending protocol, and
/// 60% of the stored capital in a DEX.
pub trait StrategicVault: Vault {
    /// Used by strategies to query for available funds.
    fn available_funds(
        vault: &Self::VaultId,
        account: &Self::AccountId,
    ) -> Result<FundsAvailability<Self::Balance>, DispatchError>;

    /// Used by strategies to withdraw funds to be used in DeFi or other protocols.
    fn withdraw(
        vault: &Self::VaultId,
        to: &Self::AccountId,
        amount: Self::Balance,
    ) -> Result<(), DispatchError>;

    /// Used by strategies to return profits and funds.
    fn deposit(
        vault: &Self::VaultId,
        from: &Self::AccountId,
        amount: Self::Balance,
    ) -> Result<(), DispatchError>;
}

///
pub trait ReportableStrategicVault: StrategicVault {
    type Report;

    fn update_strategy_report(
        vault: &Self::VaultId,
        strategy: &Self::AccountId,
        report: &Self::Report,
    ) -> Result<(), DispatchError>;
}
