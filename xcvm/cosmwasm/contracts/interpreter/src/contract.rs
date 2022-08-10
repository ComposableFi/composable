use cosmwasm_std::{
    entry_point, to_binary, wasm_execute, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdError, WasmQuery,
};
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, XCVMProgram};
use crate::state::{Config, CONFIG};
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use xcvm_asset_registry::msg::{GetAssetContractResponse, QueryMsg as AssetRegistryQueryMsg};
use xcvm_core::{Amount, Funds, Instruction, NetworkID};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let registry_address = deps.api.addr_validate(&msg.registry_address)?;

    let config = Config { registry_address };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Execute { program } => interpret_program(deps, env, info, program),
    }
}

pub fn interpret_program(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    program: XCVMProgram,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    for instruction in program.instructions {
        response = match instruction {
            Instruction::Call { encoded } => interpret_call(encoded, response),
            Instruction::Spawn {
                network,
                salt,
                assets,
                program,
            } => interpret_spawn(network, salt, assets, program, response),
            Instruction::Transfer { to, assets } => {
                interpret_transfer(&mut deps, to, assets, response)
            }
        }?;
    }
    Ok(response)
}

pub fn interpret_call(encoded: Vec<u8>, response: Response) -> Result<Response, ContractError> {
    #[derive(Deserialize)]
    struct Payload {
        address: String,
        msg: String,
    }

    let payload: Payload =
        serde_json_wasm::from_slice(&encoded).map_err(|_| ContractError::InvalidCallPayload)?;
    let msg = wasm_execute(payload.address, &payload.msg, vec![])?;

    Ok(response.add_message(msg))
}

pub fn interpret_spawn(
    network: NetworkID,
    salt: Vec<u8>,
    assets: Funds,
    program: XCVMProgram,
    response: Response,
) -> Result<Response, ContractError> {
    #[derive(Serialize)]
    struct SpawnEvent {
        network: NetworkID,
        salt: Vec<u8>,
        assets: Funds,
        program: XCVMProgram,
    }

    let data = SpawnEvent {
        network,
        salt,
        assets,
        program,
    };

    Ok(response.add_attribute(
        "spawn",
        serde_json_wasm::to_string(&data).map_err(|_| ContractError::DataSerializationError)?,
    ))
}

pub fn interpret_transfer(
    deps: &mut DepsMut,
    to: String,
    assets: Funds,
    mut response: Response,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let registry_addr = config.registry_address.into_string();

    for (asset_id, amount) in assets.0 {
        let query_msg = AssetRegistryQueryMsg::GetAssetContract(asset_id.into());

        let cw20_address: GetAssetContractResponse = deps.querier.query(
            &WasmQuery::Smart {
                contract_addr: registry_addr.clone(),
                msg: to_binary(&query_msg)?,
            }
            .into(),
        )?;
        let contract = Cw20Contract(cw20_address.addr.clone());

        let transfer_amount = match amount {
            Amount::Fixed(ref fixed) => {
                if fixed.0 == 0 {
                    return Err(ContractError::ZeroTransferAmount);
                }
                amount.apply(0)
            }
            Amount::Ratio(ratio) => {
                if ratio == 0 {
                    return Err(ContractError::ZeroTransferAmount);
                }
                let query_msg = Cw20QueryMsg::Balance {
                    address: to.clone(),
                };
                let response: BalanceResponse =
                    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                        contract_addr: cw20_address.addr.clone().into_string(),
                        msg: to_binary(&query_msg)?,
                    }))?;
                amount.apply(response.balance.into())
            }
        };

        response = response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
            recipient: to.clone(),
            amount: transfer_amount.into(),
        })?);
    }

    Ok(response)
}
