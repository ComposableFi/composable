use super::*;

use crate::routing::Context;
use ibc::core::{
	ics05_port::{
		context::{PortKeeper, PortReader},
		error::Error as ICS05Error,
	},
	ics24_host::identifier::PortId,
	ics26_routing::context::ModuleId,
};

impl<T: Config + Sync + Send> PortReader for Context<T> {
	fn lookup_module_by_port(&self, port_id: &PortId) -> Result<ModuleId, ICS05Error> {
		match port_id.as_str() {
			pallet_ibc_ping::PORT_ID => Ok(ModuleId::from_str(pallet_ibc_ping::MODULE_ID)
				.map_err(|_| ICS05Error::module_not_found(port_id.clone()))?),
			_ => Err(ICS05Error::module_not_found(port_id.clone())),
		}
	}
}

impl<T: Config + Send + Sync> PortKeeper for Context<T> {
	/// Since we are using statically defined ports and module Ids, this is not neccessary.
	fn bind_module_to_port(
		&mut self,
		_module_id: ModuleId,
		_port_id: PortId,
	) -> Result<(), ICS05Error> {
		Ok(())
	}
}
