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
use std::{
	collections::BTreeMap,
	convert::identity,
	sync::Arc,
	time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
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
	// let client_message =
	// AnyClientMessage::decode(&*hex::decode("0a2a2f6962632e6c69676874636c69656e74732e6772616e6470612e76312e436c69656e744d657373616765128c0d0a890d0a96080a20d1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656d12c2051c00000000000000d1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656db600000014d1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656db6000000f574b707ea4fa2dd168e9f4f509570d3286dcc04f0c4fee1037ddbd4748017724d353eef261f07839c84a4d01a4216bee9ec98090be4f6be4a085fbff2a4b608d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69d1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656db60000004f9df0805ffd973d805fbbe3b878b12ba0c32cc7b3f876e27801bc73cae73f53cadd864ac8e2079c9b1819ef0cda7bf0448bca6c1bba79db93027a5ac99fae03439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234fd1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656db6000000882a256a9951e59a98de302867418d8a232bc7222df3805415ab90ebd2c7ddd1cb320e991cb8b5c98389faf4b068cd82984c594f90b6d66ee80e6c4a1cceb6075e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d9d1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656db600000020ad97159c1bad1292061c25b479e824af117646b383afe979a537db563ed1450c2e2a2e3ebb6f9dff9a384b88e3f45300515ec5b2aa49dd1ac379dd20eb7f031dfe3e22cc0d45c70779c1095f7489a8ef3cf52d62fbd8c2fa38c9f1723502b5d1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656db600000092cc16b11e816180ced68f70bd602bc41b23a581a830ceb58b295383347deec6db6bac901c6949504f54f171402c1c26ecbe47c8141f20778b4a32f4fb6acc0d568cb4a574c6d178feb39c27dfc8b3f789e5f5423e19c71633c748b9acf086b5001a63ac5425ac07d54a128d365f50d624a3a69aac74f024b1e701372823be0911d6b1d102efea6b99636a902f18d716e5528bef11376025625ebc3fcdcff3b30c98f9082b0000000000000000000000000000000000000000000000000000000000000000001a6380edea8fbccb6ce8b22a80daf53a06d4a7c417d54a61983a0c9c5bdb7d1178fcd502efea6b99636a902f18d716e5528bef11376025625ebc3fcdcff3b30c98f9082b0000000000000000000000000000000000000000000000000000000000000000001a630a0186eb7471e469c26a1e494c34870541f103f34b46dbb2f6fff4d4d2786ddfd902efea6b99636a902f18d716e5528bef11376025625ebc3fcdcff3b30c98f9082b00000000000000000000000000000000000000000000000000000000000000000012cd010a200a0186eb7471e469c26a1e494c34870541f103f34b46dbb2f6fff4d4d2786ddf12a8010a95017f19cd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c363f5a4efb16ffa83d007000095018d0100000000000000000000000000000000000000000000000000000000000000000d0100000000000000000000000000000000000000000000000000000000000000002b37ea19fa04c994bb96c72f9b69c42fba07d1374dd55d3ff9884776f7c0b551001209000001000312b64f631a0342000012cd010a2080edea8fbccb6ce8b22a80daf53a06d4a7c417d54a61983a0c9c5bdb7d1178fc12a8010a95017f19cd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c363f5a4efb16ffa83d007000095018d0100000000000000000000000000000000000000000000000000000000000000000d0100000000000000000000000000000000000000000000000000000000000000002b37ea19fa04c994bb96c72f9b69c42fba07d1374dd55d3ff9884776f7c0b551001209000001000312b64f631a0342000012cd010a20d1403a42825a94d1c13dff7cf6c677f5629b00674397134ef557f037629d656d12a8010a95017f19cd710b30bd2eab0352ddcc26417aa1941b3c252fcb29d88eff4f3de5de4476c363f5a4efb16ffa83d007000095018d0100000000000000000000000000000000000000000000000000000000000000000d0100000000000000000000000000000000000000000000000000000000000000002b37ea19fa04c994bb96c72f9b69c42fba07d1374dd55d3ff9884776f7c0b551001209000001000312b64f631a03420000").unwrap()).unwrap();
	// match client_message {
	// 	AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
	// 		let justification = GrandpaJustification::<RelayChainHeader>::decode(
	// 			&mut header.finality_proof.justification.as_slice(),
	// 		)
	// 		.unwrap();
	// 		panic!("{}", justification.round);
	// 	},
	// 	_ => {},
	// }

	let (handle, channel_id, channel_b, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, Duration::from_secs(60 * 2)).await;
	handle.abort();
	// chain_b.check_for_misbehaviour(chain_a, client_message).await.unwrap();

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
	// log::info!("Latest client height: {}", client_state.latest_relay_height);

	let mut finality_event = chain_b.finality_notifications().await.next().await.expect("no event");
	let set_id = client_state.current_set_id;

	// construct an extrinsic proof with the mandatory timestamp extrinsic
	let mut para_db = MemoryDB::<BlakeTwo256>::default();

	// TODO: how to construct timestamp extrinsic via metadata?
	let mut timestamp_extrinsic =
		(1u8, 0u8, Compact(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()))
			.encode();
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

	let (update_client_msg, _, _) = chain_b
		.query_latest_ibc_events(finality_event, chain_a)
		.await
		.expect("no event");
	let mut msg =
		MsgUpdateAnyClient::<LocalClientTypes>::decode(&mut update_client_msg.value.as_slice())
			.unwrap();
	let round = match &mut msg.client_message {
		AnyClientMessage::Grandpa(ClientMessage::Header(header)) => {
			header.finality_proof.block = H256::from_low_u64_be(1337);
			let mut justification = GrandpaJustification::<RelayChainHeader>::decode(
				&mut &*header.finality_proof.justification,
			)
			.unwrap();

			justification.round
			// let mut precommit = justification.commit.precommits.iter().for_each(|x| {
			// 	dbg!(&x.precommit);
			// });
			//
			// let mut precommit = justification
			// 	.commit
			// 	.precommits
			// 	.iter()
			// 	.map(|x| x.precommit.clone())
			// 	.next()
			// 	.unwrap();
			//
			// precommit.target_hash = H256::from_low_u64_be(precommit.target_number as u64);
			// justification.commit.target_hash = precommit.target_hash;
			// let message = finality_grandpa::Message::Precommit(precommit.clone());
			// justification.commit.precommits = relaychain_authorities
			// 	.iter()
			// 	.map(|id| {
			// 		let key = id.pair();
			// 		let encoded = sp_finality_grandpa::localized_payload(
			// 			justification.round,
			// 			set_id,
			// 			&message,
			// 		);
			// 		let signature = AuthoritySignature::from(key.sign(&encoded));
			// 		SignedPrecommit {
			// 			precommit: precommit.clone(),
			// 			signature,
			// 			id: AuthorityId::from(key.public()),
			// 		}
			// 	})
			// 	.collect();
			// header.finality_proof.justification = justification.encode();
		},
		_ => panic!("unexpected client message"),
	};
	log::info!("round");

	// let round = 1;
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
		GrandpaJustification::<RelayChainHeader> { round, commit, votes_ancestries: vec![] };
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
