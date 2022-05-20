pub trait XCVMProtocol<Network, AbiEncoded> {
	fn serialize(&self, network: Network) -> AbiEncoded;
}
