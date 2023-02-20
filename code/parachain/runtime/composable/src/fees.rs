use common::fees::{ForeignToNativePriceConverter, PriceConverter};
use composable_traits::{
	currency::{Rational64, Unit},
	rational,
};

use primitives::currency::CurrencyId;

pub struct WellKnownForeignToNativePriceConverter;
impl ForeignToNativePriceConverter for WellKnownForeignToNativePriceConverter {
	fn get_ratio(asset_id: CurrencyId) -> Option<Rational64> {
		match asset_id {
			CurrencyId::xcDOT => Some(rational!(2143 / 1_000_000)),
			CurrencyId::LAYR => Some(rational!(1 / 1)),
			_ => None,
		}
	}
}

pub type FinalPriceConverter =
	PriceConverter<Unit<CurrencyId>, WellKnownForeignToNativePriceConverter>;
