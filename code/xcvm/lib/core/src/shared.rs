use crate::{prelude::*, AssetId};
use cosmwasm_std::{from_binary, to_binary, Binary, CanonicalAddr, StdResult};
use serde::{de::DeserializeOwned, Serialize};

pub type Salt = Vec<u8>;
/// absolute amounts
pub type XcFunds = Vec<(AssetId, Displayed<u128>)>;
// like `XcFunds`, but allow relative(percentages) amounts. Similar to assets filters in XCM
pub type XcBalanceFilter = crate::asset::Balance;
pub type XcFundsFilter = crate::Funds<XcBalanceFilter>;
pub type XcInstruction = crate::Instruction<Vec<u8>, XcAddr, XcFundsFilter>;
pub type XcPacket = crate::Packet<XcProgram>;
pub type XcProgram = crate::Program<VecDeque<XcInstruction>>;
