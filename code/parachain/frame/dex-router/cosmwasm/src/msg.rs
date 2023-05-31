extern crate alloc;

use serde::{Deserialize, Serialize};
use cosmwasm_vm::cosmwasm_std::{Coin, Uint128, Uint64};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
	Swap { in_asset: Coin, min_receive: Coin, pool_id: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
	GetRoute { input_denom: Coin, output_denom: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct GetRouteResponse {
    pub pool_route: Vec<SwapAmountInRoute>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SwapAmountInRoute {
    pub pool_id: Vec<Uint64>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SwapResponse {
    pub original_sender: String,
    pub token_out_denom: String,
    pub amount: Uint128,
}