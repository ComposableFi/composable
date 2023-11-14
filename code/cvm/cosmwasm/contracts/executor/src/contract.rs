use crate::{
	authenticate::{ensure_owner, Authenticated},
	error::{ContractError, Result},
	events::*,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, Step},
	state::{self, Config, CONFIG, IP_REGISTER, OWNERS, RESULT_REGISTER, TIP_REGISTER},
};
use alloc::borrow::Cow;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	ensure, ensure_eq, to_binary, wasm_execute, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps,
	DepsMut, Env, MessageInfo, QueryRequest, Reply, Response, StdError, StdResult, SubMsg,
	SubMsgResult, WasmQuery,
};
use cw2::set_contract_version;
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use cw_utils::ensure_from_older_version;
use num::Zero;
use xc_core::{
	apply_bindings,
	gateway::{AssetReference, BridgeExecuteProgramMsg, BridgeForwardMsg},
	service::dex::{
		osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute, ExchangeId,
	},
	shared, Amount, BindingValue, Destination, Funds, Instruction, NetworkId, Register,
};

const CONTRACT_NAME: &str = "cvm-executor";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CALL_ID: u64 = 1;
const SELF_CALL_ID: u64 = 2;
const EXCHANGE_ID: u64 = 3;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InstantiateMsg) -> Result {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	let gateway_address = xc_core::gateway::Gateway::addr_validate(deps.api, &msg.gateway_address)?;
	let config = Config { gateway_address, interpreter_origin: msg.interpreter_origin };
	CONFIG.save(deps.storage, &config)?;
	OWNERS.save(deps.storage, info.sender, &())?;
	Ok(Response::new().add_event(CvmInterpreterInstantiated::new(&config.interpreter_origin)))
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

		ExecuteMsg::SetErr { reason } => handle_set_error(token, deps, reason, env),
	}
}

