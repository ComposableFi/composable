use cosmwasm_std::{CosmosMsg, Event, Response, SubMsg};

#[derive(Debug, Clone, Default)]
pub struct BatchResponse {
	pub messages: Vec<SubMsg>,
	pub events: Vec<Event>,
}

impl BatchResponse {
	pub fn new() -> Self {
		<_>::default()
	}
	pub fn add_message(mut self, msg: impl Into<CosmosMsg>) -> Self {
		self.messages.push(SubMsg::new(msg));
		self
	}

	pub fn add_submessage(mut self, msg: SubMsg) -> Self {
		self.messages.push(msg);
		self
	}

	pub fn add_event(mut self, event: Event) -> Self {
		self.events.push(event);
		self
	}

	pub fn merge(&mut self, mut other: Self) {
		self.messages.append(&mut other.messages);
		self.events.append(&mut other.events);
	}
}

impl From<BatchResponse> for Response {
	fn from(resp: BatchResponse) -> Self {
		let mut result = Self::new();
		result.messages = resp.messages;
		result.events = resp.events;
		result
	}
}
