
use cosmwasm_schema::cw_serde;
use thiserror::Error;

#[derive(Error, Debug)]
#[cw_serde]
pub enum ContractError {
    Order,
}