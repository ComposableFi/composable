use composable_traits::xcm::assets::RemoteAssetRegistryInspect;
use primitives::currency::{ForeignAssetId, PrefixedDenom};

use crate::prelude::*;

pub struct ForeignIbcIcs20Assets<T>(PhantomData<T>);

impl<T> ForeignIbcIcs20Assets<T>
where
	T: RemoteAssetRegistryInspect<AssetId = CurrencyId, AssetNativeLocation = ForeignAssetId>,
{
	pub fn from_denom_to_asset_id(denom: &String) -> Result<CurrencyId, DispatchError> {
		let denom = PrefixedDenom::from_str(denom)?;
		if denom.0.trace_path.is_empty() {
			Ok(CurrencyId(denom.0.base_denom.as_str().parse().map_err(|_| {
				DispatchError::Other("IbcDenomToAssetIdConversion: denom not found")
			})?))
		} else {
			T::location_to_asset(ForeignAssetId::IbcIcs20(denom))
				.ok_or(DispatchError::Other("IbcDenomToAssetIdConversion: denom not found"))
		}
	}

	pub fn from_asset_id_to_denom(id: CurrencyId) -> Option<String> {
		match T::asset_to_remote(id) {
			Some(ForeignAssetId::IbcIcs20(denom)) => Some(denom.to_string()),
			_ => Some(id.0.to_string()),
		}
	}
}
