use crate::msg::AssetReference;
use cw_storage_plus::Map;

pub type XcvmAssetId = u128;

pub const ASSETS: Map<XcvmAssetId, AssetReference> = Map::new("assets");
