use composable_support::validation::Validate;
use frame_support::{pallet_prelude::*, traits::Get};
use scale_info::TypeInfo;
use sp_runtime::{traits::Zero, Percent};

#[derive(Debug, Decode, Copy, Clone, PartialEq, Eq, TypeInfo)]
pub struct ValidMinAnswers;

#[derive(Debug, Copy, Clone, Decode, PartialEq, Eq, TypeInfo)]
pub struct ValidThreshold;

#[derive(Debug, Decode)]
pub struct ValidMaxAnswer<U> {
	pub m: PhantomData<U>,
}

impl<U> Copy for ValidMaxAnswer<U> {}

impl<U> Clone for ValidMaxAnswer<U> {
	fn clone(&self) -> Self {
		*self
	}
}

#[derive(Debug, Decode)]
pub struct ValidBlockInterval<U> {
	pub m: PhantomData<U>,
}

impl<U> Copy for ValidBlockInterval<U> {}

impl<U> Clone for ValidBlockInterval<U> {
	fn clone(&self) -> Self {
		*self
	}
}

#[derive(Debug, Decode)]
pub struct ValidAssetId<U> {
	u: PhantomData<U>,
}

impl<U> Copy for ValidAssetId<U> {}

impl<U> Clone for ValidAssetId<U> {
	fn clone(&self) -> Self {
		*self
	}
}

#[derive(Debug, Decode)]
pub struct IsRequested;

impl<MinAnswer: Zero + PartialEq + Eq + Ord + PartialOrd> Validate<MinAnswer, ValidMinAnswers>
	for ValidMinAnswers
{
	fn validate(input: MinAnswer) -> Result<MinAnswer, &'static str> {
		if input <= MinAnswer::zero() {
			return Err("INVALID_MIN_ANSWERS")
		}

		Ok(input)
	}
}

impl Validate<Percent, ValidThreshold> for ValidThreshold {
	fn validate(input: Percent) -> Result<Percent, &'static str> {
		if input >= Percent::from_percent(100) {
			return Err("INVALID_THRESHOLD")
		}

		Ok(input)
	}
}

impl<MaxAnswer: PartialEq + Eq + PartialOrd, MaxAnswerBound>
	Validate<MaxAnswer, ValidMaxAnswer<MaxAnswerBound>> for ValidMaxAnswer<MaxAnswerBound>
where
	MaxAnswerBound: Get<MaxAnswer>,
{
	fn validate(input: MaxAnswer) -> Result<MaxAnswer, &'static str> {
		if input > MaxAnswerBound::get() {
			return Err("INVALID_MAX_ANSWER")
		}

		Ok(input)
	}
}

impl<BlockInterval: PartialOrd, StalePrice> Validate<BlockInterval, ValidBlockInterval<StalePrice>>
	for ValidBlockInterval<StalePrice>
where
	StalePrice: Get<BlockInterval>,
	ValidBlockInterval<StalePrice>: Decode,
{
	fn validate(input: BlockInterval) -> Result<BlockInterval, &'static str> {
		if input <= StalePrice::get() {
			return Err("INVALID_BLOCK_INTERVAL_LENGTH")
		}

		Ok(input)
	}
}
