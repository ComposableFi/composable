use core::num::NonZeroU64;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	RuntimeDebug,
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
)]
/// Represents a [rational number], with 64 bit numbers for the numerator and denominator.
///
/// [rational number]: https://en.wikipedia.org/wiki/Rational_number
pub struct Rational64 {
	n: u64,
	d: NonZeroU64,
}

impl Rational64 {
	pub const fn new(n: u64, d: u64) -> Option<Self> {
		match NonZeroU64::new(d) {
			Some(d) => Some(Self { n, d }),
			None => None,
		}
	}

	pub const fn n(&self) -> u64 {
		self.n
	}

	pub const fn d(&self) -> NonZeroU64 {
		self.d
	}
}

/// Used to construct a [`Rational64`] from literals. This will fail at compile time if the
/// denominator is zero.
///
/// # Example
///
/// ```rust
/// # use composable_support::{rational_64, types::rational::Rational64};
/// const VALID: Rational64 = rational_64!(100 / 1);
/// ```
///
/// ```rust,compile_fail
/// # use composable_support::{rational_64, types::rational::Rational64};
/// const INVALID: Rational64 = rational_64!(100 / 0);
/// ```
#[macro_export]
macro_rules! rational_64 {
	($n:literal / $d:literal) => {{
		const RATIONAL64: $crate::types::rational::Rational64 =
			match $crate::types::rational::Rational64::new($n, $d) {
				Some(rational) => rational,
				None => panic!("denominator cannot be zero"),
			};
		RATIONAL64
	}};
}

impl Rational64 {
	/// The smallest representation of `1` as a ratio; `1:1`.
	pub const ONE: Self = rational_64!(1 / 1);
}

#[cfg(test)]
mod test {
	use frame_support::assert_err;

	use super::*;

	mod encode {
		use super::*;

		#[test]
		fn encode_is_same_as_u128() {
			// asserts that Rational64 encodes the same as a u128 built from the numerator and
			// denominator (in little endian, since that's the encoding that SCALE uses)

			let rational = rational_64!(0x01_02_03_04 / 0x05_06_07_08);

			let rational_encoded = rational.encode();

			let u128_representation = u128::from_le_bytes(
				[0x01_02_03_04_u64.to_le_bytes(), 0x05_06_07_08_u64.to_le_bytes()]
					.concat()
					.try_into()
					.expect("8 + 8 = 16"),
			);

			let u128_representation_encoded = u128_representation.encode();

			assert_eq!(rational_encoded, u128_representation_encoded);
		}

		#[test]
		fn max_encoded_len_same_as_u128() {
			assert_eq!(u128::max_encoded_len(), Rational64::max_encoded_len())
		}
	}

	mod decode {
		use super::*;

		#[test]
		fn all_zeros_fails() {
			// asserts that decoding fails when both n and d are zero (i.e. the entirety of the
			// input is zeros)

			// 0:0
			let bytes = [0_u8; 16];

			assert_err!(
				<Rational64 as Decode>::decode(&mut bytes.as_slice()),
				// codec errors aren't typed so this is the best we can do
				codec::Error::from("cannot create non-zero number from 0")
					.chain("Could not decode `Rational64::d`")
			);
		}

		#[test]
		fn zero_denominator_fails() {
			// asserts that decoding fails when d is zero

			// 1:0
			let bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

			assert_err!(
				<Rational64 as Decode>::decode(&mut bytes.as_slice()),
				// codec errors aren't typed so this is the best we can do
				codec::Error::from("cannot create non-zero number from 0")
					.chain("Could not decode `Rational64::d`")
			);
		}
	}
}
