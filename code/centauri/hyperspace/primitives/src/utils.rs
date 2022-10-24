use crate::{mock::LocalClientTypes, Chain};
use futures::{future, StreamExt};
use ibc::{
	core::{
		ics02_client::msgs::create_client::MsgCreateAnyClient,
		ics03_connection::{connection::Counterparty, msgs::conn_open_init::MsgConnectionOpenInit},
		ics04_channel,
		ics04_channel::{
			channel,
			channel::{ChannelEnd, Order, State},
			msgs::chan_open_init::MsgChannelOpenInit,
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	protobuf::Protobuf,
	tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use std::{future::Future, time::Duration};

pub async fn timeout_future<T: Future>(future: T, secs: u64, reason: String) -> T::Output {
	let duration = Duration::from_secs(secs);
	match tokio::time::timeout(duration.clone(), future).await {
		Ok(output) => output,
		Err(_) => panic!("Future didn't finish within {duration:?}, {reason}"),
	}
}

pub async fn create_clients(
	chain_a: &impl Chain,
	chain_b: &impl Chain,
) -> Result<(ClientId, ClientId), anyhow::Error> {
	let (client_state_a, cs_state_a) = chain_a.initialize_client_state().await?;
	let (client_state_b, cs_state_b) = chain_b.initialize_client_state().await?;

	let msg = MsgCreateAnyClient::<LocalClientTypes> {
		client_state: client_state_b,
		consensus_state: cs_state_b,
		signer: chain_a.account_id(),
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };

	let (tx_hash, block_hash) = chain_a.submit(vec![msg]).await?;
	let client_id_b_on_a = chain_a.query_client_id_from_tx_hash(tx_hash, block_hash).await?;

	let msg = MsgCreateAnyClient::<LocalClientTypes> {
		client_state: client_state_a,
		consensus_state: cs_state_a,
		signer: chain_b.account_id(),
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };

	let (tx_hash, block_hash) = chain_b.submit(vec![msg]).await?;
	let client_id_a_on_b = chain_b.query_client_id_from_tx_hash(tx_hash, block_hash).await?;

	Ok((client_id_a_on_b, client_id_b_on_a))
}

/// Completes the connection handshake process
/// The relayer process must be running before this function is executed
pub async fn create_connection(
	chain_a: &impl Chain,
	chain_b: &impl Chain,
	delay_period: Duration,
) -> Result<(ConnectionId, ConnectionId), anyhow::Error> {
	let msg = MsgConnectionOpenInit {
		client_id: chain_a.client_id(),
		counterparty: Counterparty::new(chain_b.client_id(), None, chain_b.connection_prefix()),
		version: Some(Default::default()),
		delay_period,
		signer: chain_a.account_id(),
	};

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };

	chain_a.submit(vec![msg]).await?;

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

	let (connection_id_b, connection_id_a) = match events.pop() {
		Some(IbcEvent::OpenConfirmConnection(conn)) => (
			conn.connection_id().unwrap().clone(),
			conn.attributes().counterparty_connection_id.as_ref().unwrap().clone(),
		),
		got => panic!("Last event should be OpenConfirmConnection: {got:?}"),
	};

	Ok((connection_id_a, connection_id_b))
}

/// Completes the chanel handshake process
/// The relayer process must be running before this function is executed
pub async fn create_channel(
	chain_a: &impl Chain,
	chain_b: &impl Chain,
	connection_id: ConnectionId,
	port_id: PortId,
	version: String,
	order: Order,
) -> Result<(ChannelId, ChannelId), anyhow::Error> {
	let channel = ChannelEnd::new(
		State::Init,
		order,
		channel::Counterparty::new(port_id.clone(), None),
		vec![connection_id],
		ics04_channel::Version::new(version),
	);

	let msg = MsgChannelOpenInit::new(port_id, channel, chain_a.account_id());

	let msg = Any { type_url: msg.type_url(), value: msg.encode_vec() };

	chain_a.submit(vec![msg]).await?;

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

	let (channel_id_a, channel_id_b) = match events.pop() {
		Some(IbcEvent::OpenConfirmChannel(chan)) =>
			(chan.counterparty_channel_id.unwrap(), chan.channel_id().unwrap().clone()),
		got => panic!("Last event should be OpenConfirmChannel: {got:?}"),
	};

	Ok((channel_id_a, channel_id_b))
}
