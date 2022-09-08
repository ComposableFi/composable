use crate::connection_delay::has_delay_elapsed;
use ibc::{
	core::{
		ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState},
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::{ChannelEnd, Order, State},
			context::calculate_block_delay,
			msgs::{
				acknowledgement::MsgAcknowledgement, recv_packet::MsgRecvPacket,
				timeout::MsgTimeout, timeout_on_close::MsgTimeoutOnClose,
			},
			packet::Packet,
		},
		ics23_commitment::commitment::CommitmentProofBytes,
		ics24_host::{
			identifier::{ChannelId, PortId},
			path::{AcksPath, CommitmentsPath, ReceiptsPath, SeqRecvsPath},
		},
	},
	proofs::Proofs,
	timestamp::{Expiry::Expired, Timestamp},
	tx_msg::Msg,
	Height,
};
use ibc_proto::google::protobuf::Any;
use primitives::{
	apply_prefix, error::Error, query_undelivered_acks, query_undelivered_sequences, Chain,
};
use std::str::FromStr;
use tendermint_proto::Protobuf;

/// Returns a tuple of messages, with the first item being packets that are ready to be sent to the
/// sink chain. And the second item being packet timeouts that should be sent to the source.
pub async fn query_ready_and_timed_out_packets(
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<(Vec<Any>, Vec<Any>), anyhow::Error> {
	let mut messages = vec![];
	let mut timeout_messages = vec![];
	let (source_height, source_timestamp) = source.latest_height_and_timestamp().await?;
	// what is the sink's latest height/timestamp ?
	let (sink_height, sink_timestamp) = sink.latest_height_and_timestamp().await?;
	let channel_whitelist = source.channel_whitelist().await?;

	for (channel_id, port_id) in channel_whitelist {
		let source_channel_response =
			source.query_channel_end(source_height, channel_id, port_id.clone()).await?;
		let source_channel_end =
			ChannelEnd::try_from(source_channel_response.channel.ok_or_else(|| {
				Error::Custom(format!(
					"ChannelEnd not found for {:?}/{:?}",
					channel_id,
					port_id.clone()
				))
			})?)?;

		if !matches!(source_channel_end.state, State::Open | State::Closed) {
			continue
		}
		let connection_id = source_channel_end
			.connection_hops
			.get(0)
			.ok_or_else(|| Error::Custom("Channel end missing connection id".to_string()))?
			.clone();
		let connection_response =
			source.query_connection_end(source_height, connection_id.clone()).await?;
		let source_connection_end =
			ConnectionEnd::try_from(connection_response.connection.ok_or_else(|| {
				Error::Custom(format!(
					"[get_timeout_messages] ConnectionEnd not found for {:?}",
					connection_id
				))
			})?)?;

		let sink_channel_id = source_channel_end
			.counterparty()
			.channel_id
			.expect("An open channel must have a counterparty set")
			.clone();
		let sink_port_id = source_channel_end.counterparty().port_id.clone();
		let sink_channel_response = sink
			.query_channel_end(sink_height, sink_channel_id, sink_port_id.clone())
			.await?;

		let sink_channel_end =
			ChannelEnd::try_from(sink_channel_response.channel.ok_or_else(|| {
				Error::Custom(format!(
					"Failed to convert to concrete channel end from raw channel end",
				))
			})?)?;

		let next_sequence_recv = sink
			.query_next_sequence_recv(sink_height, &sink_port_id, &sink_channel_id)
			.await?;
		// query packets that are waiting for connection delay.
		let seqs = query_undelivered_sequences(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			source,
			sink,
		)
		.await?;

		// query acknowledgements that are waiting for connection delay.
		let acks = query_undelivered_acks(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			source,
			sink,
		)
		.await?;

		let sink_client_state_on_source =
			source.query_client_state(source_height, sink.client_id()).await?;
		let sink_client_state_on_source = AnyClientState::try_from(
			sink_client_state_on_source.client_state.expect(
				format!("Client state for {} should exist on {}", sink.name(), source.name())
					.as_str(),
			),
		)
		.expect("Client state conversion should not fail");

		let latest_sink_height_on_source = sink_client_state_on_source.latest_height();
		let sink_consensus_state_on_source = source
			.query_client_consensus(source_height, sink.client_id(), latest_sink_height_on_source)
			.await?;
		let sink_consensus_state_on_source = AnyConsensusState::try_from(
			sink_consensus_state_on_source
				.consensus_state
				.expect("Consensus state should exist if client state exists for that height"),
		)
		.expect("Consensus state conversion should,not fail");

		let (sink_client_update_height, sink_client_update_time) = source
			.query_client_update_time_and_height(sink.client_id(), latest_sink_height_on_source)
			.await?;

		let source_client_state_on_sink =
			sink.query_client_state(sink_height, source.client_id()).await?;
		let source_client_state_on_sink = AnyClientState::try_from(
			source_client_state_on_sink.client_state.expect(
				format!("Client state for {} should exist on {}", source.name(), sink.name())
					.as_str(),
			),
		)
		.expect("Client state conversion should not fail");
		let latest_source_height_on_sink = source_client_state_on_sink.latest_height();

		let packet_infos = source.query_send_packets(channel_id, port_id.clone(), seqs).await?;
		for packet_info in packet_infos {
			let packet = Packet {
				sequence: packet_info.sequence.into(),
				source_port: PortId::from_str(&packet_info.source_port)
					.expect("Port is should be valid"),
				source_channel: ChannelId::from_str(&packet_info.source_channel)
					.expect("Channel is should be valid"),
				destination_port: PortId::from_str(&packet_info.destination_port)
					.expect("Port is should be valid"),
				destination_channel: ChannelId::from_str(&packet_info.destination_channel)
					.expect("Channel is should be valid"),
				data: packet_info.data,
				timeout_height: packet_info.timeout_height.into(),
				timeout_timestamp: Timestamp::from_nanoseconds(packet_info.timeout_timestamp)
					.expect("Timestamp should be valid"),
			};

			// If sink channel end is closed all packets should be sent to source as
			// MsgTimeoutOnClose
			if sink_channel_end.state == State::Closed {
				// Delay period is the same for both connection ends on both chains
				let connection_delay = source_connection_end.delay_period();
				let block_delay =
					calculate_block_delay(connection_delay, source.expected_block_time());
				if !has_delay_elapsed(
					source_timestamp,
					source_height,
					sink_client_update_time,
					sink_client_update_height,
					connection_delay,
					block_delay,
				)? {
					continue
				}
				let proof_closed =
					CommitmentProofBytes::try_from(sink_channel_response.proof.clone())?;
				let key = if sink_channel_end.ordering == Order::Ordered {
					let path = format!(
						"{}",
						SeqRecvsPath(packet.destination_port.clone(), packet.destination_channel)
					);
					apply_prefix(sink.connection_prefix().into_vec(), path)
				} else {
					let path = format!(
						"{}",
						ReceiptsPath {
							port_id: packet.destination_port.clone(),
							channel_id: packet.destination_channel,
							sequence: packet.sequence
						}
					);
					apply_prefix(sink.connection_prefix().into_vec(), path)
				};

				let proof_unreceived =
					sink.query_proof(latest_sink_height_on_source, vec![key]).await?;
				let proof_unreceived = CommitmentProofBytes::try_from(proof_unreceived)?;
				let msg = MsgTimeoutOnClose {
					packet: packet.clone(),
					next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
					proofs: Proofs::new(
						proof_unreceived,
						None,
						None,
						Some(proof_closed),
						latest_sink_height_on_source,
					)?,

					signer: source.account_id(),
				};
				let value = msg.encode_vec();
				let msg = Any { value, type_url: msg.type_url() };
				timeout_messages.push(msg);
				continue
			}

			// Check if packet has timed out
			if packet.timed_out(&sink_timestamp, sink_height) {
				let connection_delay = source_connection_end.delay_period();
				let block_delay =
					calculate_block_delay(connection_delay, source.expected_block_time());
				if !has_delay_elapsed(
					source_timestamp,
					source_height,
					sink_client_update_time,
					sink_client_update_height,
					connection_delay,
					block_delay,
				)? {
					continue
				}
				let timeout_variant =
					timeout_variant(&packet, &sink_timestamp, sink_height).unwrap();

				match timeout_variant {
					TimeoutVariant::Height => {
						if latest_sink_height_on_source < packet.timeout_height {
							continue
						}
					},
					TimeoutVariant::Timestamp => {
						if sink_consensus_state_on_source.timestamp().nanoseconds() <
							packet.timeout_timestamp.nanoseconds()
						{
							continue
						}
					},
					TimeoutVariant::Both => {
						if latest_sink_height_on_source < packet.timeout_height ||
							sink_consensus_state_on_source.timestamp().nanoseconds() <
								packet.timeout_timestamp.nanoseconds()
						{
							continue
						}
					},
				}

				let key = if sink_channel_end.ordering == Order::Ordered {
					let path = format!(
						"{}",
						SeqRecvsPath(packet.destination_port.clone(), packet.destination_channel)
					);
					apply_prefix(sink.connection_prefix().into_vec(), path)
				} else {
					let path = format!(
						"{}",
						ReceiptsPath {
							port_id: packet.destination_port.clone(),
							channel_id: packet.destination_channel,
							sequence: packet.sequence
						}
					);
					apply_prefix(sink.connection_prefix().into_vec(), path)
				};
				let proof_unreceived =
					sink.query_proof(latest_sink_height_on_source, vec![key]).await?;
				let proof_unreceived = CommitmentProofBytes::try_from(proof_unreceived)?;
				let msg = MsgTimeout {
					packet: packet.clone(),
					next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
					proofs: Proofs::new(
						proof_unreceived,
						None,
						None,
						None,
						latest_sink_height_on_source,
					)?,

					signer: source.account_id(),
				};
				let value = msg.encode_vec();
				let msg = Any { value, type_url: msg.type_url() };
				timeout_messages.push(msg);
				continue
			}

			// Check if packet is ready to be sent to sink
			// If sink does not have a client height that is equal to or greater than the packet
			// creation height, we can't send it yet, packet_info.height should represent the packet
			// creation height on source chain
			if packet_info.height > latest_source_height_on_sink.revision_height {
				// Sink does not have client update required to prove recv packet message
				continue
			}

			let proof_height =
				Height::new(latest_source_height_on_sink.revision_number, packet_info.height);

			let (source_client_update_height, source_client_update_time) = sink
				.query_client_update_time_and_height(source.client_id(), proof_height)
				.await?;

			// Verify delay has passed
			let connection_delay = source_connection_end.delay_period();
			let block_delay = calculate_block_delay(connection_delay, sink.expected_block_time());
			if !has_delay_elapsed(
				sink_timestamp,
				sink_height,
				source_client_update_time,
				source_client_update_height,
				connection_delay,
				block_delay,
			)? {
				continue
			}

			let path = format!(
				"{}",
				CommitmentsPath {
					port_id: packet.source_port.clone(),
					channel_id: packet.source_channel,
					sequence: packet.sequence
				}
			);

			let key = apply_prefix(source.connection_prefix().into_vec(), path);
			let proof = source.query_proof(proof_height, vec![key]).await?;
			let commitment_proof = CommitmentProofBytes::try_from(proof)?;
			let msg = MsgRecvPacket {
				packet: packet.clone(),
				proofs: Proofs::new(commitment_proof, None, None, None, proof_height)?,
				signer: sink.account_id(),
			};
			let value = msg.encode_vec();
			let msg = Any { value, type_url: msg.type_url() };
			messages.push(msg)
		}

		// Get acknowledgement messages
		let packet_infos = source.query_recv_packets(channel_id, port_id, acks).await?;

		for packet_info in packet_infos {
			let ack = if let Some(ack) = packet_info.ack {
				ack
			} else {
				// Packet has no valid acknowledgement, skip
				continue
			};

			let packet = Packet {
				sequence: packet_info.sequence.into(),
				source_port: PortId::from_str(&packet_info.source_port)
					.expect("Port is should be valid"),
				source_channel: ChannelId::from_str(&packet_info.source_channel)
					.expect("Channel is should be valid"),
				destination_port: PortId::from_str(&packet_info.destination_port)
					.expect("Port is should be valid"),
				destination_channel: ChannelId::from_str(&packet_info.destination_channel)
					.expect("Channel is should be valid"),
				data: packet_info.data,
				timeout_height: packet_info.timeout_height.into(),
				timeout_timestamp: Timestamp::from_nanoseconds(packet_info.timeout_timestamp)
					.expect("Timestamp should be valid"),
			};
			// Check if ack is ready to be sent to sink
			// If sink does not have a client height that is equal to or greater than the packet
			// creation height, we can't send it yet packet_info.height should represent the
			// acknowledgement creation height on source chain
			if packet_info.height > latest_source_height_on_sink.revision_height {
				// Sink does not have client update required to prove acknowledgement packet message
				continue
			}

			let proof_height =
				Height::new(latest_source_height_on_sink.revision_number, packet_info.height);

			let (source_client_update_height, source_client_update_time) = sink
				.query_client_update_time_and_height(source.client_id(), proof_height)
				.await?;

			// Verify delay has passed
			let connection_delay = source_connection_end.delay_period();
			let block_delay = calculate_block_delay(connection_delay, sink.expected_block_time());
			if !has_delay_elapsed(
				sink_timestamp,
				sink_height,
				source_client_update_time,
				source_client_update_height,
				connection_delay,
				block_delay,
			)? {
				continue
			}

			let path = format!(
				"{}",
				AcksPath {
					port_id: packet.source_port.clone(),
					channel_id: packet.source_channel,
					sequence: packet.sequence
				}
			);

			let key = apply_prefix(source.connection_prefix().into_vec(), path);
			let proof = source.query_proof(proof_height, vec![key]).await?;
			let commitment_proof = CommitmentProofBytes::try_from(proof)?;
			let msg = MsgAcknowledgement {
				packet: packet.clone(),
				proofs: Proofs::new(commitment_proof, None, None, None, proof_height)?,
				acknowledgement: ack.into(),
				signer: sink.account_id(),
			};
			let value = msg.encode_vec();
			let msg = Any { value, type_url: msg.type_url() };
			messages.push(msg)
		}
	}

	Ok((messages, timeout_messages))
}

// todo: fix bug in this function in ibc-rs and remove from here
#[derive(Debug)]
pub enum TimeoutVariant {
	Height,
	Timestamp,
	Both,
}

pub fn timeout_variant(
	packet: &Packet,
	dst_chain_ts: &Timestamp,
	dst_chain_height: Height,
) -> Option<TimeoutVariant> {
	let height_timeout =
		packet.timeout_height != Height::zero() && packet.timeout_height <= dst_chain_height;
	let timestamp_timeout = packet.timeout_timestamp != Timestamp::none() &&
		(dst_chain_ts.check_expiry(&packet.timeout_timestamp) == Expired);
	if height_timeout && !timestamp_timeout {
		Some(TimeoutVariant::Height)
	} else if timestamp_timeout && !height_timeout {
		Some(TimeoutVariant::Timestamp)
	} else if timestamp_timeout && height_timeout {
		Some(TimeoutVariant::Both)
	} else {
		None
	}
}
