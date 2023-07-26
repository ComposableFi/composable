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
	pub order: u8,
	pub channel_id: u64,        //for packet or memo
	pub timestamp: Option<u64>, //for packet
	pub height: Option<u64>,    //for memo packet message forwarding
	pub retries: Option<u8>,    //for memo packet message forwarding
	pub timeout: Option<u64>,   //for memo packet message forwarding
	pub is_substrate_ibc: bool,
	pub is_substrate_xcm: bool,
	pub para_id: Option<u32>,
}
