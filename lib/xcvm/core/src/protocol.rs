use crate::network::Callable;

pub trait XCVMProtocol<Network: Callable> {
	type Error;
	fn serialize(&self, network: Network) -> Result<Network::EncodedCall, Self::Error>;
}
