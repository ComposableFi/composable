use super::*;

use crate::routing::Context;
use ibc::{
	applications::transfer::{
		MODULE_ID_STR as TRANSFER_MODULE_ID, PORT_ID_STR as TRANSFER_PORT_ID,
	},
	core::{
		ics05_port::{context::PortReader, error::Error as ICS05Error},
		ics24_host::identifier::PortId,
		ics26_routing::context::ModuleId,
	},
};

impl<T: Config + Sync + Send> PortReader for Context<T> {
	fn lookup_module_by_port(&self, port_id: &PortId) -> Result<ModuleId, ICS05Error> {
		match port_id.as_str() {
			pallet_ibc_ping::PORT_ID => Ok(ModuleId::from_str(pallet_ibc_ping::MODULE_ID)
				.map_err(|_| ICS05Error::module_not_found(port_id.clone()))?),
			TRANSFER_PORT_ID => Ok(ModuleId::from_str(TRANSFER_MODULE_ID)
				.map_err(|_| ICS05Error::module_not_found(port_id.clone()))?),
			_ => Err(ICS05Error::module_not_found(port_id.clone())),
		}
	}
}
