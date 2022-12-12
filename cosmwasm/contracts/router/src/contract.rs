extern crate alloc;

use crate::{
    error::ContractError,
    msg::{InstantiateMsg, MigrateMsg, QueryMsg},
    state::{Config, Interpreter, CONFIG, INTERPRETERS},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, Event,
    MessageInfo, Reply, Response, StdError, StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg};
use cw_utils::ensure_from_older_version;
use cw_xcvm_asset_registry::{contract::external_query_lookup_asset, msg::AssetReference};
use cw_xcvm_common::{
    router::ExecuteMsg,
    shared::{decode_base64, BridgeMsg},
};
use cw_xcvm_interpreter::contract::{
    XCVM_INTERPRETER_EVENT_DATA_ORIGIN, XCVM_INTERPRETER_EVENT_PREFIX,
};
use cw_xcvm_utils::DefaultXCVMProgram;
use xcvm_core::{BridgeSecurity, CallOrigin, Displayed, Funds, InterpreterOrigin};

const CONTRACT_NAME: &str = "composable:xcvm-router";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_REPLY_ID: u64 = 1;
pub const XCVM_ROUTER_EVENT_PREFIX: &str = "xcvm.router";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let gateway_address = info.sender;
    let registry_address = deps.api.addr_validate(&msg.registry_address)?;
    CONFIG.save(
        deps.storage,
        &Config {
            gateway_address,
            registry_address,
            interpreter_code_id: msg.interpreter_code_id,
            network_id: msg.network_id,
        },
    )?;
    Ok(Response::default()
        .add_event(Event::new(XCVM_ROUTER_EVENT_PREFIX).add_attribute("action", "instantiated")))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ExecuteProgram {
            salt,
            program,
            assets,
        } => {
            let self_address = env.contract.address;
            let call_origin = CallOrigin::Local {
                user: info.sender.clone(),
            };
            let transfers = transfer_from_user(
                &deps,
                self_address.clone(),
                info.sender,
                info.funds,
                &assets,
            )?;
            Ok(Response::default()
                .add_messages(transfers)
                .add_message(wasm_execute(
                    self_address,
                    &ExecuteMsg::ExecuteProgramPrivileged {
                        call_origin,
                        salt,
                        program,
                        assets,
                    },
                    Default::default(),
                )?))
        }

        ExecuteMsg::ExecuteProgramPrivileged {
            call_origin,
            salt,
            program,
            assets,
        } => {
            ensure_self_or_gateway(&deps, &env.contract.address, &info.sender)?;
            handle_execute_program(deps, env, call_origin, salt, program, assets)
        }

        ExecuteMsg::SetInterpreterSecurity {
            interpreter_origin,
            bridge_security,
        } => handle_set_interpreter_security(deps, info, interpreter_origin, bridge_security),

        ExecuteMsg::BridgeForward { msg } => handle_bridge_forward(deps, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

/// Ensure that the `sender` is the router gateway contract.
/// This is used for privileged operations such as [`ExecuteMsg::ExecuteProgramPrivileged`].
fn ensure_self_or_gateway(
    deps: &DepsMut,
    self_address: &Addr,
    sender: &Addr,
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if &config.gateway_address == sender || self_address == sender {
        Ok(())
    } else {
        Err(ContractError::NotAuthorized)
    }
}

/// Ensure that the `sender` is the interpreter for the provided `interpreter_origin`.
/// This function is used whenever we want an operation to be executable by an interpreter,
/// currently [`ExecuteMsg::SetInterpreterSecurity`].
fn ensure_interpreter(
    deps: &DepsMut,
    sender: &Addr,
    interpreter_origin: InterpreterOrigin,
) -> Result<(), ContractError> {
    match INTERPRETERS.load(deps.storage, interpreter_origin) {
        Ok(Interpreter {
            address: Some(address),
            ..
        }) if &address == sender => Ok(()),
        _ => Err(ContractError::NotAuthorized),
    }
}

fn transfer_from_user(
    deps: &DepsMut,
    self_address: Addr,
    user: Addr,
    funds: Vec<Coin>,
    assets: &Funds<Displayed<u128>>,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let mut transfers = Vec::with_capacity(assets.0.len());
    for (asset, Displayed(amount)) in assets.0.iter() {
        let reference =
            external_query_lookup_asset(deps.querier, config.registry_address.to_string(), *asset)?;
        match reference {
            AssetReference::Native { denom } => {
                let Coin {
                    amount: provided_amount,
                    ..
                } = funds
                    .iter()
                    .find(|c| c.denom == denom)
                    .ok_or(ContractError::InsufficientFunds)?;
                if u128::from(*provided_amount) != *amount {
                    return Err(ContractError::InsufficientFunds)?;
                }
            }
            AssetReference::Virtual { cw20_address } => transfers.push(
                Cw20Contract(cw20_address).call(Cw20ExecuteMsg::TransferFrom {
                    owner: user.to_string(),
                    recipient: self_address.to_string(),
                    amount: amount.clone().into(),
                })?,
            ),
        }
    }
    Ok(transfers)
}

/// Handle a request to forward a message to the bridge gateway.
/// The call must originate from an interpreter.
fn handle_bridge_forward(
    deps: DepsMut,
    info: MessageInfo,
    msg: BridgeMsg,
) -> Result<Response, ContractError> {
    ensure_interpreter(&deps, &info.sender, msg.interpreter_origin.clone())?;
    let config = CONFIG.load(deps.storage)?;
    let transfers = msg
        .assets
        .0
        .iter()
        .map(|(asset, Displayed(amount))| {
            let reference = external_query_lookup_asset(
                deps.querier,
                config.registry_address.to_string(),
                *asset,
            )?;
            match reference {
                AssetReference::Native { denom } => Ok(BankMsg::Send {
                    to_address: config.gateway_address.clone().into(),
                    amount: vec![Coin {
                        denom,
                        amount: amount.clone().into(),
                    }],
                }
                .into()),
                AssetReference::Virtual { cw20_address } => {
                    Cw20Contract(cw20_address).call(Cw20ExecuteMsg::Transfer {
                        recipient: config.gateway_address.clone().into(),
                        amount: amount.clone().into(),
                    })
                }
            }
        })
        .collect::<Result<Vec<CosmosMsg>, _>>()?;
    Ok(Response::default()
        .add_messages(transfers)
        .add_message(wasm_execute(
            config.gateway_address,
            &cw_xcvm_common::gateway::ExecuteMsg::Bridge {
                interpreter: info.sender,
                msg,
            },
            Default::default(),
        )?))
}

/// Handle a request to change an interpreter security level.
/// Only the interpreter instance itself is allowed to change it's security level.
/// A user is able to change it's interpreter security level by provided an [`XCVMProgram`] that
/// contains a [`XCVMInstruction::Call`] to the router contract.
fn handle_set_interpreter_security(
    deps: DepsMut,
    info: MessageInfo,
    interpreter_origin: InterpreterOrigin,
    security: BridgeSecurity,
) -> Result<Response, ContractError> {
    // Ensure that the sender is the interpreter for the given user origin.
    // The security of an interpreter can only be altered by the interpreter itself.
    // If a user is willing to alter the default security, he must submit an XCVM program with a
    // call to the router that does it for him.
    ensure_interpreter(&deps, &info.sender, interpreter_origin.clone())?;

    match INTERPRETERS.load(deps.storage, interpreter_origin.clone()) {
        Ok(Interpreter { address, .. }) => INTERPRETERS.save(
            deps.storage,
            interpreter_origin.clone(),
            &Interpreter { address, security },
        ),
        Err(_) => INTERPRETERS.save(
            deps.storage,
            interpreter_origin.clone(),
            &Interpreter {
                address: None,
                security,
            },
        ),
    }?;

    Ok(Response::default().add_event(
        Event::new(XCVM_ROUTER_EVENT_PREFIX)
            .add_attribute("action", "interpreter.setSecurity")
            .add_attribute(
                "network_id",
                format!("{}", u32::from(interpreter_origin.user_origin.network_id)),
            )
            .add_attribute(
                "user_id",
                hex::encode(&interpreter_origin.user_origin.user_id),
            )
            .add_attribute("security", format!("{}", security as u8)),
    ))
}

/// Handle a request to execute a [`XCVMProgram`].
/// Only the gateway is allowed to dispatch such operation.
/// The gateway must ensure that the `CallOrigin` is valid as the router does not do further
/// checking on it.
fn handle_execute_program(
    deps: DepsMut,
    env: Env,
    call_origin: CallOrigin,
    salt: Vec<u8>,
    program: DefaultXCVMProgram,
    assets: Funds<Displayed<u128>>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let interpreter_origin = InterpreterOrigin {
        user_origin: call_origin.user(config.network_id),
        salt,
    };
    match INTERPRETERS.load(deps.storage, interpreter_origin.clone()) {
        Ok(Interpreter {
            address: Some(interpreter_address),
            security,
        }) => {
            // Ensure that the current call origin meet the user expected security.
            call_origin
                .ensure_security(security)
                .map_err(|_| ContractError::ExpectedBridgeSecurity(security))?;

            // There is already an interpreter instance, so all we do is fund the interpreter, then
            // add a callback to it
            let response =
                send_funds_to_interpreter(deps.as_ref(), interpreter_address.clone(), assets)?;
            let wasm_msg = wasm_execute(
                interpreter_address,
                &cw_xcvm_interpreter::msg::ExecuteMsg::Execute {
                    relayer: call_origin.relayer().clone(),
                    program,
                },
                vec![],
            )?;
            Ok(response.add_message(wasm_msg))
        }
        _ => {
            // There is no interpreter, so the bridge security must be at least `Deterministic`
            // or the message should be coming from a local origin.
            call_origin
                .ensure_security(BridgeSecurity::Deterministic)
                .map_err(|_| {
                    ContractError::ExpectedBridgeSecurity(BridgeSecurity::Deterministic)
                })?;

            // First, add a callback to instantiate an interpreter (which we later get the result
            // and save it)
            let instantiate_msg: CosmosMsg = WasmMsg::Instantiate {
                // router is the default admin of a contract
                admin: Some(env.contract.address.clone().into_string()),
                code_id: config.interpreter_code_id,
                msg: to_binary(&cw_xcvm_interpreter::msg::InstantiateMsg {
                    gateway_address: config.gateway_address.into(),
                    registry_address: config.registry_address.into(),
                    router_address: env.contract.address.clone().into_string(),
                    interpreter_origin: interpreter_origin.clone(),
                })?,
                funds: vec![],
                label: format!(
                    "xcvm-interpreter-{}-{}-{}",
                    u32::from(interpreter_origin.user_origin.network_id),
                    hex::encode::<Vec<u8>>(interpreter_origin.user_origin.user_id.into()),
                    hex::encode(&interpreter_origin.salt)
                ),
            }
            .into();

            let interpreter_instantiate_submessage =
                SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_REPLY_ID);
            // Secondly, call itself again with the same parameters, so that this functions goes
            // into `Ok` state and properly executes the interpreter
            let self_call_message: CosmosMsg = wasm_execute(
                env.contract.address,
                &ExecuteMsg::ExecuteProgramPrivileged {
                    call_origin: call_origin.clone(),
                    salt: interpreter_origin.salt,
                    program,
                    assets,
                },
                vec![],
            )?
            .into();
            Ok(Response::new()
                .add_submessage(interpreter_instantiate_submessage)
                .add_message(self_call_message))
        }
    }
}

