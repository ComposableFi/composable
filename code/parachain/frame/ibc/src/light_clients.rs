use alloc::{borrow::ToOwned, format, string::ToString, vec::Vec};
use frame_support::{
	pallet_prelude::{StorageValue, ValueQuery},
	traits::StorageInstance,
};
use ibc::core::{
	ics02_client,
	ics02_client::{client_consensus::ConsensusState, client_state::ClientState},
};
use ibc_derive::{ClientDef, ClientMessage, ClientState, ConsensusState, Protobuf};
use ibc_primitives::runtime_interface;
use ibc_proto::google::protobuf::Any;
use ics10_grandpa::{
	client_message::{RelayChainHeader, GRANDPA_CLIENT_MESSAGE_TYPE_URL},
	client_state::GRANDPA_CLIENT_STATE_TYPE_URL,
	consensus_state::GRANDPA_CONSENSUS_STATE_TYPE_URL,
};
use ics11_beefy::{
	client_message::BEEFY_CLIENT_MESSAGE_TYPE_URL, client_state::BEEFY_CLIENT_STATE_TYPE_URL,
	consensus_state::BEEFY_CONSENSUS_STATE_TYPE_URL,
};
use sp_core::{ed25519, H256};
use sp_runtime::{
	app_crypto::RuntimePublic,
	traits::{BlakeTwo256, ConstU32, Header},
	BoundedBTreeSet, BoundedVec,
};
use tendermint_proto::Protobuf;

pub const TENDERMINT_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.tendermint.v1.ClientState";
pub const TENDERMINT_CLIENT_MESSAGE_TYPE_URL: &str =
	"/ibc.lightclients.tendermint.v1.ClientMessage";
pub const TENDERMINT_CONSENSUS_STATE_TYPE_URL: &str =
	"/ibc.lightclients.tendermint.v1.ConsensusState";

#[derive(Clone, Default, PartialEq, Debug, Eq)]
pub struct HostFunctionsManager;

impl ics23::HostFunctionsProvider for HostFunctionsManager {
	fn sha2_256(message: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(message)
	}

	fn sha2_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::sha2_512(message)
	}

	fn sha2_512_truncated(message: &[u8]) -> [u8; 32] {
		runtime_interface::sha2_512_truncated(message)
	}

	fn sha3_512(message: &[u8]) -> [u8; 64] {
		runtime_interface::sha3_512(message)
	}

	fn ripemd160(message: &[u8]) -> [u8; 20] {
		runtime_interface::ripemd160(message)
	}
}

impl tendermint_light_client_verifier::host_functions::HostFunctionsProvider
	for HostFunctionsManager
{
	fn sha2_256(message: &[u8]) -> [u8; 32] {
		sp_io::hashing::sha2_256(message)
	}

	fn ed25519_verify(signature: &[u8], msg: &[u8], pubkey: &[u8]) -> bool {
		if let Some((signature, public_key)) =
			ed25519::Signature::from_slice(signature).and_then(|sig| {
				let public = sp_core::ed25519::Public::try_from(pubkey).ok()?;
				Some((sig, public))
			}) {
			sp_io::crypto::ed25519_verify(&signature, msg, &public_key)
		} else {
			false
		}
	}

	fn secp256k1_verify(_: &[u8], _: &[u8], _: &[u8]) -> bool {
		unimplemented!()
	}
}

impl ics07_tendermint::HostFunctionsProvider for HostFunctionsManager {}

pub struct GrandpaHederHashesStorageInstance;
impl StorageInstance for GrandpaHederHashesStorageInstance {
	fn pallet_prefix() -> &'static str {
		"ibc.lightclients.grandpa"
	}

	const STORAGE_PREFIX: &'static str = "HeaderHashes";
}
pub type GrandpaHeaderHashesStorage = StorageValue<
	GrandpaHederHashesStorageInstance,
	BoundedVec<H256, ConstU32<GRANDPA_BLOCK_HASHES_CACHE_SIZE>>,
	ValueQuery,
