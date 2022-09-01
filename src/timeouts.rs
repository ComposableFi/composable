use crate::connection_delay::has_delay_elapsed;
use ibc::{
	core::{
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::{
			channel::{ChannelEnd, Order, State},
			context::calculate_block_delay,
			msgs::{timeout::MsgTimeout, timeout_on_close::MsgTimeoutOnClose},
			packet::Packet,
		},
		ics23_commitment::commitment::CommitmentProofBytes,
		ics24_host::{
			identifier::{ChannelId, PortId},
			path::{ChannelEndsPath, ReceiptsPath, SeqRecvsPath},
		},
	},
	proofs::Proofs,
	timestamp::Timestamp,
	tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use primitives::{apply_prefix, error::Error, query_undelivered_sequences, Chain};
use std::str::FromStr;
use tendermint_proto::Protobuf;

/// Get timeout messages that are ready to be sent back to source after factoring connection delay
pub async fn get_timed_out_packets_messages(
	source: &impl Chain,
	sink: &impl Chain,
) -> Result<Vec<Any>, anyhow::Error> {
	let mut messages = vec![];
	let (source_height, source_timestamp) = source.latest_height_and_timestamp().await?;
	let (sink_height, sink_timestamp) = sink.latest_height_and_timestamp().await?;
	let connection_whitelist = source.connection_whitelist().await?;
	for connection_id in connection_whitelist {
		let connection_response =
			source.query_connection_end(source_height, connection_id.clone()).await?;
		let connection_end =
			ConnectionEnd::try_from(connection_response.connection.ok_or_else(|| {
				Error::Custom(format!(
					"[get_timeout_messages] ConnectionEnd not found for {:?}",
					connection_id
				))
			})?)?;
		let connection_channels =
			source.query_connection_channels(source_height, &connection_id).await?;
		for identified_channel in connection_channels.channels {
			let port_id = PortId::from_str(&identified_channel.port_id)
				.map_err(|_| Error::Custom("Found an invalid port id".to_string()))?;
			let channel_id = ChannelId::from_str(&identified_channel.channel_id)
				.map_err(|_| Error::Custom("Found an invalid channel id".to_string()))?;
			let seqs = query_undelivered_sequences(
				source_height,
				sink_height,
				channel_id,
				port_id.clone(),
				source,
				sink,
			)
			.await?;
			let packet_infos = source.query_send_packets(channel_id, port_id, seqs).await?;
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

				// Check if packet has timed out
				if !packet.timed_out(&sink_timestamp, sink_height) {
					continue
				}

				// Check if connection delay is satisfied

				// If we can't get the client update time and height, skip processing of this packet
				let client_update_time_and_height = source
					.query_client_update_time_and_height(sink.client_id(), packet.timeout_height)
					.await;
				if client_update_time_and_height.is_err() {
					continue
				}

				let (client_update_height, client_update_time) =
					client_update_time_and_height.unwrap();

				let connection_delay = connection_end.delay_period();
				let block_delay =
					calculate_block_delay(connection_delay, sink.expected_block_time());

				if !has_delay_elapsed(
					source_timestamp,
					source_height,
					client_update_time,
					client_update_height,
					connection_delay,
					block_delay,
				)? {
					continue
				}

				let sink_channel_response = sink
					.query_channel_end(
						packet.timeout_height,
						packet.destination_channel,
						packet.destination_port.clone(),
					)
					.await?;

				let sink_channel_end =
					ChannelEnd::try_from(sink_channel_response.channel.ok_or_else(|| {
						Error::Custom(format!(
							"[get_timeout_messages] ChannelEnd not found for {:?}/{:?}",
							packet.destination_channel,
							packet.destination_port.clone()
						))
					})?)?;

				let mut keys = vec![];
				if sink_channel_end.state == State::Closed {
					let path = format!(
						"{}",
						ChannelEndsPath(
							packet.destination_port.clone(),
							packet.destination_channel
						)
					);
					keys.push(apply_prefix(sink.connection_prefix().into_vec(), path))
				}
				if sink_channel_end.ordering == Order::Ordered {
					let path = format!(
						"{}",
						SeqRecvsPath(packet.destination_port.clone(), packet.destination_channel)
					);
					keys.push(apply_prefix(sink.connection_prefix().into_vec(), path))
				} else {
					let path = format!(
						"{}",
						ReceiptsPath {
							port_id: packet.destination_port.clone(),
							channel_id: packet.destination_channel,
							sequence: packet.sequence
						}
					);
					keys.push(apply_prefix(sink.connection_prefix().into_vec(), path))
				};

				let proof = sink.query_proof(packet.timeout_height, keys).await?;
				let next_sequence_recv = sink
					.query_next_sequence_recv(
						packet.timeout_height,
						&packet.destination_port.clone(),
						&packet.destination_channel.clone(),
					)
					.await?;
				let commitment_proof = CommitmentProofBytes::try_from(proof)?;
				if sink_channel_end.state == State::Closed {
					let msg = MsgTimeoutOnClose {
						packet: packet.clone(),
						next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
						proofs: Proofs::new(
							commitment_proof,
							None,
							None,
							None,
							packet.timeout_height,
						)?,

						signer: source.account_id(),
					};
					let value = msg.encode_vec();
					let msg = Any { value, type_url: msg.type_url() };

					messages.push(msg)
				} else {
					let msg = MsgTimeout {
						packet: packet.clone(),
						next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
						proofs: Proofs::new(commitment_proof, None, None, None, sink_height)?,

						signer: source.account_id(),
					};
					let value = msg.encode_vec();
					let msg = Any { value, type_url: msg.type_url() };
					messages.push(msg)
				}
			}
		}
	}

	Ok(messages)
}
