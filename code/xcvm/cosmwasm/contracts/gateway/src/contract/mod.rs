pub mod execute;
pub mod ibc;
pub mod sudo;

use crate::{
	assets,
	error::{ContractError, Result},
	events::make_event,
	msg, state,
};

use cosmwasm_std::{
	to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, SubMsgResult,
};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;
use xc_core::XCVMAck;

use self::{execute::handle_instantiate_reply, ibc::make_ibc_failure_event};

const CONTRACT_NAME: &str = "composable:xcvm-gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INSTANTIATE_INTERPRETER_REPLY_ID: u64 = 0;
pub const TRANSFER_PROGRAM_REPLY_ID: u64 = 1;
pub const EXEC_PROGRAM_REPLY_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: msg::InstantiateMsg,
) -> Result {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	state::save(deps.storage, &msg.0)?;

	Ok(Response::default().add_event(make_event("instantiated")))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: msg::MigrateMsg) -> Result {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> Result<Binary> {
	match msg {
		msg::QueryMsg::GetAssetById { asset_id } => assets::get_asset_by_id(deps, asset_id)
			.and_then(|resp| to_binary(&resp).map_err(ContractError::from)),
	}
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response> {
	match msg.id {
		EXEC_PROGRAM_REPLY_ID => handle_exec_reply(msg),
		INSTANTIATE_INTERPRETER_REPLY_ID =>
			handle_instantiate_reply(deps, msg).map_err(ContractError::from),
		_ => Err(ContractError::UnknownReply),
	}
}

fn handle_exec_reply(msg: Reply) -> Result {
	let (data, event) = match msg.result {
		SubMsgResult::Ok(_) =>
			(XCVMAck::Ok, make_event("receive").add_attribute("result", "success")),
		SubMsgResult::Err(err) => (XCVMAck::Fail, make_ibc_failure_event(err.to_string())),
	};
	Ok(Response::default().add_event(event).set_data(data))
}
