use crate::prelude::*;

use sha2::{Digest, Sha256};

// Hash creates a new address from address type and key.
// The functions should only be used by new types defining their own address function
// (eg public keys).
/// https://github.com/cosmos/cosmos-sdk/blob/main/types/address/hash.go
pub fn addess_hash(typ: &str, key: &[u8]) -> [u8; 32] {
	let mut hasher = Sha256::default();
	hasher.update(typ.as_bytes());
	let th = hasher.finalize();
	let mut hasher = Sha256::default();
	hasher.update(th);
	hasher.update(key);
	hasher.finalize().into()
}

// takes a transfer message and returns ibc/<hash of denom>
// https://ibc.cosmos.network/main/architecture/adr-001-coin-source-tracing.html
// so can infer for some chain denom on hops
pub fn hash_denom_trace(unwrapped: &str) -> String {
	let digest = Sha256::digest(unwrapped.as_bytes());
	["ibc/", &hex::encode_upper(digest)].concat()
}
