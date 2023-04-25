use frame_support::parameter_types;
use sp_runtime::create_runtime_str;
#[cfg(feature = "std")]
use sp_version::NativeVersion;

use crate::prelude::*;

use crate::RUNTIME_API_VERSIONS;

#[allow(clippy::unseparated_literal_suffix)]
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("picasso"),
	impl_name: create_runtime_str!("picasso"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	spec_version: 10018,
	impl_version: 2,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}
