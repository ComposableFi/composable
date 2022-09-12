#[macro_export]
macro_rules! process_finality_event {
	($chain_a:ident, $chain_b:ident, $result:ident) => {
		match $result {
			// stream closed
			None => break,
			Some(finality_event) => {
				log::info!("Received finality notification from {}", $chain_a.name());
				let (update_client_header, events, update_type) =
					match $chain_a.query_latest_ibc_events(finality_event, &$chain_b).await {
						Ok(resp) => resp,
						Err(err) => {
							log::error!(
								"Failed to fetch IBC events for finality event for {} {:?}",
								$chain_a.name(),
								err
							);
							continue
						},
					};
				let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
				let (messages, timeouts) =
					parse_events(&mut $chain_a, &mut $chain_b, events, update_client_header)
						.await?;
				queue::flush_message_batch(timeouts, &$chain_a).await?;
				// there'd at least be the `MsgUpdateClient` packet.
				if messages.len() == 1 && update_type.is_optional() {
					// skip sending ibc messages if no new events
					log::info!("Skipping finality notification for {}", $chain_a.name());
					continue
				} else if messages.len() == 1 {
					log::info!("Sending mandatory client update message to {}", $chain_a.name());
				} else {
					log::info!(
						"Received finalized events from: {} {event_types:#?}",
						$chain_a.name()
					);
				}
				let type_urls =
					messages.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
				log::info!("Submitting messages to {}: {type_urls:#?}", $chain_b.name());
				queue::flush_message_batch(messages, &$chain_b).await?;
			},
		}
	};
}
