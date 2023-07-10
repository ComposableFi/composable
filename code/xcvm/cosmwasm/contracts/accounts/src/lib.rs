mod accounts;
mod auth;
pub mod contract;
mod error;
mod ibc;
mod state;

mod msg {
	pub(crate) use cw_xc_common::accounts::*;

	/// Creates an event with contract’s default prefix and given action attribute.
	pub(crate) fn make_event(action: Action) -> cosmwasm_std::Event {
		cosmwasm_std::Event::new(cw_xc_common::escrow::EVENT_PREFIX)
			.add_attribute("action", action.as_ref())
	}
}
