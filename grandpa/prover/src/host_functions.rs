use anyhow::anyhow;
use codec::Encode;
use primitives::{error::Error, HostFunctions};
use sp_core::{
	ed25519::{Public, Signature},
	H256,
};
use sp_runtime::{app_crypto::RuntimePublic, traits::BlakeTwo256};
use sp_trie::{LayoutV0, StorageProof};
use std::collections::BTreeMap;

pub struct HostFunctionsProvider;

impl HostFunctions for HostFunctionsProvider {
	fn ed25519_verify(sig: &Signature, msg: &[u8], pubkey: &Public) -> bool {
		pubkey.verify(&msg, sig)
	}

	fn read_proof_check<I>(
		root: &[u8; 32],
		proof: StorageProof,
		keys: I,
	) -> Result<BTreeMap<Vec<u8>, Option<Vec<u8>>>, Error>
	where
		I: IntoIterator,
		I::Item: AsRef<[u8]>,
	{
		let result =
			sp_state_machine::read_proof_check::<BlakeTwo256, _>(H256::from(root), proof, keys)
				.map_err(|err| anyhow!("Error verifying state proof: {err}"))?;
		Ok(result.into_iter().collect())
	}

	fn verify_timestamp_extrinsic(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		value: &[u8],
	) -> Result<(), Error> {
		let key = codec::Compact(0u32).encode();
		sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
			&H256::from(root),
			proof,
			&vec![(key, Some(value))],
		)
		.map_err(|err| anyhow!("Error verifying extrinsic proof: {err}"))?;
		Ok(())
	}
}
