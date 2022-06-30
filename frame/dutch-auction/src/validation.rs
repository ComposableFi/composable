use composable_support::validation::Validate;
use composable_traits::xcm::XcmSellRequest;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Clone, Copy, RuntimeDebug, PartialEq, TypeInfo, Default)]
pub struct XcmSellRequestValid;

impl Validate<XcmSellRequest, XcmSellRequestValid> for XcmSellRequestValid {
	fn validate(request: XcmSellRequest) -> Result<XcmSellRequest, &'static str> {
		ensure!(
			request.order.pair.base != request.order.pair.quote,
			"Auction creation with the same asset."
		);
		Ok(request)
	}
}
