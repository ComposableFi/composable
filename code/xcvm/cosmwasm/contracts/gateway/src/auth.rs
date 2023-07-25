//! Module with authorisation checks.
use crate::{
	error::{ContractError, Result},
	msg, state,
};
use cosmwasm_std::{Deps, Env, MessageInfo, Storage};
use xc_core::NetworkId;

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
pub(crate) struct Auth<T>(core::marker::PhantomData<T>);

/// Authorisation token for messages which can only be sent from the
/// contract itself.
pub(crate) type Contract = Auth<policy::Contract>;

/// Authorisation token for messages which come from an interpreter.
pub(crate) type Interpreter = Auth<policy::Interpreter>;

/// Authorisation token for messages which come from contract’s admin.
pub(crate) type Admin = Auth<policy::Admin>;

pub(crate) type WasmHook = Auth<policy::WasmHook>;

impl Auth<policy::Contract> {
	pub(crate) fn authorise(env: &Env, info: &MessageInfo) -> Result<Self> {
		Self::new(info.sender == env.contract.address)
	}
}

impl Auth<policy::WasmHook> {
	pub(crate) fn authorise(
		storage: &dyn Storage,
		env: &Env,
		info: &MessageInfo,
		network_id: NetworkId,
	) -> Result<Self> {
		let this = state::Config::load(storage)?;
		let channel = state::NETWORK_TO_NETWORK.load(storage, (this.network_id, network_id))?;
		let sender = state::NETWORK
			.load(storage, network_id)?
			.gateway_to_send_to
			.ok_or(ContractError::NotAuthorized)?;
		let sender = match sender {
			msg::GatewayId::CosmWasm(addr) => addr.to_string(),
		};
		let hash_of_channel_and_sender =
			xc_core::ibc::hook::derive_intermediate_sender(&channel.ics_20_channel, &sender, "")?;
		Self::new(hash_of_channel_and_sender == info.sender && info.sender == env.contract.address)
	}
}

impl Auth<policy::Interpreter> {
	pub(crate) fn authorise(
		deps: Deps,
		info: &MessageInfo,
		interpreter_origin: xc_core::InterpreterOrigin,
	) -> Result<Self> {
		let interpreter_address = state::INTERPRETERS
			.may_load(deps.storage, interpreter_origin)?
			.map(|int| int.address);
		Self::new(Some(&info.sender) == interpreter_address.as_ref())
	}
}

impl Auth<policy::Admin> {
	pub(crate) fn authorise(deps: Deps, info: &MessageInfo) -> Result<Self> {
		Self::new(info.sender == state::Config::load(deps.storage)?.admin)
	}
}

impl<T> Auth<T> {
	fn new(authorised: bool) -> Result<Self> {
		if authorised {
			Ok(Self(Default::default()))
		} else {
			Err(ContractError::NotAuthorized)
		}
	}
}

pub(crate) mod policy {
	pub(crate) enum Contract {}
	pub(crate) enum Interpreter {}
	pub(crate) enum Admin {}
	pub(crate) enum WasmHook {}
}
