use frame_support::pallet_prelude::*;
use sp_runtime::Perquintill;
use sp_std::{
	collections::{btree_map::BTreeMap, btree_set::BTreeSet},
	vec::Vec,
};

/// MarketConfig read-only structure is used to hold immutable properties of market.
/// Once market is created these properties should not be changed.
/// Changing of one of these fields may cause significant changes in market's performance
/// or even its dysfunction.
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
	pub fn whitelist(&self) -> &BTreeSet<AccountId> {
		&self.whitelist
	}
}

/// LoanConfig read-only structure is used to hold immutable properties of loan.
/// Once loan is created these properties should not be changed.
/// Changing of one of these fields means changing in the contract's terms after it has been
/// "signed".
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
	/// The asset being used for borrows.
	borrow_asset_id: AssetId,
	/// Amount of borrowed money.  
	principal: Balance,
	/// Amount of assets which should be used as collateral.
	collateral: Balance,
	/// Schedule of payments.
	schedule: BTreeMap<Timestamp, Balance>,
	/// Contract should be activated before this moment.
	activation_date: Timestamp,
	/// If this filed is None, borrower has to face his obligation in time.
	/// Otherwise borrower has possibility to postpone payment as per structure properties.
	delayed_payment_treatment: Option<DelayedPaymentTreatment>,
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
		schedule: BTreeMap<Timestamp, Balance>,
		activation_date: Timestamp,
		delayed_payment_treatment: Option<DelayedPaymentTreatment>,
	) -> Self {
		Self {
			account_id,
			market_account_id,
			borrower_account_id,
			collateral_asset_id,
			borrow_asset_id,
			principal,
			collateral,
			schedule,
			activation_date,
			delayed_payment_treatment,
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

	/// Get a reference to the loan activation moment.
	pub fn activation_date(&self) -> &Timestamp {
		&self.activation_date
	}

	/// Returns `true` if it is possible to delay a payment.
	pub fn is_payments_relaxed(&self) -> bool {
		self.delayed_payment_treatment.is_some()
	}

	pub fn delayed_payments_threshold(&self) -> Option<u32> {
		Some(self.delayed_payment_treatment.clone()?.delayed_payments_threshold)
	}

	pub fn delayed_payments_shift_in_days(&self) -> Option<i64> {
		Some(self.delayed_payment_treatment.clone()?.delayed_payments_shift_in_days)
	}

	// Get next payment date from the local payment schedule.
	// Note that local schedule make sense for dates equal or larger current date.
	pub fn get_next_payment_date(&self, date: Timestamp) -> Option<Timestamp> {
		self.schedule.keys().find(|&payment_date| payment_date > &date).cloned()
	}
}

// Some fields are hidden since they should be immutable.
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
	// Ids of liquidation strategies applicable to the market.
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
}

// Some fields are hidden since they should be immutable.
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct LoanInfo<AccountId, AssetId, Balance, Timestamp>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	Timestamp: Clone + Eq + PartialEq + Ord,
{
	config: LoanConfig<AccountId, AssetId, Balance, Timestamp>,
	pub last_payment_date: Timestamp,
	pub delayed_payments_counter: u32,
}

impl<AccountId, AssetId, Balance, Timestamp> LoanInfo<AccountId, AssetId, Balance, Timestamp>
where
	AccountId: Clone + Eq + PartialEq,
	AssetId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	Timestamp: Clone + Eq + PartialEq + Ord,
{
	pub fn new(
		config: LoanConfig<AccountId, AssetId, Balance, Timestamp>,
		last_payment_date: Timestamp,
	) -> Self {
		Self { config, last_payment_date, delayed_payments_counter: 0 }
	}

	pub fn config(&self) -> &LoanConfig<AccountId, AssetId, Balance, Timestamp> {
		&self.config
	}
}

/// Input to create market extrinsic.
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub struct MarketInput<AccountId, AssetId, BlockNumber, LiquidationStrategyId> {
	/// Borrow currency.
	pub borrow_asset: AssetId,
	/// Collateral currency.
	pub collateral_asset: AssetId,
	/// Reserve factor of market borrow vault.
	pub reserved_factor: Perquintill,
	/// List of trusted borrowers
	pub whitelist: BTreeSet<AccountId>,
	/// Liquidation engine id.
	pub liquidation_strategies: Vec<LiquidationStrategyId>,
	/// Count of blocks until throw error PriceIsTooOld.
	pub max_price_age: BlockNumber,
}

/// Input to create loan extrinsic.
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
	pub payment_schedule: BTreeMap<Timestamp, Balance>,
	/// Contract should be activated before this date.
	pub activation_date: Timestamp,
	/// If this filed is None, borrower has to face his obligation in time.
	pub delayed_payment_treatment: Option<DelayedPaymentTreatment>,
}

/// Structure contains information regarding failed payment behaviour.
#[derive(Encode, Decode, TypeInfo, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct DelayedPaymentTreatment {
	/// In the case of payment's fail,
	/// it will be shifted to this amount of days.
	pub delayed_payments_shift_in_days: i64,
	/// Borrower can fail payment this amount of time.
	/// If threshold is exceeded the loan liquidates and
	/// borrower adds to the blacklist.
	pub delayed_payments_threshold: u32,
}
