use crate::{send_packet_and_assert_height_timeout, setup_connection_and_channel, StreamExt};
use finality_grandpa::{Chain, Precommit, SignedPrecommit};
use grandpa_client_primitives::{
	justification::{AncestryChain, GrandpaJustification},
	parachain_header_storage_key, Commit, FinalityProof, ParachainHeaderProofs,
	ParachainHeadersWithFinalityProof,
};
use grandpa_light_client::verify_parachain_headers_with_grandpa_finality_proof;
use hyperspace_primitives::{mock::LocalClientTypes, TestProvider};
use ibc::{
	core::{ics02_client::msgs::update_client::MsgUpdateAnyClient, ics24_host::identifier::PortId},
	events::IbcEvent,
	tx_msg::Msg,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::{core::client::v1::MsgUpdateClient, lightclients::beefy::v1::ParachainHeader},
};
use ics10_grandpa::client_message::{ClientMessage, Header as GrandpaHeader, RelayChainHeader};
use light_client_common::state_machine;
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState, HostFunctionsManager};
use polkadot_core_primitives::Header;
use sp_core::{traits, Decode, Encode, Pair, H256};
use sp_finality_grandpa::{AuthorityId, AuthorityList, AuthoritySignature};
use sp_keyring::ed25519::Keyring;
use sp_keystore::CryptoStore;
use sp_runtime::{
	codec,
	codec::Compact,
	traits::{BlakeTwo256, Header as _},
};
use sp_state_machine::{prove_read_on_trie_backend, ProvingBackend, TrieBackend};
use sp_trie::{
	generate_trie_proof, LayoutV0, MemoryDB, PrefixedMemoryDB, StorageProof, TrieDB, TrieDBMut,
	TrieMut,
};
use std::{collections::BTreeMap, convert::identity, sync::Arc, time::Duration};
use tendermint_proto::Protobuf;
use tokio::time::{sleep, timeout};

