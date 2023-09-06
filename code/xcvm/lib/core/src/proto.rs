use prost::Message as _;

pub mod common;
pub mod pb;
pub mod xcvm;

/// Defines an isomorphism between a Rust type `Self` and a protocol message.
///
/// With the isomorphism defined, provides functions to decode and encode the
/// type from binary representation of the protobuf.
pub trait Isomorphism: Sized {
	/// Protobuf self is isomorphic with.
	type Message: prost::Message;

	/// Converts object to protobuf and encodes it as byte vector.
	fn encode(self) -> alloc::vec::Vec<u8>
	where
		Self::Message: From<Self>,
	{
		Self::Message::encode_to_vec(&self.into())
	}

	/// Decodes a protobuf and then converts it to `T`.
	fn decode(buffer: &[u8]) -> Result<Self, prost::DecodeError>
	where
		Self::Message: Default + Into<Self>,
	{
		Self::Message::decode(buffer).map(|msg| msg.into())
	}

	/// Decodes a protobuf and then tries to convert it to `T`.
	fn try_decode(buffer: &[u8]) -> Result<Self, DecodeError>
	where
		Self::Message: Default + TryInto<Self>,
	{
		Self::Message::decode(buffer)?
			.try_into()
			.map_err(|_| DecodeError::BadIsomorphism)
	}
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
