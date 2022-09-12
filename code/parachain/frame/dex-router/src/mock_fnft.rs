#[cfg(test)]
use composable_traits::fnft::FinancialNft;
use frame_support::{
	dispatch::DispatchResult,
	traits::tokens::nonfungibles::{Create, Inspect, Mutate},
};
use sp_runtime::DispatchError;

pub struct MockFnft;

impl Inspect<u128> for MockFnft {
	type ItemId = u64;
	type CollectionId = u128;

	fn owner(collection: &Self::CollectionId, item: &Self::ItemId) -> Option<u128> {
		todo!()
	}
}

impl FinancialNft<u128> for MockFnft {
	fn asset_account(collection: &Self::CollectionId, instance: &Self::ItemId) -> u128 {
		todo!()
	}

	fn get_next_nft_id(collection: &Self::CollectionId) -> Result<Self::ItemId, DispatchError> {
		todo!()
	}
}

impl Create<u128> for MockFnft {
	fn create_collection(
		collection: &Self::CollectionId,
		who: &u128,
		admin: &u128,
	) -> DispatchResult {
		Ok(())
	}
}

impl Mutate<u128> for MockFnft {}
