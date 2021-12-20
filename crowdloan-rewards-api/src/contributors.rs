use std::collections::HashMap;

use pallet_crowdloan_rewards::models::EthereumAddress;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sp_runtime::AccountId32;

// TODO: Ensure that `f64` has enough precision
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Contributors {
	success: bool,
	shares: Shares,
	#[serde(rename = "totalPercent", deserialize_with = "deserialize_number_from_string")]
	total_percent: f64,
	#[serde(rename = "totalAmount", deserialize_with = "deserialize_number_from_string")]
	total_amount: f64,
	#[serde(
		rename = "totalAmountWithoutBoost",
		deserialize_with = "deserialize_number_from_string"
	)]
	total_amount_without_boost: f64,
	message: String,
}

// TODO: Ensure that `f64` has enough precision
#[derive(Debug, Deserialize)]
pub struct Shares {
	#[serde(flatten)]
	etherium: HashMap<EthereumAddress, f64>,
	#[serde(flatten)]
	relay: HashMap<AccountId32, f64>,
}

#[cfg(test)]
mod test_contributors_serde {
	use super::*;

	#[test]
	fn test_deserialize() {
		let contrib_json = std::fs::read_to_string("contributors.json").unwrap();
		serde_json::from_str::<Contributors>(&contrib_json).unwrap();
	}
}
