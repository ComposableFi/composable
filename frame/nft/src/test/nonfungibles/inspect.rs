use codec::Encode;
use composable_traits::financial_nft::NftClass;
use frame_support::traits::tokens::nonfungibles::Inspect;

use crate::{
	test::{
		helpers::mint_nft_and_assert,
		mock::{new_test_ext, MockRuntime},
		ALICE,
	},
	Pallet,
};

#[test]
/// Tests the pallet's [`Inspect`] implementation returns the expected values (success case)
pub(crate) fn success() {
	new_test_ext().execute_with(|| {
		let created_nft_id = mint_nft_and_assert();

		// owner check
		assert_eq!(Pallet::<MockRuntime>::owner(&NftClass::STAKING, &created_nft_id), Some(ALICE));

		// attribute check
		assert_eq!(
			Pallet::<MockRuntime>::attribute(&NftClass::STAKING, &created_nft_id, &1u32.encode()),
			Some(1u32.encode())
		);

		// class attribute check
		assert_eq!(
			Pallet::<MockRuntime>::class_attribute(&NftClass::STAKING, &1u32.encode()),
			None,
			"staking class should have no attributes"
		);
	})
}

#[test]
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
				&1u32.encode()
			),
			None,
			"NFT does not exist, there should be no attributes"
		);

		// class attribute check
		assert_eq!(
			Pallet::<MockRuntime>::class_attribute(&NftClass::new(255), &1u32.encode()),
			None,
			"class does not exist, there should be no attributes"
		);
	})
}
