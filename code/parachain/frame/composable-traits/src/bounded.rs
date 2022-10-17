// we have bounds for FixedU128(really integer U128, so with serde of decimal point):
// [0..1]
// [0..1)
// (0..1]
// [1..MAX]
// (1..MAX]
//	..
//
// for integer we have
// 1 to 100, 1000, 1000
// u32 from 1 to 32 (decimals exponent)

// cannot make const generic because `the type must not depend on the parameter`
// there is no solution in crates which works with substrate numbers in substrate context
trait Bound<
	Inner: PartialOrd + Encode + Decode + MaxEncodedLen + TypeInfo + MaybeSerializeDeserialize + Sized,
>
{
	const MIN: Inner;
	const MAX: Inner;
	//  rust prevents `functions in traits cannot be const`, so cannot validate like that
	/* const  fn validate() -> bool  {
		Self::MIN < Self::MAX
	}
	*/
}

/* possible macro (similar to what Rust will allow to do on CONST parameters)
bounded_num! {
	struct BoundedRatio(FixedU128);
	// impl deref, into/from, transparent serialization traits
	impl Bound<FixedU128, const MIN = FixedU128::from_inner(1), const MAX = FixedU128::from_inner(u128::MAX)>
	for BounderRatio
	where
		// MAX can be INFINITY to produce unbounded upper result
		MIN < MAX
}
*/

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::MaybeSerializeDeserialize;
use scale_info::TypeInfo;
use sp_runtime::FixedU128;

struct BoundedRatio<
	Inner: PartialOrd + Encode + Decode + MaxEncodedLen + TypeInfo + MaybeSerializeDeserialize + Sized,
>(Inner);

impl Bound<FixedU128> for BoundedRatio<FixedU128> {
	/// never division by zero, so can overflow (which is division by zero)
	const MIN: FixedU128 = FixedU128::from_inner(1);
	const MAX: FixedU128 = FixedU128::from_inner(u128::MAX);
}

// cannot be generic
impl From<BoundedRatio<FixedU128>> for FixedU128 {
	fn from(this: BoundedRatio<FixedU128>) -> Self {
		this.0
	}
}
// cannot make generic encode, because `the type parameter `Inner` is not constrained by the impl
// trait, self type, or predicates`
impl<
		Inner: PartialOrd + Encode + Decode + MaxEncodedLen + TypeInfo + MaybeSerializeDeserialize + Sized,
	> Encode for BoundedRatio<Inner>
{
	fn size_hint(&self) -> usize {
		self.0.size_hint()
	}

	fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
		self.0.encode_to(dest);
	}
}

impl Decode for BoundedRatio<FixedU128> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		// NOTE: we do not use limits constraint bits size of number
		let raw = FixedU128::decode(input)?;
		let raw = Self(raw);
		if <Self as Bound<FixedU128>>::MIN >= <Self as Bound<FixedU128>>::MAX {
			return Err(codec::Error::from("constrains are not satisfied"))
		}
		Ok(raw)
	}
}
