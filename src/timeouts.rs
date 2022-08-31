use ibc::{
	core::{
		ics04_channel::packet::Packet,
		ics24_host::identifier::{ChannelId, PortId},
	},
	timestamp::Timestamp,
};
use primitives::{error::Error, Chain};
use std::str::FromStr;

/// Get timeout messages that are ready to be sent back to source
pub async fn get_timed_out_packets(
	source: &mut impl Chain,
	sink: &mut impl Chain,
) -> Result<Vec<Packet>, anyhow::Error> {
	let mut timed_out_packets = vec![];
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

				timed_out_packets.push(packet)
			}
		}
	}

	Ok(timed_out_packets)
}
