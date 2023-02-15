use crate::{
	assert_lt_by,
	helpers::*,
	kusama_test_net::{KusamaRelay, This, THIS_PARA_ID},
	prelude::*,
};

use common::AccountId;

#[cfg(feature = "dali")]
use frame_system::RawOrigin;

use orml_traits::currency::MultiCurrency;

use frame_support::{assert_ok, log};

#[cfg(feature = "dali")]
use xcm::VersionedXcm;

use xcm_emulator::TestExt;

#[cfg(feature = "dali")]
#[test]
fn transfer_from_relay_native_from_this_to_relay_chain_raw() {
	simtest();
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;

	mint_relay_native_on_parachain(
		transfer_amount * 2,
		&this_runtime::TreasuryAccount::get(),
		THIS_PARA_ID,
	);

	KusamaRelay::execute_with(|| {
		assert_eq!(relay_runtime::Balances::balance(&AccountId::from(bob())), 0);
	});

	log::info!(target: "bdd", "Root transfers native from this to Relay onto Bob account");
	let message = Xcm(vec![
		WithdrawAsset(MultiAssets::from(vec![MultiAsset {
			id: Concrete(MultiLocation { parents: 1, interior: Here }),
			fun: Fungible(transfer_amount),
		}])),
		InitiateReserveWithdraw {
			assets: Wild(All),
			reserve: (1, Here).into(),
			xcm: Xcm(vec![
				BuyExecution {
					fees: MultiAsset { id: Concrete(Here.into()), fun: Fungible(transfer_amount) },

					weight_limit: Limited(limit),
				},
				DepositAsset {
					assets: Wild(All),
					max_assets: 1,
					beneficiary: (0, X1(AccountId32 { network: NetworkId::Any, id: bob() })).into(),
				},
			]),
		},
	]);

	This::execute_with(|| {
		use this_runtime::*;
		let before = Assets::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get());
		assert_gt!(before, transfer_amount);
		let transferred =
			RelayerXcm::execute(RawOrigin::Root.into(), Box::new(VersionedXcm::V2(message)), limit);

		assert_ok!(transferred);

		let after = Assets::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get());

		assert_eq!(before - after, transfer_amount);
	});

	KusamaRelay::execute_with(|| {
		log::info!(target: "bdd", "Then bob has amount on Relay");
		assert_lt_by!(
			relay_runtime::Balances::balance(&AccountId::from(bob())),
			transfer_amount,
			ORDER_OF_FEE_ESTIMATE_ERROR * (THIS_CHAIN_NATIVE_FEE + RELAY_CHAIN_NATIVE_FEE) +
				ORDER_OF_FEE_ESTIMATE_ERROR * limit as u128
		)
	});
}

#[test]
fn transfer_from_relay_native_from_this_to_relay_chain_by_local_id() {
	simtest();
	let transfer_amount = 3 * RELAY_NATIVE::ONE;
	let limit = 4_600_000_000;

	mint_relay_native_on_parachain(transfer_amount * 2, &AccountId::from(alice()), THIS_PARA_ID);

	KusamaRelay::execute_with(|| {
		assert_eq!(relay_runtime::Balances::balance(&AccountId::from(bob())), 0);
	});

	log::info!(target: "bdd", "Alice transfers native from this to Relay");
	This::execute_with(|| {
		let before = this_runtime::Assets::free_balance(CurrencyId::KSM, &alice().into());
		let transferred = this_runtime::XTokens::transfer(
			this_runtime::RuntimeOrigin::signed(alice().into()),
			CurrencyId::KSM,
			transfer_amount,
			Box::new(
				MultiLocation::new(
					1,
					X1(Junction::AccountId32 { id: bob(), network: NetworkId::Any }),
				)
				.into(),
			),
			Limited(limit),
		);

		assert_ok!(transferred);

		let after = this_runtime::Assets::free_balance(CurrencyId::KSM, &alice().into());

		assert_eq!(before - after, transfer_amount);
	});

	KusamaRelay::execute_with(|| {
		assert_lt_by!(
			relay_runtime::Balances::balance(&AccountId::from(bob())),
			transfer_amount,
			ORDER_OF_FEE_ESTIMATE_ERROR * (THIS_CHAIN_NATIVE_FEE + RELAY_CHAIN_NATIVE_FEE) +
				ORDER_OF_FEE_ESTIMATE_ERROR * limit as u128
		)
	});
}
