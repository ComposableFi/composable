use super::Client;
use primitives::KeyProvider;
use std::str::FromStr;

impl KeyProvider for Client {
	fn account_id(&self) -> ibc::signer::Signer {
		ibc::signer::Signer::from_str(self.signer.as_str()).expect("Account Id should be valid")
	}
}
