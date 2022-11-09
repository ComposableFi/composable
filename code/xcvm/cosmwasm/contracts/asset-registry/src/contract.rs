use crate::{
	error::ContractError,
	msg::{
		AssetReference, ExecuteMsg, GetAssetContractResponse, InstantiateMsg, MigrateMsg, QueryMsg,
	},
	state::{XcvmAssetId, ASSETS},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;
use std::collections::BTreeMap;

const CONTRACT_NAME: &str = "composable:xcvm-asset-registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default().add_event(Event::new("xcvm.registry.instantiated")))
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
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
	match msg {
		QueryMsg::GetAssetContract(asset_id) => to_binary(&query_asset_contract(deps, asset_id)?),
	}
}

pub fn handle_set_assets(
	deps: DepsMut,
	assets: BTreeMap<String, AssetReference>,
) -> Result<Response, ContractError> {
	// Remove all keys
	for key in ASSETS
		.keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
		.collect::<Result<Vec<_>, _>>()?
	{
		ASSETS.remove(deps.storage, key);
	}

	for (asset_id, asset_reference) in assets {
		ASSETS.save(
			deps.storage,
			asset_id.parse::<XcvmAssetId>().map_err(|_| ContractError::CannotParseAssetId)?,
			&asset_reference,
		)?;
	}

	Ok(Response::new().add_event(Event::new("xcvm.registry.updated")))
}

pub fn query_asset_contract(
	deps: Deps,
	asset_id: XcvmAssetId,
) -> StdResult<GetAssetContractResponse> {
	let asset_reference = ASSETS.load(deps.storage, asset_id)?;
	Ok(GetAssetContractResponse { asset_reference })
}

#[cfg(test)]
mod tests {
	use super::*;
	use cosmwasm_std::{
		from_binary,
		testing::{mock_dependencies, mock_env, mock_info},
		Addr, Order,
	};

	#[test]
	fn proper_instantiation() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {};
		let info = mock_info("sender", &vec![]);

		let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
		assert_eq!(0, res.messages.len());
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
		assert_eq!(res.attributes.len(), 0);

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
