use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, GetAssetContractResponse, InstantiateMsg, QueryMsg},
};

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
		ASSETS.save(deps.storage, asset_id.parse::<XcvmAssetId>().unwrap(), &addr)?;
	}

	Ok(Response::new().add_attribute("action", "update_assets"))
}

pub fn query_asset_contract(
	deps: Deps,
	token_id: XcvmAssetId,
) -> StdResult<GetAssetContractResponse> {
	let contract_addr = ASSETS.load(deps.storage, token_id)?;
	Ok(GetAssetContractResponse { addr: contract_addr })
}

#[cfg(test)]
mod tests {
	use super::*;
	use cosmwasm_std::{
		from_binary,
		testing::{mock_dependencies, mock_env, mock_info},
		Addr, Attribute, Order, Storage,
	};

	#[test]
	fn proper_instantiation() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {};
		let info = mock_info("sender", &vec![]);

		let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
		assert_eq!(0, res.messages.len());

		// Make sure that the storage is empty
		assert_eq!(deps.storage.range(None, None, Order::Ascending).next(), None);
	}

	#[test]
	fn set_assets() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {};
		let info = mock_info("sender", &vec![]);

		let _ = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

		let mut assets = BTreeMap::new();
		assets.insert("1".into(), "addr1".into());
		assets.insert("2".into(), "addr2".into());

		let res =
			execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::SetAssets(assets.clone()))
				.unwrap();
		assert!(res
			.attributes
			.iter()
			.find(|&attr| attr == Attribute::new("action", "update_assets"))
			.is_some());

		assert_eq!(ASSETS.load(&deps.storage, 1).unwrap(), Addr::unchecked("addr1"));
		assert_eq!(ASSETS.load(&deps.storage, 2).unwrap(), Addr::unchecked("addr2"));

		let mut assets = BTreeMap::new();
		assets.insert("3".into(), "addr3".into());
		assets.insert("4".into(), "addr4".into());

		let _ = execute(deps.as_mut(), mock_env(), info, ExecuteMsg::SetAssets(assets.clone()))
			.unwrap();

		// Make sure that set removes the previous elements
		assert!(ASSETS.load(&deps.storage, 1).is_err());
		assert!(ASSETS.load(&deps.storage, 2).is_err());
		assert_eq!(ASSETS.load(&deps.storage, 3).unwrap(), Addr::unchecked("addr3"));
		assert_eq!(ASSETS.load(&deps.storage, 4).unwrap(), Addr::unchecked("addr4"));

		// Finally make sure that there are two elements in the assets storage
		assert_eq!(
			ASSETS
				.keys(&deps.storage, None, None, Order::Ascending)
				.collect::<Vec<_>>()
				.len(),
			2
		);
	}

	#[test]
	fn query_assets() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {};
		let info = mock_info("sender", &vec![]);

		let _ = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

		let mut assets = BTreeMap::new();
		assets.insert("1".into(), "addr1".into());

		let _ =
			execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::SetAssets(assets.clone()))
				.unwrap();

		let res: GetAssetContractResponse =
			from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::GetAssetContract(1)).unwrap())
				.unwrap();

		// Query should return the corresponding address
		assert_eq!(res, GetAssetContractResponse { addr: Addr::unchecked("addr1") });

		// This should fail since there the asset doesn't exist
		assert!(query(deps.as_ref(), mock_env(), QueryMsg::GetAssetContract(2)).is_err());
	}
}
