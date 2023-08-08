extern crate alloc;

use crate::{accounts, auth, error::Result, ibc, msg, state};

use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw_utils::ensure_from_older_version;

const CONTRACT_NAME: &str = "composable:xcvm-gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cosmwasm_std::entry_point]
pub fn instantiate(
	deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	msg: msg::InstantiateMsg,
) -> Result {
	set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	state::Config::try_instantiate(deps.api, &msg)?.save(deps.storage)?;
	for admin in msg.admins {
		auth::Admin::add(deps.storage, deps.api.addr_validate(&admin)?)?;
	}
	Ok(Response::default().add_event(msg::make_event(msg::Action::Instantiated)))
}

#[cosmwasm_std::entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> Result {
	match msg {
		msg::ExecuteMsg::CreateAccount(req) => {
			let auth = auth::User::authorise(deps.storage, &env)?;
			accounts::handle_create_account(auth, deps, info, req)
		},
		msg::ExecuteMsg::DropAccount(req) => {
			let auth = auth::Account::authorise(deps.storage, &env, info)?;
			accounts::handle_drop_account(auth, deps, req).map(Into::into)
		},
		msg::ExecuteMsg::ExecuteSolution(req) => {
			let auth = auth::Account::authorise(deps.storage, &env, info)?;
			accounts::handle_submit_problem(auth, deps, req).map(Into::into)
		},
		msg::ExecuteMsg::LocalPacket(packet) => {
			let auth = auth::EscrowContract::authorise_local(deps.storage, &env, info)?;
			let response = ibc::handle_packet(auth, deps, env, packet)?;
			Ok(response.into())
		},
		msg::ExecuteMsg::BreakGlass => {
			let auth = auth::Admin::authorise(deps.storage, info)?;
			auth::handle_break_glass(auth, deps, env)
		},
	}
}

#[cosmwasm_std::entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: msg::MigrateMsg) -> Result {
	let _ = ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	Ok(Response::default())
}

#[cosmwasm_std::entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: msg::QueryMsg) -> Result<Binary> {
	unreachable!()
}
