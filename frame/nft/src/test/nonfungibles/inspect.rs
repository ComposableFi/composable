use codec::Encode;

use composable_traits::nft::NftClass;
use frame_support::traits::tokens::nonfungibles::Inspect;

use crate::{
	test::{
		mock::{new_test_ext, MockRuntime},
		prelude::mint_nft_and_assert,
		ALICE,
	},
	Pallet,
};

#[test]
#[ignore = "TODO: fix with updates to nft pallet"]
/// Tests the pallet's [`Inspect`] implementation returns the expected values (success case)
pub(crate) fn success() {
	new_test_ext().execute_with(|| {
		let created_nft_id = mint_nft_and_assert();

		// owner check
		assert_eq!(Pallet::<MockRuntime>::owner(&NftClass::STAKING, &created_nft_id), Some(ALICE));

		// attribute check
		assert_eq!(
			Pallet::<MockRuntime>::attribute(&NftClass::STAKING, &created_nft_id, &1_u32.encode()),
			Some(1_u32.encode())
		);

		// class attribute check
		assert_eq!(
			Pallet::<MockRuntime>::collection_attribute(&NftClass::STAKING, &1_u32.encode()),
			None,
			"staking class should have no attributes"
		);
	})
}

#[test]
#[ignore = "TODO: fix with updates to nft pallet"]
/// Asserts that the pallet's [`Inspect`] implementation errors as expected.
pub(crate) fn failure() {
	new_test_ext().execute_with(|| {
		let created_nft_id = mint_nft_and_assert();

		// owner check
		assert_eq!(
			Pallet::<MockRuntime>::owner(&NftClass::STAKING, &(created_nft_id + 1)),
			None,
			"NFT does not exist, there should be no owner"
		);

		// attribute check
		assert_eq!(
			Pallet::<MockRuntime>::attribute(
				&NftClass::STAKING,
				&(created_nft_id + 1),
				&1_u32.encode()
			),
			None,
			"NFT does not exist, there should be no attributes"
		);

		// class attribute check
		assert_eq!(
			Pallet::<MockRuntime>::collection_attribute(&NftClass::new(255), &1_u32.encode()),
			None,
			"class does not exist, there should be no attributes"
		);
	})
}
