//! CW contract interface to send Parity XCM messages
use crate::prelude::*;
use xcm::{latest::*, VersionedMultiLocation, VersionedXcm};

/// manually typed interface to contracts until
/// https://github.com/paritytech/parity-common/pull/785
/// https://github.com/paritytech/polkadot-sdk/pull/1454
impl Msg {     
    pub fn send(dest: VersionedMultiLocation, message : VersionedXcm<()>) -> Self {
        Self::Send {
            dest: dest.encode().into(),
            message: message.encode().into(),
        }
    }

    pub fn reserve_transfer_assets(dest: VersionedMultiLocation, beneficiary : VersionedMultiLocation, message : VersionedXcm<()>, assets : Vec<MultiAsset>, fee_asset_item : u32) -> Self {
        Self::ReserveTransferAssets {
            dest: dest.encode().into(),
            beneficiary: beneficiary.encode().into(),
            message: message.encode().into(),
            assets: assets.encode().into(),
            fee_asset_item,
        }
    }
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
        fee_asset_item : u32,
    },
    TeleportAssets {
        dest: Binary,
        beneficiary : Binary,
        assets: Binary,
        fee_asset_item : u32,
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
    },
    LimitedTeleportAssets {
        dest: Binary,
        beneficiary : Binary,
        assets: Binary,
        fee_asset_item : Binary,        
        weight_limit : u64,
    }

}