use composable_traits::governance::SignedRawOrigin;
use orml_traits::GetByKey;

use super::runtime::{AccountId, AssetId};

pub struct GovernanceRegistry;
impl composable_traits::governance::GovernanceRegistry<AssetId, AccountId> for GovernanceRegistry {
	fn set(_k: AssetId, _value: composable_traits::governance::SignedRawOrigin<AccountId>) {}
}

impl GetByKey<AssetId, Result<SignedRawOrigin<AccountId>, sp_runtime::DispatchError>>
	for GovernanceRegistry
{
	fn get(_k: &AssetId) -> Result<SignedRawOrigin<AccountId>, sp_runtime::DispatchError> {
		Ok(SignedRawOrigin::Root)
	}
}
