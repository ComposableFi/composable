//! prelude for pallet Rust level work (not low level storage code neither for IPC calls)
pub use codec::{Decode, Encode};
pub use frame_system::{pallet_prelude::*, Config};
pub use support::RuntimeDebug;