>;

pub struct GrandpaHeaderHashesSetStorageInstance;
impl StorageInstance for GrandpaHeaderHashesSetStorageInstance {
	fn pallet_prefix() -> &'static str {
		"ibc.lightclients.grandpa"
	}

	const STORAGE_PREFIX: &'static str = "HeaderHashesSet";
}
pub type GrandpaHeaderHashesSetStorage = StorageValue<
	GrandpaHeaderHashesSetStorageInstance,
	BoundedBTreeSet<H256, ConstU32<GRANDPA_BLOCK_HASHES_CACHE_SIZE>>,
	ValueQuery,
>;

/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
const GRANDPA_BLOCK_HASHES_CACHE_SIZE: u32 = 500;

impl grandpa_client_primitives::HostFunctions for HostFunctionsManager {
	type Header = RelayChainHeader;

	fn ed25519_verify(sig: &ed25519::Signature, msg: &[u8], pub_key: &ed25519::Public) -> bool {
		pub_key.verify(&msg, sig)
	}

	fn add_relaychain_header_hashes(new_hashes: &[<Self::Header as Header>::Hash]) {
		if new_hashes.is_empty() {
			return
		}

		GrandpaHeaderHashesSetStorage::mutate(|hashes_set| {
			GrandpaHeaderHashesStorage::mutate(|hashes| {
				for hash in new_hashes {
					match hashes.try_push(*hash) {
						Ok(_) => {},
						Err(_) => {
							let hash = hashes.remove(0);
							hashes_set.remove(&hash);
							let _ = hashes.try_push(hash);
						},
					}
					match hashes_set.try_insert(*hash) {
						Ok(_) => {},
						Err(_) => {
							log::warn!("duplicated value in GrandpaHeaderHashesStorage or the storage is corrupted");
						},
					}
				}
			});
		});
	}

	fn exists_relaychain_header_hash(hash: <Self::Header as Header>::Hash) -> bool {
		GrandpaHeaderHashesSetStorage::get().contains(&hash)
	}
}

impl light_client_common::HostFunctions for HostFunctionsManager {
	type BlakeTwo256 = BlakeTwo256;
}

impl beefy_client_primitives::HostFunctions for HostFunctionsManager {
	fn keccak_256(input: &[u8]) -> [u8; 32] {
		sp_io::hashing::keccak_256(input)
	}

	fn secp256k1_ecdsa_recover_compressed(signature: &[u8; 65], msg: &[u8; 32]) -> Option<Vec<u8>> {
		sp_io::crypto::secp256k1_ecdsa_recover_compressed(signature, msg)
			.ok()
			.map(|pub_key| pub_key.to_vec())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, ClientDef)]
