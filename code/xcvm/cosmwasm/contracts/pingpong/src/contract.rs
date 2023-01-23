extern crate alloc;

use crate::{
	error::ContractError,
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{NETWORK, ROUTER},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
	wasm_execute, Binary, CanonicalAddr, Deps, DepsMut, Env, MessageInfo, Reply, Response,
	StdError, StdResult,
};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;
use cw_xcvm_utils::DefaultXCVMProgram;
use xcvm_core::{
	cosmwasm::{FlatCosmosMsg, FlatWasmMsg},
	Balance, BridgeSecurity, Funds, Juno, Network, Picasso, ProgramBuilder, UserId, UserOrigin,
};

const CONTRACT_NAME: &str = "composable:xcvm-pingpong";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const XCVM_PINGPONG_EVENT_PREFIX: &str = "xcvm.pingpong";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: InstantiateMsg,
) -> Result<Response, ContractError> {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	let router_address = deps.api.addr_validate(&msg.router_address)?;
	ROUTER.save(deps.storage, &router_address)?;
	NETWORK.save(deps.storage, &msg.network_id)?;
	Ok(Response::default())
}

fn make_program<T: Network<EncodedCall = Vec<u8>>, U: Network<EncodedCall = Vec<u8>>>(
	remote_address: UserId,
	msg: ExecuteMsg,
) -> Result<DefaultXCVMProgram, ContractError> {
	Ok(ProgramBuilder::<T, CanonicalAddr, Funds<Balance>>::new("PING".as_bytes().to_vec())
		.spawn::<_, U, (), _>(
			"PONG".as_bytes().to_vec(),
			vec![0x01, 0x02, 0x03],
			BridgeSecurity::Deterministic,
			Funds::<Balance>::empty(),
			|child| {
				Ok(child.call_raw(
					serde_json::to_vec(&FlatCosmosMsg::Wasm(FlatWasmMsg::<ExecuteMsg>::Execute {
						contract_addr: String::from_utf8_lossy(&Vec::<u8>::from(remote_address))
							.to_string(),
						msg,
						funds: Default::default(),
					}))
					.map_err(|_| ())?,
				))
			},
		)
		.map_err(|_| StdError::generic_err("invalid program"))?
		.build())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
	deps: DepsMut,
	env: Env,
	_info: MessageInfo,
	msg: ExecuteMsg,
) -> Result<Response, ContractError> {
	let network = NETWORK.load(deps.storage)?;
	let router = ROUTER.load(deps.storage)?;
	let make_program = |remote_address, msg| {
		if network == Picasso::ID {
			make_program::<Picasso, Juno>(remote_address, msg)
		} else {
			make_program::<Juno, Picasso>(remote_address, msg)
		}
	};
	let local_origin = UserOrigin {
		network_id: network,
		user_id: env.contract.address.as_bytes().to_vec().into(),
	};
	let (user_origin, message) = match msg {
		ExecuteMsg::Ping { user_origin, counter } =>
			(user_origin, ExecuteMsg::Pong { user_origin: local_origin, counter }),
		ExecuteMsg::Pong { user_origin, counter } =>
			(user_origin, ExecuteMsg::Ping { user_origin: local_origin, counter }),
	};
	Ok(Response::default().add_message(wasm_execute(
		router,
		&cw_xcvm_common::router::ExecuteMsg::ExecuteProgram {
			salt: vec![0x01, 0x02, 0x03],
			program: make_program(user_origin.user_id, message)?,
			assets: Funds::empty(),
		},
		Default::default(),
	)?))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
	Err(StdError::generic_err("unimplemented"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> StdResult<Response> {
	Ok(Response::default())
}
