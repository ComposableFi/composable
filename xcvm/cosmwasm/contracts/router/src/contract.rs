use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg},
	state::{Config, UserId, CONFIG, INTERPRETERS, INTERPRETER_CODE_ID},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
	SubMsg, WasmMsg,
};
use xcvm_core::NetworkId;
use xcvm_interpreter::msg::InstantiateMsg as InterpreterInstantiateMsg;

const INSTANTIATE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	let addr = deps.api.addr_validate(&msg.registry_address)?;
	CONFIG.save(deps.storage, &Config { registry_address: addr })?;
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
		ExecuteMsg::NewInterpreter(network_id, user_id) =>
			handle_new_interpreter(deps, network_id, user_id),
	}
}

pub fn handle_new_interpreter(
	deps: DepsMut,
	network_id: NetworkId,
	user_id: UserId,
) -> Result<Response, ContractError> {
	match INTERPRETERS.load(deps.storage, (network_id.0, user_id.clone())) {
		Ok(interpreter_addr) =>
			Ok(Response::new().add_attribute("interpreter", &interpreter_addr.into_string())),
		Err(_) => {
			// TODO(aeryz): set admin as self
			let code_id = INTERPRETER_CODE_ID.load(deps.storage)?;
			let registry_address = CONFIG.load(deps.storage)?.registry_address.into_string();

			let instantiate_msg = WasmMsg::Instantiate {
				admin: None, // TODO(aeryz): should router be admin?
				code_id,
				msg: to_binary(&InterpreterInstantiateMsg { registry_address })?,
				funds: vec![],
				label: "todo".to_string(), // TODO(aeryz): juno doesn't allow empty label
			};

			// Creating a submessage that wraps the message above
			let submessage = SubMsg::reply_on_success(instantiate_msg.into(), INSTANTIATE_REPLY_ID);

			// Creating a response with the submessage
			let response = Response::new().add_submessage(submessage);
			Ok(Response::new().add_submessage(submessage))
		},
	}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
	match msg.id {
		INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
		id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
	}
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
	deps.api.debug(&format!("{:?}", msg));
	// Save res.contract_address
	Ok(Response::new())
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
