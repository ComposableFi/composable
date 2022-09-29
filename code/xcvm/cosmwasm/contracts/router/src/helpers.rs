use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

use crate::msg::ExecuteMsg;

/// AssetRegistryContract is a wrapper around Addr that provides helpers
/// for working with this as a library.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetRegistryContract(pub Addr);

impl AssetRegistryContract {
	pub fn addr(&self) -> &Addr {
		&self.0
	}

	pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
		let msg = to_binary(&msg.into())?;
		Ok(WasmMsg::Execute { contract_addr: self.addr().into(), msg, funds: vec![] }.into())
	}
}
