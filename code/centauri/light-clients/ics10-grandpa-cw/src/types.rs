use std::ops::Deref;

use cosmwasm_schema::cw_serde;
use ibc::core::ics02_client::{
	client_consensus::ConsensusState, client_def::ClientDef, client_message::ClientMessage,
	client_state::ClientState,
};
use ics10_grandpa::{client_def::GrandpaClient, client_state::UpgradeOptions};

#[derive(Debug, Clone)]
pub struct AnyClientMessage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyClientState;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnyConsensusState;

#[derive(Clone)]
pub struct AnyClient;

impl ClientDef for AnyClient {
	type ClientMessage = AnyClientMessage;

	type ClientState = AnyClientState;

	type ConsensusState = AnyConsensusState;

	fn verify_client_message<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<(), Error> {
		todo!()
	}

	fn update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		ibc::core::ics02_client::error::Error,
	> {
		todo!()
	}

	fn update_state_on_misbehaviour(
		&self,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<Self::ClientState, ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn check_for_misbehaviour<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		client_state: Self::ClientState,
		client_msg: Self::ClientMessage,
	) -> Result<bool, ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_upgrade_and_update_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: ibc::core::ics24_host::identifier::ClientId,
		old_client_state: &Self::ClientState,
		upgrade_client_state: &Self::ClientState,
		upgrade_consensus_state: &Self::ConsensusState,
		proof_upgrade_client: Vec<u8>,
		proof_upgrade_consensus_state: Vec<u8>,
	) -> Result<
		(Self::ClientState, ibc::core::ics02_client::client_def::ConsensusUpdateResult<Ctx>),
		Error,
	> {
		todo!()
	}

	fn verify_client_consensus_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		consensus_height: ibc::Height,
		expected_consensus_state: &Ctx::AnyConsensusState,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_connection_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		connection_id: &ibc::core::ics24_host::identifier::ConnectionId,
		expected_connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_channel_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		expected_channel_end: &ibc::core::ics04_channel::channel::ChannelEnd,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_client_full_state<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_state: &Self::ClientState,
		height: ibc::Height,
		prefix: &ibc::core::ics23_commitment::commitment::CommitmentPrefix,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		expected_client_state: &Ctx::AnyClientState,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_packet_data<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
		commitment: ibc::core::ics04_channel::commitment::PacketCommitment,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_packet_acknowledgement<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
		ack: ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_next_sequence_recv<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}

	fn verify_packet_receipt_absence<Ctx: ibc::core::ics26_routing::context::ReaderContext>(
		&self,
		ctx: &Ctx,
		client_id: &ibc::core::ics24_host::identifier::ClientId,
		client_state: &Self::ClientState,
		height: ibc::Height,
		connection_end: &ibc::core::ics03_connection::connection::ConnectionEnd,
		proof: &ibc::core::ics23_commitment::commitment::CommitmentProofBytes,
		root: &ibc::core::ics23_commitment::commitment::CommitmentRoot,
		port_id: &ibc::core::ics24_host::identifier::PortId,
		channel_id: &ibc::core::ics24_host::identifier::ChannelId,
		sequence: ibc::core::ics04_channel::packet::Sequence,
	) -> Result<(), ibc::core::ics02_client::error::Error> {
		todo!()
	}
}

impl ConsensusState for AnyConsensusState {
	type Error = String;

	fn root(&self) -> &ibc::core::ics23_commitment::commitment::CommitmentRoot {
		todo!()
	}

	fn timestamp(&self) -> ibc::timestamp::Timestamp {
		todo!()
	}

	fn encode_to_vec(&self) -> Vec<u8> {
		todo!()
	}
}

impl ClientMessage for AnyClientMessage {
	fn encode_to_vec(&self) -> Vec<u8> {
		todo!()
	}
}

impl ClientState for AnyClientState {
	type UpgradeOptions = UpgradeOptions;

	type ClientDef = GrandpaClient;

	fn chain_id(&self) -> ibc::core::ics24_host::identifier::ChainId {
		todo!()
	}

	fn client_def(&self) -> Self::ClientDef {
		todo!()
	}

	fn client_type(&self) -> ibc::core::ics02_client::client_state::ClientType {
		todo!()
	}

	fn latest_height(&self) -> ibc::Height {
		todo!()
	}

	fn frozen_height(&self) -> Option<ibc::Height> {
		todo!()
	}

	fn upgrade(
		self,
		upgrade_height: ibc::Height,
		upgrade_options: Self::UpgradeOptions,
		chain_id: ibc::core::ics24_host::identifier::ChainId,
	) -> Self {
		todo!()
	}

	fn expired(&self, elapsed: std::time::Duration) -> bool {
		todo!()
	}

	fn encode_to_vec(&self) -> Vec<u8> {
		todo!()
	}
}

#[cw_serde]
pub struct Height {
	/// Previously known as "epoch"
	pub revision_number: u64,

	/// The height of a block
	pub revision_height: u64,
}

impl Deref for Height {
	type Target = ibc::Height;
	fn deref(&self) -> &Self::Target {
		&ibc::Height {
			revision_number: self.revision_number,
			revision_height: self.revision_height,
		}
	}
}
