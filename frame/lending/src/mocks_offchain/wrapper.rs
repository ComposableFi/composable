use serde::{Deserialize, Serialize};
use sp_application_crypto::BoundToRuntimeAppPublic;
use sp_core::{
	crypto::{key_types, ByteArray, CryptoType, Dummy},
	U256,
};
pub use sp_core::{sr25519, H256};
use sp_runtime::{
	codec::{Decode, Encode, MaxEncodedLen},
	scale_info::TypeInfo,
	testing::{TestSignature, UintAuthorityId},
	traits::{IdentifyAccount, OpaqueKeys, Verify},
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

impl UintAuthorityIdWrapper {
	/// Convert this authority ID into a public key.
	pub fn to_public_key<T: ByteArray>(&self) -> T {
		let bytes: [u8; 32] = U256::from(self.0 .0).into();
		T::from_slice(&bytes).unwrap()
	}
}

impl CryptoType for UintAuthorityIdWrapper {
	type Pair = Dummy;
}

impl AsRef<[u8]> for UintAuthorityIdWrapper {
	fn as_ref(&self) -> &[u8] {
		unsafe {
			let byte: *const _ = &(self.0 .0 as _);
			std::slice::from_raw_parts(byte, std::mem::size_of::<u64>())
		}
	}
}

thread_local! {
	static ALL_KEYS: RefCell<Vec<UintAuthorityIdWrapper>> = RefCell::new(vec![]);
}

impl UintAuthorityIdWrapper {
	/// Set the list of keys returned by the runtime call for all keys of that type.
	pub fn set_all_keys<T: Into<UintAuthorityIdWrapper>>(keys: impl IntoIterator<Item = T>) {
		ALL_KEYS.with(|l| *l.borrow_mut() = keys.into_iter().map(Into::into).collect())
	}
}

impl sp_application_crypto::RuntimeAppPublic for UintAuthorityIdWrapper {
	const ID: KeyTypeId = key_types::DUMMY;
	const CRYPTO_ID: CryptoTypeId = CryptoTypeId(*b"dumm");

	type Signature = TestSignature;

	fn all() -> Vec<Self> {
		ALL_KEYS.with(|l| l.borrow().clone())
	}

	fn generate_pair(_: Option<Vec<u8>>) -> Self {
		use rand::RngCore;
		Self(UintAuthorityId(rand::thread_rng().next_u64()))
	}

	fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature> {
		Some(TestSignature(self.0 .0, msg.as_ref().to_vec()))
	}

	fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
		Verify::verify(signature, msg.as_ref(), &self.0 .0)
	}

	fn to_raw_vec(&self) -> Vec<u8> {
		AsRef::<[u8]>::as_ref(self).to_vec()
	}
}

impl OpaqueKeys for UintAuthorityIdWrapper {
	type KeyTypeIdProviders = ();

	fn key_ids() -> &'static [KeyTypeId] {
		&[key_types::DUMMY]
	}

	fn get_raw(&self, _: KeyTypeId) -> &[u8] {
		self.as_ref()
	}

	fn get<T: Decode>(&self, _: KeyTypeId) -> Option<T> {
		self.using_encoded(|mut x| T::decode(&mut x)).ok()
	}
}

impl BoundToRuntimeAppPublic for UintAuthorityIdWrapper {
	type Public = Self;
}

impl IdentifyAccount for UintAuthorityIdWrapper {
	type AccountId = u64;

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
