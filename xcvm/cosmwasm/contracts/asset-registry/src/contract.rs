use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetAssetContractResponse, InstantiateMsg, QueryMsg};

use crate::state::{XcvmAssetId, ASSETS};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetAssets(asset) => handle_set_assets(deps, asset),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAssetContract(token_id) => to_binary(&query_asset_contract(deps, token_id)?),
    }
}

pub fn handle_set_assets(
    deps: DepsMut,
    assets: BTreeMap<String, String>,
) -> Result<Response, ContractError> {
    // Remove all keys
    for key in ASSETS
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<Result<Vec<_>, _>>()?
    {
        ASSETS.remove(deps.storage, key);
    }

    for (asset_id, contract_addr) in assets {
        let addr = deps.api.addr_validate(&contract_addr)?;
        ASSETS.save(deps.storage, asset_id.parse::<u32>().unwrap(), &addr)?;
    }

    Ok(Response::new().add_attribute("action", "update_assets"))
}

pub fn query_asset_contract(
    deps: Deps,
    token_id: XcvmAssetId,
) -> StdResult<GetAssetContractResponse> {
    let contract_addr = ASSETS.load(deps.storage, token_id)?;
    Ok(GetAssetContractResponse {
        addr: contract_addr,
    })
}
