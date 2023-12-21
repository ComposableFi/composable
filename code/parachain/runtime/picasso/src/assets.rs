use frame_support::traits::{fungible::Credit, tokens::Imbalance, Currency, OnUnbalanced};

use super::*;

pub type BalanceIdentifier = [u8; 8];

type MaxLocks = ConstU32<64>;

pub struct MoveDustToTreasury;

impl OnUnbalanced<Credit<<Runtime as frame_system::Config>::AccountId, balances::Pallet<Runtime>>>
	for MoveDustToTreasury
{
	fn on_nonzero_unbalanced(
		amount: Credit<<Runtime as frame_system::Config>::AccountId, balances::Pallet<Runtime>>,
	) {
		let _ = <Balances as Currency<AccountId>>::deposit_creating(
			&TreasuryPalletId::get().into_account_truncating(),
			amount.peek(),
		);
	}
}

impl balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
	type ReserveIdentifier = BalanceIdentifier;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = MoveDustToTreasury;
	type ExistentialDeposit = NativeExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::balances::SubstrateWeight<Runtime>;
	type FreezeIdentifier = BalanceIdentifier;
	type HoldIdentifier = ();
	type MaxHolds = ConstU32<32>;
	type MaxFreezes = ConstU32<32>;
}

impl pallet_assets::Config for Runtime {
	type NativeAssetId = NativeAssetId;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type NativeCurrency = Balances;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
	type AdminOrigin = EnsureRoot<Self::AccountId>;
	type CurrencyValidator = ValidateCurrencyId;
	type RuntimeHoldReason = ();
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
	type MaxLocks = MaxLocks;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = ConstU32<2>;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type CurrencyHooks = CurrencyHooks;
}
