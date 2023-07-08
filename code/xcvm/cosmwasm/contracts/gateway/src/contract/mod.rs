mod execute;

extern crate alloc;

use crate::{
	assets, auth,
	error::{ContractError, ContractResult},
	exec, msg, state, events::make_event,
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
use cw_xc_common::shared::DefaultXCVMPacket;
use xc_core::{BridgeProtocol, CallOrigin, Displayed, Funds, XCVMAck};
use xc_core::proto::{decode_packet, Encodable};

const CONTRACT_NAME: &str = "composable:xcvm-gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const EXEC_PROGRAM_REPLY_ID: u64 = 0;
pub(crate) const INSTANTIATE_INTERPRETER_REPLY_ID: u64 = 1;

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
			exec::handle_instantiate_reply(deps, msg).map_err(ContractError::from),
		_ => Err(ContractError::UnknownReply),
	}
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_open(
	_deps: DepsMut,
	_env: Env,
	msg: IbcChannelOpenMsg,
) -> ContractResult<IbcChannelOpenResponse> {
	let (channel, version) = match msg {
		IbcChannelOpenMsg::OpenInit { channel } => (channel, None),
		IbcChannelOpenMsg::OpenTry { channel, counterparty_version } =>
			(channel, Some(counterparty_version)),
	};
	const IBC_VERSION: &str = cw_xc_common::gateway::IBC_VERSION;
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
) -> ContractResult<IbcBasicResponse> {
	let channel = msg.channel();
	state::IBC_CHANNEL_INFO.save(
		deps.storage,
		channel.endpoint.channel_id.clone(),
		&state::ChannelInfo {
			id: channel.endpoint.channel_id.clone(),
			counterparty_endpoint: channel.counterparty_endpoint.clone(),
			connection_id: channel.connection_id.clone(),
		},
	)?;
	Ok(IbcBasicResponse::new().add_event(
		make_event("ibc_connect")
			.add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_close(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelCloseMsg,
) -> ContractResult<IbcBasicResponse> {
	let channel = msg.channel();
	match state::IBC_CHANNEL_NETWORK.load(deps.storage, channel.endpoint.channel_id.clone()) {
		Ok(channel_network) => {
			state::IBC_CHANNEL_NETWORK.remove(deps.storage, channel.endpoint.channel_id.clone());
			state::IBC_NETWORK_CHANNEL.remove(deps.storage, channel_network);
		},
		// Nothing to do, the channel might have never been registered to a network.
		Err(_) => {},
	}
	state::IBC_CHANNEL_INFO.remove(deps.storage, channel.endpoint.channel_id.clone());
	// TODO: are all the in flight packets timed out in this case? if not, we need to unescrow
	// assets
	Ok(IbcBasicResponse::new().add_event(
		make_event("ibc_close")
			.add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_receive(
	_deps: DepsMut,
	env: Env,
	msg: IbcPacketReceiveMsg,
) -> ContractResult<IbcReceiveResponse> {
	let response = IbcReceiveResponse::default().add_event(make_event("receive"));
	let msg = (|| -> ContractResult<_> {
		let packet: DefaultXCVMPacket =
			decode_packet(&msg.packet.data).map_err(ContractError::Protobuf)?;
		let call_origin = CallOrigin::Remote {
			protocol: BridgeProtocol::IBC,
			relayer: msg.relayer,
			user_origin: packet.user_origin,
		};
		let execute_program = msg::ExecuteProgramMsg {
			salt: packet.salt,
			program: packet.program,
			assets: packet.assets,
		};
		let msg = msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program };
		let msg = wasm_execute(env.contract.address, &msg, Default::default())?;
		Ok(SubMsg::reply_always(msg, EXEC_PROGRAM_REPLY_ID))
	})();
	Ok(match msg {
		Ok(msg) => response.set_ack(XCVMAck::OK).add_submessage(msg),
		Err(err) =>
			response.add_event(make_ibc_failure_event(err.to_string())).set_ack(XCVMAck::KO),
	})
}

fn make_ibc_failure_event(reason: String) -> cosmwasm_std::Event {
	make_event("receive")
		.add_attribute("result", "failure")
		.add_attribute("reason", reason.to_string())
}

fn handle_exec_reply(msg: Reply) -> ContractResult<Response> {
	let (data, event) = match msg.result {
		SubMsgResult::Ok(_) =>
			(XCVMAck::OK, make_event("receive").add_attribute("result", "success")),
		SubMsgResult::Err(err) => (XCVMAck::KO, make_ibc_failure_event(err.to_string())),
	};
	Ok(Response::default().add_event(event).set_data(data))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_ack(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketAckMsg,
) -> ContractResult<IbcBasicResponse> {
	let ack = XCVMAck::try_from(msg.acknowledgement.data.as_slice())
		.map_err(|_| ContractError::InvalidAck)?;
	let packet: DefaultXCVMPacket =
		decode_packet(&msg.original_packet.data).map_err(ContractError::Protobuf)?;
	let messages = match ack {
		XCVMAck::OK => {
			// We got the ACK
			burn_escrowed_assets(deps, packet.assets)
		},
		XCVMAck::KO => {
			// On failure, return the funds
			unescrow_assets(
				&deps,
				// Safe as impossible to tamper.
				String::from_utf8_lossy(&packet.interpreter).to_string(),
				packet.assets,
			)
		},
		_ => Err(ContractError::InvalidAck),
	}?;
	Ok(IbcBasicResponse::default()
		.add_event(make_event("ack").add_attribute("ack", ack.value().to_string()))
		.add_messages(messages))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_timeout(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketTimeoutMsg,
) -> ContractResult<IbcBasicResponse> {
	let packet: DefaultXCVMPacket =
		decode_packet(&msg.packet.data).map_err(ContractError::Protobuf)?;
	// On timeout, return the funds
	let burns = unescrow_assets(
		&deps,
		// Safe as impossible to tamper.
		String::from_utf8_lossy(&packet.interpreter).to_string(),
		packet.assets,
	)?;
	Ok(IbcBasicResponse::default().add_messages(burns))
}

fn burn_escrowed_assets(
	deps: DepsMut,
	assets: Funds<Displayed<u128>>,
) -> ContractResult<Vec<CosmosMsg>> {
	assets
		.into_iter()
		.map(|(asset_id, Displayed(amount))| {
			let reference = assets::query_lookup(deps.as_ref(), asset_id)?.reference;
			match &reference {
				msg::AssetReference::Native { .. } => Err(ContractError::UnsupportedAsset),
				msg::AssetReference::Virtual { cw20_address } => {
					// Burn from the current contract.
					Ok(wasm_execute(
						cw20_address.to_string(),
						&Cw20ExecuteMsg::Burn { amount: amount.into() },
						Default::default(),
					)?
					.into())
				},
			}
		})
		.collect::<Result<Vec<_>, _>>()
}

fn unescrow_assets(
	deps: &DepsMut,
	sender: String,
	assets: Funds<Displayed<u128>>,
) -> ContractResult<Vec<CosmosMsg>> {
	assets
		.into_iter()
		.map(|(asset_id, Displayed(amount))| {
			let reference = assets::query_lookup(deps.as_ref(), asset_id)?.reference;
			match &reference {
				msg::AssetReference::Native { .. } => Err(ContractError::UnsupportedAsset),
				msg::AssetReference::Virtual { cw20_address } => {
					// Transfer from the sender to the gateway
					Ok(wasm_execute(
						cw20_address.to_string(),
						&Cw20ExecuteMsg::Transfer {
							recipient: sender.clone(),
							amount: amount.into(),
						},
						Default::default(),
					)?
					.into())
				},
			}
		})
		.collect::<Result<Vec<_>, _>>()
}
