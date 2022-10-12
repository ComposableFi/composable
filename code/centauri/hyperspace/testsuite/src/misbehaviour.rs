use crate::{send_packet_and_assert_height_timeout, setup_connection_and_channel, StreamExt};
use finality_grandpa::{Precommit, SignedPrecommit};
use grandpa_client_primitives::{justification::GrandpaJustification, Commit, FinalityProof};
use hyperspace_primitives::{mock::LocalClientTypes, TestProvider};
use ibc::{
	core::{ics02_client::msgs::update_client::MsgUpdateAnyClient, ics24_host::identifier::PortId},
	tx_msg::Msg,
};
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::MsgUpdateClient};
use ics10_grandpa::client_message::{ClientMessage, Header as GrandpaHeader, RelayChainHeader};
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState};
use polkadot_core_primitives::Header;
use sp_core::{Decode, Encode, Pair, H256};
use sp_finality_grandpa::{AuthorityId, AuthorityList, AuthoritySignature};
use sp_keyring::ed25519::Keyring;
use sp_keystore::CryptoStore;
use std::time::Duration;
use tendermint_proto::Protobuf;

pub async fn ibc_messaging_submit_misbehaviour<A, B>(chain_a: &mut A, chain_b: &mut B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
	// FinalityProof<RelayChainHeader>: From<B::FinalityEvent>,
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
		hyperspace::fish(client_a_clone, client_b_clone).await.unwrap()
	});

	log::info!("Waiting for the next block...");

	let target_number = chain_b.subscribe_relaychain_blocks().await.next().await.expect("no block");
	// let target_hash = H256::from_low_u64_be(target_number as u64);
	// let precommit = Precommit { target_hash, target_number };
	// let round = 1;
	let polkadot_authorities = [
		Keyring::Alice,
		Keyring::Bob,
		Keyring::Charlie,
		Keyring::Dave,
		Keyring::Eve,
		Keyring::Ferdie,
	];
	// let message = finality_grandpa::Message::Precommit(precommit.clone());
	// let precommits = polkadot_authorities
	// 	.iter()
	// 	.map(|id| {
	// 		let key = id.pair();
	// 		let encoded = sp_finality_grandpa::localized_payload(round, set_id, &message);
	// 		SignedPrecommit {
	// 			precommit: precommit.clone(),
	// 			signature: AuthoritySignature::from(key.sign(&encoded)),
	// 			id: AuthorityId::from(key.public()),
	// 		}
	// 	})
	// 	.collect();
	// let justification = GrandpaJustification::<RelayChainHeader> {
	// 	round,
	// 	commit: Commit::<RelayChainHeader> { target_hash, target_number, precommits },
	// 	votes_ancestries: vec![],
	// };
	// let finality_proof = FinalityProof {
	// 	block: target_hash,
	// 	justification: justification.encode(),
	// 	unknown_headers: vec![],
	// };
	// let header = GrandpaHeader { finality_proof, parachain_headers: Default::default() };
	// let signer = chain_a.account_id();
	// let client_msg = AnyClientMessage::Grandpa(ClientMessage::Header(header));

	let client_id = chain_a.client_id();
	let latest_height = chain_b.latest_height_and_timestamp().await.unwrap().0;
	let response = chain_b.query_client_state(latest_height, client_id).await.unwrap();
	let client_state = match AnyClientState::try_from(response.client_state.unwrap()).unwrap() {
		AnyClientState::Grandpa(cs) => cs,
		_ => panic!("unexpected client state"),
	};
	log::info!("Latest client height: {}", client_state.latest_relay_height);
	let mut finality_event = chain_b.finality_notifications().await.next().await.expect("no event");
	let set_id = chain_b.current_set_id().await;

	let (update_client_msg, _, _) = chain_b
		.query_latest_ibc_events(finality_event, chain_a)
		.await
		.expect("no event");
	let msg =
		MsgUpdateAnyClient::<LocalClientTypes>::decode(&mut update_client_msg.value.as_slice())
			.unwrap();
	match &msg.client_message {
		AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
			let justification = GrandpaJustification::<RelayChainHeader>::decode(
				&mut &*header.finality_proof.justification,
			)
			.unwrap();
			justification.commit.precommits.iter().enumerate().for_each(|(i, precommit)| {
				if i == 0 {
					let message = finality_grandpa::Message::Precommit(precommit.precommit.clone());
					polkadot_authorities.iter().for_each(|id| {
						let key = id.pair();
						let encoded = sp_finality_grandpa::localized_payload(
							justification.round,
							set_id,
							&message,
						);
						let sig = AuthoritySignature::from(key.sign(&encoded));
						log::info!("my id = {:?}", key.public());
						log::info!("my sig = {:?}", sig);
					});
				}
				log::info!("their id = {:?}", precommit.id);
				log::info!("their sig = {:?}", precommit.signature);
			});
		},
		_ => panic!("unexpected client message"),
	}
	// let msg = MsgUpdateAnyClient::<LocalClientTypes>::new(chain_b.client_id(), client_msg,
	// signer);
	chain_a
		.submit(vec![Any { value: msg.encode_vec(), type_url: msg.type_url() }])
		.await
		.expect("failed to submit message");

	log::info!("Message submitted");

	let client_id = chain_a.client_id();
	let latest_height = chain_b.latest_height_and_timestamp().await.unwrap().0;
	let response = chain_b.query_client_state(latest_height, client_id).await.unwrap();
	let client_state = match AnyClientState::try_from(response.client_state.unwrap()).unwrap() {
		AnyClientState::Grandpa(cs) => cs,
		_ => panic!("unexpected client state"),
	};
	log::info!("Newest client height: {}", client_state.latest_relay_height);

	// send_packet_and_assert_height_timeout(chain_a, chain_b, channel_id).await;
	handle.abort()
}
