use codec::Codec;
use frame_support::{
	pallet_prelude::*,
	sp_runtime::Perquintill,
	sp_std::{collections::btree_map::BTreeMap, fmt::Debug},
};

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
	/// Orders the strategy to liquidate, no matter the cost or the fees associated. Usually
	/// indicates that a strategy is being terminated or a vault is being destroyed.
	MustLiquidate,
}

#[derive(Copy, Clone, Encode, Decode, Debug, PartialEq)]
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

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq)]
pub struct VaultConfig<AccountId, CurrencyId>
where
	AccountId: core::cmp::Ord,
{
	pub asset_id: CurrencyId,
	pub reserved: Perquintill,
	pub manager: AccountId,
	pub strategies: BTreeMap<AccountId, Perquintill>,
}

pub trait Vault {
	type AccountId: core::cmp::Ord;
	type AssetId;
	type Balance;
	type BlockNumber;
	type VaultId: Clone + Codec + Debug + PartialEq + Default + Parameter;

	fn asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError>;

	fn lp_asset_id(vault_id: &Self::VaultId) -> Result<Self::AssetId, DispatchError>;

	fn account_id(vault: &Self::VaultId) -> Self::AccountId;

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

	/// Convert from % share represented by LPs to underlying asset value
	/// Because of stock dilution, the LPs minted are not 1:1 with deposit
	fn lp_share_value(
		vault_id: &Self::VaultId,
		lp_amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError>;
}

pub trait LpTokenVault {
	type AssetId;

	fn lp_asset_id() -> Self::AssetId;
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