pub enum AnyClient {
	Grandpa(ics10_grandpa::client_def::GrandpaClient<HostFunctionsManager>),
	Beefy(ics11_beefy::client_def::BeefyClient<HostFunctionsManager>),
	Tendermint(ics07_tendermint::client_def::TendermintClient<HostFunctionsManager>),
	#[cfg(test)]
	Mock(ibc::mock::client_def::MockClient),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnyUpgradeOptions {
	Grandpa(ics10_grandpa::client_state::UpgradeOptions),
	Beefy(ics11_beefy::client_state::UpgradeOptions),
	Tendermint(ics07_tendermint::client_state::UpgradeOptions),
	#[cfg(test)]
	Mock(()),
}

#[derive(Clone, Debug, PartialEq, Eq, ClientState, Protobuf)]
pub enum AnyClientState {
	#[ibc(proto_url = "GRANDPA_CLIENT_STATE_TYPE_URL")]
	Grandpa(ics10_grandpa::client_state::ClientState<HostFunctionsManager>),
	#[ibc(proto_url = "BEEFY_CLIENT_STATE_TYPE_URL")]
	Beefy(ics11_beefy::client_state::ClientState<HostFunctionsManager>),
	#[ibc(proto_url = "TENDERMINT_CLIENT_STATE_TYPE_URL")]
	Tendermint(ics07_tendermint::client_state::ClientState<HostFunctionsManager>),
	#[cfg(test)]
	#[ibc(proto_url = "MOCK_CLIENT_STATE_TYPE_URL")]
	Mock(ibc::mock::client_state::MockClientState),
}

#[derive(Clone, Debug, PartialEq, Eq, ConsensusState, Protobuf)]
pub enum AnyConsensusState {
	#[ibc(proto_url = "GRANDPA_CONSENSUS_STATE_TYPE_URL")]
	Grandpa(ics10_grandpa::consensus_state::ConsensusState),
	#[ibc(proto_url = "BEEFY_CONSENSUS_STATE_TYPE_URL")]
	Beefy(ics11_beefy::consensus_state::ConsensusState),
	#[ibc(proto_url = "TENDERMINT_CONSENSUS_STATE_TYPE_URL")]
	Tendermint(ics07_tendermint::consensus_state::ConsensusState),
	#[cfg(test)]
	#[ibc(proto_url = "MOCK_CONSENSUS_STATE_TYPE_URL")]
	Mock(ibc::mock::client_state::MockConsensusState),
}

#[derive(Clone, Debug, ClientMessage)]
#[allow(clippy::large_enum_variant)]
pub enum AnyClientMessage {
	#[ibc(proto_url = "GRANDPA_CLIENT_MESSAGE_TYPE_URL")]
	Grandpa(ics10_grandpa::client_message::ClientMessage),
	#[ibc(proto_url = "BEEFY_CLIENT_MESSAGE_TYPE_URL")]
	Beefy(ics11_beefy::client_message::ClientMessage),
	#[ibc(proto_url = "TENDERMINT_CLIENT_MESSAGE_TYPE_URL")]
	Tendermint(ics07_tendermint::client_message::ClientMessage),
	#[cfg(test)]
	#[ibc(proto_url = "MOCK_CLIENT_MESSAGE_TYPE_URL")]
	Mock(ibc::mock::header::MockClientMessage),
}

impl Protobuf<Any> for AnyClientMessage {}

impl TryFrom<Any> for AnyClientMessage {
	type Error = ics02_client::error::Error;

	fn try_from(value: Any) -> Result<Self, Self::Error> {
		match value.type_url.as_str() {
			GRANDPA_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Grandpa(
				ics10_grandpa::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			BEEFY_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Beefy(
				ics11_beefy::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			TENDERMINT_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Tendermint(
				ics07_tendermint::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			_ => Err(ics02_client::error::Error::unknown_consensus_state_type(value.type_url)),
		}
	}
}

impl From<AnyClientMessage> for Any {
	fn from(client_msg: AnyClientMessage) -> Self {
		match client_msg {
			AnyClientMessage::Grandpa(msg) => Any {
				type_url: GRANDPA_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec(),
			},
			AnyClientMessage::Beefy(msg) =>
				Any { type_url: BEEFY_CLIENT_MESSAGE_TYPE_URL.to_string(), value: msg.encode_vec() },
			AnyClientMessage::Tendermint(msg) => Any {
				type_url: TENDERMINT_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec(),
			},
			#[cfg(test)]
			AnyClientMessage::Mock(_msg) => panic!("MockHeader can't be serialized"),
		}
	}
}

#[cfg(test)]
pub use mocks::*;

#[cfg(test)]
mod mocks {
	pub const MOCK_CLIENT_STATE_TYPE_URL: &str = "/ibc.mock.ClientState";
	pub const MOCK_CLIENT_MESSAGE_TYPE_URL: &str = "/ibc.mock.ClientMessage";
	pub const MOCK_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.mock.ConsensusState";
}
