use crate::{
	prelude::*, weights, Balances, ReleaseCommittee, Runtime, RuntimeCall, RuntimeEvent,
	RuntimeOrigin,
};
use common::{
	fees::NativeExistentialDeposit, governance::native::ReleaseCollective, AccountId,
	MaxStringSize, HOURS,
};
use composable_traits::account_proxy::ProxyType;
use cumulus_primitives_core::relay_chain::BlakeTwo256;
use frame_support::traits::{Contains, EitherOfDiverse, InstanceFilter};
use frame_system::EnsureRoot;

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::Governance => matches!(
				c,
				RuntimeCall::Democracy(..) |
					RuntimeCall::Council(..) |
					//RuntimeCall::TechnicalCommittee(..) |
					RuntimeCall::Treasury(..) |
					RuntimeCall::Utility(..)
			),
			ProxyType::CancelProxy => {
				matches!(c, RuntimeCall::Proxy(pallet_proxy::Call::reject_announcement { .. }))
			},
			ProxyType::Bridge => matches!(
				c,
				//RuntimeCall::Ibc(..) |
				RuntimeCall::CumulusXcm(..) |
					RuntimeCall::DmpQueue(..) |
					RuntimeCall::UnknownTokens(..) |
					RuntimeCall::XcmpQueue(..) |
					RuntimeCall::RelayerXcm(..)
			),
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

/// The calls we permit to be executed by extrinsics
pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		!(call_filter::Pallet::<Runtime>::contains(call) ||
			matches!(
				call,
				RuntimeCall::Tokens(_) | RuntimeCall::Indices(_) | RuntimeCall::Treasury(_)
			))
	}
}

impl call_filter::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type EnableOrigin = EnsureRoot<AccountId>; // EnsureRootOrHalfNativeTechnical;
	type DisableOrigin = EnsureRoot<AccountId>; // EnsureRootOrOneThirdNativeTechnical;
	type Hook = ();
	type WeightInfo = ();
	type MaxStringSize = MaxStringSize;
}

impl collective::Config<ReleaseCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = ConstU32<{ HOURS }>;
	type MaxProposals = ConstU32<4>;
	type MaxMembers = ConstU32<100>;
	type DefaultVote = collective::PrimeDefaultVote;
	type WeightInfo = weights::collective::WeightInfo<Runtime>;
}

pub type EnsureRootOrTwoThirds<T> =
	EitherOfDiverse<EnsureRoot<AccountId>, collective::EnsureProportionAtLeast<AccountId, T, 2, 3>>;

impl membership::Config<membership::Instance3> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrTwoThirds<ReleaseCollective>;
	type RemoveOrigin = EnsureRootOrTwoThirds<ReleaseCollective>;
	type SwapOrigin = EnsureRootOrTwoThirds<ReleaseCollective>;
	type ResetOrigin = EnsureRootOrTwoThirds<ReleaseCollective>;
	type PrimeOrigin = EnsureRootOrTwoThirds<ReleaseCollective>;
	type MembershipInitialized = ReleaseCommittee;
	type MembershipChanged = ReleaseCommittee;
	type MaxMembers = ConstU32<100>;
	type WeightInfo = weights::membership::WeightInfo<Runtime>;
}

// Minimal deposit required to place a proxy announcement as per native existential deposit.
pub type ProxyPrice = NativeExistentialDeposit;

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = composable_traits::account_proxy::ProxyType;
	type ProxyDepositBase = ProxyPrice;
	type ProxyDepositFactor = ProxyPrice;
	type MaxProxies = ConstU32<4>;
	type MaxPending = ConstU32<32>;
	type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ProxyPrice;
	type AnnouncementDepositFactor = ProxyPrice;
}
