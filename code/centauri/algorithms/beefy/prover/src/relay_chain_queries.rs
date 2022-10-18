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

use super::runtime;
use crate::{
	error::Error, runtime::api::runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
};
use beefy_light_client_primitives::get_leaf_index_for_block_number;
use beefy_primitives::{SignedCommitment, VersionedFinalityProof};
use codec::{Decode, Encode};
use pallet_mmr_rpc::{LeafBatchProof, LeafProof};
use sp_core::{hexdisplay::AsBytesRef, storage::StorageKey, H256};
use sp_runtime::traits::{Header, Zero};
use std::collections::{BTreeMap, BTreeSet};
use subxt::{rpc::rpc_params, Config, OnlineClient};

/// This contains the leaf indices of the relay chain blocks and a map of relay chain heights to a
/// map of all parachain headers at those heights Used for generating [`ParaHeadsProof`]
pub struct FinalizedParaHeads {
	/// Leaf indices
	pub leaf_indices: Vec<u32>,
	/// Map of relay chain heights to map of para ids and parachain headers SCALE-encoded
	pub raw_finalized_heads: BTreeMap<u64, BTreeMap<u32, Vec<u8>>>,
}

/// Get the raw parachain heads finalized in the provided block
pub async fn fetch_finalized_parachain_heads<T: Config>(
	client: &OnlineClient<T>,
	commitment_block_number: u32,
	latest_beefy_height: u32,
	para_id: u32,
	beefy_activation_block: u32,
	header_numbers: &BTreeSet<T::BlockNumber>,
) -> Result<FinalizedParaHeads, Error>
where
	u32: From<<T as subxt::Config>::BlockNumber>,
	T::BlockNumber: Ord + Zero,
{
	let subxt_block_number: subxt::rpc::BlockNumber = commitment_block_number.into();
	let block_hash = client.rpc().block_hash(Some(subxt_block_number)).await?;

	let mut para_ids = vec![];
	let key = runtime::api::storage().paras().parachains();
	let ids = client
		.storage()
		.fetch(&key, block_hash)
		.await?
		.ok_or_else(|| Error::Custom(format!("No ParaIds on relay chain?")))?;
	for id in ids {
		let key = runtime::api::storage().paras().para_lifecycles(&id);
		match client.storage().fetch(&key, block_hash).await?.expect("ParaId is known") {
			// only care about active parachains.
			ParaLifecycle::Parachain => para_ids.push(id),
			_ => {},
		}
	}
	let previous_finalized_block_number: subxt::rpc::BlockNumber = (latest_beefy_height + 1).into();
	let previous_finalized_hash = client
		.rpc()
		.block_hash(Some(previous_finalized_block_number))
		.await?
		.ok_or_else(|| {
			Error::Custom(
				"Failed to get previous finalized beefy block hash from block number".to_string(),
			)
		})?;

	let change_set = client
		.rpc()
		.query_storage(
			// we are interested only in the blocks where our parachain header changes.
			vec![parachain_header_storage_key(para_id).as_bytes_ref()],
			previous_finalized_hash,
			block_hash,
		)
		.await?;
	let mut finalized_blocks = BTreeMap::new();
	let mut leaf_indices = vec![];

	for changes in change_set {
		let header = client.rpc().header(Some(changes.block)).await?.ok_or_else(|| {
			Error::Custom(format!("[get_parachain_headers] block not found {:?}", changes.block))
		})?;

		let mut heads = BTreeMap::new();
		for id in para_ids.iter() {
			let key = runtime::api::storage().paras().heads(id);
			if let Some(head) = client.storage().fetch(&key, Some(header.hash())).await? {
				heads.insert(id.0, head.0);
			}
		}

		let para_header: T::Header = Decode::decode(&mut &heads[&para_id][..])
			.map_err(|_| Error::Custom(format!("Failed to decode header for {para_id}")))?;
		let para_block_number = *para_header.number();
		// skip genesis header or any unknown headers
		if para_block_number == Zero::zero() || !header_numbers.contains(&para_block_number) {
			continue
		}

		let block_number = u32::from(*header.number());
		finalized_blocks.insert(block_number as u64, heads);
		leaf_indices.push(get_leaf_index_for_block_number(beefy_activation_block, block_number));
	}

	Ok(FinalizedParaHeads { raw_finalized_heads: finalized_blocks, leaf_indices })
}

/// Get beefy justification for latest finalized beefy block
pub async fn fetch_beefy_justification<T: Config>(
	client: &OnlineClient<T>,
) -> Result<(SignedCommitment<u32, beefy_primitives::crypto::Signature>, T::Hash), Error> {
	let latest_beefy_finalized: <T as Config>::Hash =
		client.rpc().request("beefy_getFinalizedHead", rpc_params!()).await?;
	let block = client
		.rpc()
		.block(Some(latest_beefy_finalized))
		.await
		.ok()
		.flatten()
		.expect("Should find a valid block");

	let justifications = block.justifications.expect("Block should have valid justifications");

	let beefy_justification = justifications
		.into_justification(beefy_primitives::BEEFY_ENGINE_ID)
		.expect("Should have valid beefy justification");
	let VersionedFinalityProof::V1(signed_commitment) = VersionedFinalityProof::<
		u32,
		beefy_primitives::crypto::Signature,
	>::decode(&mut &*beefy_justification)
	.expect("Beefy justification should decode correctly");

	Ok((signed_commitment, latest_beefy_finalized))
}

/// Query a batch leaf proof
pub async fn fetch_mmr_batch_proof<T: Config>(
	client: &OnlineClient<T>,
	leaf_indices: Vec<u32>,
	block_hash: Option<T::Hash>,
) -> Result<LeafBatchProof<H256>, Error> {
	let proof: LeafBatchProof<H256> = client
		.rpc()
		.request("mmr_generateBatchProof", rpc_params!(leaf_indices, block_hash))
		.await?;
	Ok(proof)
}

/// Query a single leaf proof
pub async fn fetch_mmr_leaf_proof<T: Config>(
	client: &OnlineClient<T>,
	leaf_index: u64,
	block_hash: Option<T::Hash>,
) -> Result<LeafProof<H256>, Error> {
	let proof: LeafProof<H256> = client
		.rpc()
		.request("mmr_generateProof", rpc_params!(leaf_index, block_hash))
		.await?;

	Ok(proof)
}

/// This returns the storage key under which the parachain header with a given para_id is stored.
pub fn parachain_header_storage_key(para_id: u32) -> StorageKey {
	let mut storage_key = frame_support::storage::storage_prefix(b"Paras", b"Heads").to_vec();
	let encoded_para_id = para_id.encode();
	storage_key.extend_from_slice(sp_core::hashing::twox_64(&encoded_para_id).as_slice());
	storage_key.extend_from_slice(&encoded_para_id);
	StorageKey(storage_key)
}
