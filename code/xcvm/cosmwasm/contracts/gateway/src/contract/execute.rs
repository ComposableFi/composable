use crate::{
	assets, auth,
	batch::BatchResponse,
	error::{ContractError, Result},
	events::make_event,
	exchange, interpreter, msg,
	network::{self, load_this},
	state,
};

use cosmwasm_std::{
	entry_point, wasm_execute, Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
	Response,
};
use cw20::{Cw20Contract, Cw20ExecuteMsg};

use xc_core::{gateway::ConfigSubMsg, CallOrigin, Funds, InterpreterOrigin};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> Result {
	use msg::ExecuteMsg;
	let sender = &info.sender;
	let canonical_sender = deps.api.addr_canonicalize(sender.as_str())?;
	deps.api.debug(&format!(
		"cvm::gateway::execute sender on chain {}, sender cross chain {}",
		sender,
		&serde_json_wasm::to_string(&canonical_sender)?
	));
	match msg {
		ExecuteMsg::Config(msg) => {
			let auth = auth::Admin::authorise(deps.as_ref(), &info)?;
			handle_config_msg(auth, deps, msg, &env).map(Into::into)
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
				super::ibc::ics20::handle_bridge_forward(auth, deps, info, msg, env.block)
			} else {
				super::ibc::xcvm::handle_bridge_forward_no_assets(auth, deps, info, msg, env.block)
			}
		},
		msg::ExecuteMsg::MessageHook(msg) => {
			deps.api.debug(&format!("cvm::gateway::execute::message_hook {:?}", msg));

			let auth = auth::WasmHook::authorise(deps.as_ref(), &env, &info, msg.from_network_id)?;

			super::ibc::ics20::ics20_message_hook(auth, deps.as_ref(), msg, env, info)
		},
		msg::ExecuteMsg::Shortcut(msg) => handle_shortcut(deps, env, info, msg),
	}
}

fn handle_config_msg(
	auth: auth::Admin,
	mut deps: DepsMut,
	msg: ConfigSubMsg,
	env: &Env,
) -> Result<BatchResponse> {
	deps.api.debug(serde_json_wasm::to_string(&msg)?.as_str());
	match msg {
		ConfigSubMsg::ForceNetworkToNetwork(msg) =>
			network::force_network_to_network(auth, deps, msg),
		ConfigSubMsg::ForceAsset(msg) => assets::force_asset(auth, deps, msg),
		ConfigSubMsg::ForceExchange(msg) => exchange::force_exchange(auth, deps, msg),
		ConfigSubMsg::ForceRemoveAsset { asset_id } =>
			assets::force_remove_asset(auth, deps, asset_id),
		ConfigSubMsg::ForceAssetToNetworkMap { this_asset, other_network, other_asset } =>
			assets::force_asset_to_network_map(auth, deps, this_asset, other_network, other_asset),
		ConfigSubMsg::ForceNetwork(msg) => network::force_network(auth, deps, msg),
		ConfigSubMsg::ForceInstantiate { user_origin, salt } => interpreter::force_instantiate(
			auth,
			env.contract.address.clone(),
			deps,
			user_origin,
			salt,
		),
		ConfigSubMsg::Force(msgs) => {
			let mut aggregated = BatchResponse::new();
			for msg in msgs {
				let response = handle_config_msg(auth, deps.branch(), msg, env)?;
				aggregated.merge(response);
			}
			Ok(aggregated)
		},
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
	program_funds: &Funds<xc_core::shared::Displayed<u128>>,
) -> Result<Vec<CosmosMsg>> {
	deps.api
		.debug(serde_json_wasm::to_string(&(&program_funds, &host_funds))?.as_str());
	let mut transfers = Vec::with_capacity(program_funds.0.len());
	for (asset_id, program_amount) in program_funds.0.iter() {
		match assets::get_asset_by_id(deps.as_ref(), *asset_id)?.local {
			msg::AssetReference::Native { denom } => {
				let Coin { amount: host_amount, .. } = host_funds
					.iter()
					.find(|c| c.denom == denom)
					.ok_or(ContractError::ProgramFundsDenomMappingToHostNotFound)?;
				if *program_amount != u128::from(*host_amount) {
					return Err(ContractError::ProgramAmountNotEqualToHostAmount)?
				}
			},
			msg::AssetReference::Cw20 { contract } =>
				transfers.push(Cw20Contract(contract).call(Cw20ExecuteMsg::TransferFrom {
					owner: user.to_string(),
					recipient: self_address.to_string(),
					amount: (*program_amount).into(),
				})?),
			msg::AssetReference::Erc20 { .. } => Err(ContractError::RuntimeUnsupportedOnNetwork)?,
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
	tip: String,
) -> Result {
	let this = msg::Gateway::new(env.contract.address);
	let tip = deps.api.addr_validate(&tip)?;
	let call_origin = CallOrigin::Local { user: info.sender.clone() };
	let transfers = transfer_from_user(
		&deps,
		this.address(),
		info.sender,
		info.funds,
		&execute_program.assets,
	)?;
	let msg = msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program, tip };
	let msg = this.execute(msg)?;
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
		InterpreterOrigin { user_origin: call_origin.user(config.network_id), salt: salt.clone() };
	let interpreter =
		state::interpreter::get_by_origin(deps.as_ref(), interpreter_origin.clone()).ok();
	if let Some(state::interpreter::Interpreter { address, .. }) = interpreter {
		deps.api.debug("cvm:: reusing existing interpreter and adding funds");
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
			msg::GatewayId::Evm { .. } =>
				Err(ContractError::BadlyConfiguredRouteBecauseThisChainCanSendOnlyFromCosmwasm)?,
		};
		deps.api.debug("instantiating interpreter");
		let this = msg::Gateway::new(env.contract.address);

		let interpreter_instantiate_submessage = crate::interpreter::instantiate(
			deps.as_ref(),
			this.address(),
			interpreter_code_id,
			&interpreter_origin,
			salt,
		)?;

		// Secondly, call itself again with the same parameters, so that this functions goes
		// into `Ok` state and properly executes the interpreter
		let execute_program: msg::ExecuteProgramMsg =
			xc_core::gateway::ExecuteProgramMsg { salt: interpreter_origin.salt, program, assets };
		let msg = msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program, tip };
		let self_call_message = this.execute(msg)?;

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
	funds: Funds<xc_core::shared::Displayed<u128>>,
) -> Result {
	let mut response = Response::new();
	let interpreter_address = interpreter_address.into_string();
	for (asset_id, amount) in funds.0 {
		// Ignore zero amounts
		if amount == 0 {
			continue
		}
		deps.api.debug("cvm::gateway:: sending funds");

		let msg = match assets::get_asset_by_id(deps, asset_id)?.local {
			msg::AssetReference::Native { denom } => BankMsg::Send {
				to_address: interpreter_address.clone(),
				amount: vec![Coin::new(amount.into(), denom)],
			}
			.into(),
			msg::AssetReference::Cw20 { contract } => {
				let contract = Cw20Contract(contract);
				contract.call(Cw20ExecuteMsg::Transfer {
					recipient: interpreter_address.clone(),
					amount: amount.into(),
				})?
			},
			msg::AssetReference::Erc20 { .. } => Err(ContractError::RuntimeUnsupportedOnNetwork)?,
		};
		response = response.add_message(msg);
	}
	Ok(response)
}
