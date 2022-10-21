use futures::StreamExt;
use hyperspace::logging;
use hyperspace_primitives::{mock::LocalClientTypes, IbcProvider, KeyProvider};
use hyperspace_testsuite::{
	ibc_channel_close, ibc_messaging_packet_height_timeout_with_connection_delay,
	ibc_messaging_packet_timeout_on_channel_close,
	ibc_messaging_packet_timestamp_timeout_with_connection_delay,
	ibc_messaging_with_connection_delay,
};
use ibc::{core::ics02_client::msgs::create_client::MsgCreateAnyClient, tx_msg::Msg};
use parachain::{finality_protocol::FinalityProtocol, ParachainClient, ParachainClientConfig};
use subxt::tx::SubstrateExtrinsicParams;

use tendermint_proto::Protobuf;

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
	type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

async fn setup_clients() -> (ParachainClient<DefaultConfig>, ParachainClient<DefaultConfig>) {
	log::info!(target: "hyperspace", "=========================== Starting Test ===========================");
	let args = Args::default();

	// Create client configurations
	let config_a = ParachainClientConfig {
		name: format!("127.0.0.1:9988"),
		para_id: args.para_id_a,
		parachain_rpc_url: args.chain_a,
		relay_chain_rpc_url: args.relay_chain.clone(),
		client_id: None,
		beefy_activation_block: None,
		connection_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec().into(),
		ss58_version: 49,
		channel_whitelist: vec![],
		finality_protocol: FinalityProtocol::Grandpa,
		private_key: "//Alice".to_string(),
		key_type: "sr25519".to_string(),
	};
	let config_b = ParachainClientConfig {
		name: format!("127.0.0.1:9188"),
		para_id: args.para_id_b,
		parachain_rpc_url: args.chain_b,
		relay_chain_rpc_url: args.relay_chain,
		client_id: None,
		beefy_activation_block: None,
		connection_id: None,
		commitment_prefix: args.connection_prefix_b.as_bytes().to_vec().into(),
		private_key: "//Alice".to_string(),
		ss58_version: 49,
		channel_whitelist: vec![],
		finality_protocol: FinalityProtocol::Grandpa,
		key_type: "sr25519".to_string(),
	};

	let mut chain_a = ParachainClient::<DefaultConfig>::new(config_a).await.unwrap();
	let mut chain_b = ParachainClient::<DefaultConfig>::new(config_b).await.unwrap();

	// Wait until for parachains to start producing blocks
	log::info!(target: "hyperspace", "Waiting for  block production from parachains");
	let _ = chain_a
		.relay_client
		.rpc()
		.subscribe_blocks()
		.await
		.unwrap()
		.filter_map(|result| futures::future::ready(result.ok()))
		.skip_while(|h| futures::future::ready(h.number < 90))
		.take(1)
		.collect::<Vec<_>>()
		.await;
	log::info!(target: "hyperspace", "Parachains have started block production");

	let clients_on_a = chain_a.query_clients().await.unwrap();
	let clients_on_b = chain_b.query_clients().await.unwrap();

	if !clients_on_a.is_empty() && !clients_on_b.is_empty() {
		chain_a.set_client_id(clients_on_b[0].clone());
		chain_b.set_client_id(clients_on_b[0].clone());
		return (chain_a, chain_b)
	}

	let (res_1, res_2) = futures::join!(
		chain_a.set_pallet_params(true, true),
		chain_b.set_pallet_params(true, true)
	);
	res_1.unwrap();
	res_2.unwrap();

	{
		// Get initial beefy state
		let (client_state, consensus_state) =
			chain_b.construct_grandpa_client_state().await.unwrap();

		// Create client message is the same for both chains
		let msg_create_client = MsgCreateAnyClient::<LocalClientTypes> {
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
			chain_a.construct_grandpa_client_state().await.unwrap();

		// Create client message is the same for both chains
		let msg_create_client = MsgCreateAnyClient::<LocalClientTypes> {
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

	(chain_a, chain_b)
}

#[tokio::test]
async fn parachain_to_parachain_ibc_messaging_full_integration_test() {
	logging::setup_logging();
	let (mut chain_a, mut chain_b) = setup_clients().await;
	// Run tests sequentially

	// no timeouts + connection delay
	ibc_messaging_with_connection_delay(&mut chain_a, &mut chain_b).await;

	// timeouts + connection delay
	ibc_messaging_packet_height_timeout_with_connection_delay(&mut chain_a, &mut chain_b).await;
	ibc_messaging_packet_timestamp_timeout_with_connection_delay(&mut chain_a, &mut chain_b).await;

	// channel closing semantics
	ibc_messaging_packet_timeout_on_channel_close(&mut chain_a, &mut chain_b).await;
	ibc_channel_close(&mut chain_a, &mut chain_b).await;
}
