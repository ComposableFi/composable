use super::*;

use crate::routing::Context;
use ibc::core::{
	ics05_port::{
		capabilities::{Capability, CapabilityName, PortCapability},
		context::{CapabilityKeeper, CapabilityReader, PortKeeper, PortReader},
		error::Error as ICS05Error,
	},
	ics24_host::identifier::PortId,
	ics26_routing::context::ModuleId,
};
use scale_info::prelude::string::ToString;

impl<T: Config + Send + Sync> CapabilityReader for Context<T> {
	fn get_capability(&self, name: &CapabilityName) -> Result<Capability, ICS05Error> {
		let cap = Capabilities::<T>::get(name.to_string().as_bytes().to_vec())
			.ok_or(ICS05Error::implementation_specific())?;
		Ok(Capability::from(cap))
	}

	fn authenticate_capability(
		&self,
		name: &CapabilityName,
		capability: &Capability,
	) -> Result<(), ICS05Error> {
		let cap = Capabilities::<T>::get(name.to_string().as_bytes().to_vec())
			.ok_or(ICS05Error::implementation_specific())?;
		let cap = Capability::from(cap);
		if &cap == capability {
			return Ok(())
		}
		Err(ICS05Error::implementation_specific())
	}
}

impl<T: Config + Sync + Send> PortReader for Context<T> {
	fn lookup_module_by_port(
		&self,
		port_id: &PortId,
	) -> Result<(ModuleId, PortCapability), ICS05Error> {
		match port_id.as_str() {
			val if val == pallet_ibc_ping::PORT_ID => {
				let capability_name = Self::port_capability_name(port_id.clone());
				let capability_key = capability_name.to_string().as_bytes().to_vec();
				let capability = Capabilities::<T>::get(capability_key)
					.ok_or(ICS05Error::implementation_specific())?;
				let capability = Capability::from(capability);
				Ok((
					ModuleId::from_str(pallet_ibc_ping::MODULE_ID)
						.map_err(|_| ICS05Error::implementation_specific())?,
					capability.into(),
				))
			},
			_ => Err(ICS05Error::module_not_found(port_id.clone())),
		}
	}
}

impl<T: Config + Sync + Send> CapabilityKeeper for Context<T> {
	fn new_capability(&mut self, name: CapabilityName) -> Result<Capability, ICS05Error> {
		let capability_key = name.to_string().as_bytes().to_vec();
		if Capabilities::<T>::contains_key(capability_key) {
			return Err(ICS05Error::implementation_specific())
		}
		let count = Capabilities::<T>::count() as u64;
		self.claim_capability(name, Capability::from(count));
		Ok(count.into())
	}

	fn claim_capability(&mut self, name: CapabilityName, capability: Capability) {
		let capability_key = name.to_string().as_bytes().to_vec();
		let cap = capability.index();
		Capabilities::<T>::insert(capability_key, cap);
	}

	fn release_capability(&mut self, name: CapabilityName, _capability: Capability) {
		let capability_key = name.to_string().as_bytes().to_vec();
		Capabilities::<T>::remove(capability_key)
	}
}

impl<T: Config + Send + Sync> PortKeeper for Context<T> {}
