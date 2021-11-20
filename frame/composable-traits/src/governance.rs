use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use frame_system::RawOrigin;
use scale_info::TypeInfo;

/// Like `RawOrigin`, but always signed.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum SignedRawOrigin<AccountId> {
	/// The system itself ordained this dispatch to happen: this is the highest privilege level.
	Root,
	/// It is signed by some public key and we provide the `AccountId`.
	Signed(AccountId),
}

impl<T> From<SignedRawOrigin<T>> for RawOrigin<T> {
	fn from(this: SignedRawOrigin<T>) -> Self {
		match this {
			SignedRawOrigin::Root => RawOrigin::Root,
			SignedRawOrigin::Signed(x) => RawOrigin::Signed(x),
		}
	}
}

pub trait GovernanceRegistry<AssetId, AccountId> {
	fn set(k: AssetId, value: SignedRawOrigin<AccountId>);
}
