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

use core::marker::PhantomData;
#[cfg(test)]
use std::println as debug;

pub mod error;
pub mod primitives;
#[cfg(test)]
mod runtime;
#[cfg(test)]
mod tests;
pub mod traits;

use crate::error::BeefyClientError;
use crate::primitives::{
    BeefyNextAuthoritySet, KeccakHasher, MmrUpdateProof, ParachainsUpdateProof,
    SignatureWithAuthorityIndex, HASH_LENGTH,
};
use crate::traits::{AuthoritySet, HostFunctions, MmrState, StorageRead, StorageWrite};
use beefy_primitives::known_payload_ids::MMR_ROOT_ID;
use beefy_primitives::mmr::MmrLeaf;
use codec::Encode;
use sp_core::{ByteArray, H256};
use sp_runtime::traits::Convert;

use sp_std::prelude::*;

#[cfg(not(feature = "std"))]
use sp_std::vec;

pub struct BeefyLightClient<Store: StorageRead + StorageWrite, Crypto: HostFunctions> {
    store: Store,
    _phantom: PhantomData<Crypto>,
}

impl<Store: StorageRead + StorageWrite, Crypto: HostFunctions> BeefyLightClient<Store, Crypto> {
    /// Create a new instance of the light client
    pub fn new(store: Store) -> Self {
        Self {
            store,
            _phantom: PhantomData::default(),
        }
    }

    /// Return a reference to the underlying store
    pub fn store_ref(&self) -> &Store {
        &self.store
    }

    /// This should verify the signed commitment signatures, and reconstruct the
    /// authority merkle root, confirming known authorities signed the [`crate::primitives::Commitment`]
    /// then using the mmr proofs, verify the latest mmr leaf,
    /// using the latest mmr leaf to rotate its view of the next authorities.
    pub fn ingest_mmr_root_with_proof(
        &mut self,
        mmr_update: MmrUpdateProof,
    ) -> Result<(), BeefyClientError> {
        let authority_set = self.store.authority_set()?;
        let current_authority_set = &authority_set.current_authorities;
        let next_authority_set = &authority_set.next_authorities;
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
            return Err(BeefyClientError::InvalidMmrUpdate);
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
                    return Err(BeefyClientError::InvalidRootHash);
                }
            } else {
                return Err(BeefyClientError::InvalidMmrUpdate);
            }
        };

        let mmr_root_hash = H256::from_slice(&*mmr_root_vec);
        #[cfg(test)]
        debug!("Extracted mmr root hash: {:?}", mmr_root_hash);

        // Beefy validators sign the keccak_256 hash of the scale encoded commitment
        let encoded_commitment = mmr_update.signed_commitment.commitment.encode();
        let commitment_hash = <Crypto as HostFunctions>::keccak_256(&*encoded_commitment);

        #[cfg(test)]
        debug!("Recovering authority keys from signatures");
        let mut authority_indices = Vec::new();
        let authority_leaves = mmr_update
            .signed_commitment
            .signatures
            .into_iter()
            .map(|SignatureWithAuthorityIndex { index, signature }| {
                <Crypto as HostFunctions>::secp256k1_ecdsa_recover_compressed(
                    &signature,
                    &commitment_hash,
                )
                .map(|public_key_bytes| {
                    beefy_primitives::crypto::AuthorityId::from_slice(&public_key_bytes).ok()
                })
                .flatten()
                .map(|pub_key| {
                    authority_indices.push(index as usize);
                    <Crypto as HostFunctions>::keccak_256(
                        &beefy_mmr::BeefyEcdsaToEthereum::convert(pub_key),
                    )
                })
                .ok_or_else(|| BeefyClientError::InvalidSignature)
            })
            .collect::<Result<Vec<_>, BeefyClientError>>()?;

        let mut authorities_changed = false;

        let authorities_merkle_proof =
            rs_merkle::MerkleProof::<KeccakHasher>::new(mmr_update.authority_proof);

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
            _ => return Err(BeefyClientError::InvalidMmrUpdate),
        }

        let latest_beefy_height = self.store.mmr_state()?.latest_beefy_height;

        if mmr_update.signed_commitment.commitment.block_number <= latest_beefy_height {
            #[cfg(test)]
            debug!(
                "Invalid update, block_number {:?} <= latest_beefy_height {:?}",
                mmr_update.signed_commitment.commitment.block_number, latest_beefy_height
            );
            return Err(BeefyClientError::InvalidMmrUpdate);
        }

        // Move on to verify mmr_proof
        let node = pallet_mmr_primitives::DataOrHash::Data(mmr_update.latest_mmr_leaf.clone());

        #[cfg(test)]
        debug!("Verifying leaf proof {:?}", mmr_update.mmr_proof.clone());
        pallet_mmr::verify_leaf_proof::<sp_runtime::traits::Keccak256, _>(
            mmr_root_hash.into(),
            node,
            mmr_update.mmr_proof,
        )
        .map_err(|_| BeefyClientError::InvalidMmrProof)?;

        self.store.set_mmr_state(MmrState {
            latest_beefy_height: mmr_update.signed_commitment.commitment.block_number,
            mmr_root_hash: mmr_root_hash.into(),
        })?;

        if authorities_changed {
            self.store.set_authority_set(AuthoritySet {
                current_authorities: next_authority_set.clone(),
                next_authorities: mmr_update.latest_mmr_leaf.beefy_next_authority_set,
            })?;
        }
        Ok(())
    }

    pub fn verify_parachain_headers(
        &self,
        parachain_update: ParachainsUpdateProof,
    ) -> Result<(), BeefyClientError> {
        let mut mmr_leaves = Vec::new();

        for parachain_header in parachain_update.parachain_headers {
            let pair = (parachain_header.para_id, parachain_header.parachain_header);
            let leaf_bytes = pair.encode();

            let proof =
                rs_merkle::MerkleProof::<KeccakHasher>::new(parachain_header.parachain_heads_proof);
            let leaf_hash = <Crypto as HostFunctions>::keccak_256(&leaf_bytes);
            let root = proof
                .root(
                    &vec![parachain_header.heads_leaf_index as usize],
                    &vec![leaf_hash],
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
                parachain_heads: root.into(),
            };

            let node = pallet_mmr_primitives::DataOrHash::Data(mmr_leaf);
            mmr_leaves.push(node);
        }

        let mmr_state = self
            .store
            .mmr_state()
            .map_err(|_| BeefyClientError::StorageReadError)?;

        #[cfg(test)]
        debug!(
            "Verifying leaves proof {:?}, root hash {:?}",
            parachain_update.mmr_proof.clone(),
            mmr_state.mmr_root_hash
        );

        pallet_mmr::verify_leaves_proof::<sp_runtime::traits::Keccak256, _>(
            mmr_state.mmr_root_hash.into(),
            mmr_leaves,
            parachain_update.mmr_proof,
        )
        .map_err(|_| BeefyClientError::InvalidMmrProof)?;
        Ok(())
    }
}

fn validate_sigs_against_threshold(set: &BeefyNextAuthoritySet<H256>, sigs_len: usize) -> bool {
    let threshold = ((2 * set.len) / 3) + 1;
    sigs_len >= threshold as usize
}
