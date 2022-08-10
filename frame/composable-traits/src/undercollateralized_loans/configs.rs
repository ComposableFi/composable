use crate::defi::CurrencyPair;
use frame_support::pallet_prelude::*;
use sp_runtime::Perquintill;
use sp_std::{
	collections::{btree_map::BTreeMap, btree_set::BTreeSet},
	vec::Vec,
};

#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct MarketConfig<AccountId, AssetId, BlockNumber, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	/// The account id of this market
	account_id: AccountId,
	/// The owner of this market.
	manager: AccountId,
	/// The vault containing the borrow asset.
	borrow_asset_vault: VaultId,
	/// The asset being used as collateral.
	collateral_asset_id: AssetId,
	/// The asset being used as borrow asset.
	borrow_asset_id: AssetId,
	/// Number of blocks until invalidate oracle's price.
	max_price_age: BlockNumber,
	/// Borrowers which are allowed to use the service.
	whitelist: BTreeSet<AccountId>,
}

impl<AccountId, AssetId, BlockNumber, VaultId>
	MarketConfig<AccountId, AssetId, BlockNumber, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	pub fn new(
		account_id: AccountId,
		manager: AccountId,
		borrow_asset_vault: VaultId,
		borrow_asset_id: AssetId,
		collateral_asset_id: AssetId,
		max_price_age: BlockNumber,
		whitelist: BTreeSet<AccountId>,
	) -> Self {
		Self {
			account_id,
			manager,
			borrow_asset_vault,
			borrow_asset_id,
			collateral_asset_id,
			max_price_age,
			whitelist,
		}
	}

	/// Get a reference to the market config's account id.
	pub fn account_id(&self) -> &AccountId {
		&self.account_id
	}

	/// Get a reference to the market config's manager.
	pub fn manager(&self) -> &AccountId {
		&self.manager
	}
	/// Get a reference to the market config's borrow asset vault.
	pub fn borrow_asset_vault(&self) -> &VaultId {
		&self.borrow_asset_vault
	}

	/// Get a reference to the market config's borrow asset.
	pub fn borrow_asset_id(&self) -> &AssetId {
		&self.borrow_asset_id
	}

	/// Get a reference to the market config's collateral asset.
	pub fn collateral_asset_id(&self) -> &AssetId {
		&self.collateral_asset_id
	}

	/// Get a reference to the market config's max price age.
	pub fn max_price_age(&self) -> &BlockNumber {
		&self.max_price_age
	}

	/// Get a reference to the market config's whitelist.
	#[must_use]
	pub fn whitelist(&self) -> &BTreeSet<AccountId> {
		&self.whitelist
	}
}

#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct LoanConfig<AccountId, AssetId, Balance, Timestamp>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	Timestamp: Clone + Eq + PartialEq,
{
	/// Loan account id.
	account_id: AccountId,
	/// Market account id.
	market_account_id: AccountId,
	/// Borrower account id.
	/// Should be whitelisted.
	borrower_account_id: AccountId,
	/// The asset being used as collateral.
	collateral_asset_id: AssetId,
	/// The asset being used as borrow asset.
	borrow_asset_id: AssetId,
	/// Amount of borrowed money.  
	principal: Balance,
	/// Amount of assets which should be putted as collateral.
	collateral: Balance,
	/// Schedule of payments
	schedule: BTreeMap<Timestamp, Balance>,
	/// The moment of the first interest payment.
	first_payment_moment: Timestamp,
	/// The moment of the last interest payment and principal repayment.
	last_payment_moment: Timestamp,
}

impl<AccountId, AssetId, Balance, Timestamp> LoanConfig<AccountId, AssetId, Balance, Timestamp>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	Timestamp: Clone + Eq + PartialEq + Ord,
{
	pub fn new(
		account_id: AccountId,
		market_account_id: AccountId,
		borrower_account_id: AccountId,
		collateral_asset_id: AssetId,
		borrow_asset_id: AssetId,
		principal: Balance,
		collateral: Balance,
		schedule: Vec<(Timestamp, Balance)>,
	) -> Self {
		let schedule: BTreeMap<Timestamp, Balance> = schedule.into_iter().collect();
		// We are sure thate BTreeMap is not empty
		// TODO: @mikolaichuk: May be it would be better to use BiBoundedVec as input here.
		let first_payment_moment = schedule.keys().min().unwrap().clone();
		let last_payment_moment = schedule.keys().max().unwrap().clone();
		Self {
			account_id,
			market_account_id,
			borrower_account_id,
			collateral_asset_id,
			borrow_asset_id,
			principal,
			collateral,
			schedule,
			first_payment_moment,
			last_payment_moment,
		}
	}

