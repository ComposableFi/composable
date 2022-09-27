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
use subxt::Config;

use super::{error::Error, ParachainClient};

use ibc_rpc::{IbcApiClient, PacketInfo};
use primitives::{Chain, IbcProvider, KeyProvider, UpdateType};
use sp_core::H256;

use crate::{
	parachain, parachain::api::runtime_types::primitives::currency::CurrencyId, GrandpaClientState,
	LightClientProtocol,
};
use ibc::{
	applications::transfer::{Amount, PrefixedCoin, PrefixedDenom},
	core::ics02_client::client_state::ClientType,
	timestamp::Timestamp,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{channel::v1::QueryChannelsResponse, client::v1::IdentifiedClientState},
};

use crate::light_client_protocols::FinalityEvent;
use beefy_prover::helpers::fetch_timestamp_extrinsic_with_proof;
use grandpa_light_client_primitives::{FinalityProof, ParachainHeaderProofs};
use ics11_beefy::client_state::ClientState as BeefyClientState;
use pallet_ibc::{light_clients::HostFunctionsManager, HostConsensusProof};

use sp_runtime::traits::One;
use std::{collections::BTreeMap, str::FromStr, time::Duration};

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
	FinalityProof<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>>:
		From<FinalityProof<T::Header>>,
	BTreeMap<sp_core::H256, ParachainHeaderProofs>:
		From<BTreeMap<<T as subxt::Config>::Hash, ParachainHeaderProofs>>,
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
		match self.light_client_protocol {
			LightClientProtocol::Grandpa =>
				LightClientProtocol::query_latest_ibc_event_with_grandpa::<T, C>(
					self,
					finality_event,
					counterparty,
				)
				.await,
			LightClientProtocol::Beefy =>
				LightClientProtocol::query_latest_ibc_event_with_beefy::<T, C>(
					self,
					finality_event,
					counterparty,
				)
				.await,
		}
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_client_consensus_state(
			&*self.para_client.rpc().client,
			Some(at.revision_height as u32),
			client_id.to_string(),
			consensus_height.revision_height,
			consensus_height.revision_number,
			false,
		)
		.await?;
		Ok(res)
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_client_state(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			client_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_connection(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			connection_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_channel(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		let proof = IbcApiClient::<u32, H256>::query_proof(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			keys,
		)
		.await?;

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
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await?;
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
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await?;
		Ok(res)
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_next_seq_recv(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
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
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seq,
		)
		.await?;
		Ok(res)
	}

	async fn latest_height_and_timestamp(&self) -> Result<(Height, Timestamp), Self::Error> {
		let finalized_header = self
			.para_client
			.rpc()
			.header(None)
			.await?
			.ok_or_else(|| Error::Custom("Latest height query returned None".to_string()))?;
		let latest_height = *finalized_header.number();
		let height = Height::new(self.para_id.into(), latest_height.into());

		let api = self
			.para_client
			.clone()
			.to_runtime_api::<parachain::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();
		let block_hash = finalized_header.hash();
		let unix_timestamp_millis = api.storage().timestamp().now(Some(block_hash)).await?;
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
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
		Ok(res.commitments.into_iter().map(|packet_state| packet_state.sequence).collect())
	}

	async fn query_packet_acknowledgements(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<Vec<u64>, Self::Error> {
		let res = IbcApiClient::<u32, H256>::query_packet_acknowledgements(
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
		)
		.await?;
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
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
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
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
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
			&*self.para_client.rpc().client,
			at.revision_height as u32,
			connection_id.to_string(),
		)
		.await?;
		Ok(response)
	}

	async fn query_send_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_send_packets(
			&*self.para_client.rpc().client,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
		Ok(response)
	}

	async fn query_recv_packets(
		&self,
		channel_id: ChannelId,
		port_id: PortId,
		seqs: Vec<u64>,
	) -> Result<Vec<PacketInfo>, Self::Error> {
		let response = IbcApiClient::<u32, H256>::query_recv_packets(
			&*self.para_client.rpc().client,
			channel_id.to_string(),
			port_id.to_string(),
			seqs,
		)
		.await?;
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
			&*self.para_client.rpc().client,
			client_id.to_string(),
			client_height.revision_number,
			client_height.revision_height,
		)
		.await?;
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
		let api = self
			.para_client
			.clone()
			.to_runtime_api::<parachain::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();

		let account = self.public_key.clone().into_account();
		let balance = api.storage().tokens().accounts(&account, &CurrencyId(1), None).await?;

		Ok(vec![PrefixedCoin {
			denom: PrefixedDenom::from_str("PICA")?,
			amount: Amount::from_str(&format!("{}", balance.free))?,
		}])
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		CommitmentPrefix::try_from(self.commitment_prefix.clone()).expect("Should not fail")
	}

	fn client_id(&self) -> ClientId {
		self.client_id()
	}

	fn client_type(&self) -> ClientType {
		match self.light_client_protocol {
			LightClientProtocol::Grandpa =>
				GrandpaClientState::<HostFunctionsManager>::client_type(),
			LightClientProtocol::Beefy => BeefyClientState::<HostFunctionsManager>::client_type(),
		}
	}

	async fn query_timestamp_at(&self, block_number: u64) -> Result<u64, Self::Error> {
		let api = self
			.para_client
			.clone()
			.to_runtime_api::<parachain::api::RuntimeApi<T, subxt::PolkadotExtrinsicParams<_>>>();
		let block_hash = self.para_client.rpc().block_hash(Some(block_number.into())).await?;
		let unix_timestamp_millis = api.storage().timestamp().now(block_hash).await?;
		let timestamp_nanos = Duration::from_millis(unix_timestamp_millis).as_nanos() as u64;
		Ok(timestamp_nanos)
	}

	async fn query_clients(&self) -> Result<Vec<ClientId>, Self::Error> {
		let response: Vec<IdentifiedClientState> =
			IbcApiClient::<u32, H256>::query_clients(&*self.para_client.rpc().client).await?;
		response
			.into_iter()
			.map(|client| {
				ClientId::from_str(&client.client_id)
					.map_err(|_| Error::Custom("Invalid client id ".to_string()))
			})
			.collect()
	}

	async fn query_channels(&self) -> Result<Vec<(ChannelId, PortId)>, Self::Error> {
		let response =
			IbcApiClient::<u32, H256>::query_channels(&*self.para_client.rpc().client).await?;
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

	fn is_update_required(
		&self,
		latest_height: u64,
		latest_client_height_on_counterparty: u64,
	) -> bool {
		let refresh_period: u64 = if cfg!(feature = "testing") { 15 } else { 50 };
		latest_height - latest_client_height_on_counterparty >= refresh_period
	}
}
