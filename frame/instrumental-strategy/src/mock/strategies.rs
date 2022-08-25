use frame_support::PalletId;
use sp_runtime::traits::AccountIdConversion;

use super::runtime::AccountId;

#[derive(Clone, Copy)]
pub struct Strategy {
	pub pallet_id: PalletId,
}

impl Strategy {
	const fn new(pallet_id: PalletId) -> Strategy {
		Strategy { pallet_id }
	}

	// TODO(saruman9): remove dead code in the future?
	pub fn _account_id(self: Strategy) -> AccountId {
		self.pallet_id.into_account_truncating()
	}
}

// separate module so that the `allow` attribute isn't applied to the entirety of the Strategy
// module.
pub mod defined_strategies {
	#![allow(clippy::upper_case_acronyms)]
	#![allow(non_camel_case_types)]
	#![allow(unused)]

	use frame_support::PalletId;

	use super::{super::runtime::AccountId, Strategy};

	pub const PABLO_STRATEGY: Strategy = Strategy::new(PalletId(*b"stratpab"));
}

pub use defined_strategies::*;
