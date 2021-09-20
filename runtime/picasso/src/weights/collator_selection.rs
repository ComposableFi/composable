#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for pallet_collator_selection using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> collator_selection::WeightInfo for WeightInfo<T> {
	fn set_invulnerables(b: u32) -> Weight {
		(18_563_000 as Weight)
			// Standard Error: 0
			.saturating_add((68_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_desired_candidates() -> Weight {
		(16_363_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_candidacy_bond() -> Weight {
		(16_840_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn register_as_candidate(c: u32) -> Weight {
		(71_196_000 as Weight)
			// Standard Error: 0
			.saturating_add((198_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn leave_intent(c: u32) -> Weight {
		(55_336_000 as Weight)
			// Standard Error: 0
			.saturating_add((151_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn note_author() -> Weight {
		(71_461_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn new_session(r: u32, c: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 1_010_000
			.saturating_add((109_961_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_010_000
			.saturating_add((151_952_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(c as Weight)))
	}
}
