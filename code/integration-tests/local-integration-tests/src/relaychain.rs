//! utilities to work with relay chain and XCM transact calls into it

use self::kusama::RelayChainCall;
use crate::prelude::*;
use common::Balance;

use frame_support::RuntimeDebug;
use xcm::latest::{
	Junction::Parachain, Junctions::X1, MultiAsset, MultiLocation, OriginKind,
	WeightLimit::Unlimited, Xcm,
};

use frame_system::Config;
use sp_runtime::traits::StaticLookup;

pub type RelayBalance = u64;

#[derive(Encode, Decode, RuntimeDebug)]
pub enum BalancesCall<T: Config> {
	#[codec(index = 3)]
	TransferKeepAlive(<T::Lookup as StaticLookup>::Source, #[codec(compact)] RelayBalance),
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum UtilityCall<RelayChainCall> {
	#[codec(index = 1)]
	AsDerivative(u16, RelayChainCall),
	#[codec(index = 2)]
	BatchAll(Vec<RelayChainCall>),
}

mod kusama {
	use crate::*;
	use prelude::*;

	use super::{BalancesCall, UtilityCall};
	/// The encoded index corresponds to Kusama's Runtime module configuration.
	/// https://github.com/paritytech/polkadot/blob/main/runtime/kusama/src/lib.rs#L1379
	#[derive(Encode, Decode, RuntimeDebug)]
	pub enum RelayChainCall<T: Config> {
		#[codec(index = 4)]
		Balances(BalancesCall<T>),
		#[codec(index = 24)]
		Utility(Box<UtilityCall<Self>>),
	}
}

/// Transfer Staking currency to another account, disallowing "death".
///  params:
/// - to: The destination for the transfer
/// - amount: The amount of staking currency to be transferred.
#[allow(dead_code)] // for future use in cross chain tests
pub fn balances_transfer_keep_alive<T: Config>(
	to: T::AccountId,
	amount: RelayBalance,
) -> RelayChainCall<T> {
	RelayChainCall::Balances(BalancesCall::TransferKeepAlive(T::Lookup::unlookup(to), amount))
}

/// Execute a call, replacing the `Origin` with a sub-account.
///  params:
/// - call: The call to be executed. Can be nested with `utility_batch_call`
/// - index: The index of sub-account to be used as the new origin.
#[allow(dead_code)] // for future use in cross chain tests
pub fn utility_as_derivative_call<T: Config>(
	call: RelayChainCall<T>,
	index: u16,
) -> RelayChainCall<T> {
	RelayChainCall::Utility(Box::new(UtilityCall::AsDerivative(index, call)))
}

/// Wrap the final calls into the Xcm format.
///  params:
/// - call: The call to be executed
/// - extra_fee: Extra fee (in staking currency) used for buy the `weight` and `debt`.
/// - weight: the weight limit used for XCM.
/// - debt: the weight limit used to process the `call`.
#[allow(dead_code)] // for future use in cross chain tests
pub fn finalize_call_into_xcm_message<T: Config>(
	call: RelayChainCall<T>,
	extra_fee: Balance,
	weight: Weight,
) -> Xcm<()> {
	let asset =
		MultiAsset { id: Concrete(MultiLocation::here()), fun: Fungibility::Fungible(extra_fee) };
	Xcm(vec![
		WithdrawAsset(asset.clone().into()),
		BuyExecution { fees: asset, weight_limit: Unlimited },
		Transact {
			origin_type: OriginKind::SovereignAccount,
			require_weight_at_most: weight,
			call: call.encode().into(),
		},
		DepositAsset {
			assets: All.into(),
			max_assets: u32::max_value(),
			beneficiary: MultiLocation {
				parents: 0,
				interior: X1(Parachain(sibling_runtime::ParachainInfo::parachain_id().into())),
			},
		},
	])
}
