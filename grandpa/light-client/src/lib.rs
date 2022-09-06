#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use crate::justification::{find_scheduled_change, AncestryChain, GrandpaJustification};
use anyhow::anyhow;
use codec::Decode;
use finality_grandpa::Chain;
use primitives::{
	error, parachain_header_storage_key, ClientState, HostFunctions, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use sp_core::H256;
use sp_runtime::traits::{Block, Header, NumberFor};
use sp_trie::StorageProof;

pub mod justification;
#[cfg(test)]
mod tests;

/// Verify a new grandpa justification, given the old state.
pub fn verify_parachain_headers_with_grandpa_finality_proof<B, H>(
	mut client_state: ClientState<<B as Block>::Hash>,
	proof: ParachainHeadersWithFinalityProof<B>,
) -> Result<ClientState<<B as Block>::Hash>, error::Error>
where
	B: Block<Hash = H256>,
	H: HostFunctions,
	NumberFor<B>: finality_grandpa::BlockNumberOps,
{
	let ParachainHeadersWithFinalityProof { finality_proof, mut parachain_headers } = proof;

	// 1. first check that target is in proof.unknown_headers.
	let headers = AncestryChain::<B>::new(&finality_proof.unknown_headers);
	let target = headers
		.header(&finality_proof.block)
		.ok_or_else(|| anyhow!("Target header not found!"))?;

	// 2. next check that there exists a route from client.latest_relay_hash to target.
	let finalized = headers.ancestry(client_state.latest_relay_hash, finality_proof.block)?;

	// 3. verify justification.
	let justification = GrandpaJustification::<B>::decode(&mut &finality_proof.justification[..])?;
	justification.verify::<H>(client_state.current_set_id, &client_state.current_authorities)?;

	// 4. verify state proofs of parachain headers in finalized relay chain headers.
	for hash in finalized {
		let relay_chain_header =
			headers.header(&hash).expect("Headers have been checked by AncestryChain; qed");
		if let Some(proofs) = parachain_headers.remove(&hash) {
			let ParachainHeaderProofs { extrinsic_proof, extrinsic, state_proof } = proofs;
			let proof = StorageProof::new(state_proof);
			let key = parachain_header_storage_key(client_state.para_id);
			let header = H::read_proof_check(
				relay_chain_header.state_root().as_fixed_bytes(),
				proof,
				&[key.as_ref()],
			)
			.map_err(|err| anyhow!("error verifying parachain header state proof: {err}"))?
			.remove(key.as_ref())
			.flatten()
			.ok_or_else(|| anyhow!("Invalid proof, parachain header not found"))?;
			let header = Vec::<u8>::decode(&mut &header[..])?;
			let parachain_header = B::Header::decode(&mut &header[..])?;
			// verify timestamp extrinsic proof
			H::verify_timestamp_extrinsic(
				parachain_header.extrinsics_root().as_fixed_bytes(),
				&extrinsic_proof,
				&extrinsic[..],
			)?;
		}
	}

	// 5. set new client state, optionally rotating authorities
	client_state.latest_relay_hash = target.hash();
	if let Some(scheduled_change) = find_scheduled_change::<B>(&target) {
		client_state.current_set_id += 1;
		client_state.current_authorities = scheduled_change.next_authorities;
	}

	Ok(client_state)
}
