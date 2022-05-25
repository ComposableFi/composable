use alloc::collections::VecDeque;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo)]
pub enum XCVMInstruction<Network, AbiEncoded, Account, Assets> {
	Transfer(Account, Assets),
	Call(AbiEncoded),
	Spawn(Network, Assets, VecDeque<XCVMInstruction<Network, AbiEncoded, Account, Assets>>),
}
