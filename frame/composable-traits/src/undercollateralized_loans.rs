use crate::defi::{CurrencyPair, DeFiEngine}; 
use composable_support::math::safe::SafeAdd;
use frame_support::pallet_prelude::*;
use sp_runtime::{ArithmeticError, Perquintill, traits::Zero};
use sp_std::collections::btree_set::BTreeSet;
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
	collateral_asset: AssetId,
	/// The asset being used as borrow asset.
	borrow_asset: AssetId,
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
		borrow_asset: AssetId,
		collateral_asset: AssetId,
		max_price_age: BlockNumber,
		whitelist: BTreeSet<AccountId>,
	) -> Self {
		Self {
			account_id,
			manager,
			borrow_asset_vault,
			borrow_asset,
			collateral_asset,
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
	pub fn borrow_asset(&self) -> &AssetId {
		&self.borrow_asset
	}

	/// Get a reference to the market config's collateral asset.
	pub fn collateral_asset(&self) -> &AssetId {
		&self.collateral_asset
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
pub struct LoanConfig<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	Percent: Clone + Eq + PartialEq,
    RepaymentStrategy: Clone,
{
	/// Loan account id.
	account_id: AccountId,
	/// Market account id.
	market_account_id: AccountId,
	/// Borrower account id.
	/// Should be whitelisted.
	borrower_account_id: AccountId,
	/// Amount of borrowed money.  
	principal: Balance,
	/// Amount of assets which should be putted as collateral.
	collateral: Balance,
	/// Interest rate per payment.
	interest: Percent,
	/// How often borrowers have to pay interest.
	payment_frequency: BlockNumber,
	/// Activated loan lifetime in the terms of block numbers.
	maturity: BlockNumber,
    /// Payment strategie which should be applyed.
    /// For instance borrower have to pay principal when loan is mature (one strategy),
    /// or he may pay principal partially, simultaneously with interest payments.   
    repayment_strategy: RepaymentStrategy,
}

impl<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent> LoanConfig<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	Percent: Clone + Eq + PartialEq, 
    RepaymentStrategy: Clone,
{
	pub fn new(
		account_id: AccountId,
		market_account_id: AccountId,
		borrower_account_id: AccountId,
		principal: Balance,
		collateral: Balance,
		interest: Percent,
		payment_frequency: BlockNumber,
		maturity: BlockNumber,
        repayment_strategy: RepaymentStrategy,
	) -> Self {
		Self {
			account_id,
			market_account_id,
			borrower_account_id,
			principal,
			collateral,
			interest,
			payment_frequency,
			maturity,
            repayment_strategy,
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

	/// Get a reference to the loan config's principal.
	pub fn principal(&self) -> &Balance {
		&self.principal
	}

	/// Get a reference to the loan config's collateral.
	pub fn collateral(&self) -> &Balance {
		&self.collateral
	}

	/// Get a mutable reference to the loan config's interest.
	pub fn interest(&self) -> &Percent {
		&self.interest
	}

	/// Get a reference to the loan config's payment frequency.
	pub fn payment_frequency(&self) -> &BlockNumber {
		&self.payment_frequency
	}

	/// Get a reference to the loan config's maturity.
	pub fn maturity(&self) -> &BlockNumber {
		&self.maturity
	}

    /// Get a reference to the loan config's payment strategy.
    pub fn repayment_strategy(&self) -> &RepaymentStrategy {
        &self.repayment_strategy
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

// Some fields are hiden since they should be immutable
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, Eq, PartialEq)]
pub struct LoanInfo<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq,
	BlockNumber: Clone + Eq + PartialEq,
	Percent: Clone + Eq + PartialEq,
    RepaymentStrategy: Clone,
{
	/// Loan configuration defines loan terms
	config: LoanConfig<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent>,
	/// Principal should be returned before this block.
	end_block: BlockNumber,
    /// How much principal was repaid. 
    pub repaid_principal: Balance,
}

impl<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent> LoanInfo<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent>
where
	AccountId: Clone + Eq + PartialEq,
	Balance: Clone + Eq + PartialEq + Zero,
	BlockNumber: SafeAdd + Clone + Eq + PartialEq,
	Percent: Clone + Eq + PartialEq,
    RepaymentStrategy: Clone,
{
	pub fn new(
        config: LoanConfig<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent>,
		start_block: BlockNumber,
	) -> Result<Self, ArithmeticError> {
		let end_block = start_block.safe_add(config.maturity())?;
		Ok(Self { config, end_block, repaid_principal: Balance::zero() })
	}

	/// Get a reference to the loan info's config.
	pub fn config(&self) -> &LoanConfig<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent> {
		&self.config
	}

	/// Get a reference to the loan info's end block.
	pub fn end_block(&self) -> &BlockNumber {
		&self.end_block
	}
}

/// input to create market extrinsic
#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub struct MarketInput<AccountId, AssetId, BlockNumber, LiquidationStrategyId> {
	/// collateral currency and borrow currency.
	pub currency_pair: CurrencyPair<AssetId>,
	/// Reserve factor of market borrow vault.
	pub reserved_factor: Perquintill,
	// TODO: @mikolaichuk: BoundedVec
	pub whitelist: BTreeSet<AccountId>,
	/// liquidation engine id.
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

#[derive(Encode, Decode, Default, TypeInfo, RuntimeDebug, Clone, PartialEq)]
pub struct LoanInput<AccountId, Balance, BlockNumber, RepaymentStrategy, Percent> {
	/// Loan belongs to this market.
	pub market_account_id: AccountId,
	/// This account id have to be whitelisted.
	pub borrower_account_id: AccountId,
	/// Amount of borrowed money.  
	pub principal: Balance,
	/// Amount of assets which should be deposited as collateral.
	pub collateral: Balance,
	/// Interest rate per block.
	pub interest: Percent,
	/// How often borrowers have to pay interest.
	pub payment_frequency: BlockNumber,
	/// Loan shoud be paid back after this amount of blocks.
	pub loan_maturity: BlockNumber,
    /// Payment strategie which should be applyed.
    /// For instance borrower have to pay principal when loan is mature (one strategy),
    /// or he may pay principal partially, simultaneously with interest payments.   
    pub repayment_strategy: RepaymentStrategy,
}

pub trait UndercollateralizedLoans: DeFiEngine {
	type BlockNumber: Clone + Eq + PartialEq;
	type LiquidationStrategyId: Clone + Eq + PartialEq;
	type Percent: Clone + Eq + PartialEq;
	type VaultId: Clone + Eq + PartialEq;
    type RepaymentStrategy: Clone;

	fn create_market(
		manager: Self::AccountId,
		input: MarketInput<
			Self::AccountId,
			Self::MayBeAssetId,
			Self::BlockNumber,
			Self::LiquidationStrategyId,
		>,
		keep_alive: bool,
	) -> Result<
		MarketInfo<
			Self::AccountId,
			Self::MayBeAssetId,
			Self::BlockNumber,
			Self::LiquidationStrategyId,
			Self::VaultId,
		>,
		DispatchError,
	>;

	fn create_loan(
		input: LoanInput<Self::AccountId, Self::Balance, Self::BlockNumber, Self::RepaymentStrategy, Self::Percent>,
	) -> Result<
		LoanConfig<Self::AccountId, Self::Balance, Self::BlockNumber, Self::RepaymentStrategy, Self::Percent>,
		DispatchError,
	>;

	fn borrow(
		borrower_account_id: Self::AccountId,
		loan_account_id: Self::AccountId,
		keep_alive: bool,
	) -> Result<
		LoanInfo<Self::AccountId, Self::Balance, Self::BlockNumber, Self::RepaymentStrategy, Self::Percent>,
		DispatchError,
	>;

	fn market_account_id<S: Encode>(postfix: S) -> Self::AccountId;

	fn loan_account_id<S: Encode>(postfix: S) -> Self::AccountId;

	fn is_borrower_account_whitelisted(
		borrower_account_id_ref: &Self::AccountId,
		market_id_ref: &Self::AccountId,
	) -> Result<bool, DispatchError>;
}
