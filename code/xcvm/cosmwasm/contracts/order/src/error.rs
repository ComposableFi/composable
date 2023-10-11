use cosmwasm_schema::cw_serde;
use thiserror::Error;

#[derive(Error)]
#[cw_serde]
pub enum ContractError {
	#[error("Invalid solution")]
	InvalidSolution,
}
