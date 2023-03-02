use crate::{
	prelude::*, weights, ReleaseCommittee, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use common::{
	governance::native::{
		EnsureRootOrHalfNativeTechnical, EnsureRootOrOneThirdNativeTechnical, ReleaseCollective,
	},
	AccountId, MaxStringSize, HOURS,
};
use composable_traits::account_proxy::ProxyType;
use frame_support::traits::{EitherOfDiverse, InstanceFilter};
use frame_system::EnsureRoot;
use sp_core::ConstU32;

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::Governance => matches!(
				c,
				RuntimeCall::Democracy(..) |
					RuntimeCall::Council(..) |
					RuntimeCall::TechnicalCommittee(..) |
					RuntimeCall::Treasury(..) |
					RuntimeCall::Utility(..)
			),
			ProxyType::CancelProxy => {
				matches!(c, RuntimeCall::Proxy(proxy::Call::reject_announcement { .. }))
			},
			ProxyType::Bridge => matches!(
				c,
				RuntimeCall::Ibc(..) |
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
	type EnableOrigin = EnsureRootOrHalfNativeTechnical;
	type DisableOrigin = EnsureRootOrOneThirdNativeTechnical;
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
