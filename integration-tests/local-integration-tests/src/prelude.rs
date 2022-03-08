//! prelude for pallet Rust level work (not low level storage code neither for IPC calls)
pub use codec::{Decode, Encode};
pub use frame_system::{pallet_prelude::*, Config};
use primitives::currency::CurrencyId;
pub use support::RuntimeDebug;

/// just making it easier to refactor generalized code
pub type LocalAssetId = CurrencyId;
