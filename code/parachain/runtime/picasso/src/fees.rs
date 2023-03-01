use common::fees::{ForeignToNativePriceConverter, PriceConverter};
use composable_traits::{currency::Rational64, rational};

use primitives::currency::CurrencyId;

pub struct WellKnownForeignToNativePriceConverter;
impl ForeignToNativePriceConverter for WellKnownForeignToNativePriceConverter {
	fn get_ratio(asset_id: CurrencyId) -> Option<Rational64> {
		match asset_id {
			CurrencyId::KSM => Some(rational!(375 / 1_000_000)),
			CurrencyId::ibcDOT => Some(rational!(2143 / 1_000_000)),
			CurrencyId::USDT | CurrencyId::USDC => Some(rational!(15 / 1_000_000_000)),
			CurrencyId::kUSD => Some(rational!(15 / 1_000)),
			CurrencyId::PICA => Some(rational!(1 / 1)),
			CurrencyId::PBLO => Some(rational!(1 / 1)),
			CurrencyId::KSM_USDT_LPT => Some(rational!(1 / 1_000_000_000)),
			CurrencyId::PICA_USDT_LPT => Some(rational!(1 / 1_000_000_000)),
			CurrencyId::PICA_KSM_LPT => Some(rational!(1 / 1_000_000_000)),
			_ => None,
		}
	}
}

pub type FinalPriceConverter =
	PriceConverter<crate::AssetsRegistry, WellKnownForeignToNativePriceConverter>;
