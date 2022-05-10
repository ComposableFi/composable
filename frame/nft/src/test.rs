use codec::Encode;
use composable_support::math::safe::SafeSub;
use composable_traits::financial_nft::{FinancialNftProvider, NftClass};
use frame_system::EventRecord;
use std::collections::{BTreeMap, BTreeSet};

use crate::{
	mock::{Event, System, Test},
	pallet::{ClassInstances, Config, Event as NftEvent, Instance, OwnerInstances},
	NftInstanceId, Pallet,
};

const ALICE: u128 = 0;
const BOB: u128 = 0;

pub(crate) fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len().safe_sub(&1).expect("No events present!")];
	assert_eq!(event, &system_event);
}

/// Asserts the event wasn't dispatched.
fn assert_no_event(event: crate::mock::Event) {
	assert!(System::events().iter().all(|record| record.event != event));
}

/// Mints a new NFT into ALICE and checks that it was created properly, returning the id of the
/// newly created NFT.
///
/// NOTE: Only call once!
fn mint_nft_and_assert() -> NftInstanceId {
	let created_nft_id =
		Pallet::<Test>::mint_nft(&NftClass::STAKING, &ALICE, &1u32, &1u32).unwrap();

	// assert_last_event::<Test>(Event::Nft(NftEvent::NftCreated {
	// 	class_id: NftClass::STAKING,
	// 	instance_id: created_nft_id,
	// }));

	assert_eq!(
		ClassInstances::<Test>::get(&NftClass::STAKING).unwrap(),
		BTreeSet::from([created_nft_id]),
		"STAKING class should only have one instance"
	);

	assert_eq!(
		OwnerInstances::<Test>::get(&ALICE).unwrap(),
		BTreeSet::from([(NftClass::STAKING, created_nft_id)]),
		"ALICE should only have one instance"
	);

	assert_eq!(
		Instance::<Test>::get(&(NftClass::STAKING, created_nft_id)).unwrap(),
		(ALICE, BTreeMap::from([(1u32.encode(), 1u32.encode())])),
		"owner should be ALICE"
	);

	created_nft_id
}

mod financial_nft_provider {
	use crate::{mock::new_test_ext, test::mint_nft_and_assert};

	#[test]
	fn mint_nft() {
		new_test_ext().execute_with(mint_nft_and_assert);
	}
}

mod impls {
	use codec::Encode;
	use composable_traits::financial_nft::{FinancialNftProvider, NftClass};
	use frame_support::traits::tokens::nonfungibles::{Create, Inspect, Transfer};
	use std::collections::{BTreeMap, BTreeSet};

	use crate::{
		mock::{new_test_ext, Test},
		pallet::*,
		test::{mint_nft_and_assert, ALICE, BOB},
		Pallet,
	};

	#[test]
	fn inspect() {
		new_test_ext().execute_with(|| {
			let created_nft_id = mint_nft_and_assert();

			// owner check
			assert_eq!(Pallet::<Test>::owner(&NftClass::STAKING, &created_nft_id), Some(ALICE));

			// attribute check
			assert_eq!(
				Pallet::<Test>::attribute(&NftClass::STAKING, &created_nft_id, &1u32.encode()),
				Some(1u32.encode())
			);

			// class attribute check
			assert_eq!(
				Pallet::<Test>::class_attribute(&NftClass::STAKING, &1u32.encode()),
				None,
				"staking class has no attributes"
			);
		})
	}

	#[test]
	fn create() {
		new_test_ext().execute_with(|| {
			assert_eq!(Pallet::<Test>::create_class(&NftClass::new(2), &ALICE, &ALICE), Ok(()));

			assert_eq!(
				Class::<Test>::get(&NftClass::new(2)),
				Some((ALICE, ALICE, Default::default()))
			);
		})
	}

	#[test]
	fn transfer() {
		new_test_ext().execute_with(|| {
			let created_nft_id = mint_nft_and_assert();

			assert_eq!(
				Pallet::<Test>::owner(&NftClass::STAKING, &created_nft_id),
				Some(ALICE),
				"owner before transfer should be ALICE"
			);

			assert_eq!(Pallet::<Test>::transfer(&NftClass::STAKING, &created_nft_id, &BOB), Ok(()));

			assert_eq!(
				OwnerInstances::<Test>::get(&BOB).unwrap(),
				BTreeSet::from([(NftClass::STAKING, created_nft_id)]),
				"BOB should only have one NFT"
			);

			assert_eq!(
				OwnerInstances::<Test>::get(&ALICE).unwrap(),
				BTreeSet::from([]),
				"ALICE should not have any NFTs"
			);

			assert_eq!(
				Instance::<Test>::get(&(NftClass::STAKING, created_nft_id)),
				Some((BOB, BTreeMap::from([(1u32.encode(), 1u32.encode())]))),
				"owner after transfer should be BOB"
			);

			assert_eq!(
				Pallet::<Test>::owner(&NftClass::STAKING, &created_nft_id),
				Some(BOB),
				"owner after transfer should be BOB"
			);
		})
	}
}
