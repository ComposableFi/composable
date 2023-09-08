use crate::{Config, Pallet, SUBSTRATE_ECDSA_SIGNATURE_LEN};
use sp_std::{vec, vec::Vec};

use sp_core::{ecdsa, ed25519};
impl<T: Config> Pallet<T> {
	pub(crate) fn do_secp256k1_recover_pubkey(
		message_hash: &[u8],
		signature: &[u8],
		recovery_param: u8,
	) -> Result<Vec<u8>, ()> {
		// `recovery_param` must be 0 or 1. Other values are not supported from CosmWasm.
		if recovery_param >= 2 {
			return Err(())
		}

		if signature.len() != SUBSTRATE_ECDSA_SIGNATURE_LEN - 1 {
			return Err(())
		}

		// Try into a [u8; 32]
		let message_hash = message_hash.try_into().map_err(|_| ())?;

		let signature = {
			// Since we fill `signature_inner` with `recovery_param`, when 64 bytes are written
			// the final byte will be the `recovery_param`.
			let mut signature_inner = [recovery_param; SUBSTRATE_ECDSA_SIGNATURE_LEN];
			signature_inner[..SUBSTRATE_ECDSA_SIGNATURE_LEN - 1].copy_from_slice(signature);
			signature_inner
		};

		sp_io::crypto::secp256k1_ecdsa_recover(&signature, &message_hash)
			.map(|without_tag| {
				let mut with_tag = vec![0x04_u8];
				with_tag.extend_from_slice(&without_tag[..]);
				with_tag
			})
			.map_err(|_| ())
	}

	pub(crate) fn do_secp256k1_verify(
		message_hash: &[u8],
		signature: &[u8],
		public_key: &[u8],
	) -> bool {
		let message_hash = match message_hash.try_into() {
			Ok(message_hash) => message_hash,
			Err(_) => return false,
		};

		// We are expecting 64 bytes long public keys but the substrate function use an
		// additional byte for recovery id. So we insert a dummy byte.
		let signature = {
			let mut signature_inner = [0_u8; SUBSTRATE_ECDSA_SIGNATURE_LEN];
			signature_inner[..SUBSTRATE_ECDSA_SIGNATURE_LEN - 1].copy_from_slice(signature);
			ecdsa::Signature(signature_inner)
		};

		let public_key = match libsecp256k1::PublicKey::parse_slice(public_key, None) {
			Ok(public_key) => ecdsa::Public::from_raw(public_key.serialize_compressed()),
			Err(_) => return false,
		};

		sp_io::crypto::ecdsa_verify_prehashed(&signature, &message_hash, &public_key)
	}

	pub(crate) fn do_ed25519_batch_verify(
		messages: &[&[u8]],
		signatures: &[&[u8]],
		public_keys: &[&[u8]],
	) -> bool {
		// https://substrate.stackexchange.com/questions/9754/sp-iocryptoed25519-batch-verify-was-removed-amid-polkadot-0-9-39-and-0-9-4

		let mut messages = messages.to_vec();
		let mut public_keys = public_keys.to_vec();

		if messages.len() == signatures.len() && messages.len() == public_keys.len() {
			// Nothing needs to be done
		} else if messages.len() == 1 && signatures.len() == public_keys.len() {
			// There can be a single message signed with different signature-public key pairs
			messages = messages.repeat(signatures.len());
		} else if public_keys.len() == 1 && messages.len() == signatures.len() {
			// Single entity(with a public key) might wanna verify different messages
			public_keys = public_keys.repeat(signatures.len());
		} else {
			// Any other case is wrong
			return false
		}

		messages
			.iter()
			.zip(signatures)
			.zip(public_keys)
			.all(|((m, s), p)| Self::do_ed25519_verify(m, s, p))
	}

	pub(crate) fn do_ed25519_verify(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
		let signature: ed25519::Signature = match signature.try_into() {
			Ok(signature) => signature,
			Err(_) => return false,
		};

		let public_key: ed25519::Public = match public_key.try_into() {
			Ok(public_key) => public_key,
			Err(_) => return false,
		};

		sp_io::crypto::ed25519_verify(&signature, message, &public_key)
	}
}
