use crate::{
	authenticate::{ensure_owner, Authenticated},
	error::{ContractError, Result},
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, Step},
	state::{Config, CONFIG, IP_REGISTER, OWNERS, RESULT_REGISTER, TIP_REGISTER},
};
use alloc::borrow::Cow;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	ensure, to_binary, wasm_execute, Addr, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Deps,
	DepsMut, Env, Event, MessageInfo, QuerierWrapper, QueryRequest, Reply, Response, StdError,
	StdResult, SubMsg, WasmQuery,
};
use cw2::set_contract_version;
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg, TokenInfoResponse};
use cw_utils::ensure_from_older_version;
use num::Zero;
use xc_core::{
	apply_bindings,
	gateway::{
		AssetItem, AssetReference, BridgeForwardMsg, ExecuteMsg as GWExecuteMsg, ExecuteProgramMsg,
	},
	shared::{encode_base64, XcProgram},
	AssetId, Balance, BindingValue, Destination, Displayed, Funds, Instruction, NetworkId,
	Register,
};

const CONTRACT_NAME: &str = "composable:xcvm-interpreter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CALL_ID: u64 = 1;
const SELF_CALL_ID: u64 = 2;
pub const XCVM_INTERPRETER_EVENT_PREFIX: &str = "xcvm.interpreter";
pub const XCVM_INTERPRETER_EVENT_DATA_ORIGIN: &str = "data";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

	let gateway_address = deps.api.addr_validate(&msg.gateway_address)?;
	let config = Config { gateway_address, interpreter_origin: msg.interpreter_origin };
	CONFIG.save(deps.storage, &config)?;
	// Save the caller as owner, in most cases, it is the `XCVM router`
	OWNERS.save(deps.storage, info.sender, &())?;

	Ok(Response::new().add_event(Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute(
		XCVM_INTERPRETER_EVENT_DATA_ORIGIN,
		encode_base64(&config.interpreter_origin)?.as_str(),
	)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result {
	// Only owners can execute entrypoints of the interpreter
	let token = ensure_owner(deps.as_ref(), &env.contract.address, info.sender.clone())?;
	match msg {
		ExecuteMsg::Execute { tip, program } => initiate_execution(token, deps, env, tip, program),

		ExecuteMsg::ExecuteStep { step } => {
			ensure!(env.contract.address == info.sender, ContractError::NotSelf);
			handle_execute_step(token, deps, env, step)
		},

		ExecuteMsg::AddOwners { owners } => add_owners(token, deps, owners),

		ExecuteMsg::RemoveOwners { owners } => Ok(remove_owners(token, deps, owners)),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

	// Already only callable by the admin of the contract, so no need to `ensure_owner`
	let token = ensure_owner(deps.as_ref(), &env.contract.address, env.contract.address.clone())?;
	let _ = add_owners(token, deps, msg.owners)?;
	Ok(Response::default())
}

fn external_query_lookup_asset(
	querier: QuerierWrapper,
	gateway_addr: Addr,
	asset_id: AssetId,
) -> StdResult<AssetItem> {
	let query = xc_core::gateway::QueryMsg::GetAssetById { asset_id };
	let msg = WasmQuery::Smart { contract_addr: gateway_addr.into(), msg: to_binary(&query)? };
	querier
		.query::<xc_core::gateway::GetAssetResponse>(&msg.into())
		.map(|response| response.asset)
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
	tip: Addr,
	program: XcProgram,
) -> Result {
	// Reset instruction pointer to zero.
	IP_REGISTER.save(deps.storage, &0)?;
	Ok(Response::default()
		.add_event(
			Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "execution.start"),
		)
		.add_submessage(SubMsg::reply_on_error(
			wasm_execute(
				env.contract.address,
				&ExecuteMsg::ExecuteStep { step: Step { tip, instruction_pointer: 0, program } },
				Default::default(),
			)?,
			SELF_CALL_ID,
		)))
}

/// Add owners who can execute entrypoints other than `ExecuteStep`
fn add_owners(_: Authenticated, deps: DepsMut, owners: Vec<Addr>) -> Result {
	let mut event = Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "owners.add");
	for owner in owners {
		event = event.add_attribute("owner", owner.to_string());
		OWNERS.save(deps.storage, owner, &())?;
	}
	Ok(Response::default().add_event(event))
}

/// Remove a set of owners from the current owners list.
/// Beware that emptying the set of owners result in a tombstoned interpreter.
fn remove_owners(_: Authenticated, deps: DepsMut, owners: Vec<Addr>) -> Response {
	let mut event =
		Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("action", "owners.remove");
	for owner in owners {
		event = event.add_attribute("owner", owner.to_string());
		OWNERS.remove(deps.storage, owner);
	}
	Response::default().add_event(event)
}

/// Execute an XCVM program.
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
	Step { tip, instruction_pointer, mut program }: Step,
) -> Result {
	Ok(if let Some(instruction) = program.instructions.pop_front() {
		let response = match instruction {
			Instruction::Transfer { to, assets } =>
				interpret_transfer(&mut deps, &env, &tip, to, assets),
			Instruction::Call { bindings, encoded } =>
				interpret_call(deps.as_ref(), &env, bindings, encoded, instruction_pointer, &tip),
			Instruction::Spawn { network, salt, assets, program } =>
				interpret_spawn(&mut deps, &env, network, salt, assets, program),
			Instruction::Exchange { .. } => Err(ContractError::NotImplemented)?,
		}?;
		// Save the intermediate IP so that if the execution fails, we can recover at which
		// instruction it happened.
		IP_REGISTER.update::<_, ContractError>(deps.storage, |x| Ok(x + 1))?;
		response.add_message(wasm_execute(
			env.contract.address,
			&ExecuteMsg::ExecuteStep {
				step: Step { tip, instruction_pointer: instruction_pointer + 1, program },
			},
			Default::default(),
		)?)
	} else {
		// We subtract because of the extra loop to reach the empty instructions case.
		IP_REGISTER.save(deps.storage, &instruction_pointer.saturating_sub(1))?;
		TIP_REGISTER.save(deps.storage, &tip)?;
		let mut event = Event::new("xc.interpreter.step.executed")
			.add_attribute("tag", hex::encode(&program.tag));
		if program.tag.len() >= 3 {
			event = event.add_attribute(
				"tag",
				core::str::from_utf8(&program.tag).map_err(|_| ContractError::InvalidProgramTag)?,
			);
		}
		Response::default().add_event(event)
	})
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
	instruction_pointer: u16,
	tip: &Addr,
) -> Result {
	// we hacky using json, but we always know ABI encoding dependng on chain we run on send to
	let flat_cosmos_msg: xc_core::cosmwasm::FlatCosmosMsg<serde_cw_value::Value> = if !bindings
		.is_empty()
	{
		let Config { gateway_address, .. } = CONFIG.load(deps.storage)?;
		// Len here is the maximum possible length
		let mut formatted_call =
			vec![0; env.contract.address.as_bytes().len() * bindings.len() + payload.len()];

		apply_bindings(payload, bindings, &mut formatted_call, |binding| {
			let data = match binding {
				BindingValue::Register(Register::Ip) =>
					Cow::Owned(instruction_pointer.to_string().into_bytes()),
				BindingValue::Register(Register::Tip) => Cow::Owned(tip.to_string().into_bytes()),
				BindingValue::Register(Register::This) =>
					Cow::Borrowed(env.contract.address.as_bytes()),
				BindingValue::Register(Register::Result) => Cow::Owned(
					serde_json_wasm::to_vec(&RESULT_REGISTER.load(deps.storage)?)
						.map_err(|_| ContractError::DataSerializationError)?,
				),
				BindingValue::Asset(asset_id) => {
					let reference = external_query_lookup_asset(
						deps.querier,
						gateway_address.clone(),
						asset_id,
					)?;
					match reference.local {
						AssetReference::Cw20 { contract } =>
							Cow::Owned(contract.into_string().into()),
						AssetReference::Native { denom } => Cow::Owned(denom.into()),
					}
				},
				BindingValue::AssetAmount(asset_id, balance) => {
					let reference = external_query_lookup_asset(
						deps.querier,
						gateway_address.clone(),
						asset_id,
					)?;
					let amount = match reference.local {
						AssetReference::Cw20 { contract } => apply_amount_to_cw20_balance(
							deps,
							&balance,
							&contract,
							&env.contract.address,
						),
						AssetReference::Native { denom } =>
							if balance.is_unit {
								return Err(ContractError::InvalidBindings)
							} else {
								let coin = deps
									.querier
									.query_balance(env.contract.address.clone(), denom)?;
								balance
									.amount
									.apply(coin.amount.into())
									.map_err(|_| ContractError::ArithmeticError)
							},
					}?;
					Cow::Owned(amount.to_string().into_bytes())
				},
			};
			Ok(data)
		})?;

		serde_json_wasm::from_slice(&formatted_call)
			.map_err(|_| ContractError::InvalidCallPayload)?
	} else {
		serde_json_wasm::from_slice(&payload).map_err(|_| ContractError::InvalidCallPayload)?
	};

	let cosmos_msg: CosmosMsg =
		flat_cosmos_msg.try_into().map_err(|_| ContractError::DataSerializationError)?;
	Ok(Response::default()
		.add_event(Event::new(XCVM_INTERPRETER_EVENT_PREFIX).add_attribute("instruction", "call"))
		.add_submessage(SubMsg::reply_on_success(cosmos_msg, CALL_ID)))
}

