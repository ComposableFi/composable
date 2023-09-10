//! CW contract interface to send Parity XCM messages
use crate::prelude::*;
use xcm::{latest::*, VersionedMultiLocation, VersionedXcm};

/// manually typed interface to contracts until
/// https://github.com/paritytech/parity-common/pull/785
/// https://github.com/paritytech/polkadot-sdk/pull/1454
impl ExecuteMsg {
	pub fn send(dest: VersionedMultiLocation, message: VersionedXcm<()>) -> Self {
		Self::Send { dest: dest.encode().into(), message: message.encode().into() }
	}

	pub fn reserve_transfer_assets(
		dest: VersionedMultiLocation,
		beneficiary: VersionedMultiLocation,
		assets: Vec<MultiAsset>,
		fee_asset_item: u32,
	) -> Self {
		Self::ReserveTransferAssets {
			dest: dest.encode().into(),
			beneficiary: beneficiary.encode().into(),
			assets: assets.encode().into(),
			fee_asset_item,
		}
	}
	pub fn teleport_assets(
		dest: VersionedMultiLocation,
		beneficiary: VersionedMultiLocation,
		assets: Vec<MultiAsset>,
		fee_asset_item: u32,
	) -> Self {
		Self::TeleportAssets {
			dest: dest.encode().into(),
			beneficiary: beneficiary.encode().into(),
			assets: assets.encode().into(),
			fee_asset_item,
		}
	}
	pub fn execute(message: VersionedXcm<()>, max_weight: u64) -> Self {
		Self::Execute { message: message.encode().into(), max_weight }
	}

	pub fn limited_reserve_transfer_assets(
		dest: VersionedMultiLocation,
		beneficiary: VersionedMultiLocation,
		assets: Vec<MultiAsset>,
		fee_asset_item: u32,
		weight_limit: u64,
	) -> Self {
		Self::LimitedReserveTransferAssets {
			dest: dest.encode().into(),
			beneficiary: beneficiary.encode().into(),
			assets: assets.encode().into(),
			fee_asset_item,
			weight_limit,
		}
	}

	pub fn limited_teleport_assets(
		dest: VersionedMultiLocation,
		beneficiary: VersionedMultiLocation,
		assets: Vec<MultiAsset>,
		fee_asset_item: u32,
		weight_limit: u64,
	) -> Self {
		Self::LimitedTeleportAssets {
			dest: dest.encode().into(),
			beneficiary: beneficiary.encode().into(),
			assets: assets.encode().into(),
			fee_asset_item,
			weight_limit,
		}
	}
}

// see pallet-xcm interface for data encoding for now
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema, cosmwasm_schema::QueryResponses))]
pub enum ExecuteMsg {
	#[cfg_attr(feature = "std", returns(core::result::Result<(),String>))]
	Send { dest: Binary, message: Binary },
	#[cfg_attr(feature = "std", returns(core::result::Result<(),String>))]
	ReserveTransferAssets { dest: Binary, beneficiary: Binary, assets: Binary, fee_asset_item: u32 },
	#[cfg_attr(feature = "std", returns(core::result::Result<(),String>))]
	TeleportAssets { dest: Binary, beneficiary: Binary, assets: Binary, fee_asset_item: u32 },
	#[cfg_attr(feature = "std", returns(core::result::Result<(),String>))]
	Execute { message: Binary, max_weight: u64 },
	#[cfg_attr(feature = "std", returns(core::result::Result<(),String>))]
	LimitedReserveTransferAssets {
		dest: Binary,
		beneficiary: Binary,
		assets: Binary,
		fee_asset_item: u32,
		weight_limit: u64,
	},
	#[cfg_attr(feature = "std", returns(core::result::Result<(),String>))]
	LimitedTeleportAssets {
		dest: Binary,
		beneficiary: Binary,
		assets: Binary,
		fee_asset_item: u32,
		weight_limit: u64,
	},
}
