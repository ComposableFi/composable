use crate::{routing::Context, Config, MODULE_ID};
use core::{str::FromStr, time::Duration};
use frame_support::traits::Get;
use ibc::{
	clients::{
		ics07_tendermint::{
			client_state::{AllowUpdate, ClientState as TendermintClientState},
			consensus_state::ConsensusState,
			header::Header,
		},
		ics11_beefy::{
			client_state::ClientState as BeefyClientState,
			consensus_state::ConsensusState as BeefyConsensusState,
		},
	},
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState, client_state::AnyClientState,
			client_type::ClientType, header::AnyHeader, msgs::update_client::MsgUpdateAnyClient,
			trust_threshold::TrustThreshold,
		},
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty, State as ConnState},
			msgs::{
				conn_open_ack::MsgConnectionOpenAck, conn_open_confirm::MsgConnectionOpenConfirm,
				conn_open_try::MsgConnectionOpenTry,
			},
			version::Version as ConnVersion,
		},
		ics04_channel::{
			channel::{
				ChannelEnd, Counterparty as ChannelCounterParty, Order as ChannelOrder,
				State as ChannelState,
			},
			context::{ChannelKeeper, ChannelReader},
			msgs::{
				acknowledgement::MsgAcknowledgement, chan_close_confirm::MsgChannelCloseConfirm,
				chan_close_init::MsgChannelCloseInit, chan_open_ack::MsgChannelOpenAck,
				chan_open_confirm::MsgChannelOpenConfirm, chan_open_try::MsgChannelOpenTry,
				recv_packet::MsgRecvPacket, timeout::MsgTimeout,
			},
			packet::Packet,
			Version as ChannelVersion,
		},
		ics23_commitment::{commitment::CommitmentPrefix, specs::ProofSpecs},
		ics24_host::{
			identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
			path::{
				AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath,
				CommitmentsPath, ConnectionsPath, SeqRecvsPath,
			},
		},
	},
	proofs::Proofs,
	signer::Signer,
	timestamp::Timestamp,
	Height,
};
use ibc_proto::{ibc::core::commitment::v1::MerkleProof, ics23::CommitmentProof};
use scale_info::prelude::{format, string::ToString};
use sp_std::prelude::*;
use tendermint::{block::signed_header::SignedHeader, validator::Set as ValidatorSet, Hash};
use tendermint_proto::Protobuf;

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

/// Create a mock avl implementation that can be used to mock tendermint's iavl tree.
fn create_avl() -> simple_iavl::avl::AvlTree<Vec<u8>, Vec<u8>> {
	let mut avl_tree = simple_iavl::avl::AvlTree::new();
	// Insert some dummy data in tree
	for i in 0..100u8 {
		let key = vec![i; 32];
		avl_tree.insert(key, vec![0u8; 64]);
	}
	avl_tree
}

