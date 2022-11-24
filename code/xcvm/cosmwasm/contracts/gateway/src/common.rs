use crate::{
	error::ContractError,
	state::{Config, CONFIG},
};
use cosmwasm_std::Deps;

pub fn ensure_admin(deps: Deps, sender: &str) -> Result<(), ContractError> {
	let Config { admin, .. } = CONFIG.load(deps.storage)?;
	if &admin == sender {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}
