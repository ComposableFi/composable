// Ensure trapped assets are to claim
// https://github.com/AcalaNetwork/Acala/commit/f40e8f9277fe2fabefd4b51d8d2cfd97f088f3b1#diff-4918885dbae3244dd19ee256ec2d575908d8b599007adc761b8651082c4b3288

// Add barrier and ED tests


// #[test]
// fn transfer_insufficient_amount_should_fail() {
// 	simtest();
// Sibling::execute_with(|| {
//     assert!(matches!(
//         sibling_runtime::XTokens::transfer(
//             sibling_runtime::Origin::signed(alice().into()),
//             CurrencyId::PICA,
//             1_000_000 - 1,
//             Box::new(
//                 MultiLocation::new(
//                     1,
//                     X2(
//                         Junction::Parachain(THIS_PARA_ID),
//                         Junction::AccountId32 { id: bob(), network: NetworkId::Any }
//                     )
//                 )
//                 .into()
//             ),
//             399_600_000_000
//         ),
//         Err(DispatchError::Module(ModuleError { .. }))
//     ));
//     assert_eq!(sibling_runtime::Balances::free_balance(&alice().into()), 200000000000000);
// });

// This::execute_with(|| {
//     assert_eq!(
//         this_runtime::Tokens::free_balance(CurrencyId::PICA, &AccountId::from(bob())),
//         0
//     );
// });
// }



// #[test]
// fn transfer_native_of_this_to_sibling_by_local_id() {
// 	simtest();

// 	Sibling::execute_with(|| {
// 		assert_ok!(this_runtime::AssetsRegistry::update_asset(
// 			RawOrigin::Root.into(),
// 			CurrencyId::PICA,
// 			composable_traits::xcm::assets::XcmAssetLocation(MultiLocation::new(
// 				1,
// 				X1(Parachain(THIS_PARA_ID),)
// 			)),
// 			Some(Rational64::one()),
// 			None,
// 		));
// 	});

// 	This::execute_with(|| {
// 		use this_runtime::*;
// 		let before = Balances::balance(&sibling_account(SIBLING_PARA_ID));

// 		assert_ok!(XTokens::transfer(
// 			Origin::signed(alice().into()),
// 			CurrencyId::PICA,
// 			3 * PICA,
// 			Box::new(
// 				MultiLocation::new(
// 					1,
// 					X2(
// 						Junction::Parachain(SIBLING_PARA_ID),
// 						Junction::AccountId32 { id: bob(), network: NetworkId::Any }
// 					)
// 				)
// 				.into()
// 			),
// 			399_600_000_000
// 		));

// 		let after = Balances::balance(&sibling_account(SIBLING_PARA_ID));
// 		assert_eq!(Balances::free_balance(&alice().into()), 200 * PICA - 3 * PICA);
// 		assert_gt!(after, before);
// 		assert_eq!(after, 3 * PICA);
// 	});

// 	Sibling::execute_with(|| {
// 		let balance =
// 			sibling_runtime::Assets::free_balance(CurrencyId::PICA, &AccountId::from(bob()));
// 		assert_eq_error_rate!(balance, 3 * PICA, (UnitWeightCost::get() * 10) as u128);
// 	});
// }


use crate::{
	assert_lt_by,
	helpers::*,
	kusama_test_net::{KusamaRelay, Sibling, This, PICA, SIBLING_PARA_ID, THIS_PARA_ID},
	prelude::*,
};
use codec::Encode;
use common::{AccountId, Balance};
use composable_traits::{currency::RangeId, rational};

use frame_system::RawOrigin;

use num_traits::Zero;
use orml_traits::currency::MultiCurrency;

use frame_support::{assert_ok, log, weights::constants::WEIGHT_PER_MILLIS};
use primitives::currency::*;
use sp_runtime::{assert_eq_error_rate, traits::AccountIdConversion, MultiAddress};
use xcm::latest::prelude::*;
use xcm_builder::ParentIsPreset;
use xcm_emulator::TestExt;
use xcm_executor::{traits::Convert, XcmExecutor};

use frame_support::traits::fungibles::Inspect as MultiInspect;


#[test]
fn transfer_native_from_relay_enough_for_fee_but_not_enough_for_ed_ends_up_in_treasury() {
	simtest();
	let receiver = charlie();
	let (picasso_treasury, under_ed) = This::execute_with(|| {
		use this_runtime::*;
		let under_ed = under_existential_deposit::<AssetsRegistry>(LocalAssetId::KSM, 3);
		assert_eq!(Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)), 0,);
		(Tokens::free_balance(CurrencyId::KSM, &this_runtime::TreasuryAccount::get()), under_ed)
	});

	KusamaRelay::execute_with(|| {
		use relay_runtime::*;
		let _ = <Balances as frame_support::traits::fungible::Balanced<AccountId>>::deposit(
			&AccountId::from(alice()),
			under_ed * 10000,
		)
		.unwrap();
		assert_ok!(XcmPallet::reserve_transfer_assets(
			Origin::signed(alice().into()),
			Box::new(Parachain(THIS_PARA_ID).into().into()),
			Box::new(Junction::AccountId32 { id: receiver, network: NetworkId::Any }.into().into()),
			Box::new((Here, under_ed).into()),
			0
		));
	});

	This::execute_with(|| {
		use this_runtime::*;
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &AccountId::from(receiver)),
			0,
			"assets did not get to recipient as it is not enough to pay ED"
		);
		assert_eq!(
			Tokens::free_balance(CurrencyId::KSM, &TreasuryAccount::get()),
			under_ed - picasso_treasury
		);
	});
}