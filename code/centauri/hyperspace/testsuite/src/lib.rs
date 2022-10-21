#![allow(clippy::all)]

use crate::utils::{assert_timeout_packet, parse_amount, timeout_future};
use futures::{future, StreamExt};
use hyperspace::send_packet_relay::set_relay_status;
use hyperspace_primitives::TestProvider;
use ibc::{
	applications::transfer::{msgs::transfer::MsgTransfer, Amount, PrefixedCoin, VERSION},
	core::{
		ics03_connection::{
			self, connection::Counterparty, msgs::conn_open_init::MsgConnectionOpenInit,
		},
		ics04_channel::{
			self, channel,
			channel::{ChannelEnd, Order, State},
			msgs::{chan_close_init::MsgChannelCloseInit, chan_open_init::MsgChannelOpenInit},
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

pub mod ordered_channels;
mod utils;

/// This will set up a connection and ics20 channel in-between the two chains.
/// `connection_delay` should be in seconds.
async fn setup_connection_and_channel<A, B>(
	chain_a: &A,
	chain_b: &B,
	connection_delay: Duration,
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
	let (latest_height, ..) = chain_a.latest_height_and_timestamp().await.unwrap();
	let connections = chain_a
		.query_connection_using_client(
			latest_height.revision_height as u32,
			chain_b.client_id().to_string(),
		)
		.await
		.unwrap();

	for connection in connections {
		let connection_id = ConnectionId::from_str(&connection.id).unwrap();
		let connection_end = chain_a
			.query_connection_end(latest_height, connection_id.clone())
			.await
			.unwrap()
			.connection
			.unwrap();

		let delay_period = Duration::from_nanos(connection_end.delay_period);

		dbg!(&connection_delay);
		dbg!(&delay_period);

		if delay_period != connection_delay {
			continue
		}

		let channels = chain_a
			.query_connection_channels(latest_height, &connection_id)
			.await
			.unwrap()
			.channels;

		for channel in channels {
			let channel_id = ChannelId::from_str(&channel.channel_id).unwrap();
			let channel_end = chain_a
				.query_channel_end(latest_height, channel_id, PortId::transfer())
				.await
				.unwrap()
				.channel
				.unwrap();
			let channel_end = ChannelEnd::try_from(channel_end).unwrap();

			if channel_end.state == State::Open && channel.port_id == PortId::transfer().to_string()
			{
				return (
					handle,
					channel_id,
					channel_end.counterparty().channel_id.unwrap().clone(),
					channel_end.connection_hops[0].clone(),
				)
			}
		}
	}

	// Both clients have been updated, we can now start connection handshake
	let msg = MsgConnectionOpenInit {
		client_id: chain_a.client_id(),
		counterparty: Counterparty::new(chain_b.client_id(), None, chain_b.connection_prefix()),
		version: Some(ics03_connection::version::Version::default()),
		delay_period: connection_delay,
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
		15 * 60,
		format!("Didn't see OpenConfirmConnection on {}", chain_b.name()),
	)
	.await;

	let connection_id = match events.pop() {
		Some(IbcEvent::OpenConfirmConnection(conn)) => conn.connection_id().unwrap().clone(),
		got => panic!("Last event should be OpenConfirmConnection: {got:?}"),
	};

	log::info!(target: "hyperspace", "============ Connection handshake completed: ConnectionId({connection_id}) ============");
	log::info!(target: "hyperspace", "=========================== Starting channel handshake ===========================");

	let channel = ChannelEnd::new(
		State::Init,
		Order::Unordered,
		channel::Counterparty::new(PortId::transfer(), None),
		vec![connection_id.clone()],
		ics04_channel::Version::new(VERSION.to_string()),
	);

	// open the transfer channel
	let msg = MsgChannelOpenInit::new(PortId::transfer(), channel, chain_a.account_id());
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
		15 * 60,
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

/// Attempts to send 20% of funds of chain_a's signer to chain b's signer.
async fn send_transfer<A, B>(
	chain_a: &A,
	chain_b: &B,
	channel_id: ChannelId,
	timeout: Option<Timeout>,
) -> (u128, MsgTransfer<PrefixedCoin>)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let balance = chain_a
		.query_ibc_balance()
		.await
		.expect("Can't query ibc balance")
		.pop()
		.expect("No Ibc balances");

	let amount = parse_amount(balance.amount.to_string());
	let coin = PrefixedCoin {
		denom: balance.denom,
		amount: Amount::from_str(&format!("{}", (amount * 20) / 100)).expect("Infallible"),
	};

	let (height_offset, time_offset) = if let Some(timeout) = timeout {
		match timeout {
			Timeout::Offset { timestamp, height } => (height.unwrap(), timestamp.unwrap()),
			_ => panic!("Only offset timeouts allowed"),
		}
	} else {
		// Default to 200 blocks and 1 hour offset respectively
		(200, 60 * 60)
	};

	let (mut timeout_height, timestamp) = chain_b
		.latest_height_and_timestamp()
		.await
		.expect("Couldn't fetch latest_height_and_timestamp");

	timeout_height.revision_height += height_offset;
	let timeout_timestamp =
		(timestamp + Duration::from_secs(time_offset)).expect("Overflow evaluating timeout");

	let msg = MsgTransfer {
		source_port: PortId::transfer(),
		source_channel: channel_id,
		token: coin,
		sender: chain_a.account_id(),
		receiver: chain_b.account_id(),
		timeout_height,
		timeout_timestamp,
	};
	chain_a.send_transfer(msg.clone()).await.expect("Failed to send transfer: ");
	(amount, msg)
}

async fn assert_send_transfer<A>(chain: &A, previous_balance: u128, wait_time: u64)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
{
	// wait for the acknowledgment
	let future = chain
		.ibc_events()
		.await
		.skip_while(|ev| future::ready(!matches!(ev, IbcEvent::AcknowledgePacket(_))))
		.take(1)
		.collect::<Vec<_>>();
	timeout_future(future, wait_time, format!("Didn't see AcknowledgePacket on {}", chain.name()))
		.await;

	let balance = chain
		.query_ibc_balance()
		.await
		.expect("Can't query ibc balance")
		.pop()
		.expect("No Ibc balances");

	let new_amount = parse_amount(balance.amount.to_string());
	assert!(new_amount <= (previous_balance * 80) / 100);
}

/// Send a packet using a height timeout that has already passed
/// and assert the sending chain sees the timeout packet.
async fn send_packet_and_assert_height_timeout<A, B>(
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

	let (.., msg) = send_transfer(
		chain_a,
		chain_b,
		channel_id,
		Some(Timeout::Offset { timestamp: Some(60 * 60), height: Some(20) }),
	)
	.await;

	// Wait for timeout height to elapse then resume packet relay
	let future = chain_b
		.subscribe_blocks()
		.await
		.skip_while(|block_number| {
			future::ready(*block_number < msg.timeout_height.revision_height)
		})
		.take(1)
		.collect::<Vec<_>>();

	log::info!(target: "hyperspace", "Waiting for packet timeout to elapse on counterparty");
	timeout_future(
		future,
		10 * 60,
		format!("Timeout height was not reached on {}", chain_b.name()),
	)
	.await;

	log::info!(target: "hyperspace", "Resuming send packet relay");
	set_relay_status(true);

	assert_timeout_packet(chain_a).await;
	log::info!(target: "hyperspace", "🚀🚀 Timeout packet successfully processed for height timeout");
}

/// Send a packet using a timestamp timeout that has already passed
/// and assert the sending chain sees the timeout packet.
async fn send_packet_and_assert_timestamp_timeout<A, B>(
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

	let (.., msg) = send_transfer(
		chain_a,
		chain_b,
		channel_id,
		Some(Timeout::Offset { timestamp: Some(60 * 2), height: Some(400) }),
	)
	.await;

	// Wait for timeout timestamp to elapse then resume packet relay
	let future = chain_b
		.subscribe_blocks()
		.await
		.skip_while(|block_number| {
			let block_number = *block_number;
			let chain_clone = chain_b.clone();
			async move {
				let timestamp = chain_clone.query_timestamp_at(block_number).await.unwrap();
				timestamp < msg.timeout_timestamp.nanoseconds()
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

	log::info!(target: "hyperspace", "Resuming send packet relay");
	set_relay_status(true);

	assert_timeout_packet(chain_a).await;
	log::info!(target: "hyperspace", "🚀🚀 Timeout packet successfully processed for timeout timestamp");
}

/// Simply send a packet and check that it was acknowledged after the connection delay.
async fn send_packet_with_connection_delay<A, B>(chain_a: &A, chain_b: &B, channel_id: ChannelId)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (previous_balance, ..) = send_transfer(chain_a, chain_b, channel_id, None).await;
	assert_send_transfer(chain_a, previous_balance, 20 * 60).await;
	// now send from chain b.
	let (previous_balance, ..) = send_transfer(chain_b, chain_a, channel_id, None).await;
	assert_send_transfer(chain_b, previous_balance, 20 * 60).await;
	log::info!(target: "hyperspace", "🚀🚀 Token Transfer successful with connection delay");
}

/// Close a channel
async fn send_channel_close_init_and_assert_channel_close_confirm<A, B>(
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
	let msg = MsgChannelCloseInit {
		port_id: PortId::transfer(),
		channel_id,
		signer: chain_a.account_id(),
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };

	chain_a.submit(vec![msg]).await.unwrap();

	// wait channel close confirmation on chain b
	let future = chain_b
		.ibc_events()
		.await
		.skip_while(|ev| future::ready(!matches!(ev, IbcEvent::CloseConfirmChannel(_))))
		.take(1)
		.collect::<Vec<_>>();
	timeout_future(
		future,
		10 * 60,
		format!("Didn't see CloseConfirmChannel message on {}", chain_b.name()),
	)
	.await;

	log::info!(target: "hyperspace", "🚀🚀 Channel successfully closed on both chains");
}

/// Send a packet and assert timeout on channel close
async fn send_packet_and_assert_timeout_on_channel_close<A, B>(
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

	let (.., msg_transfer) = send_transfer(
		chain_a,
		chain_b,
		channel_id,
		Some(Timeout::Offset { timestamp: Some(60 * 2), height: Some(400) }),
	)
	.await;

	let msg = MsgChannelCloseInit {
		port_id: PortId::transfer(),
		channel_id,
		signer: chain_a.account_id(),
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };

	chain_a.submit(vec![msg]).await.unwrap();

	// Wait timeout timestamp to elapse, then
	let future = chain_b
		.subscribe_blocks()
		.await
		.skip_while(|block_number| {
			let block_number = *block_number;
			let chain_clone = chain_b.clone();
			async move {
				let timestamp = chain_clone.query_timestamp_at(block_number).await.unwrap();
				timestamp < msg_transfer.timeout_timestamp.nanoseconds()
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
	log::info!(target: "hyperspace", "🚀🚀 Timeout packet successfully processed for channel close");
}

///
pub async fn ibc_messaging_packet_height_timeout_with_connection_delay<A, B>(
	chain_a: &mut A,
	chain_b: &mut B,
) where
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
		hyperspace::relay(client_a_clone, client_b_clone, None, None).await.unwrap()
	});
	send_packet_and_assert_height_timeout(chain_a, chain_b, channel_id).await;
	handle.abort()
}

///
pub async fn ibc_messaging_packet_timestamp_timeout_with_connection_delay<A, B>(
	chain_a: &mut A,
	chain_b: &mut B,
) where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (handle, channel_id, channel_b, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, Duration::from_secs(60 * 2)).await;
	// Set channel whitelist and restart relayer loop
	handle.abort();
	chain_a.set_channel_whitelist(vec![(channel_id, PortId::transfer())]);
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())]);
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace::relay(client_a_clone, client_b_clone, None, None).await.unwrap()
	});
	send_packet_and_assert_timestamp_timeout(chain_a, chain_b, channel_id).await;
	handle.abort()
}

