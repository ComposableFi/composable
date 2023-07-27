extern crate alloc;

use crate::{accounts, auth, error::Result, ibc, msg, state};

use cosmwasm_std::{
	Binary, Deps, DepsMut, Env, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
	IbcChannelOpenMsg, IbcChannelOpenResponse, IbcPacketReceiveMsg, IbcReceiveResponse,
	MessageInfo, Response,
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
	state::Config::try_instantiate(deps.api, &msg)?.save(deps.storage)?;
	for admin in msg.admins {
		auth::Admin::add(deps.storage, deps.api.addr_validate(&admin)?)?;
	}
	Ok(Response::default().add_event(msg::make_event(msg::Action::Instantiated)))
}

#[cosmwasm_std::entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> Result {
	match msg {
		msg::ExecuteMsg::CreateAccount(req) => {
			let auth = auth::User::authorise(deps.storage, &env)?;
			accounts::handle_create_account(auth, deps, info, req)
		},
		msg::ExecuteMsg::DropAccount(req) => {
			let auth = auth::Account::authorise(deps.storage, &env, info)?;
			accounts::handle_drop_account(auth, deps, req).map(Into::into)
		},
		msg::ExecuteMsg::SubmitProblem(req) => {
			let auth = auth::Account::authorise(deps.storage, &env, info)?;
			accounts::handle_submit_problem(auth, deps, req).map(Into::into)
		},
		msg::ExecuteMsg::LocalPacket(packet) => {
			let auth = auth::EscrowContract::authorise_local(deps.storage, &env, info)?;
			let response = handle_packet(deps, env, auth, packet)?;
			Ok(response.into())
		},
		msg::ExecuteMsg::BreakGlass => {
			let auth = auth::Admin::authorise(deps.storage, info)?;
			auth::handle_break_glass(auth, deps, env)
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

#[cosmwasm_std::entry_point]
pub fn ibc_packet_receive(
	deps: DepsMut,
	env: Env,
	msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, cosmwasm_std::Never> {
	Ok((|| {
		let packet = msg.packet;
		let auth = auth::EscrowContract::authorise(deps.storage, &env, packet.src)?;
		let packet = ibc::decode::<msg::Packet>(packet.data)?;
		handle_packet(deps, env, auth, packet)
	})()
	.map_or_else(|_| Default::default(), IbcReceiveResponse::from))
}

/// Handles a cross-chain packet.
fn handle_packet(
	mut deps: DepsMut,
	env: Env,
	auth: auth::EscrowContract,
	packet: msg::Packet,
) -> Result<PacketResponse> {
	match packet {
		msg::Packet::DepositNotification(packet) =>
			accounts::handle_deposit_notification(auth, deps, packet),
		msg::Packet::RelayedRequest(packet) => {
			let auth = auth::Account::authorise_remote(
				&mut deps,
				&env,
				packet.account,
				auth.network_id(),
				packet.address,
			)?;
			match packet.request {
				msg::RelayedRequest::SubmitProblem(req) =>
					accounts::handle_submit_problem(auth, deps, req),
				msg::RelayedRequest::DropAccount(req) =>
					accounts::handle_drop_account(auth, deps, req),
			}
		},
	}
}

/// Helper type for handling responses to received packets.
///
/// This abstracts differences in successful responses when packet is received
/// over IBC or through local call from contract on the same chain.
pub(crate) struct PacketResponse {
	data: Binary,
}

impl PacketResponse {
	pub fn new(data: Vec<u8>) -> Self {
		Self { data: Binary::from(data) }
	}
}

impl core::convert::From<PacketResponse> for Response {
	fn from(response: PacketResponse) -> Self {
		Self::new().set_data(response.data)
	}
}

impl core::convert::From<PacketResponse> for IbcReceiveResponse {
	fn from(response: PacketResponse) -> Self {
		Self::new().set_ack(response.data)
	}
}
