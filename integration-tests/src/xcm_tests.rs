//! Basic simple XCM setup and usage sanity checks on low level (not involving too much of
//! Cumulus/ORML abstractions) Partially ported from articles and examples of https://github.com/paritytech/polkadot/blob/master/xcm/xcm-simulator/example/src/lib.rs

use crate::{
	env_logger_init,
	kusama_test_net::{KusamaNetwork, *},
};
use codec::Encode;
use common::AccountId;
use composable_traits::assets::RemoteAssetRegistry;
use cumulus_primitives_core::ParaId;
use kusama_runtime::{KusamaNetwork as KusamaNetworkId, *};
use orml_traits::currency::MultiCurrency;
use picasso_runtime as dali_runtime;
use primitives::currency::CurrencyId;
use sp_runtime::traits::AccountIdConversion;
use support::assert_ok;
use xcm::latest::prelude::*;
use xcm_emulator::TestExt;
use xcm_executor::XcmExecutor;

// Helper function for forming buy execution message
fn buy_execution<C>(fees: impl Into<MultiAsset>) -> Instruction<C> {
	BuyExecution { fees: fees.into(), weight_limit: Unlimited }
}

pub fn para_account_id(id: u32) -> AccountId {
	ParaId::from(id).into_account()
}

/// as per documentation is way to throw exception with specific error code as Trap, and that should
/// be handled
#[test]
fn throw_exception() {
	Picasso::execute_with(|| {
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

#[test]
fn initiate_reserver_withdraw_on_relay() {
	env_logger_init();
	Picasso::execute_with(|| {
		let here = MultiLocation::new(0, Here);
		let asset_id =
			MultiLocation::new(0, X2(Parachain(PICASSO_PARA_ID), GeneralKey(0u128.encode())));
		let asset_id = AssetId::Concrete(asset_id);
		let asset_id = MultiAsset { fun: Fungible(42), id: asset_id };
		let xcm = Xcm(vec![
			WithdrawAsset(asset_id.into()),
			InitiateReserveWithdraw {
				assets: All.into(),
				reserve: Parent.into(),
				xcm: Xcm(vec![]),
			},
		]);

		let executed =
			XcmExecutor::<XcmConfig>::execute_xcm_in_credit(here, xcm, 10000000000, 10000000000);
		assert!(matches!(Outcome::Complete(42), executed));
	});
}

#[test]
fn send_remark() {
	KusamaNetwork::reset();

	let remark = picasso_runtime::Call::System(
		frame_system::Call::<picasso_runtime::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	Picasso::execute_with(|| {
		assert_ok!(picasso_runtime::RelayerXcm::send_xcm(
			Here,
			(Parent, Parachain(DALI_PARA_ID)),
			Xcm(vec![Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: 40000 as u64,
				call: remark.encode().into(),
			}]),
		));
	});

	Dali::execute_with(|| {
		use dali_runtime::{Event, System};
		assert!(System::events()
			.iter()
			.any(|r| matches!(r.event, Event::System(frame_system::Event::Remarked(_, _)))));
	});
}

/// Scenario:
/// A parachain wants to be notified that a transfer worked correctly.
/// It sends a `QueryHolding` after the deposit to get notified on success.
///
/// Asserts that the balances are updated correctly and the expected XCM is sent.
#[test]
fn query_holding() {
	KusamaNetwork::reset();
	env_logger_init();

	let send_amount = 10;
	let query_id_set = 1234;

	// Send a message which fully succeeds on the relay chain
	Picasso::execute_with(|| {
		let message = Xcm(vec![
			WithdrawAsset((Here, send_amount).into()),
			buy_execution((Here, send_amount)),
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Parachain(DALI_PARA_ID).into(),
			},
			QueryHolding {
				query_id: query_id_set,
				dest: Parachain(PICASSO_PARA_ID).into(),
				assets: All.into(),
				max_response_weight: 1_000_000_000,
			},
		]);
		// Send withdraw and deposit with query holding
		assert_ok!(picasso_runtime::RelayerXcm::send_xcm(Here, Parent, message.clone(),));
	});

	// Check that transfer was executed
	KusamaRelay::execute_with(|| {
		// Withdraw executed
		assert_eq!(
			kusama_runtime::Balances::free_balance(para_account_id(PICASSO_PARA_ID)),
			PICASSO_RELAY_BALANCE - send_amount
		);
	});
}
