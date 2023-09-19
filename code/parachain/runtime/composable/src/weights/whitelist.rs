#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_whitelist`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_whitelist::WeightInfo for WeightInfo<T> {
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn whitelist_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `118`
		//  Estimated: `3556`
		// Minimum execution time: 20_665_000 picoseconds.
		Weight::from_parts(21_174_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn remove_whitelisted_call() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `3556`
		// Minimum execution time: 18_337_000 picoseconds.
		Weight::from_parts(18_705_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage PreimageFor (r:1 w:1)
	/// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 4194294]`.
	fn dispatch_whitelisted_call(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `323 + n * (1 ±0)`
		//  Estimated: `3787 + n * (1 ±0)`
		// Minimum execution time: 30_433_000 picoseconds.
		Weight::from_parts(30_800_000, 0)
			.saturating_add(Weight::from_parts(0, 3787))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_558, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
	}
	/// Storage: Whitelist WhitelistedCall (r:1 w:1)
	/// Proof: Whitelist WhitelistedCall (max_values: None, max_size: Some(40), added: 2515, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:1)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10000]`.
	fn dispatch_whitelisted_call_with_preimage(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `3556`
		// Minimum execution time: 22_062_000 picoseconds.
		Weight::from_parts(22_797_644, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			// Standard Error: 3
			.saturating_add(Weight::from_parts(1_493, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}