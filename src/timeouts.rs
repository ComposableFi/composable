use ibc::{
	core::{
		ics04_channel::{
			channel::{ChannelEnd, Order, State},
			msgs::{timeout::MsgTimeout, timeout_on_close::MsgTimeoutOnClose},
			packet::Packet,
		},
		ics23_commitment::commitment::CommitmentProofBytes,
		ics24_host::{
			identifier::{ChannelId, ConnectionId, PortId},
			path::{ChannelEndsPath, ReceiptsPath, SeqRecvsPath},
		},
	},
	proofs::Proofs,
	timestamp::Timestamp,
	tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use primitives::{apply_prefix, error::Error, Chain};
use std::{collections::HashMap, str::FromStr};
use tendermint_proto::Protobuf;

/// Get timeout messages that are ready to be sent back to source
pub async fn get_timed_out_packets(
	source: &mut impl Chain,
	sink: &mut impl Chain,
) -> Result<HashMap<ConnectionId, Vec<Any>>, anyhow::Error> {
	let mut messages: HashMap<ConnectionId, Vec<Any>> = HashMap::new();
	let (source_height, ..) = source.latest_height_and_timestamp().await?;
	let (sink_height, sink_timestamp) = sink.latest_height_and_timestamp().await?;
	let connection_whitelist = source.connection_whitelist().await?;
	for connection_id in connection_whitelist {
		let connection_channels =
			source.query_connection_channels(source_height, &connection_id).await?;
		for identified_channel in connection_channels.channels {
			let port_id = PortId::from_str(&identified_channel.port_id)
				.map_err(|_| Error::Custom("Found an invalid port id".to_string()))?;
			let channel_id = ChannelId::from_str(&identified_channel.channel_id)
				.map_err(|_| Error::Custom("Found an invalid channel id".to_string()))?;
			let seqs = source.query_undelivered_sequences(channel_id, port_id.clone()).await?;
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

				let sink_channel_response = sink
					.query_channel_end(
						sink_height,
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

				let proof = sink.query_proof(sink_height, keys).await?;
				let next_sequence_recv = sink
					.query_next_sequence_recv(
						sink_height,
						&packet.destination_port.clone(),
						&packet.destination_channel.clone(),
					)
					.await?;
				let commitment_proof = CommitmentProofBytes::try_from(proof)?;
				if sink_channel_end.state == State::Closed {
					let msg = MsgTimeoutOnClose {
						packet: packet.clone(),
						next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
						proofs: Proofs::new(commitment_proof, None, None, None, sink_height)?,

						signer: source.account_id(),
					};
					let value = msg.encode_vec();
					let msg = Any { value, type_url: msg.type_url() };

					messages
						.entry(connection_id.clone())
						.and_modify(|batch| batch.push(msg))
						.or_insert(vec![]);
				} else {
					let msg = MsgTimeout {
						packet: packet.clone(),
						next_sequence_recv: next_sequence_recv.next_sequence_receive.into(),
						proofs: Proofs::new(commitment_proof, None, None, None, sink_height)?,

						signer: source.account_id(),
					};
					let value = msg.encode_vec();
					let msg = Any { value, type_url: msg.type_url() };
					messages
						.entry(connection_id.clone())
						.and_modify(|batch| batch.push(msg))
						.or_insert(vec![]);
				}
			}
		}
	}

	Ok(messages)
}
