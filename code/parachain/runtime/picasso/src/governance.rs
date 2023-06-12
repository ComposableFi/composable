//! Runtime setup for the governance and democracy only.

use super::*;
use common::governance::native::*;
use frame_support::traits::LockIdentifier;

pub type NativeCouncilMembership = membership::Instance1;
pub type NativeTechnicalMembership = membership::Instance2;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

impl membership::Config<NativeCouncilMembership> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type MembershipInitialized = Council;
	type MembershipChanged = Council;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

impl collective::Config<NativeCouncilCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
	type SetMembersOrigin = EnsureRootOrTwoThirdNativeCouncil;
}

impl membership::Config<NativeTechnicalMembership> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrTwoThirdNativeCouncilOrTechnical;
	type RemoveOrigin = EnsureRootOrTwoThirdNativeCouncilOrTechnical;
	type SwapOrigin = EnsureRootOrTwoThirdNativeCouncilOrTechnical;
	type ResetOrigin = EnsureRootOrTwoThirdNativeCouncilOrTechnical;
	type PrimeOrigin = EnsureRootOrTwoThirdNativeCouncilOrTechnical;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

impl collective::Config<NativeTechnicalMembership> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
	type SetMembersOrigin = EnsureRootOrTwoThirds<NativeTechnicalCollective>;
}

parameter_types! {
	pub MinimumDeposit: Balance = 100 * CurrencyId::unit::<Balance>();
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	// cspell:disable-next
	pub const DemocracyId: LockIdentifier = *b"democrac";
	pub RootOrigin: RuntimeOrigin = frame_system::RawOrigin::Root.into();
}

impl democracy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;

	#[cfg(not(feature = "fastnet"))]
	type LaunchPeriod = ConstU32<{ 12 * HOURS }>;
	#[cfg(not(feature = "fastnet"))]
	type VotingPeriod = ConstU32<{ 60 * HOURS }>;
	#[cfg(not(feature = "fastnet"))]
	type EnactmentPeriod = ConstU32<{ 12 * HOURS }>;
	#[cfg(not(feature = "fastnet"))]
	type VoteLockingPeriod = ConstU32<DAYS>;

	#[cfg(feature = "fastnet")]
	type LaunchPeriod = ConstU32<{ 1 * HOURS }>;
	#[cfg(feature = "fastnet")]
	type VotingPeriod = ConstU32<{ 3 * HOURS }>;
	#[cfg(feature = "fastnet")]
	type EnactmentPeriod = ConstU32<{ 1 * HOURS }>;
	#[cfg(feature = "fastnet")]
	type VoteLockingPeriod = ConstU32<{ 1 * HOURS }>;

	type MinimumDeposit = ConstU128<5_000_000_000_000_000>;
	type ExternalOrigin = EnsureRootOrTwoThirdNativeCouncil;

	type ExternalMajorityOrigin = EnsureRootOrMoreThenHalfNativeCouncil;

	type ExternalDefaultOrigin = EnsureRootOrTwoThirdNativeCouncil;

	type FastTrackOrigin = EnsureRootOrHalfNativeTechnical;
	type InstantOrigin = EnsureRootOrHalfNativeTechnical;
	type InstantAllowed = InstantAllowed;

	#[cfg(not(feature = "fastnet"))]
	type FastTrackVotingPeriod = ConstU32<{ 3 * HOURS }>;

	#[cfg(feature = "fastnet")]
	type FastTrackVotingPeriod = ConstU32<{ 5 * common::MINUTES }>;

	type CancellationOrigin = EnsureRootOrTwoThirds<NativeTechnicalCollective>;

	type BlacklistOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type CancelProposalOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type VetoOrigin = EnsureNativeTechnicalMember;
	type Slash = Treasury;

	type CooloffPeriod = ConstU32<{ 3 * DAYS }>;
	type MaxProposals = ConstU32<50>;
	type MaxVotes = MaxVotes;
	type PalletsOrigin = OriginCaller;

	type Preimages = Preimage;
	type MaxDeposits = ConstU32<100>;
	type MaxBlacklisted = ConstU32<100>;

	type Scheduler = Scheduler;
	type WeightInfo = democracy::weights::SubstrateWeight<Runtime>;

	#[cfg(feature = "runtime-benchmarks")]
	type SubmitOrigin = system::EnsureSigned<Self::AccountId>;

	#[cfg(not(feature = "runtime-benchmarks"))]
	type SubmitOrigin = frame_support::traits::EitherOf<
		system::EnsureSignedBy<TechnicalCommitteeMembership, Self::AccountId>,
		system::EnsureSignedBy<CouncilMembership, Self::AccountId>,
	>;
}

parameter_types! {
	// cspell:disable-next
	pub const TreasuryPalletId: PalletId = PalletId(*b"picatrsy");
	/// Percentage of proposal that most be bonded by the proposer.
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub ProposalBondMinimum: Balance = 5000 * CurrencyId::unit::<Balance>();
	pub ProposalBondMaximum: Balance = 10000 * CurrencyId::unit::<Balance>();
	pub const Burn: Permill = Permill::from_percent(0);
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl treasury::Config<NativeTreasury> for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	#[cfg(feature = "fastnet")]
	type SpendPeriod = ConstU32<{ 1 * HOURS }>;
	#[cfg(not(feature = "fastnet"))]
	type SpendPeriod = ConstU32<{ 3 * DAYS }>;
	type Burn = Burn;
	type MaxApprovals = ConstU32<30>;
	type BurnDestination = ();
	type WeightInfo = treasury::weights::SubstrateWeight<Runtime>;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;

	#[cfg(feature = "testnet")]
	type ApproveOrigin = EnsureRootOrOneThirdNativeCouncil;
	#[cfg(feature = "testnet")]
	type RejectOrigin = EnsureRootOrOneThirdNativeCouncil;

	#[cfg(not(feature = "testnet"))]
	type ApproveOrigin = EnsureRootOrTwoThirdNativeCouncil;
	#[cfg(not(feature = "testnet"))]
	type RejectOrigin = EnsureRootOrTwoThirdNativeCouncil;
}

impl governance_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AssetId = CurrencyId;
	type WeightInfo = ();
}

impl sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}
