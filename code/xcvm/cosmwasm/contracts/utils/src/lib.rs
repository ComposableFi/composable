use cosmwasm_std::CanonicalAddr;
use std::collections::VecDeque;
use xc_core::{Balance, Funds, NetworkId};

pub type DefaultXCVMInstruction =
	xc_core::Instruction<NetworkId, Vec<u8>, CanonicalAddr, Funds<Balance>>;
pub type DefaultXCVMProgram = xc_core::Program<VecDeque<DefaultXCVMInstruction>>;
pub type DefaultXCVMPacket = xc_core::Packet<DefaultXCVMProgram>;
pub type Salt = Vec<u8>;
