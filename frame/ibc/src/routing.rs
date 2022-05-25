use super::*;
use core::borrow::Borrow;
use ibc::{
	applications::ics20_fungible_token_transfer::context::Ics20Context,
	core::ics26_routing::context::{Ics26Context, LightClientContext, Module, ModuleId, Router},
};
use scale_info::prelude::string::ToString;

#[derive(Clone)]
pub struct Context<T: Config> {
	pub _pd: PhantomData<T>,
	router: IbcRouter<T>,
}

impl<T: Config + Send + Sync> Context<T> {
	pub fn new() -> Self {
		Self { _pd: PhantomData::default(), router: IbcRouter::new() }
	}
}

#[derive(Clone)]
pub struct IbcRouter<T: Config> {
	pallet_ibc_ping: pallet_ibc_ping::IbcHandler<T>,
}

impl<T: Config + Send + Sync> IbcRouter<T> {
	fn new() -> Self {
		Self { pallet_ibc_ping: pallet_ibc_ping::IbcHandler::<T>::new() }
	}
}

impl<T: Config + Send + Sync> Router for IbcRouter<T> {
	fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module> {
		match module_id.borrow().to_string().as_str() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.pallet_ibc_ping),
			&_ => None,
		}
	}

	fn has_route(&self, module_id: &impl Borrow<ModuleId>) -> bool {
		match module_id.borrow().to_string().as_str() {
			pallet_ibc_ping::MODULE_ID => true,
			&_ => false,
		}
	}
}

impl<T: Config + Send + Sync> Ics26Context for Context<T>
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

impl<T: Config + Send + Sync> Ics20Context for Context<T> where
	u32: From<<T as frame_system::Config>::BlockNumber>
{
}

impl<T: Config + Send + Sync> LightClientContext for Context<T> where
	u32: From<<T as frame_system::Config>::BlockNumber>
{
}
