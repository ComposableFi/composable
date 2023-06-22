use codec::Decode;
use primitives::currency::CurrencyId;
use smoldot::identity::ss58::{Decoded, ChainPrefix};
use sp_core::crypto::Ss58AddressFormat;
use sp_runtime::{traits::Convert, AccountId32};
use crate::AccountId;
use sp_core::crypto::Ss58Codec;

pub struct CosmwasmToSubstrateAccount;

impl Convert<alloc::string::String, Result<AccountId, ()>> for CosmwasmToSubstrateAccount {
	fn convert(a: alloc::string::String) -> Result<AccountId, ()> {
		let account : AccountId32 = smoldot::identity::ss58::decode(&a).map_err(|_| ())?.public_key.as_ref().try_into().map_err(|_| ())?;
		Ok(account.into())
	}
}

impl Convert<AccountId, alloc::string::String> for CosmwasmToSubstrateAccount {
	fn convert(a: AccountId) -> alloc::string::String {
		smoldot::identity::ss58::encode(Decoded {
			chain_prefix: ChainPrefix::from(49),
    		public_key: &a,
		})
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
