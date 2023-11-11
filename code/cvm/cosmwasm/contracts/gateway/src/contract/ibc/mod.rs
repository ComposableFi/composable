use crate::events::make_event;

pub mod ics20;
pub mod ics27;

pub fn make_ibc_failure_event(reason: String) -> cosmwasm_std::Event {
	make_event("receive")
		.add_attribute("result", "failure")
		.add_attribute("reason", reason)
}
