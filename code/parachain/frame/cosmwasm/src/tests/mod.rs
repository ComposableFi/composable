use crate::{ibc::Router, mock::*, Event, Pallet};
use composable_tests_helpers::test::helper::RuntimeTrait;
use core::str::FromStr;
use cosmwasm_vm::system::CUSTOM_CONTRACT_EVENT_PREFIX;
use frame_support::{assert_ok, BoundedVec};
use ibc::core::{
	ics03_connection::context::ConnectionReader,
	ics04_channel::{
		channel::{Counterparty, Order},
		context::ChannelReader,
		packet::Packet,
	},
	ics24_host::identifier::{ConnectionId, PortId},
	ics26_routing::context::{ModuleCallbackContext, ModuleId, ModuleOutputBuilder},
};
use pallet_ibc::{routing::ModuleRouter, Signer};

mod crypto;
mod extrinsics;
mod helpers;
mod host_functions;
mod submessages;

const COMMON_SALT: &[u8] = b"common-salt";
const COMMON_LABEL: &str = "common-label";

const COMMON_AMOUNT_1: u128 = 2328472;
const COMMON_AMOUNT_2: u128 = 1237242;

fn make_event_type(custom_event_name: &str) -> Vec<u8> {
	format!("{CUSTOM_CONTRACT_EVENT_PREFIX}{custom_event_name}").into()
}

#[test]
fn pallet_contracts_hook_execute() {
	new_test_ext().execute_with(|| {
		// This tests shows two pallets with contract hooks that currently exhibit the same
		// behavior. The behavior does not need to be identical in practice.

		// The first pallet with a contract hook
		System::set_block_number(0xDEADBEEF);
		let depth = 10;
		assert_ok!(Cosmwasm::execute(
			RuntimeOrigin::signed(MOCK_PALLET_ACCOUNT_ID_1),
			MOCK_PALLET_CONTRACT_ADDRESS_1,
			Default::default(),
			100_000_000_000_000u64,
			BoundedVec::truncate_from(vec![depth])
		),);
		let expected_event_contract = MOCK_PALLET_CONTRACT_ADDRESS_1;
		let expected_event_ty = make_event_type(MOCK_CONTRACT_EVENT_TYPE_1);
		assert_eq!(
			Test::assert_event_with(|event: Event<Test>| match event {
				Event::Emitted { contract, ty, .. }
					if contract == expected_event_contract && ty == expected_event_ty =>
					Some(()),
				_ => None,
			})
			.count(),
			// recursive, should call himself until depth reach 0
			1 + depth as usize
		);

		// The second pallet with a contract hook
		let depth = 20;
		assert_ok!(Cosmwasm::execute(
			RuntimeOrigin::signed(MOCK_PALLET_ACCOUNT_ID_2),
			MOCK_PALLET_CONTRACT_ADDRESS_2,
			Default::default(),
			100_000_000_000_000u64,
			BoundedVec::truncate_from(vec![depth])
		),);
		let expected_event_contract = MOCK_PALLET_CONTRACT_ADDRESS_2;
		let expected_event_ty = make_event_type(MOCK_CONTRACT_EVENT_TYPE_2);
		assert_eq!(
			Test::assert_event_with(|event: Event<Test>| match event {
				Event::Emitted { contract, ty, .. }
					if contract == expected_event_contract && ty == expected_event_ty =>
					Some(()),
				_ => None,
			})
			.count(),
			// recursive, should call himself until depth reach 0
			1 + depth as usize
		);
	})
}

impl ConnectionReader for Test {
	fn minimum_delay_period(&self) -> core::time::Duration {
		unimplemented!()
	}

	fn connection_end(
		&self,
		_conn_id: &ibc::core::ics24_host::identifier::ConnectionId,
	) -> Result<
		ibc::core::ics03_connection::connection::ConnectionEnd,
		ibc::core::ics03_connection::error::Error,
	> {
		unimplemented!()
	}

	fn host_oldest_height(&self) -> ibc::Height {
		unimplemented!()
	}

	fn commitment_prefix(&self) -> ibc::core::ics23_commitment::commitment::CommitmentPrefix {
		unimplemented!()
	}

	fn connection_counter(&self) -> Result<u64, ibc::core::ics03_connection::error::Error> {
		unimplemented!()
	}
}

impl ChannelReader for Test {
	fn channel_end(
		&self,
		_port_channel_id: &(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
		),
	) -> Result<ibc::core::ics04_channel::channel::ChannelEnd, ibc::core::ics04_channel::error::Error>
	{
		Ok(ibc::core::ics04_channel::channel::ChannelEnd {
			remote: Counterparty { port_id: <_>::default(), channel_id: Some(<_>::default()) },
			connection_hops: [ConnectionId::default(); 1].to_vec(),
			version: <_>::default(),
			..ibc::core::ics04_channel::channel::ChannelEnd::default()
		})
	}

	fn connection_channels(
		&self,
		_cid: &ibc::core::ics24_host::identifier::ConnectionId,
	) -> Result<
		Vec<(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
		)>,
		ibc::core::ics04_channel::error::Error,
	> {
		unimplemented!()
	}

	fn get_next_sequence_send(
		&self,
		_port_channel_id: &(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
		),
	) -> Result<ibc::core::ics04_channel::packet::Sequence, ibc::core::ics04_channel::error::Error>
	{
		unimplemented!()
	}

