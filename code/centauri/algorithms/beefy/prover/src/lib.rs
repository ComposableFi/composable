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

//! BEEFY prover utilities

#![allow(clippy::all)]
#![deny(missing_docs)]

/// Errors that can be encountered by the prover
pub mod error;
/// Helper functions and types
pub mod helpers;
/// Methods for querying the relay chain
pub mod relay_chain_queries;
/// Metadata generated code for interacting with the relay chain
pub mod runtime;

use beefy_light_client_primitives::{
	get_leaf_index_for_block_number, ClientState, HostFunctions, MerkleHasher, MmrUpdateProof,
	ParachainHeader, PartialMmrLeaf, SignedCommitment,
};
use beefy_primitives::{
	known_payload_ids::MMR_ROOT_ID,
	mmr::{BeefyNextAuthoritySet, MmrLeaf},
};
use codec::{Decode, Encode};
use error::Error;
use helpers::{
	fetch_timestamp_extrinsic_with_proof, hash_authority_addresses, prove_parachain_headers,
	ParaHeadsProof, TimeStampExtWithProof,
};
use hex_literal::hex;
use pallet_mmr_primitives::BatchProof;
use relay_chain_queries::{fetch_beefy_justification, fetch_mmr_batch_proof};
use sp_core::{hexdisplay::AsBytesRef, keccak_256, H256};
use sp_io::crypto;
use sp_runtime::traits::{BlakeTwo256, Header as HeaderT};
use subxt::{rpc::rpc_params, Config, OnlineClient};

use crate::{
	relay_chain_queries::parachain_header_storage_key,
	runtime::api::runtime_types::polkadot_parachain::primitives::Id,
};
use helpers::{prove_authority_set, AuthorityProofWithSignatures};
use relay_chain_queries::{
	fetch_finalized_parachain_heads, fetch_mmr_leaf_proof, FinalizedParaHeads,
};

/// Host function implementation for beefy light client.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Crypto;

impl light_client_common::HostFunctions for Crypto {
	type BlakeTwo256 = BlakeTwo256;
}

impl HostFunctions for Crypto {
	fn keccak_256(input: &[u8]) -> [u8; 32] {
		keccak_256(input)
	}

	fn secp256k1_ecdsa_recover_compressed(
		signature: &[u8; 65],
		value: &[u8; 32],
	) -> Option<Vec<u8>> {
		crypto::secp256k1_ecdsa_recover_compressed(signature, value)
			.ok()
			.map(|val| val.to_vec())
	}
}

/// This contains methods for fetching BEEFY proofs for parachain headers.
pub struct Prover<T: Config> {
	/// Subxt client for the relay chain
	pub relay_client: OnlineClient<T>,
	/// Subxt client for the parachain
	pub para_client: OnlineClient<T>,
	/// Block at which beefy was activated
	pub beefy_activation_block: u32,
	/// Para Id for the associated parachain.
	pub para_id: u32,
}