/// Transfer funds attached to a [`XCVMProgram`] before dispatching the program to the interpreter.
fn send_funds_to_interpreter(
    deps: Deps,
    interpreter_address: Addr,
    funds: Funds<Displayed<u128>>,
) -> StdResult<Response> {
    let mut response = Response::new();
    let registry_address = CONFIG.load(deps.storage)?.registry_address.into_string();
    let interpreter_address = interpreter_address.into_string();
    for (asset_id, Displayed(amount)) in funds.0 {
        // We ignore zero amounts
        if amount == 0 {
            continue;
        }

        let reference =
            external_query_lookup_asset(deps.querier, registry_address.clone(), asset_id)?;
        response = match reference {
            AssetReference::Native { denom } => response.add_message(BankMsg::Send {
                to_address: interpreter_address.clone(),
                amount: vec![Coin::new(amount, denom)],
            }),
            AssetReference::Virtual { cw20_address } => {
                let contract = Cw20Contract(cw20_address);
                response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
                    recipient: interpreter_address.clone(),
                    amount: amount.into(),
                })?)
            }
        };
    }
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("not implemented"))
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let response = msg.result.into_result().map_err(StdError::generic_err)?;
    let interpreter_address = {
        // Catch the default `instantiate` event which contains `_contract_address` attribute that
        // has the instantiated contract's address
        let instantiate_event = response
            .events
            .iter()
            .find(|event| event.ty == "instantiate")
            .ok_or(StdError::not_found("instantiate event not found"))?;
        deps.api.addr_validate(
            &instantiate_event
                .attributes
                .iter()
                .find(|attr| &attr.key == "_contract_address")
                .ok_or(StdError::not_found("_contract_address attribute not found"))?
                .value,
        )?
    };

    let interpreter_origin = {
        // Interpreter provides `network_id, user_id` pair as an event for the router to know which
        // pair is instantiated
        let interpreter_event = response
            .events
            .iter()
            .find(|event| {
                event
                    .ty
                    .starts_with(&format!("wasm-{}", XCVM_INTERPRETER_EVENT_PREFIX))
            })
            .ok_or(StdError::not_found("interpreter event not found"))?;

        decode_base64::<_, InterpreterOrigin>(
            interpreter_event
                .attributes
                .iter()
                .find(|attr| &attr.key == XCVM_INTERPRETER_EVENT_DATA_ORIGIN)
                .ok_or(StdError::not_found(
                    "no data is returned from 'xcvm_interpreter'",
                ))?
                .value
                .as_str(),
        )?
    };

    match INTERPRETERS.load(deps.storage, interpreter_origin.clone()) {
        Ok(Interpreter { security, .. }) => INTERPRETERS.save(
            deps.storage,
            interpreter_origin,
            &Interpreter {
                address: Some(interpreter_address),
                security,
            },
        )?,
        Err(_) => INTERPRETERS.save(
            deps.storage,
            interpreter_origin,
            &Interpreter {
                security: BridgeSecurity::Deterministic,
                address: Some(interpreter_address),
            },
        )?,
    }

    Ok(Response::new())
}
