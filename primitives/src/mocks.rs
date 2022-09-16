#[cfg(test)]
mod tests {
	use crate::{Chain, Error, IbcProvider, KeyProvider, Stream, UpdateType};
	use ibc::{
		applications::transfer::PrefixedCoin,
		core::{
			ics02_client::{client_type::ClientType, header::AnyHeader},
			ics23_commitment::commitment::CommitmentPrefix,
			ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		},
		events::IbcEvent,
		signer::Signer,
		timestamp::Timestamp,
		Height,
	};
	use ibc_proto::{
		google::protobuf::Any,
		ibc::core::{
			channel::v1::{
				QueryChannelResponse, QueryChannelsResponse, QueryNextSequenceReceiveResponse,
				QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
				QueryPacketReceiptResponse,
			},
			client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
			connection::v1::QueryConnectionResponse,
		},
	};
	use ibc_rpc::PacketInfo;
	use std::{collections::HashMap, pin::Pin, time::Duration};

	struct MockChain {
		timestamps: HashMap<u64, u64>,
	}

	impl MockChain {
		fn new(timestamps: HashMap<u64, u64>) -> Self {
			Self { timestamps }
		}
	}

	#[async_trait::async_trait]
	impl IbcProvider for MockChain {
		type FinalityEvent = ();
		type Error = Error;

		async fn query_latest_ibc_events<T>(
			&mut self,
			_finality_event: Self::FinalityEvent,
			_counterparty: &T,
		) -> Result<(AnyHeader, Vec<IbcEvent>, UpdateType), Self::Error>
		where
			T: Chain,
			Self::Error: From<crate::Error>,
		{
			todo!()
		}

		async fn query_client_consensus(
			&self,
			_at: Height,
			_client_id: ClientId,
			_consensus_height: Height,
		) -> Result<QueryConsensusStateResponse, Self::Error> {
			todo!()
		}

		async fn query_client_state(
			&self,
			_at: Height,
			_client_id: ClientId,
		) -> Result<QueryClientStateResponse, Self::Error> {
			todo!()
		}

		async fn query_connection_end(
			&self,
			_at: Height,
			_connection_id: ConnectionId,
		) -> Result<QueryConnectionResponse, Self::Error> {
			todo!()
		}

		async fn query_channel_end(
			&self,
			_at: Height,
			_channel_id: ChannelId,
			_port_id: PortId,
		) -> Result<QueryChannelResponse, Self::Error> {
			todo!()
		}

		async fn query_proof(
			&self,
			_at: Height,
			_keys: Vec<Vec<u8>>,
		) -> Result<Vec<u8>, Self::Error> {
			todo!()
		}

		async fn query_packet_commitment(
			&self,
			_at: Height,
			_port_id: &PortId,
			_channel_id: &ChannelId,
			_seq: u64,
		) -> Result<QueryPacketCommitmentResponse, Self::Error> {
			todo!()
		}

		async fn query_packet_acknowledgement(
			&self,
			_at: Height,
			_port_id: &PortId,
			_channel_id: &ChannelId,
			_seq: u64,
		) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
			todo!()
		}

		async fn query_next_sequence_recv(
			&self,
			_at: Height,
			_port_id: &PortId,
			_channel_id: &ChannelId,
		) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
			todo!()
		}

		async fn query_packet_receipt(
			&self,
			_at: Height,
			_port_id: &PortId,
			_channel_id: &ChannelId,
			_seq: u64,
		) -> Result<QueryPacketReceiptResponse, Self::Error> {
			todo!()
		}

		async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
			todo!()
		}

		async fn query_packet_commitments(
			&self,
			_at: Height,
			_channel_id: ChannelId,
			_port_id: PortId,
		) -> Result<Vec<u64>, Self::Error> {
			todo!()
		}

		async fn query_packet_acknowledgements(
			&self,
			_at: Height,
			_channel_id: ChannelId,
			_port_id: PortId,
		) -> Result<Vec<u64>, Self::Error> {
			todo!()
		}

		async fn query_unreceived_packets(
			&self,
			_at: Height,
			_channel_id: ChannelId,
			_port_id: PortId,
			_seqs: Vec<u64>,
		) -> Result<Vec<u64>, Self::Error> {
			todo!()
		}

		async fn query_unreceived_acknowledgements(
			&self,
			_at: Height,
			_channel_id: ChannelId,
			_port_id: PortId,
			_seqs: Vec<u64>,
		) -> Result<Vec<u64>, Self::Error> {
			todo!()
		}

		fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)> {
			todo!()
		}

		async fn query_connection_channels(
			&self,
			_at: Height,
			_connection_id: &ConnectionId,
		) -> Result<QueryChannelsResponse, Self::Error> {
			todo!()
		}

		async fn query_send_packets(
			&self,
			_channel_id: ChannelId,
			_port_id: PortId,
			_seqs: Vec<u64>,
		) -> Result<Vec<PacketInfo>, Self::Error> {
			todo!()
		}

		async fn query_recv_packets(
			&self,
			_channel_id: ChannelId,
			_port_id: PortId,
			_seqs: Vec<u64>,
		) -> Result<Vec<PacketInfo>, Self::Error> {
			todo!()
		}

		fn expected_block_time(&self) -> Duration {
			todo!()
		}

		async fn query_client_update_time_and_height(
			&self,
			_client_id: ClientId,
			_client_height: Height,
		) -> Result<(Height, Timestamp), Self::Error> {
			todo!()
		}

		async fn query_host_consensus_state_proof(
			&self,
			_height: Height,
		) -> Result<Option<Vec<u8>>, Self::Error> {
			todo!()
		}

		async fn query_ibc_balance(&self) -> Result<Vec<PrefixedCoin>, Self::Error> {
			todo!()
		}

		fn connection_prefix(&self) -> CommitmentPrefix {
			todo!()
		}

		fn client_id(&self) -> ClientId {
			todo!()
		}

		fn client_type(&self) -> ClientType {
			todo!()
		}

		async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
			Ok(*self.timestamps.get(&block_number).expect("Timestamp should exist in test"))
		}

		async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
			todo!()
		}

		async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
			todo!()
		}
	}

	#[async_trait::async_trait]
	impl Chain for MockChain {
		fn name(&self) -> &str {
			todo!()
		}

		async fn block_max_weight(&self) -> u64 {
			todo!()
		}

		async fn estimate_weight(&self, _msg: Vec<Any>) -> u64 {
			todo!()
		}

		async fn finality_notifications(
			&self,
		) -> Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>> {
			todo!()
		}

		async fn submit(&self, _messages: Vec<Any>) -> Result<(), Self::Error> {
			todo!()
		}
	}

	impl KeyProvider for MockChain {
		fn account_id(&self) -> Signer {
			todo!()
		}
	}

	#[tokio::test]
	async fn should_match_first_block_height_that_meets_requirement() {}

	#[tokio::test]
	async fn should_find_exact_match_if_it_exists() {}

	#[tokio::test]
	async fn should_exit_quickly_when_mid_matches_criteria() {}

	#[tokio::test]
	async fn should_return_none_if_no_match_is_found() {}
}