	/// Get a reference to the loan config's account id.
	pub fn account_id(&self) -> &AccountId {
		&self.account_id
	}

	/// Get a reference to the loan config's market account id.
	pub fn market_account_id(&self) -> &AccountId {
		&self.market_account_id
	}

	/// Get a reference to the loan config's borrower account id.
	pub fn borrower_account_id(&self) -> &AccountId {
		&self.borrower_account_id
	}

	pub fn collateral_asset_id(&self) -> &AssetId {
		&self.collateral_asset_id
	}

	pub fn borrow_asset_id(&self) -> &AssetId {
		&self.borrow_asset_id
	}

	/// Get a reference to the loan config's principal.
	pub fn principal(&self) -> &Balance {
		&self.principal
	}

	/// Get a reference to the loan config's collateral.
	pub fn collateral(&self) -> &Balance {
		&self.collateral
	}

	/// Get a reference to the loan payment schedule.
	pub fn schedule(&self) -> &BTreeMap<Timestamp, Balance> {
		&self.schedule
	}

	/// Get a reference to the loan first payment moment.
	pub fn first_payment_moment(&self) -> &Timestamp {
		&self.first_payment_moment
	}

	/// Get a reference to the loan last payment moment.
	pub fn last_payment_moment(&self) -> &Timestamp {
		&self.last_payment_moment
	}

	pub fn get_payment_for_particular_moment(&self, moment: &Timestamp) -> Option<&Balance> {
		self.schedule.get(moment)
	}
}

// some fields are hiden since they should be immutable
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct MarketInfo<AccountId, AssetId, BlockNumber, LiquidationStrategyId, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	LiquidationStrategyId: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	config: MarketConfig<AccountId, AssetId, BlockNumber, VaultId>,
	pub liquidation_strategies: Vec<LiquidationStrategyId>,
}

impl<AccountId, AssetId, BlockNumber, LiquidationStrategyId, VaultId>
	MarketInfo<AccountId, AssetId, BlockNumber, LiquidationStrategyId, VaultId>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	LiquidationStrategyId: Clone + Eq + PartialEq,
	VaultId: Clone + Eq + PartialEq,
{
	pub fn new(
		config: MarketConfig<AccountId, AssetId, BlockNumber, VaultId>,
		liquidation_strategies: Vec<LiquidationStrategyId>,
	) -> Self {
		Self { config, liquidation_strategies }
	}

	/// Get a reference to the market info's config.
	pub fn config(&self) -> &MarketConfig<AccountId, AssetId, BlockNumber, VaultId> {
		&self.config
	}

	/// Get a reference to the market info's liquidation strategies.
	pub fn liquidation_strategies(&self) -> &Vec<LiquidationStrategyId> {
		&self.liquidation_strategies
	}
}

/// input to create market extrinsic
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub struct MarketInput<AccountId, AssetId, BlockNumber, LiquidationStrategyId> {
	/// Collateral currency and borrow currency.
	pub currency_pair: CurrencyPair<AssetId>,
	/// Reserve factor of market borrow vault.
	pub reserved_factor: Perquintill,
	/// List of trusted borrowers
	pub whitelist: BTreeSet<AccountId>,
	/// Liquidation engine id.
	pub liquidation_strategies: Vec<LiquidationStrategyId>,
	/// Count of blocks until throw error PriceIsTooOld.
	pub max_price_age: BlockNumber,
}

impl<AccountId, AssetId: Copy, BlockNumber, LiquidationStrategyId>
	MarketInput<AccountId, AssetId, BlockNumber, LiquidationStrategyId>
{
	pub fn borrow_asset(&self) -> AssetId {
		self.currency_pair.quote
	}
	pub fn collateral_asset(&self) -> AssetId {
		self.currency_pair.base
	}
	pub fn reserved_factor(&self) -> Perquintill {
		self.reserved_factor
	}
}

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, RuntimeDebug)]
pub struct LoanInput<AccountId, Balance, Timestamp> {
	/// Loan belongs to this market.
	pub market_account_id: AccountId,
	/// This account id have to be whitelisted.
	pub borrower_account_id: AccountId,
	/// Amount of borrowed money.  
	pub principal: Balance,
	/// Amount of assets which should be deposited as collateral.
	pub collateral: Balance,
	/// How often borrowers have to pay interest.
	pub payment_schedule: Vec<(Timestamp, Balance)>,
}
