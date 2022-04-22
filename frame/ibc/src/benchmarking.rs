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
			context::ClientKeeper,
			height::Height,
			msgs::create_client::{MsgCreateAnyClient, TYPE_URL},
			trust_threshold::TrustThreshold,
		},
		ics03_connection::{
			connection::Counterparty, msgs::conn_open_init, version::Version as ConnVersion,
		},
		ics04_channel::{channel::Order, msgs::chan_open_ack, Version as ChanVersion},
		ics23_commitment::{commitment::CommitmentPrefix, specs::ProofSpecs},
		ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
	},
	signer::Signer,
	timestamp::Timestamp,
};
use scale_info::prelude::string::ToString;
use sp_std::vec;
use tendermint::block::signed_header::SignedHeader;
use tendermint_proto::Protobuf;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn create_mock_state() -> (TendermintClientState, ConsensusState) {
	let mock_client_state = TendermintClientState::new(
		ChainId::from_string("test-chain"),
		TrustThreshold::ONE_THIRD,
		Duration::new(64000, 0),
		Duration::new(128000, 0),
		Duration::new(3, 0),
		Height::new(0, 1),
		ProofSpecs::default(),
		vec!["".to_string()],
		AllowUpdate { after_expiry: false, after_misbehaviour: false },
	)
	.unwrap();

	// Light signed header bytes obtained from
	// `tendermint_testgen::LightBlock::new_default_with_time_and_chain_id("test-chain".to_string(),
	// Time::now(), 1 ).generate().unwrap().signed_header.encode_vec();`
	let raw_signed_header = hex_literal::hex!("0a9b010a02080b120a746573742d636861696e1801220b08f1c58a93061090f49a7e4220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf984a20e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf985220e4d2147e1c5994daf958eafa8413706f1c75e1a2813a2cd0d32876a25d9bcf987214a6e7b6810df8120580f2a81710e228f454f99c9712a002080110011a480a203d0b60fee3c6e36443081e27090ec446d027e4768396842ae033c024f31c7d311224080112203d0b60fee3c6e36443081e27090ec446d027e4768396842ae033c024f31c7d31226708021214a6e7b6810df8120580f2a81710e228f454f99c971a0b08f1c58a93061090f49a7e224005cbb764dc0e7c657e830ed5c1204033943177841960d91307f72add36bdf2ce25427aaac6af75182bff7da1b76ad6b7df75eee6c3ae04109406900320a72a07226708021214c7832263600476fd6ff4c5cb0a86080d0e5f48b21a0b08f1c58a93061090f49a7e224009b666084a8f3893fe031fd86e441f7a08dcde82284874e202824b25b3dc6db6646c8ceec2947138cf8b428d999c9943db207eac75c9cd10a92a7c21ad15dc03").to_vec();
	let signed_header = SignedHeader::decode_vec(&*raw_signed_header).unwrap();
	let mock_cs_state =
		ibc::clients::ics07_tendermint::consensus_state::ConsensusState::from(signed_header.header);
	(mock_client_state, mock_cs_state)
}

benchmarks! {
	where_clause {
		where u32: From<<T as frame_system::Config>::BlockNumber>,
				T: Send + Sync
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
}

impl_benchmark_test_suite!(PalletIbc, crate::mock::new_test_ext(), crate::mock::Test,);
