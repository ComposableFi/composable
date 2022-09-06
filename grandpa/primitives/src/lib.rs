#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::collections::BTreeMap;
use codec::{Decode, Encode};
use sp_core::{ed25519, sp_std};
use sp_finality_grandpa::{AuthorityId, AuthorityList, AuthoritySignature};
use sp_runtime::traits::{Block, Header, NumberFor};
use sp_std::prelude::*;
use sp_storage::StorageKey;
use sp_trie::StorageProof;

pub mod error;

/// A commit message for this chain's block type.
pub type Commit<B> =
	finality_grandpa::Commit<<B as Block>::Hash, NumberFor<B>, AuthoritySignature, AuthorityId>;

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
pub struct ClientState<H> {
	// Current authority set
	pub current_authorities: AuthorityList,
	// Id of the current authority set.
	pub current_set_id: u64,
	// latest finalized hash on the relay chain.
	pub latest_relay_hash: H,
	// para_id of associated parachain
	pub para_id: u32,
}

/// Holds relavant parachain proofs for both header and timestamp extrinsic.
pub struct ParachainHeaderProofs {
	/// State proofs that prove a parachain header exists at a given relay chain height
	pub state_proof: Vec<Vec<u8>>,
	/// Timestamp extrinsic for ibc
	pub extrinsic: Vec<u8>,
	/// Timestamp extrinsic proof for previously proven parachain header.
	pub extrinsic_proof: Vec<Vec<u8>>,
}

/// Parachain headers with a Grandpa finality proof.
pub struct ParachainHeadersWithFinalityProof<B: Block> {
	/// The grandpa finality proof: contains relay chain headers from the
	/// last known finalized grandpa block.
	pub finality_proof: FinalityProof<B::Header>,
	/// Contains a map of relay chain header hashes to parachain headers
	/// finalzed at the relay chain height. We check for this parachain header finalization
	/// via state proofs. Also contains extrinsic proof for timestamp.
	pub parachain_headers: BTreeMap<B::Hash, ParachainHeaderProofs>,
}

/// Functions that this light client needs that should delegated to
/// a native implementation.
pub trait HostFunctions {
	/// Verify an ed25519 signature
	fn ed25519_verify(sig: &ed25519::Signature, msg: &[u8], pub_key: &ed25519::Public) -> bool;

	/// see [`sp_state_machine::read_proof_check`]
	fn read_proof_check<I>(
		root: &[u8; 32],
		proof: StorageProof,
		keys: I,
	) -> Result<BTreeMap<Vec<u8>, Option<Vec<u8>>>, error::Error>
	where
		I: IntoIterator,
		I::Item: AsRef<[u8]>;

	/// parity trie_db proof verification using BlakeTwo256 Hasher
	fn verify_timestamp_extrinsic(
		root: &[u8; 32],
		proof: &[Vec<u8>],
		value: &[u8],
	) -> Result<(), error::Error>;
}

/// This returns the storage key for a parachain header on the relay chain.
pub fn parachain_header_storage_key(para_id: u32) -> StorageKey {
	let mut storage_key = frame_support::storage::storage_prefix(b"Paras", b"Heads").to_vec();
	let encoded_para_id = para_id.encode();
	storage_key.extend_from_slice(sp_io::hashing::twox_64(&encoded_para_id).as_slice());
	storage_key.extend_from_slice(&encoded_para_id);
	StorageKey(storage_key)
}
