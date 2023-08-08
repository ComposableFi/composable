//! Module with authorisation checks.

use crate::{
	accounts,
	error::{ContractError, Result},
	ibc, state,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, Env, IbcEndpoint, MessageInfo, Response, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use xc_core::NetworkId;

/// Describes state of the broken glass.
///
/// If break glass feature has been used, describes when the broken glass state
/// stops.  The end time can be denoted via a timestamp or block height when the
/// glass fixes itself.  For the broken glass state to finish, both conditions
/// need to be met.
#[cw_serde]
struct BrokenGlassState {
	end_time: Option<cosmwasm_std::Timestamp>,
	end_height: Option<u64>,
}

/// List of admins.
const ADMINS: Map<Addr, u8> = Map::new(state::ADMINS_NS);

/// The current glass state.
///
/// Can be broken by an admin for a specified amount of time (see
/// [`handle_break_glass`]).  When glass is broken, contract stops accepting
/// requests from users.
const BREAK_GLASS: Item<BrokenGlassState> = Item::new(state::BREAK_GLASS_NS);

/// Authorisation token indicating call is authorised according to policy
/// `T`.
///
/// Intended usage of this object is to have functions which require certain
/// authorisation level to take `Auth<T>` as an argument where `T` indicates
/// the authorisation level.  Then, caller has to use `Auth::<T>::authorise`
/// method to construct such object and be able to call the function.  The
/// `authorise` method will verify caller’s authorisation level.
///
/// For convenience, type aliases are provided for the different
/// authorisation levels: [`Contract`], [`Interpreter`] and [`Admin`].
#[derive(Clone, derive_more::Deref)]
pub(crate) struct Auth<T>(T);

/// Authorisation token for messages which come from a regular user.
///
/// This authorisation requires that the break glass feature has not been used.
/// If it has been, only admin actions can be performed.
pub(crate) type User = Auth<policy::User>;

/// Authorisation token for messages which come from a holder of an account.
///
/// This authorisation requires that the break glass feature has not been used.
/// If it has been, only admin actions can be performed.
pub(crate) type Account = Auth<policy::Account>;

/// Authorisation token for messages send by an escrow account.
pub(crate) type EscrowContract = Auth<policy::EscrowContract>;

/// Authorisation token for messages which come from contract’s admin.
pub(crate) type Admin = Auth<policy::Admin>;

impl Auth<policy::User> {
	/// Checks whether the break glass feature hasn’t been used or has expired.
	///
	/// In the latter case, the status for break glass status is removed from
	/// the storage which is why this method takes `DepsMut` rather than `Deps`
	/// argument.
	pub(crate) fn authorise(storage: &mut dyn Storage, env: &Env) -> Result<Self> {
		check_break_glass(storage, env)?;
		Ok(Self(Default::default()))
	}
}

impl Auth<policy::Account> {
	/// Verifies that message’s sender holds a virtual wallet account.
	pub fn authorise(storage: &mut dyn Storage, env: &Env, info: MessageInfo) -> Result<Self> {
		check_break_glass(storage, env)?;
		match accounts::Account::load(storage, info.sender)? {
			Some(account) => Ok(Self(policy::Account(account))),
			None => Err(ContractError::UnknownAccount),
		}
	}

	/// Verifies that given remote address is a recovery address for the
	/// account.
	pub fn authorise_remote(
		auth: &EscrowContract,
		deps: &mut DepsMut,
		env: &Env,
		account: String,
		address: String,
	) -> Result<Self> {
		check_break_glass(deps.storage, env)?;
		let account = deps.api.addr_validate(&account)?;
		let account =
			accounts::Account::load(deps.storage, account)?.ok_or(ContractError::NotAuthorized)?;
		if account.has_recovery_address(deps.storage, auth.network_id, address) {
			Ok(Self(policy::Account(account)))
		} else {
			Err(ContractError::NotAuthorized)
		}
	}

	/// Returns [`accounts::Account`] object of the authorised account.
	pub fn account(&self) -> &accounts::Account {
		&self.0 .0
	}
}

impl Auth<policy::EscrowContract> {
	/// Verifies that sender of an IBC message is a remote escrow contract.
	pub fn authorise_ibc(
		storage: &mut dyn Storage,
		env: &Env,
		endpoint: IbcEndpoint,
	) -> Result<Self> {
		check_break_glass(storage, env)?;
		ibc::get_network_id_for_channel(storage, endpoint.channel_id)
			.and_then(|id| id.ok_or(ContractError::NotAuthorized))
			.map(|network_id| Self(policy::EscrowContract { network_id }))
	}

	/// Verifies that message’s sender is local escrow contract.
	pub fn authorise_local(
		storage: &mut dyn Storage,
		env: &Env,
		info: MessageInfo,
	) -> Result<Self> {
		check_break_glass(storage, env)?;
		let config = state::Config::load(storage)?;
		if config.local_escrow == Some(info.sender) {
			Ok(Self(policy::EscrowContract { network_id: config.network_id }))
		} else {
			Err(ContractError::NotAuthorized)
		}
	}
}

impl Auth<policy::Admin> {
	/// Verifies that the message has been sent by an admin.
	pub(crate) fn authorise(storage: &dyn Storage, info: MessageInfo) -> Result<Self> {
		Self::new(ADMINS.has(storage, info.sender.clone()))
	}
}

impl Admin {
	/// Adds a new admin.
	///
	/// The operation is idempotent, i.e. adding already existing admin does
	/// nothing.
	pub(crate) fn add(storage: &mut dyn Storage, address: Addr) -> StdResult<()> {
		ADMINS.save(storage, address, &1u8)
	}

	/// Removes an existing admin.
	///
	/// The operation is idempotent, i.e. removing a non-existing admin does
	/// nothing.
	// TODO(mina86): Currently unused.  Need to figure out interfaces for
	// managing admins.
	#[allow(dead_code)]
	pub(crate) fn remove(storage: &mut dyn Storage, address: Addr) {
		ADMINS.remove(storage, address)
	}
}

impl<T: Default> Auth<T> {
	/// Constructs `Auth` object if user is `authorised`.
	///
	/// If `authorised` argument is true, returns `Ok(Self)` object; otherwise
	/// returns an error indicating that sender is not authorised.
	fn new(authorised: bool) -> Result<Self> {
		if authorised {
			Ok(Self(Default::default()))
		} else {
			Err(ContractError::NotAuthorized)
		}
	}
}

/// Handles the break-glass request from an admin.
pub(crate) fn handle_break_glass(_: Admin, deps: DepsMut, env: Env) -> Result {
	const DAY_IN_NS: u64 = 24 * 3600 * 1_000_000_000;
	// TODO(mina86): Allow break glass period to be configured.
	let end_time = Some(env.block.time.plus_nanos(DAY_IN_NS));
	let state = BrokenGlassState { end_time, end_height: None };
	BREAK_GLASS.save(deps.storage, &state)?;
	Ok(Response::default())
}

/// Makes sure that the contract is not in broken-glass state.
///
/// If the glass has been broken, returns an error.
fn check_break_glass(storage: &mut dyn Storage, env: &Env) -> Result<()> {
	let ok = BREAK_GLASS.may_load(storage)?.map_or(true, |state| {
		let expired = state.end_height.unwrap_or(0) < env.block.height &&
			state.end_time.map_or(true, |time| time < env.block.time);
		if expired {
			BREAK_GLASS.remove(storage);
		}
		expired
	});
	if ok {
		Ok(())
	} else {
		Err(ContractError::BrokenGlass)
	}
}

pub(crate) mod policy {
	use super::*;

	#[derive(Default)]
	pub(crate) struct User {}
	pub(crate) struct Account(pub accounts::Account);
	pub(crate) struct EscrowContract {
		pub network_id: NetworkId,
	}
	#[derive(Default)]
	pub(crate) struct Admin {}
}
