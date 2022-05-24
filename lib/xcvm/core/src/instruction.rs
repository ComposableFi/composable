use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo)]
pub enum XCVMInstruction<Network, AbiEncoded, Account, Assets> {
	Transfer(Account, Assets),
	Bridge(Network, Assets),
	Call(AbiEncoded),
	Spawn(Network, Vec<XCVMInstruction<Network, AbiEncoded, Account, Assets>>),
}
