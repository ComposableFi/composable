use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{
	app_crypto::CryptoTypePublicPair,
	traits::{IdentifyAccount, Verify},
	KeyTypeId, MultiSignature, MultiSigner,
};
use subxt::{tx::Signer, Config};

use codec::Decode;
use primitives::KeyProvider;

/// A [`Signer`] implementation.
#[derive(Clone)]
pub struct ExtrinsicSigner<T: Config, Provider: KeyProvider> {
	account_id: T::AccountId,
	nonce: Option<T::Index>,
	signer: MultiSigner,
	key_store: SyncCryptoStorePtr,
	key_type_id: KeyTypeId,
	_phantom: std::marker::PhantomData<Provider>,
}

impl<T, P> ExtrinsicSigner<T, P>
where
	T: Config,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	P: KeyProvider,
	MultiSigner: From<MultiSigner>,
{
	/// Creates a new [`Signer`] from a key store reference and key type
	pub fn new(
		key_store: SyncCryptoStorePtr,
		key_type_id: KeyTypeId,
		public_key: MultiSigner,
	) -> Self {
		let account_id = <T::Signature as Verify>::Signer::from(public_key.clone()).into_account();
		Self {
			account_id,
			nonce: None,
			key_store,
			key_type_id,
			signer: MultiSigner::from(public_key),
			_phantom: Default::default(),
		}
	}
}

impl<T, P> Signer<T> for ExtrinsicSigner<T, P>
where
	T: Config,
	T::AccountId: Into<T::Address> + Clone + 'static,
	P: KeyProvider + 'static,
	T::Signature: From<MultiSignature>,
{
	fn nonce(&self) -> Option<T::Index> {
		self.nonce
	}

	fn account_id(&self) -> &T::AccountId {
		&self.account_id
	}

	fn address(&self) -> T::Address {
		self.account_id.clone().into()
	}

	fn sign(&self, signer_payload: &[u8]) -> T::Signature {
		let (crypto_type_id, public_key) = match &self.signer {
			MultiSigner::Ed25519(key) => (sp_core::ed25519::CRYPTO_ID, key.0.to_vec()),
			MultiSigner::Sr25519(key) => (sp_core::sr25519::CRYPTO_ID, key.0.to_vec()),
			MultiSigner::Ecdsa(key) => (sp_core::ecdsa::CRYPTO_ID, key.0.to_vec()),
		};
		let key = CryptoTypePublicPair(crypto_type_id, public_key);
		let encoded_sig =
			SyncCryptoStore::sign_with(&*self.key_store, self.key_type_id, &key, signer_payload)
				.ok()
				.flatten()
				.expect("Signing should not fail");
		let signature: MultiSignature = match self.signer {
			MultiSigner::Ed25519(_) => sp_core::ed25519::Signature::decode(&mut &encoded_sig[..])
				.expect("Should decode same signature type as public key; qed")
				.into(),
			MultiSigner::Sr25519(_) => sp_core::sr25519::Signature::decode(&mut &encoded_sig[..])
				.expect("Should decode same signature type as public key; qed")
				.into(),
			MultiSigner::Ecdsa(_) => sp_core::ecdsa::Signature::decode(&mut &encoded_sig[..])
				.expect("Should decode same signature type as public key; qed")
				.into(),
		};
		signature.into()
	}
}
