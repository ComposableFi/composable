extern crate alloc;

use crate::{
	prelude::*,
	assets, auth,
	error::{ContractError, ContractResult},
	events::make_event,
	exec, msg, state,
	topology::get_route,
};

use cosmwasm_std::{
	to_binary, wasm_execute, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, Ibc3ChannelOpenResponse,
	IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
	IbcChannelOpenResponse, IbcMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg,
	IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, IbcTimeoutBlock, MessageInfo, Reply,
	Response, SubMsg, SubMsgResult, WasmMsg, Addr,
};
use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;
use cw_utils::ensure_from_older_version;
use cw_xc_common::shared::DefaultXCVMPacket;
use xc_core::{
	ibc::{Ics20MessageHook, WasmMemo},
	proto::{decode_packet, Encodable},
	BridgeProtocol, CallOrigin, Displayed, Funds, Picasso, XCVMAck,
};

use super::EXEC_PROGRAM_REPLY_ID;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: msg::ExecuteMsg,
) -> ContractResult<Response> {
	match msg {
		msg::ExecuteMsg::IbcSetNetworkChannel { from, to, channel_id, gateway } => {
			let auth = auth::Admin::authorise(deps.as_ref(), &info)?;
			handle_ibc_set_network_channel(auth, deps, to, channel_id)
		},

		msg::ExecuteMsg::ExecuteProgram { execute_program } =>
			exec::handle_execute_program(deps, env, info, execute_program),

		msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program } => {
			let auth = auth::Contract::authorise(&env, &info)?;
			exec::handle_execute_program_privilleged(auth, deps, env, call_origin, execute_program)
		},

		msg::ExecuteMsg::Bridge(msg) => {
			let auth =
				auth::Interpreter::authorise(deps.as_ref(), &info, msg.interpreter_origin.clone())?;
			handle_bridge_forward(auth, deps, info, msg)
		},

		msg::ExecuteMsg::RegisterAsset { asset_id, reference } => {
			let auth = auth::Admin::authorise(deps.as_ref(), &info)?;
			assets::handle_register_asset(auth, deps, asset_id, reference)
		},

		msg::ExecuteMsg::UnregisterAsset { asset_id } => {
			let auth = auth::Admin::authorise(deps.as_ref(), &info)?;
			assets::handle_unregister_asset(auth, deps, asset_id)
		},
    	msg::ExecuteMsg::Wasm(msg) => {			
			let auth = auth::Wasm::authorise(deps, &env, &info, &msg.network_id)?;	
			remote_wasm_execute(auth, msg, env)
		},
	}
}

pub fn remote_wasm_execute(_ : auth::Wasm, msg: Ics20MessageHook, env: Env) -> Result<Response, ContractError> {
    let packet : DefaultXCVMPacket = decode_packet(&msg.data).map_err(ContractError::Protobuf)?;
    let call_origin = CallOrigin::Remote {
				    protocol: BridgeProtocol::IBC,
				    relayer: Addr::unchecked("no access"),
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
}

/// Handle a request gateway message.
/// The call must originate from an interpreter.
fn handle_bridge_forward(
	_: auth::Interpreter,
	deps: DepsMut,
	info: MessageInfo,
	msg: cw_xc_common::gateway::BridgeMsg,
) -> ContractResult<Response> {
	let channel_id = state::IBC_NETWORK_CHANNEL
		.load(deps.storage, msg.network_id)
		.map_err(|_| ContractError::UnknownChannel)?;
	let packet: xc_core::Packet<xc_core::Program<std::collections::VecDeque<xc_core::Instruction<Vec<u8>, cosmwasm_std::CanonicalAddr, Funds>>>> = DefaultXCVMPacket {
		interpreter: String::from(info.sender).into_bytes(),
		user_origin: msg.interpreter_origin.user_origin,
		salt: msg.execute_program.salt,
		program: msg.execute_program.program,
		assets: msg.execute_program.assets,
	};
	let mut event = make_event("bridge")
		.add_attribute("network_id", msg.network_id.to_string())
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
		// TODO(mina86): We're unnecessarily clone packet.salt here.  What we
		// want here is ‘to_base64(&packet.salt)’.
		let salt_attr = Binary::from(packet.salt.as_slice()).to_string();
		event = event.add_attribute("salt", salt_attr);
	}

	let (asset, amount) = packet.assets.0.get(0).expect("verified at outer boundaries");
	let (denom, channel_id, gateway, timeout) =
		get_route(crate::topology::this(), msg.network_id, asset)?;
	let target_prefix = "centauri";
	let coin = Coin::new(amount.into(), denom);

	let transfer = xc_core::ibc::IbcMsg::Transfer {
		channel_id: channel_id.clone(),
		to_address: gateway.to_string(),
		amount: coin.clone(),
		timeout,
		memo: Some(serde_json_wasm::to_string(&WasmMemo {
			contract: Addr,
			msg: Ics20MessageHook {
				network_id: crate::topology::this(),
				data: Binary::from(packet.encode()),
			},
			ibc_callback: None,
		})),
	};

	const IBC_PRECOMPILE: &str = "5EYCAe5g89aboD4c8naVbgG6izsMBCgtoCB9TUHiJiH2yVow";
	let msg = WasmMsg::Execute {
		contract_addr: IBC_PRECOMPILE.into(),
		msg: Binary::from(packet.encode()),
		funds: <_>::default(),
	};

	Ok(Response::default().add_event(event).add_message(msg))
}

fn handle_ibc_set_network_channel(
	_: auth::Admin,
	deps: DepsMut,
	network_id: xc_core::NetworkId,
	channel_id: state::ChannelId,
) -> ContractResult<Response> {
	state::IBC_CHANNEL_INFO
		.load(deps.storage, channel_id.clone())
		.map_err(|_| ContractError::UnknownChannel)?;
	state::IBC_NETWORK_CHANNEL.save(deps.storage, network_id, &channel_id)?;
	Ok(Response::default().add_event(
		make_event("set_network_channel")
			.add_attribute("network_id", network_id.to_string())
			.add_attribute("channel_id", channel_id),
	))
}
