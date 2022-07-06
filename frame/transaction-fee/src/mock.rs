#![allow(dead_code)]

use support::pallet_prelude::*;

use std::cell::RefCell;

use smallvec::smallvec;

use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

use orml_traits::parameter_type_with_key;
use primitives::currency::CurrencyId;
use support::{
	parameter_types,
	traits::{Everything, Imbalance, OnUnbalanced},
	weights::*,
};

type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = system::mocking::MockBlock<Runtime>;

support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: system::{Pallet, Call, Config, Storage, Event<T>},
		Tokens: orml_tokens::{Pallet, Call, Config<T>, Storage, Event<T>},
		TransactionPayment: crate::{Pallet, Storage},
	}
);

mod dex {
	use super::*;
	pub struct Dex;
	use composable_traits::dex::SimpleExchange;
	use orml_tokens::Pallet;
	use orml_traits::MultiCurrency;
	use sp_runtime::{DispatchError, Perbill};

	impl SimpleExchange for Dex {
		type AssetId = CurrencyId;
		type Balance = u64;
		type AccountId = u64;
		type Error = DispatchError;

		fn price(_asset_id: Self::AssetId) -> Option<Self::Balance> {
			todo!("not used in tests")
		}

		fn exchange(
			_from: Self::AssetId,
			_from_account: Self::AccountId,
			to: Self::AssetId,
			to_account: Self::AccountId,
			to_amount: Self::Balance,
			_slippage: Perbill,
		) -> Result<Self::Balance, Self::Error> {
			<Pallet<Runtime> as MultiCurrency<u64>>::deposit(to, &to_account, to_amount).map(|_| 1)
		}
	}
}

thread_local! {
	static EXTRINSIC_BASE_WEIGHT: RefCell<u64> = RefCell::new(0);
}

pub struct BlockWeights;
impl Get<system::limits::BlockWeights> for BlockWeights {
	fn get() -> system::limits::BlockWeights {
		system::limits::BlockWeights::builder()
			.base_block(0)
			.for_class(DispatchClass::all(), |weights| {
				weights.base_extrinsic = EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow());
			})
			.for_class(DispatchClass::non_mandatory(), |weights| {
				weights.max_total = 1024.into();
			})
			.build_or_panic()
	}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = Call;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = orml_tokens::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = support::traits::ConstU32<16>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> u64 {
		1
	};
}

type ReserveIdentifier = [u8; 8];
impl orml_tokens::Config for Runtime {
	type Balance = u64;
	type Event = Event;
	type Amount = i64;
	type OnDust = ();
	type MaxLocks = ();
	type WeightInfo = ();
	type CurrencyId = CurrencyId;
	type DustRemovalWhitelist = Everything;
	type ReserveIdentifier = ReserveIdentifier;
	type MaxReserves = frame_support::traits::ConstU32<2>;
	type ExistentialDeposits = ExistentialDeposits;
  type OnNewTokenAccount = ();
  type OnKilledTokenAccount = ();
}

impl WeightToFeePolynomial for WeightToFee {
	type Balance = u64;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			coeff_frac: Perbill::zero(),
			coeff_integer: WEIGHT_TO_FEE.with(|v| *v.borrow()),
			negative: false,
		}]
	}
}

thread_local! {
	pub static TIP_UNBALANCED_AMOUNT: RefCell<u64> = RefCell::new(0);
	pub static FEE_UNBALANCED_AMOUNT: RefCell<u64> = RefCell::new(0);
}

pub struct DealWithFees;
impl OnUnbalanced<orml_tokens::NegativeImbalance<Runtime, NativeCurrencyId>> for DealWithFees {
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<
			Item = orml_tokens::NegativeImbalance<Runtime, NativeCurrencyId>,
		>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			FEE_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += fees.peek());
			if let Some(tips) = fees_then_tips.next() {
				TIP_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += tips.peek());
			}
		}
	}
}

parameter_types! {
	pub const NativeCurrencyId: CurrencyId = CurrencyId::LAYR;
	pub static TransactionByteFee: u64 = 1;
	pub static WeightToFee: u64 = 1;
}

impl crate::Config for Runtime {
	type OnChargeTransaction = DealWithFees;
	type NativeCurrency = orml_tokens::CurrencyAdapter<Runtime, NativeCurrencyId>;
	type Dex = dex::Dex;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = WeightToFee;
	type FeeMultiplierUpdate = ();
}

pub struct ExtBuilder {
	balance_factor: (CurrencyId, u64),
	base_weight: u64,
	byte_fee: u64,
	weight_to_fee: u64,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balance_factor: (CurrencyId::LAYR, 1),
			base_weight: 0,
			byte_fee: 1,
			weight_to_fee: 1,
		}
	}
}

impl ExtBuilder {
	pub fn base_weight(mut self, base_weight: u64) -> Self {
		self.base_weight = base_weight;
		self
	}
	pub fn byte_fee(mut self, byte_fee: u64) -> Self {
		self.byte_fee = byte_fee;
		self
	}
	pub fn weight_fee(mut self, weight_to_fee: u64) -> Self {
		self.weight_to_fee = weight_to_fee;
		self
	}
	pub fn balance_factor(mut self, factor: (CurrencyId, u64)) -> Self {
		self.balance_factor = factor;
		self
	}
	fn set_constants(&self) {
		EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow_mut() = self.base_weight);
		TRANSACTION_BYTE_FEE.with(|v| *v.borrow_mut() = self.byte_fee);
		WEIGHT_TO_FEE.with(|v| *v.borrow_mut() = self.weight_to_fee);
	}
	pub fn build(self) -> sp_io::TestExternalities {
		self.set_constants();
		let mut t = system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
		orml_tokens::GenesisConfig::<Runtime> {
			balances: if self.balance_factor.1 > 0 {
				vec![
					(1, self.balance_factor.0, self.balance_factor.1),
					(2, self.balance_factor.0, self.balance_factor.1),
					(3, self.balance_factor.0, self.balance_factor.1),
					(4, self.balance_factor.0, self.balance_factor.1),
					(5, self.balance_factor.0, self.balance_factor.1),
					(6, self.balance_factor.0, self.balance_factor.1),
				]
			} else {
				vec![]
			},
		}
		.assimilate_storage(&mut t)
		.unwrap();
		t.into()
	}
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
	// pays_fee: Pays::Yes -- class: DispatchClass::Normal
	DispatchInfo { weight: w, ..Default::default() }
}

fn post_info_from_weight(w: Weight) -> PostDispatchInfo {
	PostDispatchInfo { actual_weight: Some(w), pays_fee: Default::default() }
}

fn post_info_from_pays(p: Pays) -> PostDispatchInfo {
	PostDispatchInfo { actual_weight: None, pays_fee: p }
}

fn default_post_info() -> PostDispatchInfo {
	PostDispatchInfo { actual_weight: None, pays_fee: Default::default() }
}
