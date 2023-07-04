use crate::prelude::*;

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