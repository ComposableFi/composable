#[cfg(test)]
use composable_traits::fnft::FinancialNFT;
use frame_support::{
	dispatch::DispatchResult,
	traits::tokens::nonfungibles::{Create, Inspect, Mutate},
};
use sp_runtime::DispatchError;

pub struct MockFNFT;

impl Inspect<u128> for MockFNFT {
	type ItemId = ();
	type CollectionId = ();

	fn owner(collection: &Self::CollectionId, item: &Self::ItemId) -> Option<u128> {
		todo!()
	}
}

impl FinancialNFT<u128> for MockFNFT {
	fn asset_account(collection: &Self::CollectionId, instance: &Self::ItemId) -> u128 {
		todo!()
	}

	fn get_next_nft_id(collection: &Self::CollectionId) -> Result<Self::ItemId, DispatchError> {
		todo!()
	}
}

impl Create<u128> for MockFNFT {
	fn create_collection(
		collection: &Self::CollectionId,
		who: &u128,
		admin: &u128,
	) -> DispatchResult {
		todo!()
	}
}

impl Mutate<u128> for MockFNFT {}
