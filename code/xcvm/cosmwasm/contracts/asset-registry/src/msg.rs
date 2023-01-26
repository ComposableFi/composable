use cosmwasm_std::Addr;
use cw_storage_plus::{CwIntKey, Key, KeyDeserialize, PrimaryKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xcvm_core::AssetId;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema)]
#[repr(transparent)]
pub struct AssetKey(pub AssetId);

impl From<u128> for AssetKey {
	fn from(x: u128) -> Self {
		Self(x.into())
	}
}

impl From<AssetId> for AssetKey {
	fn from(x: AssetId) -> Self {
		Self(x)
	}
}

impl From<AssetKey> for AssetId {
	fn from(AssetKey(x): AssetKey) -> Self {
		x
	}
}

impl<'a> PrimaryKey<'a> for AssetKey {
	type Prefix = ();
	type SubPrefix = ();
	type Suffix = u128;
	type SuperSuffix = u128;
	fn key(&self) -> Vec<Key> {
		vec![Key::Val128(self.0 .0 .0.to_cw_bytes())]
	}
}

impl KeyDeserialize for AssetKey {
	type Output = <u128 as KeyDeserialize>::Output;
	fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
		<u128 as KeyDeserialize>::from_vec(value)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum AssetReference {
	Native { denom: String },
	Virtual { cw20_address: Addr },
}

impl AssetReference {
	pub fn denom(&self) -> String {
		match self {
			AssetReference::Native { denom } => denom.clone(),
			AssetReference::Virtual { cw20_address } => format!("cw20:{}", cw20_address),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { pub admin: String }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	RegisterAsset { asset_id: AssetKey, reference: AssetReference },
	UnregisterAsset { asset_id: AssetKey },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
	Lookup { asset_id: AssetKey },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LookupResponse {
	pub reference: AssetReference,
}
