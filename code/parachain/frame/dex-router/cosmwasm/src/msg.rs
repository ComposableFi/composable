use alloc::string::String;
use cosmwasm_vm::cosmwasm_std::{Coin, Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	Swap { in_asset: Coin, min_receive: Coin, pool_id: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
	Price { in_asset: Coin, output_denom: String, pool_id: Uint128 },
}
