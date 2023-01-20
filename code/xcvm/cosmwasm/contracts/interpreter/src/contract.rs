extern crate alloc;

use crate::{
	authenticate::{ensure_owner, Authenticated},
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{Config, CONFIG, IP_REGISTER, OWNERS, RELAYER_REGISTER, RESULT_REGISTER},
};
use alloc::{borrow::Cow, collections::VecDeque};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, wasm_execute, Addr, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Deps, DepsMut,
	Env, Event, MessageInfo, QueryRequest, Reply, Response, StdError, StdResult, SubMsg, WasmQuery,
};
use cw2::set_contract_version;
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg, TokenInfoResponse};
use cw_utils::ensure_from_older_version;
use cw_xcvm_asset_registry::{contract::external_query_lookup_asset, msg::AssetReference};
use cw_xcvm_common::shared::{encode_base64, BridgeMsg};
use cw_xcvm_utils::{DefaultXCVMInstruction, DefaultXCVMProgram};
use num::Zero;
use xcvm_core::{
	apply_bindings, cosmwasm::*, Balance, BindingValue, BridgeSecurity, Destination, Displayed,
	Funds, NetworkId, Register,
};

type XCVMInstruction = DefaultXCVMInstruction;
type XCVMProgram = DefaultXCVMProgram;

const CONTRACT_NAME: &str = "composable:xcvm-interpreter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CALL_ID: u64 = 1;
const SELF_CALL_ID: u64 = 2;
pub const XCVM_INTERPRETER_EVENT_PREFIX: &str = "xcvm.interpreter";
pub const XCVM_INTERPRETER_EVENT_DATA_ORIGIN: &str = "data";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

	let registry_address = deps.api.addr_validate(&msg.registry_address)?;
	let gateway_address = deps.api.addr_validate(&msg.gateway_address)?;
	let router_address = deps.api.addr_validate(&msg.router_address)?;
	let config = Config {
		registry_address,
		router_address,
		gateway_address,
		interpreter_origin: msg.interpreter_origin,
	};
	CONFIG.save(deps.storage, &config)?;
	// Save the caller as owner, in most cases, it is the `XCVM router`
	OWNERS.save(deps.storage, info.sender, &())?;

	Ok(Response::new().add_event(Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute(
		XCVM_INTERPRETER_EVENT_DATA_ORIGIN,
		encode_base64(&config.interpreter_origin)?.as_str(),
	)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	// Only owners can execute entrypoints of the interpreter
	let token = ensure_owner(deps.as_ref(), &env.contract.address, info.sender.clone())?;
	match msg {
		ExecuteMsg::Execute { relayer, program } =>
			initiate_execution(token, deps, env, relayer, program),

		// ExecuteStep should be called by interpreter itself
		ExecuteMsg::ExecuteStep { relayer, program } =>
			if env.contract.address != info.sender {
				Err(ContractError::NotSelf)
			} else {
				// Encore self ownership in this token
				handle_execute_step(token, deps, env, relayer, program)
			},

		ExecuteMsg::AddOwners { owners } => add_owners(token, deps, owners),

		ExecuteMsg::RemoveOwners { owners } => Ok(remove_owners(token, deps, owners)),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

	// Already only callable by the admin of the contract, so no need to `ensure_owner`
	let token = ensure_owner(deps.as_ref(), &env.contract.address, env.contract.address.clone())?;
	let _ = add_owners(token, deps, msg.owners)?;
	Ok(Response::default())
}

/// Initiate an execution by adding a `ExecuteStep` callback. This is used to be able to prepare an
/// execution by resetting the necessary registers as well as being able to catch any failures and
/// store it in the `RESULT_REGISTER`.
/// The [`RELAYER_REGISTER`] is updated to hold the current relayer address. Note that the
/// [`RELAYER_REGISTER`] always contains a value, and the value is equal to the last relayer that
/// executed a program if any.
fn initiate_execution(
	_: Authenticated,
	deps: DepsMut,
	env: Env,
	relayer: Addr,
	program: DefaultXCVMProgram,
) -> Result<Response, ContractError> {
	// Reset instruction pointer to zero.
	IP_REGISTER.save(deps.storage, &0)?;

	// Set the new relayer, note that the relayer that is in the register is always the last relayer
	// that executed a program.
	RELAYER_REGISTER.save(deps.storage, &relayer)?;

	Ok(Response::default().add_submessage(SubMsg::reply_on_error(
		wasm_execute(
			env.contract.address,
			&ExecuteMsg::ExecuteStep { relayer, program },
			Default::default(),
		)?,
		SELF_CALL_ID,
	)))
}

/// Add owners who can execute entrypoints other than `ExecuteStep`
fn add_owners(
	_: Authenticated,
	deps: DepsMut,
	owners: Vec<Addr>,
) -> Result<Response, ContractError> {
	let mut event =
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "owners.added");
	for owner in owners {
		event = event.add_attribute("owner", format!("{}", owner));
		OWNERS.save(deps.storage, owner, &())?;
	}
	Ok(Response::default().add_event(event))
}

/// Remove a set of owners from the current owners list.
/// Beware that emptying the set of owners result in a tombstoned interpreter.
fn remove_owners(_: Authenticated, deps: DepsMut, owners: Vec<Addr>) -> Response {
	let mut event =
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "owners.removed");
	for owner in owners {
		event = event.add_attribute("owner", format!("{}", owner));
		OWNERS.remove(deps.storage, owner);
	}
	Response::default().add_event(event)
}