pub fn interpret_spawn(
	deps: &mut DepsMut,
	env: &Env,
	network: NetworkId,
	salt: Vec<u8>,
	assets: Funds<Balance>,
	program: XcProgram,
) -> Result {
	let Config { interpreter_origin, gateway_address, .. } = CONFIG.load(deps.storage)?;

	let mut normalized_funds: Funds<Displayed<u128>> = Funds::default();

	let mut response = Response::default();
	for (asset_id, balance) in assets.0 {
		let reference =
			external_query_lookup_asset(deps.querier, gateway_address.clone(), asset_id)?;
		let transfer_amount = match &reference.local {
			AssetReference::Native { denom } => {
				if balance.is_unit {
					return Err(ContractError::DecimalsInNativeToken)
				}
				let coin =
					deps.querier.query_balance(env.contract.address.clone(), denom.clone())?;
				balance
					.amount
					.apply(coin.amount.into())
					.map_err(|_| ContractError::ArithmeticError)
			},
			AssetReference::Cw20 { contract } => apply_amount_to_cw20_balance(
				deps.as_ref(),
				&balance,
				contract,
				&env.contract.address,
			),
		}?;

		if !transfer_amount.is_zero() {
			let asset_id: u128 = asset_id.into();
			normalized_funds.0.push((asset_id.into(), transfer_amount.into()));
			response = match reference.local {
				AssetReference::Native { denom } => response.add_message(BankMsg::Send {
					to_address: gateway_address.clone().into(),
					amount: vec![Coin { denom, amount: transfer_amount.into() }],
				}),
				AssetReference::Cw20 { contract } =>
					response.add_message(Cw20Contract(contract).call(Cw20ExecuteMsg::Transfer {
						recipient: gateway_address.clone().into(),
						amount: transfer_amount.into(),
					})?),
			};
		}
	}

	let execute_program = ExecuteProgramMsg { salt, program, assets: normalized_funds };
	Ok(response
		.add_message(wasm_execute(
			gateway_address,
			&GWExecuteMsg::BridgeForward(BridgeForwardMsg {
				interpreter_origin: interpreter_origin.clone(),
				msg: execute_program,
				to: network,
			}),
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
				)
				.add_attribute("network_id", network.to_string()),
		))
}

