//! possible values to use with node CLI as `--chain=` parameter

pub mod composable {
	/// Same `PROD` or must be part if file name of spec `.json`
	pub const DEFAULT: &str = "composable";

	pub const DEV: &str = "composable-dev";
	pub const TEST: &str = "composable-westend";
	pub const PROD: &str = "composable-polkadot";
}

pub mod picasso {
	/// Same as `PROD` or must be part if file name of spec `.json`
	pub const DEFAULT: &str = "picasso";

	pub const DEV: &str = "picasso-dev";
	pub const TEST: &str = "picasso-rococo";
	pub const PROD: &str = "picasso-kusama";
}
