//! Runtime setup for the governance and democracy only.

use super::*;
use common::governance::native::*;
use frame_support::traits::LockIdentifier;

pub type NativeDemocracy = democracy::Instance1;

pub type NativeCouncilMembership = membership::Instance1;
pub type NativeTechnicalMembership = membership::Instance2;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

impl membership::Config<NativeCouncilMembership> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrHalfNativeCouncil;
	type RemoveOrigin = EnsureRootOrHalfNativeCouncil;
	type SwapOrigin = EnsureRootOrHalfNativeCouncil;
	type ResetOrigin = EnsureRootOrHalfNativeCouncil;
	type PrimeOrigin = EnsureRootOrHalfNativeCouncil;
	type MembershipInitialized = Council;
	type MembershipChanged = Council;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

impl collective::Config<NativeCouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
}

impl membership::Config<NativeTechnicalMembership> for Runtime {
	type Event = Event;

	type AddOrigin = EnsureRootOrHalfNativeCouncilOrTechnical;
	type RemoveOrigin = EnsureRootOrHalfNativeCouncilOrTechnical;
	type SwapOrigin = EnsureRootOrHalfNativeCouncilOrTechnical;
	type ResetOrigin = EnsureRootOrHalfNativeCouncilOrTechnical;
	type PrimeOrigin = EnsureRootOrHalfNativeCouncilOrTechnical;

	type MembershipInitialized = TechnicalCollective;
	type MembershipChanged = TechnicalCollective;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

impl collective::Config<NativeTechnicalMembership> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
}

// NOTE: this is for testing runtime to fast track
parameter_types! {
	pub const CooloffPeriod: BlockNumber = 7 * DAYS;

	pub const LaunchPeriod: BlockNumber = HOURS;
	pub const VotingPeriod: BlockNumber = 5 * DAYS;
	pub const FastTrackVotingPeriod: BlockNumber = HOURS;
	pub MinimumDeposit: Balance = 100 * CurrencyId::unit::<Balance>();
	pub const EnactmentPeriod: BlockNumber = HOURS;
	// Note that Kusama uses 10 millis, however KSM is significantly more expensive
	// https://github.com/paritytech/polkadot/blob/dc784f9b47e4681897cfd477b4f0760330875a87/runtime/kusama/src/lib.rs#L237
	// so we increase it by a factor 10. This might still be on the low side.
	pub PreimageByteDeposit: Balance = CurrencyId::milli::<u128>() * 100_u128;
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
	pub const DemocracyId: LockIdentifier = *b"democrac";
	pub RootOrigin: Origin = frame_system::RawOrigin::Root.into();
}

impl democracy::Config<NativeDemocracy> for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod;
	type MinimumDeposit = MinimumDeposit;

	type ExternalOrigin = EnsureRootOrHalfNativeCouncil;
	type ExternalMajorityOrigin = EnsureRootOrHalfNativeCouncil;
	type ExternalDefaultOrigin = EnsureRootOrHalfNativeCouncil;

	type FastTrackOrigin = EnsureRootOrHalfNativeTechnical;
	type InstantOrigin = EnsureRootOrHalfNativeTechnical;
	type InstantAllowed = InstantAllowed;

	type FastTrackVotingPeriod = FastTrackVotingPeriod;

	type CancellationOrigin = EnsureRootOrAllNativeTechnical;

	type BlacklistOrigin = EnsureRootOrHalfNativeCouncil;
	type CancelProposalOrigin = EnsureRootOrHalfNativeCouncil;
	type VetoOrigin = EnsureNativeTechnicalMember;
	type OperationalPreimageOrigin = EnsureNativeCouncilMember;
	type Slash = Treasury;

	type CooloffPeriod = CooloffPeriod;
	type MaxProposals = MaxProposals;
	type MaxVotes = MaxVotes;
	type PalletsOrigin = OriginCaller;

	type PreimageByteDeposit = PreimageByteDeposit;
	type Scheduler = Scheduler;
	type WeightInfo = weights::democracy::WeightInfo<Runtime>;
	type EnsureRoot = EnsureRoot<AccountId>;
	type DemocracyId = DemocracyId;
	type Origin = Origin;
	type ProtocolRoot = RootOrigin;
}

// NOTE: making it multi via module_cdp_treasury seems fails other pallets
impl treasury::Config<NativeTreasury> for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = EnsureRootOrHalfNativeCouncil;
	type RejectOrigin = EnsureRootOrHalfNativeCouncil;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type MaxApprovals = MaxApprovals;
	type BurnDestination = ();
	type WeightInfo = weights::treasury::WeightInfo<Runtime>;
	// TODO: add bounties?
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
}

impl governance_registry::Config for Runtime {
	type Event = Event;
	type AssetId = CurrencyId;
	type WeightInfo = governance_registry::weights::SubstrateWeight<Runtime>;
}
