/// Creates an event with contract’s default prefix and given action attribute.
pub(crate) fn make_event(action: &str) -> cosmwasm_std::Event {
	cosmwasm_std::Event::new(cw_xc_common::gateway::EVENT_PREFIX).add_attribute("action", action)
}

/// Module with authorisation checks.
pub(crate) mod auth {
	use crate::{
		error::{ContractError, ContractResult},
		state,
	};
	use cosmwasm_std::{Deps, Env, MessageInfo};

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

	impl Auth<policy::Contract> {
		pub(crate) fn authorise(env: &Env, info: &MessageInfo) -> ContractResult<Self> {
			Self::new(info.sender == env.contract.address)
		}
	}

	impl Auth<policy::Interpreter> {
		pub(crate) fn authorise(
			deps: Deps,
			info: &MessageInfo,
			interpreter_origin: xc_core::InterpreterOrigin,
		) -> ContractResult<Self> {
			let interpreter_address = state::INTERPRETERS
				.may_load(deps.storage, interpreter_origin)?
				.map(|int| int.address);
			Self::new(Some(&info.sender) == interpreter_address.as_ref())
		}
	}

	impl Auth<policy::Admin> {
		pub(crate) fn authorise(deps: Deps, info: &MessageInfo) -> ContractResult<Self> {
			Self::new(info.sender == state::Config::load(deps.storage)?.admin)
		}
	}

	impl<T> Auth<T> {
		fn new(authorised: bool) -> ContractResult<Self> {
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
	}
}
