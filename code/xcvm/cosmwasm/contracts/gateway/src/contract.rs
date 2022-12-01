extern crate alloc;

use crate::{
	common::{ensure_admin, ensure_router},
	error::ContractError,
	msg::{InstantiateMsg, MigrateMsg, QueryMsg},
	state::{
		ChannelInfo, Config, CONFIG, IBC_CHANNEL_INFO, IBC_CHANNEL_NETWORK, IBC_NETWORK_CHANNEL,
		ROUTER,
	},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	wasm_execute, wasm_instantiate, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, Event,
	Ibc3ChannelOpenResponse, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg,
	IbcChannelOpenMsg, IbcChannelOpenResponse, IbcMsg, IbcOrder, IbcPacketAckMsg,
	IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, IbcTimeoutBlock,
	MessageInfo, Reply, Response, StdError, SubMsg, SubMsgResult,
};
use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;
use cw_utils::ensure_from_older_version;
use cw_xcvm_asset_registry::{contract::external_query_lookup_asset, msg::AssetReference};
use cw_xcvm_common::{gateway::ExecuteMsg, shared::BridgeMsg};
use cw_xcvm_utils::{DefaultXCVMPacket, DefaultXCVMProgram};
use xcvm_core::{
	BridgeProtocol, BridgeSecurity, CallOrigin, Displayed, Funds, NetworkId, UserOrigin, XCVMAck,
};
use xcvm_proto::{decode_packet, Encodable};

pub const CONTRACT_NAME: &str = "composable:xcvm-gateway";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const XCVM_GATEWAY_EVENT_PREFIX: &str = "xcvm.gateway";
pub const XCVM_GATEWAY_IBC_VERSION: &str = "xcvm-gateway-v0";
pub const XCVM_GATEWAY_IBC_ORDERING: IbcOrder = IbcOrder::Unordered;

