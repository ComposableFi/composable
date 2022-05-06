//! prelude for pallet Rust level work (not low level storage code neither for IPC calls)
pub use codec::{Decode, Encode};
pub use common::AccountId;
pub use frame_support::RuntimeDebug;
pub use frame_system::{pallet_prelude::*, Config};
use primitives::currency::CurrencyId;
pub use sp_runtime::{FixedPointNumber, FixedU128};
pub use xcm::latest::prelude::*;

/// just making it easier to refactor generalized code
pub type LocalAssetId = CurrencyId;

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
