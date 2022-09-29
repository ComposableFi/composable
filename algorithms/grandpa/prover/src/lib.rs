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

use crate::runtime::api::runtime_types::polkadot_parachain::primitives::Id;
use anyhow::anyhow;
pub use beefy_prover;
use beefy_prover::helpers::{
	fetch_timestamp_extrinsic_with_proof, unsafe_cast_to_jsonrpsee_client, TimeStampExtWithProof,
};
use codec::{Decode, Encode};
use finality_grandpa_rpc::GrandpaApiClient;
use primitives::{
	parachain_header_storage_key, FinalityProof, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::traits::{Header, Zero};
use std::collections::BTreeMap;
use subxt::{
	ext::{
		sp_core::hexdisplay::AsBytesRef,
		sp_runtime::traits::{Header as _, One},
	},
	rpc::NumberOrHex,
	Config, OnlineClient,
};

pub mod host_functions;
pub mod runtime;

pub struct GrandpaProver<T: Config> {
	pub relay_client: OnlineClient<T>,
	pub para_client: OnlineClient<T>,
	pub para_id: u32,
}

/// An encoded justification proving that the given header has been finalized
#[derive(Clone, Serialize, Deserialize)]
pub struct JustificationNotification(pub sp_core::Bytes);

impl<T> GrandpaProver<T>
where
	T: Config,
	T::BlockNumber: Ord + Zero,
	u32: From<T::BlockNumber>,
{
	/// Returns the finalized parachain headers in between the given relay chain hashes.
	pub async fn query_finalized_parachain_headers_between(
		&self,
		latest_finalized_hash: T::Hash,
		previous_finalized_hash: T::Hash,
	) -> Result<Vec<T::Header>, anyhow::Error> {
		let change_set = self
			.relay_client
			.rpc()
			.query_storage(
				// we are interested only in the blocks where our parachain header changes.
				vec![parachain_header_storage_key(self.para_id).as_bytes_ref()],
				previous_finalized_hash,
				Some(latest_finalized_hash),
			)
			.await?;

		let mut headers = vec![];
		for changes in change_set {
			let header =
				self.relay_client.rpc().header(Some(changes.block)).await?.ok_or_else(|| {
					anyhow!("[get_parachain_headers] block not found {:?}", changes.block)
				})?;

			let head = {
				let key = runtime::api::storage().paras().heads(&Id(self.para_id));
				self.relay_client
					.storage()
					.fetch(&key, Some(header.hash()))
					.await?
					.expect("Header exists in its own changeset; qed")
			};

			let para_header = T::Header::decode(&mut &head.0[..])
				.map_err(|_| anyhow!("Failed to decode header"))?;
			headers.push(para_header);
		}

		Ok(headers)
	}

	/// Returns the finality proof for the given parachain header numbers in between the given relay
	/// chain hashes.
	pub async fn query_finalized_parachain_headers_with_proof<H>(
		&self,
		latest_finalized_hash: T::Hash,
		previous_finalized_hash: T::Hash,
		header_numbers: Vec<T::BlockNumber>,
	) -> Result<Option<ParachainHeadersWithFinalityProof<H>>, anyhow::Error>
	where
		H: Header,
		H::Hash: From<T::Hash>,
		T::BlockNumber: One,
	{
		let header = self
			.relay_client
			.rpc()
			.header(Some(latest_finalized_hash))
			.await?
			.ok_or_else(|| anyhow!("Header not found!"))?;

		let client = unsafe { unsafe_cast_to_jsonrpsee_client(&self.relay_client) };
		let encoded = GrandpaApiClient::<JustificationNotification, H256, u32>::prove_finality(
			&*client,
			u32::from(*header.number()),
		)
		.await?
		.ok_or_else(|| anyhow!("No justification found for block: {:?}", header.hash()))?
		.0;
		let mut finality_proof = FinalityProof::<H>::decode(&mut &encoded[..])?;
		finality_proof.unknown_headers = {
			let mut unknown_headers = vec![H::decode(&mut &header.encode()[..])?];
			let mut current = *header.parent_hash();
			while current != previous_finalized_hash {
				let header = self
					.relay_client
					.rpc()
					.header(Some(current))
					.await?
					.ok_or_else(|| anyhow!("Header with hash: {current:?} not found!"))?;
				unknown_headers.push(H::decode(&mut &header.encode()[..])?);
				current = *header.parent_hash();
			}
			unknown_headers
		};

		// we are interested only in the blocks where our parachain header changes.
		let para_storage_key = parachain_header_storage_key(self.para_id);
		let keys = vec![para_storage_key.as_bytes_ref()];
		let previous_finalized = self
			.relay_client
			.rpc()
			.header(Some(previous_finalized_hash))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized header"))?;
		let start = self
			.relay_client
			.rpc()
			.block_hash(Some(
				NumberOrHex::Number((*previous_finalized.number() + One::one()).into()).into(),
			))
			.await?
			.ok_or_else(|| anyhow!("Failed to fetch previous finalized hash + 1"))?;
		let change_set = self
			.relay_client
			.rpc()
			.query_storage(keys.clone(), start, Some(latest_finalized_hash))
			.await?;

		let mut parachain_headers = BTreeMap::<H::Hash, ParachainHeaderProofs>::default();

		// no new parachain headers have been finalized
		if change_set.len() == 1 {
			return Ok(None)
		}

		for changes in change_set {
			let header = self
				.relay_client
				.rpc()
				.header(Some(changes.block))
				.await?
				.ok_or_else(|| anyhow!("block not found {:?}", changes.block))?;

			let parachain_header_bytes = {
				let key = runtime::api::storage().paras().heads(&Id(self.para_id));
				self.relay_client
					.storage()
					.fetch(&key, Some(header.hash()))
					.await?
					.expect("Header exists in its own changeset; qed")
					.0
			};

			let para_header: T::Header = Decode::decode(&mut &parachain_header_bytes[..])?;
			let para_block_number = *para_header.number();
			// skip genesis header or any unknown headers
			if para_block_number == Zero::zero() || !header_numbers.contains(&para_block_number) {
				continue
			}

			let state_proof = self
				.relay_client
				.rpc()
				.read_proof(keys.clone(), Some(header.hash()))
				.await?
				.proof
				.into_iter()
				.map(|p| p.0)
				.collect();

			let TimeStampExtWithProof { ext: extrinsic, proof: extrinsic_proof } =
				fetch_timestamp_extrinsic_with_proof(&self.para_client, Some(para_header.hash()))
					.await
					.map_err(|err| anyhow!("Error fetching timestamp with proof: {err:?}"))?;
			let proofs = ParachainHeaderProofs { state_proof, extrinsic, extrinsic_proof };
			parachain_headers.insert(header.hash().into(), proofs);
		}

		Ok(Some(ParachainHeadersWithFinalityProof { finality_proof, parachain_headers }))
	}
}
