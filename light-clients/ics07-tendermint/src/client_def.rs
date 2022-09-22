use core::{convert::TryInto, fmt::Debug, marker::PhantomData};

use ibc::core::{
	ics02_client::{
		client_consensus::ConsensusState as _,
		client_def::{ClientDef, ConsensusUpdateResult},
		client_state::ClientState as _,
		error::Error as Ics02Error,
	},
	ics03_connection::connection::ConnectionEnd,
	ics04_channel::{
		channel::ChannelEnd,
		commitment::{AcknowledgementCommitment, PacketCommitment},
		packet::Sequence,
	},
	ics23_commitment::{
		commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot},
		merkle::{apply_prefix, MerkleProof},
	},
	ics24_host::{
		identifier::{ChannelId, ClientId, ConnectionId, PortId},
		path::{
			AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, CommitmentsPath,
			ConnectionsPath, ReceiptsPath, SeqRecvsPath,
		},
		Path,
	},
	ics26_routing::context::ReaderContext,
};
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use prost::Message;
use tendermint_light_client_verifier::{
	types::{TrustedBlockState, UntrustedBlockState},
	ProdVerifier, Verdict, Verifier,
};
use tendermint_proto::Protobuf;

use crate::{
	client_message::{ClientMessage, Header},
	client_state::ClientState,
	consensus_state::ConsensusState,
	error::Error,
	HostFunctionsProvider,
};
use ibc::{prelude::*, timestamp::Timestamp, Height};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TendermintClient<H>(PhantomData<H>);