fn handle_set_error(_: Authenticated, deps: DepsMut, reason: String, _env: Env) -> Result {
	RESULT_REGISTER.save(deps.storage, &Err(reason.clone()))?;
	let event = CvmInterpreterCrosschainFailed::new(reason);
	Ok(Response::default().add_event(event))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result {
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
	tip: Addr,
	program: shared::XcProgram,
) -> Result {
	// Reset instruction pointer to zero.
	IP_REGISTER.save(deps.storage, &0)?;
	Ok(Response::default()
		.add_event(CvmInterpreterExecutionStarted::new())
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
	for owner in owners.iter() {
		OWNERS.save(deps.storage, owner.clone(), &())?;
	}
	Ok(Response::default().add_event(CvmInterpreterOwnerAdded::new(owners)))
}

/// Remove a set of owners from the current owners list.
/// Beware that emptying the set of owners result in a tombstoned interpreter.
fn remove_owners(_: Authenticated, deps: DepsMut, owners: Vec<Addr>) -> Response {
	for owner in owners.iter() {
		OWNERS.remove(deps.storage, owner.clone());
	}
	Response::default().add_event(CvmInterpreterOwnerRemoved::new(owners))
}

/// Execute an CVM program.
/// The function will execute the program instructions one by one.
/// If the program contains a [`CVMInstruction::Call`], the execution is suspended and resumed
/// after having executed the call.
/// The [`IP_REGISTER`] is updated accordingly.
/// A final `executed` event is yield whenever a program come to completion (all it's instructions
/// has been executed).
/// If some step fails, its result is recorded in the [`RESULT_REGISTER`] and the execution is
/// halted. Default behavior not to abort transaction.
pub fn handle_execute_step(
	_: Authenticated,
	mut deps: DepsMut,
	env: Env,
	Step { tip, instruction_pointer, mut program }: Step,
) -> Result {
	Ok(if let Some(instruction) = program.instructions.pop_front() {
		deps.api.debug(&format!("cvm::executor::execute:: {:?}", &instruction));
		let response = match instruction {
			Instruction::Transfer { to, assets } =>
				interpret_transfer(&mut deps, &env, &tip, to, assets),
			Instruction::Call { bindings, encoded } =>
				interpret_call(deps.as_ref(), &env, bindings, encoded, instruction_pointer, &tip),
			Instruction::Spawn { network_id, salt, assets, program } =>
				interpret_spawn(&mut deps, &env, network_id, salt, assets, program),
			Instruction::Exchange { exchange_id, give, want } =>
				interpret_exchange(&mut deps, give, want, exchange_id, env.contract.address.clone()),
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
		Response::default().add_event(CvmInterpreterStepExecuted::new(&program.tag))
	})
}

fn interpret_exchange(
	deps: &mut DepsMut,
	give: Funds,
	want: Funds,
	exchange_id: ExchangeId,
	sender: Addr,
) -> Result {
	let Config { gateway_address, .. } = CONFIG.load(deps.storage)?;
	let exchange: xc_core::service::dex::ExchangeItem = gateway_address
		.get_exchange_by_id(deps.querier, exchange_id)
		.map_err(ContractError::ExchangeNotFound)?;

	use prost::Message;
	use xc_core::service::dex::{
		osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn, ExchangeType::*,
	};
	ensure_eq!(give.0.len(), 1, ContractError::OnlySingleAssetExchangeIsSupportedByPool);
	ensure_eq!(want.0.len(), 1, ContractError::OnlySingleAssetExchangeIsSupportedByPool);

	let give = give.0[0].clone();
	let want = want.0[0].clone();

	let asset = gateway_address
		.get_asset_by_id(deps.querier, give.0)
		.map_err(ContractError::AssetNotFound)?;

	let amount: Coin = deps.querier.query_balance(&sender, asset.denom())?;
	let amount = give.1.apply(amount.amount.u128())?;
	let give: xc_core::cosmos::Coin =
		xc_core::cosmos::Coin { denom: asset.denom(), amount: amount.to_string() };

	let asset = gateway_address
		.get_asset_by_id(deps.querier, want.0)
		.map_err(ContractError::AssetNotFound)?;

	if want.1.is_absolute() && want.1.is_ratio() {
		return Err(ContractError::CannotDefineBothSlippageAndLimitAtSameTime)
	}

	let want = if want.1.is_absolute() {
		xc_core::cosmos::Coin { denom: asset.denom(), amount: want.1.intercept.to_string() }
	} else {
		// use https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/contracts/swaprouter/src/msg.rs to allow slippage
		xc_core::cosmos::Coin { denom: asset.denom(), amount: "1".to_string() }
	};

	let response = match exchange.exchange {
		OsmosisCrossChainSwap { pool_id, .. } => {
			let msg = MsgSwapExactAmountIn {
				routes: vec![SwapAmountInRoute { pool_id, token_out_denom: want.denom }],
				sender: sender.to_string(),
				token_in: Some(give),
				token_out_min_amount: want.amount,
			};
			deps.api.debug(&format!("cvm::executor::execute::exchange {:?}", &msg));
			let msg = CosmosMsg::Stargate {
				type_url: MsgSwapExactAmountIn::PROTO_MESSAGE_URL.to_string(),
				value: Binary::from(msg.encode_to_vec()),
			};
			let msg = SubMsg::reply_always(msg, EXCHANGE_ID);
			Response::default()
				.add_submessage(msg)
				.add_attribute("exchange_id", exchange_id.to_string())
		},
	};
	Ok(response.add_event(CvmInterpreterExchangeStarted::new(exchange_id)))
}

/// Interpret the `Call` instruction
/// * `encoded`: JSON-encoded `LateCall` as bytes
///
/// Late-bindings are actually done in this function. If our CVM SDK is not used,
/// make sure that indices in the `LateCall` is sorted in an ascending order.
pub fn interpret_call(
	deps: Deps,
	env: &Env,
	bindings: Vec<(u32, BindingValue)>,
	mut payload: Vec<u8>,
	instruction_pointer: u16,
	tip: &Addr,
) -> Result {
	if !bindings.is_empty() {
		let resolver = BindingResolver::new(&deps, env, instruction_pointer, tip)?;
		let p = core::mem::take(&mut payload);
		payload = apply_bindings(p, &bindings, |binding| resolver.resolve(binding))?;
	}
	// we hacky using json, but we always know ABI encoding dependng on chain we
	// run on send to
	let cosmos_msg: CosmosMsg = serde_json_wasm::from_slice::<
		xc_core::cosmwasm::FlatCosmosMsg<serde_cw_value::Value>,
	>(&payload)
	.map_err(|_| ContractError::InvalidCallPayload)?
	.try_into()
	.map_err(|_| ContractError::DataSerializationError)?;
	Ok(Response::default()
		.add_event(CvmInterpreterInstructionCallInitiated::new())
		.add_submessage(SubMsg::reply_on_success(cosmos_msg, CALL_ID)))
}

/// Resolver for `BindingValue`s.
struct BindingResolver<'a> {
	deps: &'a Deps<'a>,
	env: &'a Env,
	instruction_pointer: u16,
	tip: &'a Addr,
	gateway: xc_core::gateway::Gateway,
}

impl<'a> BindingResolver<'a> {
	/// Creates a new binding resolver.
	///
	/// Fetches gateway configuration from storage thus it may fail with storage
	/// read error.
	fn new(deps: &'a Deps, env: &'a Env, instruction_pointer: u16, tip: &'a Addr) -> Result<Self> {
		let Config { gateway_address: gateway, .. } = CONFIG.load(deps.storage)?;
		Ok(Self { deps, env, instruction_pointer, tip, gateway })
	}

	/// Resolves a single binding returning it’s value.
	fn resolve(&'a self, binding: &BindingValue) -> Result<Cow<'a, [u8]>> {
		match binding {
			BindingValue::Register(reg) => self.resolve_register(*reg),
			BindingValue::Asset(asset_id) => self.resolve_asset(*asset_id),
			BindingValue::AssetAmount(asset_id, balance) =>
				self.resolve_asset_amount(*asset_id, balance),
		}
	}

	fn resolve_register(&'a self, reg: Register) -> Result<Cow<'a, [u8]>> {
		Ok(match reg {
			Register::Carry(_) => Err(ContractError::NotImplemented)?,
			Register::Ip => Cow::Owned(self.instruction_pointer.to_string().into_bytes()),
			Register::Tip => Cow::Owned(self.tip.to_string().into_bytes()),
			Register::This => Cow::Borrowed(self.env.contract.address.as_bytes()),
			Register::Result => Cow::Owned(
				serde_json_wasm::to_vec(&RESULT_REGISTER.load(self.deps.storage)?)
					.map_err(|_| ContractError::DataSerializationError)?,
			),
		})
	}

	fn resolve_asset(&'a self, asset_id: xc_core::AssetId) -> Result<Cow<'a, [u8]>> {
		let reference = self.gateway.get_asset_by_id(self.deps.querier, asset_id)?;
		let value = match reference.local {
			AssetReference::Cw20 { contract } => contract.into_string(),
			AssetReference::Native { denom } => denom,
			AssetReference::Erc20 { contract } => contract.to_string(),
		};
		Ok(Cow::Owned(value.into()))
	}

	fn resolve_asset_amount(
		&'a self,
		asset_id: xc_core::AssetId,
		balance: &Amount,
	) -> Result<Cow<'a, [u8]>> {
		let reference = self.gateway.get_asset_by_id(self.deps.querier, asset_id)?;
		let amount = match reference.local {
			AssetReference::Cw20 { contract } => apply_amount_to_cw20_balance(
				*self.deps,
				balance,
				&contract,
				&self.env.contract.address,
			)?,
			AssetReference::Native { denom } => {
				let coin =
					self.deps.querier.query_balance(self.env.contract.address.clone(), denom)?;
				balance.apply(coin.amount.into()).map_err(|_| ContractError::ArithmeticError)?
			},
			AssetReference::Erc20 { .. } => Err(ContractError::AssetUnsupportedOnThisNetwork)?,
		};
		Ok(Cow::Owned(amount.to_string().into_bytes()))
	}
}

pub fn interpret_spawn(
	deps: &mut DepsMut,
	env: &Env,
	network_id: NetworkId,
	salt: Vec<u8>,
	assets: Funds<Amount>,
	program: shared::XcProgram,
) -> Result {
	let Config { interpreter_origin, gateway_address: gateway, .. } = CONFIG.load(deps.storage)?;

	let mut normalized_funds = Funds::default();

	let mut response = Response::default();
	for (asset_id, balance) in assets.0 {
		let reference = gateway.get_asset_by_id(deps.querier, asset_id)?;
		let transfer_amount = match &reference.local {
			AssetReference::Native { denom } => {
				let coin =
					deps.querier.query_balance(env.contract.address.clone(), denom.clone())?;
				balance.apply(coin.amount.into()).map_err(|_| ContractError::ArithmeticError)
			},
			AssetReference::Cw20 { contract } => apply_amount_to_cw20_balance(
				deps.as_ref(),
				&balance,
				contract,
				&env.contract.address,
			),
			AssetReference::Erc20 { .. } => Err(ContractError::AssetUnsupportedOnThisNetwork)?,
		}?;

		if !transfer_amount.is_zero() {
			let asset_id: u128 = asset_id.into();
			normalized_funds.0.push((asset_id.into(), transfer_amount.into()));
			response = match reference.local {
				AssetReference::Native { denom } => response.add_message(BankMsg::Send {
					to_address: gateway.address().into(),
					amount: vec![Coin { denom, amount: transfer_amount.into() }],
				}),
				AssetReference::Cw20 { contract } =>
					response.add_message(Cw20Contract(contract).call(Cw20ExecuteMsg::Transfer {
						recipient: gateway.address().into(),
						amount: transfer_amount.into(),
					})?),
				AssetReference::Erc20 { .. } => Err(ContractError::AssetUnsupportedOnThisNetwork)?,
			};
		}
	}

	let execute_program =
		BridgeExecuteProgramMsg { salt, program, assets: normalized_funds, tip: None };
	Ok(response
		.add_message(gateway.execute(BridgeForwardMsg {
			executor_origin: interpreter_origin.clone(),
			msg: execute_program,
			to: network_id,
		})?)
		.add_event(CvmInterpreterInstructionSpawned::new(
			interpreter_origin.user_origin.network_id,
			interpreter_origin.user_origin.user_id,
			network_id,
		)))
}

pub fn interpret_transfer(
	deps: &mut DepsMut,
	env: &Env,
	tip: &Addr,
	to: Destination<shared::XcAddr>,
	assets: Funds<Amount>,
) -> Result {
	let Config { gateway_address: gateway, .. } = CONFIG.load(deps.storage)?;
	deps.api.debug(&format!("cvm::executor::transfer:: to {:?}", &to));
	let recipient = match to {
		Destination::Account(account) => account.encode_cosmwasm(deps.api)?,
		Destination::Tip => tip.into(),
	};

	let mut response = Response::default();
	for (asset_id, balance) in assets.0 {
		let reference = gateway.get_asset_by_id(deps.querier, asset_id)?;
		response = match reference.local {
			AssetReference::Native { denom } => {
				let mut coin = deps.querier.query_balance(env.contract.address.clone(), denom)?;
				coin.amount = balance.apply(coin.amount.into())?.into();
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
			AssetReference::Erc20 { .. } => Err(ContractError::AssetUnsupportedOnThisNetwork)?,
		};
	}

	Ok(response.add_event(CvmInterpreterTransferred::new()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::Register(Register::Ip) => Ok(to_binary(&IP_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::Result) =>
			Ok(to_binary(&RESULT_REGISTER.load(deps.storage)?)?),
		QueryMsg::Register(Register::This) => Ok(to_binary(&env.contract.address)?),
		QueryMsg::Register(Register::Carry(_)) =>
			Err(StdError::generic_err("Carry register is not implemented yet")),
		QueryMsg::Register(Register::Tip) => Ok(to_binary(&TIP_REGISTER.load(deps.storage)?)?),
		QueryMsg::State() => Ok(state::read(deps.storage)?.try_into()?),
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
	match msg.id {
		CALL_ID => handle_call_result(deps, msg),
		SELF_CALL_ID => handle_self_call_result(deps, msg),
		EXCHANGE_ID => handle_exchange_result(deps, msg),
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
			let event = CvmInterpreterSelfFailed::new(e);
			Ok(Response::default().add_event(event).add_attribute("ip", ip))
		}
	}
}

fn handle_call_result(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	let response = msg.result.into_result().map_err(StdError::generic_err)?;
	RESULT_REGISTER.save(deps.storage, &Ok(response))?;
	Ok(Response::default())
}

fn handle_exchange_result(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	deps.api.debug(&format!("cvm::executor::exchanged {:?}", &msg));
	let response = match &msg.result {
		SubMsgResult::Ok(ok) => {
			let exchange_id: ExchangeId = ok
				.events
				.iter()
				.find(|x| x.ty == "cvm.executor.exchange.started")
				.and_then(|x| x.attributes.iter().find(|x| x.key == "exchange_id"))
				.map(|x| x.value.parse().unwrap())
				.unwrap_or(ExchangeId::default());
			Response::new().add_event(CvmInterpreterExchangeSucceeded::new(exchange_id))
		},
		SubMsgResult::Err(err) =>
			Response::new().add_event(CvmInterpreterExchangeFailed::new(err.clone())),
	};
	RESULT_REGISTER.save(deps.storage, &msg.result.into_result())?;
	Ok(response)
}

/// Calculates and returns the actual balance to process
///
/// * `balance`: Balance to be transformed into the actual balance
/// * `contract`: Address of the corresponding cw20 contract
/// * `self_address`: This interpreter's address
fn apply_amount_to_cw20_balance<A: Into<String> + Clone>(
	deps: Deps,
	balance: &Amount,
	contract: A,
	self_address: A,
) -> Result<u128> {
	let balance_response =
		deps.querier.query::<BalanceResponse>(&QueryRequest::Wasm(WasmQuery::Smart {
			contract_addr: contract.clone().into(),
			msg: to_binary(&Cw20QueryMsg::Balance { address: self_address.into() })?,
		}))?;

	balance.apply(balance_response.balance.into()).map_err(ContractError::from)
}
