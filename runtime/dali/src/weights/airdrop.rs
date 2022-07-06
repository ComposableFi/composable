#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `airdrop`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> airdrop::weights::WeightInfo for WeightInfo<T> {
	fn create_airdrop() -> Weight {
		0
	}

	fn add_recipient(_x: u32) -> Weight {
		0
	}

	fn remove_recipient() -> Weight {
		0
	}

	fn enable_airdrop() -> Weight {
		0
	}

	fn disable_airdrop() -> Weight {
		0
	}

	fn claim(_x: u32) -> Weight {
		0
	}
} 
