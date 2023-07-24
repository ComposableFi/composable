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
	pub chain_id: u128, // please use u32 for chain id, i asked in one of review before
	pub order: u8, // please document what order means
	pub channel_id: u64,        //for packet or memo
	// what is timestamp? what does it means? why chain has timestamp?
	pub timestamp: Option<u64>, //for packet
	// instead of code comment, please use stucture to unite peoperites
	
	// please use ibc_rs_scale for Timeout Height/Timeout
	pub height: Option<u64>,    //for memo packet message forwarding
	// use u8
	pub retries: Option<u64>,   //for memo packet message forwarding
	// what is it seconds? please leave comment
	pub timeout: Option<u64>,   //for memo packet message forwarding
	// please use Option(Substrate(IBC || XCM),not 2 bools
	pub is_substrate_ibc: bool,
	pub is_substrate_xcm: bool,
	pub para_id: Option<u32>,
}
