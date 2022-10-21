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

//! Primitive types and traits used by the GRANDPA prover & verifier.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]
#![deny(missing_docs)]

extern crate alloc;
extern crate core;

use alloc::collections::BTreeMap;
use codec::{Decode, Encode};
use core::fmt::Debug;
use sp_core::{ed25519, sp_std, H256};
use sp_finality_grandpa::{AuthorityId, AuthorityList, AuthoritySignature};
use sp_runtime::traits::Header;
use sp_std::prelude::*;
use sp_storage::StorageKey;

/// GRANPA errors
pub mod error;
/// GRANDPA justification utilities
pub mod justification;

/// A commit message for this chain's block type.
pub type Commit<H> = finality_grandpa::Commit<
	<H as Header>::Hash,
	<H as Header>::Number,
	AuthoritySignature,
	AuthorityId,
>;

/// Finality for block B is proved by providing:
/// 1) the justification for the descendant block F;
/// 2) headers sub-chain (B; F] if B != F;
#[derive(Debug, PartialEq, Encode, Decode, Clone)]
pub struct FinalityProof<H: Header> {
	/// The hash of block F for which justification is provided.
	pub block: H::Hash,
	/// Justification of the block F.
	pub justification: Vec<u8>,
	/// The set of headers in the range (B; F] that we believe are unknown to the caller. Ordered.
	pub unknown_headers: Vec<H>,
}

/// Previous light client state.
#[derive(Clone)]
pub struct ClientState<H> {
	/// Current authority set
	pub current_authorities: AuthorityList,
	/// Id of the current authority set.
	pub current_set_id: u64,
	/// latest finalized height on the relay chain.
	pub latest_relay_height: u32,
	/// latest finalized height on the parachain.
	pub latest_para_height: u32,
	/// latest finalized hash on the relay chain.
	pub latest_relay_hash: H,
	/// para_id of associated parachain
	pub para_id: u32,
}

/// Holds relavant parachain proofs for both header and timestamp extrinsic.
#[derive(Clone, Debug)]
pub struct ParachainHeaderProofs {
	/// State proofs that prove a parachain header exists at a given relay chain height
	pub state_proof: Vec<Vec<u8>>,
	/// Timestamp extrinsic for ibc
	pub extrinsic: Vec<u8>,
	/// Timestamp extrinsic proof for previously proven parachain header.
	pub extrinsic_proof: Vec<Vec<u8>>,
}

/// Parachain headers with a Grandpa finality proof.
#[derive(Clone)]
pub struct ParachainHeadersWithFinalityProof<H: Header> {
	/// The grandpa finality proof: contains relay chain headers from the
	/// last known finalized grandpa block.
	pub finality_proof: FinalityProof<H>,
	/// Contains a map of relay chain header hashes to parachain headers
	/// finalzed at the relay chain height. We check for this parachain header finalization
	/// via state proofs. Also contains extrinsic proof for timestamp.
	pub parachain_headers: BTreeMap<H::Hash, ParachainHeaderProofs>,
}

/// Host functions that allow the light client perform cryptographic operations in native.
pub trait HostFunctions: light_client_common::HostFunctions + 'static {
	/// TODO: docs
	type Header: Header;

	/// Verify an ed25519 signature
	fn ed25519_verify(sig: &ed25519::Signature, msg: &[u8], pub_key: &ed25519::Public) -> bool;
	/// TODO: docs
	fn add_relaychain_headers(header: &Self::Header);
	/// TODO: docs
	fn get_relaychain_headers(hash: H256) -> Option<Self::Header>;
}

/// This returns the storage key for a parachain header on the relay chain.
pub fn parachain_header_storage_key(para_id: u32) -> StorageKey {
	let mut storage_key = frame_support::storage::storage_prefix(b"Paras", b"Heads").to_vec();
	let encoded_para_id = para_id.encode();
	storage_key.extend_from_slice(sp_io::hashing::twox_64(&encoded_para_id).as_slice());
	storage_key.extend_from_slice(&encoded_para_id);
	StorageKey(storage_key)
}
