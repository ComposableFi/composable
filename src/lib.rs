#![warn(unused_variables)]

use futures::StreamExt;
use primitives::Chain;

pub mod connection_delay;
pub mod events;
pub mod logging;
pub mod timeouts;

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
						let (update_client_header, events, update_type) = match chain_a.query_latest_ibc_events(finality_event, &chain_b).await {
							Ok(resp) => resp,
							Err(err) => {
								log::error!("Failed to fetch IBC events for finality event on Chain A: {:?}", err);
								continue
							}
						};
						let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
						let messages = parse_events(&mut chain_a, &mut chain_b,  events, update_client_header).await?;
						// there'd at least be the `MsgUpdateClient` packet.
						if messages.len() == 1 && update_type.is_optional() {
							// skip sending ibc messages if no new events
							log::info!("Skipping finality notification for {}", chain_a.name());
							continue
						} else if messages.len() == 1 {
							log::info!("Sending mandatory client update message to {}", chain_a.name());
						} else {
							log::info!("Received finalized events from: {} {event_types:#?}", chain_a.name());
						}
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
						let (update_client_header, events, update_type) = match chain_b.query_latest_ibc_events(finality_event, &chain_a).await {
							Ok(resp) => resp,
							Err(err) => {
								log::error!("Failed to fetch IBC events for finality event on Chain B: {:?}", err);
								continue
							}
						};
						let event_types = events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>();
						let messages = parse_events(&mut chain_b, &mut chain_a, events, update_client_header).await?;
						// there'd at least be the `MsgUpdateClient` packet.
						if messages.len() == 1 && update_type.is_optional() {
							log::info!("Skipping finality notification for {}", chain_b.name());
							// skip sending ibc messages if no new events
							continue
						} else if messages.len() == 1 {
							log::info!("Sending mandatory client update message to {}", chain_b.name());
						} else {
							log::info!("Received finalized events from {}: {event_types:#?}", chain_b.name());
						}
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
