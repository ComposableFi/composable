use num_traits::Zero;
use sp_arithmetic::{biguint, helpers_128bit::to_big_uint};

/// Fixed point division with remainder using the underlying integers (`a` and `b`) and the
/// precision (`acc`) of the fixed point implementation.
pub fn div_rem_with_acc(
	mut a: u128,
	mut b: u128,
	mut acc: u128,
) -> Result<(u128, u128), &'static str> {
	if a.is_zero() || acc.is_zero() {
		return Ok((Zero::zero(), Zero::zero()))
	}
	b = b.max(1);

	// a and acc are interchangeable by definition in this function. It always helps to assume the
	// bigger of which is being multiplied by a `0 < acc/b < 1`. Hence, a should be the bigger and
	// acc the smaller one.
	if acc > a {
		sp_std::mem::swap(&mut a, &mut acc);
	}

	// Attempt to perform the division first
	if a % b == 0 {
		a /= b;
		b = 1;
	} else if acc % b == 0 {
		acc /= b;
		b = 1;
	}

	if let Some(x) = a.checked_mul(acc) {
		// This is the safest way to go. Try it.
		let q = x / b;
		Ok((q, a - (b * q) / acc))
	} else {
		// [`to_big_uint`] strips leading zeroes
		let a_num = to_big_uint(a); // a limbs
		let b_num = to_big_uint(b); // b limbs
		let acc_num = to_big_uint(acc); // c limbs

		let mut aa = a_num.mul(&acc_num); // a + c limbs
		aa.lstrip(); // b,c < aa <= a + c limbs, since a.checked_mul(acc) failed
		let (mut q, r) = if b_num.len() == 1 {
			// PROOF: if `b_num.len() == 1` then `b` fits in one limb.
			// TODO(0xangelo): verify that the remainder for this type of division is indeed 0
			(aa.div_unit(b as biguint::Single), Zero::zero())
		} else {
			// 1 < b limbs
			// PROOF: both `aa` and `b` cannot have leading zero limbs; if length of `b` is 1,
			// the previous branch would handle. Also, if `aa` for sure has a bigger size than
			// `b`, because `a.checked_mul(acc)` has failed, hence `aa` must be at least one limb
			// bigger than `b`. In this case, returning zero is defensive-only and div should
			// always return Some.
			let (q, r) = aa.clone().div(&b_num, true).unwrap_or((Zero::zero(), Zero::zero()));
			//   ^  ^ r = b limbs
			//   | q = aa - b + 1 >= 2 limbs

			// We can't project the remainder of `aa.div` above back to the original accuracy
			// because:
			// q = (a * acc) // b
			// r = (a * acc) % b
			// a = (q * b + r) // acc >= (q * b) // acc + r // acc
			//
			// So we first compute `a_ = b * q // acc` (what we would get by multiplying the
			// underlying fixed point numbers)
			let mut bq = aa.sub(&r).expect("remainder is always less than the dividend; qed");
			//      ^^ aa limbs
			bq.lstrip(); // >= aa - 1 limbs
			let a_: u128 = if acc_num.len() == 1 {
				bq.div_unit(acc as biguint::Single)
			} else {
				// 1 < c limbs
				bq.div(&acc_num, false)
					.expect(
						"Both `bq` and `acc_num` are stripped. \
							 If `acc_num` is single-limbed, the previous branch would handle it. \
							 If `bq` isn't at least one limb bigger than `acc_num`, then ...",
					)
					.0
			}
			.try_into()
			.expect("quotient times divisor is less or equal than original value; qed");

			// Then we compute the residual
			let r = a
				.checked_sub(a_)
				.expect("remainder of division is always less than or equal to the dividend; qed");

			(q, r)
		};
		q.lstrip();
		Ok((q.try_into().map_err(|_| "result cannot fit in u128")?, r))
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn it_works() {
		let result =
			div_rem_with_acc(10000000000000000000, 2000000000000000000, 1000000000000000000);
		assert_eq!(result, Ok((5000000000000000000, 0)));
	}
}

#[cfg(kani)]
pub mod proofs {
	use crate::*;
	use sp_arithmetic::biguint::{Single, Double};

	#[kani::proof]
	fn tautology() {
		let a: bool = kani::any();
		assert!(a || !a);
	}

	#[kani::proof]
	fn div_unit_has_zero_rem() {
		let a: u128 = kani::any();
		let b: Single = kani::any();

		let big_a = to_big_uint(a); // already strips
		let big_b = to_big_uint(b.into());

		let q = big_a.clone().div_unit(b.clone()); // clone to move as mutable
		let qb = big_b.mul(&q);
		assert_eq!(big_a - qb, Zero::zero());
	}

	// #[kani::proof]
	// fn limb_assumptions() {
	// 	let a: u128 = kani::any();
	// 	let b: u128 = kani::any();
	// 	let acc: u128 = kani::any();

	// 	kani::assume(b > 0);
	// 	kani::assume(acc > 0);
	// 	kani::assume(a.checked_mul(acc).is_none());

	// 	let a_num = to_big_uint(a); // a limbs
	// 	let b_num = to_big_uint(b); // b limbs
	// 	let acc_num = to_big_uint(acc); // c limbs

	// 	let mut aa = a_num.mul(&acc_num); // a + c limbs
	// 	aa.lstrip(); // b,c < aa <= a + c limbs, since a.checked_mul(acc) failed

	// 	assert!(aa.len() > b_num.len());
	// 	assert!(aa.len() > acc_num.len());

	// 	// if b_num.len() == 1 {
	// 	// 	let (mut q, r) = {
	// 	// 		// PROOF: if `b_num.len() == 1` then `b` fits in one limb.
	// 	// 		// TODO(0xangelo): verify that the remainder for this type of division is indeed 0
	// 	// 		(aa.div_unit(b as biguint::Single), Zero::zero())
	// 	// 	};
	// 	// }
	// }

	// #[kani::proof]
	// fn doesnt_panic() {
	// 	div_rem_with_acc(kani::any(), 2000000000000000000, 1000000000000000000);
	// }
}
