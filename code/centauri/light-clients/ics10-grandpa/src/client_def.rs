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

use crate::{client_state::ClientState, consensus_state::ConsensusState, error::Error};
use ibc::core::ics02_client::{
	client_consensus::ConsensusState as _, client_state::ClientState as _,
};

use crate::client_message::{ClientMessage, RelayChainHeader};
use alloc::{format, string::ToString, vec, vec::Vec};
use codec::Decode;
use core::marker::PhantomData;
use finality_grandpa::Chain;
use grandpa_client_primitives::{
	justification::{check_equivocation_proof, find_scheduled_change, AncestryChain},
	ParachainHeadersWithFinalityProof,
};
use ibc::{
	core::{
		ics02_client::{
			client_def::{ClientDef, ConsensusUpdateResult},
			error::Error as Ics02Error,
		},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::ChannelEnd,
			commitment::{AcknowledgementCommitment, PacketCommitment},
			packet::Sequence,
		},
		ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		ics24_host::{
			identifier::{ChannelId, ClientId, ConnectionId, PortId},
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, ReceiptsPath, SeqRecvsPath,
			},
		},
		ics26_routing::context::ReaderContext,
	},
	Height,
};
use light_client_common::{
	state_machine, verify_delay_passed, verify_membership, verify_non_membership,
};
use primitive_types::H256;
use sp_runtime::traits::Header as _;
use sp_trie::StorageProof;
use tendermint_proto::Protobuf;

const CLIENT_STATE_UPGRADE_PATH: &[u8] = b"client-state-upgrade-path";
const CONSENSUS_STATE_UPGRADE_PATH: &[u8] = b"consensus-state-upgrade-path";

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GrandpaClient<T>(PhantomData<T>);

