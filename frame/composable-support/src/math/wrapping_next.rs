use sp_runtime::traits::One;

/// An object from which we can derive a second object of the same type.
/// This function cannot fail and might return the same object if a boundary is about to be crossed.
// This kind of function is usually called an Endomorphism. But let's keep it simple.
pub trait WrappingNext {
	/// pallet must be coded that way that wrapping around does not do harm except of error
	/// so additional check should be check on pallet level
	fn next(&self) -> Self;
}

impl<T> WrappingNext for T
where
	T: Copy + One + num_traits::WrappingAdd,
{
	fn next(&self) -> Self {
		self.wrapping_add(&T::one())
	}
}
