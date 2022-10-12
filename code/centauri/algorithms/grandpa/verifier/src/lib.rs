// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! GRANDPA light client verification function

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]

extern crate alloc;

use alloc::vec;
use anyhow::anyhow;
use codec::{Decode, Encode};
use finality_grandpa::Chain;
use hash_db::Hasher;
use light_client_common::state_machine;
use primitive_types::H256;
use primitives::{
	error,
	justification::{find_scheduled_change, AncestryChain, GrandpaJustification},
	parachain_header_storage_key, ClientState, HostFunctions, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use sp_runtime::traits::Header;
use sp_trie::{LayoutV0, StorageProof};

#[cfg(test)]
mod tests;

/// Verify a new grandpa justification, given the old state.
pub fn verify_parachain_headers_with_grandpa_finality_proof<H, Host>(
	mut client_state: ClientState<H::Hash>,
	proof: ParachainHeadersWithFinalityProof<H>,
) -> Result<ClientState<H::Hash>, error::Error>
where
	H: Header<Hash = H256>,
	H::Number: finality_grandpa::BlockNumberOps + Into<u32>,
	Host: HostFunctions,
	Host::BlakeTwo256: Hasher<Out = H256>,
{
	let ParachainHeadersWithFinalityProof { finality_proof, mut parachain_headers } = proof;

	// 1. first check that target is in proof.unknown_headers.
	let headers = AncestryChain::<H>::new(&finality_proof.unknown_headers);
	let target = headers
		.header(&finality_proof.block)
		.ok_or_else(|| anyhow!("Target header with hash: {:?} not found!", finality_proof.block))?;

	// 2. next check that there exists a route from client.latest_relay_hash to target.
	let finalized = headers
		.ancestry(client_state.latest_relay_hash, finality_proof.block)
		.map_err(|_| anyhow!("Invalid ancestry! 1"))?;

	// 3. verify justification.
	let justification = GrandpaJustification::<H>::decode(&mut &finality_proof.justification[..])?;
	justification.verify::<Host>(client_state.current_set_id, &client_state.current_authorities)?;

	// 4. verify state proofs of parachain headers in finalized relay chain headers.
	for hash in finalized {
		let relay_chain_header =
			headers.header(&hash).expect("Headers have been checked by AncestryChain; qed");
		if let Some(proofs) = parachain_headers.remove(&hash) {
			let ParachainHeaderProofs { extrinsic_proof, extrinsic, state_proof } = proofs;
			let proof = StorageProof::new(state_proof);
			let key = parachain_header_storage_key(client_state.para_id);
			let header = state_machine::read_proof_check::<Host::BlakeTwo256, _>(
				relay_chain_header.state_root(),
				proof,
				&[key.as_ref()],
			)
			.map_err(|err| anyhow!("error verifying parachain header state proof: {err}"))?
			.remove(key.as_ref())
			.flatten()
			.ok_or_else(|| anyhow!("Invalid proof, parachain header not found"))?;
			let parachain_header = H::decode(&mut &header[..])?;
			// Timestamp extrinsic should be the first inherent and hence the first extrinsic
			// https://github.com/paritytech/substrate/blob/d602397a0bbb24b5d627795b797259a44a5e29e9/primitives/trie/src/lib.rs#L99-L101
			let key = codec::Compact(0u32).encode();
			sp_trie::verify_trie_proof::<LayoutV0<Host::BlakeTwo256>, _, _, _>(
				parachain_header.extrinsics_root(),
				&extrinsic_proof,
				&vec![(key, Some(&extrinsic[..]))],
			)
			.map_err(|_| anyhow!("Invalid extrinsic proof"))?;
		}
	}

	// 5. set new client state, optionally rotating authorities
	client_state.latest_relay_hash = target.hash();
	client_state.latest_relay_height = (*target.number()).into();
	if let Some(scheduled_change) = find_scheduled_change::<H>(&target) {
		client_state.current_set_id += 1;
		client_state.current_authorities = scheduled_change.next_authorities;
	}

	Ok(client_state)
}
