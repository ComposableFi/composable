use crate::{setup_connection_and_channel, StreamExt};
use finality_grandpa::{Precommit, SignedPrecommit};
use grandpa_client_primitives::{
	justification::GrandpaJustification, parachain_header_storage_key, Commit, FinalityProof,
	ParachainHeaderProofs,
};
use hyperspace_primitives::{mock::LocalClientTypes, TestProvider};
use ibc::{
	core::ics02_client::msgs::update_client::MsgUpdateAnyClient, events::IbcEvent, tx_msg::Msg,
};
use ibc_proto::google::protobuf::Any;
use ics10_grandpa::client_message::{ClientMessage, Header as GrandpaHeader, RelayChainHeader};
use log::info;
use pallet_ibc::light_clients::{AnyClientMessage, AnyClientState};
use polkadot_core_primitives::Header;
use sp_core::{Decode, Encode, Pair};
use sp_finality_grandpa::{AuthorityId, AuthoritySignature};
use sp_keyring::ed25519::Keyring;
use sp_runtime::{codec::Compact, traits::BlakeTwo256};
use sp_state_machine::{prove_read_on_trie_backend, TrieBackend};
use sp_trie::{generate_trie_proof, LayoutV0, MemoryDB, TrieDBMut, TrieMut};
use std::{
	collections::BTreeMap,
	convert::identity,
	time::{Duration, SystemTime, UNIX_EPOCH},
};
use tendermint_proto::Protobuf;
use tokio::time::timeout;

/// Submits a misbehaviour message of client B on chain A.
pub async fn ibc_messaging_submit_misbehaviour<A, B>(chain_a: &mut A, chain_b: &mut B)
where
	A: TestProvider,
	A::FinalityEvent: Send + Sync,
	A::Error: From<B::Error>,
	B: TestProvider,
	B::FinalityEvent: Send + Sync,
	B::Error: From<A::Error>,
{
	let (handle, _channel_id, _channel_b, _connection_id) =
		setup_connection_and_channel(chain_a, chain_b, Duration::from_secs(60 * 2)).await;
	handle.abort();

	let client_a_clone = chain_a.clone();
	let client_b_clone = chain_b.clone();
	let handle = tokio::task::spawn(async move {
		hyperspace::fish(client_a_clone, client_b_clone).await.unwrap()
	});
	info!("Waiting for the next block...");

	let relaychain_authorities = [
		Keyring::Alice,
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

	let finality_event = chain_b.finality_notifications().await.next().await.expect("no event");
	let set_id = client_state.current_set_id;

	// construct an extrinsic proof with the mandatory timestamp extrinsic
	let mut para_db = MemoryDB::<BlakeTwo256>::default();

	// TODO: how to construct timestamp extrinsic via metadata?
	let mut timestamp_extrinsic =
		(1u8, 0u8, Compact(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()))
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
			let justification = GrandpaJustification::<RelayChainHeader>::decode(
				&mut &*header.finality_proof.justification,
			)
			.unwrap();

			justification.round
		},
		_ => panic!("unexpected client message"),
	};

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

	let grandpa_header = GrandpaHeader { finality_proof, parachain_headers };
	let client_message = AnyClientMessage::Grandpa(ClientMessage::Header(grandpa_header));

	let msg =
		MsgUpdateAnyClient::<LocalClientTypes>::new(msg.client_id, client_message, msg.signer);

	chain_a
		.submit(vec![Any { value: msg.encode_vec(), type_url: msg.type_url() }])
		.await
		.expect("failed to submit message");

	let client_a_clone = chain_a.clone();
	let misbehavour_event_handle = tokio::task::spawn(async move {
		let mut events = client_a_clone.ibc_events().await;
		while let Some((_, events)) = events.next().await {
			for event in events.into_iter().filter_map(identity) {
				match event {
					IbcEvent::ClientMisbehaviour { .. } => return,
					_ => (),
				}
			}
		}
	});

	timeout(Duration::from_secs(30), misbehavour_event_handle)
		.await
		.expect("timeout")
		.expect("failed to receive misbehaviour event");

	handle.abort()
}
