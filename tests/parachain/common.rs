use hyperspace::relay;
use ibc_proto::google::protobuf::Any;
use parachain::{
	calls::{DeliverPermissioned, OpenChannelParams, OpenPingChannel, OpenTransferChannel},
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

use std::{
	str::FromStr,
	sync::Arc,
	time::{Duration, Instant},
};
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

	let mut client_a = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();
	let mut client_b = ParachainClient::<DefaultConfig>::new(config_b).await.unwrap();

	// Wait until for parachains to start producing blocks
	let block_subscription = client_a.para_client.rpc().subscribe_blocks().await.unwrap();
	println!("Waiting for  block production from parachains");
	let _ = block_subscription.take(2).collect::<Vec<_>>().await;
	println!("Parachains have started block production");

	let client_id_b_on_a = {
		// Get initial beefy state
		let (client_state, consensus_state) =
			client_b.construct_beefy_client_state(0).await.unwrap();

		// Create client message is the same for both chains
		let msg_create_client = MsgCreateAnyClient {
			client_state: client_state.clone(),
			consensus_state,
			signer: client_a.account_id(),
		};

		let msg =
			Any { type_url: msg_create_client.type_url(), value: msg_create_client.encode_vec() };
		client_a
			.submit_create_client_msg(msg.clone())
			.await
			.expect("Client was not created successfully")
	};
	let client_id_a_on_b = {
		// Get initial beefy state
		let (client_state, consensus_state) =
			client_a.construct_beefy_client_state(0).await.unwrap();

		// Create client message is the same for both chains
		let msg_create_client = MsgCreateAnyClient {
			client_state: client_state.clone(),
			consensus_state,
			signer: client_a.account_id(),
		};

		let msg =
			Any { type_url: msg_create_client.type_url(), value: msg_create_client.encode_vec() };
		client_b
			.submit_create_client_msg(msg)
			.await
			.expect("Client was not created successfully")
	};

	client_a.set_client_id(client_id_a_on_b.clone());

	client_b.set_client_id(client_id_b_on_a.clone());

	// Start relayer loop

	let client_a_clone = client_a.clone();

	let client_b_clone = client_b.clone();
	let handle =
		tokio::task::spawn(async move { relay(client_a_clone, client_b_clone).await.unwrap() });

	// Create a hot loop to monitor chain state and send transactions after connection handshake is
	// done
	let mut conn_state_a = false;
	let mut conn_state_b = false;
	let mut chan_state_a = false;
	let mut chan_state_b = false;
	let mut connection_id = ConnectionId::default();
	let mut channel_id = ChannelId::default();
	let port_id = match channel {
		ChannelToOpen::Ping => PortId::from_str("ping").unwrap(),
		ChannelToOpen::Transfer => PortId::transfer(),
	};
	let mut initialized_channel = false;
	let mut initialized_connection = false;

	let (mut chain_a_events, mut chain_b_events) =
		(client_a.ibc_events().await, client_b.ibc_events().await);

	let start = Instant::now();
	loop {
		// If channel states are open on both chains we can stop listening for events
		if chan_state_a && chan_state_b {
			println!("Channel handshake completed");
			break
		}

		let time_elapsed = Instant::now().duration_since(start);
		if time_elapsed >= Duration::from_secs(1200) {
			println!("Could not verify connection and channel handshake after waiting 20mins");
			break
		}

		// Both clients have been updated, we can now start connection handshake
		if !initialized_connection {
			let msg = MsgConnectionOpenInit {
				client_id: client_a.client_id(),
				counterparty: Counterparty::new(
					client_b.client_id(),
					None,
					client_b.connection_prefix(),
				),
				version: Some(ConnVersion::default()),
				delay_period: Duration::from_nanos(0),
				signer: client_a.account_id(),
			};

			let msg = pallet_ibc::Any {
				type_url: msg.type_url().as_bytes().to_vec(),
				value: msg.encode_vec(),
			};

			client_a
				.submit_sudo_call(DeliverPermissioned { messages: vec![msg] })
				.await
				.expect("Connection creation failed");
			initialized_connection = true;
		}

		if conn_state_a && conn_state_b {
			if !initialized_channel {
				println!("Connection handshake completed, starting channel handshake");
				match channel {
					ChannelToOpen::Ping => {
						let params = OpenChannelParams {
							order: 1,
							connection_id: connection_id.as_bytes().to_vec(),
							counterparty_port_id: port_id.as_bytes().to_vec(),
							version: b"ibc-ping".to_vec(),
						};
						client_a.submit_sudo_call(OpenPingChannel { params }).await.unwrap();
					},
					ChannelToOpen::Transfer => {
						let params = OpenChannelParams {
							order: 1,
							connection_id: connection_id.as_bytes().to_vec(),
							counterparty_port_id: port_id.as_bytes().to_vec(),
							version: VERSION.as_bytes().to_vec(),
						};
						client_a.submit_sudo_call(OpenTransferChannel { params }).await.unwrap();
					},
				}

				initialized_channel = true;
			}
		}
		tokio::select! {
			result = chain_a_events.next() => {
				match result {
					None => {
						println!("Event Stream from chain A ended");
						break
					},
					Some(Ok(events)) => {
						for event in events {
							match event {
								IbcEvent::OpenAckConnection(open_ack) if !conn_state_a => {
									connection_id = open_ack.connection_id().unwrap().clone();
									conn_state_a = true;
									break
								},
								IbcEvent::OpenAckChannel(open_ack) if !chan_state_a => {
									channel_id = open_ack.channel_id.unwrap();
									chan_state_a = true;
									break
								}
								_ => continue
							}
						}
					}
					Some(Err(err)) => {
						println!("[wait_for_client_and_connection] Received Error from stream A {:?}", err);
					}
				}
			}
			result = chain_b_events.next() => {
				match result {
					None => {
						println!("Event Stream from chain B ended");
						break
					},
					Some(Ok(events)) => {
						for event in events {
							match event {
								IbcEvent::OpenConfirmConnection(_) if !conn_state_b=> {
									conn_state_b = true;
									break
								},
								IbcEvent::OpenConfirmChannel(_) if !chan_state_b => {
									chan_state_b = true;
									break
								}
								_ => continue
							}
						}
					}
					Some(Err(err)) => {
						println!("[wait_for_client_and_connection] Error from stream B {:?}", err);
					}
				}
			}
		}
	}
	(handle, connection_id, channel_id, client_a, client_b)
}
