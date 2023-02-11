use crate::{mock::*, Event};
use composable_tests_helpers::test::helper::RuntimeTrait;
use sha2::{Digest, Sha256};
use sha3::Keccak256;

// took these from: https://github.com/CosmWasm/cosmwasm/blob/main/contracts/crypto-verify/tests/integration.rs
const SECP256K1_MESSAGE_HEX: &str = "5c868fedb8026979ebd26f1ba07c27eedf4ff6d10443505a96ecaf21ba8c4f0937b3cd23ffdc3dd429d4cd1905fb8dbcceeff1350020e18b58d2ba70887baa3a9b783ad30d3fbf210331cdd7df8d77defa398cdacdfc2e359c7ba4cae46bb74401deb417f8b912a1aa966aeeba9c39c7dd22479ae2b30719dca2f2206c5eb4b7";
const SECP256K1_SIGNATURE_HEX: &str = "207082eb2c3dfa0b454e0906051270ba4074ac93760ba9e7110cd9471475111151eb0dbbc9920e72146fb564f99d039802bf6ef2561446eb126ef364d21ee9c4";
const SECP256K1_PUBLIC_KEY_HEX: &str = "04051c1ee2190ecfb174bfe4f90763f2b4ff7517b70a2aec1876ebcfd644c4633fb03f3cfbd94b1f376e34592d9d41ccaf640bb751b00a1fadeb0c01157769eb73";

// TEST 3 test vector from https://tools.ietf.org/html/rfc8032#section-7.1
const ED25519_MESSAGE_HEX: &str = "af82";
const ED25519_SIGNATURE_HEX: &str = "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a";
const ED25519_PUBLIC_KEY_HEX: &str =
	"fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025";

// Signed text "connect all the things" using MyEtherWallet with private key
// b5b1870957d373ef0eeffecc6e4812c0fd08f554b37b233526acc331bf1544f7
const ETHEREUM_MESSAGE: &str = "connect all the things";
const ETHEREUM_SIGNATURE_HEX: &str = "dada130255a447ecf434a2df9193e6fbba663e4546c35c075cd6eea21d8c7cb1714b9b65a4f7f604ff6aad55fba73f8c36514a512bbbba03709b37069194f8a41b";
const ETHEREUM_SIGNER_ADDRESS: &str = "0x12890D2cce102216644c59daE5baed380d84830c";

// TEST 2 test vector from https://tools.ietf.org/html/rfc8032#section-7.1
const ED25519_MESSAGE2_HEX: &str = "72";
const ED25519_SIGNATURE2_HEX: &str = "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00";
const ED25519_PUBLIC_KEY2_HEX: &str =
	"3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c";

#[test]
fn works() {
	new_test_ext().execute_with(|| {})
}

#[test]
fn secp256k1_verify_verifies() {
	new_test_ext().execute_with(|| {
		let message = hex::decode(SECP256K1_MESSAGE_HEX).unwrap();
		let signature = hex::decode(SECP256K1_SIGNATURE_HEX).unwrap();
		let public_key = hex::decode(SECP256K1_PUBLIC_KEY_HEX).unwrap();
		let hash = Sha256::digest(message);

		assert!(Cosmwasm::do_secp256k1_verify(&hash, &signature, &public_key))
	})
}

#[test]
fn secp256k1_verify_fails() {
	new_test_ext().execute_with(|| {
		let message = hex::decode(SECP256K1_MESSAGE_HEX).unwrap();
		let mut signature = hex::decode(SECP256K1_SIGNATURE_HEX).unwrap();
		let public_key = hex::decode(SECP256K1_PUBLIC_KEY_HEX).unwrap();
		let hash = Sha256::digest(message);

		*signature.last_mut().unwrap() += 1;

		assert!(!Cosmwasm::do_secp256k1_verify(&hash, &signature, &public_key))
	})
}

#[test]
fn secp256k1_recover_pubkey_works() {
	new_test_ext().execute_with(|| {
		let mut hasher = Keccak256::new();
		hasher.update(format!("\x19Ethereum Signed Message:\n{}", ETHEREUM_MESSAGE.len()));
		hasher.update(ETHEREUM_MESSAGE);
		let message_hash = hasher.finalize();
		let signature = hex::decode(ETHEREUM_SIGNATURE_HEX).unwrap();
		let signer_address = hex::decode(&ETHEREUM_SIGNER_ADDRESS[2..]).unwrap();

		let (recovery, signature) = signature.split_last().unwrap();

		let recovered_pubkey =
			Cosmwasm::do_secp256k1_recover_pubkey(&message_hash, signature, *recovery - 27)
				.unwrap();
		let recovered_pubkey_hash = Keccak256::digest(&recovered_pubkey[1..]);

		assert_eq!(signer_address, recovered_pubkey_hash[recovered_pubkey_hash.len() - 20..]);
	})
}

