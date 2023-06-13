use crate::{
	error::{ContractError, ContractResult},
	state,
};

use cosmwasm_std::{Deps, Env, Event, MessageInfo};
use xc_core::InterpreterOrigin;

/// Creates an event with contract’s default prefix and given action attribute.
pub(crate) fn make_event(action: &str) -> Event {
	Event::new(cw_xc_common::gateway::EVENT_PREFIX).add_attribute("action", action)
}

/// Ensure that the sender of the message is current contract.
///
/// Address of the current contract is read from the environment passed in `env`
/// and sender of the message is read from message info passed in `info`
/// argument.
pub(crate) fn ensure_self(env: &Env, info: &MessageInfo) -> ContractResult<()> {
	if info.sender == env.contract.address {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}

/// Ensure that the sender of the message is an interpreter for the provided
/// `interpreter_origin`.
///
/// Sender of the message is read from message info passed in `info` argument.
/// Address of the interpreter is read from storage (see
/// [`state::INTERPRETERS`]).  Check fails if interpreter is not known or its
/// address is not known
pub(crate) fn ensure_interpreter(
	deps: Deps,
	info: &MessageInfo,
	interpreter_origin: InterpreterOrigin,
) -> ContractResult<()> {
	let interpreter_address = state::INTERPRETERS
		.may_load(deps.storage, interpreter_origin)?
		.ok_or(ContractError::NotAuthorized)?
		.address;
	if info.sender == interpreter_address {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}

/// Ensure that the sender of the message is contract’s admin.
///
/// Sender of the message is read from message info passed in `info` argument.
/// Address of the interpreter is read from storage (see [`state::CONFIG`]).
pub(crate) fn ensure_admin(deps: Deps, info: &MessageInfo) -> ContractResult<()> {
	if info.sender == state::Config::load(deps.storage)?.admin {
		Ok(())
	} else {
		Err(ContractError::NotAuthorized)
	}
}