impl<H> ClientDef for TendermintClient<H>
where
	H: HostFunctionsProvider,
{
	type ClientMessage = ClientMessage;
	type ClientState = ClientState<H>;
	type ConsensusState = ConsensusState;

	fn verify_client_message<Ctx>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		client_state: Self::ClientState,
		message: Self::ClientMessage,
	) -> Result<(), Ics02Error>
	where
		Ctx: ReaderContext,
	{
		match message {
			ClientMessage::Header(header) => {
				if header.height().revision_number != client_state.chain_id.version() {
					return Err(Ics02Error::client_error(
						client_state.client_type().to_owned(),
						Error::mismatched_revisions(
							client_state.chain_id.version(),
							header.height().revision_number,
						)
						.to_string(),
					))
				}

				// Check if a consensus state is already installed; if so skip
				let header_consensus_state = <ConsensusState as From<Header>>::from(header.clone());

				let _ = match ctx.maybe_consensus_state(&client_id, header.height())? {
					Some(cs) => {
						let cs: ConsensusState =
							cs.downcast().ok_or(Ics02Error::client_args_type_mismatch(
								client_state.client_type().to_owned(),
							))?;
						// If this consensus state matches, skip verification
						// (optimization)
						if cs == header_consensus_state {
							// Header is already installed and matches the incoming
							// header (already verified)
							return Ok(())
						}
						Some(cs)
					},
					None => None,
				};

				let trusted_consensus_state: Self::ConsensusState = ctx
					.consensus_state(&client_id, header.trusted_height)?
					.downcast()
					.ok_or(Ics02Error::client_args_type_mismatch(
						ClientState::<H>::client_type().to_owned(),
					))?;

				let trusted_state = TrustedBlockState {
					header_time: trusted_consensus_state.timestamp().into_tm_time().unwrap(),
					height: header.trusted_height.revision_height.try_into().map_err(|_| {
						Ics02Error::client_error(
							client_state.client_type().to_owned(),
							Error::invalid_header_height(header.trusted_height).to_string(),
						)
					})?,
					next_validators: &header.trusted_validator_set,
					next_validators_hash: trusted_consensus_state.next_validators_hash,
				};

				let untrusted_state = UntrustedBlockState {
					signed_header: &header.signed_header,
					validators: &header.validator_set,
					// NB: This will skip the
					// VerificationPredicates::next_validators_match check for the
					// untrusted state.
					next_validators: None,
				};

				let options = client_state.as_light_client_options()?;

				let verifier = ProdVerifier::<H>::default();
				let verdict = verifier.verify(
					untrusted_state,
					trusted_state,
					&options,
					ctx.host_timestamp().into_tm_time().unwrap(),
				);

				match verdict {
					Verdict::Success => {},
					Verdict::NotEnoughTrust(voting_power_tally) =>
						return Err(Error::not_enough_trusted_vals_signed(format!(
							"voting power tally: {}",
							voting_power_tally
						))
						.into()),
					Verdict::Invalid(detail) =>
						return Err(Error::verification_error(detail).into()),
				}
			},
			ClientMessage::Misbehaviour(misbehaviour) => {
				self.verify_client_message(
					ctx,
					client_id.clone(),
					client_state.clone(),
					ClientMessage::Header(misbehaviour.header1),
				)?;
				self.verify_client_message(
					ctx,
					client_id,
					client_state,
					ClientMessage::Header(misbehaviour.header2),
				)?;
			},
		};

		Ok(())
	}

	fn update_state<Ctx: ReaderContext>(
		&self,
		_ctx: &Ctx,
		_client_id: ClientId,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<(Self::ClientState, ConsensusUpdateResult<Ctx>), Ics02Error> {
		let header = match client_message {
			ClientMessage::Header(header) => header,
			_ => unreachable!("02-client will check for Header before calling update_state; qed"),
		};
		let header_consensus_state = <ConsensusState as From<Header>>::from(header.clone());
		let cs = Ctx::AnyConsensusState::wrap(&header_consensus_state).ok_or_else(|| {
			Ics02Error::unknown_consensus_state_type("Ctx::AnyConsensusState".to_string())
		})?;
		Ok((client_state.with_header(header), ConsensusUpdateResult::Single(cs)))
	}

	fn update_state_on_misbehaviour(
		&self,
		client_state: Self::ClientState,
		client_message: Self::ClientMessage,
	) -> Result<Self::ClientState, Ics02Error> {
		let misbehaviour = match client_message {
			ClientMessage::Misbehaviour(misbehaviour) => misbehaviour,
			_ => unreachable!(
				"02-client will check for misbehaviour before calling update_state_on_misbehaviour; qed"
			),
		};
		client_state
			.with_frozen_height(misbehaviour.header1.height())
			.map_err(|e| e.into())
	}

	fn check_for_misbehaviour<Ctx: ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ClientId,
		client_state: Self::ClientState,
		message: Self::ClientMessage,
	) -> Result<bool, Ics02Error> {
		match message {
			ClientMessage::Header(header) => {
				// Check if a consensus state is already installed; if so it should
				// match the untrusted header.
				let header_consensus_state = <ConsensusState as From<Header>>::from(header.clone());

				let existing_consensus_state =
					match ctx.maybe_consensus_state(&client_id, header.height())? {
						Some(cs) => {
							let cs = cs.downcast::<ConsensusState>().ok_or(
								Ics02Error::client_args_type_mismatch(
									ClientState::<()>::client_type().to_owned(),
								),
							)?;
							// If this consensus state matches, skip verification
							// (optimization)
							if header_consensus_state == cs {
								// Header is already installed and matches the incoming
								// header (already verified)
								return Ok(false)
							}
							Some(cs)
						},
						None => None,
					};

				// If the header has verified, but its corresponding consensus state
				// differs from the existing consensus state for that height, freeze the
				// client and return the installed consensus state.
				if let Some(cs) = existing_consensus_state {
					if cs != header_consensus_state {
						return Ok(true)
					}
				}

				// Monotonicity checks for timestamps for in-the-middle updates
				// (cs-new, cs-next, cs-latest)
				if header.height() < client_state.latest_height() {
					let maybe_next_cs = ctx
						.next_consensus_state(&client_id, header.height())?
						.map(|cs| {
							cs.downcast::<ConsensusState>().ok_or(
								Ics02Error::client_args_type_mismatch(
									ClientState::<H>::client_type().to_owned(),
								),
							)
						})
						.transpose()?;

					if let Some(next_cs) = maybe_next_cs {
						// New (untrusted) header timestamp cannot occur after next
						// consensus state's height
						if Timestamp::from(header.signed_header.header().time).nanoseconds() >
							next_cs.timestamp().nanoseconds()
						{
							return Err(Error::header_timestamp_too_high(
								header.signed_header.header().time.to_string(),
								next_cs.timestamp().to_string(),
							)
							.into())
						}
					}
				}
				// (cs-trusted, cs-prev, cs-new)
				if header.trusted_height < header.height() {
					let maybe_prev_cs = ctx
						.prev_consensus_state(&client_id, header.height())?
						.map(|cs| {
							cs.downcast::<ConsensusState>().ok_or(
								Ics02Error::client_args_type_mismatch(
									ClientState::<()>::client_type().to_owned(),
								),
							)
						})
						.transpose()?;

					if let Some(prev_cs) = maybe_prev_cs {
						// New (untrusted) header timestamp cannot occur before the
						// previous consensus state's height
						if header.signed_header.header().time < prev_cs.timestamp {
							return Err(Error::header_timestamp_too_low(
								header.signed_header.header().time.to_string(),
								prev_cs.timestamp.to_string(),
							)
							.into())
						}
					}
				}
			},
			ClientMessage::Misbehaviour(_misbehaviour) => return Ok(true),
		};

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
		Err(Ics02Error::implementation_specific("Not implemented".to_string()))
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
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		verify_membership::<H, _>(client_state, prefix, proof, root, path, value)
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
		verify_delay_passed(ctx, height, connection_end)?;

		let commitment_path =
			CommitmentsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };

		verify_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			commitment_path,
			commitment.into_vec(),
		)
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
		ack_commitment: AcknowledgementCommitment,
	) -> Result<(), Ics02Error> {
		// client state height = consensus state height
		client_state.verify_height(height)?;
		verify_delay_passed(ctx, height, connection_end)?;

		let ack_path = AcksPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		verify_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			ack_path,
			ack_commitment.into_vec(),
		)
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
		verify_delay_passed(ctx, height, connection_end)?;

		let mut seq_bytes = Vec::new();
		u64::from(sequence).encode(&mut seq_bytes).expect("buffer size too small");

		let seq_path = SeqRecvsPath(port_id.clone(), *channel_id);
		verify_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			seq_path,
			seq_bytes,
		)
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
		verify_delay_passed(ctx, height, connection_end)?;

		let receipt_path =
			ReceiptsPath { port_id: port_id.clone(), channel_id: *channel_id, sequence };
		verify_non_membership::<H, _>(
			client_state,
			connection_end.counterparty().prefix(),
			proof,
			root,
			receipt_path,
		)
	}
}

