#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]

//! ICS 07: Tendermint Client implements a client verification algorithm for blockchains which use
//! the Tendermint consensus algorithm.

#[macro_use]
#[cfg(any(test, feature = "mocks"))]
extern crate serde;
#[cfg(any(test, feature = "mocks"))]
#[macro_use]
extern crate ibc_derive;
extern crate alloc;

use core::fmt::Debug;

pub mod client_def;
pub mod client_message;
pub mod client_state;
pub mod consensus_state;
pub mod error;
mod merkle;
#[cfg(any(test, feature = "mocks"))]
pub mod mock;
#[cfg(any(test, feature = "mocks"))]
mod query;

/// Host functions that allow the light client verify cryptographic proofs in native.
pub trait HostFunctionsProvider:
	ics23::HostFunctionsProvider
	+ tendermint_light_client_verifier::host_functions::HostFunctionsProvider
	+ Debug
	+ Clone
	+ Send
	+ Sync
	+ Default
	+ Eq
{
}

#[cfg(test)]
mod tests {
	use crate::client_state::{
		test_util::get_dummy_tendermint_client_state, ClientState as TendermintClientState,
		ClientState,
	};

	use crate::{
		client_message::test_util::{get_dummy_ics07_header, get_dummy_tendermint_header},
		mock::{AnyClientState, AnyConsensusState, MockClientTypes},
	};

	use crate::{client_message::ClientMessage, mock::AnyClientMessage};
	use ibc::{
		core::{
			ics02_client::{
				context::ClientReader,
				handler::{dispatch, ClientResult},
				msgs::{
					create_client::MsgCreateAnyClient, update_client::MsgUpdateAnyClient, ClientMsg,
				},
				trust_threshold::TrustThreshold,
			},
			ics23_commitment::specs::ProofSpecs,
			ics24_host::identifier::ClientId,
		},
		events::IbcEvent,
		handler::HandlerOutput,
		mock::context::MockContext,
		prelude::*,
		test_utils::get_dummy_account_id,
		Height,
	};
	use ibc_proto::ibc::core::client::v1::{MsgCreateClient, MsgUpdateClient};
	use std::time::Duration;
	use test_log::test;

	#[test]
	fn msg_create_client_serialization() {
		let signer = get_dummy_account_id();

		let tm_header = get_dummy_tendermint_header();
		let tm_client_state = get_dummy_tendermint_client_state(tm_header.clone());

		let msg = MsgCreateAnyClient::<MockContext<MockClientTypes>>::new(
			tm_client_state,
			AnyConsensusState::Tendermint(tm_header.try_into().unwrap()),
			signer,
		)
		.unwrap();

		let raw = MsgCreateClient::from(msg.clone());
		let msg_back =
			MsgCreateAnyClient::<MockContext<MockClientTypes>>::try_from(raw.clone()).unwrap();
		let raw_back = MsgCreateClient::from(msg_back.clone());
		assert_eq!(msg, msg_back);
		assert_eq!(raw, raw_back);
	}

	#[test]
	fn test_tm_create_client_ok() {
		let signer = get_dummy_account_id();

		let ctx = MockContext::default();

		let tm_header = get_dummy_tendermint_header();

		let tm_client_state = AnyClientState::Tendermint(
			TendermintClientState::new(
				tm_header.chain_id.clone().into(),
				TrustThreshold::ONE_THIRD,
				Duration::from_secs(64000),
				Duration::from_secs(128000),
				Duration::from_millis(3000),
				Height::new(0, u64::from(tm_header.height)),
				ProofSpecs::default(),
				vec!["".to_string()],
			)
			.unwrap(),
		);

		let msg = MsgCreateAnyClient::<MockContext<MockClientTypes>>::new(
			tm_client_state,
			AnyConsensusState::Tendermint(tm_header.try_into().unwrap()),
			signer,
		)
		.unwrap();

		let output = dispatch(&ctx, ClientMsg::CreateClient(msg.clone()));

		match output {
			Ok(HandlerOutput { result, mut events, .. }) => {
				assert_eq!(events.len(), 1);
				let event = events.pop().unwrap();
				let expected_client_id =
					ClientId::new(&ClientState::<()>::client_type(), 0).unwrap();
				assert!(
					matches!(event, IbcEvent::CreateClient(ref e) if e.client_id() == &expected_client_id)
				);
				assert_eq!(event.height(), ctx.host_height());
				match result {
					ClientResult::Create(create_res) => {
						assert_eq!(create_res.client_type, ClientState::<()>::client_type());
						assert_eq!(create_res.client_id, expected_client_id);
						assert_eq!(create_res.client_state, msg.client_state);
						assert_eq!(create_res.consensus_state, msg.consensus_state);
					},
					_ => {
						panic!("expected result of type ClientResult::CreateResult");
					},
				}
			},
			Err(err) => {
				panic!("unexpected error: {}", err);
			},
		}
	}

	#[test]
	fn msg_update_client_serialization() {
		let client_id: ClientId = "tendermint".parse().unwrap();
		let signer = get_dummy_account_id();

		let header = get_dummy_ics07_header();

		let msg = MsgUpdateAnyClient::<MockContext<MockClientTypes>>::new(
			client_id,
			AnyClientMessage::Tendermint(ClientMessage::Header(header)),
			signer,
		);
		let raw = MsgUpdateClient::from(msg.clone());
		let msg_back = MsgUpdateAnyClient::try_from(raw.clone()).unwrap();
		let raw_back = MsgUpdateClient::from(msg_back.clone());
		assert_eq!(msg, msg_back);
		assert_eq!(raw, raw_back);
	}
}
