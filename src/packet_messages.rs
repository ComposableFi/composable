use crate::{connection_delay::has_delay_elapsed, packet_relay_status};
use ibc::{
	core::{
		ics02_client::client_state::AnyClientState,
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
		ics24_host::path::{
			AcksPath, ChannelEndsPath, CommitmentsPath, ReceiptsPath, SeqRecvsPath,
		},
	},
	proofs::Proofs,
	timestamp::{Expiry::Expired, Timestamp},
	tx_msg::Msg,
	Height,
};
use ibc_proto::google::protobuf::Any;
use primitives::{
	apply_prefix, error::Error, find_block_height_by_timestamp, packet_info_to_packet,
	query_undelivered_acks, query_undelivered_sequences, Chain,
};

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
			let packet = packet_info_to_packet(&packet_info);

			// Check if packet has timed out
			if packet.timed_out(&sink_timestamp, sink_height) {
				let timeout_variant =
					timeout_variant(&packet, &sink_timestamp, sink_height).unwrap();

				let proof_height = match timeout_variant {
					TimeoutVariant::Height => packet.timeout_height.add(1),
					TimeoutVariant::Timestamp =>
						if let Some(height) = find_block_height_by_timestamp(
							sink,
							packet.timeout_timestamp,
							sink_timestamp,
							sink_height,
						)
						.await
						{
							height
						} else {
							continue
						},
					TimeoutVariant::Both => {
						let timeout_height = if let Some(height) = find_block_height_by_timestamp(
							sink,
							packet.timeout_timestamp,
							sink_timestamp,
							sink_height,
						)
						.await
						{
							height
						} else {
							continue
						};
						if timeout_height < packet.timeout_height {
							packet.timeout_height.add(1)
						} else {
							timeout_height
						}
					},
				};

				let (sink_client_update_height, sink_client_update_time) =
					if let Ok(client_update) = source
						.query_client_update_time_and_height(sink.client_id(), proof_height)
						.await
					{
						client_update
					} else {
						// If the source does not have a client update for the proof height yet, we
						// skip
						continue
					};

				let connection_delay = source_connection_end.delay_period();
				let block_delay =
					calculate_block_delay(connection_delay, source.expected_block_time());
				if !has_delay_elapsed(
					source_timestamp,
					source_height,
					sink_client_update_time,
					sink_client_update_height, // shouldn't be the latest.
					connection_delay,
					block_delay,
				)? {
					continue
				}

				let key = if sink_channel_end.ordering == Order::Ordered {
					let path = get_key_path(KeyPathType::SeqRecv, &packet);
					apply_prefix(sink.connection_prefix().into_vec(), path)
				} else {
					let path = get_key_path(KeyPathType::ReceiptPath, &packet);
					apply_prefix(sink.connection_prefix().into_vec(), path)
				};

				let proof_unreceived = sink.query_proof(proof_height, vec![key]).await?;
				let proof_unreceived = CommitmentProofBytes::try_from(proof_unreceived)?;
				let msg = if sink_channel_end.state == State::Closed {
					let path = get_key_path(KeyPathType::ChannelPath, &packet);
					let channel_key = apply_prefix(sink.connection_prefix().into_vec(), path);
					let proof_closed = sink.query_proof(proof_height, vec![channel_key]).await?;
					let proof_closed = CommitmentProofBytes::try_from(proof_closed)?;
					let msg = MsgTimeoutOnClose {
						packet: packet.clone(),
						next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
						proofs: Proofs::new(
							proof_unreceived,
							None,
							None,
							Some(proof_closed),
							proof_height,
						)?,
						signer: source.account_id(),
					};
					let value = msg.encode_vec();
					Any { value, type_url: msg.type_url() }
				} else {
					let msg = MsgTimeout {
						packet: packet.clone(),
						next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
						proofs: Proofs::new(proof_unreceived, None, None, None, proof_height)?,

						signer: source.account_id(),
					};
					let value = msg.encode_vec();
					Any { value, type_url: msg.type_url() }
				};
				timeout_messages.push(msg);
				continue
			}

			// If packet has not timed out but channel is closed on sink we skip
			// Since we have no reference point for when this channel was closed so we can't
			// calculate connection delays yet
			if sink_channel_end.state == State::Closed {
				continue
			}

			// If packet relay status is paused skip
			if !packet_relay_status() {
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

			let path = get_key_path(KeyPathType::CommitmentPath, &packet);

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
			let packet = packet_info_to_packet(&packet_info);
			let ack = if let Some(ack) = packet_info.ack {
				ack
			} else {
				// Packet has no valid acknowledgement, skip
				continue
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

			let path = get_key_path(KeyPathType::AcksPath, &packet);

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

enum KeyPathType {
	SeqRecv,
	ReceiptPath,
	CommitmentPath,
	AcksPath,
	ChannelPath,
}

fn get_key_path(key_path_type: KeyPathType, packet: &Packet) -> String {
	match key_path_type {
		KeyPathType::SeqRecv => {
			format!(
				"{}",
				SeqRecvsPath(packet.destination_port.clone(), packet.destination_channel.clone())
			)
		},
		KeyPathType::ReceiptPath => {
			format!(
				"{}",
				ReceiptsPath {
					port_id: packet.destination_port.clone(),
					channel_id: packet.destination_channel.clone(),
					sequence: packet.sequence.clone()
				}
			)
		},
		KeyPathType::CommitmentPath => {
			format!(
				"{}",
				CommitmentsPath {
					port_id: packet.source_port.clone(),
					channel_id: packet.source_channel.clone(),
					sequence: packet.sequence.clone()
				}
			)
		},
		KeyPathType::AcksPath => {
			format!(
				"{}",
				AcksPath {
					port_id: packet.source_port.clone(),
					channel_id: packet.source_channel.clone(),
					sequence: packet.sequence.clone()
				}
			)
		},
		KeyPathType::ChannelPath => {
			format!(
				"{}",
				ChannelEndsPath(
					packet.destination_port.clone(),
					packet.destination_channel.clone()
				)
			)
		},
	}
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
