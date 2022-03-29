use super::*;

use crate::routing::Context;
use ibc::core::{
	ics05_port::{
		capabilities::{Capability, CapabilityName},
		context::{CapabilityKeeper, CapabilityReader, PortKeeper, PortReader},
		error::Error as ICS05Error,
	},
	ics24_host::identifier::PortId,
};
use scale_info::prelude::string::ToString;

impl<T: Config> CapabilityReader for Context<T> {
	fn get_capability(&self, name: &CapabilityName) -> Result<Capability, ICS05Error> {
		let cap = Capabilities::<T>::get(name.to_string().as_bytes().to_vec());
		u64::decode(&mut &*cap)
			.map(|cap| cap.into())
			.map_err(|_| ICS05Error::implementation_specific())
	}

	fn authenticate_capability(
		&self,
		name: &CapabilityName,
		capability: &Capability,
	) -> Result<(), ICS05Error> {
		let cap = Capabilities::<T>::get(name.to_string().as_bytes().to_vec());
		let cap: Capability = u64::decode(&mut &*cap)
			.map(|cap| cap.into())
			.map_err(|_| ICS05Error::implementation_specific())?;
		if &cap == capability {
			return Ok(())
		}
		Err(ICS05Error::implementation_specific())
	}
}

impl<T: Config> PortReader for Context<T> {
	type ModuleId = ();

	// TODO: Revisit when port binding and routing is implemented
	fn lookup_module_by_port(
		&self,
		_port_id: &PortId,
	) -> Result<(Self::ModuleId, Capability), ICS05Error> {
		log::trace!("in port: [look_module_by_port]");

		Ok(((), Capability::default()))
	}
}

impl<T: Config> CapabilityKeeper for Context<T> {
	fn new_capability(&mut self, name: CapabilityName) -> Result<Capability, ICS05Error> {
		let capability_key = name.to_string().as_bytes().to_vec();
		if Capabilities::<T>::contains_key(capability_key) {
			return Err(ICS05Error::implementation_specific())
		}
		let count = Capabilities::<T>::count() as u64;
		Ok(count.into())
	}

	fn claim_capability(&mut self, name: CapabilityName, capability: Capability) {
		let capability_key = name.to_string().as_bytes().to_vec();
		let cap = capability.index().encode();
		Capabilities::<T>::insert(capability_key, cap);
	}

	fn release_capability(&mut self, name: CapabilityName, _capability: Capability) {
		let capability_key = name.to_string().as_bytes().to_vec();
		Capabilities::<T>::remove(capability_key)
	}
}

impl<T: Config> PortKeeper for Context<T> {}
