//! Fee adjustment utilities

use sp_runtime::{
	traits::{Convert, Saturating},
	FixedPointNumber, FixedU128, Perquintill,
};
use support::{pallet_prelude::*, weights::DispatchClass};

/// A struct to update the weight multiplier per block. It implements `Convert<Multiplier,
/// Multiplier>`, meaning that it can convert the previous multiplier to the next one. This should
/// be called on `on_finalize` of a block, prior to potentially cleaning the weight data from the
/// system pallet.
///
/// given:
///     s = previous block weight
///     s'= ideal block weight
///     m = maximum block weight
///     diff = (s - s')/m
///     v = 0.00001
///     t1 = (v * diff)
///     t2 = (v * diff)^2 / 2
/// then:
///     next_multiplier = prev_multiplier * (1 + t1 + t2)
///
/// Where `(s', v)` must be given as the `Get` implementation of the `T` generic type. Moreover, `M`
/// must provide the minimum allowed value for the multiplier. Note that a runtime should ensure
/// with tests that the combination of this `M` and `V` is not such that the multiplier can drop to
/// zero and never recover.
///
/// note that `s'` is interpreted as a portion in the _normal transaction_ capacity of the block.
/// For example, given `s' == 0.25` and `AvailableBlockRatio = 0.75`, then the target fullness is
/// _0.25 of the normal capacity_ and _0.1875 of the entire block_.
///
/// This implementation implies the bound:
/// - `v ≤ p / k * (s − s')`
/// - or, solving for `p`: `p >= v * k * (s - s')`
///
/// where `p` is the amount of change over `k` blocks.
///
/// Hence:
/// - in a fully congested chain: `p >= v * k * (1 - s')`.
/// - in an empty chain: `p >= v * k * (-s')`.
///
/// For example, when all blocks are full and there are 28800 blocks per day (default in
/// `substrate-node`) and v == 0.00001, s' == 0.1875, we'd have:
///
/// p >= 0.00001 * 28800 * 0.8125
/// p >= 0.234
///
/// Meaning that fees can change by around ~23% per day, given extreme congestion.
///
/// More info can be found at:
/// <https://w3f-research.readthedocs.io/en/latest/polkadot/Token%20Economics.html>
pub struct TargetedFeeAdjustment<T, S, V, M>(sp_std::marker::PhantomData<(T, S, V, M)>);

/// Fee multiplier.
pub type Multiplier = FixedU128;

/// Something that can convert the current multiplier to the next one.
pub trait MultiplierUpdate: Convert<Multiplier, Multiplier> {
	/// Minimum multiplier
	fn min() -> Multiplier;
	/// Target block saturation level
	fn target() -> Perquintill;
	/// Variability factor
	fn variability() -> Multiplier;
}

impl MultiplierUpdate for () {
	fn min() -> Multiplier {
		Default::default()
	}
	fn target() -> Perquintill {
		Default::default()
	}
	fn variability() -> Multiplier {
		Default::default()
	}
}

impl<T, S, V, M> MultiplierUpdate for TargetedFeeAdjustment<T, S, V, M>
where
	T: system::Config,
	S: Get<Perquintill>,
	V: Get<Multiplier>,
	M: Get<Multiplier>,
{
	fn min() -> Multiplier {
		M::get()
	}
	fn target() -> Perquintill {
		S::get()
	}
	fn variability() -> Multiplier {
		V::get()
	}
}

impl<T, S, V, M> Convert<Multiplier, Multiplier> for TargetedFeeAdjustment<T, S, V, M>
where
	T: system::Config,
	S: Get<Perquintill>,
	V: Get<Multiplier>,
	M: Get<Multiplier>,
{
	fn convert(previous: Multiplier) -> Multiplier {
		// Defensive only. The multiplier in storage should always be at most positive. Nonetheless
		// we recover here in case of errors, because any value below this would be stale and can
		// never change.
		let min_multiplier = M::get();
		let previous = previous.max(min_multiplier);

		let weights = T::BlockWeights::get();
		// the computed ratio is only among the normal class.
		let normal_max_weight =
			weights.get(DispatchClass::Normal).max_total.unwrap_or(weights.max_block);
		let current_block_weight = <system::Pallet<T>>::block_weight();
		let normal_block_weight =
			*current_block_weight.get(DispatchClass::Normal).min(&normal_max_weight);

		let s = S::get();
		let v = V::get();

		let target_weight = (s * normal_max_weight) as u128;
		let block_weight = normal_block_weight as u128;

		// determines if the first_term is positive
		let positive = block_weight >= target_weight;
		let diff_abs = block_weight.max(target_weight) - block_weight.min(target_weight);

		// defensive only, a test case assures that the maximum weight diff can fit in Multiplier
		// without any saturation.
		let diff = Multiplier::saturating_from_rational(diff_abs, normal_max_weight.max(1));
		let diff_squared = diff.saturating_mul(diff);

		let v_squared_2 = v.saturating_mul(v) / Multiplier::saturating_from_integer(2);

		let first_term = v.saturating_mul(diff);
		let second_term = v_squared_2.saturating_mul(diff_squared);

		if positive {
			let excess = first_term.saturating_add(second_term).saturating_mul(previous);
			previous.saturating_add(excess).max(min_multiplier)
		} else {
			// Defensive-only: first_term > second_term. Safe subtraction.
			let negative = first_term.saturating_sub(second_term).saturating_mul(previous);
			previous.saturating_sub(negative).max(min_multiplier)
		}
	}
}
