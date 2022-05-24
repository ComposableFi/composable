use alloc::collections::VecDeque;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo)]
pub struct XCVMProgram<Instruction> {
	pub instructions: VecDeque<Instruction>,
}