/// Send a packet over a connection with a connection delay and assert the sending chain only sees
/// the packet after the delay has elapsed.
pub async fn ibc_messaging_with_connection_delay<A, B>(chain_a: &mut A, chain_b: &mut B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (handle, channel_id, channel_b, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, Duration::from_secs(60 * 5)).await; // 5 mins delay
	handle.abort();
	// Set channel whitelist and restart relayer loop
	chain_a.set_channel_whitelist(vec![(channel_id, PortId::transfer())]);
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())]);
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace::relay(client_a_clone, client_b_clone, None, None).await.unwrap()
	});
	send_packet_with_connection_delay(chain_a, chain_b, channel_id).await;
	handle.abort()
}

///
pub async fn ibc_channel_close<A, B>(chain_a: &mut A, chain_b: &mut B)
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
		hyperspace::relay(client_a_clone, client_b_clone, None, None).await.unwrap()
	});
	send_channel_close_init_and_assert_channel_close_confirm(chain_a, chain_b, channel_id).await;
	handle.abort()
}

///
pub async fn ibc_messaging_packet_timeout_on_channel_close<A, B>(chain_a: &mut A, chain_b: &mut B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (handle, channel_id, channel_b, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, Duration::from_secs(0)).await;
	handle.abort();
	// Set channel whitelist and restart relayer loop
	chain_a.set_channel_whitelist(vec![(channel_id, PortId::transfer())]);
	chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())]);
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace::relay(client_a_clone, client_b_clone, None, None).await.unwrap()
	});
	send_packet_and_assert_timeout_on_channel_close(chain_a, chain_b, channel_id).await;
	handle.abort()
}
