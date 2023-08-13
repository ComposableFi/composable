//! Module with authorisation checks.
use crate::{
	error::{ContractError, Result},
	msg, state,
};
use cosmwasm_std::{Deps, Env, MessageInfo, Storage};
use xc_core::{gateway::OtherNetworkItem, NetworkId};

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
		let this = state::load(storage)?;
		let this_to_other: OtherNetworkItem = state::NETWORK_TO_NETWORK
			.load(storage, (this.here_id, network_id))
			.map_err(|_| ContractError::NoConnectionInformationFromThisToOtherNetwork(
				this.here_id,
				network_id,
			))?;
		let sender = state::NETWORK
			.load(storage, network_id)?
			.gateway
			.ok_or(ContractError::GatewayForNetworkNotFound(network_id))?;

		let sender = match sender {
			msg::GatewayId::CosmWasm { contract, .. } => contract.to_string(),
		};

		let channel = this_to_other.ics_20.ok_or(ContractError::ICS20NotFound)?.source;
		let hash_of_channel_and_sender =
			xc_core::transport::ibc::ics20::hook::derive_intermediate_sender(
				&channel, &sender, "",
			)?;
		Self::new(hash_of_channel_and_sender == info.sender && info.sender == env.contract.address)
	}
}

impl Auth<policy::Interpreter> {
	pub(crate) fn authorise(
		deps: Deps,
		info: &MessageInfo,
		interpreter_origin: xc_core::InterpreterOrigin,
	) -> Result<Self> {
		let interpreter_address = state::interpreter::get_by_origin(deps, interpreter_origin)
			.map(|int| int.address)
			.ok();
		Self::new(Some(&info.sender) == interpreter_address.as_ref())
	}
}

impl Auth<policy::Admin> {
	pub(crate) fn authorise(deps: Deps, info: &MessageInfo) -> Result<Self> {
		let this = state::load(deps.storage)?;
		Self::new(info.sender == this.admin)
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
