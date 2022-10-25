#[macro_export]
macro_rules! process_finality_event {
	($source:ident, $sink:ident, $metrics:expr, $result:ident) => {
		match $result {
			// stream closed
			None => break,
			Some(finality_event) => {
				log::info!("=======================================================");
				log::info!("Received finality notification from {}", $source.name());
				let (msg_update_client, events, update_type) =
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
				if let Some(metrics) = $metrics.as_mut() {
					if let Err(e) = metrics.handle_events(events.as_slice()).await {
						log::error!("Failed to handle metrics for {} {:?}", $source.name(), e);
					}
				}
				let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
				let (mut messages, timeouts) =
					parse_events(&mut $source, &mut $sink, events).await?;
				if !timeouts.is_empty() {
					if let Some(metrics) = $metrics.as_ref() {
						metrics.handle_timeouts(timeouts.as_slice()).await;
					}
					let type_urls =
						timeouts.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
					log::info!("Submitting timeout messages to {}: {type_urls:#?}", $source.name());
					queue::flush_message_batch(timeouts, $metrics.as_ref(), &$source).await?;
				}
				// We want to send client update if packet messages exist but where not sent due to
				// a connection delay even if client update message is optional
				match (
					update_type.is_optional(),
					has_packet_events(&event_types),
					messages.is_empty(),
				) {
					(true, false, true) => {
						// skip sending ibc messages if no new events
						log::info!(
							"Skipping finality notification for {}, No new events",
							$source.name()
						);
						continue
					},
					(false, _, true) =>
						log::info!("Sending mandatory client update message for {}", $source.name()),
					_ => log::info!(
						"Received finalized events from: {} {event_types:#?}",
						$source.name()
					),
				};
				// insert client update at first position.
				messages.insert(0, msg_update_client);
				if let Some(metrics) = $metrics.as_ref() {
					metrics.handle_messages(messages.as_slice()).await;
				}
				let type_urls =
					messages.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
				log::info!("Submitting messages to {}: {type_urls:#?}", $sink.name());
				queue::flush_message_batch(messages, $metrics.as_ref(), &$sink).await?;
			},
		}
	};
}
