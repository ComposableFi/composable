#![allow(unused)]

use tendermint_rpc::abci::transaction::Hash;

use ibc::core::{
	ics02_client::client_consensus::QueryClientEventRequest,
	ics04_channel::channel::QueryPacketEventDataRequest,
};

/// Used for queries and not yet standardized in channel's query.proto
#[derive(Clone, Debug)]
pub enum QueryTxRequest {
	Packet(QueryPacketEventDataRequest),
	Client(QueryClientEventRequest),
	Transaction(QueryTxHash),
}

#[derive(Clone, Debug)]
pub enum QueryBlockRequest {
	Packet(QueryPacketEventDataRequest),
}

#[derive(Clone, Debug)]
pub struct QueryTxHash(pub Hash);
