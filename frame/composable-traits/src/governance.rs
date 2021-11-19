/// Like `RawOrigin`, but always signed.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum SignedRawOrigin<AccountId> {
    /// The system itself ordained this dispatch to happen: this is the highest privilege level.
    Root,
    /// It is signed by some public key and we provide the `AccountId`.
    Signed(AccountId),
}

impl Into<RawOrigin<T> for SignedRawOrigin<T> {

}