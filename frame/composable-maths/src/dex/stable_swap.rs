use sp_runtime::{biguint::BigUint, helpers_128bit::to_big_uint, ArithmeticError, DispatchError};

fn safe_div(a: &mut BigUint, b: &mut BigUint) -> Result<BigUint, DispatchError> {
	a.lstrip();
	b.lstrip();
	let a = a.clone();
	if b.len() == 1 {
		return Ok(a.div_unit(b.get(0)))
	}
	let div = a.div(b, false).ok_or(ArithmeticError::Overflow)?;
	Ok(div.0)
}

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
/// For two assets, n = 2 used while computation
pub fn compute_d(
	base_asset_aum: u128,
	quote_asset_aum: u128,
	amp_coeff: u128,
) -> Result<u128, DispatchError> {
	let base_asset_amount = to_big_uint(base_asset_aum);
	let quote_asset_amount = to_big_uint(quote_asset_aum);
	let amplification_coefficient = to_big_uint(amp_coeff);
	// pool has only 2 assets
	let n = to_big_uint(2_u128);
	let zero = to_big_uint(0_u128);
	let one = to_big_uint(1_u128);

	let sum = base_asset_amount.clone().add(&quote_asset_amount);
	if sum == zero {
		return Ok(0_u128)
	}
	let ann = amplification_coefficient.mul(&n).mul(&n);
	let mut d = sum.clone();

	let mut base_n = base_asset_amount.mul(&n);
	let mut quote_n = quote_asset_amount.mul(&n);
	for _ in 0..255 {
		let mut d_p = d.clone();
		let ann_d = ann.clone().mul(&d);
		// d_p = d_p * d / (x * n)

		let mut d_p_d = d_p.mul(&d);
		d_p = safe_div(&mut d_p_d, &mut base_n)?;
		let mut d_p_d = d_p.mul(&d);
		d_p = safe_div(&mut d_p_d, &mut quote_n)?;

		let d_prev = d.clone();
		// d = (ann * sum + d_p * n) * d / (ann * d + (n + 1) * d_p - d)
		let mut numerator = ann.clone().mul(&sum).add(&d_p.clone().mul(&n)).mul(&d);
		let mut denominator = ann_d
			.add(&n.clone().add(&one).mul(&d_p))
			.sub(&d)
			.map_err(|_| ArithmeticError::Underflow)?;
		d = safe_div(&mut numerator, &mut denominator)?;

		if d.clone() > d_prev {
			if d.clone() - d_prev <= one {
				d.lstrip();
				return Ok(d.try_into().map_err(|_| ArithmeticError::Overflow)?)
			}
		} else if d_prev - d.clone() <= one {
			d.lstrip();
			return Ok(d.try_into().map_err(|_| ArithmeticError::Overflow)?)
		}
	}
	Err(DispatchError::Other("could not compute d"))
}

/// See <https://github.com/equilibrium-eosdt/equilibrium-curve-amm/blob/master/docs/deducing-get_y-formulas.pdf>
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
/// For two assets, n = 2 used while computation
pub fn compute_base(new_quote: u128, amp_coeff: u128, d: u128) -> Result<u128, DispatchError> {
	let mut n = to_big_uint(2_u128);
	let two = to_big_uint(2_u128);
	let one = to_big_uint(1_u128);
	let mut d = to_big_uint(d);
	let amplification_coefficient = to_big_uint(amp_coeff);
	let ann = amplification_coefficient.mul(&n).mul(&n);

	// s and p are same as input base amount as pool supports only 2 assets.
	let s = to_big_uint(new_quote);
	let mut p = to_big_uint(new_quote);

	// term1 = d^(n + 1) / n^n * p
	// term2 = 2*y + s - d

	let d_n = safe_div(&mut d, &mut n)?;
	let mut c = d_n.clone().mul(&d_n).mul(&d);
	let term1 = safe_div(&mut c, &mut p)?;

	let mut y = d.clone();

	// y = (y^2 * ann + term1) / (ann * term2 + d)
	for _ in 0..255 {
		let y_prev = y.clone();
		let term2 = two.clone().mul(&y).add(&s).sub(&d).map_err(|_| ArithmeticError::Underflow)?;
		let mut numerator = ann.clone().mul(&y).mul(&y).add(&term1);
		let mut denominator = ann.clone().mul(&term2).add(&d);

		y = safe_div(&mut numerator, &mut denominator)?;
		if y.clone() > y_prev {
			if y.clone() - y_prev <= one {
				y.lstrip();
				return Ok(y.try_into().map_err(|_| ArithmeticError::Overflow)?)
			}
		} else if y_prev - y.clone() <= one {
			y.lstrip();
			return Ok(y.try_into().map_err(|_| ArithmeticError::Overflow)?)
		}
	}
	Err(DispatchError::Other("could not compute d"))
}
