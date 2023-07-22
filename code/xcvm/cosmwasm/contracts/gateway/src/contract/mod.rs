pub(crate) mod execute;
pub mod ibc;

extern crate alloc;

use crate::{
	assets, auth,
	error::{ContractError, ContractResult},
	events::make_event,
	exec, msg, state,
};

use cosmwasm_std::{
	to_binary, wasm_execute, Binary, CosmosMsg, Deps, DepsMut, Env, Ibc3ChannelOpenResponse,
	IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
	IbcChannelOpenResponse, IbcMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg,
	IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, IbcTimeoutBlock, MessageInfo, Reply,
	Response, SubMsg, SubMsgResult,
};
use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;
use cw_utils::ensure_from_older_version;
use xc_core::{
	proto::{decode_packet, Encodable},
	shared::XcPacket,
	CallOrigin, Displayed, Funds, XCVMAck,
};

use self::ibc::make_ibc_failure_event;

const CONTRACT_NAME: &str = "composable:xcvm-gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const TRANSFER_PROGRAM_REPLY_ID: u64 = 0;
const EXEC_PROGRAM_REPLY_ID: u64 = 1;
const INSTANTIATE_INTERPRETER_REPLY_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: msg::InstantiateMsg,
) -> ContractResult<Response> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	state::Config {
		interpreter_code_id: msg.interpreter_code_id,
		network_id: msg.network_id,
		admin: deps.api.addr_validate(&msg.admin)?,
		ibc_ics_20_sender: msg.ibc_ics_20_sender,
	}
	.save(deps.storage)?;

	Ok(Response::default().add_event(make_event("instantiated")))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: msg::MigrateMsg) -> ContractResult<Response> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> ContractResult<Binary> {
	match msg {
		msg::QueryMsg::LookupAsset { asset_id } => assets::query_lookup(deps, asset_id)
			.and_then(|resp| to_binary(&resp).map_err(ContractError::from)),
	}
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> ContractResult<Response> {
	match msg.id {
		EXEC_PROGRAM_REPLY_ID => handle_exec_reply(msg),
		INSTANTIATE_INTERPRETER_REPLY_ID =>
			exec::instantiate_reply(deps, msg).map_err(ContractError::from),
		_ => Err(ContractError::UnknownReply),
	}
}

fn handle_exec_reply(msg: Reply) -> ContractResult<Response> {
	let (data, event) = match msg.result {
		SubMsgResult::Ok(_) =>
			(XCVMAck::OK, make_event("receive").add_attribute("result", "success")),
		SubMsgResult::Err(err) => (XCVMAck::FAIL, make_ibc_failure_event(err.to_string())),
	};
	Ok(Response::default().add_event(event).set_data(data))
}
