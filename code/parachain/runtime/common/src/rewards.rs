use frame_support::traits::{Currency, Imbalance, OnUnbalanced};

pub type NegativeImbalance<T> =
	<balances::Pallet<T> as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

/// Logic for the author to get a portion of fees.
pub struct ToStakingPot<R, I>(sp_std::marker::PhantomData<(R, I)>);
impl<R, I: 'static> OnUnbalanced<NegativeImbalance<R>> for ToStakingPot<R, I>
where
	R: balances::Config
		+ collator_selection::Config
		+ treasury::Config<I, Currency = balances::Pallet<R>>,
	<R as frame_system::Config>::AccountId: From<polkadot_primitives::v2::AccountId>,
	<R as frame_system::Config>::AccountId: Into<polkadot_primitives::v2::AccountId>,
	<R as frame_system::Config>::Event: From<balances::Event<R>>,
	<R as balances::Config>::Balance: From<u128>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		// TODO (vim): This configuration must be a part of each runtime configuration instead
		// being a  common configuration. We could build a small custom pallet to capture this
		// configuration while  also making part of on-chain governance through extrinsics.
		// Collator's get half the fees
		let (to_collators, to_treasury) = amount.ration(25, 75);
		// 30% gets burned 20% to treasury
		// let (_pre_burn, to_treasury) = half.ration(30, 20);

		let staking_pot = <collator_selection::Pallet<R>>::account_id();
		<balances::Pallet<R>>::resolve_creating(&staking_pot, to_collators);
		<treasury::Pallet<R, I> as OnUnbalanced<_>>::on_unbalanced(to_treasury);
	}
}

/// OnUnbalanced handler for pallet-transaction-payment.
pub struct DealWithFees<R, I>(sp_std::marker::PhantomData<(R, I)>);

