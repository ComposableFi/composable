use super::*;
use ibc::core::{ics05_port::capabilities::PortCapability, ics24_host::identifier::PortId};
use ibc_primitives::SendPacketData;

pub trait IbcTrait<T: Config> {
	fn send_packet(data: SendPacketData) -> Result<(), Error<T>>;
	fn bind_port(port_id: PortId) -> Result<PortCapability, Error<T>>;
}
