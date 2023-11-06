//! Runtime setup for the governance and democracy only.

use super::*;
use common::governance::native::*;
use frame_support::traits::EitherOf;

pub type NativeCouncilMembership = membership::Instance1;
pub type NativeTechnicalMembership = membership::Instance2;
pub type NativeRelayerMembership = membership::Instance4;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 7 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
	pub const AlarmInterval: BlockNumber = 1;
}

impl membership::Config<NativeCouncilMembership> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type PrimeOrigin = EnsureRoot<AccountId>;
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
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type MaxProposalWeight = MaxProposalWeight;
}

impl membership::Config<NativeTechnicalMembership> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrTwoThirdNativeTechnical;
	type RemoveOrigin = EnsureRootOrTwoThirdNativeTechnical;
	type SwapOrigin = EnsureRootOrTwoThirdNativeTechnical;
	type ResetOrigin = EnsureRootOrTwoThirdNativeTechnical;
	type PrimeOrigin = EnsureRootOrTwoThirdNativeTechnical;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

impl collective::Config<NativeTechnicalCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
	type SetMembersOrigin = EnsureRootOrTwoThirdNativeTechnical;
	type MaxProposalWeight = MaxProposalWeight;
}

impl membership::Config<NativeRelayerMembership> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type PrimeOrigin = EnsureRoot<AccountId>;
	type MembershipInitialized = RelayerCommittee;
	type MembershipChanged = RelayerCommittee;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

impl collective::Config<NativeRelayerMembership> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
	type SetMembersOrigin = GeneralAdminOrRoot;
	type MaxProposalWeight = MaxProposalWeight;
}

pallet_referenda::impl_tracksinfo_get!(TracksInfo, Balance, BlockNumber);
impl pallet_referenda::Config for Runtime {
	type RuntimeCall = RuntimeCall;

	type RuntimeEvent = RuntimeEvent;

	type WeightInfo = pallet_referenda::weights::SubstrateWeight<Self>;

	type Scheduler = Scheduler;

	type Currency = Balances;

	// everyone can submit
	type SubmitOrigin = frame_system::EnsureSigned<AccountId>;
	type CancelOrigin = EitherOf<EnsureRoot<Self::AccountId>, ReferendumCanceller>;
	type KillOrigin = EitherOf<EnsureRoot<Self::AccountId>, ReferendumKiller>;

	type Slash = Treasury;

	type Votes = pallet_conviction_voting::VotesOf<Runtime>;

	type Tally = pallet_conviction_voting::TallyOf<Runtime>;

	type SubmissionDeposit = ConstU128<1_000_000_000_000_000>;

	type MaxQueued = ConstU32<100>;

	type UndecidingTimeout = ConstU32<{ 3 * DAYS }>;

	type AlarmInterval = AlarmInterval;

	type Tracks = TracksInfo;

	type Preimages = Preimage;
}

parameter_types! {
	pub const VoteLockingPeriod: BlockNumber = 1 * DAYS;
}

impl pallet_conviction_voting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_conviction_voting::weights::SubstrateWeight<Self>;
	type Currency = Balances;

	type Polls = Referenda;

	type MaxTurnout = frame_support::traits::TotalIssuanceOf<Balances, Self::AccountId>;

	type MaxVotes = ConstU32<20>;

	type VoteLockingPeriod = VoteLockingPeriod;
}

impl pallet_custom_origins::Config for Runtime {}

use pallet_custom_origins::{ReferendumCanceller, ReferendumKiller};
pub use pallet_custom_origins::WhitelistedCaller;

impl pallet_whitelist::Config for Runtime {
	type WeightInfo = pallet_whitelist::weights::SubstrateWeight<Self>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	#[cfg(not(feature = "fastnet"))]
	type WhitelistOrigin = EnsureRootOrOneThirdNativeCouncilOrTechnical;
	#[cfg(feature = "fastnet")]
	type WhitelistOrigin = EnsureRootOrOneSixthNativeCouncilOrTechnical;
	type DispatchWhitelistedOrigin = EitherOf<EnsureRoot<Self::AccountId>, WhitelistedCaller>;
	type Preimages = Preimage;
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
	type ApproveOrigin = EnsureRootOrOneSixthNativeCouncil;
	#[cfg(feature = "testnet")]
	type RejectOrigin = EnsureRootOrOneSixthNativeCouncil;

	#[cfg(not(feature = "testnet"))]
	type ApproveOrigin = EnsureRootOrTwoThirdNativeCouncil;
	#[cfg(not(feature = "testnet"))]
	type RejectOrigin = EnsureRootOrMoreThenHalfNativeCouncil;
}

impl sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = ();
}