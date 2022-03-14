//! Basic simple XCM setup and usage sanity checks on low level (not involving too much of
//! Cumulus/ORML abstractions) Partially ported from articles and examples of https://github.com/paritytech/polkadot/blob/master/xcm/xcm-simulator/example/src/lib.rs
//! Cannot port QueryHold because it is not implemented

use crate::{helpers::*, kusama_test_net::*, prelude::*};
use composable_traits::assets::{RemoteAssetRegistry, XcmAssetLocation};

use primitives::currency::CurrencyId;
use support::assert_ok;
use xcm::latest::prelude::*;
use xcm_emulator::TestExt;
use xcm_executor::XcmExecutor;

// Helper function for forming buy execution message
fn buy_execution<C>(fees: impl Into<MultiAsset>) -> Instruction<C> {
	BuyExecution { fees: fees.into(), weight_limit: Unlimited }
}

/// as per documentation is way to throw exception with specific error code as Trap, and that should
/// be handled
#[test]
fn throw_exception() {
	This::execute_with(|| {
		let here = MultiLocation::new(0, Here);
		let xcm = Xcm(vec![Trap(42)]);

		let executed =
			XcmExecutor::<XcmConfig>::execute_xcm_in_credit(here, xcm, 1000000000, 1000000000);

		match executed {
			Outcome::Incomplete(_, error) => assert_eq!(XcmError::Trap(42), error),
			_ => unreachable!(),
		}
	});
}

/// this is low levl
#[test]
fn initiate_reserver_withdraw_on_relay() {
	simtest();
	This::execute_with(|| {
		assert_ok!(<this_runtime::AssetsRegistry as RemoteAssetRegistry>::set_location(
			CurrencyId::KSM,
			XcmAssetLocation::RELAY_NATIVE,
		));

		let origin = MultiLocation::new(
			0,
			X1(AccountId32 {
				id: crate::kusama_test_net::ALICE,
				// it assumes that above account public key was used on all networks by bob, not
				// mapping, so it will match any
				network: NetworkId::Any,
			}),
		);
		let asset_id = AssetId::Concrete(MultiLocation::parent());
		let assets = MultiAsset { fun: Fungible(42), id: asset_id };
		let xcm = Xcm(vec![
			WithdrawAsset(assets.into()),
			InitiateReserveWithdraw {
				assets: All.into(),
				reserve: Parent.into(),
				xcm: Xcm(vec![]),
			},
		]);
		let units = xcm.len() as u64;

		let executed = <this_runtime::Runtime as cumulus_pallet_xcmp_queue::Config>::XcmExecutor::execute_xcm_in_credit(origin, xcm, 10000000000, 10000000000);

		match executed {
			Outcome::Complete(weight) if weight == UnitWeightCost::get() * units => {},
			_ => unreachable!("{:?}", executed),
		}
	});
}

#[test]
fn send_remark() {
	simtest();
	let remark = this_runtime::Call::System(
		frame_system::Call::<this_runtime::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	let execution = (UnitWeightCost::get() * 5) as u128;
	This::execute_with(|| {
		assert_ok!(this_runtime::RelayerXcm::send_xcm(
			Here,
			(Parent, Parachain(SIBLING_PARA_ID)),
			Xcm(vec![
				ReserveAssetDeposited((Parent, execution).into()),
				BuyExecution { fees: (Parent, execution).into(), weight_limit: Unlimited },
				Transact {
					origin_type: OriginKind::SovereignAccount,
					require_weight_at_most: execution as u64,
					call: remark.encode().into(),
				}
			]),
		));
	});

	Sibling::execute_with(|| {
		use sibling_runtime::{Event, System};
		assert!(System::events().iter().any(|r| matches!(
			r.event,
			Event::System(frame_system::Event::Remarked { sender: _, hash: _ })
		)));
	});
}

#[test]
fn withdraw_and_deposit_back() {
	simtest();
	let send_amount = 10;

	This::execute_with(|| {
		let message = Xcm(vec![
			WithdrawAsset((Here, send_amount).into()),
			buy_execution((Here, send_amount)),
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Parachain(THIS_PARA_ID).into(),
			},
		]);
		assert_ok!(this_runtime::RelayerXcm::send_xcm(Here, Parent, message,));
	});

	KusamaRelay::execute_with(|| {
		assert_eq!(
			kusama_runtime::Balances::free_balance(para_account_id(THIS_PARA_ID)),
			PICASSO_RELAY_BALANCE - send_amount
		);
	});
}

#[test]
fn location_of_deposit_asset() {}
