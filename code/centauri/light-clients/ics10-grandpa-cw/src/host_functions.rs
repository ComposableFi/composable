#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HostFunctionsManager;

impl ics23::HostFunctionsProvider for HostFunctionsManager {
	fn sha2_256(message: &[u8]) -> [u8; 32] {
		unimplemented!()
	}

	fn sha2_512(message: &[u8]) -> [u8; 64] {
		unimplemented!()
	}

	fn sha2_512_truncated(message: &[u8]) -> [u8; 32] {
		unimplemented!()
	}

	fn sha3_512(message: &[u8]) -> [u8; 64] {
		unimplemented!()
	}

	fn ripemd160(message: &[u8]) -> [u8; 20] {
		unimplemented!()
	}
}