use composable_support::validation::Validate;
use composable_traits::xcm::XcmSellRequest;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Clone, Copy, RuntimeDebug, PartialEq, TypeInfo, Default)]
pub struct XcmSellRequestValid;

impl Validate<XcmSellRequest, XcmSellRequestValid> for XcmSellRequestValid {
	fn validate(request: XcmSellRequest) -> Result<XcmSellRequest, &'static str> {
		let base = request.order.pair.base;
		let quote = request.order.pair.quote;
		if base == quote {
			return Err("Auction creation with the same asset.")
		}
		Ok(request)
	}
}
