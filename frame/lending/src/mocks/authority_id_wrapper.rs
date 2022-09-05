use serde::{Deserialize, Serialize};
use sp_application_crypto::BoundToRuntimeAppPublic;
use sp_core::crypto::{key_types, ByteArray, CryptoType, Dummy};

pub use sp_core::{sr25519, H256};
use sp_runtime::{
	codec::{Decode, Encode, MaxEncodedLen},
	scale_info::TypeInfo,
	testing::{TestSignature, UintAuthorityId},
	traits::{IdentifyAccount, OpaqueKeys},
	CryptoTypeId, KeyTypeId,
};
use std::{cell::RefCell, ops::Deref};

#[derive(
	Default,
	PartialEq,
	Eq,
	Clone,
	Encode,
	Decode,
	Debug,
	Hash,
	Serialize,
	Deserialize,
	PartialOrd,
	Ord,
	MaxEncodedLen,
	TypeInfo,
)]
// The wrapper was added since AppCrypto is not implemented for UintAuthorityId
pub struct UintAuthorityIdWrapper(pub UintAuthorityId);

impl From<u64> for UintAuthorityIdWrapper {
	fn from(id: u64) -> Self {
		UintAuthorityIdWrapper(UintAuthorityId(id))
	}
}

impl From<UintAuthorityIdWrapper> for u64 {
	fn from(id: UintAuthorityIdWrapper) -> u64 {
		id.0 .0
	}
}

impl From<UintAuthorityIdWrapper> for UintAuthorityId {
	fn from(id: UintAuthorityIdWrapper) -> UintAuthorityId {
		id.0
	}
}

impl UintAuthorityIdWrapper {
	/// Convert this authority ID into a public key.
	pub fn to_public_key<T: ByteArray>(&self) -> T {
		self.0.to_public_key()
	}
}

impl CryptoType for UintAuthorityIdWrapper {
	type Pair = Dummy;
}

impl AsRef<[u8]> for UintAuthorityIdWrapper {
	fn as_ref(&self) -> &[u8] {
		self.0.as_ref()
	}
}

thread_local! {
	static ALL_KEYS: RefCell<Vec<UintAuthorityIdWrapper>> = RefCell::new(vec![]);
}

impl UintAuthorityIdWrapper {
	/// Set the list of keys returned by the runtime call for all keys of that type.
	pub fn set_all_keys<T: Into<UintAuthorityIdWrapper>>(keys: impl IntoIterator<Item = T>)
	where
		UintAuthorityId: From<T>,
	{
		UintAuthorityId::set_all_keys(keys);
	}
}

impl sp_application_crypto::RuntimeAppPublic for UintAuthorityIdWrapper {
	const ID: KeyTypeId = key_types::DUMMY;
	// cspell:disable-next
	const CRYPTO_ID: CryptoTypeId = CryptoTypeId(*b"dumm");

	type Signature = TestSignature;

	fn all() -> Vec<Self> {
		UintAuthorityId::all().into_iter().map(Self).collect()
	}

	fn generate_pair(input: Option<Vec<u8>>) -> Self {
		Self(UintAuthorityId::generate_pair(input))
	}

	fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature> {
		self.0.sign(msg)
	}

	fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
		self.0.verify(msg, signature)
	}

	fn to_raw_vec(&self) -> Vec<u8> {
		self.0.to_raw_vec()
	}
}

impl OpaqueKeys for UintAuthorityIdWrapper {
	type KeyTypeIdProviders = ();

	fn key_ids() -> &'static [KeyTypeId] {
		UintAuthorityId::key_ids()
	}

	fn get_raw(&self, key_type_id: KeyTypeId) -> &[u8] {
		self.0.get_raw(key_type_id)
	}

	fn get<T: Decode>(&self, key_type_id: KeyTypeId) -> Option<T> {
		self.0.get(key_type_id)
	}
}

impl BoundToRuntimeAppPublic for UintAuthorityIdWrapper {
	type Public = Self;
}

impl IdentifyAccount for UintAuthorityIdWrapper {
	type AccountId = <UintAuthorityId as IdentifyAccount>::AccountId;

	fn into_account(self) -> Self::AccountId {
		self.0 .0
	}
}

impl Deref for UintAuthorityIdWrapper {
	type Target = [u8];
	fn deref(&self) -> &Self::Target {
		&self.as_ref()
	}
}
impl frame_system::offchain::AppCrypto<UintAuthorityIdWrapper, TestSignature>
	for UintAuthorityIdWrapper
{
	type RuntimeAppPublic = Self;
	type GenericSignature = TestSignature;
	type GenericPublic = Self;
}
