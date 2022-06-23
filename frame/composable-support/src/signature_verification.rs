//! Functions for supporting signature verification for various chains.
//!
//! Signed messages/proofs are expected to be in the format of `{perfix}-{msg}` before they
//! are modified to fit their chains signature specifications.
use crate::types::{CosmosEcdsaSignature, CosmosPublicKey, EcdsaSignature, EthereumAddress};
use frame_support::pallet_prelude::Encode;
use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use sp_io::hashing::{keccak_256, sha2_256};
use sp_runtime::{traits::Verify, AccountId32, MultiSignature};
use sp_std::vec::Vec;

/// Verify the proof is valid for a given relay account.
///
/// Returns `false` if the verifycation process fails, returns `true` otherwise
///
/// # Associated Types
/// * `AccountId` - The `AccountId` being used by frame system
/// * `RelayChainAccountId` - The `AccountId` type being used to represent relay chain accounts
/// such as KSM accounts
pub fn verify_relay<AccountId>(
	prefix: &[u8],
	reward_account: AccountId,
	relay_account: AccountId32,
	proof: &MultiSignature,
) -> bool
where
	AccountId: Encode,
{
	// Polkadot.js wrapper tags
	const WRAPPED_PREFIX: &[u8] = b"<Bytes>";
	const WRAPPED_POSTFIX: &[u8] = b"</Bytes>";
	let mut msg = WRAPPED_PREFIX.to_vec();

	msg.append(&mut prefix.to_vec());
	msg.append(&mut reward_account.using_encoded(|x| hex::encode(x).as_bytes().to_vec()));
	msg.append(&mut WRAPPED_POSTFIX.to_vec());

	proof.verify(&msg[..], &relay_account)
}

/// Recover the public key of an `eth_sign` signature.
///
/// Requires the original message.
pub fn ethereum_recover(
	prefix: &[u8],
	msg: &[u8],
	EcdsaSignature(sig): &EcdsaSignature,
) -> Option<EthereumAddress> {
	let msg = keccak_256(&ethereum_signable_message(prefix, msg));
	let mut address = EthereumAddress::default();

	address.0.copy_from_slice(
		&keccak_256(&sp_io::crypto::secp256k1_ecdsa_recover(sig, &msg).ok()?[..])[12..],
	);

	Some(address)
}

/// Genrates a message that is compatitible with the Ethereum signing process.
///
/// Requires the original message.
pub fn ethereum_signable_message(prefix: &[u8], msg: &[u8]) -> Vec<u8> {
	let mut length = prefix.len() + msg.len();
	let mut msg_len = Vec::new();

	while length > 0 {
		msg_len.push(b'0' + (length % 10) as u8);
		length /= 10;
	}

	let mut signed_message = b"\x19Ethereum Signed Message:\n".to_vec();
	signed_message.extend(msg_len.into_iter().rev());
	signed_message.extend_from_slice(prefix);
	signed_message.extend_from_slice(msg);

	signed_message
}

/// From a signature and message, will attempt to recover and validate a Cosmos public key.
///
/// Supports both secp256k1 and secp256r1 signatures.
///
/// Returns `None` if signature is invalid, otherwise returns the `CosmosAddress` type wrapping the
/// public key.
pub fn cosmos_recover(
	prefix: &[u8],
	msg: &[u8],
	cosmos_address: CosmosPublicKey,
	CosmosEcdsaSignature(sig): &CosmosEcdsaSignature,
) -> Option<CosmosPublicKey> {
	let msg = sha2_256(&[prefix, msg].concat());

	match cosmos_address {
		CosmosPublicKey::Secp256k1(pub_key) => {
			// Cosmos gives us a 64-byte signature, we convert it into the more standard 65-byte
			// signature here
			let sig: EcdsaSignature = CosmosEcdsaSignature(*sig).into();

			if pub_key == sp_io::crypto::secp256k1_ecdsa_recover_compressed(&sig.0, &msg).ok()? {
				return Some(CosmosPublicKey::Secp256k1(pub_key))
			}

			None
		},
		CosmosPublicKey::Secp256r1(pub_key) => {
			// Deconstruct `sig` into `r` and `s` values so we can construct a p256
			// friendly signature
			let mut r: [u8; 32] = [0; 32];
			let mut s: [u8; 32] = [0; 32];
			r.copy_from_slice(&sig[..32]);
			s.copy_from_slice(&sig[32..64]);

			let sig = Signature::from_scalars(r, s).ok()?;
			let verify_key = VerifyingKey::from_sec1_bytes(&pub_key).ok()?;
			let _ = verify_key.verify(&msg, &sig).ok()?;
			Some(CosmosPublicKey::Secp256r1(pub_key))
		},
	}
}
