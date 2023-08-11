//! Helps connecting identifiers into networks.
//! Allows to map asset identifiers, contracts, networks, channels, denominations from, to and on
//! each chain via contract storage, precompiles, host extensions.
//! handles PFM and IBC wasm hooks
use crate::prelude::*;
use cosmwasm_std::{
	ensure_eq, wasm_execute, Binary, Coin, DepsMut, Env, MessageInfo, Response, Storage, SubMsg,
};
use xc_core::{
	gateway::{AssetItem, ExecuteMsg, ExecuteProgramMsg, GatewayId, OtherNetworkItem},
	shared::{XcPacket, XcProgram},
	transport::ibc::{to_cw_message, IbcIcs20Route, XcMessageData},
	AssetId, CallOrigin,
};

use crate::{
	auth,
	contract::EXEC_PROGRAM_REPLY_ID,
	error::{ContractError, Result},
	events::make_event,
	network::load_this,
	state,
};

pub(crate) fn handle_bridge_forward(
	_: auth::Interpreter,
	deps: DepsMut,
	info: MessageInfo,
	msg: xc_core::gateway::BridgeForwardMsg,
) -> Result {
	deps.api.debug(&format!(
		"xcvm::gateway:: forwarding over IBC ICS20 MEMO {}",
		&serde_json_wasm::to_string(&msg)?
	));
	ensure_eq!(msg.msg.assets.0.len(), 1, ContractError::ProgramCannotBeHandledByDestination);
	// algorithm to handle for multihop
	// 1. recurse on program until can with memo
	// 2. as soon as see no Spawn/Transfer, stop memo and do Wasm call with remaining Packet

	let packet = XcPacket {
		interpreter: String::from(info.sender).into_bytes(),
		user_origin: msg.interpreter_origin.user_origin,
		salt: msg.msg.salt,
		program: msg.msg.program,
		assets: msg.msg.assets,
	};

	let (local_asset, amount) = packet.assets.0.get(0).expect("proved above");

	let route = get_route(deps.storage, msg.to, *local_asset)?;
	deps.api.debug(&format!(
		"xcvm::gateway::ibc::ics20 route {}",
		&serde_json_wasm::to_string(&route)?
	));
	let mut event = make_event("bridge")
		.add_attribute("to_network_id", msg.to.to_string())
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

	let coin = Coin::new(amount.0, route.local_native_denom.clone());

	let msg = to_cw_message(deps.api, coin, route, packet)?;
	deps.api.debug(&format!(
		"xcvm::gateway::ibc::ics20:: payload {}",
		&serde_json_wasm::to_string(&msg)?
	));

	Ok(Response::default().add_event(event).add_message(msg))
}

/// given target network and this network assets identifier,
/// find channels, target denomination and gateway on other network
/// so can form and sent ICS20 PFM Wasm terminated packet
pub fn get_route(
	storage: &dyn Storage,
	to: xc_core::NetworkId,
	this_asset_id: AssetId,
) -> Result<IbcIcs20Route, ContractError> {
	let this = load_this(storage)?;
	let other: NetworkItem = state::NETWORK
		.load(storage, to)
		.map_err(|_| ContractError::UnknownTargetNetwork)?;
	let this_to_other: OtherNetworkItem = state::NETWORK_TO_NETWORK
		.load(storage, (this.network_id, to))
		.map_err(|_| ContractError::NoConnectionInformationFromThisToOtherNetwork)?;
	let asset: AssetItem = state::assets::ASSETS
		.load(storage, this_asset_id)
		.map_err(|_| ContractError::AssetNotFoundById(this_asset_id))?;
	let to_asset: AssetId = state::assets::NETWORK_ASSET
		.load(storage, (this_asset_id, to))
		.map_err(|_| ContractError::AssetCannotBeTransferredToNetwork(this_asset_id, to))?;
	let gateway_to_send_to = other.gateway.ok_or(ContractError::UnsupportedNetwork)?;
	let gateway_to_send_to = match gateway_to_send_to {
		GatewayId::CosmWasm { contract, .. } => contract,
	};

	let sender_gateway = match this.gateway.expect("we execute here") {
		GatewayId::CosmWasm { contract, .. } => contract,
	};

	let channel = this_to_other.ics_20.ok_or(ContractError::ICS20NotFound)?.source;

	Ok(IbcIcs20Route {
		from_network: this.network_id,
		local_native_denom: asset.local.denom(),
		channel_to_send_over: channel,
		gateway_to_send_to,
		sender_gateway,
		counterparty_timeout: this_to_other.counterparty_timeout,
		ibc_ics_20_sender: this
			.ibc
			.ok_or(ContractError::ICS20NotFound)?
			.channels
			.ok_or(ContractError::ICS20NotFound)?
			.ics20
			.ok_or(ContractError::ICS20NotFound)?
			.sender,
		on_remote_asset: to_asset,
	})
}

pub(crate) fn ics20_message_hook(
	_: auth::WasmHook,
	msg: XcMessageData,
	env: Env,
	info: MessageInfo,
) -> Result<Response, ContractError> {
	let packet: XcPacket = msg.packet;
	ensure_anonymous(&packet.program)?;
	let call_origin = CallOrigin::Remote { user_origin: packet.user_origin };
	let execute_program =
		ExecuteProgramMsg { salt: packet.salt, program: packet.program, assets: packet.assets };
	let msg =
		ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program, tip: info.sender };
	let msg = wasm_execute(env.contract.address, &msg, Default::default())?;
	Ok(Response::new().add_submessage(SubMsg::reply_always(msg, EXEC_PROGRAM_REPLY_ID)))
}

fn ensure_anonymous(program: &XcProgram) -> Result<()> {
	for ix in &program.instructions {
		match ix {
			xc_core::Instruction::Transfer { .. } => {},
			xc_core::Instruction::Spawn { program, .. } => ensure_anonymous(program)?,
			_ => Err(ContractError::NotAuthorized)?,
		}
	}
	Ok(())
}
