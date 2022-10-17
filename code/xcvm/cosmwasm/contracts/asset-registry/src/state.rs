use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub type XcvmAssetId = u128;

pub const ASSETS: Map<XcvmAssetId, Addr> = Map::new("assets");
