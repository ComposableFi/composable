use composable_traits::governance::SignedRawOrigin;
use orml_traits::GetByKey;

use super::{currency::CurrencyId, runtime::AccountId};

pub struct GovernanceRegistry;
impl composable_traits::governance::GovernanceRegistry<CurrencyId, AccountId>
	for GovernanceRegistry
{
	fn set(_k: CurrencyId, _value: composable_traits::governance::SignedRawOrigin<AccountId>) {}
}

impl
	GetByKey<
		CurrencyId,
		Result<SignedRawOrigin<sp_core::sr25519::Public>, sp_runtime::DispatchError>,
	> for GovernanceRegistry
{
	fn get(
		_k: &CurrencyId,
	) -> Result<SignedRawOrigin<sp_core::sr25519::Public>, sp_runtime::DispatchError> {
		Ok(SignedRawOrigin::Root)
	}
}