impl<H> ClientDef for GrandpaClient<H>
where
	H: grandpa_client_primitives::HostFunctions<Header = RelayChainHeader>,
{
	type ClientMessage = ClientMessage;
	type ClientState = ClientState<H>;
	type ConsensusState = ConsensusState;

	fn verify_client_message<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(), Ics02Error> {
		match client_message {
			ClientMessage::Header(header) => {
				let headers_with_finality_proof = ParachainHeadersWithFinalityProof {
					finality_proof: header.finality_proof,
					parachain_headers: header.parachain_headers,
				};

				grandpa_client::verify_parachain_headers_with_grandpa_finality_proof::<
					RelayChainHeader,
					H,
				>(client_state.into(), headers_with_finality_proof)
				.map_err(Error::GrandpaPrimitives)?;
			},
			ClientMessage::Misbehaviour(misbehavior) => {
				// first off is the number of equivocations >= 1/3?
				if misbehavior.equivocations.len() < (client_state.current_authorities.len() / 3) {
					Err(Error::Custom(
						"Not enough equivocations to warrant a misbehavior".to_string(),
					))?
				}

				misbehavior
					.equivocations
					.into_iter()
					.map(|equivocation| {
						check_equivocation_proof::<H, _, _>(
							client_state.current_set_id,
							equivocation,
						)
					})
					.collect::<Result<(), _>>()
					.map_err(Error::Anyhow)?;

				// whoops equivocation is valid.
			},
		}

		Ok(())
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		mut client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		let header = match client_message {
			ClientMessage::Header(header) => header,
			_ => unreachable!(
				"02-client will check for misbehaviour before calling update_state; qed"
			),
		};
		let ancestry =
			AncestryChain::<RelayChainHeader>::new(&header.finality_proof.unknown_headers);
		let mut consensus_states = vec![];

		let from = client_state.latest_relay_hash;

		let mut finalized = ancestry
			.ancestry(from, header.finality_proof.block)
			.map_err(|_| Error::Custom(format!("Invalid ancestry!")))?;
		finalized.sort();

		for (relay_hash, parachain_header_proof) in header.parachain_headers {
			// we really shouldn't set consensus states for parachain headers not in the finalized
			// chain.
			if finalized.binary_search(&relay_hash).is_err() {
				continue
			}

			let header = ancestry.header(&relay_hash).ok_or_else(|| {
				Error::Custom(format!("No relay chain header found for hash: {relay_hash:?}"))
			})?;
			let (height, consensus_state) = ConsensusState::from_header::<H>(
				parachain_header_proof,
				client_state.para_id,
				header.state_root.clone(),
			)?;
			let wrapped = Ctx::AnyConsensusState::wrap(&consensus_state)
				.expect("AnyConsenusState is type checked; qed");
			consensus_states.push((height, wrapped));
		}

		// updates
		let target = ancestry
			.header(&header.finality_proof.block)
			.expect("target header has already been checked in verify_client_message; qed");

		// can't try to rewind relay chain
		if target.number <= client_state.latest_relay_height {
			Err(Ics02Error::implementation_specific(format!(
				"Light client can only be updated to new relay chain height."
			)))?
		}

		let mut heights = consensus_states
			.iter()
			.map(|(h, ..)| {
				// this cast is safe, see [`ConsensusState::from_header`]
				h.revision_height as u32
			})
			.collect::<Vec<_>>();

		heights.sort();

		if let Some((min_height, max_height)) = heights.first().zip(heights.last()) {
			// can't try to rewind parachain.
			if *min_height <= client_state.latest_para_height {
				Err(Ics02Error::implementation_specific(format!(
					"Light client can only be updated to new parachain height."
				)))?
			}
			client_state.latest_para_height = *max_height
		}

		client_state.latest_relay_hash = header.finality_proof.block;
		client_state.latest_relay_height = target.number;

		if let Some(scheduled_change) = find_scheduled_change(target) {
			client_state.current_set_id += 1;
			client_state.current_authorities = scheduled_change.next_authorities;
		}

		for header in &finalized {
			let header = ancestry.header(header).expect("finalized headers are in ancestry; qed");
			H::add_relaychain_headers(header);
		}

		Ok((client_state, ConsensusUpdateResult::Batch(consensus_states)))
	}

	fn update_state_on_misbehaviour(
		&self,
		mut client_state: Self::ClientState,
		_client_message: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02Error> {
		client_state.frozen_height =
			Some(Height::new(client_state.para_id as u64, client_state.latest_para_height as u64));
		Ok(client_state)
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<bool, Ics02Error> {
		if matches!(client_message, ClientMessage::Misbehaviour(_)) {
			return Ok(true)
		}

		// we also check that this update doesn't include competing consensus states for heights we
		// already processed.
		let header = match client_message {
			ClientMessage::Header(header) => header,
			_ => unreachable!("We've checked for misbehavior in line 180; qed"),
		};
		let ancestry =
			AncestryChain::<RelayChainHeader>::new(&header.finality_proof.unknown_headers);

		for (relay_hash, parachain_header_proof) in header.parachain_headers {
			let header = ancestry.header(&relay_hash).ok_or_else(|| {
				Error::Custom(format!("No relay chain header found for hash: {relay_hash:?}"))
			})?;

			let (height, consensus_state) = ConsensusState::from_header::<H>(
				parachain_header_proof,
				client_state.para_id,
				header.state_root.clone(),
			)?;

			match ctx.maybe_consensus_state(&client_id, height)? {
				Some(cs) => {
					let cs: ConsensusState = cs
						.downcast()
						.ok_or(Ics02Error::client_args_type_mismatch(client_state.client_type()))?;

					if cs != consensus_state {
						// Houston we have a problem
						return Ok(true)
					}
				},
				None => {},
			};
		}

		Ok(false)
	}

	fn verify_upgrade_and_update_state<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		old_client_state: &Self::ClientState,
		upgrade_client_state: &Self::ClientState,
		upgrade_consensus_state: &Self::ConsensusState,
		proof_upgrade_client: Vec<u8>,
		proof_upgrade_consensus_state: Vec<u8>,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		let height = Height::new(
			old_client_state.para_id as u64,
			old_client_state.latest_para_height as u64,
		);

		let consenus_state = ctx.consensus_state(&client_id, height)?
			.downcast::<Self::ConsensusState>()
			.ok_or_else(|| Error::Custom(format!("Wrong consensus state type stored for Grandpa client with {client_id} at {height}")))?;

		let root = H256::from_slice(consenus_state.root.as_bytes());

		// verify client state upgrade proof
		{
			let proof_upgrade_client = {
				let nodes: Vec<Vec<u8>> =
					Decode::decode(&mut &proof_upgrade_client[..]).map_err(Error::Codec)?;
				StorageProof::new(nodes)
			};

			let encoded = Ctx::AnyClientState::wrap(&upgrade_client_state.clone())
				.expect("AnyConsensusState is type-checked; qed")
				.encode_to_vec();

			let value = state_machine::read_proof_check::<H::BlakeTwo256, _>(
				&root,
				proof_upgrade_client,
				vec![CLIENT_STATE_UPGRADE_PATH],
			)
			.map_err(|err| Error::Custom(format!("{err}")))?
			.remove(CLIENT_STATE_UPGRADE_PATH)
			.flatten()
			.ok_or_else(|| Error::Custom(format!("Invalid proof for client state upgrade")))?;

			if value != encoded {
				Err(Error::Custom(format!("Invalid proof for client state upgrade")))?
			}
		}

		// verify consensus state upgrade proof
		{
			let proof_upgrade_consensus_state = {
				let nodes: Vec<Vec<u8>> = Decode::decode(&mut &proof_upgrade_consensus_state[..])
					.map_err(Error::Codec)?;
				StorageProof::new(nodes)
			};

			let encoded = Ctx::AnyConsensusState::wrap(upgrade_client_state)
				.expect("AnyConsensusState is type-checked; qed")
				.encode_to_vec();

			let value = state_machine::read_proof_check::<H::BlakeTwo256, _>(
				&root,
				proof_upgrade_consensus_state,
				vec![CONSENSUS_STATE_UPGRADE_PATH],
			)
			.map_err(|err| Error::Custom(format!("{err}")))?
			.remove(CONSENSUS_STATE_UPGRADE_PATH)
			.flatten()
			.ok_or_else(|| Error::Custom(format!("Invalid proof for client state upgrade")))?;

			if value != encoded {
				Err(Error::Custom(format!("Invalid proof for client state upgrade")))?
			}
		}

		Ok((
			upgrade_client_state.clone(),
			ConsensusUpdateResult::Single(
				Ctx::AnyConsensusState::wrap(upgrade_consensus_state)
					.expect("AnyConsensusState is type-checked; qed"),
			),
		))
	}

	fn verify_client_consensus_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		client_id: &ClientId,
		consensus_height: Height,
		expected_consensus_state: &Ctx::AnyConsensusState,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		let path = ClientConsensusStatePath {
			client_id: client_id.clone(),
			epoch: consensus_height.revision_number,
			height: consensus_height.revision_height,
		};
		let value = expected_consensus_state.encode_to_vec();
		verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
			.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_connection_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		connection_id: &ConnectionId,
		expected_connection_end: &ConnectionEnd,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		let path = ConnectionsPath(connection_id.clone());
		let value = expected_connection_end.encode_vec();
		verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
			.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_channel_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		expected_channel_end: &ChannelEnd,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		let path = ChannelEndsPath(port_id.clone(), *channel_id);
		let value = expected_channel_end.encode_vec();
		verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
			.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_client_full_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		client_state: &Self::ClientState,
		height: Height,
		prefix: &CommitmentPrefix,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		client_id: &ClientId,
		expected_client_state: &Ctx::AnyClientState,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		let path = ClientStatePath(client_id.clone());
		let value = expected_client_state.encode_to_vec();
		verify_membership::<H::BlakeTwo256, _>(prefix, proof, root, path, value)
			.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_packet_data<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
		commitment: PacketCommitment,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;

		let commitment_path =
			CommitmentsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };

		verify_membership::<H::BlakeTwo256, _>(
			connection_end.counterparty().prefix(),
			proof,
			root,
			commitment_path,
			commitment.into_vec(),
		)
		.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_packet_acknowledgement<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
		ack: AcknowledgementCommitment,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;

		let ack_path = AcksPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		verify_membership::<H::BlakeTwo256, _>(
			connection_end.counterparty().prefix(),
			proof,
			root,
			ack_path,
			ack.into_vec(),
		)
		.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_next_sequence_recv<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;

		let seq_bytes = codec::Encode::encode(&u64::from(sequence));

		let seq_path = SeqRecvsPath(port_id.clone(), *channel_id);
		verify_membership::<H::BlakeTwo256, _>(
			connection_end.counterparty().prefix(),
			proof,
			root,
			seq_path,
			seq_bytes,
		)
		.map_err(Error::Anyhow)?;
		Ok(())
	}

	fn verify_packet_receipt_absence<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		_client_id: &ClientId,
		client_state: &Self::ClientState,
		height: Height,
		connection_end: &ConnectionEnd,
		proof: &CommitmentProofBytes,
		root: &CommitmentRoot,
		port_id: &PortId,
		channel_id: &ChannelId,
		sequence: Sequence,
	) -> Result<(), Ics02Error> {
		client_state.verify_height(height)?;
		verify_delay_passed::<H, _>(ctx, height, connection_end).map_err(Error::Anyhow)?;

		let receipt_path =
			ReceiptsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		verify_non_membership::<H::BlakeTwo256, _>(
			connection_end.counterparty().prefix(),
			proof,
			root,
			receipt_path,
		)
		.map_err(Error::Anyhow)?;
		Ok(())
	}
}
