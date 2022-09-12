#[cfg(feature = "testing")]
use crate::send_packet_relay::packet_relay_status;

use crate::packet_messages_utils::{
	construct_ack_message, construct_recv_message, construct_timeout_message,
	get_timeout_proof_height, verify_delay_passed, VerifyDelayOn,
};
use ibc::{
	core::{
		ics02_client::client_state::AnyClientState,
		ics03_connection::connection::ConnectionEnd,
		ics04_channel::channel::{ChannelEnd, State},
	},
	Height,
};
use ibc_proto::google::protobuf::Any;
use primitives::{
	error::Error, packet_info_to_packet, query_undelivered_acks, query_undelivered_sequences, Chain,
};

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
	let channel_whitelist = source.channel_whitelist();

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
					"[query_ready_and_timed_out_packets] ConnectionEnd not found for {:?}",
					connection_id
				))
			})?)?;

		let sink_channel_id = source_channel_end
			.counterparty()
			.channel_id
			.ok_or_else(|| {
				Error::Custom(
					" An Open Channel End should have a valid counterparty channel id".to_string(),
				)
			})?
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
			source_client_state_on_sink.client_state.ok_or_else(|| {
				Error::Custom(format!(
					"Client state for {} should exist on {}",
					source.name(),
					sink.name()
				))
			})?,
		)
		.map_err(|_| {
			Error::Custom(format!(
				"Invalid Client state for {} should found on {}",
				source.name(),
				sink.name()
			))
		})?;
		let latest_source_height_on_sink = source_client_state_on_sink.latest_height();

		let packet_infos = source.query_send_packets(channel_id, port_id.clone(), seqs).await?;
		for packet_info in packet_infos {
			let packet = packet_info_to_packet(&packet_info);

			// Check if packet has timed out
			if packet.timed_out(&sink_timestamp, sink_height) {
				let proof_height = if let Some(proof_height) =
					get_timeout_proof_height(sink, sink_timestamp, sink_height, &packet).await
				{
					proof_height
				} else {
					continue
				};

				if !verify_delay_passed(
					source,
					sink,
					source_timestamp,
					source_height,
					sink_timestamp,
					sink_height,
					source_connection_end.delay_period(),
					proof_height,
					VerifyDelayOn::Source,
				)
				.await?
				{
					continue
				}

				let msg = construct_timeout_message(
					source,
					sink,
					&sink_channel_end,
					packet,
					next_sequence_recv.next_sequence_receive,
					proof_height,
				)
				.await?;
				timeout_messages.push(msg);
				continue
			}

			// If packet has not timed out but channel is closed on sink we skip
			// Since we have no reference point for when this channel was closed so we can't
			// calculate connection delays yet
			if sink_channel_end.state == State::Closed {
				continue
			}

			#[cfg(feature = "testing")]
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

			if !verify_delay_passed(
				source,
				sink,
				source_timestamp,
				source_height,
				sink_timestamp,
				sink_height,
				source_connection_end.delay_period(),
				proof_height,
				VerifyDelayOn::Sink,
			)
			.await?
			{
				continue
			}

			let msg = construct_recv_message(source, sink, packet, proof_height).await?;
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

			if !verify_delay_passed(
				source,
				sink,
				source_timestamp,
				source_height,
				sink_timestamp,
				sink_height,
				source_connection_end.delay_period(),
				proof_height,
				VerifyDelayOn::Sink,
			)
			.await?
			{
				continue
			}

			let msg = construct_ack_message(source, sink, packet, ack, proof_height).await?;

			messages.push(msg)
		}
	}

	Ok((messages, timeout_messages))
}
