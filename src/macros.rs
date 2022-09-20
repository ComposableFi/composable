#[macro_export]
macro_rules! process_finality_event {
	($source:ident, $sink:ident, $result:ident) => {
		match $result {
			// stream closed
			None => break,
			Some(finality_event) => {
				log::info!("Received finality notification from {}", $source.name());
				let (update_client_header, events, update_type) =
					match $source.query_latest_ibc_events(finality_event, &$sink).await {
						Ok(resp) => resp,
						Err(err) => {
							log::error!(
								"Failed to fetch IBC events for finality event for {} {:?}",
								$source.name(),
								err
							);
							continue
						},
					};
				let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
				let (messages, timeouts) =
					parse_events(&mut $source, &mut $sink, events, update_client_header).await?;
				if !timeouts.is_empty() {
					let type_urls =
						timeouts.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
					log::info!("Submitting timeout messages to {}: {type_urls:#?}", $source.name());
					queue::flush_message_batch(timeouts, &$source).await?;
				}
				// there'd at least be the `MsgUpdateClient` packet.
				// We want to send client update if packet message exist but where not sent due to a
				// connection delay
				if messages.len() == 1 &&
					!has_packet_events(&event_types) &&
					update_type.is_optional()
				{
					// skip sending ibc messages if no new events
					log::info!("Skipping finality notification for {}", $source.name());
					continue
				} else if messages.len() == 1 {
					log::info!("Sending mandatory client update message to {}", $sink.name());
				} else {
					log::info!(
						"Received finalized events from: {} {event_types:#?}",
						$source.name()
					);
				}
				let type_urls =
					messages.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
				log::info!("Submitting messages to {}: {type_urls:#?}", $sink.name());
				queue::flush_message_batch(messages, &$sink).await?;
			},
		}
	};
}
