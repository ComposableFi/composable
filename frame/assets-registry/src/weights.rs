#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

// The weight info trait for `pallet_assets_registry`.
pub trait WeightInfo {
	fn set_local_admin() -> Weight;
	fn set_foreign_admin() -> Weight;
	fn approve_assets_mapping_candidate() -> Weight;
	fn set_metadata() -> Weight;
}

/// Weights for pallet_assets_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: AssetsRegistry LocalAdmin (r:0 w:1)
	fn set_local_admin() -> Weight {
		(9_958_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry ForeignAdmin (r:0 w:1)
	fn set_foreign_admin() -> Weight {
		(9_838_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry LocalAdmin (r:1 w:0)
	// Storage: AssetsRegistry ForeignAdmin (r:1 w:0)
	// Storage: AssetsRegistry LocalToForeign (r:1 w:0)
	// Storage: AssetsRegistry ForeignToLocal (r:1 w:0)
	// Storage: AssetsRegistry AssetsMappingCandidates (r:1 w:1)
	fn approve_assets_mapping_candidate() -> Weight {
		(22_981_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry LocalAdmin (r:1 w:0)
	// Storage: AssetsRegistry ForeignAdmin (r:1 w:0)
	// Storage: AssetsRegistry LocalToForeign (r:1 w:0)
	// Storage: AssetsRegistry ForeignAssetMetadata (r:0 w:1)
	fn set_metadata() -> Weight {
		(17_812_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AssetsRegistry LocalAdmin (r:0 w:1)
	fn set_local_admin() -> Weight {
		(9_958_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry ForeignAdmin (r:0 w:1)
	fn set_foreign_admin() -> Weight {
		(9_838_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry LocalAdmin (r:1 w:0)
	// Storage: AssetsRegistry ForeignAdmin (r:1 w:0)
	// Storage: AssetsRegistry LocalToForeign (r:1 w:0)
	// Storage: AssetsRegistry ForeignToLocal (r:1 w:0)
	// Storage: AssetsRegistry AssetsMappingCandidates (r:1 w:1)
	fn approve_assets_mapping_candidate() -> Weight {
		(22_981_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AssetsRegistry LocalAdmin (r:1 w:0)
	// Storage: AssetsRegistry ForeignAdmin (r:1 w:0)
	// Storage: AssetsRegistry LocalToForeign (r:1 w:0)
	// Storage: AssetsRegistry ForeignAssetMetadata (r:0 w:1)
	fn set_metadata() -> Weight {
		(17_812_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