/// Creates a tendermint header
/// Light signed header bytes obtained from
/// `tendermint_testgen::LightBlock::new_default_with_time_and_chain_id("test-chain".to_string(),
/// Time::now(), 2).generate().unwrap().signed_header.encode_vec();`
fn create_tendermint_header() -> Header {
	let raw_validator_set = hex_literal::hex!("0a3c0a14a6e7b6810df8120580f2a81710e228f454f99c9712220a2050c4a5871ad3379f2879d12cef750d1211633283a9c3730238e6ddf084db4c8a18320a3c0a14c7832263600476fd6ff4c5cb0a86080d0e5f48b212220a20ebe80b7cadea277ac05fb85c7164fe15ebd6873c4a74b3296a462a1026fd9b0f18321864").to_vec();
	let raw_signed_header = hex_literal::hex!("0a9c010a02080b120a746573742d636861696e1802220c08abc49a930610a8f39fc1014220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf984a20e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf985220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf987214a6e7b6810df8120580f2a81710e228f454f99c9712a202080210011a480a20afc35ec1d9620052c6d71122cb5504ee68802184023a217547ca2248df902fbb122408011220afc35ec1d9620052c6d71122cb5504ee68802184023a217547ca2248df902fbb226808021214a6e7b6810df8120580f2a81710e228f454f99c971a0c08abc49a930610a8f39fc1012240a91380fe3cde0147994b82a0b00b28bd82870df38b2cad5b4ba25c9a4c833cd50f3143ffaa4e924eccd143639fb3decf6b94570aff2c50f1346e88d06555fd0d226808021214c7832263600476fd6ff4c5cb0a86080d0e5f48b21a0c08abc49a930610a8f39fc10122407e2e349d9a0adfc3564654fcf88d328cf50a13179cc5ddaf87dd0e1abd4b45685312def0affbae29e8b7882af3b76b056f81b701bb2e43769fb63fe3696b090f").to_vec();
	let signed_header = SignedHeader::decode_vec(&*raw_signed_header).unwrap();

	let validator_set = ValidatorSet::decode_vec(&*raw_validator_set).unwrap();
	Header {
		signed_header,
		validator_set: validator_set.clone(),
		trusted_height: Height::new(0, 1),
		trusted_validator_set: validator_set,
	}
}

pub fn create_mock_state() -> (TendermintClientState, ConsensusState) {
	let spec = simple_iavl::avl::get_proof_spec();
	// The tendermint light client requires two proof specs one for the iavl tree used to
	// in constructing the ibc commitment root and another for the tendermint state tree
	// For the benchmarks we use the same spec for both so we can use one single iavl tree
	// to generate both proofs.
	let proof_specs = ProofSpecs::from(vec![spec.clone(), spec]);
	let mock_client_state = TendermintClientState::new(
		ChainId::from_string("test-chain"),
		TrustThreshold::ONE_THIRD,
		Duration::new(65000, 0),
		Duration::new(128000, 0),
		Duration::new(3, 0),
		Height::new(0, 1),
		proof_specs,
		vec!["".to_string()],
		AllowUpdate { after_expiry: true, after_misbehaviour: false },
	)
	.unwrap();

	// Light signed header bytes obtained from
	// `tendermint_testgen::LightBlock::new_default_with_time_and_chain_id("test-chain".to_string(),
	// Time::now(), 1 ).generate().unwrap().signed_header.encode_vec();`
	let raw_signed_header = hex_literal::hex!("0a9c010a02080b120a746573742d636861696e1801220c08c9b99a93061088cdfc87014220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf984a20e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf985220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf987214a6e7b6810df8120580f2a81710e228f454f99c9712a202080110011a480a20219a163917d9297e8f15bff09da55f82dc4594002fd3b0ade63971c1c7768333122408011220219a163917d9297e8f15bff09da55f82dc4594002fd3b0ade63971c1c7768333226808021214a6e7b6810df8120580f2a81710e228f454f99c971a0c08c9b99a93061088cdfc870122401ba8b679b2cbf5cd7b166a704fa299c8b2161b96da49068a25bf84141242aa3550ee2b2f7ad78ef520a8d723267864dcf7f814382d4418bed783746732d45a0e226808021214c7832263600476fd6ff4c5cb0a86080d0e5f48b21a0c08c9b99a93061088cdfc870122406cb04246b99e1813aae77b6b8328728b0763a5396eef72b3300b81814671ccb8d44d0e28cdcf818a3002f837c09c5b6cddefc4ba36f6408e51eb4ed9d95fbd08").to_vec();
	let signed_header = SignedHeader::decode_vec(&*raw_signed_header).unwrap();
	let mock_cs_state =
		ibc::clients::ics07_tendermint::consensus_state::ConsensusState::from(signed_header.header);
	(mock_client_state, mock_cs_state)
}