/// Execute a [`XCVMProgram`].
/// The function will execute the program instructions one by one.
/// If the program contains a [`XCVMInstruction::Call`], the execution is suspended and resumed
/// after having executed the call.
/// The [`IP_REGISTER`] is updated accordingly.
/// A final `executed` event is yield whenever a program come to completion (all it's instructions
/// has been executed).
pub fn handle_execute_step(
	_: Authenticated,
	mut deps: DepsMut,
	env: Env,
	relayer: Addr,
	program: XCVMProgram,
) -> Result<Response, ContractError> {
	let mut response = Response::new();
	let instruction_len = program.instructions.len();
	let mut instruction_iter = program.instructions.into_iter().enumerate();
	let mut ip = IP_REGISTER.load(deps.storage)?;
	while let Some((index, instruction)) = instruction_iter.next() {
		response = match instruction {
			XCVMInstruction::Call { bindings, encoded } => {
				if index >= instruction_len - 1 {
					// If the call is the final instruction, do not yield execution
					interpret_call(deps.as_ref(), &env, bindings, encoded, ip as usize, response)?
				} else {
					// If the call is not the final instruction:
					// 1. interpret the call: this will add the call to the response's
					//    submessages.
					// 2. yield the execution by adding a call to the interpreter with the
					//    rest of the instructions as XCVM program. This will make sure that
					//    previous call instruction will run first, then the rest of the program
					//    will run.
					let response =
						interpret_call(deps.as_ref(), &env, bindings, encoded, index, response)?;
					let instructions: VecDeque<XCVMInstruction> =
						instruction_iter.map(|(_, instr)| instr).collect();
					let program = XCVMProgram { tag: program.tag, instructions };
					IP_REGISTER.save(deps.storage, &ip)?;
					return Ok(response.add_message(wasm_execute(
						env.contract.address,
						&ExecuteMsg::ExecuteStep { relayer: relayer.clone(), program },
						vec![],
					)?))
				}
			},
			XCVMInstruction::Spawn { network, bridge_security, salt, assets, program } =>
				interpret_spawn(
					&deps,
					&env,
					network,
					bridge_security,
					salt,
					assets,
					program,
					response,
				)?,
			XCVMInstruction::Transfer { to, assets } =>
				interpret_transfer(&mut deps, &env, relayer.clone(), to, assets, response)?,
			instr => return Err(ContractError::InstructionNotSupported(format!("{:?}", instr))),
		};
		ip += 1;
	}

	IP_REGISTER.save(deps.storage, &ip)?;

	let mut event = Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "executed");
	if program.tag.len() >= 3 {
		event = event.add_attribute(
			"tag",
			core::str::from_utf8(&program.tag).map_err(|_| ContractError::InvalidProgramTag)?,
		);
	}

	Ok(response.add_event(event))
}

