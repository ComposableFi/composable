pub mod execute;
pub mod ibc;
pub mod query;
pub mod sudo;

use crate::{
	error::{ContractError, Result},
	events::make_event,
	msg, state,
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Reply, Response, SubMsgResponse, SubMsgResult};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;
use xc_core::{
	transport::ibc::{ics20::MsgTransferResponse, TransportTrackerId},
	XCVMAck,
};

use self::ibc::make_ibc_failure_event;

const CONTRACT_NAME: &str = "composable:xcvm-gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq, Debug, Clone, Copy, enumn::N)]
#[repr(u64)]
pub enum ReplyId {
	InstantiateInterpreter = 0,
	ExecProgram = 2,
	TransportSent = 3,
}

impl From<ReplyId> for u64 {
	fn from(val: ReplyId) -> Self {
		val as u64
	}
}

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
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response> {
	deps.api.debug(&format!("cvm::cosmwasm::reply {msg:?}"));
	if let Some(reply_id) = ReplyId::n(msg.id) {
		return match reply_id {
			ReplyId::InstantiateInterpreter =>
				crate::interpreter::handle_instantiate_reply(deps, msg).map_err(ContractError::from),
			ReplyId::TransportSent => handle_transfer_sent(deps, msg),
			ReplyId::ExecProgram => handle_exec_reply(msg),
		}
	}
	Err(ContractError::UnknownReply)
}

fn handle_transfer_sent(deps: DepsMut, msg: Reply) -> Result {
	deps.api.debug(&format!("cvm::gateway::handle_transfer_sent {:?}", msg.result));
	let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result else {
		return Err(ContractError::FailedIBCTransfer(format!("cvm::failed reply: {:?}", msg.result)))
	};

	use prost::Message;
	let transfer_response = MsgTransferResponse::decode(&b[..]).map_err(|_e| {
		ContractError::FailedIBCTransfer(format!("could not decode response: {b}"))
	})?;

	let (bridge, route) = state::tracking::bridge_unlock(deps.storage)?;
	let tracker_id = TransportTrackerId::Ibc {
		channel_id: route.channel_to_send_over.clone(),
		sequence: transfer_response.sequence,
	};
	state::tracking::track(deps.storage, tracker_id, bridge)?;

	Ok(Response::new().add_event(
		make_event("bridge.track.added")
			.add_attribute("channel_id", route.channel_to_send_over.to_string())
			.add_attribute("sequence", transfer_response.sequence.to_string()),
	))
}

fn handle_exec_reply(msg: Reply) -> Result {
	let (data, event) = match msg.result {
		SubMsgResult::Ok(_) =>
			(XCVMAck::Ok, make_event("receive").add_attribute("result", "success")),
		SubMsgResult::Err(err) => (XCVMAck::Fail, make_ibc_failure_event(err)),
	};
	Ok(Response::default().add_event(event).set_data(data))
}
