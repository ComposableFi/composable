#[derive(Clone)]
pub enum XCVMInstruction<Network, AbiEncoded, Account, Assets> {
	Transfer(Account, Assets),
	Bridge(Network, Assets),
	Call(AbiEncoded),
}
