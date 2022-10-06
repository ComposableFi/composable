#[cfg(feature = "testing")]
use actix::dev::Stream;
#[cfg(feature = "testing")]
use std::pin::Pin;

use super::error::Error;
use crate::Client;
use ibc::{
	core::{
		ics02_client::{
			client_consensus::AnyConsensusState, client_state::AnyClientState, header::AnyHeader,
		},
		ics04_channel::packet::Packet,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	Height,
};
use ibc_proto::ibc::core::{
	channel::v1::{
		Packet as RawPacket, QueryChannelResponse, QueryNextSequenceReceiveResponse,
		QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
		QueryPacketReceiptResponse,
	},
	client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
	connection::v1::QueryConnectionResponse,
};
use near_indexer::StreamerMessage;
use near_jsonrpc_primitives::types::{
	blocks::RpcBlockRequest,
	query::{QueryResponseKind, RpcQueryRequest},
	validator::RpcValidatorRequest,
};
use near_primitives::{
	types::{BlockId, BlockReference, EpochReference, Finality, FunctionArgs},
	views::QueryRequest,
};
use near_sdk::BlockHeight;
use primitives::{Chain, IbcProvider, UpdateType};
use serde::{de::DeserializeOwned, Serialize};

impl Client {
	fn make_contract_query_at<T: Serialize>(
		&self,
		at: BlockHeight,
		method: impl ToString,
		args: &T,
	) -> Result<RpcQueryRequest, <Self as IbcProvider>::Error> {
		Ok(RpcQueryRequest {
			block_reference: BlockReference::BlockId(BlockId::Height(at)),
			request: QueryRequest::CallFunction {
				account_id: self.contract_id.clone(),
				method_name: method.to_string(),
				args: FunctionArgs::from(serde_json::to_vec(args)?),
			},
		})
	}

	fn make_contract_query_at_final<T: Serialize>(
		&self,
		method: impl ToString,
		args: &T,
	) -> Result<RpcQueryRequest, <Self as IbcProvider>::Error> {
		Ok(RpcQueryRequest {
			block_reference: BlockReference::Finality(Finality::Final),
			request: QueryRequest::CallFunction {
				account_id: self.contract_id.clone(),
				method_name: method.to_string(),
				args: FunctionArgs::from(serde_json::to_vec(args)?),
			},
		})
	}

	async fn send_query<R: DeserializeOwned>(
		&self,
		query: RpcQueryRequest,
	) -> Result<R, <Self as IbcProvider>::Error> {
		self.rpc_client
			.call(query)
			.await
			.map_err(|e| e.into())
			.and_then(|resp| match resp.kind {
				QueryResponseKind::CallResult(res) =>
					serde_json::from_slice(&res.result).map_err(|e| e.into()),
				_ => unreachable!(),
			})
	}
}

#[async_trait::async_trait]
impl IbcProvider for Client {
	type IbcEvent = Result<Vec<IbcEvent>, String>;
	type FinalityEvent = StreamerMessage;
	type Error = Error;

	async fn client_update_header<C>(
		&mut self,
		_finality_event: Self::FinalityEvent,
		_counterparty: &C,
	) -> Result<(AnyHeader, AnyClientState, UpdateType), Self::Error>
	where
		C: Chain,
		Self::Error: From<C::Error>,
	{
		unimplemented!()
	}

	async fn query_latest_ibc_events(
		&mut self,
		_header: &AnyHeader,
		_client_state: &AnyClientState,
	) -> Result<Vec<IbcEvent>, Self::Error> {
		unimplemented!()
	}

	async fn host_consensus_state(
		&self,
		_height: Height,
	) -> Result<AnyConsensusState, Self::Error> {
		unimplemented!()
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let args = (consensus_height, client_id, false);
		let query =
			self.make_contract_query_at(at.revision_height, "query_client_consensus_state", &args)?;
		self.send_query(query).await
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let args = (client_id,);
		let query = self.make_contract_query_at(at.revision_height, "query_client_state", &args)?;
		self.send_query(query).await
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let args = (connection_id,);
		let query = self.make_contract_query_at(at.revision_height, "query_connection", &args)?;
		self.send_query(query).await
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let args = (channel_id, port_id);
		let query = self.make_contract_query_at(at.revision_height, "query_channel", &args)?;
		self.send_query(query).await
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let args = (keys,);
		let query = self.make_contract_query_at(at.revision_height, "query_proof", &args)?;
		self.send_query(query).await
	}

	async fn query_packets(
		&self,
		_at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seqs: Vec<u64>,
	) -> Result<Vec<Packet>, Self::Error> {
		let args = (port_id, channel_id, &seqs);
		let query = self.make_contract_query_at_final("query_packets", &args)?;
		let packets: Vec<RawPacket> =
			self.send_query(query).await.map_err(|e| Error::QueryPackets {
				channel_id: channel_id.to_string(),
				port_id: port_id.to_string(),
				sequences: seqs,
				err: e.to_string(),
			})?;
		let packets = packets
			.into_iter()
			.map(|raw_packet| raw_packet.try_into())
			.collect::<Result<Vec<Packet>, _>>()?;
		Ok(packets)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let args = (port_id, channel_id, seq);
		let query =
			self.make_contract_query_at(at.revision_height, "query_packet_commitment", &args)?;
		self.send_query(query).await
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let args = (port_id, channel_id, seq);
		let query =
			self.make_contract_query_at(at.revision_height, "query_packet_acknowledgement", &args)?;
		self.send_query(query).await
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let args = (port_id, channel_id);
		let query =
			self.make_contract_query_at(at.revision_height, "query_next_seq_recv", &args)?;
		self.send_query(query).await
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let args = (port_id, channel_id, seq);
		let query =
			self.make_contract_query_at(at.revision_height, "query_packet_receipt", &args)?;
		self.send_query(query).await
	}

	async fn latest_height_and_timestamp(&self) -> Result<Height, Self::Error> {
		let finalized_block = self
			.rpc_client
			.call(RpcBlockRequest { block_reference: BlockReference::Finality(Finality::Final) })
			.await?;
		let validator_response = self
			.rpc_client
			.call(RpcValidatorRequest {
				epoch_reference: EpochReference::BlockId(BlockId::Height(
					finalized_block.header.height,
				)),
			})
			.await?;
		if validator_response.epoch_start_height > finalized_block.header.height {
			return Err(Error::Custom(
				"validator epoch height is greater than finalized header epoch height".to_string(),
			))
		}
		let epoch = validator_response.epoch_height;
		let latest_height = finalized_block.header.height;
		Ok(Height::new(epoch, latest_height.into()))
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.commitment_prefix.clone()).expect("Should not fail")
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	#[cfg(feature = "testing")]
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + Sync>> {
		todo!()
	}
}