	fn get_next_sequence_recv(
		&self,
		_port_channel_id: &(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
		),
	) -> Result<ibc::core::ics04_channel::packet::Sequence, ibc::core::ics04_channel::error::Error>
	{
		unimplemented!()
	}

	fn get_next_sequence_ack(
		&self,
		_port_channel_id: &(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
		),
	) -> Result<ibc::core::ics04_channel::packet::Sequence, ibc::core::ics04_channel::error::Error>
	{
		unimplemented!()
	}

	fn get_packet_commitment(
		&self,
		_key: &(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics04_channel::packet::Sequence,
		),
	) -> Result<
		ibc::core::ics04_channel::commitment::PacketCommitment,
		ibc::core::ics04_channel::error::Error,
	> {
		unimplemented!()
	}

	fn get_packet_receipt(
		&self,
		_key: &(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics04_channel::packet::Sequence,
		),
	) -> Result<ibc::core::ics04_channel::packet::Receipt, ibc::core::ics04_channel::error::Error> {
		unimplemented!()
	}

	fn get_packet_acknowledgement(
		&self,
		_key: &(
			ibc::core::ics24_host::identifier::PortId,
			ibc::core::ics24_host::identifier::ChannelId,
			ibc::core::ics04_channel::packet::Sequence,
		),
	) -> Result<
		ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
		ibc::core::ics04_channel::error::Error,
	> {
		unimplemented!()
	}

	fn hash(&self, _value: Vec<u8>) -> Vec<u8> {
		unimplemented!()
	}

	fn client_update_time(
		&self,
		_client_id: &ibc::core::ics24_host::identifier::ClientId,
		_height: ibc::Height,
	) -> Result<ibc::timestamp::Timestamp, ibc::core::ics04_channel::error::Error> {
		unimplemented!()
	}

	fn client_update_height(
		&self,
		_client_id: &ibc::core::ics24_host::identifier::ClientId,
		_height: ibc::Height,
	) -> Result<ibc::Height, ibc::core::ics04_channel::error::Error> {
		unimplemented!()
	}

	fn channel_counter(&self) -> Result<u64, ibc::core::ics04_channel::error::Error> {
		unimplemented!()
	}

	fn max_expected_time_per_block(&self) -> core::time::Duration {
		unimplemented!()
	}
}
impl ModuleCallbackContext for Test {}

#[test]
fn open_channel_ceremony() {
	new_test_ext().execute_with(|| {
		let mut ibc = Router::<Test>::default();
		let module_id = ModuleId::from_str("cosmwasm").unwrap();
		let ibc = ibc.get_route_mut(&module_id).unwrap();
		assert_ok!(ibc.on_chan_open_init(
			&Test::default(),
			&mut ModuleOutputBuilder::new(),
			Order::Ordered,
			&[ConnectionId::new(42)],
			&PortId::from_str(
				format!(
					"wasm.{}",
					Pallet::<Test>::account_to_cosmwasm_addr(MOCK_PALLET_IBC_CONTRACT_ADDRESS)
				)
				.as_str(),
			)
			.unwrap(),
			&<_>::default(),
			&Counterparty { port_id: <_>::default(), channel_id: Some(<_>::default()) },
			&<_>::default(),
			&Signer::from_str("42").unwrap(),
		));
	});
}

#[test]
fn ibc_calls_and_callbacks() {
	new_test_ext().execute_with(|| {
		let mut ibc = Router::<Test>::default();
		let module_id = ModuleId::from_str("cosmwasm").unwrap();
		let ibc = ibc.get_route_mut(&module_id).unwrap();
		let ctx = &Test::default();
		let output = &mut ModuleOutputBuilder::new();
		let relayer = &Signer::from_str("42").unwrap();
		ibc.on_chan_open_init(
			ctx,
			output,
			Order::Ordered,
			&[ConnectionId::new(42)],
			&PortId::from_str(
				format!(
					"wasm.{}",
					Pallet::<Test>::account_to_cosmwasm_addr(MOCK_PALLET_IBC_CONTRACT_ADDRESS)
				)
				.as_str(),
			)
			.unwrap(),
			&<_>::default(),
			&Counterparty { port_id: <_>::default(), channel_id: Some(<_>::default()) },
			&<_>::default(),
			&Signer::from_str("42").unwrap(),
		)
		.unwrap();

		let port_id = &PortId::from_str(
			format!(
				"wasm.{}",
				Pallet::<Test>::account_to_cosmwasm_addr(MOCK_PALLET_IBC_CONTRACT_ADDRESS)
			)
			.as_str(),
		)
		.unwrap();

		assert_ok!(ibc.on_chan_open_try(
			&Test::default(),
			&mut ModuleOutputBuilder::new(),
			Order::Ordered,
			&[ConnectionId::new(42)],
			port_id,
			&<_>::default(),
			&Counterparty { port_id: <_>::default(), channel_id: Some(<_>::default()) },
			&<_>::default(),
			&<_>::default(),
			&Signer::from_str("42").unwrap(),
		));

		assert_ok!(ibc.on_chan_open_confirm(ctx, output, port_id, &<_>::default(), relayer));

		assert_ok!(ibc.on_recv_packet(
			ctx,
			output,
			&Packet {
				sequence: 42.into(),
				source_port: port_id.clone(),
				source_channel: <_>::default(),
				destination_port: port_id.clone(),
				destination_channel: <_>::default(),
				data: [42; 1].to_vec(),
				..Packet::default()
			},
			relayer,
		));
	});
}
