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
use core::marker::PhantomData;
use grandpa_client::justification::{
	check_equivocation_proof, find_scheduled_change, AncestryChain,
};
use grandpa_client_primitives::ParachainHeadersWithFinalityProof;
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
use light_client_common::{verify_delay_passed, verify_membership, verify_non_membership};
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GrandpaClient<T>(PhantomData<T>);

impl<H> ClientDef for GrandpaClient<H>
where
	H: grandpa_client_primitives::HostFunctions,
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
				let client_state = grandpa_client_primitives::ClientState {
					current_authorities: client_state.current_authorities,
					current_set_id: client_state.current_set_id,
					latest_relay_hash: client_state.latest_relay_hash,
					para_id: client_state.para_id,
				};

				grandpa_client::verify_parachain_headers_with_grandpa_finality_proof::<
					RelayChainHeader,
					H,
				>(client_state, headers_with_finality_proof)
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

		for (relay_hash, parachain_header_proof) in header.parachain_headers {
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

		if let Some(max_height) = consensus_states.iter().map(|(h, ..)| h.revision_height).max() {
			// this cast is safe, see [`ConsensusState::from_header`]
			client_state.latest_para_height = max_height as u32
		}

		client_state.latest_relay_hash = header.finality_proof.block;

		let target = ancestry
			.header(&header.finality_proof.block)
			.expect("target header has already been checked in verify_client_message; qed");
		if let Some(scheduled_change) = find_scheduled_change(target) {
			client_state.current_set_id += 1;
			client_state.current_authorities = scheduled_change.next_authorities;
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
					let cs: ConsensusState =
						cs.downcast().ok_or(Ics02Error::client_args_type_mismatch(
							client_state.client_type().to_owned(),
						))?;

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
		_client_state: &Self::ClientState,
		_consensus_state: &Self::ConsensusState,
		_proof_upgrade_client: Vec<u8>,
		_proof_upgrade_consensus_state: Vec<u8>,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		// TODO:
		Err(Error::Custom("Not implemented".to_string()).into())
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
