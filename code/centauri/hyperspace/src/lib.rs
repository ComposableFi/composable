#![warn(unused_variables)]

use futures::StreamExt;
use ibc::events::IbcEvent;
use primitives::Chain;
use std::time::Duration;
use tokio::time::timeout;

pub mod events;
pub mod logging;
mod macros;
pub mod packets;
pub mod queue;

use events::{has_packet_events, parse_events};
use metrics::handler::MetricsHandler;

/// Core relayer loop, waits for new finality events and forwards any new [`ibc::IbcEvents`]
/// to the counter party chain.
pub async fn relay<A, B>(
	mut chain_a: A,
	mut chain_b: B,
	mut chain_a_metrics: Option<MetricsHandler>,
	mut chain_b_metrics: Option<MetricsHandler>,
) -> Result<(), anyhow::Error>
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
				process_finality_event!(chain_a, chain_b, chain_a_metrics, result)
			}
			// new finality event from chain B
			result = chain_b_finality.next() => {
				process_finality_event!(chain_b, chain_a, chain_b_metrics, result)
			}
		}
	}

	Ok(())
}

pub async fn fish<A, B>(chain_a: A, chain_b: B) -> Result<(), anyhow::Error>
where
	A: Chain,
	A::Error: From<B::Error>,
	B: Chain,
	B::Error: From<A::Error>,
{
	// TODO: use block subscription to retrieve events and extrinsics simultaneously
	let (mut chain_a_client_updates, mut chain_b_client_updates) =
		(chain_a.ibc_events().await, chain_b.ibc_events().await);
	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			result = chain_a_client_updates.next() => {
				match result {
					// stream closed
					None => break,
					Some((transaction_id, events)) => {
						for (i, event) in events.into_iter().enumerate() {
							if let Some(IbcEvent::UpdateClient(client_update)) = event {
								if *client_update.client_id() != chain_b.client_id() {
									continue;
								}
								let message = timeout(Duration::from_secs(20), chain_a.query_client_message(
									transaction_id.block_hash,
									transaction_id.tx_index,
									i,
								)).await??;
								chain_b.check_for_misbehaviour(&chain_a, message).await?;
							}
						}
					}
				};
			}
			// new finality event from chain B
			result = chain_b_client_updates.next() => {
				match result {
					// stream closed
					None => break,
					Some((transaction_id, events)) => {
						for (i, event) in events.into_iter().enumerate() {
							if let Some(IbcEvent::UpdateClient(client_update)) = event {
								if *client_update.client_id() != chain_a.client_id() {
									continue;
								}
								let message = timeout(Duration::from_secs(20), chain_b.query_client_message(
									transaction_id.block_hash,
									transaction_id.tx_index,
									i,
								)).await??;
								chain_a.check_for_misbehaviour(&chain_b, message).await?;
							}
						}
					}
				};
			}
		}
	}

	Ok(())
}