pub fn create_mock_beefy_client_state() -> (BeefyClientState, BeefyConsensusState) {
	let client_state = BeefyClientState {
		chain_id: Default::default(),
		mmr_root_hash: Default::default(),
		latest_beefy_height: 1,
		frozen_height: None,
		beefy_activation_block: 0,
		authority: Default::default(),
		next_authority_set: Default::default(),
	};

	let timestamp = ibc::timestamp::Timestamp::from_nanoseconds(1).unwrap();
	let timestamp = timestamp.into_tm_time().unwrap();
	let cs_state = BeefyConsensusState { timestamp, root: vec![].into() };
	(client_state, cs_state)
}

pub fn create_client_update() -> MsgUpdateAnyClient {
	MsgUpdateAnyClient {
		client_id: ClientId::new(ClientType::Tendermint, 0).unwrap(),
		header: AnyHeader::Tendermint(create_tendermint_header()),
		signer: Signer::from_str(MODULE_ID).unwrap(),
	}
}
// Proof generation process for all tendermint benchmarks
// The process is as follows, we insert the all the required ibc paths and values needed to generate
// the proof in the context of the benchmark in question, then we extract the root from the tree and
// also extract a proof for any key we need After this we insert the extracted root inside the avl
// tree as the value for the commitment prefix. We then get a proof for the commitment prefix.
// We then extract the new root and use this as the commitment root
// This new root is then set as the ibc commitment root in the light client consensus state.

