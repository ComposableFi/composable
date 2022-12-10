use crate::{
	error::ContractError,
	state::{Config, CONFIG, ROUTER},
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

pub fn ensure_router(deps: Deps, sender: &str) -> Result<(), ContractError> {
	let router = ROUTER.load(deps.storage)?;
	if router.as_ref() == sender {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}
