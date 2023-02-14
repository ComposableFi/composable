//! utilities to work with relay chain and XCM transact calls into it
pub use kusama::*;

mod kusama {
	use crate::*;
	use common::Balance;
	use prelude::*;

	/// Wrap the final calls into the Xcm format.
	///  params:
	/// - call: The call to be executed
	/// - extra_fee: Extra fee (in staking currency) used for buy the `weight` and `debt`.
	/// - weight: the weight limit used for XCM.
	/// - debt: the weight limit used to process the `call`.
	#[allow(dead_code)] // for future use in cross chain tests
	pub fn finalize_call_into_xcm_message<T: Config>(
		call: relay_runtime::RuntimeCall,
		extra_fee: Balance,
		weight: Weight,
		parachain: ParaId,
	) -> Xcm<()> {
		let asset = MultiAsset {
			id: Concrete(MultiLocation::here()),
			fun: Fungibility::Fungible(extra_fee),
		};
		Xcm(vec![
			WithdrawAsset(asset.clone().into()),
			BuyExecution { fees: asset, weight_limit: Unlimited },
			Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: weight.ref_time(),
				call: call.encode().into(),
			},
			DepositAsset {
				assets: All.into(),
				max_assets: u32::max_value(),
				beneficiary: MultiLocation {
					parents: 0,
					interior: X1(Parachain(parachain.into())),
				},
			},
		])
	}
}
