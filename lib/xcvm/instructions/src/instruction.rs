use alloc::vec::Vec;

#[derive(Clone)]
pub enum XCVMInstruction<Account, Assets> {
	Transfer(Account, Assets),
	Bridge(XCVMNetwork, Assets),
	Call(Vec<u8>),
}
