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

//! Beefy Light Client Implementation
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "prover"), deny(missing_docs))]

use core::marker::PhantomData;

pub mod error;
pub mod primitives;
#[cfg(any(test, feature = "prover"))]
pub mod queries;
#[cfg(test)]
mod tests;
pub mod traits;

use crate::error::BeefyClientError;
use crate::primitives::{
    BeefyNextAuthoritySet, MmrUpdateProof, ParachainsUpdateProof, SignatureWithAuthorityIndex,
    HASH_LENGTH,
};
use crate::traits::{ClientState, HostFunctions};
use beefy_primitives::known_payload_ids::MMR_ROOT_ID;
use beefy_primitives::mmr::MmrLeaf;
use codec::Encode;
use frame_support::sp_runtime::app_crypto::ByteArray;
use frame_support::sp_runtime::traits::Convert;
use sp_core::H256;

use sp_std::prelude::*;
use sp_std::vec;

/// Beefy light client
#[derive(Clone)]
pub struct BeefyLightClient<Crypto: HostFunctions + Clone>(PhantomData<Crypto>);

impl<Crypto: HostFunctions + Clone> BeefyLightClient<Crypto> {
    /// Create a new instance of the light client
    pub fn new() -> Self {
        Self(PhantomData::default())
    }

    /// This should verify the signed commitment signatures, and reconstruct the
    /// authority merkle root, confirming known authorities signed the [`crate::primitives::Commitment`]
    /// then using the mmr proofs, verify the latest mmr leaf,
    /// using the latest mmr leaf to rotate its view of the next authorities.
    pub fn verify_mmr_root_with_proof(
        &mut self,
        mut trusted_client_state: ClientState,
        mmr_update: MmrUpdateProof,
    ) -> Result<ClientState, BeefyClientError> {
        let current_authority_set = &trusted_client_state.current_authorities;
        let next_authority_set = &trusted_client_state.next_authorities;
        let signatures_len = mmr_update.signed_commitment.signatures.len();
        let validator_set_id = mmr_update.signed_commitment.commitment.validator_set_id;

        // If signature threshold is not satisfied, return
        if !validate_sigs_against_threshold(current_authority_set, signatures_len)
            && !validate_sigs_against_threshold(next_authority_set, signatures_len)
        {
            return Err(BeefyClientError::IncompleteSignatureThreshold);
        }

        if current_authority_set.id != validator_set_id && next_authority_set.id != validator_set_id
        {
            return Err(BeefyClientError::AuthoritySetMismatch {
                current_set_id: current_authority_set.id,
                next_set_id: next_authority_set.id,
                commitment_set_id: validator_set_id,
            });
        }

        // Extract root hash from signed commitment and validate it
        let mmr_root_vec = {
            if let Some(root) = mmr_update
                .signed_commitment
                .commitment
                .payload
                .get_raw(&MMR_ROOT_ID)
            {
                if root.len() == HASH_LENGTH {
                    root
                } else {
                    return Err(BeefyClientError::InvalidRootHash {
                        root_hash: root.clone(),
                        len: root.len() as u64,
                    });
                }
            } else {
                return Err(BeefyClientError::MmrRootHashNotFound);
            }
        };

        let mmr_root_hash = H256::from_slice(&*mmr_root_vec);

        // Beefy validators sign the keccak_256 hash of the scale encoded commitment
        let encoded_commitment = mmr_update.signed_commitment.commitment.encode();
        let commitment_hash = Crypto::keccak_256(&*encoded_commitment);

        let mut authority_indices = Vec::new();
        let authority_leaves = mmr_update
            .signed_commitment
            .signatures
            .into_iter()
            .map(|SignatureWithAuthorityIndex { index, signature }| {
                Crypto::secp256k1_ecdsa_recover_compressed(&signature, &commitment_hash)
                    .and_then(|public_key_bytes| {
                        beefy_primitives::crypto::AuthorityId::from_slice(&public_key_bytes).ok()
                    })
                    .map(|pub_key| {
                        authority_indices.push(index as usize);
                        Crypto::keccak_256(&beefy_mmr::BeefyEcdsaToEthereum::convert(pub_key))
                    })
                    .ok_or(BeefyClientError::InvalidSignature)
            })
            .collect::<Result<Vec<_>, BeefyClientError>>()?;

        let mut authorities_changed = false;

        let authorities_merkle_proof =
            rs_merkle::MerkleProof::<MerkleHasher<Crypto>>::new(mmr_update.authority_proof);
        // Verify mmr_update.authority_proof against store root hash
        match validator_set_id {
            id if id == current_authority_set.id => {
                let root_hash = current_authority_set.root;
                if !authorities_merkle_proof.verify(
                    root_hash.into(),
                    &authority_indices,
                    &authority_leaves,
                    current_authority_set.len as usize,
                ) {
                    return Err(BeefyClientError::InvalidAuthorityProof);
                }
            }
            id if id == next_authority_set.id => {
                let root_hash = next_authority_set.root;
                if !authorities_merkle_proof.verify(
                    root_hash.into(),
                    &authority_indices,
                    &authority_leaves,
                    next_authority_set.len as usize,
                ) {
                    return Err(BeefyClientError::InvalidAuthorityProof);
                }
                authorities_changed = true;
            }
            _ => {
                return Err(BeefyClientError::AuthoritySetMismatch {
                    current_set_id: current_authority_set.id,
                    next_set_id: next_authority_set.id,
                    commitment_set_id: validator_set_id,
                })
            }
        }

        let latest_beefy_height = trusted_client_state.latest_beefy_height;

        let commitment_block_number = mmr_update.signed_commitment.commitment.block_number;
        if commitment_block_number <= latest_beefy_height {
            return Err(BeefyClientError::OutdatedCommitment {
                latest_beefy_height,
                commitment_block_number,
            });
        }

        // Move on to verify mmr_proof
        let node = mmr_update
            .latest_mmr_leaf
            .using_encoded(|leaf| Crypto::keccak_256(leaf));

        let mmr_size = NodesUtils::new(mmr_update.mmr_proof.leaf_count).size();
        let proof = mmr_lib::MerkleProof::<_, MerkleHasher<Crypto>>::new(
            mmr_size,
            mmr_update.mmr_proof.items,
        );

        let leaf_pos = mmr_lib::leaf_index_to_pos(mmr_update.mmr_proof.leaf_index);

        let root = proof.calculate_root(vec![(leaf_pos, node.into())])?;
        if root != mmr_root_hash {
            return Err(BeefyClientError::InvalidMmrProof);
        }

        trusted_client_state.latest_beefy_height =
            mmr_update.signed_commitment.commitment.block_number;
        trusted_client_state.mmr_root_hash = mmr_root_hash;

        if authorities_changed {
            trusted_client_state.current_authorities = next_authority_set.clone();
            trusted_client_state.next_authorities =
                mmr_update.latest_mmr_leaf.beefy_next_authority_set;
        }
        Ok(trusted_client_state)
    }

