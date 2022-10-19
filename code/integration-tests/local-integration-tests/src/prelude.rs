//! prelude for pallet Rust level work (not low level storage code neither for IPC calls)
pub use codec::{Decode, Encode};
pub use common::{topology, AccountId};
pub use composable_traits::{currency::CurrencyFactory, xcm::assets::XcmAssetLocation};
pub use cumulus_primitives_core::ParaId;
pub use frame_support::{traits::fungible::Inspect, RuntimeDebug};
pub use frame_system::{pallet_prelude::*, Config};
use primitives::currency::CurrencyId;
pub use sp_runtime::{traits::StaticLookup, FixedPointNumber, FixedU128};
pub use xcm::{latest::prelude::*, VersionedMultiLocation};

#[cfg(test)]
pub use more_asserts::*;

pub const UNIT_12: u128 = 1_000_000_000_000;
pub const RELAY_NATIVE_UNIT: u128 = UNIT_12;
pub const USDT_UNIT: u128 = 10_000;

// <= what we may think users are ok
pub const ORDER_OF_FEE_ESTIMATE_ERROR: u128 = 10;

pub const THIS_CHAIN_NATIVE_FEE: u128 = 4_000_000_000;

pub const RELAY_CHAIN_NATIVE_FEE: u128 = 706_666_660;

/// just making it easier to refactor generalized code
pub type LocalAssetId = CurrencyId;

#[cfg(feature = "rococo")]
pub use rococo_runtime as relay_runtime;

#[cfg(feature = "kusama")]
pub use kusama_runtime as relay_runtime;

#[cfg(feature = "dali")]
pub use dali_runtime as this_runtime;

#[cfg(feature = "dali")]
pub use dali_runtime as sibling_runtime;

#[cfg(feature = "dali")]
pub use dali_runtime::{MaxInstructions, UnitWeightCost, Weight, XcmConfig};

#[cfg(feature = "picasso")]
pub use picasso_runtime as this_runtime;

#[cfg(feature = "picasso")]
pub use picasso_runtime as sibling_runtime;

#[cfg(feature = "picasso")]
pub use picasso_runtime::{MaxInstructions, UnitWeightCost, Weight, XcmConfig};

pub const ALICE: [u8; 32] = [4_u8; 32];
pub const BOB: [u8; 32] = [5_u8; 32];
pub const CHARLIE: [u8; 32] = [6_u8; 32];

/// 40 < 42 < 40 + 3
#[macro_export]
macro_rules! assert_gt_by {
	($actual:expr, $lower:expr, $positive_delta:expr $(,)?) => {{
		more_asserts::assert_gt!($actual, $lower);
		more_asserts::assert_lt!($actual, $lower + $positive_delta);
	}};
}

/// 43 - 3 < 42 < 43
/// ```ignore
/// local_integration_tests::assert_lt_by!(42,43,3);
/// ```
#[macro_export]
macro_rules! assert_lt_by {
	($actual:expr, $upper:expr, $negative_delta:expr $(,)?) => {{
		more_asserts::assert_lt!($actual, $upper);
		more_asserts::assert_gt!($actual, $upper - $negative_delta);
	}};
}
