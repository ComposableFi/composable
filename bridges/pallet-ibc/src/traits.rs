use super::*;
use ibc_primitives::SendPacketData;

pub trait SendPacketTrait<T: Config> {
	fn send_packet(data: SendPacketData) -> Result<(), Error<T>>;
}
