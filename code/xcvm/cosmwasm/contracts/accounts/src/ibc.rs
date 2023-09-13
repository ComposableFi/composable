use xc_core::NetworkId;

use crate::{
	accounts, auth,
	error::{ContractError, Result},
	msg, state,
};
use cosmwasm_std::{
	DepsMut, Env, Ibc3ChannelOpenResponse, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg,
	IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcOrder, IbcPacketReceiveMsg,
	IbcReceiveResponse, Response, Storage,
};
use cw_storage_plus::Map;

use xc_core::proto::Isomorphism;

/// Information about given IBC channel.
const IBC_CHANNEL_INFO: Map<String, NetworkId> = Map::new(state::IBC_CHANNEL_INFO_NS);
/// Mapping from network id to IBC channel an escrow account is listening on.
const IBC_NETWORK_CHANNEL: Map<u32, String> = Map::new(state::IBC_NETWORK_CHANNEL_NS);

// TODO(mina86): Add a way to fill out those maps.

pub(crate) fn get_network_id_for_channel(
	storage: &dyn Storage,
	channel_id: String,
) -> Result<Option<NetworkId>> {
	IBC_CHANNEL_INFO.may_load(storage, channel_id).map_err(ContractError::from)
}

#[cosmwasm_std::entry_point]
pub fn ibc_channel_open(
	_deps: DepsMut,
	_env: Env,
	msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse> {
	fn check_version(version: String) -> Result<String> {
		if version == xc_core::accounts::IBC_VERSION {
			Ok(version)
		} else {
			Err(ContractError::InvalidIbcVersion(version))
		}
	}

	let channel = match msg {
		IbcChannelOpenMsg::OpenInit { channel } => channel,
		IbcChannelOpenMsg::OpenTry { channel, counterparty_version } => {
			check_version(counterparty_version)?;
			channel
		},
	};
	let version = check_version(channel.version)?;
	if channel.order != IbcOrder::Unordered {
		Err(ContractError::InvalidIbcOrdering(channel.order))
	} else {
		Ok(Some(Ibc3ChannelOpenResponse { version }))
	}
}

#[cosmwasm_std::entry_point]
pub fn ibc_channel_connect(
	_deps: DepsMut,
	_env: Env,
	msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse> {
	let IbcChannel { endpoint, .. } = msg.into();
	Ok(IbcBasicResponse::new().add_event(
		msg::make_event(msg::Action::IbcConnect).add_attribute("channel_id", endpoint.channel_id),
	))
}

#[cosmwasm_std::entry_point]
pub fn ibc_channel_close(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse> {
	let IbcChannel { endpoint, counterparty_endpoint, .. } = msg.into();
	let path = IBC_CHANNEL_INFO.key(counterparty_endpoint.channel_id);
	if let Some(network_id) = path.may_load(deps.storage)? {
		path.remove(deps.storage);
		IBC_NETWORK_CHANNEL.remove(deps.storage, network_id.0);
	}
	Ok(IbcBasicResponse::new().add_event(
		msg::make_event(msg::Action::IbcClose).add_attribute("channel_id", endpoint.channel_id),
	))
}

/// Handles an incoming IBC packet.
///
/// Determines network of the escrow contract which sent the message based on
/// the source IBC packet and then forwards handling of the packet to
/// [`handle_packet`].
///
/// The function never returns an error.  If error is encountered, an empty
/// response is returned instead.
#[cosmwasm_std::entry_point]
pub fn ibc_packet_receive(
	deps: DepsMut,
	env: Env,
	msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, std::convert::Infallible> {
	let response = (|| {
		let packet = msg.packet;
		let auth = auth::EscrowContract::authorise_ibc(deps.storage, &env, packet.src)?;
		let packet = msg::Packet::decode(packet.data.as_slice())?;
		handle_packet(auth, deps, env, packet)
	})();
	Ok(response.map_or_else(|_| Default::default(), IbcReceiveResponse::from))
}

/// Handles a packet sent from an escrow contract.
///
/// Typically this is called from [`ibc_packet_receive`] when a cross-chain
/// packet comes from an escrow contract on another account.  However, it may
/// also come from escrow contract on local chain when it’s sent as
/// [`msg::ExecuteMsg::LocalPacket`].
///
/// Whatever the case, it returns [`PacketResponse`] with response body.  The
/// object can be easily converted into [`IbcReceiveResponse`] or [`Response`]
/// as necessary.
///
/// Returns an error if packet cannot be decoded.
pub(crate) fn handle_packet(
	auth: auth::EscrowContract,
	mut deps: DepsMut,
	env: Env,
	packet: msg::Packet,
) -> Result<EmptyResponse> {
	match packet {
		msg::Packet::DepositNotification(packet) =>
			accounts::handle_deposit_notification(auth, deps, packet),
		msg::Packet::RelayedRequest(packet) => {
			let auth = auth::Account::authorise_remote(
				&auth,
				&mut deps,
				&env,
				packet.account,
				packet.address,
			)?;
			match packet.request {
				msg::RelayedRequest::ExecuteSolution(req) =>
					accounts::handle_submit_problem(auth, deps, req),
				msg::RelayedRequest::DropAccount(req) =>
					accounts::handle_drop_account(auth, deps, req),
			}
		},
	}
}

/// An empty successful response to requests which may be sent via IBC packet or
/// as regular requests.
pub(crate) struct EmptyResponse;

impl From<EmptyResponse> for Response {
	fn from(_: EmptyResponse) -> Self {
		Self::default()
	}
}

impl From<EmptyResponse> for IbcReceiveResponse {
	fn from(_: EmptyResponse) -> Self {
		let data = Result::<(), String>::Ok(()).encode();
		Self::default().set_ack(data)
	}
}
