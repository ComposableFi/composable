use primitives::currency::CurrencyId;
use crate::AccountId;

pub struct CosmwasmToSubstrate;

impl Convert<alloc::string::String, Result<AccountId, ()>> for CosmwasmToSubstrate {
	fn convert(a: alloc::string::String) -> Result<AccountId, ()> {
		let account =
			ibc_primitives::runtime_interface::ss58_to_account_id_32(&a).map_err(|_| ())?;
		Ok(account.into())
	}
}

impl Convert<AccountId, alloc::string::String> for CosmwasmToSubstrate {
	fn convert(a: AccountId) -> alloc::string::String {
		let account = ibc_primitives::runtime_interface::account_id_to_ss58(a.into(), 49);
		String::from_utf8_lossy(account.as_slice()).to_string()
	}
}

impl Convert<Vec<u8>, Result<AccountId, ()>> for CosmwasmToSubstrate {
	fn convert(a: Vec<u8>) -> Result<AccountId, ()> {
		Ok(<[u8; 32]>::try_from(a).map_err(|_| ())?.into())
	}
}

impl Convert<alloc::string::String, Result<CurrencyId, ()>> for CosmwasmToSubstrate {
	fn convert(currency_id: alloc::string::String) -> Result<CurrencyId, ()> {
		core::str::FromStr::from_str(&currency_id).map_err(|_| ())
	}
}

impl Convert<CurrencyId, alloc::string::String> for CosmwasmToSubstrate {
	fn convert(CurrencyId(currency_id): CurrencyId) -> alloc::string::String {
		currency_id.to_string()
	}
}
