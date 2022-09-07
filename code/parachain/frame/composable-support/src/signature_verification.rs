//! Functions for supporting signature verification for various chains.
//!
//! Signed messages/proofs are expected to be in the format of `{prefix}-{msg}` before they
//! are modified to fit their chains signature specifications.
use crate::types::{CosmosEcdsaSignature, CosmosPublicKey, EcdsaSignature, EthereumAddress};
use codec::{Decode, Encode};
use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use scale_info::TypeInfo;
use sp_io::hashing::{keccak_256, sha2_256};
use sp_runtime::{traits::Verify, AccountId32, MultiSignature};
use sp_std::vec::Vec;

/// Error type for all signature verification.
#[derive(Clone, Debug, PartialEq, Eq, TypeInfo, Encode, Decode)]
pub enum SignatureVerificationError {
	/// Unable to process signature for verification.
	InvalidSignature,
	/// Unable to process public key for verification.
	InvalidPublicKey,
	/// Signature failed the verification process.
	FailedVerification,
}

/// Result type for signature verification.
pub type Result<T> = core::result::Result<T, SignatureVerificationError>;

impl From<sp_io::EcdsaVerifyError> for SignatureVerificationError {
	fn from(error: sp_io::EcdsaVerifyError) -> Self {
		match error {
			sp_io::EcdsaVerifyError::BadRS => Self::FailedVerification,
			sp_io::EcdsaVerifyError::BadV => Self::FailedVerification,
			sp_io::EcdsaVerifyError::BadSignature => Self::InvalidSignature,
		}
	}
}

pub fn get_encoded_vec(data: &[u8]) -> Vec<u8> {
	hex::encode(data).as_bytes().to_vec()
}

/// Verify the proof is valid for a given relay account.
///
/// Returns `false` if the verification process fails, returns `true` otherwise
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
	msg.append(&mut reward_account.using_encoded(get_encoded_vec));
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
) -> Result<EthereumAddress> {
	let msg = keccak_256(&ethereum_signable_message(prefix, msg));
	let mut address = EthereumAddress::default();

	address.0.copy_from_slice(
		&keccak_256(&sp_io::crypto::secp256k1_ecdsa_recover(sig, &msg)?[..])[12..],
	);

	Ok(address)
}

/// Generates a message that is compatible with the Ethereum signing process.
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
) -> Result<CosmosPublicKey> {
	let msg = sha2_256(&[prefix, msg].concat());

	match cosmos_address {
		CosmosPublicKey::Secp256k1(pub_key) => {
			// Cosmos gives us a 64-byte signature, we convert it into the more standard 65-byte
			// signature here
			let sig: EcdsaSignature = CosmosEcdsaSignature(*sig).into();

			if pub_key == sp_io::crypto::secp256k1_ecdsa_recover_compressed(&sig.0, &msg)? {
				return Ok(CosmosPublicKey::Secp256k1(pub_key))
			}

			Err(SignatureVerificationError::FailedVerification)
		},
		CosmosPublicKey::Secp256r1(pub_key) => {
			// Deconstruct `sig` into `r` and `s` values so we can construct a p256
			// friendly signature
			let mut r: [u8; 32] = [0; 32];
			let mut s: [u8; 32] = [0; 32];
			r.copy_from_slice(&sig[..32]);
			s.copy_from_slice(&sig[32..64]);

			let sig = Signature::from_scalars(r, s)
				.map_err(|_| SignatureVerificationError::InvalidSignature)?;
			let verify_key = VerifyingKey::from_sec1_bytes(&pub_key)
				.map_err(|_| SignatureVerificationError::InvalidPublicKey)?;
			verify_key
				.verify(&msg, &sig)
				.map_err(|_| SignatureVerificationError::FailedVerification)?;
			Ok(CosmosPublicKey::Secp256r1(pub_key))
		},
	}
}