    /// Takes the updated client state and parachains headers update proof
    /// and verifies inclusion in mmr
    pub fn verify_parachain_headers(
        &self,
        trusted_client_state: ClientState,
        parachain_update: ParachainsUpdateProof,
    ) -> Result<(), BeefyClientError> {
        let mut mmr_leaves = Vec::new();

        for parachain_header in parachain_update.parachain_headers {
            let pair = (parachain_header.para_id, parachain_header.parachain_header);
            let leaf_bytes = pair.encode();

            let proof = rs_merkle::MerkleProof::<MerkleHasher<Crypto>>::new(
                parachain_header.parachain_heads_proof,
            );
            let leaf_hash = Crypto::keccak_256(&leaf_bytes);
            let root = proof
                .root(
                    &[parachain_header.heads_leaf_index as usize],
                    &[leaf_hash],
                    parachain_header.heads_total_count as usize,
                )
                .map_err(|_| BeefyClientError::InvalidMerkleProof)?;
            // reconstruct leaf
            let mmr_leaf = MmrLeaf {
                version: parachain_header.partial_mmr_leaf.version,
                parent_number_and_hash: parachain_header.partial_mmr_leaf.parent_number_and_hash,
                beefy_next_authority_set: parachain_header
                    .partial_mmr_leaf
                    .beefy_next_authority_set,
                leaf_extra: H256::from_slice(&root),
            };

            let node = mmr_leaf.using_encoded(|leaf| Crypto::keccak_256(leaf));
            let leaf_index = get_leaf_index_for_block_number(
                trusted_client_state.beefy_activation_block,
                parachain_header.partial_mmr_leaf.parent_number_and_hash.0 + 1,
            );

            let leaf_pos = mmr_lib::leaf_index_to_pos(leaf_index as u64);
            mmr_leaves.push((leaf_pos, H256::from_slice(&node)));
        }

        let mmr_size = NodesUtils::new(parachain_update.mmr_proof.leaf_count).size();
        let proof = mmr_lib::MerkleProof::<_, MerkleHasher<Crypto>>::new(
            mmr_size,
            parachain_update.mmr_proof.items,
        );

        let root = proof.calculate_root(mmr_leaves)?;
        if root != trusted_client_state.mmr_root_hash {
            return Err(BeefyClientError::InvalidMmrProof);
        }
        Ok(())
    }
}

