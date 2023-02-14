use frame_support::parameter_types;
use sp_runtime::create_runtime_str;
use sp_version::RuntimeVersion;

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("dali"),
	impl_name: create_runtime_str!("dali"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 10_008,
	impl_version: 3,
	apis: crate::RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> sp_version::NativeVersion {
	sp_version::NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}