// Creates a MsgConnectionOpenTry from a tendermint chain submitted to a substrate chain
pub fn create_conn_open_try<T: Config>() -> (ConsensusState, MsgConnectionOpenTry) {
	let client_id = ClientId::new(ClientType::Tendermint, 0).unwrap();
	let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
	let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
	let chain_a_counterparty = Counterparty::new(
		counterparty_client_id.clone(),
		Some(ConnectionId::new(1)),
		commitment_prefix.clone(),
	);
	let delay_period = core::time::Duration::from_nanos(1000);
	let chain_b_connection_counterparty =
		Counterparty::new(client_id.clone(), None, commitment_prefix.clone());
	let mut avl_tree = create_avl();
	let connection_end = ConnectionEnd::new(
		ConnState::Init,
		counterparty_client_id.clone(),
		chain_b_connection_counterparty,
		vec![ConnVersion::default()],
		delay_period,
	);
	crate::Pallet::<T>::insert_default_consensus_state(1);
	let (client_state, cs_state) = create_mock_beefy_client_state();
	let consensus_path = format!(
		"{}",
		ClientConsensusStatePath {
			client_id: counterparty_client_id.clone(),
			epoch: u32::from(parachain_info::Pallet::<T>::get()).into(),
			height: 1
		}
	)
	.as_bytes()
	.to_vec();

	let client_path = format!("{}", ClientStatePath(counterparty_client_id)).as_bytes().to_vec();
	let path = format!("{}", ConnectionsPath(ConnectionId::new(1))).as_bytes().to_vec();
	avl_tree.insert(path.clone(), connection_end.encode_vec());
	avl_tree.insert(consensus_path.clone(), AnyConsensusState::Beefy(cs_state).encode_vec());
	avl_tree.insert(client_path.clone(), AnyClientState::Beefy(client_state.clone()).encode_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	let consensus_proof = avl_tree.get_proof(&*consensus_path).unwrap();
	let client_proof = avl_tree.get_proof(&*client_path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&consensus_proof, &mut buf).unwrap();
	let consensus_proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&client_proof, &mut buf).unwrap();
	let client_proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0.clone()] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let consensus_proof = MerkleProof { proofs: vec![consensus_proof, proof_0.clone()] };
	let client_proof = MerkleProof { proofs: vec![client_proof, proof_0] };
	let mut consensus_buf = Vec::new();
	let mut client_buf = Vec::new();
	prost::Message::encode(&consensus_proof, &mut consensus_buf).unwrap();
	prost::Message::encode(&client_proof, &mut client_buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};
	(
		cs_state,
		MsgConnectionOpenTry {
			previous_connection_id: Some(ConnectionId::new(0)),
			client_id,
			client_state: Some(AnyClientState::Beefy(client_state)),
			counterparty: chain_a_counterparty,
			counterparty_versions: vec![ConnVersion::default()],
			proofs: Proofs::new(
				buf.try_into().unwrap(),
				Some(client_buf.try_into().unwrap()),
				Some(
					ibc::proofs::ConsensusProof::new(
						consensus_buf.try_into().unwrap(),
						Height::new(u32::from(parachain_info::Pallet::<T>::get()).into(), 1),
					)
					.unwrap(),
				),
				None,
				Height::new(0, 2),
			)
			.unwrap(),
			delay_period,
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_conn_open_ack<T: Config>() -> (ConsensusState, MsgConnectionOpenAck) {
	let client_id = ClientId::new(ClientType::Tendermint, 0).unwrap();
	let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
	let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
	let delay_period = core::time::Duration::from_nanos(1000);
	let chain_b_connection_counterparty =
		Counterparty::new(client_id.clone(), Some(ConnectionId::new(0)), commitment_prefix.clone());
	let mut avl_tree = create_avl();
	let connection_end = ConnectionEnd::new(
		ConnState::TryOpen,
		counterparty_client_id.clone(),
		chain_b_connection_counterparty,
		vec![ConnVersion::default()],
		delay_period,
	);
	crate::Pallet::<T>::insert_default_consensus_state(1);
	let (client_state, cs_state) = create_mock_beefy_client_state();
	let consensus_path = format!(
		"{}",
		ClientConsensusStatePath {
			client_id: counterparty_client_id.clone(),
			epoch: u32::from(parachain_info::Pallet::<T>::get()).into(),
			height: 1
		}
	)
	.as_bytes()
	.to_vec();

	let client_path = format!("{}", ClientStatePath(counterparty_client_id)).as_bytes().to_vec();
	let path = format!("{}", ConnectionsPath(ConnectionId::new(1))).as_bytes().to_vec();
	avl_tree.insert(path.clone(), connection_end.encode_vec());
	avl_tree.insert(consensus_path.clone(), AnyConsensusState::Beefy(cs_state).encode_vec());
	avl_tree.insert(client_path.clone(), AnyClientState::Beefy(client_state.clone()).encode_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	let consensus_proof = avl_tree.get_proof(&*consensus_path).unwrap();
	let client_proof = avl_tree.get_proof(&*client_path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&consensus_proof, &mut buf).unwrap();
	let consensus_proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&client_proof, &mut buf).unwrap();
	let client_proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0.clone()] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let consensus_proof = MerkleProof { proofs: vec![consensus_proof, proof_0.clone()] };
	let client_proof = MerkleProof { proofs: vec![client_proof, proof_0] };
	let mut consensus_buf = Vec::new();
	let mut client_buf = Vec::new();
	prost::Message::encode(&consensus_proof, &mut consensus_buf).unwrap();
	prost::Message::encode(&client_proof, &mut client_buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};
	(
		cs_state,
		MsgConnectionOpenAck {
			connection_id: ConnectionId::new(0),
			counterparty_connection_id: ConnectionId::new(1),
			client_state: Some(AnyClientState::Beefy(client_state)),
			proofs: Proofs::new(
				buf.try_into().unwrap(),
				Some(client_buf.try_into().unwrap()),
				Some(
					ibc::proofs::ConsensusProof::new(
						consensus_buf.try_into().unwrap(),
						Height::new(u32::from(parachain_info::Pallet::<T>::get()).into(), 1),
					)
					.unwrap(),
				),
				None,
				Height::new(0, 2),
			)
			.unwrap(),
			version: ConnVersion::default(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_conn_open_confirm<T: Config>() -> (ConsensusState, MsgConnectionOpenConfirm) {
	let client_id = ClientId::new(ClientType::Tendermint, 0).unwrap();
	let counterparty_client_id = ClientId::new(ClientType::Beefy, 1).unwrap();
	let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
	let delay_period = core::time::Duration::from_nanos(1000);
	let chain_b_connection_counterparty =
		Counterparty::new(client_id.clone(), Some(ConnectionId::new(0)), commitment_prefix.clone());
	let mut avl_tree = create_avl();
	let connection_end = ConnectionEnd::new(
		ConnState::Open,
		counterparty_client_id.clone(),
		chain_b_connection_counterparty,
		vec![ConnVersion::default()],
		delay_period,
	);
	crate::Pallet::<T>::insert_default_consensus_state(1);
	let (.., cs_state) = create_mock_beefy_client_state();
	let consensus_path = format!(
		"{}",
		ClientConsensusStatePath {
			client_id: counterparty_client_id.clone(),
			epoch: u32::from(parachain_info::Pallet::<T>::get()).into(),
			height: 1
		}
	)
	.as_bytes()
	.to_vec();

	let path = format!("{}", ConnectionsPath(ConnectionId::new(1))).as_bytes().to_vec();
	avl_tree.insert(path.clone(), connection_end.encode_vec());
	avl_tree.insert(consensus_path.clone(), AnyConsensusState::Beefy(cs_state).encode_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	let consensus_proof = avl_tree.get_proof(&*consensus_path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&consensus_proof, &mut buf).unwrap();
	let consensus_proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0.clone()] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let consensus_proof = MerkleProof { proofs: vec![consensus_proof, proof_0.clone()] };
	let mut consensus_buf = Vec::new();
	prost::Message::encode(&consensus_proof, &mut consensus_buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};
	(
		cs_state,
		MsgConnectionOpenConfirm {
			connection_id: ConnectionId::new(0),
			proofs: Proofs::new(
				buf.try_into().unwrap(),
				None,
				Some(
					ibc::proofs::ConsensusProof::new(
						consensus_buf.try_into().unwrap(),
						Height::new(u32::from(parachain_info::Pallet::<T>::get()).into(), 1),
					)
					.unwrap(),
				),
				None,
				Height::new(0, 2),
			)
			.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_chan_open_try() -> (ConsensusState, MsgChannelOpenTry) {
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	let counterparty = ChannelCounterParty::new(port_id.clone(), None);
	let channel_end = ChannelEnd::new(
		ChannelState::Init,
		ChannelOrder::Unordered,
		counterparty.clone(),
		vec![ConnectionId::new(1)],
		ChannelVersion::default(),
	);
	let mut avl_tree = create_avl();
	let path = format!("{}", ChannelEndsPath(port_id.clone(), ChannelId::new(0)))
		.as_bytes()
		.to_vec();
	avl_tree.insert(path.clone(), channel_end.encode_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};
	let mut channel_end = ChannelEnd::new(
		ChannelState::Init,
		ChannelOrder::Unordered,
		counterparty.clone(),
		vec![ConnectionId::new(0)],
		ChannelVersion::default(),
	);
	channel_end.set_counterparty_channel_id(ChannelId::new(0));
	(
		cs_state,
		MsgChannelOpenTry {
			port_id,
			previous_channel_id: Some(ChannelId::new(0)),
			channel: channel_end,
			counterparty_version: ChannelVersion::default(),
			proofs: Proofs::new(buf.try_into().unwrap(), None, None, None, Height::new(0, 2))
				.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_chan_open_ack() -> (ConsensusState, MsgChannelOpenAck) {
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	let counterparty = ChannelCounterParty::new(port_id.clone(), Some(ChannelId::new(0)));
	let channel_end = ChannelEnd::new(
		ChannelState::TryOpen,
		ChannelOrder::Unordered,
		counterparty.clone(),
		vec![ConnectionId::new(1)],
		ChannelVersion::default(),
	);
	let mut avl_tree = create_avl();
	let path = format!("{}", ChannelEndsPath(port_id.clone(), ChannelId::new(0)))
		.as_bytes()
		.to_vec();
	avl_tree.insert(path.clone(), channel_end.encode_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};

	(
		cs_state,
		MsgChannelOpenAck {
			port_id,
			channel_id: ChannelId::new(0),
			counterparty_channel_id: ChannelId::new(0),
			counterparty_version: ChannelVersion::default(),
			proofs: Proofs::new(buf.try_into().unwrap(), None, None, None, Height::new(0, 2))
				.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_chan_open_confirm() -> (ConsensusState, MsgChannelOpenConfirm) {
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	let counterparty = ChannelCounterParty::new(port_id.clone(), Some(ChannelId::new(0)));
	let channel_end = ChannelEnd::new(
		ChannelState::Open,
		ChannelOrder::Unordered,
		counterparty.clone(),
		vec![ConnectionId::new(1)],
		ChannelVersion::default(),
	);
	let mut avl_tree = create_avl();
	let path = format!("{}", ChannelEndsPath(port_id.clone(), ChannelId::new(0)))
		.as_bytes()
		.to_vec();
	avl_tree.insert(path.clone(), channel_end.encode_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};

	(
		cs_state,
		MsgChannelOpenConfirm {
			port_id,
			channel_id: ChannelId::new(0),
			proofs: Proofs::new(buf.try_into().unwrap(), None, None, None, Height::new(0, 2))
				.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_chan_close_init() -> MsgChannelCloseInit {
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	MsgChannelCloseInit {
		port_id,
		channel_id: ChannelId::new(0),
		signer: Signer::from_str(MODULE_ID).unwrap(),
	}
}

pub fn create_chan_close_confirm() -> (ConsensusState, MsgChannelCloseConfirm) {
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	let counterparty = ChannelCounterParty::new(port_id.clone(), Some(ChannelId::new(0)));
	let channel_end = ChannelEnd::new(
		ChannelState::Closed,
		ChannelOrder::Unordered,
		counterparty.clone(),
		vec![ConnectionId::new(1)],
		ChannelVersion::default(),
	);
	let mut avl_tree = create_avl();
	let path = format!("{}", ChannelEndsPath(port_id.clone(), ChannelId::new(0)))
		.as_bytes()
		.to_vec();
	avl_tree.insert(path.clone(), channel_end.encode_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};

	(
		cs_state,
		MsgChannelCloseConfirm {
			port_id,
			channel_id: Default::default(),
			proofs: Proofs::new(buf.try_into().unwrap(), None, None, None, Height::new(0, 2))
				.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_recv_packet<T: Config + Send + Sync>(data: Vec<u8>) -> (ConsensusState, MsgRecvPacket)
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	let packet = Packet {
		sequence: 1u64.into(),
		source_port: port_id.clone(),
		source_channel: ChannelId::new(0),
		destination_port: port_id.clone(),
		destination_channel: ChannelId::new(0),
		data,
		timeout_height: Height::new(2000, 5),
		timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
			.unwrap(),
	};
	let ctx = Context::<T>::new();
	let commitment =
		ctx.packet_commitment(packet.data.clone(), packet.timeout_height, packet.timeout_timestamp);
	let mut avl_tree = create_avl();
	let path = format!(
		"{}",
		CommitmentsPath { port_id, channel_id: ChannelId::new(0), sequence: 1.into() }
	)
	.as_bytes()
	.to_vec();
	avl_tree.insert(path.clone(), commitment.into_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};

	(
		cs_state,
		MsgRecvPacket {
			packet,
			proofs: Proofs::new(buf.try_into().unwrap(), None, None, None, Height::new(0, 2))
				.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_ack_packet<T: Config + Send + Sync>(
	data: Vec<u8>,
	ack: Vec<u8>,
) -> (ConsensusState, MsgAcknowledgement)
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	let packet = Packet {
		sequence: 1u64.into(),
		source_port: port_id.clone(),
		source_channel: ChannelId::new(0),
		destination_port: port_id.clone(),
		destination_channel: ChannelId::new(0),
		data: data.clone(),
		timeout_height: Height::new(2000, 5),
		timeout_timestamp: Timestamp::from_nanoseconds(1690894363u64.saturating_mul(1000000000))
			.unwrap(),
	};
	let mut ctx = Context::<T>::new();
	let commitment = ctx.packet_commitment(data, packet.timeout_height, packet.timeout_timestamp);
	ctx.store_packet_commitment((port_id.clone(), ChannelId::new(0), 1.into()), commitment)
		.unwrap();
	let ack_commitment = ctx.ack_commitment(ack.clone().into());

	let mut avl_tree = create_avl();
	let path =
		format!("{}", AcksPath { port_id, channel_id: ChannelId::new(0), sequence: 1.into() })
			.as_bytes()
			.to_vec();
	avl_tree.insert(path.clone(), ack_commitment.into_vec());
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};

	(
		cs_state,
		MsgAcknowledgement {
			packet,
			acknowledgement: ack.into(),
			proofs: Proofs::new(buf.try_into().unwrap(), None, None, None, Height::new(0, 2))
				.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}

pub fn create_timeout_packet<T: Config + Send + Sync>(data: Vec<u8>) -> (ConsensusState, MsgTimeout)
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
	let packet = Packet {
		sequence: 1u64.into(),
		source_port: port_id.clone(),
		source_channel: ChannelId::new(0),
		destination_port: port_id.clone(),
		destination_channel: ChannelId::new(0),
		data: data.clone(),
		timeout_height: Height::new(0, 1),
		timeout_timestamp: Timestamp::from_nanoseconds(1620894363u64.saturating_mul(1000000000))
			.unwrap(),
	};
	let mut ctx = Context::<T>::new();
	let commitment = ctx.packet_commitment(data, packet.timeout_height, packet.timeout_timestamp);
	ctx.store_packet_commitment((port_id.clone(), ChannelId::new(0), 1.into()), commitment)
		.unwrap();

	let mut avl_tree = create_avl();
	let path = format!("{}", SeqRecvsPath(port_id, ChannelId::new(0))).as_bytes().to_vec();
	let mut seq_bytes = Vec::new();
	prost::Message::encode(&1u64, &mut seq_bytes).unwrap();
	avl_tree.insert(path.clone(), seq_bytes);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof = avl_tree.get_proof(&*path).unwrap();
	avl_tree.insert("ibc".as_bytes().to_vec(), root);
	let root = match avl_tree.root_hash().unwrap().clone() {
		Hash::Sha256(root) => root.to_vec(),
		Hash::None => panic!("Failed to generate root hash"),
	};
	let proof_0 = avl_tree.get_proof("ibc".as_bytes()).unwrap();
	let mut buf = Vec::new();
	prost::Message::encode(&proof, &mut buf).unwrap();
	let proof: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	buf.clear();
	prost::Message::encode(&proof_0, &mut buf).unwrap();
	let proof_0: CommitmentProof = prost::Message::decode(buf.as_ref()).unwrap();
	let merkle_proof = MerkleProof { proofs: vec![proof, proof_0] };
	buf.clear();
	prost::Message::encode(&merkle_proof, &mut buf).unwrap();
	let header = create_tendermint_header();
	let cs_state = ConsensusState {
		timestamp: header.signed_header.header.time,
		root: root.into(),
		next_validators_hash: header.signed_header.header.next_validators_hash,
	};

	(
		cs_state,
		MsgTimeout {
			packet,
			next_sequence_recv: Default::default(),
			proofs: Proofs::new(buf.try_into().unwrap(), None, None, None, Height::new(0, 2))
				.unwrap(),
			signer: Signer::from_str(MODULE_ID).unwrap(),
		},
	)
}
