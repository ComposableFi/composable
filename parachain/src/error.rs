use ibc::core::ics02_client;
use sp_runtime::traits::BlakeTwo256;
use sp_trie::TrieError;
use std::num::ParseIntError;
use ibc::timestamp::ParseTimestampError;
use thiserror::Error;

/// Error definition for the parachain client
#[derive(Error, Debug)]
pub enum Error {
	/// An error from the rpc interface
	#[error("Rpc client error: {0}")]
	RpcError(String),
	/// Scale codec error
	#[error("Scale decoding error: {0}")]
	Codec(#[from] codec::Error),
	/// Update pallet name in call definition
	#[error("Pallet '{0}' not found in metadata, update static definition of call")]
	PalletNotFound(&'static str),
	/// Call not found, update function name in call definition
	#[error("Call '{0}' not found in metadata, update static definition of call")]
	CallNotFound(&'static str),
	/// subxt error
	#[error("Subxt error: {0:?}")]
	Subxt(#[from] subxt::BasicError),
	/// subxt rpc error
	#[error("Rpc threw an error")]
	SubxtRRpc(#[from] subxt::rpc::RpcError),
	/// hex error
	#[error("Error decoding hex: {0:?}")]
	Hex(#[from] hex::FromHexError),
	/// Trie error
	#[error("Trie proof generation error")]
	TrieProof(#[from] Box<TrieError<sp_trie::LayoutV0<BlakeTwo256>>>),
	/// Custom error
	#[error("{0}")]
	Custom(String),
	#[error("Ibc channel error")]
	IbcChannel(#[from] ibc::core::ics04_channel::error::Error),
	/// Error querying packets
	#[error("Could not retrieve packets from {channel_id}/{port_id} for sequences {:?}", .sequences)]
	QueryPackets { channel_id: String, port_id: String, sequences: Vec<u64>, err: String },
	/// Failed to rehydrate client state
	#[error("Error decoding some value: {0}")]
	ClientStateRehydration(String),
	/// Failed to get client update header from finality notification
	#[error("Error constructing a client update header: {0}")]
	HeaderConstruction(String),
	/// Errors associated with ics-02 client
	#[error("Ibc client error: {0}")]
	IbcClient(#[from] ics02_client::error::Error),
	/// Errors associated with beefy
	#[error("Beefy error: {0:?}")]
	BeefyProver(beefy_prover::error::Error),
	/// parse error
	#[error("Failed to parse block numbers: {0}")]
	ParseIntError(#[from] ParseIntError),
	/// Ics-20 errors
	#[error("Ics-20 error: {0}")]
	Ics20Error(#[from] ibc::applications::transfer::error::Error),
	/// Error occured parsing timestamp
	#[error("Timestamp error: {0}")]
	ParseTimestamp(#[from] ParseTimestampError)
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}
