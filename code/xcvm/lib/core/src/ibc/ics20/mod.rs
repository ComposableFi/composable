use self::{hook::WasmMemo, pfm::Forward};

pub mod hook;
pub mod pfm;

use crate::prelude::*;

#[derive(
	Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct MemoData {
	pub forward: Forward,
	pub wasm: Option<WasmMemo>,
}

impl MemoData {
	pub fn forward(forward: Forward) -> Self {
		Self { forward, wasm: None }
	}
}
