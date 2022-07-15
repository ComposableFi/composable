use super::*;
use core::borrow::Borrow;
use ibc::{
	applications::transfer::MODULE_ID_STR as IBC_TRANSFER_MODULE_ID,
	core::ics26_routing::context::{Ics26Context, Module, ModuleId, ReaderContext, Router},
};
use scale_info::prelude::string::ToString;

#[derive(Clone)]
pub struct Context<T: Config> {
	pub _pd: PhantomData<T>,
	router: IbcRouter<T>,
}

impl<T: Config + Send + Sync> Default for Context<T> {
	fn default() -> Self {
		Self { _pd: PhantomData::default(), router: IbcRouter::default() }
	}
}

impl<T: Config + Send + Sync> Context<T> {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Clone)]
pub struct IbcRouter<T: Config> {
	pallet_ibc_ping: pallet_ibc_ping::IbcHandler<T>,
	ibc_transfer: transfer::IbcCallbackHandler<T>,
}

impl<T: Config> Default for IbcRouter<T> {
	fn default() -> Self {
		Self {
			pallet_ibc_ping: pallet_ibc_ping::IbcHandler::<T>::default(),
			ibc_transfer: transfer::IbcCallbackHandler::<T>::default(),
		}
	}
}

impl<T: Config + Send + Sync> Router for IbcRouter<T> {
	fn get_route_mut(&mut self, module_id: &impl Borrow<ModuleId>) -> Option<&mut dyn Module> {
		match module_id.borrow().to_string().as_str() {
			pallet_ibc_ping::MODULE_ID => Some(&mut self.pallet_ibc_ping),
			IBC_TRANSFER_MODULE_ID => Some(&mut self.ibc_transfer),
			&_ => None,
		}
	}

	fn has_route(&self, module_id: &impl Borrow<ModuleId>) -> bool {
		matches!(
			module_id.borrow().to_string().as_str(),
			pallet_ibc_ping::MODULE_ID | IBC_TRANSFER_MODULE_ID
		)
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

impl<T: Config + Send + Sync> ReaderContext for Context<T> where
	u32: From<<T as frame_system::Config>::BlockNumber>
{
}
