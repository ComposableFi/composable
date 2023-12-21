use crate::{prelude::*, NetworkId};

pub mod osmosis_std;

pub type ExchangeId = crate::shared::Displayed<u128>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ExchangeType {
	OsmosisCrossChainSwap { pool_id: u64, token_a: String, token_b: String },
}

/// allows to execute Exchange instruction
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct ExchangeItem {
	pub exchange_id: ExchangeId,
	pub network_id: NetworkId,
	pub exchange: ExchangeType,
}
