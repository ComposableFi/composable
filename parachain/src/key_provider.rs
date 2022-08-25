use std::str::FromStr;

use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::KeyTypeId;
use subxt::Config;

use primitives::KeyProvider;

use super::ParachainClient;

impl<T: Config> KeyProvider for ParachainClient<T> {
	type Public = sp_runtime::MultiSigner;
	type Signature = sp_core::sr25519::Signature;

	fn account_id(&self) -> ibc::signer::Signer {
		// todo: use ss58 encoding
		let hex_string = format!("0x{}", hex::encode(self.public_key.clone()));

		ibc::signer::Signer::from_str(&hex_string).expect("Account Id should be valid")
	}

	fn public_key(&self) -> Self::Public {
		self.public_key.clone()
	}

	fn key_store(&self) -> SyncCryptoStorePtr {
		self.key_store.clone()
	}

	fn key_type_id(&self) -> KeyTypeId {
		self.key_type_id
	}
}
