//! Helps connecting identifiers into networks.
//! Allows to map asset identifiers, contracts, networks, channels, denominations from, to and on
//! each chain via contract storage, precompiles, host extensions.
//! handles PFM and IBC wasm hooks
use cosmwasm_std::{
	ensure_eq, to_binary, wasm_execute, Binary, Coin, DepsMut, Env, MessageInfo, Response, Storage,
	SubMsg,
};
use xc_core::{
	gateway::{Asset, ExecuteMsg, ExecuteProgramMsg, GatewayId},
	ibc::{ics20::hook::WasmMemo, to_cw_message, IbcRoute, Ics20MessageHook},
	proto::{decode_packet, Encodable},
	shared::{DefaultXCVMProgram, XcPacket},
	AssetId, CallOrigin, Funds,
};

use crate::{
	auth,
	contract::EXEC_PROGRAM_REPLY_ID,
	error::{ContractError, Result},
	events::make_event,
	state,
	state::{NetworkItem, OtherNetworkItem},
};

pub(crate) fn handle_bridge_forward(
	_: auth::Interpreter,
	deps: DepsMut,
	info: MessageInfo,
	msg: xc_core::gateway::BridgeMsg,
) -> Result {
	let packet: xc_core::Packet<
		xc_core::Program<
			std::collections::VecDeque<
				xc_core::Instruction<Vec<u8>, cosmwasm_std::CanonicalAddr, Funds>,
			>,
		>,
	> = XcPacket {
		interpreter: String::from(info.sender).into_bytes(),
		user_origin: msg.interpreter_origin.user_origin,
		salt: msg.msg.salt,
		program: msg.msg.program,
		assets: msg.msg.assets,
	};

	ensure_eq!(packet.assets.0.len(), 0, ContractError::ProgramCannotBeHandledByDestination);
	let (local_asset, amount) = packet.assets.0.get(0).expect("proved above");
	let route = get_route(deps.storage, msg.network_id, *local_asset)?;

	let mut event = make_event("bridge")
		.add_attribute("to_network_id", msg.network_id.to_string())
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

	let coin = Coin::new(amount.0.into(), route.local_native_denom.clone());

	let memo = serde_json_wasm::to_string(&WasmMemo {
		contract: route.gateway_to_send_to.clone(),
		msg: to_binary(&Ics20MessageHook {
			from_network_id: route.from_network,
			data: Binary::from(packet.encode()),
		})?
		.to_vec(),
		ibc_callback: None,
	})?;

	let msg = to_cw_message(memo, coin, route)?;

	Ok(Response::default().add_event(event).add_message(msg))
}

/// given target network and this network assets identifier,
/// find channels, target denomination and gateway on other network
/// so can form and sent ICS20 PFM Wasm terminated packet
pub fn get_route(
	storage: &mut dyn Storage,
	to: xc_core::NetworkId,
	asset_id: AssetId,
) -> Result<IbcRoute, ContractError> {
	let this = state::Config::load(storage)?;
	let other: NetworkItem = state::NETWORK.load(storage, to)?;
	let this_to_other: OtherNetworkItem =
		state::NETWORK_TO_NETWORK.load(storage, (this.network_id, to))?;
	let asset: Asset = state::ASSETS.load(storage, asset_id)?;
	let to_asset: AssetId = state::NETWORK_ASSET.load(storage, (asset_id, to))?;
	let gateway_to_send_to = other.gateway_to_send_to.ok_or(ContractError::UnsupportedNetwork)?;
	let gateway_to_send_to = match gateway_to_send_to {
		GatewayId::CosmWasm(addr) => addr.to_string(),
	};
	Ok(IbcRoute {
		from_network: this.network_id,
		local_native_denom: asset.local.denom(),
		channel_to_send_to: this_to_other.ics_20_channel,
		gateway_to_send_to,
		counterparty_timeout: this_to_other.counterparty_timeout,
		ibc_ics_20_sender: this.ibc_ics_20_sender.ok_or(ContractError::UnsupportedNetwork)?,
		on_remote_asset: to_asset,
	})
}

pub(crate) fn ics20_message_hook(
	_: auth::WasmHook,
	msg: Ics20MessageHook,
	env: Env,
	info: MessageInfo,
) -> Result<Response, ContractError> {
	let packet: XcPacket = decode_packet(&msg.data).map_err(ContractError::Protobuf)?;

	ensure_anonymous(&packet.program)?;
	let call_origin = CallOrigin::Remote { user_origin: packet.user_origin };
	let execute_program =
		ExecuteProgramMsg { salt: packet.salt, program: packet.program, assets: packet.assets };
	let msg =
		ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program, tip: info.sender };
	let msg = wasm_execute(env.contract.address, &msg, Default::default())?;
	Ok(Response::new().add_submessage(SubMsg::reply_always(msg, EXEC_PROGRAM_REPLY_ID)))
}

fn ensure_anonymous(program: &DefaultXCVMProgram) -> Result<()> {
	for ix in &program.instructions {
		match ix {
			xc_core::Instruction::Transfer { .. } => {},
			xc_core::Instruction::Spawn { program, .. } => ensure_anonymous(&program)?,
			_ => Err(ContractError::NotAuthorized)?,
		}
	}
	Ok(())
}
