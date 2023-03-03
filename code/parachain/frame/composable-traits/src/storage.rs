use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Semantic abstraction of `Option` for updating storage items in monolithic interfaces where not
/// all fields need to be updated.
#[derive(Decode, Encode, Debug, Clone, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum UpdateValue<T> {
	/// Value will **not** be set/updated - will remain the same.
	Ignore,
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
	/// ```ignore
	/// let x: UpdateValue<u32> = Ignore;
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
			UpdateValue::Ignore => Default::default(),
		}
	}
}

impl<T> From<Option<T>> for UpdateValue<T> {
	fn from(value: Option<T>) -> Self {
		match value {
			Some(x) => UpdateValue::Set(x),
			None => UpdateValue::Ignore,
		}
	}
}

impl<T> const Default for UpdateValue<T> {
	fn default() -> Self {
		Self::Ignore
	}
}
