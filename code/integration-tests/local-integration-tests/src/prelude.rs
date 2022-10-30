//! prelude for pallet Rust level work (not low level storage code neither for IPC calls)
pub use crate::testing::*;
pub use codec::{Decode, Encode};
pub use common::{topology, AccountId, fees::{PriceConverter, multi_existential_deposits, NativeExistentialDeposit}};
pub use composable_traits::{currency::CurrencyFactory, xcm::assets::XcmAssetLocation};
pub use cumulus_primitives_core::ParaId;
pub use frame_support::{
	assert_err, assert_err_ignore_postinfo, assert_ok, log, traits::fungible::Inspect, RuntimeDebug,
};
pub use frame_system::{pallet_prelude::*, Config};
use primitives::currency::CurrencyId;
pub use sp_runtime::{traits::StaticLookup, FixedPointNumber, FixedU128};
pub use xcm::{latest::prelude::*, VersionedMultiLocation};
pub use xcm_emulator::TestExt;
pub use xcm_executor::XcmExecutor;

#[cfg(test)]
pub use more_asserts::*;

pub type XcmCurrency<
	Consensus,
	const ID: u128,
	const EXPONENT: u8 = 12,
	const RESERVE_EXPONENT: u8 = 12,
> = composable_tests_helpers::test::currency::ComposableCurrency<
	Consensus,
	ID,
	EXPONENT,
	RESERVE_EXPONENT,
>;

pub type USDT = XcmCurrency<statemine_runtime::Runtime, 1984, 6, 6>;

#[allow(non_camel_case_types)]
pub type xUSDT = XcmCurrency<this_runtime::Runtime, 1984, 12, 6>;

pub type STABLE = XcmCurrency<statemine_runtime::Runtime, 666, 12, 3>;
pub type PICA = XcmCurrency<this_runtime::Runtime, 1, 12, 12>;
pub type KSM = XcmCurrency<statemine_runtime::Runtime, 1, 12, 12>;

#[allow(non_camel_case_types)]
pub type RELAY_NATIVE = KSM;

pub type SHIB = XcmCurrency<statemine_runtime::Runtime, 100500, 12>;

// <= what we may think users are ok
pub const ORDER_OF_FEE_ESTIMATE_ERROR: u128 = 10;

pub const THIS_CHAIN_NATIVE_FEE: u128 = 4_000_000_000;

pub const RELAY_CHAIN_NATIVE_FEE: u128 = 706_666_660;

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
