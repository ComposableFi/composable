use super::{Isomorphism, NonEmptyExt};
use prost::Message;

/// A generic definition of a protocol message with an Ok and Err variants.
#[derive(Clone, PartialEq, Message)]
pub struct ResultMessage<T: Default + Send + Sync + Message, E: Default + Send + Sync + Message> {
	#[prost(oneof = "ResultEnum", tags = "1, 2")]
	pub result: core::option::Option<ResultEnum<T, E>>,
}

/// Nested enum type in [`ResultMessage`].
#[derive(Clone, PartialEq, prost::Oneof)]
pub enum ResultEnum<T: Default + Send + Sync + Message, E: Default + Send + Sync + Message> {
	#[prost(message, tag = "1")]
	Ok(T),
	#[prost(message, tag = "2")]
	Err(E),
}

impl<T, E> Isomorphism for Result<T, E>
where
	T: Isomorphism,
	T::Message: Default + Send + Sync + TryInto<T> + From<T>,
	E: Isomorphism,
	E::Message: Default + Send + Sync + TryInto<E> + From<E>,
{
	type Message = ResultMessage<T::Message, E::Message>;
}

impl<T: Isomorphism, E: Isomorphism> TryFrom<ResultMessage<T::Message, E::Message>>
	for Result<T, E>
{
	type Error = ();
	fn try_from(result: ResultMessage<T::Message, E::Message>) -> Result<Self, Self::Error> {
		match result.result.non_empty()? {
			ResultEnum::Ok(ok) => Ok(Ok(ok.try_into().map_err(|_| ())?)),
			ResultEnum::Err(ok) => Ok(Err(ok.try_into().map_err(|_| ())?)),
		}
	}
}

impl<T: Isomorphism, E: Isomorphism> From<Result<T, E>> for ResultMessage<T::Message, E::Message> {
	fn from(result: Result<T, E>) -> Self {
		let result = match result {
			Ok(ok) => ResultEnum::Ok(ok.into()),
			Err(err) => ResultEnum::Err(err.into()),
		};
		Self { result: Some(result) }
	}
}

#[test]
fn test_encoding() {
	let want = Result::<String, String>::Ok("ok".into());
	let encoded = want.clone().encode();
	let got = Result::<String, String>::decode(encoded.as_slice()).unwrap();
	assert_eq!(want, got);

	let want = Result::<String, String>::Err("err".into());
	let encoded = want.clone().encode();
	let got = Result::<String, String>::decode(encoded.as_slice()).unwrap();
	assert_eq!(want, got);
}
