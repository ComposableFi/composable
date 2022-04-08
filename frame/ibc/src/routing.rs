use super::*;
use core::borrow::Borrow;
use ibc::{
	applications::ics20_fungible_token_transfer::context::Ics20Context,
	core::ics26_routing::context::{Ics26Context, Module, ModuleId, Router},
};

#[derive(Clone)]
pub struct Context<T: Config> {
	pub _pd: PhantomData<T>,
	router: IbcRouter<T>,
}

impl<T: Config> Context<T> {
	pub fn new() -> Self {
		Self { _pd: PhantomData::default(), router: IbcRouter(PhantomData::default()) }
	}
}

#[derive(Clone)]
pub struct IbcRouter<T: Config>(PhantomData<T>);

impl<T: Config> Router for IbcRouter<T> {
	fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module> {
		todo!()
	}

	fn has_route(&self, module_id: &impl Borrow<ModuleId>) -> bool {
		todo!()
	}
}

impl<T: Config> Ics26Context for Context<T>
where
	u32: From<<T as frame_system::Config>::BlockNumber>,
{
	type Router = IbcRouter<T>;

	fn router(&self) -> &Self::Router {
		&self.router
	}

	fn router_mut(&mut self) -> &mut Self::Router {
		&mut self.router
	}
}

impl<T: Config> Ics20Context for Context<T> where u32: From<<T as frame_system::Config>::BlockNumber>
{}
