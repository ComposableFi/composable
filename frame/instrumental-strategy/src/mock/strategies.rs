use frame_support::PalletId;
use sp_runtime::traits::AccountIdConversion;

use super::runtime::AccountId;

#[derive(Clone, Copy)]
pub struct Strategy {
    pub pallet_id: PalletId,
}

impl Strategy {
    const fn new(pallet_id: PalletId) -> Strategy {
        Strategy {
            pallet_id: pallet_id,
        }
    }

    pub fn account_id(self: Strategy) -> AccountId {
        self.pallet_id.into_account()
    }
}

// separate module so that the `allow` attribute isn't applied to the entirety of the Strategy
// module.
pub mod strategies {
	#![allow(clippy::upper_case_acronyms)]
    #![allow(non_camel_case_types)]
    #![allow(unused)]

    use super::super::runtime::AccountId;

    use super::Strategy;
	use frame_support::PalletId;

    pub const PABLO_STRATEGY: Strategy = Strategy::new(PalletId(*b"stratpab"));
}

pub use strategies::*;