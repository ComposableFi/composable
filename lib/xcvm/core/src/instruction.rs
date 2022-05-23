use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum XCVMInstruction<Network, AbiEncoded, Account, Assets> {
	Transfer(Account, Assets),
	Bridge(Network, Assets),
	Call(AbiEncoded),
}
