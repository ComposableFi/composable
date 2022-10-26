#![warn(unused_variables)]

use futures::StreamExt;
use ibc::events::IbcEvent;
use primitives::Chain;

pub mod events;
pub mod logging;
mod macros;
pub mod packets;
pub mod queue;

use events::{has_packet_events, parse_events};
use ibc::core::ics24_host::identifier::ClientId;
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

	let filter_events = |events: Vec<Option<IbcEvent>>, counterparty_client_id: ClientId| {
		events.into_iter().enumerate().filter(move |(_, event)| match event {
			Some(IbcEvent::UpdateClient(client_update)) =>
				*client_update.client_id() == counterparty_client_id,
			_ => false,
		})
	};

	// loop forever
	loop {
		tokio::select! {
			// new finality event from chain A
			result = chain_a_client_updates.next() => {
				let (transaction_id, events) = match result {
					// stream closed
					None => break,
					Some(val) => val,
				};

				for (i, _event) in filter_events(events, chain_b.client_id()) {
					let message = chain_a.query_client_message(
						transaction_id.block_hash,
						transaction_id.tx_index as usize,
						i,
					).await?;
					chain_b.check_for_misbehaviour(&chain_a, message).await?;
				}
			}
			// new finality event from chain B
			result = chain_b_client_updates.next() => {
				let (transaction_id, events) = match result {
					// stream closed
					None => break,
					Some(val) => val,
				};

				for (i, _event) in filter_events(events, chain_a.client_id()) {
					let message = chain_b.query_client_message(
						transaction_id.block_hash,
						transaction_id.tx_index as usize,
						i,
					).await?;
					chain_a.check_for_misbehaviour(&chain_a, message).await?;
				}
			}
		}
	}

	Ok(())
}
