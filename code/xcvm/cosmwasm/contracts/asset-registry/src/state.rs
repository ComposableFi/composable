use crate::msg::{AssetReference, AssetKey};
use cw_storage_plus::Map;

pub const ASSETS: Map<AssetKey, AssetReference> = Map::new("assets");
