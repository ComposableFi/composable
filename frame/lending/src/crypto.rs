use super::KEY_TYPE;
use frame_system::offchain;
use sp_core::sr25519::{self, Signature as Sr25519Signature};
use sp_runtime::{app_crypto::app_crypto, traits::Verify, MultiSignature, MultiSigner};

app_crypto!(sr25519, KEY_TYPE);

pub struct TestAuthId;

// implementation for runtime
impl offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
	type RuntimeAppPublic = Public;
	type GenericSignature = sr25519::Signature;
	type GenericPublic = sr25519::Public;
}

// implementation for mock runtime in test
impl offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for TestAuthId {
	type RuntimeAppPublic = Public;
	type GenericSignature = sr25519::Signature;
	type GenericPublic = sr25519::Public;
}