impl<T: Config> Prover<T>
where
	u32: From<<T as subxt::Config>::BlockNumber>,
{
	/// Returns the initial state for bootstrapping a BEEFY light client.
	pub async fn get_initial_client_state(client: Option<&OnlineClient<T>>) -> ClientState {
		if client.is_none() {
			return ClientState {
				latest_beefy_height: 0,
				mmr_root_hash: Default::default(),
				current_authorities: BeefyNextAuthoritySet {
					id: 0,
					len: 5,
					root: H256::from(hex!(
						"baa93c7834125ee3120bac6e3342bd3f28611110ad21ab6075367abdffefeb09"
					)),
				},
				next_authorities: BeefyNextAuthoritySet {
					id: 1,
					len: 5,
					root: H256::from(hex!(
						"baa93c7834125ee3120bac6e3342bd3f28611110ad21ab6075367abdffefeb09"
					)),
				},
				beefy_activation_block: 0,
			}
		}
		// Get initial validator set
		// In development mode validators are the same for all sessions only validator set_id
		// changes
		let client = client.expect("Client should be defined");
		let latest_beefy_finalized: <T as Config>::Hash =
			client.rpc().request("beefy_getFinalizedHead", rpc_params!()).await.unwrap();
		let header = client.rpc().header(Some(latest_beefy_finalized)).await.unwrap().unwrap();
		let validator_set_id = {
			let key = runtime::api::storage().beefy().validator_set_id();
			client.storage().fetch(&key, None).await.unwrap().unwrap()
		};

		let next_val_set = {
			let key = runtime::api::storage().mmr_leaf().beefy_next_authorities();
			client
				.storage()
				.fetch(&key, None)
				.await
				.unwrap()
				.expect("Authorirty set should be defined")
		};
		let latest_beefy_height: u64 = (*header.number()).into();
		ClientState {
			latest_beefy_height: latest_beefy_height as u32,
			mmr_root_hash: Default::default(),
			current_authorities: BeefyNextAuthoritySet {
				id: validator_set_id,
				len: next_val_set.len,
				root: next_val_set.root,
			},
			next_authorities: BeefyNextAuthoritySet {
				id: validator_set_id + 1,
				len: next_val_set.len,
				root: next_val_set.root,
			},
			beefy_activation_block: 0,
		}
	}

	/// Use this fetch all parachain headers finalized at this new
	/// beefy height.
	pub async fn query_finalized_parachain_headers_at(
		&self,
		commitment_block_number: u32,
		latest_beefy_height: u32,
	) -> Result<Vec<T::Header>, Error>
	where
		u32: From<T::BlockNumber>,
		T::BlockNumber: From<u32>,
	{
		let subxt_block_number: subxt::rpc::BlockNumber = commitment_block_number.into();
		let block_hash = self.relay_client.rpc().block_hash(Some(subxt_block_number)).await?;
		let previous_finalized_block_number: subxt::rpc::BlockNumber =
			(latest_beefy_height + 1).into();
		let previous_finalized_hash = self
			.relay_client
			.rpc()
			.block_hash(Some(previous_finalized_block_number))
			.await?
			.ok_or_else(|| {
				Error::Custom(
					"Failed to get previous finalized beefy block hash from block number"
						.to_string(),
				)
			})?;

		let change_set = self
			.relay_client
			.rpc()
			.query_storage(
				// we are interested only in the blocks where our parachain header changes.
				vec![parachain_header_storage_key(self.para_id).as_bytes_ref()],
				previous_finalized_hash,
				block_hash,
			)
			.await?;
		let mut headers = vec![];
		for changes in change_set {
			let header =
				self.relay_client.rpc().header(Some(changes.block)).await?.ok_or_else(|| {
					Error::Custom(format!(
						"[get_parachain_headers] block not found {:?}",
						changes.block
					))
				})?;

			let key = runtime::api::storage().paras().heads(&Id(self.para_id));
			let head = self
				.relay_client
				.storage()
				.fetch(&key, Some(header.hash()))
				.await?
				.expect("Header exists in its own changeset; qed");

			let para_header = T::Header::decode(&mut &head.0[..])
				.map_err(|_| Error::Custom(format!("Failed to decode header")))?;
			headers.push(para_header);
		}

		Ok(headers)
	}

	/// This will query the finalized parachain headers between the given relay chain blocks
	/// Only including the given parachain headers into the [`MmrBatchProof`]
	pub async fn query_finalized_parachain_headers_with_proof(
		&self,
		commitment_block_number: u32,
		latest_beefy_height: u32,
		header_numbers: Vec<T::BlockNumber>,
	) -> Result<(Vec<ParachainHeader>, BatchProof<H256>), Error>
	where
		T::BlockNumber: Ord + sp_runtime::traits::Zero,
	{
		let header_numbers = header_numbers.into_iter().collect();
		let FinalizedParaHeads { leaf_indices, raw_finalized_heads: finalized_blocks } =
			fetch_finalized_parachain_heads(
				&self.relay_client,
				commitment_block_number,
				latest_beefy_height,
				self.para_id,
				self.beefy_activation_block,
				&header_numbers,
			)
			.await?;

		let subxt_block_number: subxt::rpc::BlockNumber = commitment_block_number.into();
		let block_hash = self.relay_client.rpc().block_hash(Some(subxt_block_number)).await?;

		let batch_proof =
			fetch_mmr_batch_proof(&self.relay_client, leaf_indices, block_hash).await?;

		let leaves: Vec<Vec<u8>> = Decode::decode(&mut &*batch_proof.leaves.to_vec())?;

		let mut parachain_headers = vec![];
		for leaf_bytes in leaves {
			let leaf: MmrLeaf<u32, H256, H256, H256> = Decode::decode(&mut &*leaf_bytes)?;
			let parent_block: u32 = leaf.parent_number_and_hash.0.into();
			let leaf_block_number = (parent_block + 1) as u64;
			let para_headers = finalized_blocks.get(&leaf_block_number).ok_or_else(|| {
				Error::Custom(format!(
					"[get_parachain_headers] Para Headers not found for relay chain block {}",
					leaf_block_number
				))
			})?;
			let ParaHeadsProof {
				parachain_heads_proof,
				para_head,
				heads_leaf_index,
				heads_total_count,
			} = prove_parachain_headers(&para_headers, self.para_id)?;

			let decoded_para_head = T::Header::decode(&mut &para_head[..])?;
			let TimeStampExtWithProof { ext: timestamp_extrinsic, proof: extrinsic_proof } =
				fetch_timestamp_extrinsic_with_proof(
					&self.para_client,
					Some(decoded_para_head.hash()),
				)
				.await?;

			let header = ParachainHeader {
				parachain_header: para_head,
				partial_mmr_leaf: PartialMmrLeaf {
					version: leaf.version,
					parent_number_and_hash: leaf.parent_number_and_hash,
					beefy_next_authority_set: leaf.beefy_next_authority_set.clone(),
				},
				para_id: self.para_id,
				parachain_heads_proof,
				heads_leaf_index,
				heads_total_count,
				extrinsic_proof,
				timestamp_extrinsic,
			};

			parachain_headers.push(header);
		}

		let batch_proof: pallet_mmr_primitives::BatchProof<H256> =
			Decode::decode(&mut batch_proof.proof.0.as_slice())?;
		Ok((parachain_headers, batch_proof))
	}

	/// This will fetch the latest leaf in the mmr as well as a proof for this leaf in the latest
	/// mmr root hash.
	pub async fn fetch_mmr_update_proof_for(
		&self,
		signed_commitment: beefy_primitives::SignedCommitment<
			u32,
			beefy_primitives::crypto::Signature,
		>,
	) -> Result<MmrUpdateProof, Error> {
		let subxt_block_number: subxt::rpc::BlockNumber =
			signed_commitment.commitment.block_number.into();
		let block_hash = self.relay_client.rpc().block_hash(Some(subxt_block_number)).await?;

		let current_authorities = {
			let key = runtime::api::storage().beefy().authorities();
			self.relay_client
				.storage()
				.fetch(&key, block_hash)
				.await?
				.ok_or_else(|| Error::Custom(format!("No beefy authorities found!")))?
				.0
		};

		// Current LeafIndex
		let block_number = signed_commitment.commitment.block_number;
		let leaf_index = get_leaf_index_for_block_number(self.beefy_activation_block, block_number);
		let leaf_proof =
			fetch_mmr_leaf_proof(&self.relay_client, leaf_index.into(), block_hash).await?;

		let opaque_leaf: Vec<u8> = codec::Decode::decode(&mut &*leaf_proof.leaf.0)?;
		let latest_leaf: MmrLeaf<u32, H256, H256, H256> =
			codec::Decode::decode(&mut &*opaque_leaf)?;
		let mmr_proof: pallet_mmr_primitives::Proof<H256> =
			codec::Decode::decode(&mut &*leaf_proof.proof.0)?;

		let authority_address_hashes = hash_authority_addresses(
			current_authorities.into_iter().map(|x| x.encode()).collect(),
		)?;

		let AuthorityProofWithSignatures { authority_proof, signatures } =
			prove_authority_set(&signed_commitment, authority_address_hashes)?;

		Ok(MmrUpdateProof {
			signed_commitment: SignedCommitment {
				commitment: signed_commitment.commitment.clone(),
				signatures,
			},
			latest_mmr_leaf: latest_leaf.clone(),
			mmr_proof,
			authority_proof,
		})
	}

	/// Construct a beefy client state to be submitted to the counterparty chain
	pub async fn construct_beefy_client_state(
		&self,
		beefy_activation_block: u32,
	) -> Result<ClientState, Error> {
		let (signed_commitment, latest_beefy_finalized) =
			fetch_beefy_justification(&self.relay_client).await?;

		// Encoding and decoding to fix dependency version conflicts
		let next_authority_set = {
			let key = runtime::api::storage().mmr_leaf().beefy_next_authorities();
			self.relay_client
				.storage()
				.fetch(&key, Some(latest_beefy_finalized))
				.await?
				.expect("Should retrieve next authority set")
				.encode()
		};
		let next_authority_set = BeefyNextAuthoritySet::decode(&mut &*next_authority_set)
			.expect("Should decode next authority set correctly");

		let current_authorities = {
			let key = runtime::api::storage().beefy().authorities();
			self.relay_client
				.storage()
				.fetch(&key, Some(latest_beefy_finalized))
				.await?
				.expect("Should retrieve next authority set")
				.0
		};

		let authority_address_hashes = hash_authority_addresses(
			current_authorities.into_iter().map(|x| x.encode()).collect(),
		)?;
		let tree =
			rs_merkle::MerkleTree::<MerkleHasher<Crypto>>::from_leaves(&authority_address_hashes);

		let authority_root = tree.root().expect("Should generate root");
		let authority_root: H256 = authority_root.into();
		let current_authority_set = BeefyNextAuthoritySet {
			id: next_authority_set.id - 1,
			len: authority_address_hashes.len() as u32,
			root: authority_root,
		};

		let mmr_root_hash = signed_commitment
			.commitment
			.payload
			.get_decoded::<H256>(&MMR_ROOT_ID)
			.expect("Mmr root hash should decode correctly");

		let client_state = ClientState {
			mmr_root_hash,
			latest_beefy_height: signed_commitment.commitment.block_number as u32,
			beefy_activation_block,
			current_authorities: current_authority_set.clone(),
			next_authorities: next_authority_set.clone(),
		};

		Ok(client_state)
	}
}
