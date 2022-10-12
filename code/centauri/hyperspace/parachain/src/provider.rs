use codec::Encode;
use ibc::{
	core::{
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	events::IbcEvent,
	Height,
};
use std::fmt::Display;

use ibc_proto::ibc::core::{
	channel::v1::{
		QueryChannelResponse, QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
		QueryPacketCommitmentResponse, QueryPacketReceiptResponse,
	},
	client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
	connection::v1::QueryConnectionResponse,
};
use sp_runtime::{
	traits::{Header as HeaderT, IdentifyAccount, Verify},
	MultiSignature, MultiSigner,
};
use subxt::{
	tx::{AssetTip, BaseExtrinsicParamsBuilder, ExtrinsicParams},
	Config,
};

use super::{error::Error, ParachainClient};

use crate::{parachain, FinalityProtocol, GrandpaClientState};
use ibc::{
	applications::transfer::{Amount, PrefixedCoin, PrefixedDenom},
	core::ics02_client::client_state::ClientType,
	timestamp::Timestamp,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{channel::v1::QueryChannelsResponse, client::v1::IdentifiedClientState},
};
use ibc_rpc::{IbcApiClient, PacketInfo};
use primitives::{Chain, IbcProvider, KeyProvider, TransactionId, UpdateType};
use sp_core::H256;

use crate::finality_protocol::FinalityEvent;
use beefy_prover::helpers::fetch_timestamp_extrinsic_with_proof;
use futures::{future::ready, stream, Stream, StreamExt};
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ibc_proto::ibc::core::connection::v1::IdentifiedConnection;
use ics11_beefy::client_state::ClientState as BeefyClientState;
use pallet_ibc::{
	events::IbcEvent as RawIbcEvent, light_clients::HostFunctionsManager, HostConsensusProof,
};
use sp_runtime::traits::One;
use std::{collections::BTreeMap, pin::Pin, str::FromStr, time::Duration};
use subxt::events::Phase;

#[async_trait::async_trait]
impl<T: Config + Send + Sync> IbcProvider for ParachainClient<T>
where
	u32: From<<<T as Config>::Header as HeaderT>::Number>,
	u32: From<<T as Config>::BlockNumber>,
	Self: KeyProvider,
	<T::Signature as Verify>::Signer: From<MultiSigner> + IdentifyAccount<AccountId = T::AccountId>,
	MultiSigner: From<MultiSigner>,
	<T as subxt::Config>::Address: From<<T as subxt::Config>::AccountId>,
	T::Signature: From<MultiSignature>,
	T::BlockNumber: From<u32> + Display + Ord + sp_runtime::traits::Zero + One,
	T::Hash: From<sp_core::H256>,
	sp_core::H256: From<T::Hash>,
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
	<T::ExtrinsicParams as ExtrinsicParams<T::Index, T::Hash>>::OtherParams:
		From<BaseExtrinsicParamsBuilder<T, AssetTip>> + Send + Sync,
{
	type FinalityEvent = FinalityEvent;
	type Error = Error;

	async fn query_latest_ibc_events<C>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &C,
	) -> Result<(Any, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		C: Chain,
	{
		self.finality_protocol
			.clone()
			.query_latest_ibc_events(self, finality_event, counterparty)
			.await
	}

	async fn ibc_events(
		&self,
	) -> Pin<Box<dyn Stream<Item = (TransactionId, Vec<Option<IbcEvent>>)> + Send + 'static>> {
		let stream = self
			.para_client
			.events()
			.subscribe()
			.await
			.expect("Failed to subscribe to events")
			.filter_events::<(parachain::api::ibc::events::Events,)>()
			.filter_map(|result| {
				log::info!("rcvd events: {:?}", result);
				let events = match result {
					Ok(ev) => ev,
					Err(err) => {
						log::error!("Error in IbcEvent stream: {err:?}");
						return futures::future::ready(None)
					},
				};
				let tx_index = match events.phase {
					Phase::ApplyExtrinsic(index) => index,
					phase => {
						log::error!("Received IbcEvent with unexpected phase: {phase:?}");
						return futures::future::ready(None)
					},
				};
				let result = events
					.event
					.events
					.into_iter()
					.map(|ev| {
						ev.ok()
							.map(|ev| {
								IbcEvent::try_from(RawIbcEvent::from(ev))
									.map_err(|e| subxt::Error::Other(e.to_string()))
							})
							.transpose()
					})
					.collect::<Result<Vec<Option<_>>, _>>();
				let tx_id = TransactionId { block_hash: H256::from(events.block_hash).0, tx_index };
				let events = match result {
					Ok(ev) => (tx_id, ev),
					Err(err) => {
						log::error!("Failed to decode event: {err:?}");
						return futures::future::ready(None)
					},
				};
				futures::future::ready(Some(events))
				// futures::future::ready(Some(stream::iter(events)))
			});

		Box::pin(Box::new(stream))
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_client_consensus_state(
			&*self.para_ws_client,
			Some(at.revision_height as u32),
			client_id.to_string(),
			consensus_height.revision_height,
			consensus_height.revision_number,
			false,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_client_state(
			&*self.para_ws_client,
			at.revision_height as u32,
			client_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_connection(
			&*self.para_ws_client,
			at.revision_height as u32,
			connection_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_channel(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let proof = IbcApiClient::<u32, H256>::query_proof(
			&*self.para_ws_client,
			at.revision_height as u32,
			keys,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		Ok(proof.proof)
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_commitment(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_acknowledgement(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_next_seq_recv(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_receipt(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		let finalized_header = self
			.para_client
			.rpc()
			.header(None)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let latest_height: u64 = (*finalized_header.number()).into();
		let height = Height::new(self.para_id.into(), latest_height.into());

		let subxt_block_number: subxt::rpc::BlockNumber = latest_height.into();
		let block_hash = self.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();
		let timestamp_addr = parachain::api::storage().timestamp().now();
		let unix_timestamp_millis = self
			.para_client
			.storage()
			.fetch(&timestamp_addr, block_hash)
			.await?
			.ok_or_else(|| Error::from("Timestamp should exist".to_string()))?;
		let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

		Ok((height, Timestamp::from_nanoseconds(timestamp_nanos)?))
	}

	async fn query_packet_commitments(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_commitments(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res.commitments.into_iter().map(|packet_state| packet_state.sequence).collect())
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_acknowledgements(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res
			.acknowledgements
			.into_iter()
			.map(|packet_state| packet_state.sequence)
			.collect())
	}

	async fn query_unreceived_packets(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_unreceived_packets(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	async fn query_unreceived_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_unreceived_acknowledgements(
			&*self.para_ws_client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(res)
	}

	fn channel_whitelist(&self) -> Vec<(ChannelId, PortId)> {
		self.channel_whitelist.clone()
	}

	async fn query_connection_channels(
		&self,
		at: Height,
		connection_id: &ConnectionId,
	) -> Result<QueryChannelsResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_connection_channels(
			&*self.para_ws_client,
			at.revision_height as u32,
			connection_id.to_string(),
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_send_packets(
			&*self.para_ws_client,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	async fn query_recv_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_recv_packets(
			&*self.para_ws_client,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok(response)
	}

	fn expected_block_time(&self) -> Duration {
		// Parachains have an expected block time of 12 seconds
		Duration::from_secs(12)
	}

	async fn query_client_update_time_and_height(
		&self,
		client_id: ClientId,
		client_height: Height,
	) -> Result<(Height, Timestamp), Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_client_update_time_and_height(
			&*self.para_ws_client,
			client_id.to_string(),
			client_height.revision_number,
			client_height.revision_height,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		Ok((
			response.height.into(),
			Timestamp::from_nanoseconds(response.timestamp)
				.map_err(|_| Error::Custom("Received invalid timestamp".to_string()))?,
		))
	}

	async fn query_host_consensus_state_proof(
		&self,
		height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		let hash = self.para_client.rpc().block_hash(Some(height.revision_height.into())).await?;
		let header = self
			.para_client
			.rpc()
			.header(hash)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let extrinsic_with_proof =
			fetch_timestamp_extrinsic_with_proof(&self.para_client, Some(header.hash()))
				.await
				.map_err(Error::BeefyProver)?;

		let host_consensus_proof = HostConsensusProof {
			header: header.encode(),
			extrinsic: extrinsic_with_proof.ext,
			extrinsic_proof: extrinsic_with_proof.proof,
		};
		Ok(Some(host_consensus_proof.encode()))
	}

	async fn query_ibc_balance(&self) -> Result<Vec<PrefixedCoin>, Self::Error> {
		let account = self.public_key.clone().into_account();
		let account_addr = parachain::api::storage().system().account(&account);
		let balance = self
			.para_client
			.storage()
			.fetch(&account_addr, None)
			.await?
			.expect("Account data should exist");

		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom::from_str("PICA")?,
			amount: Amount::from_str(&format!("{}", balance.data.free))?,
		}])
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.commitment_prefix.clone()).expect("Should not fail")
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	fn client_type(&self) -> ClientType {
		match self.finality_protocol {
			FinalityProtocol::Grandpa => GrandpaClientState::<HostFunctionsManager>::client_type(),
			FinalityProtocol::Beefy => BeefyClientState::<HostFunctionsManager>::client_type(),
		}
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let subxt_block_number: subxt::rpc::BlockNumber = block_number.into();
		let block_hash = self.para_client.rpc().block_hash(Some(subxt_block_number)).await.unwrap();
		let timestamp_addr = parachain::api::storage().timestamp().now();
		let unix_timestamp_millis = self
			.para_client
			.storage()
			.fetch(&timestamp_addr, block_hash)
			.await?
			.expect("Timestamp should exist");
		let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;

		Ok(timestamp_nanos)
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let response: Vec<IdentifiedClientState> =
			IbcApiClient::<u32, H256>::query_clients(&*self.para_ws_client)
				.await
				.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		response
			.into_iter()
			.map(|client| {
				ClientId::from_str(&client.client_id)
					.map_err(|_| Error::Custom("Invalid client id ".to_string()))
			})
			.collect()
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_channels(&*self.para_ws_client)
			.await
			.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;
		response
			.channels
			.into_iter()
			.map(|identified_chan| {
				Ok((
					ChannelId::from_str(&identified_chan.channel_id)
						.expect("Failed to convert invalid string to channel id"),
					PortId::from_str(&identified_chan.port_id)
						.expect("Failed to convert invalid string to port id"),
				))
			})
			.collect::<Result<Vec<_>, _>>()
	}

	async fn query_connection_using_client(
		&self,
		height: u32,
		client_id: String,
	) -> Result<Vec<IdentifiedConnection>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_connection_using_client(
			&*self.para_ws_client,
			height,
			client_id,
		)
		.await
		.map_err(|e| Error::from(format!("Rpc Error {:?}", e)))?;

		Ok(response)
	}

	fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> bool {
		let refresh_period: u64 = if cfg!(feature = "testing") { 15 } else { 50 };
		latest_height - latest_client_height_on_counterparty >= refresh_period
	}
}
