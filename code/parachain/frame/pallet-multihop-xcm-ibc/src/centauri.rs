use ibc_rs_scale::core::ics24_host::identifier::{ChannelId, PortId};

use crate::prelude::*;

pub struct Map;
impl Map {
	pub fn from_cw(mut value: MemoData) -> pallet_ibc::ics20::MemoData {
		let next = value.forward.next.take().map(|e| Box::new(Map::from_cw(*e)));
		let value = value.forward;
		let forward = pallet_ibc::ics20::Forward {
			receiver: value.receiver,
			port: value.port.map(|x| x.to_string()),
			channel: value.channel.map(|x| x.to_string()),
			timeout: value.timeout,
			retries: value.retries.map(Into::into),
			para_id: value.substrate.and_then(|x| x.para_id),
			substrate: value.substrate.map(|_| true),
			next,
		};

		pallet_ibc::ics20::MemoData { forward }
	}
}

#[derive(
	Serialize, Deserialize, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo, Encode, Decode,
)]
pub struct WasmMemo {
	pub contract: String,
	pub msg: Vec<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ibc_callback: Option<String>,
}

#[derive(
	Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo,
)]
#[serde(rename_all = "snake_case")]
pub struct Forward {
	pub receiver: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub port: Option<PortId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub channel: Option<ChannelId>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub retries: Option<u8>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub substrate: Option<IbcSubstrate>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub next: Option<Box<MemoData>>,
}

#[derive(
	Serialize,
	Deserialize,
	Clone,
	Debug,
	PartialEq,
	Eq,
	Encode,
	Decode,
	scale_info::TypeInfo,
	Default,
	Copy,
)]
#[serde(rename_all = "snake_case")]
pub struct IbcSubstrate {
	/// since other parachain does not support ibc memo
	/// there is only two option: send to parachain or send to relay-chain
	/// if para id is none, it means send to relay-chain
	#[serde(skip_serializing_if = "Option::is_none")]
	pub para_id: Option<u32>, //if para id is none, it means send to relay-chain
}

impl IbcSubstrate {
	pub fn new(para_id: Option<u32>) -> Self {
		Self { para_id }
	}
}

impl Forward {
	pub fn new_ibc_memo(
		receiver: String,
		port: PortId,
		channel: ChannelId,
		timeout: String,
		retries: u8,
	) -> Self {
		Self {
			receiver,
			port: Some(port),
			channel: Some(channel),
			timeout: Some(timeout),
			retries: Some(retries),
			substrate: <_>::default(),
			next: None,
		}
	}

	pub fn new_xcm_memo(receiver: String, substrate: IbcSubstrate) -> Self {
		Self {
			receiver,
			port: None,
			channel: None,
			timeout: None,
			retries: None,
			substrate: Some(substrate),
			next: None,
		}
	}
}

#[derive(
	Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo,
)]
#[serde(rename_all = "snake_case")]
pub struct MemoData {
	pub forward: Forward,
	pub wasm: Option<WasmMemo>,
}

impl MemoData {
	pub fn forward(forward: Forward) -> Self {
		Self { forward, wasm: None }
	}
}
