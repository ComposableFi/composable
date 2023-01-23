use crate::{
	error::ContractError,
	msg::{
		AssetKey, AssetReference, ExecuteMsg, InstantiateMsg, LookupResponse, MigrateMsg, QueryMsg,
	},
	state::ASSETS,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, QuerierWrapper, Response, StdResult,
	WasmQuery,
};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;

const CONTRACT_NAME: &str = "composable:xcvm-asset-registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const XCVM_ASSET_REGISTRY_EVENT_PREFIX: &str = "xcvm.registry";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default().add_event(
		Event::new(XCVM_ASSET_REGISTRY_EVENT_PREFIX).add_attribute("action", "instantiated"),
	))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	match msg {
		ExecuteMsg::RegisterAsset { asset_id, reference } =>
			handle_register_asset(deps, asset_id, reference),
		ExecuteMsg::UnregisterAsset { asset_id } => handle_unregister_asset(deps, asset_id),
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
		QueryMsg::Lookup { asset_id } => to_binary(&query_lookup(deps, asset_id)?),
	}
}

pub fn handle_register_asset(
	deps: DepsMut,
	asset_id: AssetKey,
	reference: AssetReference,
) -> Result<Response, ContractError> {
	ASSETS.save(deps.storage, asset_id, &reference)?;
	Ok(Response::new().add_event(
		Event::new(XCVM_ASSET_REGISTRY_EVENT_PREFIX)
			.add_attribute("action", "register")
			.add_attribute("asset_id", format!("{}", asset_id.0 .0 .0))
			.add_attribute("denom", reference.denom()),
	))
}

pub fn handle_unregister_asset(
	deps: DepsMut,
	asset_id: AssetKey,
) -> Result<Response, ContractError> {
	ASSETS.remove(deps.storage, asset_id);
	Ok(Response::new().add_event(
		Event::new(XCVM_ASSET_REGISTRY_EVENT_PREFIX)
			.add_attribute("action", "unregister")
			.add_attribute("asset_id", format!("{}", asset_id.0 .0 .0)),
	))
}

pub fn query_lookup(deps: Deps, asset_id: AssetKey) -> StdResult<LookupResponse> {
	let reference = ASSETS.load(deps.storage, asset_id)?;
	Ok(LookupResponse { reference })
}

pub fn external_query_lookup_asset(
	querier: QuerierWrapper,
	registry_addr: String,
	asset_id: impl Into<AssetKey>,
) -> StdResult<AssetReference> {
	querier
		.query::<LookupResponse>(
			&WasmQuery::Smart {
				contract_addr: registry_addr,
				msg: to_binary(&QueryMsg::Lookup { asset_id: asset_id.into() })?,
			}
			.into(),
		)
		.map(|response| response.reference)
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
	fn register_unregister_assets() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {};
		let info = mock_info("sender", &vec![]);

		let _ = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

		let addr1 = AssetReference::Virtual { cw20_address: Addr::unchecked("addr1") };
		let addr2 = AssetReference::Virtual { cw20_address: Addr::unchecked("addr2") };
		let addr3 = AssetReference::Virtual { cw20_address: Addr::unchecked("addr3") };
		let addr4 = AssetReference::Virtual { cw20_address: Addr::unchecked("addr4") };

		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::RegisterAsset { asset_id: 1.into(), reference: addr1.clone() },
		)
		.unwrap();

		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::RegisterAsset { asset_id: 2.into(), reference: addr2.clone() },
		)
		.unwrap();

		assert_eq!(ASSETS.load(&deps.storage, 1.into()).unwrap(), addr1);
		assert_eq!(ASSETS.load(&deps.storage, 2.into()).unwrap(), addr2);

		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::UnregisterAsset { asset_id: 1.into() },
		)
		.unwrap();

		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::UnregisterAsset { asset_id: 2.into() },
		)
		.unwrap();

		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::RegisterAsset { asset_id: 3.into(), reference: addr3.clone() },
		)
		.unwrap();

		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::RegisterAsset { asset_id: 4.into(), reference: addr4.clone() },
		)
		.unwrap();

		// Make sure that set removes the previous elements
		assert!(ASSETS.load(&deps.storage, 1.into()).is_err());
		assert!(ASSETS.load(&deps.storage, 2.into()).is_err());
		assert_eq!(ASSETS.load(&deps.storage, 3.into()).unwrap(), addr3);
		assert_eq!(ASSETS.load(&deps.storage, 4.into()).unwrap(), addr4);

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

		let addr1 = AssetReference::Virtual { cw20_address: Addr::unchecked("addr1") };
		let asset_id = AssetKey::from(1);
		let _ = execute(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			ExecuteMsg::RegisterAsset { asset_id, reference: addr1.clone() },
		)
		.unwrap();

		let res: LookupResponse =
			from_binary(&query(deps.as_ref(), mock_env(), QueryMsg::Lookup { asset_id }).unwrap())
				.unwrap();

		// Query should return the corresponding address
		assert_eq!(res, LookupResponse { reference: addr1 });

		// This should fail since there the asset doesn't exist
		assert!(query(deps.as_ref(), mock_env(), QueryMsg::Lookup { asset_id: AssetKey::from(2) })
			.is_err());
	}
}
