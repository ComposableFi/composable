use crate::{assert_timeout_packet, timeout_future, StreamExt};
use futures::future;
use hyperspace::send_packet_relay::set_relay_status;
use hyperspace_primitives::TestProvider;
use ibc::{
	core::{
		ics03_connection,
		ics03_connection::{connection::Counterparty, msgs::conn_open_init::MsgConnectionOpenInit},
		ics04_channel,
		ics04_channel::{
			channel,
			channel::{ChannelEnd, Order, State},
			msgs::chan_open_init::MsgChannelOpenInit,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
	},
	events::IbcEvent,
	tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use pallet_ibc::Timeout;
use std::{str::FromStr, time::Duration};
use tendermint_proto::Protobuf;
use tokio::task::JoinHandle;

/// This will set up a connection and an ordered channel in-between the two chains.
/// `connection_delay` should be in seconds.
pub async fn setup_connection_and_ordered_channel<A, B>(
	chain_a: &A,
	chain_b: &B,
	connection_delay: u64,
	connection_id: Option<ConnectionId>,
) -> (JoinHandle<()>, ChannelId, ChannelId, ConnectionId)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	// Start relayer loop
	let handle = tokio::task::spawn(async move {
		hyperspace::relay(client_a_clone, client_b_clone, None, None).await.unwrap()
	});
	// check if an open transfer channel exists
	let ping_port = PortId::from_str("ping").unwrap();
	let channels = chain_a.query_channels().await.unwrap();
	if !channels.is_empty() {
		for (channel_id, port_id) in channels {
			let (latest_height, ..) = chain_a.latest_height_and_timestamp().await.unwrap();
			if let Ok(channel_response) =
				chain_a.query_channel_end(latest_height, channel_id, port_id.clone()).await
			{
				let channel_end = ChannelEnd::try_from(channel_response.channel.unwrap()).unwrap();
				if channel_end.state == State::Open && port_id == ping_port {
					return (
						handle,
						channel_id,
						channel_end.counterparty().channel_id.unwrap().clone(),
						channel_end.connection_hops[0].clone(),
					)
				}
			}
		}
	}

	let connection_id = if connection_id.is_none() {
		// Both clients have been updated, we can now start connection handshake
		let msg = MsgConnectionOpenInit {
			client_id: chain_a.client_id(),
			counterparty: Counterparty::new(chain_b.client_id(), None, chain_b.connection_prefix()),
			version: Some(ics03_connection::version::Version::default()),
			delay_period: Duration::from_secs(connection_delay),
			signer: chain_a.account_id(),
		};
		let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };
		chain_a.submit(vec![msg]).await.expect("Connection creation failed");

		log::info!(target: "hyperspace", "============= Wait till both chains have completed connection handshake =============");

		// wait till both chains have completed connection handshake
		let future = chain_b
			.ibc_events()
			.await
			.skip_while(|ev| future::ready(!matches!(ev, IbcEvent::OpenConfirmConnection(_))))
			.take(1)
			.collect::<Vec<_>>();

		let mut events = timeout_future(
			future,
			10 * 60,
			format!("Didn't see OpenConfirmConnection on {}", chain_b.name()),
		)
		.await;

		let connection_id = match events.pop() {
			Some(IbcEvent::OpenConfirmConnection(conn)) => conn.connection_id().unwrap().clone(),
			got => panic!("Last event should be OpenConfirmConnection: {got:?}"),
		};

		log::info!(target: "hyperspace", "============ Connection handshake completed: ConnectionId({connection_id}) ============");
		log::info!(target: "hyperspace", "=========================== Starting channel handshake ===========================");
		connection_id
	} else {
		connection_id.unwrap()
	};

	let channel = ChannelEnd::new(
		State::Init,
		Order::Ordered,
		channel::Counterparty::new(ping_port.clone(), None),
		vec![connection_id.clone()],
		ics04_channel::Version::new("ping-1".to_string()),
	);

	// open the transfer channel
	let msg = MsgChannelOpenInit::new(ping_port, channel, chain_a.account_id());
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };

	chain_a.submit(vec![msg]).await.expect("Connection creation failed");

	// wait till both chains have completed channel handshake
	log::info!(target: "hyperspace", "============= Wait till both chains have completed channel handshake =============");
	let future = chain_b
		.ibc_events()
		.await
		.skip_while(|ev| future::ready(!matches!(ev, IbcEvent::OpenConfirmChannel(_))))
		.take(1)
		.collect::<Vec<_>>();

	let mut events = timeout_future(
		future,
		10 * 60,
		format!("Didn't see OpenConfirmChannel on {}", chain_b.name()),
	)
	.await;

	let (channel_id, chain_b_channel_id) = match events.pop() {
		Some(IbcEvent::OpenConfirmChannel(chan)) =>
			(chan.counterparty_channel_id.unwrap(), chan.channel_id().unwrap().clone()),
		got => panic!("Last event should be OpenConfirmConnection: {got:?}"),
	};

	// channel handshake completed
	log::info!(target: "hyperspace", "============ Channel handshake completed: ChannelId({channel_id}) ============");

	(handle, channel_id, chain_b_channel_id, connection_id)
}

/// Send a ordered packets and assert acknowledgement
pub async fn send_ordered_packets_and_assert_acknowledgement<A, B>(
	chain_a: &A,
	chain_b: &B,
	channel_id: ChannelId,
) where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	chain_a
		.send_ping(channel_id, Timeout::Offset { height: Some(100), timestamp: Some(60 * 60) })
		.await
		.unwrap();

	chain_a
		.send_ping(channel_id, Timeout::Offset { height: Some(100), timestamp: Some(60 * 60) })
		.await
		.unwrap();

	let future = chain_b
		.ibc_events()
		.await
		.skip_while(|ev| future::ready(!matches!(ev, IbcEvent::AcknowledgePacket(_))))
		.take(2)
		.collect::<Vec<_>>();
	timeout_future(
		future,
		20 * 60,
		format!("Didn't see Acknowledgement packet on {}", chain_b.name()),
	)
	.await;
}

/// Send a packet on an ordered channel and assert timeout
pub async fn send_a_packet_on_ordered_channel_and_assert_timeout<A, B>(
	chain_a: &A,
	chain_b: &B,
	channel_id: ChannelId,
) where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	log::info!(target: "hyperspace", "Suspending send packet relay");
	set_relay_status(false);

	let timestamp = 60 * 2;
	chain_a
		.send_ping(channel_id, Timeout::Offset { height: Some(200), timestamp: Some(timestamp) })
		.await
		.unwrap();
	let timeout_timestamp = Duration::from_secs(timestamp).as_nanos() as u64;

	// Wait timeout timestamp to elapse, then
	let future = chain_b
		.subscribe_blocks()
		.await
		.skip_while(|block_number| {
			let block_number = *block_number;
			let chain_clone = chain_b.clone();
			async move {
				let timestamp = chain_clone.query_timestamp_at(block_number).await.unwrap();
				timestamp < timeout_timestamp
			}
		})
		.take(1)
		.collect::<Vec<_>>();

	log::info!(target: "hyperspace", "Waiting for packet timeout to elapse on counterparty");
	timeout_future(
		future,
		10 * 60,
		format!("Timeout timestamp was not reached on {}", chain_b.name()),
	)
	.await;

	set_relay_status(true);

	assert_timeout_packet(chain_a).await;
	log::info!(target: "hyperspace", "🚀🚀 Timeout packet successfully processed for ordered channel");
}
