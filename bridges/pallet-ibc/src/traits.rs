use super::*;
use ibc::core::ics05_port::capabilities::Capability;

pub struct SendPacketData {
    pub data: Vec<u8>,
    pub timeout_height_offset: u64,
    /// This value should be represent nano seconds
    pub timeout_timestamp_offset: u64,
    pub capability: Capability,
    pub port_id: String,
    pub channel_id: String
}

pub trait SendPacketTrait<T: Config> {
    fn send_packet(data: SendPacketData) -> Result<(), Error<T>>;
}