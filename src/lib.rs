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
#![cfg_attr(not(feature = "std"), no_std)]

pub mod error;
pub mod primitives;
pub mod traits;

use crate::error::BeefyClientError;
use crate::primitives::{BeefyAuthoritySet, Hasher, MmrUpdateProof, MMR_ROOT_ID};
use crate::traits::{StorageRead, StorageWrite};
use codec::Encode;
use sp_core::{ByteArray, H256};
use sp_core_hashing::keccak_256;
use sp_io::crypto;
use sp_runtime::traits::Convert;

use sp_std::prelude::*;
#[cfg(not(feature = "std"))]
use sp_std::vec;

pub trait BeefyLightClient {
    type Store: StorageRead + StorageWrite;

    /// This should verify the signed commitment signatures, and reconstruct the
    /// authority merkle root, confirming known authorities signed the [`crate::primitives::Commitment`]
    /// then using the mmr proofs, verify the latest mmr leaf,
    /// using the latest mmr leaf to rotate its view of the next authorities.
    fn ingest_mmr_root_with_proof(mmr_update: MmrUpdateProof) -> Result<(), BeefyClientError> {
        let current_authority_set = Self::Store::current_authority_set()?;
        let next_authority_set = Self::Store::next_authority_set()?;
        let signatures_len = mmr_update.signed_commitment.signatures.len();
        let validator_set_id = mmr_update.signed_commitment.commitment.validator_set_id;

        // If signature threshold is not satisfied, return
        if !validate_sigs_against_threshold(&current_authority_set, signatures_len)
            && !validate_sigs_against_threshold(&next_authority_set, signatures_len)
        {
            return Err(BeefyClientError::InvalidMmrUpdate);
        }

        if current_authority_set.id != validator_set_id && next_authority_set.id != validator_set_id
        {
            return Err(BeefyClientError::InvalidMmrUpdate);
        }

        // Beefy validators sign the kekak256 hash of the scale encoded commitment
        let encoded_commitment = mmr_update.signed_commitment.commitment.encode();
        let commitment_hash = keccak_256(&*encoded_commitment);

        let authority_leaves = mmr_update
            .signed_commitment
            .signatures
            .into_iter()
            .map(|sig| {
                crypto::secp256k1_ecdsa_recover(&sig.signature, &commitment_hash)
                    .map(|public_key_bytes| {
                        beefy_primitives::crypto::AuthorityId::from_slice(&public_key_bytes).ok()
                    })
                    .ok()
                    .flatten()
                    .map(|pub_key| beefy_mmr::BeefyEcdsaToEthereum::convert(pub_key.clone()))
                    .ok_or_else(|| BeefyClientError::InvalidSignature)
            })
            .collect::<Result<Vec<_>, BeefyClientError>>()?;

        let mut authorities_changed = false;
        let root =
            beefy_merkle_tree::merkle_root::<beefy_merkle_tree::Keccak256, _, _>(authority_leaves);
        if current_authority_set.id == validator_set_id {
            if current_authority_set.merkle_root != root.into() {
                return Err(BeefyClientError::InvalidRootHash);
            }
        } else if next_authority_set.id == validator_set_id {
            if next_authority_set.merkle_root != root.into() {
                return Err(BeefyClientError::InvalidRootHash);
            }

            authorities_changed = true;
        }

        let latest_beefy_height = Self::Store::latest_height()?;

        if mmr_update.signed_commitment.commitment.block_number <= latest_beefy_height {
            return Err(BeefyClientError::InvalidMmrUpdate);
        }

        // Move on to verify mmr_proof
        let mmr_size = mmr_lib::leaf_index_to_mmr_size(mmr_update.latest_mmr_leaf_with_index.index);
        let proof = mmr_lib::MerkleProof::<H256, Hasher<H256>>::new(mmr_size, mmr_update.mmr_proof);
        let mmr_root_vec = mmr_update
            .signed_commitment
            .commitment
            .payload
            .into_iter()
            .find(|item| item.0 == MMR_ROOT_ID)
            .ok_or_else(|| BeefyClientError::InvalidMmrUpdate)?
            .1
            .clone();
        if mmr_root_vec.len() != 32 {
            return Err(BeefyClientError::InvalidRootHash);
        }
        let mut mmr_root_hash = [0u8; 32];
        mmr_root_hash.copy_from_slice(&mmr_root_vec);

        let leaf_pos = mmr_lib::leaf_index_to_pos(mmr_update.latest_mmr_leaf_with_index.index);
        let mmr_leaf_hash = keccak_256(&mmr_update.latest_mmr_leaf_with_index.leaf.encode());

        match proof.verify(mmr_root_hash.into(), vec![(leaf_pos, mmr_leaf_hash.into())]) {
            Ok(false) | Err(_) => return Err(BeefyClientError::InvalidMmrProof),
            _ => {}
        }

        Ok(())
    }
}

fn authority_threshold(set: &BeefyAuthoritySet) -> u64 {
    ((2 * set.len) / 3) + 1
}

fn validate_sigs_against_threshold(set: &BeefyAuthoritySet, sigs_len: usize) -> bool {
    let threshold = authority_threshold(set);
    sigs_len >= threshold as usize
}
