#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]
use frame_support::sp_runtime::Perbill;
use sp_runtime::{
	traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero},
	DispatchError, FixedPointNumber, FixedU128, Permill,
};

use sp_std::convert::TryFrom;

/// Describes a simple exchanges which does not allow advanced configurations such as slippage.
pub trait SimpleExchange {
	type AssetId;
	type Balance;
	type AccountId;
	type Error;

	/// Obtains the current price for a given asset, possibly routing through multiple markets.
	fn price(asset_id: Self::AssetId) -> Option<Self::Balance>;

	/// Exchange `amount` of `from` asset for `to` asset. The maximum price paid for the `to` asset
	/// is `SimpleExchange::price * (1 + slippage)`
	fn exchange(
		from: Self::AssetId,
		from_account: Self::AccountId,
		to: Self::AssetId,
		to_account: Self::AccountId,
		to_amount: Self::Balance,
		slippage: Perbill,
	) -> Result<Self::Balance, Self::Error>;
}

pub struct TakeResult<BALANCE> {
	pub amount: BALANCE,
	pub total_price: BALANCE,
}

/// see for examples:
/// - https://github.com/galacticcouncil/Basilisk-node/blob/master/pallets/exchange/src/lib.rs
/// - https://github.com/Polkadex-Substrate/polkadex-aura-node/blob/master/pallets/polkadex/src/lib.rs
/// expected that failed exchanges are notified by events.
pub trait Orderbook {
	type AssetId;
	type Balance;
	type AccountId;
	type OrderId;

	/// sell. exchanges specified amount of asset to other at specific price
	/// `source_price` price per unit
	/// `amm_slippage` set to zero to avoid AMM sell
	/// for remote auction we should  have sent some random to make sure we have idempotent request
	fn post(
		account_from: &Self::AccountId,
		asset: Self::AssetId,
		want: Self::AssetId,
		source_amount: Self::Balance,
		source_price: Self::Balance,
		amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError>;

	/// sell. exchanges specified amount of asset to other at market price.
	fn market_sell(
		account: &Self::AccountId,
		asset: Self::AssetId,
		want: Self::AssetId,
		amount: Self::Balance,
		amm_slippage: Permill,
	) -> Result<Self::OrderId, DispatchError>;

	/// buy
	fn take(
		account: &Self::AccountId,
		orders: impl Iterator<Item = Self::OrderId>,
		up_to: Self::Balance,
	) -> Result<TakeResult<Self::Balance>, DispatchError>;

	fn is_order_executed(order_id: &Self::OrderId) -> bool;
}

/// Implement AMM curve from "StableSwap - efficient mechanism for Stablecoin liquidity by Micheal
/// Egorov" Also blog at https://miguelmota.com/blog/understanding-stableswap-curve/ has very good explanation.

const N_COINS: usize = 2;
type CoinId = u32;

struct StableSwapPool {
	/// coins in the DEX pool.
	coins: [CoinId; N_COINS],
	/// balances of the coins in the DEX.
	balances: [FixedU128; N_COINS],
	/// initial amplification coefficient `A`
	initial_amp_coef: FixedU128,
	/// future amplification_coefficient `A`, used for ramping `A`.
	future_amp_coef: FixedU128,
	/// block time-stamp when initial_amp_coef was set.
	initial_amp_coef_time: FixedU128,
	/// block time-stamp when future_amp_coef_time was set.
	future_amp_coef_time: FixedU128,
}

impl StableSwapPool {
	pub fn new(
		coins: [CoinId; N_COINS],
		balances: [FixedU128; N_COINS],
		amplification_coefficient: FixedU128,
	) -> Self {
		StableSwapPool {
			coins,
			balances, /* TODO: should we check for any inveriants for total balances like both
			           * coins have equal initial balance? */
			initial_amp_coef: amplification_coefficient,
			future_amp_coef: amplification_coefficient,
			initial_amp_coef_time: FixedU128::zero(),
			future_amp_coef_time: FixedU128::zero(),
		}
	}

