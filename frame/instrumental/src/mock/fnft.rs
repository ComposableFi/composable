use composable_traits::fnft::FinancialNft;
use frame_support::{
	dispatch::DispatchResult,
	traits::tokens::nonfungibles::{Create, Inspect, Mutate},
};
use sp_runtime::DispatchError;

use super::account_id::AccountId;

pub struct MockFnft;

impl Inspect<AccountId> for MockFnft {
	type ItemId = ();
	type CollectionId = ();

	fn owner(_collection: &Self::CollectionId, _item: &Self::ItemId) -> Option<AccountId> {
		todo!()
	}
}

impl FinancialNft<AccountId> for MockFnft {
	fn asset_account(_collection: &Self::CollectionId, _instance: &Self::ItemId) -> AccountId {
		todo!()
	}

	fn get_next_nft_id(_collection: &Self::CollectionId) -> Result<Self::ItemId, DispatchError> {
		todo!()
	}
}

impl Create<AccountId> for MockFnft {
	fn create_collection(
		_collection: &Self::CollectionId,
		_who: &AccountId,
		_admin: &AccountId,
	) -> DispatchResult {
		todo!()
	}
}

impl Mutate<AccountId> for MockFnft {}
