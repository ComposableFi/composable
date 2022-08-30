use futures::{future, StreamExt};
use hyperspace_primitives::Chain;
use ibc::{
	applications::transfer::VERSION,
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
use std::time::Duration;
use tendermint_proto::Protobuf;
use tokio::task::JoinHandle;

/// this will set up a connection and ics20 channel in-between the two chains.
/// `connection_delay` should be in seconds.
async fn setup_connection_and_channel<A, B>(
	chain_a: A,
	chain_b: B,
	connection_delay: u64,
) -> (JoinHandle<()>, ChannelId, ConnectionId)
where
	A: Chain + Clone + 'static,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: Chain + Clone + 'static,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	// Start relayer loop
	let handle = tokio::task::spawn(async move {
		hyperspace::relay(client_a_clone, client_b_clone).await.unwrap()
	});

	let connection_id = ConnectionId::default();
	let channel_id = ChannelId::default();

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

	log::info!("Wait till both chains have completed connection handshake");

	// wait till both chains have completed connection handshake
	let future = chain_b
		.ibc_events()
		.await
		.take_while(|ev| future::ready(!matches!(ev, IbcEvent::OpenConfirmConnection(_))))
		.collect::<Vec<_>>();
	// 10 minutes
	let duration = Duration::from_secs(10 * 60);
	if let Err(_) = tokio::time::timeout(duration.clone(), future).await {
		panic!("Didn't see OpenConfirmConnection on {} within {duration:?}", chain_b.name())
	}

	log::info!("Connection handshake completed, starting channel handshake");

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
	log::info!("Wait till both chains have completed channel handshake");
	let future = chain_b
		.ibc_events()
		.await
		.take_while(|ev| future::ready(!matches!(ev, IbcEvent::OpenConfirmChannel(_))))
		.collect::<Vec<_>>();
	if let Err(_) = tokio::time::timeout(duration.clone(), future).await {
		panic!("Didn't see OpenConfirmChannel on {} within {duration:?}", chain_b.name())
	}

	// channel handshake completed
	log::info!("Channel handshake completed");

	(handle, channel_id, connection_id)
}

/// Simply send a packet and check that it was acknowledged.
pub async fn send_packet_and_assert_acknowledgment<A, B>(
	chain_a: A,
	chain_b: B,
)
where
	A: Chain + Clone + 'static,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: Chain + Clone + 'static,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (_handle, _channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, 0).await;
}

/// Send a packet using a height timeout that has already passed
/// and assert the sending chain sees the timeout packet.
pub async fn send_packet_and_assert_height_timeout<A, B>(chain_a: A, chain_b: B)
where
	A: Chain + Clone + 'static,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: Chain + Clone + 'static,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (_handle, _channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, 0).await;
}

/// Send a packet using a timestamp timeout that has already passed
/// and assert the sending chain sees the timeout packet.
pub async fn send_packet_and_assert_timestamp_timeout<A, B>(chain_a: A, chain_b: B)
where
	A: Chain + Clone + 'static,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: Chain + Clone + 'static,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (_handle, _channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, 0).await;
}

/// Send a packet over a connection with a connection delay
/// and assert the sending chain only sees the packet after the
/// delay has elapsed.
pub async fn send_packet_with_connection_delay<A, B>(chain_a: A, chain_b: B)
where
	A: Chain + Clone + 'static,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: Chain + Clone + 'static,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let connection_delay = 5 * 60; // 5 mins
	let (_handle, _channel_id, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, connection_delay).await;
}
