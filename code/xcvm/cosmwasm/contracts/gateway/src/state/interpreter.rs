use xc_core::InterpreterOrigin;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub(crate) struct Interpreter {
	pub address: Addr,
}

pub(crate) const INTERPRETERS: Map<InterpreterOrigin, Interpreter> = Map::new("interpreters");
