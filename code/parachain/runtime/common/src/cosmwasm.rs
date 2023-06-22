use primitives::currency::CurrencyId;
use sp_core::crypto::Ss58AddressFormat;
use sp_runtime::traits::Convert;
use crate::AccountId;
use sp_core::crypto::Ss58Codec;

pub struct CosmwasmToSubstrateAccount;

impl Convert<alloc::string::String, Result<AccountId, ()>> for CosmwasmToSubstrateAccount {
	fn convert(a: alloc::string::String) -> Result<AccountId, ()> {
		let account = AccountId::from_ss58check(&a).map_err(|_| ())?;			
		Ok(account.into())
	}
}

impl Convert<AccountId, alloc::string::String> for CosmwasmToSubstrateAccount {
	fn convert(a: AccountId) -> alloc::string::String {
		AccountId::to_ss58check_with_version(&a, Ss58AddressFormat::custom(59))
	}
}


impl Convert<Vec<u8>, Result<AccountId, ()>> for CosmwasmToSubstrateAccount {
	fn convert(a: Vec<u8>) -> Result<AccountId, ()> {
		Ok(<[u8; 32]>::try_from(a).map_err(|_| ())?.into())
	}
}

pub struct  CosmwasmToSubstrateAssetId;

impl Convert<alloc::string::String, Result<CurrencyId, ()>> for CosmwasmToSubstrateAssetId {
	fn convert(currency_id: alloc::string::String) -> Result<CurrencyId, ()> {
		core::str::FromStr::from_str(&currency_id).map_err(|_| ())
	}
}

impl Convert<CurrencyId, alloc::string::String> for CosmwasmToSubstrateAssetId {
	fn convert(CurrencyId(currency_id): CurrencyId) -> alloc::string::String {
		currency_id.to_string()
	}
}
