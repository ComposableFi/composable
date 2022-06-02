use alloc::collections::VecDeque;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize)]
pub enum XCVMInstruction<Network, Payload, Account, Assets> {
	Transfer { to: Account, assets: Assets },
	Call { encoded: Payload },
	Spawn { network: Network, assets: Assets, program: VecDeque<Self> },
}