#[test]
fn ed25519_verify_verifies() {
	new_test_ext().execute_with(|| {
		let message = hex::decode(ED25519_MESSAGE_HEX).unwrap();
		let signature = hex::decode(ED25519_SIGNATURE_HEX).unwrap();
		let public_key = hex::decode(ED25519_PUBLIC_KEY_HEX).unwrap();

		assert!(Cosmwasm::do_ed25519_verify(&message, &signature, &public_key));
	})
}

#[test]
fn ed25519_verify_fails() {
	new_test_ext().execute_with(|| {
		let message = hex::decode(ED25519_MESSAGE_HEX).unwrap();
		let mut signature = hex::decode(ED25519_SIGNATURE_HEX).unwrap();
		let public_key = hex::decode(ED25519_PUBLIC_KEY_HEX).unwrap();

		*signature.last_mut().unwrap() += 1;

		assert!(!Cosmwasm::do_ed25519_verify(&message, &signature, &public_key));
	})
}

#[test]
fn ed25519_batch_verify_verifies() {
	new_test_ext().execute_with(|| {
		let decode = |m| -> Vec<u8> { hex::decode(m).unwrap() };

		let messages: Vec<Vec<u8>> =
			[ED25519_MESSAGE_HEX, ED25519_MESSAGE2_HEX].iter().map(decode).collect();
		let signatures: Vec<Vec<u8>> =
			[ED25519_SIGNATURE_HEX, ED25519_SIGNATURE2_HEX].iter().map(decode).collect();
		let public_keys: Vec<Vec<u8>> =
			[ED25519_PUBLIC_KEY_HEX, ED25519_PUBLIC_KEY2_HEX].iter().map(decode).collect();

		let ref_messages: Vec<&[u8]> = messages.iter().map(|b| b.as_slice()).collect();
		let ref_signatures: Vec<&[u8]> = signatures.iter().map(|b| b.as_slice()).collect();
		let ref_public_keys: Vec<&[u8]> = public_keys.iter().map(|b| b.as_slice()).collect();

		assert!(Cosmwasm::do_ed25519_batch_verify(
			&ref_messages,
			&ref_signatures,
			&ref_public_keys
		));
	})
}

#[test]
fn ed25519_batch_verify_verifies_multisig() {
	new_test_ext().execute_with(|| {
		let decode = |m| -> Vec<u8> { hex::decode(m).unwrap() };

		let messages: Vec<Vec<u8>> = [ED25519_MESSAGE_HEX].iter().map(decode).collect();
		let signatures: Vec<Vec<u8>> =
			[ED25519_SIGNATURE_HEX, ED25519_SIGNATURE_HEX].iter().map(decode).collect();
		let public_keys: Vec<Vec<u8>> =
			[ED25519_PUBLIC_KEY_HEX, ED25519_PUBLIC_KEY_HEX].iter().map(decode).collect();

		let ref_messages: Vec<&[u8]> = messages.iter().map(|b| b.as_slice()).collect();
		let ref_signatures: Vec<&[u8]> = signatures.iter().map(|b| b.as_slice()).collect();
		let ref_public_keys: Vec<&[u8]> = public_keys.iter().map(|b| b.as_slice()).collect();

		assert!(Cosmwasm::do_ed25519_batch_verify(
			&ref_messages,
			&ref_signatures,
			&ref_public_keys
		));
	})
}

#[test]
fn ed25519_batch_verify_verifies_with_single_pubkey_multi_msg() {
	new_test_ext().execute_with(|| {
		let decode = |m| -> Vec<u8> { hex::decode(m).unwrap() };

		let messages: Vec<Vec<u8>> =
			[ED25519_MESSAGE_HEX, ED25519_MESSAGE_HEX].iter().map(decode).collect();
		let signatures: Vec<Vec<u8>> =
			[ED25519_SIGNATURE_HEX, ED25519_SIGNATURE_HEX].iter().map(decode).collect();
		let public_keys: Vec<Vec<u8>> = [ED25519_PUBLIC_KEY_HEX].iter().map(decode).collect();

		let ref_messages: Vec<&[u8]> = messages.iter().map(|b| b.as_slice()).collect();
		let ref_signatures: Vec<&[u8]> = signatures.iter().map(|b| b.as_slice()).collect();
		let ref_public_keys: Vec<&[u8]> = public_keys.iter().map(|b| b.as_slice()).collect();

		assert!(Cosmwasm::do_ed25519_batch_verify(
			&ref_messages,
			&ref_signatures,
			&ref_public_keys
		));
	})
}

