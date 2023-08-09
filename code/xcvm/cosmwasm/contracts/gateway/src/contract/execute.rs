use crate::{
	assets, auth,
	contract::INSTANTIATE_INTERPRETER_REPLY_ID,
	error::{ContractError, Result},
	events::make_event,
	msg,
	network::{self, load_this},
	state,
};

use cosmwasm_std::{
	entry_point, to_binary, wasm_execute, Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env,
	MessageInfo, Reply, Response, StdError, StdResult, SubMsg, WasmMsg,
};
use cw20::{Cw20Contract, Cw20ExecuteMsg};
use cw_xc_interpreter::contract::{
	XCVM_INTERPRETER_EVENT_DATA_ORIGIN, XCVM_INTERPRETER_EVENT_PREFIX,
};

use xc_core::{gateway::ConfigSubMsg, CallOrigin, Displayed, Funds, InterpreterOrigin};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> Result {
	use msg::ExecuteMsg;
	match msg {
		ExecuteMsg::Config(msg) => {
			let auth = auth::Admin::authorise(deps.as_ref(), &info)?;
			handle_config_msg(auth, deps, msg)
		},

		msg::ExecuteMsg::ExecuteProgram { execute_program, tip } =>
			handle_execute_program(deps, env, info, execute_program, tip),

		msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program, tip } => {
			let auth = auth::Contract::authorise(&env, &info)?;
			handle_execute_program_privilleged(auth, deps, env, call_origin, execute_program, tip)
		},

		msg::ExecuteMsg::BridgeForward(msg) => {
			let auth =
				auth::Interpreter::authorise(deps.as_ref(), &info, msg.interpreter_origin.clone())?;

			if !msg.msg.assets.0.is_empty() {
				super::ibc::ics20::handle_bridge_forward(auth, deps, info, msg)
			} else {
				super::ibc::xcvm::handle_bridge_forward_no_assets(auth, deps, info, msg)
			}
		},
		msg::ExecuteMsg::MessageHook(msg) => {
			deps.api.debug(&serde_json_wasm::to_string(&msg)?);
			let auth = auth::WasmHook::authorise(deps.storage, &env, &info, msg.from_network_id)?;
			super::ibc::ics20::ics20_message_hook(auth, msg, env, info)
		},
		msg::ExecuteMsg::Shortcut(msg) => handle_shortcut(deps, env, info, msg),
	}
}

fn handle_config_msg(auth: auth::Admin, deps: DepsMut, msg: ConfigSubMsg) -> Result {
	deps.api.debug(serde_json_wasm::to_string(&msg)?.as_str());
	match msg {
		ConfigSubMsg::ForceNetworkToNetwork(msg) =>
			network::force_network_to_network(auth, deps, msg),
		ConfigSubMsg::ForceAsset(msg) => assets::force_asset(auth, deps, msg),
		ConfigSubMsg::ForceRemoveAsset { asset_id } =>
			assets::force_remove_asset(auth, deps, asset_id),
		ConfigSubMsg::ForceNetwork(msg) => network::force_network(auth, deps, msg),
	}
}

fn handle_shortcut(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: msg::ShortcutSubMsg,
) -> Result {
	// seems it would be hard each time to form big json to smoke test transfer, so will store some
	// routes and generate program on chain for testing
	Err(ContractError::NotImplemented)
}

/// transfers assets from user
/// if case of bask assets, transfers directly,
/// in case of cw20 asset transfers using messages
fn transfer_from_user(
	deps: &DepsMut,
	self_address: Addr,
	user: Addr,
	host_funds: Vec<Coin>,
	program_funds: &Funds<Displayed<u128>>,
) -> Result<Vec<CosmosMsg>> {
	deps.api
		.debug(serde_json_wasm::to_string(&(&program_funds, &host_funds))?.as_str());
	let mut transfers = Vec::with_capacity(program_funds.0.len());
	for (asset_id, Displayed(program_amount)) in program_funds.0.iter() {
		let reference = assets::get_asset_by_id(deps.as_ref(), *asset_id)?.asset;
		match reference.local {
			msg::AssetReference::Native { denom } => {
				let Coin { amount: host_amount, .. } = host_funds
					.iter()
					.find(|c| c.denom == denom)
					.ok_or(ContractError::ProgramFundsDenomMappingToHostNotFound)?;
				if u128::from(*host_amount) != *program_amount {
					return Err(ContractError::ProgramAmountNotEqualToHostAmount)?
				}
			},
			msg::AssetReference::Cw20 { contract } =>
				transfers.push(Cw20Contract(contract).call(Cw20ExecuteMsg::TransferFrom {
					owner: user.to_string(),
					recipient: self_address.to_string(),
					amount: (*program_amount).into(),
				})?),
		}
	}
	Ok(transfers)
}

/// Handles request to execute an [`XCVMProgram`].
///
/// This is the entry point for executing a program from a user.  Handling
pub(crate) fn handle_execute_program(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	execute_program: msg::ExecuteProgramMsg,
	tip: Addr,
) -> Result {
	let self_address = env.contract.address;
	let call_origin = CallOrigin::Local { user: info.sender.clone() };
	let transfers = transfer_from_user(
		&deps,
		self_address.clone(),
		info.sender,
		info.funds,
		&execute_program.assets,
	)?;
	let msg = wasm_execute(
		self_address,
		&msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program, tip },
		Default::default(),
	)?;
	Ok(Response::default().add_messages(transfers).add_message(msg))
}

