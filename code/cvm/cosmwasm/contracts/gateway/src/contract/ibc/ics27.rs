//! ics27 integration to do txes

use std::str::FromStr;

use crate::{
	auth,
	contract::ReplyId,
	error::{ContractError, Result},
	events::make_event,
	msg,
	network::load_other,
	state,
};

use cosmwasm_std::{
	ensure_eq, wasm_execute, Binary, BlockInfo, DepsMut, Env, Ibc3ChannelOpenResponse,
	IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
	IbcChannelOpenResponse, IbcMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg,
	IbcPacketTimeoutMsg, IbcReceiveResponse, MessageInfo, Response, SubMsg,
};
use ibc_rs_scale::core::ics24_host::identifier::{ChannelId, ConnectionId};
use xc_core::{
	proto::Isomorphism, shared::XcPacket, transport::ibc::ChannelInfo, CallOrigin, XCVMAck,
};

use super::make_ibc_failure_event;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_open(
	_deps: DepsMut,
	_env: Env,
	msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse> {
	let (channel, version) = match msg {
		IbcChannelOpenMsg::OpenInit { channel } => (channel, None),
		IbcChannelOpenMsg::OpenTry { channel, counterparty_version } =>
			(channel, Some(counterparty_version)),
	};
	const IBC_VERSION: &str = xc_core::gateway::IBC_VERSION;
	if version.is_some() && version.as_deref() != Some(IBC_VERSION) {
		Err(ContractError::InvalidIbcVersion(version.unwrap()))
	} else if channel.order != IbcOrder::Unordered {
		Err(ContractError::InvalidIbcOrdering(channel.order))
	} else {
		let version = version.unwrap_or_else(|| String::from(IBC_VERSION));
		Ok(Some(Ibc3ChannelOpenResponse { version }))
	}
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_connect(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse> {
	let channel = msg.channel();
	state::xcvm::IBC_CHANNEL_INFO.save(
		deps.storage,
		channel.endpoint.channel_id.clone(),
		&ChannelInfo {
			id: ChannelId::from_str(&channel.endpoint.channel_id)?,
			counterparty_endpoint: channel.counterparty_endpoint.clone(),
			connection_id: ConnectionId::from_str(&channel.connection_id)?,
		},
	)?;
	Ok(IbcBasicResponse::new().add_event(
		make_event("ibc_connect").add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_close(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse> {
	let channel = msg.channel();
	state::xcvm::IBC_CHANNEL_INFO.remove(deps.storage, channel.endpoint.channel_id.clone());

	state::xcvm::IBC_CHANNEL_NETWORK.remove(deps.storage, channel.endpoint.channel_id.clone());
	Ok(IbcBasicResponse::new().add_event(
		make_event("ibc_close").add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_receive(
	_deps: DepsMut,
	env: Env,
	msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse> {
	let response = IbcReceiveResponse::default().add_event(make_event("receive"));
	let msg = (|| -> Result<_> {
		let packet = XcPacket::decode(&msg.packet.data)?;
		let call_origin = CallOrigin::Remote { user_origin: packet.user_origin };
		let execute_program = msg::ExecuteProgramMsg {
			salt: packet.salt,
			program: packet.program,
			assets: packet.assets,
			tip: Some(msg.relayer.to_string()),
		};

		let msg = msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program };
		let msg = wasm_execute(env.contract.address, &msg, Default::default())?;
		Ok(SubMsg::reply_always(msg, ReplyId::ExecProgram.into()))
	})();
	Ok(match msg {
		Ok(msg) => response.set_ack(XCVMAck::Ok).add_submessage(msg),
		Err(err) => response
			.add_event(make_ibc_failure_event(err.to_string()))
			.set_ack(XCVMAck::Fail),
	})
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_ack(_deps: DepsMut, _env: Env, msg: IbcPacketAckMsg) -> Result<IbcBasicResponse> {
	let ack = XCVMAck::try_from(msg.acknowledgement.data.as_slice())
		.map_err(|_| ContractError::InvalidAck)?;
	XcPacket::decode(&msg.original_packet.data)?;
	Ok(IbcBasicResponse::default().add_event(make_event("ack").add_attribute("ack", ack)))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_timeout(
	_deps: DepsMut,
	_env: Env,
	msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse> {
	XcPacket::decode(&msg.packet.data)?;
	// https://github.com/cosmos/ibc/pull/998
	Ok(IbcBasicResponse::default())
}

/// Handle a request gateway message.
/// The call must originate from an interpreter.
pub(crate) fn handle_bridge_forward_no_assets(
	_: auth::Executor,
	deps: DepsMut,
	info: MessageInfo,
	msg: msg::BridgeForwardMsg,
	block: BlockInfo,
) -> Result<Response> {
	ensure_eq!(msg.msg.assets.0.len(), 0, ContractError::CannotTransferAssets);
	let other = load_other(deps.storage, msg.to)?;
	let channel_id = other
		.connection
		.ics27_channel
		.map(|x| x.id)
		.ok_or(ContractError::UnknownChannel)?;
	let packet = XcPacket {
		interpreter: String::from(info.sender).into_bytes(),
		user_origin: msg.executor_origin.user_origin,
		salt: msg.msg.salt,
		program: msg.msg.program,
		assets: msg.msg.assets,
	};
	let mut event = make_event("bridge")
		.add_attribute("network_id", msg.to.to_string())
		.add_attribute(
			"assets",
			serde_json_wasm::to_string(&packet.assets)
				.map_err(|_| ContractError::FailedToSerialize)?,
		)
		.add_attribute(
			"program",
			serde_json_wasm::to_string(&packet.program)
				.map_err(|_| ContractError::FailedToSerialize)?,
		);
	if !packet.salt.is_empty() {
		let salt_attr = Binary::from(packet.salt.as_slice()).to_string();
		event = event.add_attribute("salt", salt_attr);
	}
	Ok(Response::default().add_event(event).add_message(IbcMsg::SendPacket {
		channel_id: channel_id.to_string(),
		data: Binary::from(packet.encode()),
		// TODO: should be a parameter or configuration
		timeout: other.connection.counterparty_timeout.absolute(block),
	}))
}