	/// Find `ann = amp * n^n` where `amp` - amplification coefficient,
	/// `n` - number of coins.
	pub fn get_ann(amp: FixedU128, n: usize) -> Option<FixedU128> {
		let n_coins = FixedU128::saturating_from_integer(u128::try_from(n).ok()?);
		let mut ann = amp;
		for _ in 0..n {
			ann = ann.checked_mul(&n_coins)?;
		}
		Some(ann)
	}

	/// Find `d` preserving StableSwap invariant.
	/// Here `d` - total amount of coins when they have an equal price,
	/// `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`,
	/// where `n` is number of coins.
	///
	/// # Notes
	///
	/// D invariant calculation in non-overflowing integer operations iteratively
	///
	/// ```pseudocode
	///  A * sum(x_i) * n^n + D = A * D * n^n + D^(n+1) / (n^n * prod(x_i))
	/// ```
	///
	/// Converging solution:
	///
	/// ```pseudocode
	/// D[j + 1] = (A * n^n * sum(x_i) - D[j]^(n+1) / (n^n * prod(x_i))) / (A * n^n - 1)
	/// ```
	pub fn get_d(xp: &[FixedU128], ann: FixedU128) -> Option<FixedU128> {
		let zero = FixedU128::zero();
		let one = FixedU128::one();
		let prec = FixedU128::from_inner(FixedU128::accuracy());
		let n = FixedU128::saturating_from_integer(u128::try_from(xp.len()).ok()?);

		let sum = xp.iter().try_fold(zero, |s, x| s.checked_add(x))?;
		if sum == zero {
			return Some(zero)
		}
		let mut d = sum;

		for _ in 0..255 {
			let mut d_p = d;
			for x in xp.iter() {
				// d_p = d_p * d / (x * n)
				d_p = d_p.checked_mul(&d)?.checked_div(&x.checked_mul(&n)?)?;
			}
			let d_prev = d;

			// d = (ann * sum + d_p * n) * d / ((ann - 1) * d + (n + 1) * d_p)
			d = ann
				.checked_mul(&sum)?
				.checked_add(&d_p.checked_mul(&n)?)?
				.checked_mul(&d)?
				.checked_div(
					&ann.checked_sub(&one)?
						.checked_mul(&d)?
						.checked_add(&n.checked_add(&one)?.checked_mul(&d_p)?)?,
				)?;

			if d > d_prev {
				if d.checked_sub(&d_prev)? <= prec {
					return Some(d)
				}
			} else if d_prev.checked_sub(&d)? <= prec {
				return Some(d)
			}
		}
		None
	}

