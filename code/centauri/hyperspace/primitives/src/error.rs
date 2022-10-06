use thiserror::Error;

#[derive(Error, Debug)]
/// Error definition for the relayer
pub enum Error {
	/// subxt error
	#[error("Subxt basic error")]
	Subxt(#[from] subxt::Error),
	/// subxt rpc error
	#[error("Subxt rpc error")]
	SubxtRRpc(#[from] subxt::error::RpcError),
	/// Custom error
	#[error("{0}")]
	Custom(String),
	/// Scale codec error
	#[error("Scale decoding error")]
	Codec(#[from] codec::Error),
	/// Ibc client error
	#[error("Ibc client error")]
	IbcClientError(#[from] ibc::core::ics02_client::error::Error),
	#[error("Ibc channel error")]
	IbcChannelError(#[from] ibc::core::ics04_channel::error::Error),
	#[error("Ibc connection error")]
	IbcConnectionError(#[from] ibc::core::ics03_connection::error::Error),
	#[error("Ibc proof error")]
	IbcProofError(#[from] ibc::proofs::ProofError),
	#[error("Hex decode error")]
	HexDecode(#[from] hex::FromHexError),
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}
