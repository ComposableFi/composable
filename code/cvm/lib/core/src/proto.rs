use prost::Message as _;

pub mod common;
pub mod cvm;
pub mod pb;
pub mod result;

/// Defines an isomorphism between a Rust type `Self` and a protocol message.
///
/// With the isomorphism defined, provides functions to decode and encode the
/// type from binary representation of the protobuf.
pub trait Isomorphism: Sized {
	/// Protobuf self is isomorphic with.
	type Message: Default + From<Self> + TryInto<Self> + prost::Message;

	/// Converts object to protobuf and encodes it as byte vector.
	fn encode(self) -> alloc::vec::Vec<u8> {
		Self::Message::encode_to_vec(&self.into())
	}

	/// Decodes a protobuf and then tries to convert it to `T`.
	fn decode(buffer: &[u8]) -> Result<Self, DecodeError> {
		Self::Message::decode(buffer)?
			.try_into()
			.map_err(|_| DecodeError::BadIsomorphism)
	}
}

impl Isomorphism for alloc::string::String {
	type Message = Self;
}

impl Isomorphism for () {
	type Message = ();
}

/// Error when trying to decode protobuf into a Rust type.
#[derive(Clone, Debug, derive_more::From)]
pub enum DecodeError {
	/// Failed to decode the protocol message.
	BadProtobuf(prost::DecodeError),
	/// Protocol message doesn’t map into a Rust type.  This is often because
	/// some required fields not being set.
	BadIsomorphism,
}

impl core::fmt::Display for DecodeError {
	fn fmt(&self, fmtr: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::BadProtobuf(err) => err.fmt(fmtr),
			Self::BadIsomorphism => fmtr.write_str("Failed to convert protobuf to Rust type"),
		}
	}
}

/// Defines conversions between protocol buffer message `$pb` and Rust type
/// `$ty`.
///
/// Specifically, implements `TryFrom<$pb> for $ty` and `From<$ty> for $pb`.
/// That is, conversion from protocol message is fallible while conversion to
/// protocol message isn’t.  The error for the `TryFrom` conversion is `()`.
macro_rules! define_conversion {
	(($pb_name:ident: $pb:ty) -> { $($from_pb:tt)* }
	 ($ty_name:ident: $ty:ty) -> { $($from_ty:tt)* }) => {
		impl TryFrom<$pb> for $ty {
			type Error = ();
			fn try_from($pb_name: $pb) -> Result<Self, Self::Error> {
				$($from_pb)*
			}
		}

		impl From<$ty> for $pb {
			fn from($ty_name: $ty) -> $pb {
				$($from_ty)*
			}
		}
	}
}

use define_conversion;

/// Maps elements of one sequence and produces the other.
///
/// This is a convenience function for `Vec<T> → Vec<U>` operation (though it
/// works for any containers) where infallible `T → U` conversion exists.
fn from_sequence<R: core::iter::FromIterator<U>, T, U: From<T>>(
	sequence: impl core::iter::IntoIterator<Item = T>,
) -> R {
	sequence.into_iter().map(U::from).collect()
}

/// Tries to map elements of one sequence and produces the other.
///
/// This is a convenience function for `Vec<T> → Vec<U>` operation (though it
/// works for any containers) where fallible `T → U` conversion exists.  Returns
/// error on the first conversion that fails.
fn try_from_sequence<R: core::iter::FromIterator<U>, T, U: TryFrom<T>>(
	sequence: impl core::iter::IntoIterator<Item = T>,
) -> Result<R, ()> {
	sequence.into_iter().map(U::try_from).collect::<Result<R, _>>().map_err(|_| ())
}

/// Trait providing method which converts ‘empty’ values to ‘Err(())’.
///
/// Useful for checking whether fields in protocol buffer messages are set.
trait NonEmptyExt: Sized {
	type Output: Sized;
	fn non_empty(self) -> Result<Self::Output, ()>;
}

impl NonEmptyExt for alloc::string::String {
	type Output = Self;
	fn non_empty(self) -> Result<Self::Output, ()> {
		if self.is_empty() {
			Err(())
		} else {
			Ok(self)
		}
	}
}

impl<T> NonEmptyExt for alloc::vec::Vec<T> {
	type Output = Self;
	fn non_empty(self) -> Result<Self::Output, ()> {
		if self.is_empty() {
			Err(())
		} else {
			Ok(self)
		}
	}
}

impl<T> NonEmptyExt for Option<T> {
	type Output = T;
	fn non_empty(self) -> Result<Self::Output, ()> {
		self.ok_or(())
	}
}
