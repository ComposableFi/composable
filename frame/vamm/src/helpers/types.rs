use crate::Config;

pub struct CalculateSwapAsset<T: Config> {
	pub output_amount: T::Balance,
	pub input_amount: T::Balance,
}
