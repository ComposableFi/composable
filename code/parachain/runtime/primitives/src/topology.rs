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
