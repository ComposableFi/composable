use ibc::core::ics02_client;

use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_primitives::types::{
	blocks::RpcBlockError, query::RpcQueryError, transactions::RpcTransactionError,
	validator::RpcValidatorError,
};
use std::io;
use thiserror::Error;

/// Near RPC error
#[derive(Error, Debug)]
pub enum RpcError {
	#[error("{0}")]
	Query(#[from] JsonRpcError<RpcQueryError>),
	#[error("{0}")]
	Transaction(#[from] JsonRpcError<RpcTransactionError>),
	#[error("{0}")]
	Block(#[from] JsonRpcError<RpcBlockError>),
	#[error("{0}")]
	Validator(#[from] JsonRpcError<RpcValidatorError>),
}

/// Error definition for the NEAR client
#[derive(Error, Debug)]
pub enum Error {
	/// Borsh de/serialization error
	#[error("Borsh error: {0}")]
	Borsh(#[from] io::Error),
	/// Json de/serialization error
	#[error("Json error: {0}")]
	Json(#[from] serde_json::Error),
	/// RPC error
	#[error("RPC error: {0}")]
	Rpc(#[from] RpcError),
	/// Update pallet name in call definition
	#[error("Pallet '{0}' not found in metadata, update static definition of call")]
	PalletNotFound(&'static str),
	/// hex error
	#[error("Error decoding hex: {0:?}")]
	Hex(#[from] hex::FromHexError),
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
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		Self::Custom(error)
	}
}

impl<T> From<JsonRpcError<T>> for Error
where
	RpcError: From<JsonRpcError<T>>,
{
	fn from(error: JsonRpcError<T>) -> Self {
		Self::Rpc(RpcError::from(error))
	}
}
