use crate::prelude::*;
use sp_std::boxed::Box;

use sp_runtime::DispatchError;

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
	pub is_substrate_xcm: bool,
	pub para_id: Option<u32>,
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

	pub fn new_xcm_memo(receiver: String, para_id: Option<u32>) -> Self {
		Self {
			receiver,
			port: None,
			channel: None,
			timeout: None,
			retries: None,
			para_id: para_id,
			substrate: Some(true),
			next: None,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MemoData {
	pub forward: Forward,
}

impl MemoData {
	pub fn new(forward: Forward) -> Self {
		Self { forward }
	}
}
