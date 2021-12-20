use std::{collections::HashMap, hash::Hash};

use pallet_crowdloan_rewards::models::RemoteAccount;
use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::deserialize_number_from_string;
use sp_runtime::AccountId32;

pub(crate) fn get_contributors() -> Contributors {
	let contrib_json = std::fs::read_to_string("contributors.json").unwrap();
	serde_json::from_str::<Contributors>(&contrib_json).unwrap()
}

// TODO: Ensure that `f64` has enough precision
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Contributors {
	pub(crate) success: bool,
	#[serde(deserialize_with = "hash_map_with_numeric_string_values")]
	pub(crate) shares: HashMap<RemoteAccount<AccountId32>, f64>,
	#[serde(rename = "totalPercent", deserialize_with = "deserialize_number_from_string")]
	pub(crate) total_percent: f64,
	#[serde(rename = "totalAmount", deserialize_with = "deserialize_number_from_string")]
	pub(crate) total_amount: f64,
	#[serde(
		rename = "totalAmountWithoutBoost",
		deserialize_with = "deserialize_number_from_string"
	)]
	pub(crate) total_amount_without_boost: f64,
	pub(crate) message: String,
}

fn hash_map_with_numeric_string_values<'de, D, T: Deserialize<'de> + Eq + Hash>(
	deserializer: D,
) -> Result<HashMap<T, f64>, D::Error>
where
	D: Deserializer<'de>,
{
	#[derive(Deserialize)]
	struct Wrapper(#[serde(deserialize_with = "deserialize_number_from_string")] f64);

	let v = HashMap::<T, Wrapper>::deserialize(deserializer)?;
	Ok(v.into_iter().map(|(k, Wrapper(v))| (k, v)).collect())
}

#[cfg(test)]
mod test_contributors_serde {
	use super::*;

	#[test]
	fn test_deserialize() {
		let contrib_json = std::fs::read_to_string("contributors.json").unwrap();
		serde_json::from_str::<Contributors>(&contrib_json).unwrap();
	}

	// #[test]
	// fn test_relay() {
	// 	let _: AccountId32 =
	// 		serde_json::from_str("\"<put address here>\"").unwrap();
	// }
}
