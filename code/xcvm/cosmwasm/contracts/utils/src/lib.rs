use cosmwasm_std::CanonicalAddr;
use std::collections::VecDeque;
use xcvm_core::{Funds, NetworkId};

pub type DefaultXCVMInstruction = xcvm_core::Instruction<NetworkId, Vec<u8>, CanonicalAddr, Funds>;
pub type DefaultXCVMProgram = xcvm_core::Program<VecDeque<DefaultXCVMInstruction>>;
pub type DefaultXCVMPacket = xcvm_core::Packet<DefaultXCVMProgram>;
pub type Salt = Vec<u8>;