/// Handle a request to execute a [`XCVMProgram`].
/// Only the gateway is allowed to dispatch such operation.
/// The gateway must ensure that the `CallOrigin` is valid as the router does not do further
/// checking on it.
pub(crate) fn handle_execute_program_privilleged(
	_: auth::Contract,
	deps: DepsMut,
	env: Env,
	call_origin: CallOrigin,
	msg::ExecuteProgramMsg { salt, program, assets }: msg::ExecuteProgramMsg,
	tip: Addr,
) -> Result {
	let config = load_this(deps.storage)?;
	let interpreter_origin =
		InterpreterOrigin { user_origin: call_origin.user(config.network_id), salt };
	let interpreter =
		state::interpreter::INTERPRETERS.may_load(deps.storage, interpreter_origin.clone())?;
	if let Some(state::interpreter::Interpreter { address }) = interpreter {
		deps.api.debug("reusing existing interpreter and adding funds");
		let response = send_funds_to_interpreter(deps.as_ref(), address.clone(), assets)?;
		let wasm_msg = wasm_execute(
			address.clone(),
			&cw_xc_interpreter::msg::ExecuteMsg::Execute { tip, program },
			vec![],
		)?;
		Ok(response
			.add_event(
				make_event("route.execute").add_attribute("interpreter", address.into_string()),
			)
			.add_message(wasm_msg))
	} else {
		// First, add a callback to instantiate an interpreter (which we later get the result
		// and save it)
		let interpreter_code_id = match config.gateway.expect("expected setup") {
			msg::GatewayId::CosmWasm { interpreter_code_id, .. } => interpreter_code_id,
		};
		deps.api.debug("instantiating interpreter");
		let instantiate_msg: CosmosMsg = WasmMsg::Instantiate2 {			
			admin: Some(env.contract.address.clone().into_string()),
			code_id: interpreter_code_id,
			msg: to_binary(&cw_xc_interpreter::msg::InstantiateMsg {
				gateway_address: env.contract.address.clone().into_string(),
				interpreter_origin: interpreter_origin.clone(),
			})?,
			funds: vec![],
			label: interpreter_origin.to_string(),
			salt: to_binary(&interpreter_origin.to_string().as_bytes())?,
		}
		.into();

		let interpreter_instantiate_submessage =
			SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_INTERPRETER_REPLY_ID);
		// Secondly, call itself again with the same parameters, so that this functions goes
		// into `Ok` state and properly executes the interpreter
		let self_call_message: CosmosMsg = wasm_execute(
			env.contract.address,
			&xc_core::gateway::ExecuteMsg::ExecuteProgramPrivileged {
				call_origin,
				execute_program: xc_core::gateway::ExecuteProgramMsg {
					salt: interpreter_origin.salt,
					program,
					assets,
				},
				tip,
			},
			vec![],
		)?
		.into();
		Ok(Response::new()
			.add_event(make_event("route.create"))
			.add_submessage(interpreter_instantiate_submessage)
			.add_message(self_call_message))
	}
}

/// Transfer funds attached to a [`XCVMProgram`] before dispatching the program to the interpreter.
fn send_funds_to_interpreter(
	deps: Deps,
	interpreter_address: Addr,
	funds: Funds<Displayed<u128>>,
) -> Result {
	let mut response = Response::new();
	let interpreter_address = interpreter_address.into_string();
	for (asset_id, Displayed(amount)) in funds.0 {
		// We ignore zero amounts
		if amount == 0 {
			continue
		}

		let reference = assets::get_asset_by_id(deps, asset_id)?.asset;
		let msg: CosmosMsg = match reference.local {
			msg::AssetReference::Native { denom } => BankMsg::Send {
				to_address: interpreter_address.clone(),
				amount: vec![Coin::new(amount, denom)],
			}
			.into(),
			msg::AssetReference::Cw20 { contract } => {
				let contract = Cw20Contract(contract);
				contract.call(Cw20ExecuteMsg::Transfer {
					recipient: interpreter_address.clone(),
					amount: amount.into(),
				})?
			},
		};
		response = response.add_message(msg);
	}
	Ok(response)
}

pub(crate) fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;

	// Catch the default `instantiate` event which contains `_contract_address` attribute that
	// has the instantiated contract's address
	let address = &response
		.events
		.iter()
		.find(|event| event.ty == "instantiate")
		.ok_or_else(|| StdError::not_found("instantiate event not found"))?
		.attributes
		.iter()
		.find(|attr| &attr.key == "_contract_address")
		.ok_or_else(|| StdError::not_found("_contract_address attribute not found"))?
		.value;
	let interpreter_address = deps.api.addr_validate(address)?;

	// Interpreter provides `network_id, user_id` pair as an event for the router to know which
	// pair is instantiated
	let event_name = format!("wasm-{}", XCVM_INTERPRETER_EVENT_PREFIX);
	let interpreter_origin = &response
		.events
		.iter()
		.find(|event| event.ty.starts_with(&event_name))
		.ok_or_else(|| StdError::not_found("interpreter event not found"))?
		.attributes
		.iter()
		.find(|attr| &attr.key == XCVM_INTERPRETER_EVENT_DATA_ORIGIN)
		.ok_or_else(|| StdError::not_found("no data is returned from 'xcvm_interpreter'"))?
		.value;
	let interpreter_origin =
		xc_core::shared::decode_base64::<_, InterpreterOrigin>(interpreter_origin.as_str())?;

	let interpreter = state::interpreter::Interpreter { address: interpreter_address };
	state::interpreter::INTERPRETERS.save(deps.storage, interpreter_origin, &interpreter)?;

	Ok(Response::new())
}
