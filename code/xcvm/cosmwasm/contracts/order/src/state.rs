use cosmwasm_std::{to_binary, Addr, Binary, Order, StdError, StdResult, Storage, SubMsgResponse};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use xc_core::InterpreterOrigin;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct Config {
	pub gateway_address: xc_core::gateway::Gateway,
	pub interpreter_origin: InterpreterOrigin,
}

/// The interpreter configuration.
pub const CONFIG: Item<Config> = Item::new("config");

/// List of owners able to execute programs on our behalf. Be aware that only `trusted` address must
/// be added.
pub const OWNERS: Map<Addr, ()> = Map::new("owners");

/// This register hold the latest program instruction (index) executed.
pub const IP_REGISTER: Item<u16> = Item::new("ip_register");

/// This register contains the latest executed instruction result for the program.
/// It can be either a success `SubMsgResponse` or an error message (in this case changes of message
/// were not applied).
pub const RESULT_REGISTER: Item<Result<SubMsgResponse, String>> = Item::new("result_register");

pub const TIP_REGISTER: Item<Addr> = Item::new("tip_register");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct State {
	pub result_register: Result<SubMsgResponse, String>,
	pub ip_register: u16,
	pub owners: Vec<Addr>,
	pub config: Config,
}

impl TryInto<Binary> for State {
	type Error = StdError;

	fn try_into(self) -> StdResult<Binary> {
		to_binary(&self)
	}
}

pub(crate) fn read(storage: &dyn Storage) -> StdResult<State> {
	Ok(State {
		result_register: RESULT_REGISTER.load(storage)?,
		ip_register: IP_REGISTER.load(storage).unwrap_or(0),
		owners: OWNERS
			.range(storage, None, None, Order::Ascending)
			.map(|e| e.map(|(k, _)| k))
			.collect::<StdResult<Vec<_>>>()?,
		config: CONFIG.load(storage)?,
	})
}
