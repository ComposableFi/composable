#![warn(unused_variables)]

use futures::StreamExt;
use primitives::Chain;

pub mod events;
pub mod logging;

use events::parse_events;

/// Core relayer loop, waits for new finality events and forwards any new [`ibc::IbcEvents`]
/// to the counter party chain.
pub async fn relay<A, B>(mut chain_a: A, mut chain_b: B) -> Result<(), anyhow::Error>
where
	A: Chain,
	A::Error: From<B::Error>,
	B: Chain,
	B::Error: From<A::Error>,
{
	let (mut chain_a_finality, mut chain_b_finality) =
		(chain_a.finality_notifications().await, chain_b.finality_notifications().await);
	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			result = chain_a_finality.next() => {
				match result {
					// stream closed
					None => break,
					Some(finality_event) => {
						log::info!("Received finality notification from {}", chain_a.name());
						let (msg_update_client, events, update_type) = match chain_a.query_latest_ibc_events(finality_event, &chain_b).await {
							Ok(resp) => resp,
							Err(err) => {
								log::error!("Failed to fetch IBC events for finality event on Chain A: {:?}", err);
								continue
							}
						};
						let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
						let mut messages = parse_events(&mut chain_a, &mut chain_b, events).await?;
						match (update_type.is_optional(), messages.is_empty()) {
							(true, true) => {
								// skip sending ibc messages if no new events
								log::info!("Skipping finality notification for {}, No new events", chain_a.name());
								continue
							},
							(false, true) => log::info!("Sending mandatory client update message for {}", chain_a.name()),
							_ => log::info!("Received finalized events from: {} {event_types:#?}", chain_a.name()),
						};
						// insert client update at first position.
						messages.insert(0, msg_update_client);
						log::info!("Received finalized events from: {} {event_types:#?}", chain_a.name());
						let type_urls = messages.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
						log::info!("Submitting messages to {}: {type_urls:#?}", chain_a.name());
						chain_b.submit_ibc_messages(messages).await?;
					}
				}
			},
			// new finality event from chain B
			result = chain_b_finality.next() => {
				match result {
					// stream closed
					None => break,
					Some(finality_event) => {
						log::info!("Received finality notification from {}", chain_b.name());
						let (msg_update_client, events, update_type) = match chain_b.query_latest_ibc_events(finality_event, &chain_a).await {
							Ok(resp) => resp,
							Err(err) => {
								log::error!("Failed to fetch IBC events for finality event on Chain B: {:?}", err);
								continue
							}
						};
						let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
						let mut messages = parse_events(&mut chain_b, &mut chain_a, events).await?;
						match (update_type.is_optional(), messages.is_empty()) {
							(true, true) => {
								// skip sending ibc messages if no new events
								log::info!("Skipping finality notification for {}, No new events", chain_b.name());
								continue
							},
							(false, true) => log::info!("Sending mandatory client update message for {}", chain_b.name()),
							_ => log::info!("Received finalized events from: {} {event_types:#?}", chain_a.name()),
						};
						// insert client update at first position.
						messages.insert(0, msg_update_client);
						let type_urls = messages.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
						log::info!("Submitting messages to {}: {type_urls:#?}", chain_b.name());
						chain_a.submit_ibc_messages(messages).await?;
					}
				}
			}
		}
	}

	Ok(())
}
