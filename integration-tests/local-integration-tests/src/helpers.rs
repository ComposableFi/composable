use common::AccountId;
use cumulus_primitives_core::ParaId;
use sp_runtime::traits::AccountIdConversion;

/// create account ids from test paraid
pub fn para_account_id(id: u32) -> AccountId {
	ParaId::from(id).into_account()
}
