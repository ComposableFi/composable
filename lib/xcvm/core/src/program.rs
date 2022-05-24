use alloc::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo)]
pub struct XCVMProgram<Instruction> {
	pub instructions: Vec<Instruction>,
	pub instruction_pointer: u32,
}
