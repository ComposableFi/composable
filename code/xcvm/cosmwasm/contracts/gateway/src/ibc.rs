use crate::{
	contract::XCVM_GATEWAY_EVENT_PREFIX,
	error::ContractError,
	state::{
		ChannelInfo, ConfigState, CONFIG, IBC_CHANNEL_INFO, IBC_CHANNEL_NETWORK,
		IBC_NETWORK_CHANNEL,
	},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	from_binary, to_binary, Binary, CosmosMsg, DepsMut, Empty, Env, Event, Ibc3ChannelOpenResponse,
	IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
	IbcChannelOpenResponse, IbcMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg,
	IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, IbcTimeoutBlock, MessageInfo, Response,
	WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use cw_xcvm_asset_registry::{contract::external_query_lookup_asset, msg::AssetReference};
use cw_xcvm_utils::{DefaultXCVMPacket, DefaultXCVMProgram};
use serde::{Deserialize, Serialize};
use xcvm_core::{BridgeSecurity, Displayed, Funds, NetworkId, UserOrigin};
use xcvm_proto::{decode_packet, Encodable};

pub const XCVM_GATEWAY_IBC_VERSION: &str = "xcvm-gateway-v0";
pub const XCVM_GATEWAY_IBC_ORDERING: IbcOrder = IbcOrder::Unordered;

pub enum XCVMAck {
	Ko,
	Ok,
}

impl Serialize for XCVMAck {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			XCVMAck::Ko => <u8 as Serialize>::serialize(&0u8, serializer),
			XCVMAck::Ok => <u8 as Serialize>::serialize(&1u8, serializer),
		}
	}
}

impl<'de> Deserialize<'de> for XCVMAck {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let value = <u8 as Deserialize>::deserialize::<D>(deserializer)?;
		match value {
			0u8 => Ok(XCVMAck::Ko),
			1u8 => Ok(XCVMAck::Ok),
			_ => panic!("ack cannot be tampered; qed;"),
		}
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
	_deps: DepsMut,
	_env: Env,
	_msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
	todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
	deps: DepsMut,
	env: Env,
	msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
	let ack = from_binary::<XCVMAck>(&msg.acknowledgement.data)?;
	match (ack, CONFIG.load(deps.storage)?) {
		(XCVMAck::Ok, ConfigState::Initialized { registry_address, .. }) => {
			let packet: DefaultXCVMPacket =
				decode_packet(&msg.original_packet.data).map_err(ContractError::Protobuf)?;
			// We got the ACK
			let burns = burn_escrowed_assets(deps, env, registry_address.as_str(), packet.assets)?;
			Ok(IbcBasicResponse::default().add_messages(burns))
		},
		(XCVMAck::Ko, ConfigState::Initialized { registry_address, .. }) => {
			let packet: DefaultXCVMPacket =
				decode_packet(&msg.original_packet.data).map_err(ContractError::Protobuf)?;
			// On failure, return the funds
			let burns = unescrow_assets(
				&deps,
				// Safe as impossible to tamper.
				String::from_utf8_lossy(&packet.interpreter).to_string(),
				registry_address.as_str(),
				packet.assets,
			)?;
			Ok(IbcBasicResponse::default().add_messages(burns))
		},
		(_, _) => Err(ContractError::NotInitialized),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
	match CONFIG.load(deps.storage)? {
		ConfigState::NotInitialized => Err(ContractError::NotInitialized),
		ConfigState::Initialized { registry_address, .. } => {
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
		},
	}
}

fn burn_escrowed_assets(
	deps: DepsMut,
	env: Env,
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
					Ok(CosmosMsg::<Empty>::Wasm(WasmMsg::Execute {
						contract_addr: cw20_address.to_string(),
						msg: to_binary(&Cw20ExecuteMsg::BurnFrom {
							owner: env.contract.address.to_string(),
							amount: amount.into(),
						})?,
						funds: Default::default(),
					}))
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
					Ok(CosmosMsg::<Empty>::Wasm(WasmMsg::Execute {
						contract_addr: cw20_address.to_string(),
						msg: to_binary(&Cw20ExecuteMsg::Transfer {
							recipient: sender.clone(),
							amount: amount.into(),
						})?,
						funds: Default::default(),
					}))
				},
			}
		})
		.collect::<Result<Vec<_>, _>>()
}

fn escrow_assets(
	deps: &DepsMut,
	env: Env,
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
					Ok(CosmosMsg::<Empty>::Wasm(WasmMsg::Execute {
						contract_addr: cw20_address.to_string(),
						msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
							owner: sender.clone(),
							recipient: env.contract.address.to_string(),
							amount: amount.into(),
						})?,
						funds: Default::default(),
					}))
				},
			}
		})
		.collect::<Result<Vec<_>, _>>()
}

pub fn handle_bridge(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	network_id: NetworkId,
	security: BridgeSecurity,
	salt: Vec<u8>,
	program: DefaultXCVMProgram,
	assets: Funds<Displayed<u128>>,
) -> Result<Response, ContractError> {
	let config = CONFIG.load(deps.storage)?;
	match (security, config) {
		// Only allow deterministic over IBC here
		(BridgeSecurity::Deterministic, ConfigState::Initialized { registry_address, .. }) => {
			let transfers = escrow_assets(
				&deps,
				env,
				info.sender.to_string(),
				registry_address.as_str(),
				assets.clone(),
			)?;
			let channel_id = IBC_NETWORK_CHANNEL.load(deps.storage, network_id)?;
			let packet = DefaultXCVMPacket {
				interpreter: info.sender.as_bytes().to_vec().into(),
				user_origin: UserOrigin {
					network_id,
					user_id: info.sender.as_bytes().to_vec().into(),
				},
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
				.add_messages(transfers)
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
