use crate::{
	error::ContractError,
	state::{ConfigState, CONFIG},
};
use cosmwasm_std::Deps;

pub fn ensure_admin(deps: Deps, sender: &str) -> Result<(), ContractError> {
	match CONFIG.load(deps.storage)? {
		ConfigState::Initialized { admin, .. } =>
			if admin.as_ref() == sender {
				Ok(())
			} else {
				Err(ContractError::NotAuthorized)
			},
		ConfigState::NotInitialized => Err(ContractError::NotAuthorized),
	}
}
