use crate::msg::{AssetKey, AssetReference};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const ASSETS: Map<AssetKey, AssetReference> = Map::new("assets");
pub const ADMIN: Item<Addr> = Item::new("admin");