/// Interpret the `Call` instruction
/// * `encoded`: JSON-encoded `LateCall` as bytes
///
/// Late-bindings are actually done in this function. If our XCVM SDK is not used,
/// make sure that indices in the `LateCall` is sorted in an ascending order.
pub fn interpret_call(
	deps: Deps,
	env: &Env,
	bindings: Vec<(u32, BindingValue)>,
	payload: Vec<u8>,
	_ip: usize,
	response: Response,
) -> Result<Response, ContractError> {
	// We don't know the type of the payload, so we use `serde_json::Value`
	let flat_cosmos_msg: FlatCosmosMsg<serde_json::Value> = if !bindings.is_empty() {
		let Config { registry_address, .. } = CONFIG.load(deps.storage)?;
		// Len here is the maximum possible length
		let mut formatted_call =
			vec![0; env.contract.address.as_bytes().len() * bindings.len() + payload.len()];

		apply_bindings(payload, bindings, &mut formatted_call, |binding| {
			let data = match binding {
				BindingValue::Register(Register::Ip) =>
					Cow::Owned(format!("{}", IP_REGISTER.load(deps.storage)?).into()),
				BindingValue::Register(Register::Relayer) =>
					Cow::Owned(format!("{}", RELAYER_REGISTER.load(deps.storage)?).into()),
				BindingValue::Register(Register::This) =>
					Cow::Borrowed(env.contract.address.as_bytes()),
				BindingValue::Register(Register::Result) => Cow::Owned(
					serde_json_wasm::to_vec(&RESULT_REGISTER.load(deps.storage)?)
						.map_err(|_| ContractError::DataSerializationError)?,
				),
				BindingValue::Asset(asset_id) => {
					let reference = external_query_lookup_asset(
						deps.querier,
						registry_address.clone().into(),
						asset_id,
					)?;
					match reference {
						AssetReference::Virtual { cw20_address } =>
							Cow::Owned(cw20_address.into_string().into()),
						AssetReference::Native { denom } => Cow::Owned(denom.into()),
					}
				},
				BindingValue::AssetAmount(asset_id, balance) => {
					let reference = external_query_lookup_asset(
						deps.querier,
						registry_address.clone().into(),
						asset_id,
					)?;
					let amount = match reference {
						AssetReference::Virtual { cw20_address } => apply_amount_to_cw20_balance(
							deps,
							&balance,
							&cw20_address,
							&env.contract.address,
						)?,
						AssetReference::Native { denom } =>
							if balance.is_unit {
								return Err(ContractError::InvalidBindings)
							} else {
								let coin = deps
									.querier
									.query_balance(env.contract.address.clone(), denom.clone())?;
								balance.amount.apply(coin.amount.into())
							},
					};
					Cow::Owned(format!("{}", amount).into())
				},
			};
			Ok(data)
		})?;

		serde_json_wasm::from_slice(&formatted_call)
			.map_err(|_| ContractError::InvalidCallPayload)?
	} else {
		// We don't have any binding, just deserialize the data
		serde_json_wasm::from_slice(&payload).map_err(|_| ContractError::InvalidCallPayload)?
	};

	let cosmos_msg: CosmosMsg =
		flat_cosmos_msg.try_into().map_err(|_| ContractError::DataSerializationError)?;
	Ok(response
		.add_event(Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("instruction", "call"))
		.add_submessage(SubMsg::reply_on_success(cosmos_msg, CALL_ID)))
}

pub fn interpret_spawn(
	deps: &DepsMut,
	env: &Env,
	network: NetworkId,
	bridge_security: BridgeSecurity,
	salt: Vec<u8>,
	assets: Funds<Balance>,
	program: XCVMProgram,
	mut response: Response,
) -> Result<Response, ContractError> {
	let Config { interpreter_origin, registry_address, router_address, .. } =
		CONFIG.load(deps.storage)?;

	let registry_address = registry_address.into_string();
	let mut normalized_funds: Funds<Displayed<u128>> = Funds::empty();

	for (asset_id, balance) in assets.0 {
		if balance.amount.is_zero() {
			// We ignore zero amounts
			continue
		}

		let reference =
			external_query_lookup_asset(deps.querier, registry_address.clone(), asset_id)?;
		let transfer_amount = match &reference {
			AssetReference::Native { denom } => {
				if balance.is_unit {
					return Err(ContractError::DecimalsInNativeToken)
				}
				let coin =
					deps.querier.query_balance(env.contract.address.clone(), denom.clone())?;
				balance.amount.apply(coin.amount.into())
			},
			AssetReference::Virtual { cw20_address } => apply_amount_to_cw20_balance(
				deps.as_ref(),
				&balance,
				cw20_address,
				&env.contract.address,
			)?,
		};

		if !transfer_amount.is_zero() {
			let asset_id: u128 = asset_id.into();
			normalized_funds.0.push((asset_id.into(), transfer_amount.into()));
			response = match reference {
				AssetReference::Native { denom } => response.add_message(BankMsg::Send {
					to_address: router_address.clone().into(),
					amount: vec![Coin { denom, amount: transfer_amount.into() }],
				}),
				AssetReference::Virtual { cw20_address } => response.add_message(
					Cw20Contract(cw20_address).call(Cw20ExecuteMsg::Transfer {
						recipient: router_address.clone().into(),
						amount: transfer_amount.into(),
					})?,
				),
			};
		}
	}

	Ok(response
		.add_message(wasm_execute(
			router_address,
			&cw_xcvm_common::router::ExecuteMsg::BridgeForward {
				msg: BridgeMsg {
					interpreter_origin: interpreter_origin.clone(),
					network_id: network,
					security: bridge_security,
					salt,
					program,
					assets: normalized_funds,
				},
			},
			Default::default(),
		)?)
		.add_event(
			Event::new(XCVM_INTERPRETER_EVENT_PREFIX)
				.add_attribute("instruction", "spawn")
				.add_attribute(
					"origin_network_id",
					serde_json_wasm::to_string(&interpreter_origin.user_origin.network_id)
						.map_err(|_| ContractError::DataSerializationError)?,
				)
				.add_attribute(
					"origin_user_id",
					serde_json_wasm::to_string(&interpreter_origin.user_origin.user_id)
						.map_err(|_| ContractError::DataSerializationError)?,
				),
		))
}