/// Calculate the leaf index for this block number
pub fn get_leaf_index_for_block_number(activation_block: u32, block_number: u32) -> u32 {
    // calculate the leaf index for this leaf.
    if activation_block == 0 {
        // in this case the leaf index is the same as the block number - 1 (leaf index starts at 0)
        block_number - 1
    } else {
        // in this case the leaf index is activation block - current block number.
        activation_block - (block_number + 1)
    }
}

/// Merkle Hasher for mmr library
#[derive(Clone)]
pub struct MerkleHasher<T: HostFunctions>(PhantomData<T>);

impl<T: HostFunctions + Clone> mmr_lib::Merge for MerkleHasher<T> {
    type Item = H256;

    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut concat = left.as_bytes().to_vec();
        concat.extend_from_slice(right.as_bytes());
        T::keccak_256(&*concat).into()
    }
}

impl<T: HostFunctions + Clone> rs_merkle::Hasher for MerkleHasher<T> {
    type Hash = [u8; 32];
    fn hash(data: &[u8]) -> Self::Hash {
        T::keccak_256(data)
    }
}

/// Validate signatures against threshold
fn validate_sigs_against_threshold(set: &BeefyNextAuthoritySet<H256>, sigs_len: usize) -> bool {
    let threshold = ((2 * set.len) / 3) + 1;
    sigs_len >= threshold as usize
}

/// MMR nodes & size -related utilities.
pub struct NodesUtils {
    no_of_leaves: u64,
}

impl NodesUtils {
    /// Create new instance of MMR nodes utilities for given number of leaves.
    pub fn new(no_of_leaves: u64) -> Self {
        Self { no_of_leaves }
    }

    /// Calculate number of peaks in the MMR.
    pub fn number_of_peaks(&self) -> u64 {
        self.number_of_leaves().count_ones() as u64
    }

    /// Return the number of leaves in the MMR.
    pub fn number_of_leaves(&self) -> u64 {
        self.no_of_leaves
    }

    /// Calculate the total size of MMR (number of nodes).
    pub fn size(&self) -> u64 {
        2 * self.no_of_leaves - self.number_of_peaks()
    }

    /// Calculate maximal depth of the MMR.
    pub fn _depth(&self) -> u32 {
        if self.no_of_leaves == 0 {
            return 0;
        }

        64 - self.no_of_leaves.next_power_of_two().leading_zeros()
    }
}
