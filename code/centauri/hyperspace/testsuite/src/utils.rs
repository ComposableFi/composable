use crate::StreamExt;
use futures::future;
use hyperspace_primitives::TestProvider;
use ibc::events::IbcEvent;
use hyperspace_primitives::utils::timeout_future;

pub fn parse_amount(amount: String) -> u128 {
	str::parse::<u128>(&amount).expect("Failed to parse as u128")
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
		.skip_while(|ev| {
			future::ready(!matches!(
				ev,
				IbcEvent::TimeoutPacket(_) | IbcEvent::TimeoutOnClosePacket(_)
			))
		})
		.take(1)
		.collect::<Vec<_>>();
	timeout_future(future, 20 * 60, format!("Didn't see Timeout packet on {}", chain.name())).await;
}
