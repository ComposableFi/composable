use crate::prelude::*;
use serde_json::Value;
use sp_std::boxed::Box;

#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	Hash,
	codec::Encode,
	codec::Decode,
	scale_info::TypeInfo,
	Ord,
	PartialOrd,
	MaxEncodedLen,
	Debug,
)]
pub struct ChainInfo {
	pub chain_id: u128,
	pub channel_id: u64,        //for packet or memo
	pub timestamp: Option<u64>, //for packet
	pub height: Option<u64>,    //for memo packet message forwarding
	pub retries: Option<u64>,   //for memo packet message forwarding
	pub timeout: Option<u64>,   //for memo packet message forwarding
	pub is_substrate_ibc: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Forward {
	pub receiver: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub port: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub retries: Option<u64>,

	/// since other parachain does not support ibc memo
	/// there is only two option: send to parachain or send to relay-chain
	// #[serde(skip_serializing_if = "Option::is_none")]
	/// we do not need parrent id. if para id is none, it means send to relay-chain
	// pub parent: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub para_id: Option<u32>, //if para id is none, it means send to relay-chain
	#[serde(skip_serializing_if = "Option::is_none")]
	pub substrate: Option<bool>,
	///
	#[serde(skip_serializing_if = "Option::is_none")]
	pub next: Option<Box<MemoData>>,
}

impl Forward {
	pub fn new_ibc_memo(
		receiver: String,
		port: String,
		channel: String,
		timeout: String,
		retries: u64,
	) -> Self {
		Self {
			receiver,
			port: Some(port),
			channel: Some(channel),
			timeout: Some(timeout),
			retries: Some(retries),
			para_id: None,
			substrate: None,
			next: None,
		}
	}

	pub fn new_xcm_memo(receiver: String, para_id: u32, substrate: bool) -> Self {
		Self {
			receiver,
			port: None,
			channel: None,
			timeout: None,
			retries: None,
			para_id: Some(para_id),
			substrate: Some(substrate),
			next: None,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MemoData {
	Forward(Forward),
	Wasm(Wasm),
}

/// see https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Wasm {
	contract: String,
	msg: Value,
	#[serde(skip_serializing_if = "Option::is_none")]
	ibc_callback: Option<String>,
}

impl MemoData {
	pub fn forward(forward: Forward) -> Self {
		Self::Forward(forward)
	}

	pub fn wasm(wasm: Wasm) -> Self {
		Self::Wasm(wasm)
	}
}


pub const SENDER_PREFIX : &str = "ibc-wasm-hook-intermediary";

pub fn derive_intermediate_sender(channel: String, original_sender : String, bech32_prefix : String) -> Result<String, ()> {

	use bech32_no_std::ToBase32;
	let sender_str = format!("{channel}/{original_sender}");
	let sender_hash_32 = hash(SENDER_PREFIX, sender_str.as_bytes());
	let sender = sender_hash_32.to_base32();
	bech32_no_std::encode(&bech32_prefix, sender).map_err(|_| ())
}

// Hash creates a new address from address type and key.
// The functions should only be used by new types defining their own address function
// (eg public keys).
/// https://github.com/cosmos/cosmos-sdk/blob/main/types/address/hash.go
fn hash(typ : &str, key: &[u8])  -> [u8; 32] {
	use sha2::{Sha256, Digest};
	let mut hasher = Sha256::default();
	hasher.update(typ.as_bytes());
	let th = hasher.finalize();
	let mut hasher = Sha256::default();
	hasher.update(th);
	hasher.update(key);	
	hasher.finalize().into()
}