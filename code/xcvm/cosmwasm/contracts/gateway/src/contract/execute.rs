
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
use xc_proto::{decode_packet, Encodable};

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: msg::ExecuteMsg,
) -> ContractResult<Response> {
	match msg {
		msg::ExecuteMsg::IbcSetNetworkChannel { network_id, channel_id } => {
			let auth = auth::Admin::authorise(deps.as_ref(), &info)?;
			handle_ibc_set_network_channel(auth, deps, network_id, channel_id)
		},

		msg::ExecuteMsg::ExecuteProgram { execute_program } =>
			exec::handle_execute_program(deps, env, info, execute_program),

		msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program } => {
			let auth = auth::Contract::authorise(&env, &info)?;
			exec::handle_execute_program_privilleged(auth, deps, env, call_origin, execute_program)
		},

		msg::ExecuteMsg::Bridge(msg) => {
			let auth = auth::Interpreter::authorise(
				deps.as_ref(),
				&info,
				msg.interpreter_origin.clone(),
			)?;
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
	}
}
