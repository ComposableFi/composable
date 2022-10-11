use crate::{send_packet_and_assert_height_timeout, setup_connection_and_channel};
use hyperspace_primitives::TestProvider;
use ibc::core::ics24_host::identifier::PortId;
use std::time::Duration;

pub async fn ibc_messaging_submit_misbehaviour<A, B>(chain_a: &mut A, chain_b: &mut B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (handle, channel_id, channel_b, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, Duration::from_secs(60 * 2)).await;
	handle.abort();
	// Set channel whitelist and restart relayer loop
	chain_a.set_channel_whitelist(vec![(channel_id, PortId::transfer())]);
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())]);
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace::relay(client_a_clone, client_b_clone).await.unwrap()
	});
	send_packet_and_assert_height_timeout(chain_a, chain_b, channel_id).await;
	handle.abort()
}
