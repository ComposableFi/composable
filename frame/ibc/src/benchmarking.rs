//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as PalletIbc;
use core::time::Duration;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use ibc::{
	clients::ics07_tendermint::{
		client_state::{AllowUpdate, ClientState as TendermintClientState},
		consensus_state::ConsensusState,
		header::Header,
	},
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState,
			client_state::{AnyClientState, ClientState},
			client_type::ClientType,
			context::ClientKeeper,
			header::AnyHeader,
			height::Height,
			msgs::{
				create_client::{MsgCreateAnyClient, TYPE_URL},
				update_client::{MsgUpdateAnyClient, TYPE_URL as UPDATE_CLIENT_TYPE_URL},
			},
			trust_threshold::TrustThreshold,
		},
		ics03_connection::{
			connection::{ConnectionEnd, Counterparty, State},
			context::ConnectionKeeper,
			msgs::conn_open_init,
			version::Version as ConnVersion,
		},
		ics04_channel::{
			channel::{ChannelEnd, Order},
			msgs::chan_open_init::{MsgChannelOpenInit, TYPE_URL as CHAN_OPEN_TYPE_URL},
			Version as ChanVersion,
		},
		ics23_commitment::{commitment::CommitmentPrefix, specs::ProofSpecs},
		ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
	},
	signer::Signer,
	timestamp::Timestamp,
};
use ibc_trait::IbcTrait;
use scale_info::prelude::string::ToString;
use sp_std::vec;
use tendermint::{block::signed_header::SignedHeader, validator::Set as ValidatorSet};
use tendermint_proto::Protobuf;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn create_mock_state() -> (TendermintClientState, ConsensusState) {
	let mock_client_state = TendermintClientState::new(
		ChainId::from_string("test-chain"),
		TrustThreshold::ONE_THIRD,
		Duration::new(65000, 0),
		Duration::new(128000, 0),
		Duration::new(3, 0),
		Height::new(0, 1),
		ProofSpecs::default(),
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

fn create_client_update() -> MsgUpdateAnyClient {
	let raw_validator_set = hex_literal::hex!("0a3c0a14a6e7b6810df8120580f2a81710e228f454f99c9712220a2050c4a5871ad3379f2879d12cef750d1211633283a9c3730238e6ddf084db4c8a18320a3c0a14c7832263600476fd6ff4c5cb0a86080d0e5f48b212220a20ebe80b7cadea277ac05fb85c7164fe15ebd6873c4a74b3296a462a1026fd9b0f18321864").to_vec();
	let raw_signed_header = hex_literal::hex!("0a9c010a02080b120a746573742d636861696e1802220c08abc49a930610a8f39fc1014220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf984a20e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf985220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf987214a6e7b6810df8120580f2a81710e228f454f99c9712a202080210011a480a20afc35ec1d9620052c6d71122cb5504ee68802184023a217547ca2248df902fbb122408011220afc35ec1d9620052c6d71122cb5504ee68802184023a217547ca2248df902fbb226808021214a6e7b6810df8120580f2a81710e228f454f99c971a0c08abc49a930610a8f39fc1012240a91380fe3cde0147994b82a0b00b28bd82870df38b2cad5b4ba25c9a4c833cd50f3143ffaa4e924eccd143639fb3decf6b94570aff2c50f1346e88d06555fd0d226808021214c7832263600476fd6ff4c5cb0a86080d0e5f48b21a0c08abc49a930610a8f39fc10122407e2e349d9a0adfc3564654fcf88d328cf50a13179cc5ddaf87dd0e1abd4b45685312def0affbae29e8b7882af3b76b056f81b701bb2e43769fb63fe3696b090f").to_vec();
	let signed_header = SignedHeader::decode_vec(&*raw_signed_header).unwrap();

	let validator_set = ValidatorSet::decode_vec(&*raw_validator_set).unwrap();
	let header = Header {
		signed_header,
		validator_set: validator_set.clone(),
		trusted_height: Height::new(0, 1),
		trusted_validator_set: validator_set,
	};

	MsgUpdateAnyClient {
		client_id: ClientId::new(ClientType::Tendermint, 0).unwrap(),
		header: AnyHeader::Tendermint(header),
		signer: Signer::new("relayer"),
	}
}

benchmarks! {
	where_clause {
		where u32: From<<T as frame_system::Config>::BlockNumber>,
				T: Send + Sync + pallet_timestamp::Config<Moment = u64>,
	}
	// create_client
	create_client {
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let msg = MsgCreateAnyClient::new(
			AnyClientState::Tendermint(mock_client_state),
			AnyConsensusState::Tendermint(mock_cs_state),
			Signer::new("relayer"),
		)
		.unwrap()
		.encode_vec()
		.unwrap();

		let msg = Any { type_url: TYPE_URL.to_string().as_bytes().to_vec(), value: msg };
		let caller: T::AccountId = whitelisted_caller();

	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(Clients::<T>::count(), 1)
	}

	// update_client
	update_client {
		let mut ctx = routing::Context::<T>::new();
		let now: <T as pallet_timestamp::Config>::Moment = 1650894363u64.saturating_mul(1000);
		pallet_timestamp::Pallet::<T>::set_timestamp(now);
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let value = create_client_update().encode_vec().unwrap();

		let msg = Any { type_url: UPDATE_CLIENT_TYPE_URL.to_string().as_bytes().to_vec(), value };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_last_event::<T>(Event::<T>::ProcessedIBCMessages.into());
		let client_state = ClientStates::<T>::get(client_id.as_bytes().to_vec());
		let client_state = AnyClientState::decode_vec(&*client_state).unwrap();
		assert_eq!(client_state.latest_height(), Height::new(0, 2));
	}

	// create connection
	connection_init {
		let mut ctx = routing::Context::<T>::new();
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let counterparty = Counterparty::new(counterparty_client_id, None, commitment_prefix);
		let delay_period = core::time::Duration::from_nanos(1000);

		let value = conn_open_init::MsgConnectionOpenInit {
			client_id: client_id.clone(),
			counterparty,
			version: None,
			delay_period,
			signer: Signer::new("relayer")
		}.encode_vec().unwrap();

		let msg = Any { type_url: conn_open_init::TYPE_URL.as_bytes().to_vec(), value };
		let caller: T::AccountId = whitelisted_caller();
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(ConnectionClient::<T>::get(client_id.as_bytes().to_vec()).len(), 1);
		assert_last_event::<T>(Event::<T>::ProcessedIBCMessages.into())
	}

	// create channel
	create_channel {
		let mut ctx = routing::Context::<T>::new();
		let (mock_client_state, mock_cs_state) = create_mock_state();
		let mock_client_state = AnyClientState::Tendermint(mock_client_state);
		let mock_cs_state = AnyConsensusState::Tendermint(mock_cs_state);
		let client_id = ClientId::new(mock_client_state.client_type(), 0).unwrap();
		let counterparty_client_id = ClientId::new(mock_client_state.client_type(), 1).unwrap();
		ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
		ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
		ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

		let connection_id = ConnectionId::new(0);
		let commitment_prefix: CommitmentPrefix = "ibc".as_bytes().to_vec().try_into().unwrap();
		let delay_period = core::time::Duration::from_nanos(1000);
		let connection_counterparty = Counterparty::new(counterparty_client_id, Some(ConnectionId::new(1)), commitment_prefix);
		let connection_end = ConnectionEnd::new(State::Open, client_id.clone(), connection_counterparty, vec![ConnVersion::default()], delay_period);

		ctx.store_connection(connection_id.clone(), &connection_end).unwrap();
		ctx.store_connection_to_client(connection_id, &client_id).unwrap();

		let counterparty_channel = ibc::core::ics04_channel::channel::Counterparty::new(PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap(), None);
		let channel_end = ChannelEnd::new(
			ibc::core::ics04_channel::channel::State::Init,
			ibc::core::ics04_channel::channel::Order::Unordered,
			counterparty_channel,
			vec![ConnectionId::new(0)],
			ibc::core::ics04_channel::Version::default()
		);
		let port_id = PortId::from_str(pallet_ibc_ping::PORT_ID).unwrap();
		let value = MsgChannelOpenInit {
			port_id: port_id.clone(),
			channel: channel_end,
			signer: Signer::new("relayer")
		}.encode_vec().unwrap();

		let capability = PalletIbc::<T>::bind_port(port_id).unwrap();
		pallet_ibc_ping::Pallet::<T>::set_capability(capability.index());

		let caller: T::AccountId = whitelisted_caller();
		let msg = Any { type_url: CHAN_OPEN_TYPE_URL.as_bytes().to_vec(), value };
	}: deliver(RawOrigin::Signed(caller), vec![msg])
	verify {
		assert_eq!(ChannelCounter::<T>::get(), 1);
	}
}

impl_benchmark_test_suite!(PalletIbc, crate::mock::new_test_ext(), crate::mock::Test,);
