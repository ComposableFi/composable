use hyperspace::relay;
use parachain::{
	calls::{OpenChannelParams, OpenPingChannel, OpenTransferChannel},
	ParachainClient, ParachainClientConfig,
};
use primitives::{IbcProvider, KeyProvider};
use sp_keystore::{testing::KeyStore, SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{KeyTypeId, MultiSigner};

use futures::StreamExt;
use ibc::{
	applications::transfer::VERSION,
	core::{
		ics02_client::msgs::create_client::MsgCreateAnyClient,
		ics03_connection::{
			connection::Counterparty, msgs::conn_open_init::MsgConnectionOpenInit,
			version::Version as ConnVersion,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
	},
	events::IbcEvent,
	tx_msg::Msg,
};
use tokio::task::JoinHandle;

use parachain::calls::Deliver;
use std::{str::FromStr, sync::Arc, time::Duration};
use tendermint_proto::Protobuf;

pub struct TestParams {
	pub should_check: bool,
	pub confirmed_acknowledgement: bool,
	pub confirmed_receipt: bool,
}

#[derive(Debug, Clone)]
pub struct Args {
	pub chain_a: String,
	pub chain_b: String,
	pub relay_chain: String,
	pub para_id_a: u32,
	pub para_id_b: u32,
	pub connection_prefix_a: String,
	pub connection_prefix_b: String,
}

impl Default for Args {
	fn default() -> Self {
		Args {
			chain_a: "ws://127.0.0.1:9988".to_string(),
			chain_b: "ws://127.0.0.1:9188".to_string(),
			relay_chain: "ws://127.0.0.1:9944".to_string(),
			para_id_a: 2001,
			para_id_b: 2000,
			connection_prefix_a: "ibc/".to_string(),
			connection_prefix_b: "ibc/".to_string(),
		}
	}
}

#[derive(Debug, Clone)]
pub enum DefaultConfig {}

impl subxt::Config for DefaultConfig {
	type Index = u32;
	type BlockNumber = u32;
	type Hash = sp_core::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = sp_runtime::AccountId32;
	type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
	type Header = sp_runtime::generic::Header<Self::BlockNumber, sp_runtime::traits::BlakeTwo256>;
	type Signature = sp_runtime::MultiSignature;
	type Extrinsic = sp_runtime::OpaqueExtrinsic;
}

#[derive(Clone, Copy)]
pub enum ChannelToOpen {
	Ping,
	Transfer,
}

pub async fn wait_for_client_and_connection(
	args: Args,
	channel: ChannelToOpen,
) -> (
	JoinHandle<()>,
	ConnectionId,
	ChannelId,
	ParachainClient<DefaultConfig>,
	ParachainClient<DefaultConfig>,
) {
	let alice = sp_keyring::AccountKeyring::Alice;
	let alice_pub_key = MultiSigner::Sr25519(alice.public());

	let key_store: SyncCryptoStorePtr = Arc::new(KeyStore::new());
	let key_type_id = KeyTypeId::from(0u32);

	SyncCryptoStore::insert_unknown(&*key_store, key_type_id, "//Alice", &alice.public().0)
		.unwrap();

	// Create client configurations
	let config_a = ParachainClientConfig {
		name: format!("Dali ParaId({})", args.para_id_a),
		para_id: args.para_id_a,
		parachain_rpc_url: args.chain_a,
		relay_chain_rpc_url: args.relay_chain.clone(),
		client_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec(),
		public_key: alice_pub_key.clone(),
		key_store: key_store.clone(),
		ss58_version: 49,
		key_type_id,
	};

	let config_b = ParachainClientConfig {
		name: format!("Dali ParaId({})", args.para_id_b),
		para_id: args.para_id_b,
		parachain_rpc_url: args.chain_b,
		relay_chain_rpc_url: args.relay_chain,
		client_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec(),
		public_key: alice_pub_key,
		key_store,
		ss58_version: 49,
		key_type_id,
	};

	let mut chain_a = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();
	let mut chain_b = ParachainClient::<DefaultConfig>::new(config_b).await.unwrap();

	// Wait until for parachains to start producing blocks
	println!("Waiting for  block production from parachains");
	let _ = chain_a
		.para_client
		.rpc()
		.subscribe_blocks()
		.await
		.unwrap()
		.take(2)
		.collect::<Vec<_>>()
		.await;
	println!("Parachains have started block production");

	{
		// Get initial beefy state
		let (client_state, consensus_state) =
			chain_b.construct_beefy_client_state(0).await.unwrap();

		// Create client message is the same for both chains
		let msg_create_client = MsgCreateAnyClient {
			client_state: client_state.clone(),
			consensus_state,
			signer: chain_a.account_id(),
		};

		let msg = pallet_ibc::Any {
			type_url: msg_create_client.type_url().as_bytes().to_vec(),
			value: msg_create_client.encode_vec(),
		};
		let client_id_b_on_a = chain_a
			.submit_create_client_msg(msg.clone())
			.await
			.expect("Client was not created successfully");
		chain_b.set_client_id(client_id_b_on_a.clone());
	};

	{
		// Get initial beefy state
		let (client_state, consensus_state) =
			chain_a.construct_beefy_client_state(0).await.unwrap();

		// Create client message is the same for both chains
		let msg_create_client = MsgCreateAnyClient {
			client_state: client_state.clone(),
			consensus_state,
			signer: chain_a.account_id(),
		};

		let msg = pallet_ibc::Any {
			type_url: msg_create_client.type_url().as_bytes().to_vec(),
			value: msg_create_client.encode_vec(),
		};
		let client_id_a_on_b = chain_b
			.submit_create_client_msg(msg)
			.await
			.expect("Client was not created successfully");
		chain_a.set_client_id(client_id_a_on_b.clone());
	};

	// Start relayer loop
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();

	let handle =
		tokio::task::spawn(async move { relay(client_a_clone, client_b_clone).await.unwrap() });

	let connection_id = ConnectionId::default();
	let channel_id = ChannelId::default();
	let port_id = match channel {
		ChannelToOpen::Ping => PortId::from_str("ping").unwrap(),
		ChannelToOpen::Transfer => PortId::transfer(),
	};

	// Both clients have been updated, we can now start connection handshake
	let msg = MsgConnectionOpenInit {
		client_id: chain_a.client_id(),
		counterparty: Counterparty::new(chain_b.client_id(), None, chain_b.connection_prefix()),
		version: Some(ConnVersion::default()),
		delay_period: Duration::from_nanos(0),
		signer: chain_a.account_id(),
	};

	let msg =
		pallet_ibc::Any { type_url: msg.type_url().as_bytes().to_vec(), value: msg.encode_vec() };

	chain_a
		.submit_call(Deliver { messages: vec![msg] })
		.await
		.expect("Connection creation failed");

	// wait till both chains have completed connection handshake
	let mut chain_b_events = chain_b.ibc_events().await;
	println!("wait till both chains have completed connection handshake");

	while let Some(ev) = chain_b_events.next().await {
		// connection handshake completed.
		if matches!(ev, IbcEvent::OpenConfirmConnection(_)) {
			break
		}
	}

	println!("Connection handshake completed, starting channel handshake");

	match channel {
		ChannelToOpen::Ping => {
			let params = OpenChannelParams {
				order: 1,
				connection_id: connection_id.as_bytes().to_vec(),
				counterparty_port_id: port_id.as_bytes().to_vec(),
				version: b"ibc-ping".to_vec(),
			};
			chain_a.submit_sudo_call(OpenPingChannel { params }).await.unwrap();
		},
		ChannelToOpen::Transfer => {
			let params = OpenChannelParams {
				order: 1,
				connection_id: connection_id.as_bytes().to_vec(),
				counterparty_port_id: port_id.as_bytes().to_vec(),
				version: VERSION.as_bytes().to_vec(),
			};
			chain_a.submit_sudo_call(OpenTransferChannel { params }).await.unwrap();
		},
	};

	// wait till both chains have completed channel handshake
	println!("wait till both chains have completed channel handshake");
	while let Some(ev) = chain_b_events.next().await {
		// channel handshake completed
		if matches!(ev, IbcEvent::OpenConfirmChannel(_)) {
			break
		}
	}

	println!("Channel handshake completed");

	(handle, connection_id, channel_id, chain_a, chain_b)
}
