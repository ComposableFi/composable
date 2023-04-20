#[cfg(feature = "std")]
use crate::{currency::VersionedMultiLocation, prelude::*};

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

#[cfg(feature = "std")]
use composable_traits::{
	assets::{AssetInfo, BiBoundedAssetName, BiBoundedAssetSymbol},
	rational,
	xcm::Balance,
};

#[cfg(feature = "std")]
use ibc_rs_scale::{
	applications::transfer::{PrefixedDenom as InnerPrefixedDenom, TracePrefix},
	core::ics24_host::identifier::{ChannelId, PortId},
};

#[cfg(feature = "std")]
use crate::currency::{CurrencyId, ForeignAssetId, PrefixedDenom, WellKnownCurrency};
#[cfg(not(feature = "std"))]
use crate::currency::{CurrencyId, WellKnownCurrency};

pub struct Picasso;

#[cfg(feature = "std")]
impl Picasso {
	#[allow(clippy::unseparated_literal_suffix)]
	pub fn assets() -> Vec<(u64, Option<ForeignAssetId>, AssetInfo<Balance>)> {
		let usdt = (
			CurrencyId::USDT.0 as u64,
			Some(ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::new(
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
				existential_deposit: 1500,
				ratio: Some(rational!(2 / 10000000)),
			},
		);

		let ksm = (
			CurrencyId::KSM.0 as u64,
			Some(ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::new(1, Here)))),
			AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Kusama".to_vec())
						.expect("String is within bounds"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"KSM".to_vec())
						.expect("String is within bounds"),
				),
				decimals: Some(12),
				existential_deposit: 375000000,
				ratio: Some(rational!(70 / 10000)),
			},
		);

		let mut dot =
			InnerPrefixedDenom::from_str(CurrencyId::DOT.to_string().as_str()).expect("genesis");
		dot.add_trace_prefix(TracePrefix::new(PortId::transfer(), ChannelId::new(0)));

		let dot = (
			CurrencyId::DOT.0 as u64,
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
				decimals: Some(10),
				existential_deposit: 21430000,
				ratio: Some(rational!(3 / 10000)),
			},
		);

		vec![usdt, dot, ksm]
	}
}

impl WellKnownCurrency for Picasso {
	const NATIVE: CurrencyId = CurrencyId::PICA;
	const RELAY_NATIVE: CurrencyId = CurrencyId::KSM;
}

pub struct Composable;

#[cfg(feature = "std")]
impl Composable {
	#[allow(clippy::unseparated_literal_suffix)]
	pub fn assets() -> Vec<(u64, Option<ForeignAssetId>, AssetInfo<Balance>)> {
		let usdt_statemint = (
			CurrencyId::USDTP.0 as u64,
			Some(ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::new(
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
				existential_deposit: 1500,
				ratio: Some(rational!(2 / 10000000)),
			},
		);
		let mut pica =
			InnerPrefixedDenom::from_str(CurrencyId::PICA.to_string().as_str()).expect("genesis");
		pica.add_trace_prefix(TracePrefix::new(PortId::transfer(), ChannelId::new(0)));
		let pica = (
			CurrencyId::PICA.0 as u64,
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
				existential_deposit: 1000000000,
				ratio: Some(rational!(1 / 1)),
			},
		);
		let dot = (
			CurrencyId::DOT.0 as u64,
			Some(ForeignAssetId::Xcm(VersionedMultiLocation::V3(MultiLocation::new(1, Here)))),
			AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Polkadot".to_vec())
						.expect("String is within bounds"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"DOT".to_vec())
						.expect("String is within bounds"),
				),
				decimals: Some(10),
				existential_deposit: 21430000,
				ratio: Some(rational!(3 / 10000)),
			},
		);

		let mut ksm =
			InnerPrefixedDenom::from_str(CurrencyId::KSM.to_string().as_str()).expect("genesis");
		ksm.add_trace_prefix(TracePrefix::new(PortId::transfer(), ChannelId::new(0)));

		let ksm = (
			CurrencyId::KSM.0 as u64,
			Some(ForeignAssetId::IbcIcs20(PrefixedDenom(ksm))),
			AssetInfo {
				name: Some(
					BiBoundedAssetName::from_vec(b"Kusama".to_vec())
						.expect("String is within bounds"),
				),
				symbol: Some(
					BiBoundedAssetSymbol::from_vec(b"KSM".to_vec())
						.expect("String is within bounds"),
				),
				decimals: Some(12),
				existential_deposit: 375000000,
				ratio: Some(rational!(70 / 10000)),
			},
		);

		vec![usdt_statemint, pica, dot, ksm]
	}
}

impl WellKnownCurrency for Composable {
	const NATIVE: CurrencyId = CurrencyId::LAYR;
	const RELAY_NATIVE: CurrencyId = CurrencyId::DOT;
}
