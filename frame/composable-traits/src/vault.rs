use codec::Codec;
use frame_support::{
	pallet_prelude::*,
	sp_runtime::Perquintill,
	sp_std::{collections::btree_map::BTreeMap, fmt::Debug},
};
use scale_info::TypeInfo;

use crate::defi::Rate;

/// An indication for strategies as to how they should be rebalancing. Strategies should evaluate if
/// it is worth it to deposit or withdraw based on fees.
#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq, Eq, TypeInfo)]
pub enum FundsAvailability<Balance> {
	/// Withdrawable balance in the vault, which the strategy may use.
	Withdrawable(Balance),
	/// Depositable balance, such as earnings from strategies, or due to rebalancing. A strategy
	/// should evaluate the magnitude of the depositable balance before returning funds to minimize
	/// losses to fees.
	Depositable(Balance),
	/// Orders the strategy to liquidate, no matter the cost or the fees associated. Usually
	/// indicates that a strategy is being terminated or a vault is being destroyed.
	/// Example, the strategy was removed by the fund manager or governance, so all funds should
	/// be returned.
	MustLiquidate,
	/// When there are no balance that can be withdrawn or deposit and don't need to be liquidated.
	None,
}

#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum Deposit<Balance, BlockNumber> {
	/// Indicates that the vault has deposited an amount large enough to forever be exempt from
	/// rent payments.
	Existential,
	Rent {
		/// The amount left in the deposit.
		amount: Balance,
		/// The last block number at which payment was performed.
		at: BlockNumber,
	},
}

impl<Balance, BlockNumber> Default for Deposit<Balance, BlockNumber> {
	fn default() -> Self {
		Deposit::Existential
	}
}

impl<Balance, BlockNumber> Deposit<Balance, BlockNumber> {
	pub fn is_existential(&self) -> bool {
		matches!(self, Deposit::Existential)
	}
}

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, Eq, TypeInfo)]
pub struct VaultConfig<AccountId, CurrencyId>
where
	AccountId: core::cmp::Ord,
{
	pub asset_id: CurrencyId,
	/// Idle amount of assets for withdraw
	pub reserved: Perquintill,
	pub manager: AccountId,
	/// Not the vault strategy per si but rather a pool of funds that are used for strategies
	pub strategies: BTreeMap<AccountId, Perquintill>,
}

pub trait Vault {
	type AccountId: core::cmp::Ord;
	type AssetId;
	type Balance;
	type BlockNumber;
	type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;

	fn token_vault(token: Self::AssetId) -> Result<Self::VaultId, DispatchError>;

	/// underlying asset id
	fn asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError>;

	/// asset issues for underlying `asset_id`
	fn lp_asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError>;

	fn account_id(vault: &Self::VaultId) -> Self::AccountId;

	/// creates new vault for assets
	fn create(
		deposit: Deposit<Self::Balance, Self::BlockNumber>,
		config: VaultConfig<Self::AccountId, Self::AssetId>,
	) -> Result<Self::VaultId, DispatchError>;

	/// Used by users to deposit tokens.
	/// Returns true amount of wrapper token minted to user.
	fn deposit(
		vault_id: &Self::VaultId,
		from: &Self::AccountId,
		asset_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	fn withdraw(
		vault_id: &Self::VaultId,
		to: &Self::AccountId,
		lp_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	/// Return the current rate representing the stock dilution of the vault.
	fn stock_dilution_rate(vault_id: &Self::VaultId) -> Result<Rate, DispatchError>;

	fn calculate_lp_tokens_to_mint(
		vault_id: &Self::VaultId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	fn lp_share_value(
		vault_id: &Self::VaultId,
		lp_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;

	fn amount_of_lp_token_for_added_liquidity(
		vault_id: &Self::VaultId,
		asset_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;
}

/// CapabilityVault exposes functionalities for stopping and limiting vault functionality.
///
/// # Terminology
///  - `Tombstoning`: marks the vault as ready to be deleted, stopping almost all functionalities,
///    such as withdrawals and deposits. Empty vaults which pay no rent may be tombstoned.
///  - `Stopping`: Used as an emergency feature to stop all functionality, akin to tombstoning but
///    not marking it ready for deletion. Consider limiting this to sudo or a multisig council
///    account.
pub trait CapabilityVault: Vault {
	/// Stops all functionality of the vault. Call [`start`](CapabilityVault::start) to re-enable
	/// the vault.
	fn stop(vault_id: &Self::VaultId) -> DispatchResult;
	/// Indicates if the vault has been stopped. Stopped vaults have all functionality stopped,
	/// except for [`start`](CapabilityVault::start)ing.
	fn is_stopped(vault_id: &Self::VaultId) -> Result<bool, DispatchError>;
	/// Starts the vault after [`stop`](CapabilityVault::stop) was called.
	fn start(vault_id: &Self::VaultId) -> DispatchResult;
	/// Marks the vault as ready to be removed. Most functionality is stopped, except for
	/// withdrawals.
	fn tombstone(vault_id: &Self::VaultId) -> DispatchResult;
	/// Removes the tombstone mark.
	fn untombstone(vault_id: &Self::VaultId) -> DispatchResult;
	/// Indicates if the vault is tombstoned. Tombstoned vaults will be deleted after funds are
	/// returned, unless the rent is topped up.
	fn is_tombstoned(vault_id: &Self::VaultId) -> Result<bool, DispatchError>;
	/// Stops withdrawals but allows deposits and other functionalities.
	fn stop_withdrawals(vault_id: &Self::VaultId) -> DispatchResult;
	/// Allows withdrawals. If the vault is stopped, withdrawals remain blocked.
	fn allow_withdrawals(vault_id: &Self::VaultId) -> DispatchResult;
	/// Indicates if the vault is allowing withdrawals. If the vault is either stopped, or if
	/// withdrawals are disabled, this returns `false`.
	fn withdrawals_allowed(vault_id: &Self::VaultId) -> Result<bool, DispatchError>;
	/// Stops deposits but allows withdrawals and other functionalities.
	fn stop_deposits(vault_id: &Self::VaultId) -> DispatchResult;
	/// Allows withdrawals. If the vault is stopped or tombstoned, withdrawals remain blocked.
	fn allow_deposits(vault_id: &Self::VaultId) -> DispatchResult;
	/// Indicates if the vault is allowing deposits. If the vault is stopped, tombstoned or if
	/// withdrawals are disabled, this returns `false`.
	fn deposits_allowed(vault_id: &Self::VaultId) -> Result<bool, DispatchError>;
}

/// A vault which can be used by different strategies, such as pallets and smart contracts, to
/// efficiently use capital. An example may be a vault which allocates 40% in a lending protocol,
/// and 60% of the stored capital in a DEX.
pub trait StrategicVault: Vault {
	/// Used by strategies to query for available funds.
	fn available_funds(
		vault: &Self::VaultId,
		account: &Self::AccountId,
	) -> Result<FundsAvailability<Self::Balance>, DispatchError>;

	/// Used by strategies to withdraw funds to be used in DeFi or other protocols.
	/// Even if vault want its funds back, it is up to strategy to decide to go above available
	/// funds. At least there is such possibility.
	/// In most cases default behavior is to check `available_funds` before `withdraw`
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

/// A vault which allow the strategy to do periodic report.
pub trait ReportableStrategicVault: StrategicVault {
	type Report;

	fn update_strategy_report(
		vault: &Self::VaultId,
		strategy: &Self::AccountId,
		report: &Self::Report,
	) -> Result<(), DispatchError>;
}
