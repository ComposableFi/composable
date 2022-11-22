use crate::{error::ContractError, state::CONFIG};
use cosmwasm_std::Deps;

pub fn ensure_admin(deps: Deps, sender: &str) -> Result<(), ContractError> {
	let config = CONFIG.load(deps.storage)?;
	if config.admin.as_ref() == sender {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}
