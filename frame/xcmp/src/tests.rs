use frame_support::{
	construct_runtime, parameter_types,
	traits::{Everything, Nothing},
	weights::Weight,
};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup,};

use polkadot_parachain::primitives::Id as ParaId;
use polkadot_runtime_parachains::{configuration, origin, shared, ump};
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowUnpaidExecutionFrom, ChildParachainAsNative,
	ChildParachainConvertsVia, ChildSystemParachainAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, FixedRateOfFungible, FixedWeightBounds, IsConcrete,
	LocationInverter, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
};
use xcm_executor::{Config, XcmExecutor};
use sp_runtime::traits::AccountIdConversion;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};


use super::*;

use codec::Encode;
use frame_support::assert_ok;
use xcm::latest::prelude::*;
use xcm_simulator::TestExt;


// Helper function for forming buy execution message
fn buy_execution<C>(fees: impl Into<MultiAsset>) -> Instruction<C> {
	BuyExecution { fees: fees.into(), weight_limit: Unlimited }
}

#[test]
fn dmp_from_relay_to_composable() {
	MockNet::reset();

	let remark =
		parachain::Call::System(frame_system::Call::<parachain::Runtime>::remark_with_event {
			remark: vec![1, 2, 3],
		});
	Relay::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::send_xcm(
			Here,
			Parachain(COMPOSABLE),
			Xcm(vec![Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: INITIAL_BALANCE as u64,
				call: remark.encode().into(),
			}]),
		));
	});

	ComposableParachain::execute_with(|| {
		use parachain::{Event, System};
		assert!(System::events()
			.iter()
			.any(|r| matches!(r.event, Event::System(frame_system::Event::Remarked(_, _)))));
	});
}

#[test]
fn ump() {
	MockNet::reset();

	let remark = relay_chain::Call::System(
		frame_system::Call::<relay_chain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	ComposableParachain::execute_with(|| {
		assert_ok!(ParachainPalletXcm::send_xcm(
			Here,
			Parent,
			Xcm(vec![Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: INITIAL_BALANCE as u64,
				call: remark.encode().into(),
			}]),
		));
	});

	Relay::execute_with(|| {
		use relay_chain::{Event, System};
		assert!(System::events()
			.iter()
			.any(|r| matches!(r.event, Event::System(frame_system::Event::Remarked(_, _)))));
	});
}

#[test]
fn xcmp_via_relay() {
	MockNet::reset();

	let remark =
		parachain::Call::System(frame_system::Call::<parachain::Runtime>::remark_with_event {
			remark: vec![1, 2, 3],
		});
	ComposableParachain::execute_with(|| {
		assert_ok!(ParachainPalletXcm::send_xcm(
			Here,
			(Parent, Parachain(HYDRADX)),
			Xcm(vec![Transact {
				origin_type: OriginKind::SovereignAccount,
				require_weight_at_most: INITIAL_BALANCE as u64,
				call: remark.encode().into(),
			}]),
		));
	});

	HydraDx::execute_with(|| {
		use parachain::{Event, System};
		assert!(System::events()
			.iter()
			.any(|r| matches!(r.event, Event::System(frame_system::Event::Remarked(_, _)))));
	});
}

#[test]
fn reserve_transfer_in_low_trust() {
	MockNet::reset();

	let withdraw_amount = 123;

	Relay::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::reserve_transfer_assets(
			relay_chain::Origin::signed(ALICE),
			Box::new(X1(Parachain(COMPOSABLE)).into().into()),
			Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
			Box::new((Here, withdraw_amount).into()),
			0,
		));
		assert_eq!(
			parachain::Balances::free_balance(&para_account_id(1)),
			INITIAL_BALANCE + withdraw_amount
		);
	});

	ComposableParachain::execute_with(|| {
		// free execution, full amount received
		assert_eq!(
			pallet_balances::Pallet::<parachain::Runtime>::free_balance(&ALICE),
			INITIAL_BALANCE + withdraw_amount
		);
	});
}




/// Scenario:
/// A parachain transfers funds on the relay chain to another parachain account.
///
/// Asserts that the parachain accounts are updated as expected.
#[test]
fn withdraw_and_deposit() {
	MockNet::reset();

	let send_amount = 10;

	ComposableParachain::execute_with(|| {
		let message = Xcm(vec![
			WithdrawAsset((Here, send_amount).into()),
			buy_execution((Here, send_amount)),
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Parachain(2).into(),
			},
		]);
		// Send withdraw and deposit
		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
	});

	Relay::execute_with(|| {
		assert_eq!(
			relay_chain::Balances::free_balance(para_account_id(COMPOSABLE)),
			INITIAL_BALANCE - send_amount
		);
		assert_eq!(relay_chain::Balances::free_balance(para_account_id(HYDRADX)), send_amount);
	});
}

/// Scenario:
/// A parachain wants to be notified that a transfer worked correctly.
/// It sends a `QueryHolding` after the deposit to get notified on success.
///
/// Asserts that the balances are updated correctly and the expected XCM is sent.
#[test]
fn query_holding() {
	MockNet::reset();

	let send_amount = 10;
	let query_id_set = 1234;

	// Send a message which fully succeeds on the relay chain
	ComposableParachain::execute_with(|| {
		let message = Xcm(vec![
			WithdrawAsset((Here, send_amount).into()),
			buy_execution((Here, send_amount)),
			DepositAsset {
				assets: All.into(),
				max_assets: 1,
				beneficiary: Parachain(HYDRADX).into(),
			},
			QueryHolding {
				query_id: query_id_set,
				dest: Parachain(COMPOSABLE).into(),
				assets: All.into(),
				max_response_weight: 1_000_000_000,
			},
		]);
		// Send withdraw and deposit with query holding
		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
	});

	// Check that transfer was executed
	Relay::execute_with(|| {
		// Withdraw executed
		assert_eq!(
			relay_chain::Balances::free_balance(para_account_id(COMPOSABLE)),
			INITIAL_BALANCE - send_amount
		);
		// Deposit executed
		assert_eq!(relay_chain::Balances::free_balance(para_account_id(HYDRADX)), send_amount);
	});

	// Check that QueryResponse message was received
	ComposableParachain::execute_with(|| {
		assert_eq!(
			parachain::MsgQueue::received_dmp(),
			vec![Xcm(vec![QueryResponse {
				query_id: query_id_set,
				response: Response::Assets(MultiAssets::new()),
				max_weight: 1_000_000_000,
			}])],
		);
	});
}

#[test]
fn teleport() {
	MockNet::reset();
	let sell_asset = MultiAsset::new();
	let mut assets = MultiAssets::new();
	assets.push(sell_asset);
	let x =  MultiAssetFilter::Definite(assets);
	Instruction::
	let teleportAndAuction = Instruction::InitiateTeleport {
		assets:x ,
		dest: Parachain(HYDRADX).into(),
		xcm : Xcm(vec![])
	};
	let message = Xcm(
		vec![

		]
	);
}
