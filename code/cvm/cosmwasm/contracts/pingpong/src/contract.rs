use crate::{
	error::{ContractError, Result},
	msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
	state::{GATEWAY, NETWORK},
};
use cosmwasm_std::{
	wasm_execute, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;
use xc_core::{
	cosmwasm::{FlatCosmosMsg, FlatWasmMsg},
	shared::XcAddr,
	Balance, Centauri, Funds, Network, Picasso, ProgramBuilder, UserId, UserOrigin,
};

const CONTRACT_NAME: &str = "composable:xcvm-pingpong";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const XCVM_PINGPONG_EVENT_PREFIX: &str = "cvm.pingpong";

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn instantiate(deps: DepsMut, _env: Env, _info: MessageInfo, msg: InstantiateMsg) -> Result {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	let gateway_address = deps.api.addr_validate(&msg.gateway_address)?;
	GATEWAY.save(deps.storage, &gateway_address)?;
	NETWORK.save(deps.storage, &msg.network_id)?;
	Ok(Response::default())
}

fn make_program<T: Network<EncodedCall = Vec<u8>>, U: Network<EncodedCall = Vec<u8>>>(
	remote_address: UserId,
	msg: ExecuteMsg,
) -> Result<xc_core::shared::XcProgram, ContractError> {
	Ok(ProgramBuilder::<T, XcAddr, Funds<Balance>>::new("PING".as_bytes().to_vec())
		.spawn::<U, (), _, _>(
			"PONG".as_bytes().to_vec(),
			vec![0x01, 0x02, 0x03],
			Funds::<Balance>::default(),
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

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result {
	let network = NETWORK.load(deps.storage)?;
	let gateway = GATEWAY.load(deps.storage)?;
	let make_program = |remote_address, msg| {
		if network == Picasso::ID {
			make_program::<Picasso, Centauri>(remote_address, msg)
		} else {
			make_program::<Centauri, Picasso>(remote_address, msg)
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
	let execute_program = xc_core::gateway::ExecuteProgramMsg {
		salt: vec![0x01, 0x02, 0x03],
		program: make_program(user_origin.user_id, message)?,
		assets: Funds::default(),
	};
	Ok(Response::default().add_message(wasm_execute(
		gateway,
		&xc_core::gateway::ExecuteMsg::ExecuteProgram { execute_program, tip: info.sender.into() },
		Default::default(),
	)?))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
	Err(StdError::generic_err("unimplemented"))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> StdResult<Response> {
	Ok(Response::default())
}
