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

mod precompiled {
	use super::*;
	use cosmwasm_vm::system::CUSTOM_CONTRACT_EVENT_PREFIX;
	use frame_support::{assert_ok, BoundedVec};

	fn make_event_type(custom_event_name: &str) -> Vec<u8> {
		format!("{CUSTOM_CONTRACT_EVENT_PREFIX}{custom_event_name}").into()
	}

	#[test]
	fn hook_execute() {
		new_test_ext().execute_with(|| {
			System::set_block_number(0xDEADBEEF);
			let depth = 10;
			assert_ok!(Cosmwasm::execute(
				Origin::signed(ALICE),
				MOCK_CONTRACT_ADDRESS,
				Default::default(),
				100_000_000_000_000u64,
				BoundedVec::truncate_from(vec![depth])
			),);
			let expected_event_contract = MOCK_CONTRACT_ADDRESS;
			let expected_event_ty = make_event_type(MOCK_CONTRACT_EVENT_TY);
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
}
