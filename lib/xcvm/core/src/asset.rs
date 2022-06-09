use alloc::collections::BTreeMap;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Debug,
	Encode,
	Decode,
	TypeInfo,
	Serialize,
	Deserialize,
)]
#[repr(transparent)]
pub struct XCVMAsset(pub u32);

impl XCVMAsset {
	pub const PICA: XCVMAsset = XCVMAsset(1);
	pub const ETH: XCVMAsset = XCVMAsset(2);
	pub const USDT: XCVMAsset = XCVMAsset(3);
	pub const USDC: XCVMAsset = XCVMAsset(4);

	pub const UST: XCVMAsset = XCVMAsset(0xDEADC0DE);
}

impl From<XCVMAsset> for u32 {
	fn from(val: XCVMAsset) -> Self {
		val.0
	}
}

impl From<u32> for XCVMAsset {
	fn from(asset: u32) -> Self {
		XCVMAsset(asset)
	}
}

#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[derive(
	Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo, Serialize, Deserialize,
)]
#[repr(transparent)]
pub struct XCVMTransfer(pub BTreeMap<XCVMAsset, u128>);

impl<U, V> From<BTreeMap<U, V>> for XCVMTransfer
where
	U: Into<XCVMAsset>,
	V: Into<u128>,
{
	fn from(assets: BTreeMap<U, V>) -> Self {
		XCVMTransfer(
			assets
				.into_iter()
				.map(|(asset, amount)| (asset.into(), amount.into()))
				.collect(),
		)
	}
}

impl<U, V, const K: usize> From<[(U, V); K]> for XCVMTransfer
where
	U: Into<XCVMAsset>,
	V: Into<u128>,
{
	fn from(x: [(U, V); K]) -> Self {
		XCVMTransfer(x.into_iter().map(|(asset, amount)| (asset.into(), amount.into())).collect())
	}
}

impl From<XCVMTransfer> for BTreeMap<u32, u128> {
	fn from(XCVMTransfer(assets): XCVMTransfer) -> Self {
		assets.into_iter().map(|(XCVMAsset(asset), amount)| (asset, amount)).collect()
	}
}
