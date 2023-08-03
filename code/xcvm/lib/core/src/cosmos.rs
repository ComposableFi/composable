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

#[cfg(test)]
mod tests {
	use super::*;

	// various devnet channels hashes
	#[test]
	fn devnet() {
		let pica = hash_denom_trace("/transfer/channel-1/1");
		assert_eq!(pica, "ibc/B62D63F2BD5A7B70AB15F84BCB70EAC88222D3A8E8E0B22793EE788068EA22BA");
		let pica = hash_denom_trace("/transfer/channel-0/1");
		assert_eq!(pica, "ibc/F2B6EF5B6F86990A3863B78687ADE3D95E412657AAEB7CF2B3B8131B8055C1F1");
		
		let pica: String = hash_denom_trace("/transfer/channel-1/ppica");
		assert_eq!(pica, "ibc/661BD30059657725608DF36907F06B70C1FA7A1772FF92AEE1844A3E35A80D63");

		let pica: String = hash_denom_trace("/transfer/channel-0/ppica");
		assert_eq!(pica, "ibc/F0E228914E0E69E7B5E9231282FE6B7595CF90CB76E7193C6AFDCACDF5E83821");
		
		let osmo: String = hash_denom_trace("/transfer/channel-1/uosmo");
		assert_eq!(osmo, "ibc/BCACECE44E39A9009D793D68CC5DF76B402607B1C574379DCB4F3A5D24BC1936");
		
		let osmo: String = hash_denom_trace("/transfer/channel-0/uosmo");
		assert_eq!(osmo, "ibc/B4511F40A2844906F5940444691EE3AE877E8E0DD8354C8C4D36670A46C5680D");

	}
}