#[test]
fn ed25519_batch_verify_fails_if_one_fail() {
	new_test_ext().execute_with(|| {
		let decode = |m| -> Vec<u8> { hex::decode(m).unwrap() };

		let messages: Vec<Vec<u8>> =
			[ED25519_MESSAGE_HEX, ED25519_MESSAGE2_HEX].iter().map(decode).collect();
		let mut signatures: Vec<Vec<u8>> =
			[ED25519_SIGNATURE_HEX, ED25519_SIGNATURE2_HEX].iter().map(decode).collect();
		let public_keys: Vec<Vec<u8>> =
			[ED25519_PUBLIC_KEY_HEX, ED25519_PUBLIC_KEY2_HEX].iter().map(decode).collect();

		*signatures.last_mut().unwrap().last_mut().unwrap() += 1;

		let ref_messages: Vec<&[u8]> = messages.iter().map(|b| b.as_slice()).collect();
		let ref_signatures: Vec<&[u8]> = signatures.iter().map(|b| b.as_slice()).collect();
		let ref_public_keys: Vec<&[u8]> = public_keys.iter().map(|b| b.as_slice()).collect();

		assert!(!Cosmwasm::do_ed25519_batch_verify(
			&ref_messages,
			&ref_signatures,
			&ref_public_keys
		));
	})
}

#[test]
fn ed25519_batch_verify_fails_if_input_lengths_are_incorrect() {
	new_test_ext().execute_with(|| {
		let decode = |m| -> Vec<u8> { hex::decode(m).unwrap() };

		let messages: Vec<Vec<u8>> =
			[ED25519_MESSAGE_HEX, ED25519_MESSAGE2_HEX].iter().map(decode).collect();
		let signatures: Vec<Vec<u8>> = [ED25519_SIGNATURE_HEX].iter().map(decode).collect();
		let public_keys: Vec<Vec<u8>> =
			[ED25519_PUBLIC_KEY_HEX, ED25519_PUBLIC_KEY2_HEX].iter().map(decode).collect();

		let ref_messages: Vec<&[u8]> = messages.iter().map(|b| b.as_slice()).collect();
		let ref_signatures: Vec<&[u8]> = signatures.iter().map(|b| b.as_slice()).collect();
		let ref_public_keys: Vec<&[u8]> = public_keys.iter().map(|b| b.as_slice()).collect();

		assert!(!Cosmwasm::do_ed25519_batch_verify(
			&ref_messages,
			&ref_signatures,
			&ref_public_keys
		));
	})
}

#[test]
fn ss58_address_format_is_supported_correctly() {
	new_test_ext().execute_with(|| {
		let valid_ss58_addresses = [
			(
				"5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL",
				"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
			),
			(
				"5txRkPpGeTRJyZ96t5aSxLKaQDa32ZY21rq8MDHaN7dLGCBe",
				"10dbdfc9a706a4cf96b9e9dfb25384a2cf25faeaddabd4c98079f8360bc4ad46",
			),
			(
				"5uawZPfyfP9hdowPJbeiR2GMSZatLq3b9wpWc6yWjSLeakgh",
				"2cb50f2480175397eb320e637fc56be1939e18fb2b326eab5fdeaad9d43ffc74",
			),
			(
				"5umjqLRoE5wrXUGyedwbATZjj1SukRC9eh8qGJPpVx47bUam",
				"34f149d3a32ff2afe4daee3f4c917b90a73b88ee84a2666b477cdd67d6c5d17b",
			),
		];
		for (ss58_addr, hex_addr) in valid_ss58_addresses {
			// ss58 string to AccountId works
			let lhs = Cosmwasm::cosmwasm_addr_to_account(ss58_addr.into()).unwrap();
			// address binary to canonical AccountId works
			let binary_addr = hex::decode(hex_addr).unwrap();
			let rhs = Cosmwasm::canonical_addr_to_account(binary_addr.into()).unwrap();
			assert_eq!(lhs, rhs);
		}

		let not_valid_ss58_addresses = [
			// length is correct but with some garbage string
			"5yasdX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL",
			// total garbage
			"someaddr",
		];

		for garbage_addr in not_valid_ss58_addresses {
			assert!(Cosmwasm::cosmwasm_addr_to_account(garbage_addr.into()).is_err());
		}
	})
}

mod pallet_contracts {
	use core::str::FromStr;

	use crate::{ibc::Router, Pallet};

	use super::*;
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
		) -> Result<
			ibc::core::ics04_channel::channel::ChannelEnd,
			ibc::core::ics04_channel::error::Error,
		> {
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
		) -> Result<
			ibc::core::ics04_channel::packet::Sequence,
			ibc::core::ics04_channel::error::Error,
		> {
			unimplemented!()
		}

		fn get_next_sequence_recv(
			&self,
			_port_channel_id: &(
				ibc::core::ics24_host::identifier::PortId,
				ibc::core::ics24_host::identifier::ChannelId,
			),
		) -> Result<
			ibc::core::ics04_channel::packet::Sequence,
			ibc::core::ics04_channel::error::Error,
		> {
			unimplemented!()
		}

		fn get_next_sequence_ack(
			&self,
			_port_channel_id: &(
				ibc::core::ics24_host::identifier::PortId,
				ibc::core::ics24_host::identifier::ChannelId,
			),
		) -> Result<
			ibc::core::ics04_channel::packet::Sequence,
			ibc::core::ics04_channel::error::Error,
		> {
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
		) -> Result<ibc::core::ics04_channel::packet::Receipt, ibc::core::ics04_channel::error::Error>
		{
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
}
