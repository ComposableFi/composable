use super::*;

impl pallet_assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureRootOrHalfCouncil;
	type CurrencyValidator = ValidateCurrencyId;
	type RuntimeHoldReason = TemporalHoldIdentifier;
}

pub struct CurrencyHooks;
impl orml_traits::currency::MutationHooks<AccountId, CurrencyId, Balance> for CurrencyHooks {
	type OnDust = orml_tokens::TransferDust<Runtime, TreasuryAccount>;
	type OnSlash = ();
	type PreDeposit = ();
	type PostDeposit = ();
	type PreTransfer = ();
	type PostTransfer = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = weights::tokens::WeightInfo<Runtime>;
	type ExistentialDeposits = MultiExistentialDeposits;
	type MaxLocks = ConstU32<32>;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type CurrencyHooks = CurrencyHooks;
}

parameter_types! {
	/// Minimum amount an account has to hold to stay in state.
	// minimum account balance is given as 0.1 PICA ~ 100 CurrencyId::milli()
	pub ExistentialDeposit: Balance = 100 * CurrencyId::milli::<Balance>();
}

pub type BalanceIdentifier = [u8; 8];

/// until next upgrade when ORM will be upgraded
pub type TemporalHoldIdentifier = ();

impl balances::Config for Runtime {
	type MaxLocks = ConstU32<64>;
	type MaxReserves = ();
	type ReserveIdentifier = BalanceIdentifier;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::balances::SubstrateWeight<Runtime>;

	type HoldIdentifier = TemporalHoldIdentifier;

	type FreezeIdentifier = BalanceIdentifier;

	type MaxHolds = ConstU32<32>;

	type MaxFreezes = ConstU32<32>;
}