impl<R, I: 'static> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R, I>
where
	R: balances::Config
		+ collator_selection::Config
		+ treasury::Config<I, Currency = balances::Pallet<R>>,
	<R as frame_system::Config>::AccountId: From<polkadot_primitives::v2::AccountId>,
	<R as frame_system::Config>::AccountId: Into<polkadot_primitives::v2::AccountId>,
	<R as frame_system::Config>::Event: From<balances::Event<R>>,
	<R as balances::Config>::Balance: From<u128>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(mut fees) = fees_then_tips.next() {
			if let Some(tips) = fees_then_tips.next() {
				tips.merge_into(&mut fees);
			}
			<ToStakingPot<R, I> as OnUnbalanced<_>>::on_unbalanced(fees);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{governance::native::NativeTreasury, Balance, BlockNumber, DAYS};
	use collator_selection::IdentityCollator;
	use frame_support::{
		ord_parameter_types, parameter_types,
		traits::{Everything, FindAuthor, ValidatorRegistration},
		PalletId,
	};
	use frame_system::{limits, EnsureRoot};
	use num_traits::Zero;
	use orml_traits::parameter_type_with_key;
	use polkadot_primitives::v2::AccountId;
	use primitives::currency::CurrencyId;
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup},
		Perbill, Permill,
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Balances: balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			Treasury: treasury::<Instance1>::{Pallet, Call, Storage, Config, Event<T>} = 31,
			CollatorSelection: collator_selection::{Pallet, Call, Storage, Event<T>},
			Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
			Sudo: sudo::{Pallet, Call, Config<T>, Storage, Event<T>} = 2,
		}
	);

	pub type Amount = i128;

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub BlockLength: limits::BlockLength = limits::BlockLength::max(2 * 1024);
		pub const AvailableBlockRatio: Perbill = Perbill::one();
	}

	impl frame_system::Config for Test {
		type BaseCallFilter = Everything;
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Call = Call;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = AccountId;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = BlockHashCount;
		type BlockLength = BlockLength;
		type BlockWeights = ();
		type DbWeight = ();
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = balances::AccountData<u128>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 3;
	}

	impl balances::Config for Test {
		type Balance = u128;
		type Event = Event;
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type MaxLocks = ();
		type ReserveIdentifier = [u8; 8];
		type MaxReserves = ();
		type WeightInfo = ();
	}

	pub struct OneAuthor;
	impl FindAuthor<AccountId> for OneAuthor {
		fn find_author<'a, I>(_: I) -> Option<AccountId>
		where
			I: 'a,
		{
			Some(AccountId::from([0_u8; 32]))
		}
	}

	pub struct IsRegistered;
	impl ValidatorRegistration<AccountId> for IsRegistered {
		fn is_registered(_id: &AccountId) -> bool {
			true
		}
	}

	parameter_types! {
		pub const PotId: PalletId = PalletId(*b"PotStake");
		pub const MaxCandidates: u32 = 20;
		pub const MaxInvulnerables: u32 = 20;
		pub const MinCandidates: u32 = 1;
	}

	impl collator_selection::Config for Test {
		type Event = Event;
		type Currency = Balances;
		type UpdateOrigin = EnsureRoot<AccountId>;
		type PotId = PotId;
		type MaxCandidates = MaxCandidates;
		type MinCandidates = MinCandidates;
		type MaxInvulnerables = MaxInvulnerables;
		type ValidatorId = <Self as frame_system::Config>::AccountId;
		type ValidatorIdOf = IdentityCollator;
		type ValidatorRegistration = IsRegistered;
		type KickThreshold = ();
		type WeightInfo = ();
	}

	parameter_type_with_key! {
		// TODO: make this non zero
		pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
			Zero::zero()
		};
	}

	type ReserveIdentifier = [u8; 8];
	impl orml_tokens::Config for Test {
		type Event = Event;
		type Balance = Balance;
		type Amount = Amount;
		type CurrencyId = CurrencyId;
		type WeightInfo = ();
		type ExistentialDeposits = ExistentialDeposits;
		type OnDust = ();
		type MaxLocks = ();
		type ReserveIdentifier = ReserveIdentifier;
		type MaxReserves = frame_support::traits::ConstU32<2>;
		type DustRemovalWhitelist = Everything;
		type OnKilledTokenAccount = ();
		type OnNewTokenAccount = ();
	}

	ord_parameter_types! {
		pub const RootAccount: u128 = 2;
	}

	impl authorship::Config for Test {
		type FindAuthor = OneAuthor;
		type UncleGenerations = ();
		type FilterUncle = ();
		type EventHandler = ();
	}

	impl sudo::Config for Test {
		type Event = Event;
		type Call = Call;
	}

	parameter_types! {
		pub const TreasuryPalletId: PalletId = PalletId(*b"picatrsy");
		/// percentage of proposal that most be bonded by the proposer
		pub const ProposalBond: Permill = Permill::from_percent(5);
		// TODO: rationale?
		pub ProposalBondMinimum: Balance = 5 * CurrencyId::unit::<Balance>();
		pub ProposalBondMaximum: Balance = 1000 * CurrencyId::unit::<Balance>();
		pub const SpendPeriod: BlockNumber = 7 * DAYS;
		pub const Burn: Permill = Permill::from_percent(0);

		pub const MaxApprovals: u32 = 30;
	}

	impl treasury::Config<NativeTreasury> for Test {
		type PalletId = TreasuryPalletId;
		type Currency = Balances;
		type ApproveOrigin = EnsureRoot<AccountId>;
		type RejectOrigin = EnsureRoot<AccountId>;
		type Event = Event;
		type OnSlash = Treasury;
		type ProposalBond = ProposalBond;
		type ProposalBondMinimum = ProposalBondMinimum;
		type ProposalBondMaximum = ProposalBondMaximum;
		type SpendPeriod = SpendPeriod;
		type Burn = Burn;
		type MaxApprovals = MaxApprovals;
		type BurnDestination = ();
		type WeightInfo = ();
		// TODO: add bounties?
		type SpendFunds = ();
		type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	}

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		balances::GenesisConfig::<Test>::default().assimilate_storage(&mut t).unwrap();
		t.into()
	}

	#[test]
	fn test_fees_and_tip_split() {
		new_test_ext().execute_with(|| {
			let fee = Balances::issue(30);
			let tip = Balances::issue(70);

			DealWithFees::on_unbalanceds(vec![fee, tip].into_iter());

			// Author gets 25% of tip and 25% of fee = 25
			assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 25);
			// Treasury gets 20%
			assert_eq!(Balances::free_balance(Treasury::account_id()), 75);
		});
	}

	#[test]
	fn test_fees_and_tip_split_0() {
		new_test_ext().execute_with(|| {
			let fee = Balances::issue(0);
			let tip = Balances::issue(0);

			DealWithFees::on_unbalanceds(vec![fee, tip].into_iter());

			// Author gets 50% of tip and 50% of fee = 15
			assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 0);
			assert_eq!(Balances::free_balance(Treasury::account_id()), 0);
		});
	}

	#[test]
	fn test_fees_and_tip_split_under_ed() {
		new_test_ext().execute_with(|| {
			let fee = Balances::issue(1);
			let tip = Balances::issue(1);

			DealWithFees::on_unbalanceds(vec![fee, tip].into_iter());

			// Author gets 50% of tip and 50% of fee = 15
			assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 0);
			assert_eq!(Balances::free_balance(Treasury::account_id()), 0);
		});
	}
}
