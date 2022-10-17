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
#![deny(missing_docs)]

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

/// This function verifies the GRANDPA finality proof for relay chain headers.
///
/// Next, we prove the finality of parachain headers, by verifying patricia-merkle trie state proofs
/// of these headers, stored at the recently finalized relay chain heights.
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
	let ParachainHeadersWithFinalityProof { finality_proof, parachain_headers } = proof;

	// 1. First validate unknown headers.
	let headers = AncestryChain::<H>::new(&finality_proof.unknown_headers);
	let target = finality_proof
		.unknown_headers
		.iter()
		.max_by_key(|h| *h.number())
		.ok_or_else(|| anyhow!("Unknown headers can't be empty!"))?;

	// this is illegal
	if target.hash() != finality_proof.block {
		Err(anyhow!("Latest finalized block should be highest block in unknown_headers"))?;
	}

	let from = finality_proof
		.unknown_headers
		.iter()
		.min_by_key(|h| *h.number())
		.ok_or_else(|| anyhow!("Unknown headers can't be empty!"))?;
	let mut finalized = headers
		.ancestry(from.hash(), target.hash())
		.map_err(|_| anyhow!("Invalid ancestry!"))?;
	finalized.sort();

	// 2. verify justification.
	let justification = GrandpaJustification::<H>::decode(&mut &finality_proof.justification[..])?;
	justification.verify::<Host>(client_state.current_set_id, &client_state.current_authorities)?;

	// 3. verify state proofs of parachain headers in finalized relay chain headers.
	let mut para_heights = vec![];
	for (hash, proofs) in parachain_headers {
		if finalized.binary_search(&hash).is_err() {
			// seems relay hash isn't in the finalized chain.
			continue
		}
		let relay_chain_header =
			headers.header(&hash).expect("Headers have been checked by AncestryChain; qed");

		let ParachainHeaderProofs { extrinsic_proof, extrinsic, state_proof } = proofs;
		let proof = StorageProof::new(state_proof);
		let key = parachain_header_storage_key(client_state.para_id);
		// verify patricia-merkle state proofs
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
		para_heights.push(parachain_header.number().clone().into());
		// Timestamp extrinsic should be the first inherent and hence the first extrinsic
		// https://github.com/paritytech/substrate/blob/d602397a0bbb24b5d627795b797259a44a5e29e9/primitives/trie/src/lib.rs#L99-L101
		let key = codec::Compact(0u32).encode();
		// verify extrinsic proof for timestamp extrinsic
		sp_trie::verify_trie_proof::<LayoutV0<Host::BlakeTwo256>, _, _, _>(
			parachain_header.extrinsics_root(),
			&extrinsic_proof,
			&vec![(key, Some(&extrinsic[..]))],
		)
		.map_err(|_| anyhow!("Invalid extrinsic proof"))?;
	}

	// 4. set new client state, optionally rotating authorities
	client_state.latest_relay_hash = target.hash();
	client_state.latest_relay_height = (*target.number()).into();
	if let Some(max_height) = para_heights.into_iter().max() {
		client_state.latest_para_height = max_height;
	}
	if let Some(scheduled_change) = find_scheduled_change::<H>(&target) {
		client_state.current_set_id += 1;
		client_state.current_authorities = scheduled_change.next_authorities;
	}

	Ok(client_state)
}