pub const XCVM_GATEWAY_INSTANTIATE_ROUTER_REPLY_ID: u64 = 0;
pub const XCVM_GATEWAY_BATCH_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	mut msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	msg.config.registry_address =
		deps.api.addr_validate(&msg.config.registry_address)?.into_string();
	msg.config.admin = deps.api.addr_validate(&msg.config.admin)?.into_string();
	CONFIG.save(deps.storage, &msg.config)?;
	Ok(Response::default()
		.add_event(Event::new(XCVM_GATEWAY_EVENT_PREFIX).add_attribute("action", "instantiated"))
		.add_submessage(SubMsg::reply_on_success(
			wasm_instantiate(
				msg.config.router_code_id,
				&cw_xcvm_router::msg::InstantiateMsg {
					registry_address: msg.config.registry_address,
					interpreter_code_id: msg.config.interpreter_code_id,
					network_id: msg.config.network_id,
				},
				Default::default(),
				"xcvm-router".into(),
			)?,
			XCVM_GATEWAY_INSTANTIATE_ROUTER_REPLY_ID,
		)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	match msg {
		ExecuteMsg::IbcSetNetworkChannel { network_id, channel_id } => {
			ensure_admin(deps.as_ref(), info.sender.as_ref())?;
			let _ = IBC_CHANNEL_INFO
				.load(deps.storage, channel_id.clone())
				.map_err(|_| ContractError::UnknownChannel)?;
			IBC_NETWORK_CHANNEL.save(deps.storage, network_id, &channel_id)?;
			Ok(Response::default().add_event(
				Event::new(XCVM_GATEWAY_EVENT_PREFIX)
					.add_attribute("action", "set_network_channel")
					.add_attribute("network_id", format!("{network_id}"))
					.add_attribute("channel_id", channel_id),
			))
		},

		ExecuteMsg::Bridge {
			interpreter,
			msg: BridgeMsg { user_origin, network_id, security, salt, program, assets },
		} => handle_bridge(
			deps,
			info,
			interpreter,
			user_origin,
			network_id,
			security,
			salt,
			program,
			assets,
		),

		ExecuteMsg::Batch { msgs } =>
			if info.sender != env.contract.address {
				Err(ContractError::NotAuthorized)
			} else {
				Ok(Response::default().add_messages(msgs))
			},
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
	Err(StdError::generic_err("not implemented").into())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
	match msg.id {
		XCVM_GATEWAY_INSTANTIATE_ROUTER_REPLY_ID => handle_instantiate_reply(deps, msg),
		XCVM_GATEWAY_BATCH_REPLY_ID => handle_batch_reply(msg),
		_ => Err(ContractError::UnknownReply),
	}
}

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
	// TODO: are all the in flight packets timed out in this case? if not, we need to unescrow
	// assets
	Ok(IbcBasicResponse::new().add_event(
		Event::new(XCVM_GATEWAY_EVENT_PREFIX)
			.add_attribute("action", "ibc_close")
			.add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
	deps: DepsMut,
	env: Env,
	msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
	let batch = (|| -> Result<_, ContractError> {
		let Config { registry_address, .. } = CONFIG.load(deps.storage)?;
		let router_address = ROUTER.load(deps.storage)?;
		let packet: DefaultXCVMPacket =
			decode_packet(&msg.packet.data).map_err(ContractError::Protobuf)?;
		// Execute both mints + execution in a single sub-transaction.
		let mut msgs = mint_counterparty_assets(
			&deps,
			router_address.as_ref(),
			registry_address.as_ref(),
			packet.assets.clone(),
		)?;
		msgs.push(
			wasm_execute(
				router_address,
				&cw_xcvm_common::router::ExecuteMsg::ExecuteProgramPrivileged {
					call_origin: CallOrigin::Remote {
						protocol: BridgeProtocol::IBC,
						relayer: msg.relayer,
						user_origin: packet.user_origin,
					},
					salt: packet.salt,
					program: packet.program,
					assets: packet.assets,
				},
				Default::default(),
			)?
			.into(),
		);
		Ok(SubMsg::reply_always(
			wasm_execute(env.contract.address, &ExecuteMsg::Batch { msgs }, Default::default())?,
			XCVM_GATEWAY_BATCH_REPLY_ID,
		))
	})();
	match batch {
		Ok(batch) => Ok(IbcReceiveResponse::default()
			.set_ack(XCVMAck::OK.into_vec())
			.add_submessage(batch)),
		Err(_) => Ok(IbcReceiveResponse::default().set_ack(XCVMAck::KO.into_vec())),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
	let ack = XCVMAck::try_from(msg.acknowledgement.data.as_slice())
		.map_err(|_| ContractError::InvalidAck)?;
	let Config { registry_address, .. } = CONFIG.load(deps.storage)?;
	let packet: DefaultXCVMPacket =
		decode_packet(&msg.original_packet.data).map_err(ContractError::Protobuf)?;
	let messages = match ack {
		XCVMAck::OK => {
			// We got the ACK
			burn_escrowed_assets(deps, registry_address.as_str(), packet.assets)
		},
		XCVMAck::KO => {
			// On failure, return the funds
			unescrow_assets(
				&deps,
				// Safe as impossible to tamper.
				String::from_utf8_lossy(&packet.interpreter).to_string(),
				registry_address.as_str(),
				packet.assets,
			)
		},
		_ => Err(ContractError::InvalidAck),
	}?;
	Ok(IbcBasicResponse::default().add_messages(messages))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
	let Config { registry_address, .. } = CONFIG.load(deps.storage)?;
	let packet: DefaultXCVMPacket =
		decode_packet(&msg.packet.data).map_err(ContractError::Protobuf)?;
	// On timeout, return the funds
	let burns = unescrow_assets(
		&deps,
		// Safe as impossible to tamper.
		String::from_utf8_lossy(&packet.interpreter).to_string(),
		registry_address.as_str(),
		packet.assets,
	)?;
	Ok(IbcBasicResponse::default().add_messages(burns))
}

pub fn handle_batch_reply(msg: Reply) -> Result<Response, ContractError> {
	match msg.result {
		SubMsgResult::Ok(_) => Ok(Response::default().set_data(XCVMAck::OK.into_vec())),
		SubMsgResult::Err(_) => Ok(Response::default().set_data(XCVMAck::KO.into_vec())),
	}
}

fn mint_counterparty_assets(
	deps: &DepsMut,
	router_address: &str,
	registry_address: &str,
	assets: Funds<Displayed<u128>>,
) -> Result<Vec<CosmosMsg>, ContractError> {
	assets
		.into_iter()
		.map(|(asset_id, Displayed(amount))| {
			let reference =
				external_query_lookup_asset(deps.querier, registry_address.to_string(), asset_id)?;
			match &reference {
				AssetReference::Native { .. } => Err(ContractError::UnsupportedAsset),
				AssetReference::Virtual { cw20_address } => {
					// Burn from the current contract.
					Ok(wasm_execute(
						cw20_address.to_string(),
						&Cw20ExecuteMsg::Mint {
							recipient: router_address.to_string(),
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

fn burn_escrowed_assets(
	deps: DepsMut,
	registry_address: &str,
	assets: Funds<Displayed<u128>>,
) -> Result<Vec<CosmosMsg>, ContractError> {
	assets
		.into_iter()
		.map(|(asset_id, Displayed(amount))| {
			let reference =
				external_query_lookup_asset(deps.querier, registry_address.to_string(), asset_id)?;
			match &reference {
				AssetReference::Native { .. } => Err(ContractError::UnsupportedAsset),
				AssetReference::Virtual { cw20_address } => {
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
	registry_address: &str,
	assets: Funds<Displayed<u128>>,
) -> Result<Vec<CosmosMsg>, ContractError> {
	assets
		.into_iter()
		.map(|(asset_id, Displayed(amount))| {
			let reference =
				external_query_lookup_asset(deps.querier, registry_address.to_string(), asset_id)?;
			match &reference {
				AssetReference::Native { .. } => Err(ContractError::UnsupportedAsset),
				AssetReference::Virtual { cw20_address } => {
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

pub fn handle_bridge(
	deps: DepsMut,
	info: MessageInfo,
	interpreter: Addr,
	user_origin: UserOrigin,
	network_id: NetworkId,
	security: BridgeSecurity,
	salt: Vec<u8>,
	program: DefaultXCVMProgram,
	assets: Funds<Displayed<u128>>,
) -> Result<Response, ContractError> {
	ensure_router(deps.as_ref(), info.sender.as_ref())?;
	match security {
		// Only allow deterministic over IBC here
		BridgeSecurity::Deterministic => {
			let channel_id = IBC_NETWORK_CHANNEL.load(deps.storage, network_id)?;
			let packet = DefaultXCVMPacket {
				interpreter: interpreter.as_bytes().to_vec(),
				user_origin,
				salt,
				program,
				assets,
			};
			Ok(Response::default()
				.add_event(
					Event::new(XCVM_GATEWAY_EVENT_PREFIX)
						.add_attribute("action", "bridge")
						.add_attribute("network_id", format!("{network_id}"))
						.add_attribute("salt", format!("{}", Binary::from(packet.salt.clone())))
						.add_attribute(
							"program",
							serde_json_wasm::to_string(&packet.program)
								.map_err(|_| ContractError::FailedToSerialize)?,
						)
						.add_attribute(
							"assets",
							serde_json_wasm::to_string(&packet.assets)
								.map_err(|_| ContractError::FailedToSerialize)?,
						),
				)
				.add_message(IbcMsg::SendPacket {
					channel_id,
					data: Binary::from(packet.encode()),
					// TODO: should be a parameter or configuration
					timeout: IbcTimeout::with_block(IbcTimeoutBlock { revision: 0, height: 10000 }),
				}))
		},
		_ => Err(ContractError::UnsupportedBridgeSecurity),
	}
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	let router_address = {
		let instantiate_event = response
			.events
			.iter()
			.find(|event| event.ty == "instantiate")
			.ok_or(StdError::not_found("instantiate event not found"))?;
		deps.api.addr_validate(
			&instantiate_event
				.attributes
				.iter()
				.find(|attr| &attr.key == "_contract_address")
				.ok_or(StdError::not_found("_contract_address attribute not found"))?
				.value,
		)?
	};
	ROUTER.save(deps.storage, &router_address)?;
	Ok(Response::default())
}
