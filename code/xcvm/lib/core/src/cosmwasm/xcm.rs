//! CW contract interface to send Parity XCM messages
use crate::prelude::*;

impl Msg {
    pub 
}

pub enum Msg {
    /// Send a message to another parachain
    Send{
        /// VersionedMultiLocation, it does not support schemars, so we use a binary
        dest: Binary,
        message : Binary,
    },
    /// Send a message to another parachain
    ReserveTransferAssets{
        /// VersionedMultiLocation, it does not support schemars, so we use a binary
        dest: Binary,
        beneficiary : Binary,
        message : Binary,
        assets : Binary, 
    },
    Execute {
        message: Binary,
        max_weight: u64,
    },
    LimitedReserveTransferAssets {
        dest: Binary,
        beneficiary : Binary,
        assets: Binary,
        fee_asset_item : Binary,        
        weight_limit : u64,
    }
    LimitedTeleportAssets {
        dest: Binary,
        beneficiary : Binary,
        assets: Binary,
        fee_asset_item : Binary,        
        weight_limit : u64,
    }

}