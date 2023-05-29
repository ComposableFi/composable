use frame_support::parameter_types;
use sp_api::RuntimeVersion;
use sp_runtime::create_runtime_str;

use crate::RUNTIME_API_VERSIONS;

#[allow(clippy::unseparated_literal_suffix)]
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("composable"),
	impl_name: create_runtime_str!("composable"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	spec_version: 10024,
	impl_version: 3,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 2,
	state_version: 0,
};

#[cfg(feature = "std")]
use sp_version::NativeVersion;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}
