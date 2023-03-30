use crate::prelude::*;

pub mod karura {
	pub const ID: u32 = 2000;
	pub const AUSD_KEY: [u8; 2] = [0, 129];
	pub const KAR_KEY: [u8; 2] = [0, 128];
}

pub mod statemine {
	use super::common_good_assets;
	pub const ID: u32 = common_good_assets::ID;
	pub const ASSETS: u8 = common_good_assets::ASSETS;
	pub const USDT: u128 = common_good_assets::USDT;
}
pub mod rockmine {
	use super::common_good_assets;
	pub const ID: u32 = common_good_assets::ID;
	pub const ASSETS: u8 = common_good_assets::ASSETS;
	pub const USDT: u128 = common_good_assets::USDT;
}

pub mod common_good_assets {
	pub const ID: u32 = 1000;
	pub const ASSETS: u8 = 50;
	pub const USDT: u128 = 1984;
}

pub mod relay {
	use xcm::latest::prelude::*;
	pub const LOCATION: MultiLocation = MultiLocation { parents: 1, interior: Here };
}

pub mod this {
	use xcm::latest::prelude::*;
	pub const LOCAL: MultiLocation = MultiLocation { parents: 0, interior: Here };
	pub fn sibling(para_id: u32) -> MultiLocation {
		MultiLocation::new(1, X1(Parachain(para_id)))
	}
}

use composable_traits::{
	assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol},
	rational,
	xcm::{assets::XcmAssetLocation, Balance},
};
use ibc_rs_scale::{
	applications::transfer::{PrefixedDenom as InnerPrefixedDenom, TracePrefix},
	core::ics24_host::identifier::{ChannelId, PortId},
};

use crate::currency::{CurrencyId, ForeignAssetId, PrefixedDenom, WellKnownCurrency};

pub struct Picasso;

impl Picasso {
	pub fn assets() -> Vec<(u64, Option<ForeignAssetId>, AssetInfo<Balance>)> {
		let usdt = (
			1984,
			Some(ForeignAssetId::Xcm(XcmAssetLocation::new(MultiLocation::new(
				1,
				X3(
					Parachain(statemine::ID),
					PalletInstance(statemine::ASSETS),
					GeneralIndex(statemine::USDT),
				),
			)))),
			AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Statemine USDT".to_vec())
						.expect("String is within bounds"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"USDT".to_vec())
						.expect("String is within bounds"),
				),
				decimals: Some(6),
				existential_deposit: 10_000,
				ratio: Some(rational!(375 / 1_000_000)),
			},
		);
		let mut dot = InnerPrefixedDenom::from_str("6").expect("genesis");
		dot.add_trace_prefix(TracePrefix::new(PortId::transfer(), ChannelId::new(0)));
		let dot = (
			6,
			Some(ForeignAssetId::IbcIcs20(PrefixedDenom(dot))),
			AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Polkadot".to_vec())
						.expect("String is within bounds"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"DOT".to_vec())
						.expect("String is within bounds"),
				),
				decimals: Some(12),
				existential_deposit: 1_000_000_000,
				ratio: Some(rational!(375 / 1_000)),
			},
		);

		vec![usdt, dot]
	}
}

impl WellKnownCurrency for Picasso {
	const NATIVE: CurrencyId = CurrencyId::PICA;
	const RELAY_NATIVE: CurrencyId = CurrencyId::KSM;
}

pub struct Composable;

impl Composable {
	pub fn assets() -> Vec<(u64, Option<ForeignAssetId>, AssetInfo<Balance>)> {
		let usdt = (
			1984,
			Some(ForeignAssetId::Xcm(XcmAssetLocation::new(MultiLocation::new(
				1,
				X3(
					Parachain(statemine::ID),
					PalletInstance(statemine::ASSETS),
					GeneralIndex(statemine::USDT),
				),
			)))),
			AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Statemint USDT".to_vec())
						.expect("String is within bounds"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"USDT".to_vec())
						.expect("String is within bounds"),
				),
				decimals: Some(6),
				existential_deposit: 10_000,
				ratio: Some(rational!(375 / 1_000_000)),
			},
		);
		let mut pica = InnerPrefixedDenom::from_str("6").expect("genesis");
		pica.add_trace_prefix(TracePrefix::new(PortId::transfer(), ChannelId::new(0)));
		let pica = (
			1,
			Some(ForeignAssetId::IbcIcs20(PrefixedDenom(pica))),
			AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Picasso".to_vec())
						.expect("String is within bounds"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"PICA".to_vec())
						.expect("String is within bounds"),
				),
				decimals: Some(12),
				existential_deposit: 1_000_000_000,
				ratio: Some(rational!(1 / 1)),
			},
		);

		vec![usdt, pica]
	}
}

impl WellKnownCurrency for Composable {
	const NATIVE: CurrencyId = CurrencyId::LAYR;
	const RELAY_NATIVE: CurrencyId = CurrencyId::DOT;
}
