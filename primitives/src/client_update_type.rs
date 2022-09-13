use crate::{query_undelivered_acks, query_undelivered_sequences, Chain, Error};
use ibc::core::ics04_channel::{channel::ChannelEnd, context::calculate_block_delay};
use std::time::Duration;

/// Checks if the client update for a header at the given height is mandatory
/// Accepts the `source`, `sink`, source header height and timestamp of the header
pub async fn is_mandatory_update(
	source: &impl Chain,
	sink: &impl Chain,
	header_height: u64,
	timestamp: u64,
) -> Result<bool, anyhow::Error> {
	let (source_height, ..) = source.latest_height_and_timestamp().await?;
	let (sink_height, ..) = sink.latest_height_and_timestamp().await?;
	let channel_whitelist = source.channel_whitelist();

	for (channel_id, port_id) in channel_whitelist {
		let seqs = query_undelivered_sequences(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			source,
			sink,
		)
		.await?;

		let acks = query_undelivered_acks(
			source_height,
			sink_height,
			channel_id,
			port_id.clone(),
			source,
			sink,
		)
		.await?;

		let packet_infos = source.query_send_packets(channel_id, port_id.clone(), seqs).await?;
		for packet_info in packet_infos {
			// If at least one packet would require this header update for proof verification exit
			if packet_info.height == header_height {
				return Ok(true)
			}
		}

		let packet_infos = source.query_recv_packets(channel_id, port_id.clone(), acks).await?;
		for packet_info in packet_infos {
			// If at least one packet acknowledgement would require this header update for proof
			// verification exit
			if packet_info.height == header_height {
				return Ok(true)
			}
		}

		// Check for potential packet timeouts on sink
		let channel_response = source.query_channel_end(source_height, channel_id, port_id).await?;
		let channel_end = ChannelEnd::try_from(
			channel_response
				.channel
				.ok_or_else(|| Error::Custom("ChannelEnd not could not be decoded".to_string()))?,
		)
		.map_err(|e| Error::Custom(e.to_string()))?;
		let counterparty_channel_id = channel_end
			.counterparty()
			.channel_id
			.ok_or_else(|| Error::Custom("Expected counterparty channel id".to_string()))?;
		let counterparty_port_id = channel_end.counterparty().port_id.clone();

		// Undelivered packets from sink to source
		let undelivered_sink_seqs = query_undelivered_sequences(
			sink_height,
			source_height,
			counterparty_channel_id,
			counterparty_port_id.clone(),
			sink,
			source,
		)
		.await?;
		let packet_infos = sink
			.query_send_packets(
				counterparty_channel_id,
				counterparty_port_id,
				undelivered_sink_seqs,
			)
			.await?;
		for packet_info in packet_infos {
			let timeout_height = packet_info.timeout_height.revision_height + 1;
			// Check if it matches timeout height
			if header_height == timeout_height {
				return Ok(true)
			}

			if timestamp < packet_info.timeout_timestamp {
				continue
			}
			let timeout_diff = timestamp - packet_info.timeout_timestamp;
			let timeout_duration = Duration::from_nanos(timeout_diff);
			let timeout_duration_blocks =
				calculate_block_delay(timeout_duration, source.expected_block_time());
			// If the difference in blocks is approximately 2 or less we should send the update
			if timeout_duration_blocks <= 2 {
				return Ok(true)
			}
		}
	}

	Ok(false)
}
