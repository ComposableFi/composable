//! Runtime setup for the governance and democracy only.

use super::*;
use common::governance::native::*;
use frame_support::traits::{EitherOf, LockIdentifier};

pub type NativeCouncilMembership = membership::Instance1;
pub type NativeTechnicalMembership = membership::Instance2;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub const AlarmInterval: BlockNumber = 1;
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
	type SetMembersOrigin = EnsureRootOrTwoThirds<NativeCouncilCollective>;
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

pallet_referenda::impl_tracksinfo_get!(TracksInfo, Balance, BlockNumber);
impl pallet_referenda::Config for Runtime {
	type RuntimeCall = RuntimeCall;

	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = weights::referenda::WeightInfo<Self>;

	type Scheduler = Scheduler;

	type Currency = Balances;

	type SubmitOrigin = frame_support::traits::EitherOf<
		system::EnsureSignedBy<TechnicalCommitteeMembership, Self::AccountId>,
		system::EnsureSignedBy<CouncilMembership, Self::AccountId>,
	>;
	
	type CancelOrigin = EnsureRootOrOneThirdNativeTechnical;

	type KillOrigin = EnsureRootOrMoreThenHalfNativeCouncil;

	type Slash = ();

	type Votes = pallet_conviction_voting::VotesOf<Runtime>;

	type Tally = pallet_conviction_voting::TallyOf<Runtime>;

	type SubmissionDeposit = ConstU128<0>;

	type MaxQueued = ConstU32<16>;

	type UndecidingTimeout = ConstU32<{ 3 * DAYS }>;

	type AlarmInterval = AlarmInterval;

	type Tracks = TracksInfo;

	type Preimages = Preimage;
}

parameter_types! {
	pub const VoteLockingPeriod: BlockNumber = 0;
}

impl pallet_conviction_voting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::conviction_voting::WeightInfo<Self>;
	type Currency = OpenGovBalances;

	type Polls = Referenda;

	type MaxTurnout = frame_support::traits::TotalIssuanceOf<OpenGovBalances, Self::AccountId>;

	type MaxVotes = ConstU32<20>;

	type VoteLockingPeriod = VoteLockingPeriod;
}

impl pallet_custom_origins::Config for Runtime {}

pub use pallet_custom_origins::WhitelistedCaller;

impl pallet_whitelist::Config for Runtime {
	type WeightInfo = weights::whitelist::WeightInfo<Self>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type WhitelistOrigin = EnsureRootOrOneThirdNativeTechnical;
	type DispatchWhitelistedOrigin = EitherOf<EnsureRoot<Self::AccountId>, WhitelistedCaller>;
	type Preimages = Preimage;
}

parameter_types! {
	pub const EnactmentPeriod: BlockNumber = 2 * DAYS;
	pub const CooloffPeriod: BlockNumber = 7 * DAYS;
	pub const VotingPeriod: BlockNumber = 5 * DAYS;
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
	// cspell:disable-next
	pub const DemocracyId: LockIdentifier = *b"democrac";
	pub RootOrigin: RuntimeOrigin = frame_system::RawOrigin::Root.into();
}

impl democracy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = ConstU32<{ 5 * DAYS }>;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod;
	type MinimumDeposit = ConstU128<5_000_000_000_000_000>;
	type ExternalOrigin = EnsureRootOrTwoThirdNativeCouncil;

	type ExternalMajorityOrigin = EnsureRootOrMoreThenHalfNativeCouncil;

	type ExternalDefaultOrigin = EnsureRootOrTwoThirdNativeCouncil;

	type FastTrackOrigin = EnsureRootOrHalfNativeTechnical;
	type InstantOrigin = EnsureRootOrHalfNativeTechnical;
	type InstantAllowed = InstantAllowed;

	#[cfg(not(feature = "westend"))]
	type FastTrackVotingPeriod = ConstU32<{ 3 * HOURS }>;

	#[cfg(feature = "westend")]
	type FastTrackVotingPeriod = ConstU32<HOURS>;

	type CancellationOrigin = EnsureRootOrAllNativeTechnical;

	type BlacklistOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type CancelProposalOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type VetoOrigin = EnsureNativeTechnicalMember;
	type Slash = Treasury;

	type CooloffPeriod = CooloffPeriod;
	type MaxProposals = MaxProposals;
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
	pub const TreasuryPalletId: PalletId = PalletId(*b"lyartrsy");
	/// Percentage of proposal that most be bonded by the proposer.
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub ProposalBondMinimum: Balance = 5000 * CurrencyId::unit::<Balance>();
	pub ProposalBondMaximum: Balance = 10000 * CurrencyId::unit::<Balance>();
	pub const SpendPeriod: BlockNumber = 7 * DAYS;
	pub const Burn: Permill = Permill::from_percent(0);
	pub const MaxApprovals: u32 = 30;
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl treasury::Config<NativeTreasury> for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type RejectOrigin = EnsureRootOrTwoThirdNativeCouncil;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type MaxApprovals = MaxApprovals;
	type BurnDestination = ();
	type WeightInfo = treasury::weights::SubstrateWeight<Runtime>;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
}

impl sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}
