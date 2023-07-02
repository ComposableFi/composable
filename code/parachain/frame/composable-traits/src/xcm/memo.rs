use crate::{prelude::*};
use sp_std::boxed::Box;

use sp_runtime::{
	DispatchError
};

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
pub struct MemoForward {
    pub receiver: String,
    pub port: String,
    pub channel: String,
    pub timeout: String,
    pub retries: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<Box<MemoForward>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MemoData {
    pub forward: MemoForward,
}