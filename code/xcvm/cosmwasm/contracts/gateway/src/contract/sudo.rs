use crate::error::ContractError;
use cosmwasm_std::{entry_point, DepsMut, Env};
use xc_core::transport::ibc::{ics20::hook::IBCLifecycleComplete, SudoMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> crate::error::Result {
	match msg {
		SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
			channel: _,
			sequence: _,
			ack: _,
			success: _,
		}) => Err(ContractError::NotImplemented),
		SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout {
			channel: _,
			sequence: _,
		}) => Err(ContractError::NotImplemented),
	}
}
