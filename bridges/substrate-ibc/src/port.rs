use super::*;

use crate::routing::Context;
use ibc::core::{
	ics05_port::{
		capabilities::{Capability, CapabilityName},
		context::{CapabilityReader, PortReader},
		error::Error as ICS05Error,
	},
	ics24_host::identifier::PortId,
};

impl<T: Config> CapabilityReader for Context<T> {
	fn get_capability(&self, _name: &CapabilityName) -> Result<Capability, ICS05Error> {
		todo!()
	}

	fn authenticate_capability(
		&self,
		_name: &CapabilityName,
		_capability: &Capability,
	) -> Result<(), ICS05Error> {
		todo!()
	}
}

impl<T: Config> PortReader for Context<T> {
	type ModuleId = ();

	fn lookup_module_by_port(
		&self,
		_port_id: &PortId,
	) -> Result<(Self::ModuleId, Capability), ICS05Error> {
		log::info!("in port: [look_module_by_port]");

		Ok(((), Capability::default()))
	}
	fn authenticate(&self, _port_id: PortId, _cap: &Capability) -> bool {
		log::info!("in port: [authenticate]");
		true
	}
}
