use crate::StreamExt;
use futures::future;
use hyperspace_primitives::TestProvider;
use ibc::events::IbcEvent;
use std::{future::Future, time::Duration};

pub fn parse_amount(amount: String) -> u128 {
	str::parse::<u128>(&amount).expect("Failed to parse as u128")
}

pub async fn timeout_future<T: Future>(future: T, secs: u64, reason: String) -> T::Output {
	let duration = Duration::from_secs(secs);
	match tokio::time::timeout(duration.clone(), future).await {
		Ok(output) => output,
		Err(_) => panic!("Future didn't finish within {duration:?}, {reason}"),
	}
}

pub async fn assert_timeout_packet<A>(chain: &A)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
{
	// wait for the timeout packet
	let future = chain
		.ibc_events()
		.await
		.filter_map(|(_, evs)| {
			future::ready(evs.into_iter().find(|ev| {
				matches!(ev, Some(IbcEvent::TimeoutPacket(_) | IbcEvent::TimeoutOnClosePacket(_)))
			}))
		})
		.take(1)
		.collect::<Vec<_>>();
	timeout_future(future, 20 * 60, format!("Didn't see Timeout packet on {}", chain.name())).await;
}
