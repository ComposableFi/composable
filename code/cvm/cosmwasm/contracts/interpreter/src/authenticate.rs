use crate::{
	error::{ContractError, Result},
	state::OWNERS,
};
use cosmwasm_std::{Addr, Deps};

/// Authenticated token, MUST be private and kept in this module.
/// MUST ONLY be instantiated by [`ensure_owner`].
pub struct Authenticated(());

/// Ensure that the caller is either the current interpreter or listed in the owners of the
/// interpreter.
/// Any operation executing against the interpreter must pass this check.
pub fn ensure_owner(deps: Deps, self_addr: &Addr, sender: Addr) -> Result<Authenticated> {
	if sender == self_addr || OWNERS.has(deps.storage, sender) {
		Ok(Authenticated(()))
	} else {
		Err(ContractError::NotAuthorized)
	}
}