fn verify_membership<H, P>(
	client_state: &ClientState<H>,
	prefix: &CommitmentPrefix,
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	path: P,
	value: Vec<u8>,
) -> Result<(), Ics02Error>
where
	P: Into<Path>,
	H: ics23::HostFunctionsProvider,
{
	let merkle_path = apply_prefix(prefix, vec![path.into().to_string()]);
	let merkle_proof: MerkleProof<H> = RawMerkleProof::try_from(proof.clone())
		.map_err(Ics02Error::invalid_commitment_proof)?
		.into();

	merkle_proof
		.verify_membership(&client_state.proof_specs, root.clone().into(), merkle_path, value, 0)
		.map_err(|e| Error::ics23_error(e).into())
}

fn verify_non_membership<H, P>(
	client_state: &ClientState<H>,
	prefix: &CommitmentPrefix,
	proof: &CommitmentProofBytes,
	root: &CommitmentRoot,
	path: P,
) -> Result<(), Ics02Error>
where
	P: Into<Path>,
	H: ics23::HostFunctionsProvider,
{
	let merkle_path = apply_prefix(prefix, vec![path.into().to_string()]);
	let merkle_proof: MerkleProof<H> = RawMerkleProof::try_from(proof.clone())
		.map_err(Ics02Error::invalid_commitment_proof)?
		.into();

	merkle_proof
		.verify_non_membership(&client_state.proof_specs, root.clone().into(), merkle_path)
		.map_err(|e| Error::ics23_error(e).into())
}

fn verify_delay_passed<Ctx: ReaderContext>(
	ctx: &Ctx,
	height: Height,
	connection_end: &ConnectionEnd,
) -> Result<(), Ics02Error> {
	let current_timestamp = ctx.host_timestamp();
	let current_height = ctx.host_height();

	let client_id = connection_end.client_id();
	let processed_time = ctx
		.client_update_time(client_id, height)
		.map_err(|_| Error::processed_time_not_found(client_id.clone(), height))?;
	let processed_height = ctx
		.client_update_height(client_id, height)
		.map_err(|_| Error::processed_height_not_found(client_id.clone(), height))?;

	let delay_period_time = connection_end.delay_period();
	let delay_period_height = ctx.block_delay(delay_period_time);

	ClientState::<()>::verify_delay_passed(
		current_timestamp,
		current_height,
		processed_time,
		processed_height,
		delay_period_time,
		delay_period_height,
	)
	.map_err(|e| e.into())
}