pub fn interpret_transfer(
	deps: &mut DepsMut,
	env: &Env,
	tip: &Addr,
	to: Destination<CanonicalAddr>,
	assets: Funds<Balance>,
) -> Result {
	let Config { gateway_address, .. } = CONFIG.load(deps.storage)?;

	let recipient = match to {
		Destination::Account(account) => deps.api.addr_humanize(&account)?.into_string(),
		Destination::Tip => tip.into(),
	};

	let mut response = Response::default();
	for (asset_id, balance) in assets.0 {
		if balance.amount.is_zero() {
			continue
		}

		let reference =
			external_query_lookup_asset(deps.querier, gateway_address.clone(), asset_id)?;
		response = match reference.local {
			AssetReference::Native { denom } => {
				if balance.is_unit {
					return Err(ContractError::DecimalsInNativeToken)
				}
				let mut coin = deps.querier.query_balance(env.contract.address.clone(), denom)?;
				coin.amount = balance.amount.apply(coin.amount.into())?.into();
				response.add_message(BankMsg::Send {
					to_address: recipient.clone(),
					amount: vec![coin],
				})
			},
			AssetReference::Cw20 { contract } => {
				let contract = Cw20Contract(contract.clone());
				let transfer_amount = apply_amount_to_cw20_balance(
					deps.as_ref(),
					&balance,
					&contract.0,
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
		QueryMsg::Register(Register::Tip) => Ok(to_binary(&TIP_REGISTER.load(deps.storage)?)?),
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
	match msg.result.into_result() {
		Ok(_) => Err(StdError::generic_err("Returned OK from a reply that is called with `reply_on_error`. This should never happen")),
		Err(e) => {
			// Save the result that is returned from the sub-interpreter
			// this way, only the `RESULT_REGISTER` is persisted. All
			// other state changes are reverted.
			RESULT_REGISTER.save(deps.storage, &Err(e.clone()))?;
			let ip = IP_REGISTER.load(deps.storage)?.to_string();
			let event = Event::new(XCVM_INTERPRETER_EVENT_PREFIX)
				.add_attribute("action", "execution.failure")
				.add_attribute("reason", e);
			Ok(Response::default().add_event(event).add_attribute("ip", ip))
		}
	}
}

fn handle_call_result(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	RESULT_REGISTER.save(deps.storage, &Ok(response))?;
	Ok(Response::default())
}

/// Calculates and returns the actual balance to process
///
/// * `balance`: Balance to be transformed into the actual balance
/// * `contract`: Address of the corresponding cw20 contract
/// * `self_address`: This interpreter's address
fn apply_amount_to_cw20_balance<A: Into<String> + Clone>(
	deps: Deps,
	balance: &Balance,
	contract: A,
	self_address: A,
) -> Result<u128> {
	let balance_response =
		deps.querier.query::<BalanceResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
			contract_addr: contract.clone().into(),
			msg: to_binary(&Cw20QueryMsg::Balance { address: self_address.into() })?,
		}))?;

	if balance.is_unit {
		// If the balance is unit, we need to take `decimals` into account.
		let token_info =
			deps.querier.query::<TokenInfoResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
				contract_addr: contract.into(),
				msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
			}))?;
		balance
			.amount
			.apply_with_decimals(token_info.decimals, balance_response.balance.into())
	} else {
		balance.amount.apply(balance_response.balance.into())
	}
	.map_err(ContractError::from)
}
