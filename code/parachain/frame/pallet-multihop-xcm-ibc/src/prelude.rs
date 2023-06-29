pub use composable_traits::{
	assets::Asset,
	currency::{
		AssetExistentialDepositInspect, AssetRatioInspect, BalanceLike, Exponent,
		Rational64 as Rational,
	},
	defi::Ratio,
	xcm::assets::{ForeignMetadata, RemoteAssetRegistryInspect, RemoteAssetRegistryMutate},
};

pub use codec::{Decode, Encode, FullCodec};

// use serde::Serialize;
// use xcm::v3::*;
// pub trait MultiCurrencyCallback<T: Config> {
// 	fn deposit_asset(
// 		asset: &MultiAsset,
// 		location: &MultiLocation,
// 		context: &XcmContext,
// 		deposit_result: Result,
// 		asset_id: Option<<T as pallet_ibc::Config>::AssetId>,
// 	) -> core::result::Result<(), Error<T>>;
// 	//check result, unwrap memo if exists and execute ibc packet
// }
