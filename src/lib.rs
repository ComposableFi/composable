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
use crate::primitives::{BeefyAuthoritySet, MmrUpdateProof};
use crate::traits::{StorageRead, StorageWrite};

pub trait BeefyLightClient {
    type store: StorageRead + StorageWrite;

    /// This should verify the signed commitment signatures, and reconstruct the
    /// authority merkle root, confirming known authorities signed the [`crate::primitives::Commitment`]
    /// then using the mmr proofs, verify the latest mmr leaf,
    /// using the latest mmr leaf to rotate its view of the next authorities.
    fn ingest_mmr_root_with_proof(mmr_update: MmrUpdateProof) -> Result<(), BeefyClientError> {
        Ok(())
    }
}

fn authority_threshold(set: BeefyAuthoritySet) -> u64 {
    ((2 * set.len) / 3) + 1
}
