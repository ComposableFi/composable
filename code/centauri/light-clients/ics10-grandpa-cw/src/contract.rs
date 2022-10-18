#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ics10-grandpa-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	match msg {
		ExecuteMsg::ValidateMsg(_) => todo!(),
		ExecuteMsg::StatusMsg(_) => todo!(),
		ExecuteMsg::ExportedMetadataMsg(_) => todo!(),
		ExecuteMsg::ZeroCustomFieldsMsg(_) => todo!(),
		ExecuteMsg::GetTimestampAtHeightMsg(_) => todo!(),
		ExecuteMsg::InitializeMsg(_) => todo!(),
		ExecuteMsg::VerifyMembershipMsg(_) => todo!(),
		ExecuteMsg::VerifyClientMessage(_) => todo!(),
		ExecuteMsg::CheckForMisbehaviourMsg(_) => todo!(),
		ExecuteMsg::UpdateStateOnMisbehaviourMsg(_) => todo!(),
		ExecuteMsg::UpdateStateMsg(_) => todo!(),
		ExecuteMsg::CheckSubstituteAndUpdateStateMsg(_) => todo!(),
		ExecuteMsg::VerifyUpgradeAndUpdateStateMsg(_) => todo!(),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::ClientTypeMsg(_) => todo!(),
		QueryMsg::GetLatestHeightsMsg(_) => todo!(),
	}
}

#[cfg(test)]
mod tests {}
