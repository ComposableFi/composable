use xc_core::{gateway::AssetItem, AssetId, NetworkId};

use crate::prelude::*;

/// when assets to be sent to other network it should be mapped before sent
pub(crate) const NETWORK_ASSET: Map<(AssetId, NetworkId), AssetId> = Map::new("network_asset");

pub(crate) const ASSETS: Map<AssetId, AssetItem> = Map::new("assets");
