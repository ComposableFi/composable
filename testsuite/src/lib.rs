use crate::utils::{parse_amount, timeout_future};
use futures::{future, StreamExt};
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
			msgs::chan_open_init::MsgChannelOpenInit,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
	},
	events::IbcEvent,
	tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use std::{str::FromStr, time::Duration};
use tendermint_proto::Protobuf;
use tokio::task::JoinHandle;

mod utils;

/// This will set up a connection and ics20 channel in-between the two chains.
/// `connection_delay` should be in seconds.
async fn setup_connection_and_channel<A, B>(
	chain_a: &A,
	chain_b: &B,
	connection_delay: u64,
) -> (JoinHandle<()>, ChannelId, ConnectionId)
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
		hyperspace::relay(client_a_clone, client_b_clone).await.unwrap()
	});

	// Both clients have been updated, we can now start connection handshake
	let msg = MsgConnectionOpenInit {
		client_id: chain_a.client_id(),
		counterparty: Counterparty::new(chain_b.client_id(), None, chain_b.connection_prefix()),
		version: Some(ics03_connection::version::Version::default()),
		delay_period: Duration::from_secs(connection_delay),
		signer: chain_a.account_id(),
	};
	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };
	chain_a
		.submit_ibc_messages(vec![msg])
		.await
		.expect("Connection creation failed");

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

	chain_a
		.submit_ibc_messages(vec![msg])
		.await
		.expect("Connection creation failed");

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

	let channel_id = match events.pop() {
		Some(IbcEvent::OpenConfirmChannel(chan)) => chan.channel_id().unwrap().clone(),
		got => panic!("Last event should be OpenConfirmConnection: {got:?}"),
	};

	// channel handshake completed
	log::info!(target: "hyperspace", "============ Channel handshake completed: ChannelId({channel_id}) ============");

	(handle, channel_id, connection_id)
}

/// Attempts to send 70% of funds of chain_a's signer to chain b's signer.
async fn send_transfer<A, B>(chain_a: &A, chain_b: &B, channel_id: ChannelId)
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
		amount: Amount::from_str(&format!("{}", (amount * 70) / 100)).expect("Infallible"),
	};

	let (mut timeout_height, timestamp) = chain_b
		.latest_height_and_timestamp()
		.await
		.expect("Couldn't fetch latest_height_and_timestamp");

	// 50 blocks
	timeout_height.revision_height += 50;
	let timeout_timestamp = (timestamp + Duration::from_secs(60 * 60)).expect("Overflow evaluating timeout");

	let msg = MsgTransfer {
		source_port: PortId::transfer(),
		source_channel: channel_id,
		token: coin,
		sender: chain_a.account_id(),
		receiver: chain_b.account_id(),
		timeout_height,
		timeout_timestamp,
	};
	chain_a.send_transfer(msg).await.expect("Failed to send transfer: ");

	// wait for the acknowledgment
	let future = chain_a
		.ibc_events()
		.await
		.skip_while(|ev| future::ready(!matches!(ev, IbcEvent::AcknowledgePacket(_))))
		.take(1)
		.collect::<Vec<_>>();
	timeout_future(future, 10 * 60, format!("Didn't see AcknowledgePacket on {}", chain_a.name()))
		.await;

	let balance = chain_a
		.query_ibc_balance()
		.await
		.expect("Can't query ibc balance")
		.pop()
		.expect("No Ibc balances");

	let new_amount = parse_amount(balance.amount.to_string());
	assert!(new_amount <= (amount * 30) / 100);
}

/// Simply send a packet and check that it was acknowledged.
pub async fn send_packet_and_assert_acknowledgment<A, B>(chain_a: &A, chain_b: &B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (_handle, channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, 0).await;

	send_transfer(chain_a, chain_b, channel_id).await;
	// now send from chain b.
	send_transfer(chain_b, chain_a, channel_id).await;
}

/// Send a packet using a height timeout that has already passed
/// and assert the sending chain sees the timeout packet.
pub async fn send_packet_and_assert_height_timeout<A, B>(chain_a: &A, chain_b: &B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (_handle, _channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, 0).await;
}

/// Send a packet using a timestamp timeout that has already passed
/// and assert the sending chain sees the timeout packet.
pub async fn send_packet_and_assert_timestamp_timeout<A, B>(chain_a: &A, chain_b: &B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (_handle, _channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, 0).await;
}

/// Send a packet over a connection with a connection delay
/// and assert the sending chain only sees the packet after the
/// delay has elapsed.
pub async fn send_packet_with_connection_delay<A, B>(chain_a: &A, chain_b: &B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let connection_delay = 5 * 60; // 5 mins
	let (_handle, _channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, connection_delay).await;
}
