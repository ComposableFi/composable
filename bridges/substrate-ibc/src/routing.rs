use super::*;
use ibc::{
	applications::ics20_fungible_token_transfer::context::Ics20Context,
	core::ics26_routing::context::Ics26Context,
};

#[derive(Clone)]
pub struct Context<T: Config> {
	pub _pd: PhantomData<T>,
	pub tmp: u8,
}

impl<T: Config> Context<T> {
	pub fn new() -> Self {
		Self { _pd: PhantomData::default(), tmp: 0 }
	}
}

impl<T: Config> Ics26Context for Context<T> {}

impl<T: Config> Ics20Context for Context<T> {}