pub fn interpret_transfer(
	deps: &mut DepsMut,
	env: &Env,
	relayer: Addr,
	to: Destination<CanonicalAddr>,
	assets: Funds<Balance>,
	mut response: Response,
) -> Result<Response, ContractError> {
	let config = CONFIG.load(deps.storage)?;
	let registry_addr = config.registry_address.into_string();

	let recipient = match to {
		Destination::Account(account) => deps.api.addr_humanize(&account)?.into_string(),
		Destination::Relayer => relayer.into(),
	};

	for (asset_id, balance) in assets.0 {
		if balance.amount.is_zero() {
			continue
		}

		let reference = external_query_lookup_asset(deps.querier, registry_addr.clone(), asset_id)?;
		response = match reference {
			AssetReference::Native { denom } => {
				if balance.is_unit {
					return Err(ContractError::DecimalsInNativeToken)
				}
				let mut coin = deps.querier.query_balance(env.contract.address.clone(), denom)?;
				coin.amount = balance.amount.apply(coin.amount.into()).into();
				response.add_message(BankMsg::Send {
					to_address: recipient.clone(),
					amount: vec![coin],
				})
			},
			AssetReference::Virtual { cw20_address } => {
				let contract = Cw20Contract(cw20_address.clone());
				let transfer_amount = apply_amount_to_cw20_balance(
					deps.as_ref(),
					&balance,
					&cw20_address,
					&env.contract.address,
				)?;
				response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
					recipient: recipient.clone(),
					amount: transfer_amount.into(),
				})?)
			},
		};
	}

	Ok(response.add_event(
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("instruction", "transfer"),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::Register(Register::Ip) => Ok(to_binary(&IP_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::Result) =>
			Ok(to_binary(&RESULT_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::This) => Ok(to_binary(&env.contract.address)?),
		QueryMsg::Register(Register::Relayer) =>
			Ok(to_binary(&RELAYER_REGISTER.load(deps.storage)?)?),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
	match msg.id {
		CALL_ID => handle_call_result(deps, msg),
		SELF_CALL_ID => handle_self_call_result(deps, msg),
		id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
	}
}

fn handle_self_call_result(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	// TODO(aeryz): we can have an intermediate data type to bundle all errors with the IP_REGISTER
	match msg.result.into_result() {
		Ok(_) => Err(StdError::generic_err("Returned OK from a reply that is called with `reply_on_error`. This should never happen")),
		Err(e) => {
			// Save the result that is returned from the sub-interpreter
			// this way, only the `RESULT_REGISTER` is persisted. All
			// other state changes are reverted.
			RESULT_REGISTER.save(deps.storage, &Err(e))?;
			// Ip register should be incremented by one
			let ip_register = IP_REGISTER.load(deps.storage)?;
			IP_REGISTER.save(deps.storage, &(ip_register + 1))?;
			Ok(Response::default())
		}
	}
}

fn handle_call_result(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	RESULT_REGISTER.save(deps.storage, &Ok(response.clone()))?;
	Ok(Response::default())
}

/// Calculates and returns the actual balance to process
///
/// * `balance`: Balance to be transformed into the actual balance
/// * `cw20_address`: Address of the corresponding cw20 contract
/// * `self_address`: This interpreter's address
fn apply_amount_to_cw20_balance<A: Into<String> + Clone>(
	deps: Deps,
	balance: &Balance,
	cw20_address: A,
	self_address: A,
) -> Result<u128, ContractError> {
	let balance_response =
		deps.querier.query::<BalanceResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
			contract_addr: cw20_address.clone().into(),
			msg: to_binary(&Cw20QueryMsg::Balance { address: self_address.into() })?,
		}))?;

	let processed_amount = if balance.is_unit {
		// If the balance is unit, we need to take `decimals` into account.
		let token_info =
			deps.querier.query::<TokenInfoResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
				contract_addr: cw20_address.into(),
				msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
			}))?;
		balance
			.amount
			.apply_with_decimals(token_info.decimals, balance_response.balance.into())
	} else {
		balance.amount.apply(balance_response.balance.into())
	};

	Ok(processed_amount)
}
