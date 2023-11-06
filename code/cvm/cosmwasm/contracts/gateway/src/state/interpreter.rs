use cosmwasm_std::{Deps, StdResult};
use cw_storage_plus::Item;
use xc_core::InterpreterOrigin;

use crate::prelude::*;

pub type InterpreterId = xc_core::shared::Displayed<u128>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub(crate) struct Interpreter {
	/// contract address
	pub address: Addr,
	pub interpreter_id: InterpreterId,
}

pub(crate) fn get_by_origin(deps: Deps, origin: InterpreterOrigin) -> StdResult<Interpreter> {
	let id = INTERPRETERS_ORIGIN_TO_ID.load(deps.storage, origin)?;
	INTERPRETERS.load(deps.storage, id)
}

pub(crate) const INTERPRETERS_COUNT: Item<u128> = Item::new("interpreter_count");

pub(crate) const INTERPRETERS_ORIGIN_TO_ID: Map<InterpreterOrigin, u128> =
	Map::new("interpreters_origin_to_id");

pub(crate) const INTERPRETERS: Map<u128, Interpreter> = Map::new("interpreters");
