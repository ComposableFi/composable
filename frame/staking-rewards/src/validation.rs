use composable_support::validation::Validate;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::Permill;

#[derive(Debug, Copy, Clone, Decode, PartialEq, Eq, TypeInfo)]
pub struct ValidSplitRatio;

impl Validate<Permill, ValidSplitRatio> for ValidSplitRatio {
	fn validate(input: Permill) -> Result<Permill, &'static str> {
		if input.is_zero() || input.is_one() {
			return Err("INVALID_SPLIT_RATIO")
		}
		Ok(input)
	}
}
