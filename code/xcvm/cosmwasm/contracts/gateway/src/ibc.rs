use crate::{
	contract::XCVM_GATEWAY_EVENT_PREFIX,
	error::ContractError,
	state::{ChannelInfo, IBC_CHANNEL_INFO, IBC_CHANNEL_NETWORK, IBC_NETWORK_CHANNEL},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	DepsMut, Env, Event, Ibc3ChannelOpenResponse, IbcBasicResponse, IbcChannelCloseMsg,
	IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcOrder, IbcPacketAckMsg,
	IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse,
};

pub const XCVM_GATEWAY_IBC_VERSION: &str = "xcvm-gateway-v0";
pub const XCVM_GATEWAY_IBC_ORDERING: IbcOrder = IbcOrder::Unordered;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
	_deps: DepsMut,
	_env: Env,
	msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
	let channel = msg.channel().clone();
	match (msg.counterparty_version(), channel.order) {
		// If the version is specified and does match, cancel handshake.
		(Some(counter_version), _) if counter_version != XCVM_GATEWAY_IBC_VERSION =>
			Err(ContractError::InvalidIbcVersion(counter_version.to_string())),
		// If the order is not the expected one, cancel handshake.
		(_, order) if order != XCVM_GATEWAY_IBC_ORDERING =>
			Err(ContractError::InvalidIbcOrdering(order)),
		// In any other case, overwrite the version.
		_ => Ok(Some(Ibc3ChannelOpenResponse { version: XCVM_GATEWAY_IBC_VERSION.to_string() })),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
	let channel = msg.channel();
	IBC_CHANNEL_INFO.save(
		deps.storage,
		channel.endpoint.channel_id.clone(),
		&ChannelInfo {
			id: channel.endpoint.channel_id.clone(),
			counterparty_endpoint: channel.counterparty_endpoint.clone(),
			connection_id: channel.connection_id.clone(),
		},
	)?;
	Ok(IbcBasicResponse::new().add_event(
		Event::new(XCVM_GATEWAY_EVENT_PREFIX)
			.add_attribute("action", "ibc_connect")
			.add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
	let channel = msg.channel();
	match IBC_CHANNEL_NETWORK.load(deps.storage, channel.endpoint.channel_id.clone()) {
		Ok(channel_network) => {
			IBC_CHANNEL_NETWORK.remove(deps.storage, channel.endpoint.channel_id.clone());
			IBC_NETWORK_CHANNEL.remove(deps.storage, channel_network);
		},
		// Nothing to do, the channel might have never been registered to a network.
		Err(_) => {},
	}
	IBC_CHANNEL_INFO.remove(deps.storage, channel.endpoint.channel_id.clone());
	Ok(IbcBasicResponse::new().add_event(
		Event::new(XCVM_GATEWAY_EVENT_PREFIX)
			.add_attribute("action", "ibc_close")
			.add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
	_deps: DepsMut,
	_env: Env,
	_msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
	todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
	_deps: DepsMut,
	_env: Env,
	_msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
	todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
	_deps: DepsMut,
	_env: Env,
	_msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
	todo!()
}
