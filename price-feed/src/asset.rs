custom_derive! {
	#[derive(EnumFromStr, Copy, Clone, PartialEq, Eq, Hash, Debug)]
	pub enum Asset {
		BTC,
		ETH,
		LTC,
		USD,
	}
}

pub type AssetPair = (Asset, Asset);

pub fn symbol((x, y): AssetPair) -> String {
    format!("{:?}/{:?}", x, y)
}
