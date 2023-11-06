use crate::network::Network;

pub trait Protocol<N: Network> {
	type Error;
	fn serialize(&self) -> Result<N::EncodedCall, Self::Error>;
}
