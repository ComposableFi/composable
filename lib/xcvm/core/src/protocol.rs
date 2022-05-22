use crate::network::Callable;

pub trait XCVMProtocol<Network: Callable> {
	fn serialize(&self, network: Network) -> Network::EncodedCall;
}
