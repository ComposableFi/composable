use cosmwasm_std::CanonicalAddr;
use std::collections::VecDeque;

pub type DefaultXCVMInstruction = xc_core::Instruction<Vec<u8>, CanonicalAddr, xc_core::Funds>;
pub type DefaultXCVMProgram = xc_core::Program<VecDeque<DefaultXCVMInstruction>>;
pub type DefaultXCVMPacket = xc_core::Packet<DefaultXCVMProgram>;
pub type Salt = Vec<u8>;
