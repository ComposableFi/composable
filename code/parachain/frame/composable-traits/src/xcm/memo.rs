use crate::{prelude::*};

use sp_runtime::{
	DispatchError
};

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    codec::Encode,
    codec::Decode,
    scale_info::TypeInfo,
    Ord,
    PartialOrd,
    MaxEncodedLen,
    Debug,
)]
pub struct ChainInfo {
    pub chain_id: u128,
    pub channel_id: u64,        //for packet or memo
    pub timestamp: Option<u64>, //for packet
    pub height: Option<u64>,    //for memo packet message forwarding
    pub retries: Option<u64>,   //for memo packet message forwarding
    pub timeout: Option<u64>,   //for memo packet message forwarding
    pub is_substrate_ibc: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MemoForward {
    receiver: String,
    port: String,
    channel: String,
    timeout: String,
    retries: u64,
    // next: Option<Box<MemoForward>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MemoData {
    forward: MemoForward,
}

impl MemoData {
    /// Support only addresses from cosmos ecosystem based on bech32.
    pub fn new(
        mut vec: Vec<(ChainInfo, Vec<u8>, [u8; 32])>,
    ) -> Result<Option<MemoData>, DispatchError> {
        vec.reverse();
        let mut memo_data: Option<MemoData> = None;
        for (_, _name, address) in vec {
            // use std::borrow::Cow;
            // use std::{error, fmt};
            let mut v = sp_std::vec::Vec::new();
            // bech32
            // use bech32_no_std::{FromBase32, ToBase32};
            //iterate over address and use bech32::u5::try_from_u8 to convert to bech32::u5 and save into vec
            for item in address{
                let x = bech32_no_std::u5::try_from_u8(item).unwrap();
                v.push(item);
            }
            // let _: core::result::Result<Vec<bech32::u5>, bech32::Error> =
            // 	address.into_iter().map(bech32::u5::try_from_u8).collect();
            // let data =
            // 	// result.map_err(|_| Error::<T>::IncorrectAddress { chain_id: i.chain_id as u8 })?;
            // 	result.map_err(|_| DispatchError::Other("()"))?;

            // let name = String::from_utf8(name.into())
            // 	// .map_err(|_| Error::<T>::IncorrectChainName { chain_id: i.chain_id as u8 })?;
            //     .map_err(|_| DispatchError::Other("()"))?;
            // let _ = bech32::encode(&name, data.clone()).map_err(|_| {
            // 	// Error::<T>::FailedToEncodeBech32Address { chain_id: i.chain_id as u8 }
            //     DispatchError::Other("()")
            // })?;

            // let new_memo = MemoData {
            // 	forward: MemoForward {
            // 		receiver: result_address,
            // 		port: String::from("transfer"),
            // 		channel: String::from(format!("channel-{}", i.channel_id)),
            // 		timeout: String::from(i.timeout.unwrap_or_default().to_string()),
            // 		retries: i.retries.unwrap_or_default(),
            // 		// next: memo_data.map(|x| Box::new(x.forward)), // memo_data is boxed here
            // 	},
            // };
            // memo_data = Some(new_memo);
        }
        // Ok(memo_data)
        Ok(memo_data)
    }
}