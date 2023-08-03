extern crate alloc;

use crate::{auth, deposits, error::Result, ibc, msg};

use cosmwasm_std::{
	Binary, Deps, DepsMut, Env, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
	IbcChannelOpenMsg, IbcChannelOpenResponse, IbcPacketAckMsg, IbcPacketTimeoutMsg, MessageInfo,
	Response,
};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;

const CONTRACT_NAME: &str = "composable:xcvm-gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cosmwasm_std::entry_point]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: msg::InstantiateMsg,
) -> Result {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	for admin in msg.admins {
		auth::Admin::add(deps.storage, deps.api.addr_validate(&admin)?)?;
	}
	deposits::init_state(deps.storage)?;
	Ok(Response::default().add_event(msg::make_event(msg::Action::Instantiated)))
}

#[cosmwasm_std::entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> Result {
	match msg {
		msg::ExecuteMsg::DepositAssets(msg) =>
			deposits::handle_deposit_request(deps, env, info, msg),
		msg::ExecuteMsg::Receive(msg) => {
			let auth = auth::Cw20Contract::authorise(deps.storage, info.sender)?;
			deposits::handle_receive(auth, deps, env, msg)
		},
		msg::ExecuteMsg::Relay(req) => {
			let auth = auth::User::authorise(deps.storage, &env)?;
			handle_relay(auth, deps, env, info, req)
		},
		msg::ExecuteMsg::BreakGlass => {
			let auth = auth::Admin::authorise(deps.storage, &info)?;
			auth::handle_break_glass(auth, deps, env, info)
		},
	}
}

#[cosmwasm_std::entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: msg::MigrateMsg) -> Result {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cosmwasm_std::entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: msg::QueryMsg) -> Result<Binary> {
	unreachable!()
}

#[cosmwasm_std::entry_point]
pub fn ibc_channel_open(
	_deps: DepsMut,
	_env: Env,
	_msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse> {
	todo!()
}

#[cosmwasm_std::entry_point]
pub fn ibc_channel_connect(
	_deps: DepsMut,
	_env: Env,
	_msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse> {
	todo!()
}

#[cosmwasm_std::entry_point]
pub fn ibc_channel_close(
	_deps: DepsMut,
	_env: Env,
	_msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse> {
	todo!()
}

/// Relays a message to the accounts contract.
fn handle_relay(
	_: auth::User,
	_deps: DepsMut,
	_env: Env,
	info: MessageInfo,
	req: msg::RelayRequest,
) -> Result {
	let packet = msg::accounts::RelayedRequestPacket {
		address: info.sender.into(),
		account: req.account,
		request: req.request,
	};
	let packet = msg::accounts::Packet::from(packet);
	Ok(Response::default().add_message(crate::ibc::make_message(&packet)))
}

#[cosmwasm_std::entry_point]
pub fn ibc_packet_ack(deps: DepsMut, _env: Env, msg: IbcPacketAckMsg) -> Result<IbcBasicResponse> {
	ibc_packet_done(deps, msg.original_packet, Some(msg.acknowledgement.data))
}

#[cosmwasm_std::entry_point]
pub fn ibc_packet_timeout(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse> {
	ibc_packet_done(deps, msg.packet, None)
}

/// Handler for an acknowledged or timed-out packet.
///
/// If `ack` is `Some` than the packet has been delivered successfully and it’s
/// the acknowledgement packet sent by the destination.  Otherwise, the packet
/// has timed out.
fn ibc_packet_done(
	deps: DepsMut,
	packet: cosmwasm_std::IbcPacket,
	ack: Option<Binary>,
) -> Result<IbcBasicResponse> {
	match ibc::decode::<msg::accounts::Packet>(packet.data)? {
		msg::accounts::Packet::DepositNotification(packet) =>
			deposits::handle_deposit_done(deps, packet, ack),
		msg::accounts::Packet::RelayedRequest(_) => {
			// TODO: Ignore for now.  On timeouts user will have to notice and
			// retry themselves.  Ideally we would notify user somehow.
			Ok(IbcBasicResponse::default())
		},
	}
}
