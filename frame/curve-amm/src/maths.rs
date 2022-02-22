use sp_runtime::{biguint::BigUint, helpers_128bit::to_big_uint, ArithmeticError, DispatchError};

fn safe_div(a: &mut BigUint, b: &mut BigUint) -> Result<BigUint, DispatchError> {
	a.lstrip();
	b.lstrip();
	// sp_std::if_std! {
	// 	println!("a {:?}", a);
	// 	println!("b {:?}", b);
	// }
	let a = a.clone();
	if b.len() == 1 {
		return Ok(a.div_unit(b.get(0)))
	}
	let div = a.div(&b, false).ok_or(ArithmeticError::Overflow)?;
	Ok(div.0)
}

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
	let ann_one = ann.clone().sub(&one).map_err(|_| ArithmeticError::Underflow)?;
	// sp_std::if_std! {
	// 	println!("base_asset_amount {:?}", base_asset_amount);
	// 	println!("quote_asset_amount {:?}", quote_asset_amount);
	// }
	let mut d = sum.clone();

	let mut base_n = base_asset_amount.mul(&n);
	let mut quote_n = quote_asset_amount.mul(&n);
	for _ in 0..255 {
		let mut d_p = d.clone();
		// d_p = d_p * d / (x * n)

		let mut d_p_d = d_p.mul(&d);
		// sp_std::if_std! {
		// 	println!("d_p_d {:?}", d_p_d);
		// 	println!("base_n {:?}", base_n);
		// }
		d_p = safe_div(&mut d_p_d, &mut base_n)?;
		let mut d_p_d = d_p.mul(&d);
		// sp_std::if_std! {
		// 	println!("d_p_d {:?}", d_p_d);
		// 	println!("quote_n {:?}", quote_n);
		// }
		d_p = safe_div(&mut d_p_d, &mut quote_n)?;

		let d_prev = d.clone();

		// d = (ann * sum + d_p * n) * d / ((ann - 1) * d + (n + 1) * d_p)
		let mut term1 = ann.clone().mul(&sum).add(&d_p.clone().mul(&n)).mul(&d);
		let mut term2 = d.mul(&ann_one).add(&n.clone().add(&one).mul(&d_p));
		d = safe_div(&mut term1, &mut term2)?;

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

pub fn compute_base(new_quote: u128, amp_coeff: u128, d: u128) -> Result<u128, DispatchError> {
	let mut n = to_big_uint(2_u128);
	let two = to_big_uint(2_u128);
	let one = to_big_uint(1_u128);
	let mut d = to_big_uint(d);
	let amplification_coefficient = to_big_uint(amp_coeff);
	let mut ann = amplification_coefficient.mul(&n).mul(&n);

	// s and p are same as input base amount as pool supports only 2 assets.
	let s = to_big_uint(new_quote);
	let mut p = to_big_uint(new_quote);
	// b = s + (d / ann) -d
	// c = d^(n + 1) / (ann * n^n * p)

	let d_ann = safe_div(&mut d, &mut ann)?;
	let d_n = safe_div(&mut d, &mut n)?;
	let b = s.add(&d_ann); // substract d later
	let mut c = d_ann.mul(&d_n).mul(&d_n);
	let c = safe_div(&mut c, &mut p)?;

	let mut y = d.clone();

	// y = (y^2 + c) / (2y + b)
	for _ in 0..255 {
		let y_prev = y.clone();
		let mut term1 = y.clone().mul(&y).add(&c);
		let term2 = two.clone().mul(&y).add(&b);
		let mut term2 = term2.sub(&d).map_err(|_| ArithmeticError::Underflow)?;

		y = safe_div(&mut term1, &mut term2)?;
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
