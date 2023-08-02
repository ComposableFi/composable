mod assets;
mod auth;
pub mod contract;
mod deposits;
mod error;
mod ibc;
mod state;

mod msg {
	pub(crate) use xc_core::{accounts, escrow::*};

	/// Creates an event with contract’s default prefix and given action attribute.
	pub(crate) fn make_event(action: Action) -> cosmwasm_std::Event {
		cosmwasm_std::Event::new(xc_core::escrow::EVENT_PREFIX)
			.add_attribute("action", action.as_ref())
	}
}
