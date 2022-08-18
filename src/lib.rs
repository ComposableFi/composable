#[warn(unused_variables)]
use chain::Chain;
use futures::StreamExt;

pub mod chain;
mod error;
mod events;
pub mod logging;
mod messages;

pub use crate::messages::Messages;

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
						let (update_client_header, client_state, update_type) = chain_a.client_update_header(finality_event, &chain_b).await?;
						let events = match chain_a.query_latest_ibc_events(&update_client_header, &client_state).await {
							Ok(resp) => resp,
							Err(err) => {
								log::error!("Failed to fetch IBC events for finality event on Chain A: {:?}", err);
								continue
							}
						};
						log::info!("Received finality notification from chain A {:#?}", events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>());
						let messages = Messages::from(&mut chain_a, &mut chain_b,  events, update_client_header).await?;
						// there'd at least be the `MsgUpdateClient` packet.
						if messages.len() == 1 && update_type.is_optional() {
							// skip sending ibc messages if no new events
							continue
						}
						let type_urls = messages.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
						log::info!("Submitting messages to chain B {type_urls:#?}");
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
						let (update_client_header, client_state, update_type) = chain_b.client_update_header(finality_event, &chain_a).await?;
						let events = match chain_b.query_latest_ibc_events(&update_client_header, &client_state).await {
							Ok(resp) => resp,
							Err(err) => {
								log::error!("Failed to fetch IBC events for finality event on Chain B: {:?}", err);
								continue
							}
						};
						log::info!("Received finality notification from chain B {:#?}", events.iter().map(|ev| ev.event_type()).collect::<Vec<_>>());
						let messages = Messages::from(&mut chain_b, &mut chain_a, events, update_client_header).await?;
						// there'd at least be the `MsgUpdateClient` packet.
						if messages.len() == 1 && update_type.is_optional() {
							// skip sending ibc messages if no new events
							continue
						}
						let type_urls = messages.iter().map(|msg| msg.type_url.as_str()).collect::<Vec<_>>();
						log::info!("Submitting messages to chain A {type_urls:#?}");
						chain_a.submit_ibc_messages(messages).await?;
					}
				}
			}
		}
	}

	Ok(())
}
