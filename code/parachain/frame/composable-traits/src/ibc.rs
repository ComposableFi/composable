use crate::prelude::*;

#[derive(
	Serialize,
	Deserialize,
	Clone,
	Debug,
	PartialEq,
	Eq,
	codec::Encode,
	codec::Decode,
	scale_info::TypeInfo,
)]
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
			para_id,
			substrate: Some(true),
			next: None,
		}
	}
}

impl From<MemoData> for pallet_ibc::ics20::MemoData {
	fn from(value: MemoData) -> Self {
		pallet_ibc::ics20::MemoData { forward: value.forward.into() }
	}
}

impl From<Forward> for pallet_ibc::ics20::Forward {
	fn from(value: Forward) -> Self {
		let next = value
			.next
			.map(|e| sp_std::boxed::Box::new(pallet_ibc::ics20::MemoData::from(*e)));
		pallet_ibc::ics20::Forward {
			receiver: value.receiver,
			port: value.port,
			channel: value.channel,
			timeout: value.timeout,
			retries: value.retries,
			para_id: value.para_id,
			substrate: value.substrate,
			next,
		}
	}
}

#[derive(
	Serialize,
	Deserialize,
	Clone,
	Debug,
	PartialEq,
	Eq,
	codec::Encode,
	codec::Decode,
	scale_info::TypeInfo,
)]
pub struct MemoData {
	forward: Forward,
}

impl MemoData {
	pub fn new(forward: Forward) -> Self {
		Self { forward }
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemoDataEnum {
	Forward(Forward),
	Wasm(xc_core::ibc::WasmMemo),
}
