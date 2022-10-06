use crate::AnyConfig;
use async_trait::async_trait;
use derive_more::From;
use futures::{Stream, StreamExt};
use ibc::{
	core::{
		ics02_client::client_state::ClientType,
		ics04_channel::packet::Packet,
		ics23_commitment::commitment::CommitmentPrefix,
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
	},
	downcast,
	events::IbcEvent,
	signer::Signer,
	Height,
};
use ibc_proto::{
	google::protobuf::Any,
	ibc::core::{
		channel::v1::{
			QueryChannelResponse, QueryNextSequenceReceiveResponse,
			QueryPacketAcknowledgementResponse, QueryPacketCommitmentResponse,
			QueryPacketReceiptResponse,
		},
		client::v1::{QueryClientStateResponse, QueryConsensusStateResponse},
		connection::v1::QueryConnectionResponse,
	},
};
#[cfg(feature = "parachain")]
use parachain::ParachainClient;
use primitives::{Chain, IbcProvider, KeyProvider, UpdateType};
use std::pin::Pin;
#[cfg(feature = "parachain")]
use subxt::DefaultConfig;

pub enum AnyChain {
	#[cfg(feature = "parachain")]
	Parachain(ParachainClient<DefaultConfig>),
}

#[derive(From)]
pub enum AnyFinalityEvent {
	#[cfg(feature = "parachain")]
	Parachain(parachain::provider::FinalityEvent),
}

#[derive(Error, Debug)]
pub enum AnyError {
	#[cfg(feature = "parachain")]
	#[error("{0}")]
	Parachain(#[from] parachain::error::Error),
	#[error("{0}")]
	Other(String),
}

impl From<String> for AnyError {
	fn from(s: String) -> Self {
		Self::Other(s)
	}
}

#[async_trait]
impl IbcProvider for AnyChain {
	type FinalityEvent = AnyFinalityEvent;
	type Error = AnyError;

	async fn query_latest_ibc_events<T>(
		&mut self,
		finality_event: Self::FinalityEvent,
		counterparty: &T,
	) -> Result<(Any, Vec<IbcEvent>, UpdateType), anyhow::Error>
	where
		T: Chain,
	{
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => {
				let finality_event = downcast!(finality_event => AnyFinalityEvent::Parachain)
					.ok_or_else(|| AnyError::Other("Invalid finality event type".to_owned()))?;
				let (client_msg, events, update_type) =
					chain.query_latest_ibc_events(finality_event, counterparty).await?;
				Ok((client_msg, events, update_type))
			},
		}
	}

	async fn query_client_consensus(
		&self,
		at: Height,
		client_id: ClientId,
		consensus_height: Height,
	) -> Result<QueryConsensusStateResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain
				.query_client_consensus(at, client_id, consensus_height)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_client_state(
		&self,
		at: Height,
		client_id: ClientId,
	) -> Result<QueryClientStateResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain.query_client_state(at, client_id).await.map_err(Into::into),
		}
	}

	async fn query_connection_end(
		&self,
		at: Height,
		connection_id: ConnectionId,
	) -> Result<QueryConnectionResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) =>
				chain.query_connection_end(at, connection_id).await.map_err(Into::into),
		}
	}

	async fn query_channel_end(
		&self,
		at: Height,
		channel_id: ChannelId,
		port_id: PortId,
	) -> Result<QueryChannelResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) =>
				chain.query_channel_end(at, channel_id, port_id).await.map_err(Into::into),
		}
	}

	async fn query_proof(&self, at: Height, keys: Vec<Vec<u8>>) -> Result<Vec<u8>, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain.query_proof(at, keys).await.map_err(Into::into),
		}
	}

	async fn query_packets(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seqs: Vec<u64>,
	) -> Result<Vec<Packet>, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) =>
				chain.query_packets(at, port_id, channel_id, seqs).await.map_err(Into::into),
		}
	}

	async fn query_packet_commitment(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketCommitmentResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain
				.query_packet_commitment(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_packet_acknowledgement(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketAcknowledgementResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain
				.query_packet_acknowledgement(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_next_sequence_recv(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
	) -> Result<QueryNextSequenceReceiveResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain
				.query_next_sequence_recv(at, port_id, channel_id)
				.await
				.map_err(Into::into),
		}
	}

	async fn query_packet_receipt(
		&self,
		at: Height,
		port_id: &PortId,
		channel_id: &ChannelId,
		seq: u64,
	) -> Result<QueryPacketReceiptResponse, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain
				.query_packet_receipt(at, port_id, channel_id, seq)
				.await
				.map_err(Into::into),
		}
	}

	async fn latest_height(&self) -> Result<Height, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain.latest_height().await.map_err(Into::into),
		}
	}

	async fn query_host_consensus_state_proof(
		&self,
		height: Height,
	) -> Result<Option<Vec<u8>>, Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) =>
				chain.query_host_consensus_state_proof(height).await.map_err(Into::into),
		}
	}

	fn connection_prefix(&self) -> CommitmentPrefix {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain.connection_prefix(),
		}
	}

	fn client_id(&self) -> ClientId {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain.client_id(),
		}
	}

	fn client_type(&self) -> ClientType {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain.client_type(),
		}
	}

	#[cfg(feature = "testing")]
	async fn ibc_events(&self) -> Pin<Box<dyn Stream<Item = IbcEvent> + Send + Sync>> {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(chain) => chain.ibc_events().await,
			_ => unreachable!(),
		}
	}
}

impl KeyProvider for AnyChain {
	fn account_id(&self) -> Signer {
		match self {
			#[cfg(feature = "parachain")]
			AnyChain::Parachain(parachain) => parachain.account_id(),
		}
	}
}

#[async_trait]
impl Chain for AnyChain {
	fn name(&self) -> &str {
		match self {
			#[cfg(feature = "parachain")]
			Self::Parachain(chain) => chain.name(),
		}
	}

	async fn finality_notifications(
		&self,
	) -> Pin<Box<dyn Stream<Item = Self::FinalityEvent> + Send + Sync>> {
		match self {
			#[cfg(feature = "parachain")]
			Self::Parachain(chain) => Box::pin(chain.finality_notifications().await.map(|x| x.into())),
		}
	}

	async fn submit_ibc_messages(&self, messages: Vec<Any>) -> Result<(), Self::Error> {
		match self {
			#[cfg(feature = "parachain")]
			Self::Parachain(chain) => chain.submit_ibc_messages(messages).await.map_err(Into::into),
		}
	}
}

impl AnyConfig {
	pub async fn into_client(self) -> anyhow::Result<AnyChain> {
		Ok(match self {
			#[cfg(feature = "parachain")]
			AnyConfig::Parachain(config) => AnyChain::Parachain(ParachainClient::new(config).await?),
		})
	}
}
