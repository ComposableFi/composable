use codec::{Decode, Encode};
use composable_traits::{prelude::Deserialize, xcm::assets::RemoteAssetRegistryInspect};
use core::fmt::{Display, Formatter};
use pallet_ibc::ics20::{MemoData, ValidateMemo};
use primitives::currency::{ForeignAssetId, PrefixedDenom};
use scale_info::TypeInfo;
use sp_core::ConstU64;
use std::convert::Infallible;

use crate::prelude::*;

pub struct ForeignIbcIcs20Assets<T>(PhantomData<T>);

impl<T> ForeignIbcIcs20Assets<T>
where
	T: RemoteAssetRegistryInspect<AssetId = CurrencyId, AssetNativeLocation = ForeignAssetId>,
{
	pub fn from_denom_to_asset_id(denom: &str) -> Result<CurrencyId, DispatchError> {
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

pub type MinimumConnectionDelaySeconds = ConstU64<10>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MemoMiddlewareNamespaceChain {
	Forward { next: Option<Box<Self>> },
	Wasm { next: Option<Box<Self>> },
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Encode, Decode, TypeInfo)]
pub struct RawMemo(pub String);

impl FromStr for RawMemo {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self(s.to_string()))
	}
}

impl TryFrom<MemoData> for RawMemo {
	type Error = <String as TryFrom<MemoData>>::Error;

	fn try_from(value: MemoData) -> Result<Self, Self::Error> {
		Ok(Self(value.try_into()?))
	}
}

impl TryFrom<RawMemo> for MemoData {
	type Error = <MemoData as TryFrom<String>>::Error;

	fn try_from(value: RawMemo) -> Result<Self, Self::Error> {
		Ok(value.0.try_into()?)
	}
}

impl Display for RawMemo {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl ValidateMemo for RawMemo {
	fn validate(&self) -> Result<(), String> {
		// the MiddlewareNamespaceChain type contains all the supported middlewares
		serde_json_wasm::from_str::<MemoMiddlewareNamespaceChain>(&self.0)
			.map(|| ())
			.map_err(ToString::to_string)
	}
}

#[test]
fn test_memo_validation() {
	let memo = r#"{
	   "forward":{
		  "receiver":"osmo1uj22h9ksaykwvr33psnq5dhk9f48zsvnyuxr68ryuhlu3jucrqctyh2vq0",
		  "port":"transfer",
		  "channel":"channel-0",
		  "next":{
			 "wasm":{
				"contract":"osmo1usnq5dhk9f48zsvnyuxqctyh2vq0r68ryuhluj22h9ksaykwvr33p3jucr",
				"msg":{
				   "execute_swap_operations":{
					  "minimum_receive":"60325",
					  "routes":[
						 {
							"offer_amount":"5812128242124",
							"operations":[
							   {
								  "t_f_m_swap":{
									 "pool_id":351,
									 "offer_asset_info":{
										"native_token":{
										   "denom":"ibc/F3AE996E091BD322EDD14C27CF37E05AEE4C1E66D7C03B8F6A3E0AC59725107A"
										}
									 },
									 "ask_asset_info":{
										"native_token":{
										   "denom":"uosmo"
										}
									 }
								  }
							   },
							   {
								  "t_f_m_swap":{
									 "pool_id":1,
									 "offer_asset_info":{
										"native_token":{
										   "denom":"uosmo"
										}
									 },
									 "ask_asset_info":{
										"native_token":{
										   "denom":"ibc/4C1F92EEB2527374F36EF416001CEADA9CA97CCD56123CE5EB2A62294FB092D2"
										}
									 }
								  }
							   }
							]
						 }
					  ],
					  "to":"osmo1z0yde9mnpvreulck3ue0umfxfr7pst9h5yffaf"
				   }
				}
			 }
		  }
	   }
	}"#;
	assert_eq!(RawMemo(memo.to_string()).validate(), Ok(()));

	let memo = r#"{
	   "forward":{
		  "receiver":"osmo1uj22h9ksaykwvr33psnq5dhk9f48zsvnyuxr68ryuhlu3jucrqctyh2vq0",
		  "port":"transfer",
		  "channel":"channel-0",
		  "next":{
			 "unknown":{}
		  }
	   }
	}"#;
	assert_eq!(RawMemo(memo.to_string()).validate(), Err(()));
}