	/// Here `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`, where
	/// `n` is number of coins.
	///
	/// See https://github.com/equilibrium-eosdt/equilibrium-curve-amm/blob/master/docs/deducing-get_y-formulas.pdf
	/// for detailed explanation about formulas this function uses.
	///
	/// # Notes
	///
	/// Done by solving quadratic equation iteratively.
	///
	/// ```pseudocode
	/// x_1^2 + x_1 * (sum' - (A * n^n - 1) * D / (A * n^n)) = D^(n+1) / (n^2n * prod' * A)
	/// x_1^2 + b * x_1 = c
	///
	/// x_1 = (x_1^2 + c) / (2 * x_1 + b)
	/// ```
	pub fn get_y(
		i: usize,
		j: usize,
		x: FixedU128,
		xp: &[FixedU128],
		ann: FixedU128,
	) -> Option<FixedU128> {
		let zero = FixedU128::zero();
		let one = FixedU128::one();
		let two = FixedU128::saturating_from_integer(u128::try_from(2).ok()?);
		let n = FixedU128::saturating_from_integer(u128::try_from(xp.len()).ok()?);

		// Same coin
		if i == j {
			return None
		}
		// j above n
		if j >= xp.len() {
			return None
		}
		if i >= xp.len() {
			return None
		}
		let d = Self::get_d(xp, ann)?;

		let mut c = d;
		let mut s = zero;

		// Calculate s and c
		// p is implicitly calculated as part of c
		// note that loop makes n - 1 iterations
		for (k, xp_k) in xp.iter().enumerate() {
			let x_k;
			if k == i {
				x_k = x;
			} else if k != j {
				x_k = *xp_k;
			} else {
				continue
			}
			// s = s + x_k
			s = s.checked_add(&x_k)?;
			// c = c * d / (x_k * n)
			c = c.checked_mul(&d)?.checked_div(&x_k.checked_mul(&n)?)?;
		}
		// c = c * d / (ann * n)
		// At this step we have d^n in the numerator of c
		// and n^(n-1) in its denominator.
		// So we multiplying it by remaining d/n
		c = c.checked_mul(&d)?.checked_div(&ann.checked_mul(&n)?)?;

		// b = s + d / ann
		// We subtract d later
		let b = s.checked_add(&d.checked_div(&ann)?)?;
		let mut y = d;

		for _ in 0..255 {
			let y_prev = y;
			// y = (y^2 + c) / (2 * y + b - d)
			// Subtract d to calculate b finally
			y = y
				.checked_mul(&y)?
				.checked_add(&c)?
				.checked_div(&two.checked_mul(&y)?.checked_add(&b)?.checked_sub(&d)?)?;

			// Equality with the specified precision
			if y > y_prev {
				if y.checked_sub(&y_prev)? <= one {
					return Some(y)
				}
			} else if y_prev.checked_sub(&y)? <= one {
				return Some(y)
			}
		}

		None
	}

