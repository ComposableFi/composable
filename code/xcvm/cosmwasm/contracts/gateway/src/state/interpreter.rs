use cosmwasm_std::{Deps, StdResult, StdError, Storage};
use cw_storage_plus::Item;
use xc_core::{InterpreterOrigin, Displayed};

use crate::prelude::*;

pub type InterpreterId = Displayed<u128>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub(crate) struct Interpreter {
	pub address: Addr,
	pub interpreter_id : InterpreterId

}

pub fn get_by_origin(deps: Deps, origin: InterpreterOrigin) -> StdResult<Interpreter> {
	INTERPRETERS_ORIGIN_TO_ID
		.may_load(deps.storage, origin)?
		.and_then(|id| INTERPRETERS.may_load(deps.storage, &id.0))
		.and_then(|interpreter| interpreter.ok_or_else(|| StdError::not_found("interpreter")))
}

pub const INTERPRETERS_COUNT: Item<u128> = Item::new("interpreter_count");

pub(crate) const INTERPRETERS_ORIGIN_TO_ID: Map<InterpreterOrigin, InterpreterId > = Map::new("interpreters_origin_to_id");

pub(crate) const INTERPRETERS: Map<InterpreterId, Interpreter> = Map::new("interpreters");
