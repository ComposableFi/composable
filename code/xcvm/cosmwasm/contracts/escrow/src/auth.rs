//! Module with authorisation checks.

use crate::{
	error::{ContractError, Result},
	state,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdResult, Storage};
use cw_storage_plus::{Item, Map};

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
/// `P`.
///
/// Intended usage of this object is to have functions which require certain
/// authorisation level to take `Auth<P>` as an argument where `P` indicates
/// the authorisation level.  Then, caller has to use `Auth::<P>::authorise`
/// method to construct such object and be able to call the function.  The
/// `authorise` method will verify caller’s authorisation level.
///
/// For convenience, type aliases are provided for the different
/// authorisation levels: [`Contract`], [`Interpreter`] and [`Admin`].
#[derive(Clone, derive_more::Deref)]
pub(crate) struct Auth<P>(P);

/// Authorisation token for messages which come from a regular user.
///
/// This authorisation requires that the break glass feature has not been used.
/// If it has been, only admin actions can be performed.
pub(crate) type User = Auth<policy::User>;

/// Authorisation token for messages which come from contract’s admin.
pub(crate) type Admin = Auth<policy::Admin>;

/// Authorisation token for messages coming from a known CW20 contract.
///
/// The token allows accessing address of the CW20 contract as well as asset id
/// we’re using for the asset.
pub(crate) type Cw20Contract = Auth<policy::Cw20Contract>;

impl Auth<policy::User> {
	/// Checks whether the break glass feature hasn’t been used or has expired.
	///
	/// In the latter case, the status for break glass status is removed from
	/// the storage which is why this method takes `DepsMut` rather than `Deps`
	/// argument.
	pub(crate) fn authorise(storage: &mut dyn Storage, env: &Env) -> Result<Self> {
		let ok = BREAK_GLASS.may_load(storage)?.map_or(true, |state| {
			let expired = state.end_height.unwrap_or(0) < env.block.height &&
				state.end_time.map_or(true, |time| time < env.block.time);
			if expired {
				BREAK_GLASS.remove(storage);
			}
			expired
		});
		if ok {
			Ok(Self(Default::default()))
		} else {
			Err(ContractError::BrokenGlass)
		}
	}
}

impl Auth<policy::Admin> {
	/// Checks that the sender of the message is an admin.
	pub(crate) fn authorise(storage: &dyn Storage, info: &MessageInfo) -> Result<Self> {
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

impl Auth<policy::Cw20Contract> {
	/// Verifies that given address is address of a known CW20 contract.
	pub(crate) fn authorise(
		gateway: &xc_core::gateway::Gateway,
		querier: cosmwasm_std::QuerierWrapper,
		address: Addr,
	) -> Result<Self> {
		use xc_core::gateway::config::AssetReference;
		let reference = AssetReference::Cw20 { contract: address.clone() };
		let asset = gateway.get_local_asset_by_reference(querier, reference)?;
		let asset_id = asset.asset_id;
		let address = cw20::Cw20Contract(address);
		Ok(Self(policy::Cw20Contract { asset_id, address }))
	}
}

impl<P> Auth<P> {
	/// Constructs `Auth` object if user is `authorised`.
	///
	/// If `authorised` argument is true, returns `Ok(Self)` object; otherwise
	/// returns an error indicating that sender is not authorised.
	fn new(authorised: bool) -> Result<Self>
	where
		P: Default,
	{
		if authorised {
			Ok(Self(Default::default()))
		} else {
			Err(ContractError::NotAuthorized)
		}
	}

	pub fn into_inner(self) -> P {
		self.0
	}
}

pub(crate) fn handle_break_glass(_: Admin, deps: DepsMut, env: Env, _info: MessageInfo) -> Result {
	const DAY_IN_NS: u64 = 24 * 3600 * 1_000_000_000;
	// TODO(mina86): Allow break glass period to be configured.
	let end_time = Some(env.block.time.plus_nanos(DAY_IN_NS));
	let state = BrokenGlassState { end_time, end_height: None };
	BREAK_GLASS.save(deps.storage, &state)?;
	Ok(Response::default())
}

pub(crate) mod policy {
	#[derive(Clone, Default)]
	pub(crate) struct User;

	#[derive(Clone, Default)]
	pub(crate) struct Admin;

	#[derive(Clone)]
	pub(crate) struct Cw20Contract {
		pub asset_id: xc_core::AssetId,
		pub address: cw20::Cw20Contract,
	}
}
