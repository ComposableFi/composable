#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ibc::core::{ics02_client::client_def::ClientDef, ics26_routing::context::ReaderContext};
use ics10_grandpa::client_def::GrandpaClient;
use sp_runtime::traits::BlakeTwo256;
// use cw2::set_contract_version;

use crate::{
	client_state::validate_client_state,
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
	state::get_client_state,
};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ics10-grandpa-cw";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[derive(Clone, Copy, Debug, PartialEq, Default, Eq)]
pub struct HostFunctions;

impl light_client_common::HostFunctions for HostFunctions {
	type BlakeTwo256 = BlakeTwo256;
}

impl grandpa_light_client_primitives::HostFunctions for HostFunctions {
	fn ed25519_verify(
		sig: &sp_core::ed25519::Signature,
		msg: &[u8],
		pub_key: &sp_core::ed25519::Public,
	) -> bool {
		todo!()
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	let client = GrandpaClient::<HostFunctions>::default();
	match msg {
		ExecuteMsg::ValidateMsg(validate_msg) => todo!(),
		ExecuteMsg::StatusMsg(_) => todo!(),
		ExecuteMsg::ExportedMetadataMsg(_) => todo!(),
		ExecuteMsg::ZeroCustomFieldsMsg(_) => todo!(),
		ExecuteMsg::GetTimestampAtHeightMsg(_) => todo!(),
		ExecuteMsg::InitializeMsg(_) => todo!(),
		ExecuteMsg::VerifyMembershipMsg(_) => todo!(),
		ExecuteMsg::VerifyClientMessage(msg) => {
			// let ctx: &GoContext<HostFunctions> = ();
			// let client_id = ();
			// client.verify_client_message(&ctx, client_id, client_state, client_message)?;
			panic!()
		},
		ExecuteMsg::CheckForMisbehaviourMsg(_) => todo!(),
		ExecuteMsg::UpdateStateOnMisbehaviourMsg(_) => todo!(),
		ExecuteMsg::UpdateStateMsg(_) => todo!(),
		ExecuteMsg::CheckSubstituteAndUpdateStateMsg(_) => todo!(),
		ExecuteMsg::VerifyUpgradeAndUpdateStateMsg(_) => todo!(),
	}
	Ok(Response::default())
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
