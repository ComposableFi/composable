use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Semantic abstraction of `Option` for updating storage items in monolithic interfaces where not
/// all fields need updated.
#[derive(Decode, Encode, Debug, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum UpdateValue<T> {
	/// Value will **not** be set/updated - will remain the same.
	DoNotSet,
	/// Value will be set/updated - will be changed.
	Set(T),
}

impl<T> UpdateValue<T> {
	/// Returns the contained [`Set`] value or a default.
	///
	/// Consumes the `self` argument then, if [`Set`], returns the contained
	/// value, otherwise if [`DoNotSet`], returns the [default value] for that
	/// type.
	///
	/// # Examples
	///
	/// ```
	/// let x: UpdateValue<u32> = DoNotSet;
	/// let y: UpdateValue<u32> = Set(12);
	///
	/// assert_eq!(x.unwrap_or_default(), 0);
	/// assert_eq!(y.unwrap_or_default(), 12);
	/// ```
	pub fn unwrap_or_default(self) -> T
	where
		T: Default,
	{
		match self {
			UpdateValue::Set(x) => x,
			UpdateValue::DoNotSet => Default::default(),
		}
	}
}

impl<T> From<Option<T>> for UpdateValue<T> {
	fn from(value: Option<T>) -> Self {
		match value {
			Some(x) => UpdateValue::Set(x),
			None => UpdateValue::DoNotSet,
		}
	}
}

impl<T> const Default for UpdateValue<T> {
	fn default() -> Self {
		Self::DoNotSet
	}
}
