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
pub fn hash_denom_trace(denom: &PrefixedDenom) -> String {
	let denom = denom.to_string();
	let digest = Sha256::digest(denom.as_bytes());
	["ibc/", &hex::encode_upper(digest)].concat()
}

#[cfg(test)]
mod tests {
	use super::*;

	// various devnet channels hashes
	#[test]
	fn devnet() {
		let pica = hash_denom_trace(&PrefixedDenom::from_str("transfer/channel-1/1").expect("const"));
		assert_eq!(pica, "ibc/71B5DB2263A5A5B160BBA26A307BF5441BDB330534C19A9F551F63D9CC0C3026");
		let pica = hash_denom_trace(&PrefixedDenom::from_str("transfer/channel-0/1").expect("const"));
		assert_eq!(pica, "ibc/632DBFDB06584976F1351A66E873BF0F7A19FAA083425FEC9890C90993E5F0A4");

		let pica: String = hash_denom_trace(&PrefixedDenom::from_str("transfer/channel-1/ppica").expect("const"));
		assert_eq!(pica, "ibc/6188228DA6C48BB205E30BD8850E2E5ADBD75010B9BF542F7E77A87D9D7DCCB7");

		let pica: String = hash_denom_trace(&PrefixedDenom::from_str("transfer/channel-0/ppica").expect("const"));
		assert_eq!(pica, "ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B");

		let osmo: String = hash_denom_trace(&PrefixedDenom::from_str("transfer/channel-1/uosmo").expect("const"));
		assert_eq!(osmo, "ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B");

		let osmo: String = hash_denom_trace(&PrefixedDenom::from_str("transfer/channel-0/uosmo").expect("const"));
		assert_eq!(osmo, "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518");
	}
}