/// Submits a misbehaviour transaction on
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

	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace::fish(client_a_clone, client_b_clone).await.unwrap()
	});
	// Set channel whitelist and restart relayer loop
	// chain_a.set_channel_whitelist(vec![(channel_id, PortId::transfer())]);
	// chain_b.set_channel_whitelist(vec![(channel_b, PortId::transfer())]);
	log::info!("Waiting for the next block...");

	// let target_number = chain_b.subscribe_relaychain_blocks().await.next().await.expect("no
	// block");

	// finality proofs are signed by relay-chain authorities. We're using everyone except Alice
	// who will be submitting the misbehaviour
	let relaychain_authorities = [
		// Keyring::Alice,
		Keyring::Bob,
		Keyring::Charlie,
		Keyring::Dave,
		Keyring::Eve,
		Keyring::Ferdie,
	];

	// query the current client state that will be used to construct a fraudulent finality proof
	let client_id = chain_b.client_id();
	let latest_height = chain_a.latest_height_and_timestamp().await.unwrap().0;
	let response = chain_a.query_client_state(latest_height, client_id).await.unwrap();
	let client_state = match AnyClientState::try_from(response.client_state.unwrap()).unwrap() {
		AnyClientState::Grandpa(cs) => cs,
		_ => panic!("unexpected client state"),
	};
	log::info!("Latest client height: {}", client_state.latest_relay_height);

	let mut finality_event = chain_b.finality_notifications().await.next().await.expect("no event");
	let set_id = client_state.current_set_id;

	// construct an extrinsic proof with the mandatory timestamp extrinsic
	let mut para_db = MemoryDB::<BlakeTwo256>::default();

	// TODO: how to construct timestamp extrinsic via metadata?
	let mut timestamp_extrinsic = (1u8, 0u8, Compact(1u64)).encode();
	timestamp_extrinsic.insert(0, 0);
	timestamp_extrinsic.insert(0, 0);
	let key = Compact(0u32).encode();
	let extrinsics_root = {
		let mut root = Default::default();
		let mut trie = <TrieDBMut<LayoutV0<BlakeTwo256>>>::new(&mut para_db, &mut root);
		trie.insert(&key, &timestamp_extrinsic).unwrap();
		*trie.root()
	};
	let extrinsic_proof = generate_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
		&para_db,
		extrinsics_root,
		vec![&key],
	)
	.unwrap();

	// construct a state root from the parachain header
	let parachain_header = Header {
		parent_hash: Default::default(),
		number: client_state.latest_para_height + 1,
		state_root: Default::default(),
		extrinsics_root,
		digest: Default::default(),
	};
	let key = parachain_header_storage_key(client_state.para_id);
	let mut db = PrefixedMemoryDB::<BlakeTwo256>::default();
	let mut root = Default::default();
	let state_root = {
		let mut trie = TrieDBMut::<LayoutV0<BlakeTwo256>>::new(&mut para_db, &mut root);
		trie.insert(key.as_ref(), &parachain_header.encode().encode()).unwrap();
		*trie.root()
	};

	// build a chain of relaychain blocks
	let mut prev_hash = client_state.latest_relay_hash;
	let mut headers = vec![];
	for i in 0..3 {
		let header = RelayChainHeader {
			parent_hash: prev_hash,
			number: client_state.latest_relay_height + 1 + i,
			state_root,
			extrinsics_root: Default::default(),
			digest: Default::default(),
		};
		prev_hash = header.hash();
		headers.push(header);
	}
	let header = headers.last().unwrap().clone();
	let header_hash = header.hash();
	let precommit = Precommit { target_hash: header_hash, target_number: header.number };
	let message = finality_grandpa::Message::Precommit(precommit.clone());
	let round = 1;
	// sign pre-commits by the authorities to vote for the highest block in the chain
	let precommits = relaychain_authorities
		.iter()
		.map(|id| {
			let key = id.pair();
			let encoded = sp_finality_grandpa::localized_payload(round, set_id, &message);
			let signature = AuthoritySignature::from(key.sign(&encoded));
			SignedPrecommit {
				precommit: precommit.clone(),
				signature,
				id: AuthorityId::from(key.public()),
			}
		})
		.collect();
	let commit = Commit::<RelayChainHeader> {
		target_hash: header_hash,
		target_number: header.number,
		precommits,
	};
	// headers.iter().for_each(|x| {
	// 	dbg!(x.parent_hash, x.hash());
	// });
	let justification =
		GrandpaJustification::<RelayChainHeader> { round: 1, commit, votes_ancestries: vec![] };
	let finality_proof = FinalityProof {
		block: header_hash,
		justification: justification.encode(),
		unknown_headers: headers.clone(),
	};
	let mut parachain_headers = BTreeMap::default();
	let state_proof =
		prove_read_on_trie_backend(&TrieBackend::new(para_db.clone(), root), &[key.as_ref()])
			.unwrap()
			.into_nodes()
			.into_iter()
			.collect::<Vec<_>>();
	// for each relaychain header we construct a corresponding parachain header proofs
	for header in &headers {
		parachain_headers.insert(
			header.hash(),
			ParachainHeaderProofs {
				state_proof: state_proof.clone(),
				extrinsic: timestamp_extrinsic.clone(),
				extrinsic_proof: extrinsic_proof.clone(),
			},
		);
	}

	// let headers = AncestryChain::<RelayChainHeader>::new(&finality_proof.unknown_headers);
	// let target = headers.header(&finality_proof.block).unwrap();
	// let finalized = headers.ancestry(client_state.latest_relay_hash,
	// finality_proof.block).unwrap();

	// for hash in finalized {
	// 	let relay_chain_header =
	// 		headers.header(&hash).expect("Headers have been checked by AncestryChain; qed");
	// 	if let Some(proofs) = parachain_headers.remove(&hash) {
	// 		let ParachainHeaderProofs { extrinsic_proof, extrinsic, state_proof } = proofs;
	// 		let proof = StorageProof::new(state_proof);
	// 		let header = state_machine::read_proof_check::<BlakeTwo256, _>(
	// 			relay_chain_header.state_root(),
	// 			proof,
	// 			&[key.as_ref()],
	// 		)
	// 		.unwrap()
	// 		.remove(key.as_ref())
	// 		.flatten()
	// 		.unwrap();
	// 		let parachain_header = Header::decode(&mut &header[..]).unwrap();
	// 		// Timestamp extrinsic should be the first inherent and hence the first extrinsic
	// 		// https://github.com/paritytech/substrate/blob/d602397a0bbb24b5d627795b797259a44a5e29e9/primitives/trie/src/lib.rs#L99-L101
	// 		let key = codec::Compact(0u32).encode();
	// 		sp_trie::verify_trie_proof::<LayoutV0<BlakeTwo256>, _, _, _>(
	// 			parachain_header.extrinsics_root(),
	// 			&extrinsic_proof,
	// 			&vec![(key, Some(&extrinsic[..]))],
	// 		)
	// 		.unwrap();
	// 	}
	// }

	log::info!("ok");

	let grandpa_header = GrandpaHeader { finality_proof, parachain_headers };
	// verify_parachain_headers_with_grandpa_finality_proof::<RelayChainHeader,
	// HostFunctionsManager>( 	grandpa_client_primitives::ClientState {
	// 		current_authorities: client_state.current_authorities,
	// 		current_set_id: client_state.current_set_id,
	// 		latest_relay_height: client_state.latest_relay_height,
	// 		latest_relay_hash: client_state.latest_relay_hash,
	// 		para_id: client_state.para_id,
	// 	},
	// 	ParachainHeadersWithFinalityProof {
	// 		finality_proof: grandpa_header.finality_proof.clone(),
	// 		parachain_headers: grandpa_header.parachain_headers.clone(),
	// 	},
	// )
	// .unwrap();
	let client_message = AnyClientMessage::Grandpa(ClientMessage::Header(grandpa_header));
	let (update_client_msg, _, _) = chain_b
		.query_latest_ibc_events(finality_event, chain_a)
		.await
		.expect("no event");
	let mut msg =
		MsgUpdateAnyClient::<LocalClientTypes>::decode(&mut update_client_msg.value.as_slice())
			.unwrap();
	// match &mut msg.client_message {
	// 	AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
	// 		header.finality_proof.block = H256::from_low_u64_be(1337);
	// 		let mut justification = GrandpaJustification::<RelayChainHeader>::decode(
	// 			&mut &*header.finality_proof.justification,
	// 		)
	// 		.unwrap();
	//
	// 		let mut precommit = justification.commit.precommits.iter().for_each(|x| {
	// 			dbg!(&x.precommit);
	// 		});
	//
	// 		let mut precommit = justification
	// 			.commit
	// 			.precommits
	// 			.iter()
	// 			.map(|x| x.precommit.clone())
	// 			.next()
	// 			.unwrap();
	//
	// 		precommit.target_hash = H256::from_low_u64_be(precommit.target_number as u64);
	// 		justification.commit.target_hash = precommit.target_hash;
	// 		let message = finality_grandpa::Message::Precommit(precommit.clone());
	// 		justification.commit.precommits = relaychain_authorities
	// 			.iter()
	// 			.map(|id| {
	// 				let key = id.pair();
	// 				let encoded = sp_finality_grandpa::localized_payload(
	// 					justification.round,
	// 					set_id,
	// 					&message,
	// 				);
	// 				let signature = AuthoritySignature::from(key.sign(&encoded));
	// 				SignedPrecommit {
	// 					precommit: precommit.clone(),
	// 					signature,
	// 					id: AuthorityId::from(key.public()),
	// 				}
	// 			})
	// 			.collect();
	// 		header.finality_proof.justification = justification.encode();
	// 	},
	// 	_ => panic!("unexpected client message"),
	// }
	log::info!("{} = {}, {}", msg.client_id, chain_a.client_id(), chain_b.client_id());
	log::info!("{:?} = {:?}, {:?}", msg.signer, chain_a.account_id(), chain_b.account_id());
	let msg =
		MsgUpdateAnyClient::<LocalClientTypes>::new(msg.client_id, client_message, msg.signer);

	log::info!("submitting");
	chain_a
		.submit(vec![Any { value: msg.encode_vec(), type_url: msg.type_url() }])
		.await
		.expect("failed to submit message");

	log::info!("Message submitted");

	let balance_b0 = chain_b.query_relaychain_balance().await.unwrap();
	let balance_a0 = chain_a.query_relaychain_balance().await.unwrap();

	log::info!("balance_b0 = {:?}", balance_b0);
	log::info!("balance_a0 = {:?}", balance_a0);

	// let client_id = chain_a.client_id();
	// let latest_height = chain_b.latest_height_and_timestamp().await.unwrap().0;
	// let response = chain_b.query_client_state(latest_height, client_id).await.unwrap();
	// let client_state = match AnyClientState::try_from(response.client_state.unwrap()).unwrap() {
	// 	AnyClientState::Grandpa(cs) => cs,
	// 	_ => panic!("unexpected client state"),
	// };
	// log::info!("Newest client height: {}", client_state.latest_relay_height);

	// once misbehaviour is submitted, a `ClientMisbehaviour` event should be emitted
	// and equivocation on relaychain should happen (changing the balance of the account)
	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let misbehavour_event_handle = tokio::task::spawn(async move {
		let mut events = client_a_clone.ibc_events().await;
		while let Some((_, events)) = events.next().await {
			for event in events.into_iter().filter_map(identity) {
				log::info!("got event 1: {:?}", event);
				match event {
					IbcEvent::ClientMisbehaviour { .. } => {
						log::info!("Misbehaviour event received");

						sleep(Duration::from_secs(20)).await;

						let balance_b1 = client_b_clone.query_relaychain_balance().await.unwrap();
						let balance_a1 = client_a_clone.query_relaychain_balance().await.unwrap();

						log::info!("balance_b1 = {:?}", balance_b1);
						log::info!("balance_a1 = {:?}", balance_a1);

						break
					},
					_ => (),
				}
			}
		}
	});

	timeout(Duration::from_secs(30), misbehavour_event_handle)
		.await
		.unwrap()
		.unwrap();

	handle.abort()
}
