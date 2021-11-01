use std::convert::TryInto;

use codec::{Decode, Encode};
use frame_support::{
	construct_runtime, parameter_types,
	traits::{Everything, Nothing},
	weights::Weight,
};
use frame_system::Origin;
use polkadot_parachain::primitives::Id as ParaId;
use polkadot_runtime_parachains::{configuration, origin, shared, ump};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{AccountIdConversion, IdentityLookup},
};
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowUnpaidExecutionFrom, ChildParachainAsNative,
	ChildParachainConvertsVia, ChildSystemParachainAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, FixedRateOfFungible, FixedWeightBounds, IsConcrete,
	LocationInverter, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
};
use xcm_executor::{Config, XcmExecutor};
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

use crate::parachain::AccountId;

use super::*;

use frame_support::assert_ok;
use xcm::{latest::prelude::*, v2::*};
use xcm_simulator::TestExt;

use num_enum::{IntoPrimitive, TryFromPrimitive};

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

	let remark =
		relay_chain::Call::System(frame_system::Call::<relay_chain::Runtime>::remark_with_event {
			remark: vec![1, 2, 3],
		});
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

	HydraDxParachain::execute_with(|| {
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
			parachain::Balances::free_balance(&para_account_id(COMPOSABLE)),
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
				beneficiary: Parachain(HYDRADX).into(),
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

/// fungible
#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	PartialOrd,
	Ord,
	TypeInfo,
	IntoPrimitive,
	TryFromPrimitive,
)]
#[repr(u8)] // we can make it to be u16, but that would prevent optimize later with processor table jump, so can
			// consider 9 bit so. or preclude optimization and make 16 bit.
pub enum KnownAssetId {
	#[num_enum(default)]
	PICA = 1,
	LAYR = 2,
	CROWD_LOAN = 3,
	BTC = 4,
	ETH = 5,
	USDT = 6,
	SOL = 42,
}

/// are used to allow only certain operations for these
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, PartialOrd, Ord, TypeInfo)]
pub struct MappedId(u128);

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, PartialOrd, Ord, TypeInfo)]
pub struct Liquid(u128);

/// for those which were bridged
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, PartialOrd, Ord, TypeInfo)]
pub struct Bridged(MappedId);

/// must implement custom Encode, Decode so it validates numeric properties and compresses data into
/// one 128 bit, and IntoPrimitive and TryFromPrimitive fungible
#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, PartialOrd, Ord, TypeInfo)]
pub enum MultiAssetId {
	LocalKnown(KnownAssetId),
	// starts from 256 to u128/4
	Local(u128),
	/// to other parachains starts from u128/4 to u128/2, can always be mapped to ParaId and
	/// ExternalAssetId
	Mapped(MappedId),
	/// rest of space as can make infinite derives :)
	Liquid(Liquid),
}

impl KnownAssetId {
	pub fn new(value: u8) -> Option<KnownAssetId> {
		KnownAssetId::try_from_primitive(value).ok()
	}

	// that it, we cannot create it here, because need need storage access
}

struct CurrencyMetadata {
	id: MultiAssetId,
	/// any Liquid token should have decimals of parent, so if you have can find decimals from
	/// which it derived
	decimals: u8,
}

pub trait AssetIds {
	/// will return option if not found
	fn from_number(value: u128) -> Option<MultiAssetId>;
	fn diluted(value: Liquid) -> Option<MultiAssetId>;
	fn as_on_target(value: MappedId) -> Option<(ParaId, u128)>;
}

pub struct MockAssetIds;

impl AssetIds for MockAssetIds {
	fn from_number(value: u128) -> Option<MultiAssetId> {
		let value: u8 = value.try_into().ok()?;
		let value = KnownAssetId::new(value)?;
		Some(MultiAssetId::LocalKnown(value))
	}

	fn diluted(value: Liquid) -> Option<MultiAssetId> {
		todo!()
	}

	fn as_on_target(value: MappedId) -> Option<(ParaId, u128)> {
		todo!()
	}
}

#[test]
fn full_trust_teleport_and_dex() {
	MockNet::reset();

	let account_with_some_amount =
		Junction::AccountId32 { id: BTC_ACCOUNT.into(), network: NetworkId::Any };
	let mut location = MultiLocation::here();
	location
		.append_with(Junctions::X2(Junction::Parachain(COMPOSABLE), account_with_some_amount))
		.unwrap();
	let from_id = AssetId::Concrete(location);
	let from = MultiAsset { id: from_id, fun: Fungible(42) };
	let mut assets = MultiAssets::new();
	assets.push(from);
	let assets_sell = MultiAssetFilter::Definite(assets);

	let account_with_some_amount =
		Junction::AccountId32 { id: USDT.into(), network: NetworkId::Any };
	let mut location = MultiLocation::here();
	location
		.append_with(Junctions::X2(Junction::Parachain(COMPOSABLE), account_with_some_amount))
		.unwrap();
	let from_id = AssetId::Concrete(location);
	let from = MultiAsset { id: from_id, fun: Fungible(42 * 10000) };
	let mut assets_buy = MultiAssets::new();
	assets_buy.push(from);

	let exchange = Instruction::ExchangeAsset { give: assets_sell.clone(), receive: assets_buy };

	let teleport_and_dex = Xcm(vec![Instruction::InitiateTeleport {
		assets: assets_sell,
		dest: Parachain(HYDRADX).into(),
		xcm: Xcm(vec![exchange]),
	}]);

	// TODO: avoid here jump via relay (need to change simulator ) TODO: create github issue for
	// polka about
	ComposableParachain::execute_with(|| {
		assert_ok!(ParachainPalletXcm::send_xcm(
			Here,
			(Parent, Parachain(HYDRADX)),
			teleport_and_dex.clone()
		));
	});

	HydraDxParachain::execute_with(|| {
		// TODO: setup currency pallet as example
	});
}



#[test]
#[ignore = "how to share extrinsic Call metadata without sharing pallet?"]
fn teleport_and_transaction_via_auction() {
	MockNet::reset();

	let alice = AccountId::default();
	let origin = crate::parachain::Origin::signed(alice.into());
	ComposableParachain::execute_with(|| {
		let result = ParachainContracts::ping(
			origin,
				42,
				Vec::new(),
			);

		assert_ok!(result);
	});
}
