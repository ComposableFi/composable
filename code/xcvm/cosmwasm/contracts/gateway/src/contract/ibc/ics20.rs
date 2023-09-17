//! Helps connecting identifiers into networks.
//! Allows to map asset identifiers, contracts, networks, channels, denominations from, to and on
//! each chain via contract storage, precompiles, host extensions.
//! handles PFM and IBC wasm hooks
use crate::{network, prelude::*};
use cosmwasm_std::{
	ensure_eq, wasm_execute, Binary, BlockInfo, Coin, Deps, DepsMut, Env, MessageInfo, Response,
	Storage, SubMsg,
};
use xc_core::{
	gateway::{AssetItem, ExecuteMsg, ExecuteProgramMsg, GatewayId},
	shared::{XcFunds, XcPacket, XcProgram},
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
	block: BlockInfo,
) -> Result {
	deps.api.debug(&format!(
		"xcvm::gateway:: forwarding over IBC ICS20 MEMO {}",
		&serde_json_wasm::to_string(&msg)?
	));
	ensure_eq!(msg.msg.assets.0.len(), 1, ContractError::ProgramCannotBeHandledByDestination);
	// algorithm to handle for multihop
	// 1. recurse on program until can with memo
	// 2. as soon as see no Spawn/Transfer, stop memo and do Wasm call with remaining Packet

	let (local_asset, amount) = msg.msg.assets.0.get(0).expect("proved above");

	let route: IbcIcs20Route = get_route(deps.storage, msg.to, *local_asset)?;

	let asset = msg
		.msg
		.assets
		.0
		.get(0)
		.map(|(_, amount)| (route.on_remote_asset, *amount))
		.expect("not empty");

	let packet = XcPacket {
		interpreter: String::from(info.sender).into_bytes(),
		user_origin: msg.interpreter_origin.user_origin.clone(),
		salt: msg.msg.salt,
		program: msg.msg.program,
		assets: vec![asset].into(),
	};

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

	let (ret_msg, tracker) =
		to_cw_message(deps.as_ref(), deps.api, coin.clone(), route, packet, block)?;
	state::tracking::track(
		deps.storage,
		msg.interpreter_origin,
		tracker,
		state::tracking::TrackedState { assets: vec![coin] },
	)?;
	Ok(Response::default().add_event(event).add_message(ret_msg))
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
	let other = network::load_other(storage, to)?;
	let asset: AssetItem = state::assets::ASSETS
		.load(storage, this_asset_id)
		.map_err(|_| ContractError::AssetNotFoundById(this_asset_id))?;
	let to_asset: AssetId = state::assets::NETWORK_ASSET
		.load(storage, (this_asset_id, to))
		.map_err(|_| ContractError::AssetCannotBeTransferredToNetwork(this_asset_id, to))?;
	let gateway_to_send_to = other.network.gateway.ok_or(ContractError::UnsupportedNetwork)?;
	let gateway_to_send_to = match gateway_to_send_to {
		GatewayId::CosmWasm { contract, .. } => contract,
	};

	let sender_gateway = match this.gateway.expect("we execute here") {
		GatewayId::CosmWasm { contract, .. } => contract,
	};

	let channel = other.connection.ics_20.ok_or(ContractError::ICS20NotFound)?.source;

	Ok(IbcIcs20Route {
		from_network: this.network_id,
		local_native_denom: asset.local.denom(),
		channel_to_send_over: channel,
		gateway_to_send_to,
		sender_gateway,
		counterparty_timeout: other.connection.counterparty_timeout,
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
	deps: Deps,
	msg: XcMessageData,
	env: Env,
	info: MessageInfo,
) -> Result<Response, ContractError> {
	let packet: XcPacket = msg.packet;
	ensure_anonymous(&packet.program)?;
	deps.api.debug(&format!(
		"xcvm::gateway::ibc::ics20:: received assets {:?}, packet assets {:?}",
		&info.funds, &packet.assets
	));

	let assets: Result<XcFunds, ContractError> = info
		.funds
		.into_iter()
		.map(|coin| {
			let asset = crate::assets::get_local_asset_by_reference(
				deps,
				AssetReference::Native { denom: coin.denom },
			)?;
			Ok((asset.asset_id, coin.amount.into()))
		})
		.collect();
	let call_origin = CallOrigin::Remote { user_origin: packet.user_origin };
	let execute_program =
		ExecuteProgramMsg { salt: packet.salt, program: packet.program, assets: assets?.into() };
	let msg =
		ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program, tip: info.sender };
	let msg = wasm_execute(env.contract.address, &msg, Default::default())?;
	Ok(Response::new().add_submessage(SubMsg::reply_always(msg, EXEC_PROGRAM_REPLY_ID)))
}

fn ensure_anonymous(program: &XcProgram) -> Result<()> {
	use xc_core::Instruction::*;
	for ix in &program.instructions {
		match ix {
			Transfer { .. } => {},
			Exchange { .. } => {},
			Spawn { program, .. } => ensure_anonymous(program)?,
			_ => Err(ContractError::AnonymousCallsCanDoOnlyLimitedSetOfActions)?,
		}
	}
	Ok(())
}