	/// Here `xp` - coin amounts, `ann` is amplification coefficient multiplied by `n^n`, where
	/// `n` is number of coins.
	/// Calculate `x[i]` if one reduces `d` from being calculated for `xp` to `d`.
	///
	/// # Notes
	///
	/// Done by solving quadratic equation iteratively.
	///
	/// ```pseudocode
	/// x_1^2 + x_1 * (sum' - (A * n^n - 1) * D / (A * n^n)) = D^(n+1) / (n^2n * prod' * A)
	/// x_1^2 + b * x_1 = c
	///
	/// x_1 = (x_1^2 + c) / (2 * x_1 + b)
	/// ```
	pub fn get_y_d(i: usize, d: FixedU128, xp: &[FixedU128], ann: FixedU128) -> Option<FixedU128> {
		let prec = FixedU128::from_inner(FixedU128::accuracy());
		let zero = FixedU128::one();
		let two = FixedU128::saturating_from_integer(u128::try_from(2).ok()?);
		let n = FixedU128::saturating_from_integer(u128::try_from(xp.len()).ok()?);

		if i >= xp.len() {
			return None
		}

		let mut c = d;
		let mut s = zero;

		for (k, xp_k) in xp.iter().enumerate() {
			if k == i {
				continue
			}

			let x = xp_k;

			s = s.checked_add(x)?;
			// c = c * d / (x * n)
			c = c.checked_mul(&d)?.checked_div(&x.checked_mul(&n)?)?;
		}
		// c = c * d / (ann * n)
		c = c.checked_mul(&d)?.checked_div(&ann.checked_mul(&n)?)?;
		// b = s + d / ann
		let b = s.checked_add(&d.checked_div(&ann)?)?;
		let mut y = d;

		for _ in 0..255 {
			let y_prev = y;
			// y = (y*y + c) / (2 * y + b - d)
			y = y
				.checked_mul(&y)?
				.checked_add(&c)?
				.checked_div(&two.checked_mul(&y)?.checked_add(&b)?.checked_sub(&d)?)?;

			// Equality with the specified precision
			if y > y_prev {
				if y.checked_sub(&y_prev)? <= prec {
					return Some(y)
				}
			} else if y_prev.checked_sub(&y)? <= prec {
				return Some(y)
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_runtime::{traits::Saturating, FixedPointNumber, FixedU128};
	use sp_std::cmp::Ordering;

	#[test]
	fn compute_d_works() {
		let xp = vec![
			FixedU128::saturating_from_rational(11, 10),
			FixedU128::saturating_from_rational(88, 100),
		];
		let amp = FixedU128::saturating_from_rational(292, 100);
		let ann = StableSwapPool::get_ann(amp, xp.len()).unwrap();
		let d = StableSwapPool::get_d(&xp, ann);
		// expected d is 1.978195735374521596
		// expected precision is 1e-13
		let delta = d
			.map(|x| {
				x.saturating_sub(FixedU128::saturating_from_rational(
					1978195735374521596u128,
					10_000_000_000_000_000u128,
				))
				.saturating_abs()
			})
			.map(|x| x.cmp(&FixedU128::saturating_from_rational(1u128, 10_000_000_000_000u128)));
		assert_eq!(delta, Some(Ordering::Less));
	}

	#[test]
	fn compute_d_empty() {
		let xp = vec![];
		let amp = FixedU128::saturating_from_rational(292, 100);
		let ann = StableSwapPool::get_ann(amp, xp.len()).unwrap();
		let result = StableSwapPool::get_d(&xp, ann);
		assert_eq!(result, Some(FixedU128::zero()));
	}

	#[test]
	fn get_y_successful() {
		let i = 0;
		let j = 1;
		let x = FixedU128::saturating_from_rational(111, 100);
		let xp = vec![
			FixedU128::saturating_from_rational(11, 10),
			FixedU128::saturating_from_rational(88, 100),
		];
		let amp = FixedU128::saturating_from_rational(292, 100);
		let ann = StableSwapPool::get_ann(amp, xp.len()).unwrap();

		let result = StableSwapPool::get_y(i, j, x, &xp, ann);
		// expected y is 1.247108067356516682
		// expected precision is 1e-13
		let delta = result
			.map(|x| {
				x.saturating_sub(FixedU128::saturating_from_rational(
					1247108067356516682u128,
					10_000_000_000_000_000u128,
				))
				.saturating_abs()
			})
			.map(|x| x.cmp(&FixedU128::saturating_from_rational(1, 10_000_000_000_000u128)));
		assert_eq!(delta, Some(Ordering::Less));
	}

	#[test]
	fn get_y_same_coin() {
		let i = 1;
		let j = 1;
		let x = FixedU128::saturating_from_rational(111, 100);
		let xp = vec![
			FixedU128::saturating_from_rational(11, 10),
			FixedU128::saturating_from_rational(88, 100),
		];
		let amp = FixedU128::saturating_from_rational(292, 100);
		let ann = StableSwapPool::get_ann(amp, xp.len()).unwrap();

		let result = StableSwapPool::get_y(i, j, x, &xp, ann);

		assert_eq!(result, None);
	}

	#[test]
	fn get_y_i_greater_than_n() {
		let i = 33;
		let j = 1;
		let x = FixedU128::saturating_from_rational(111, 100);
		let xp = vec![
			FixedU128::saturating_from_rational(11, 10),
			FixedU128::saturating_from_rational(88, 100),
		];
		let amp = FixedU128::saturating_from_rational(292, 100);
		let ann = StableSwapPool::get_ann(amp, xp.len()).unwrap();

		let result = StableSwapPool::get_y(i, j, x, &xp, ann);

		assert_eq!(result, None);
	}

	#[test]
	fn get_y_j_greater_than_n() {
		let i = 1;
		let j = 33;
		let x = FixedU128::saturating_from_rational(111, 100);
		let xp = vec![
			FixedU128::saturating_from_rational(11, 10),
			FixedU128::saturating_from_rational(88, 100),
		];
		let amp = FixedU128::saturating_from_rational(292, 100);
		let ann = StableSwapPool::get_ann(amp, xp.len()).unwrap();

		let result = StableSwapPool::get_y(i, j, x, &xp, ann);

		assert_eq!(result, None);
	}
}
