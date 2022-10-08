use crate::Error;
use beefy_light_client_primitives::{ClientState, MmrUpdateProof};
use std::sync::Arc;

use crate::parachain::api::runtime_types::pallet_ibc::events::IbcEvent as MetadataIbcEvent;
use beefy_primitives::known_payload_ids::MMR_ROOT_ID;
use beefy_prover::helpers::unsafe_arc_cast;
use codec::Decode;
use frame_support::weights::DispatchClass;
use frame_system::limits::BlockWeights;
use pallet_ibc::events::IbcEvent as RawIbcEvent;
use sp_core::H256;

impl From<MetadataIbcEvent> for RawIbcEvent {
	fn from(event: MetadataIbcEvent) -> Self {
		match event {
			MetadataIbcEvent::NewBlock { revision_height, revision_number } =>
				RawIbcEvent::NewBlock { revision_height, revision_number },
			MetadataIbcEvent::OpenInitConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			} => RawIbcEvent::OpenInitConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			},
			MetadataIbcEvent::OpenTryConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			} => RawIbcEvent::OpenTryConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			},
			MetadataIbcEvent::OpenAckConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			} => RawIbcEvent::OpenAckConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			},
			MetadataIbcEvent::OpenConfirmConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			} => RawIbcEvent::OpenConfirmConnection {
				revision_height,
				revision_number,
				connection_id,
				counterparty_connection_id,
				client_id,
				counterparty_client_id,
			},
			MetadataIbcEvent::OpenInitChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			} => RawIbcEvent::OpenInitChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			},
			MetadataIbcEvent::OpenTryChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			} => RawIbcEvent::OpenTryChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			},
			MetadataIbcEvent::OpenAckChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			} => RawIbcEvent::OpenAckChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			},
			MetadataIbcEvent::OpenConfirmChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			} => RawIbcEvent::OpenConfirmChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			},
			MetadataIbcEvent::CloseInitChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			} => RawIbcEvent::CloseInitChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			},
			MetadataIbcEvent::CloseConfirmChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			} => RawIbcEvent::CloseConfirmChannel {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				connection_id,
				counterparty_port_id,
				counterparty_channel_id,
			},
			MetadataIbcEvent::SendPacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				dest_port,
				dest_channel,
				sequence,
			} => RawIbcEvent::SendPacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				dest_port,
				dest_channel,
				sequence,
			},
			MetadataIbcEvent::WriteAcknowledgement {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				dest_port,
				dest_channel,
				sequence,
			} => RawIbcEvent::WriteAcknowledgement {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				dest_port,
				dest_channel,
				sequence,
			},
			MetadataIbcEvent::TimeoutPacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				sequence,
			} => RawIbcEvent::TimeoutPacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				sequence,
			},
			MetadataIbcEvent::TimeoutOnClosePacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				sequence,
			} => RawIbcEvent::TimeoutOnClosePacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				sequence,
			},
			MetadataIbcEvent::CreateClient {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			} => RawIbcEvent::CreateClient {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			},
			MetadataIbcEvent::UpdateClient {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			} => RawIbcEvent::UpdateClient {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			},
			MetadataIbcEvent::UpgradeClient {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			} => RawIbcEvent::UpgradeClient {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			},
			MetadataIbcEvent::ClientMisbehaviour {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			} => RawIbcEvent::ClientMisbehaviour {
				client_id,
				client_type,
				revision_height,
				revision_number,
				consensus_height,
				consensus_revision_number,
			},
			MetadataIbcEvent::ReceivePacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				dest_port,
				dest_channel,
				sequence,
			} => RawIbcEvent::ReceivePacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				dest_port,
				dest_channel,
				sequence,
			},
			MetadataIbcEvent::AcknowledgePacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				sequence,
			} => RawIbcEvent::AcknowledgePacket {
				revision_height,
				revision_number,
				port_id,
				channel_id,
				sequence,
			},
			MetadataIbcEvent::AppModule { kind, module_id } =>
				RawIbcEvent::AppModule { kind, module_id },
			MetadataIbcEvent::Empty => RawIbcEvent::Empty,
			MetadataIbcEvent::ChainError => RawIbcEvent::ChainError,
		}
	}
}

pub fn get_updated_client_state(
	mut client_state: ClientState,
	mmr_update: &MmrUpdateProof,
) -> ClientState {
	if mmr_update.signed_commitment.commitment.validator_set_id == client_state.next_authorities.id
	{
		client_state.current_authorities = client_state.next_authorities.clone();
		client_state.next_authorities = mmr_update.latest_mmr_leaf.beefy_next_authority_set.clone();
	}

	client_state.latest_beefy_height = mmr_update.signed_commitment.commitment.block_number;
	if let Some(mmr_root_hash) =
		mmr_update.signed_commitment.commitment.payload.get_raw(&MMR_ROOT_ID)
	{
		let mmr_root_hash = H256::from_slice(&*mmr_root_hash);
		client_state.mmr_root_hash = mmr_root_hash;
	}

	client_state
}

/// Fetch the maximum allowed extrinsic weight from a substrate node with the given client.
pub async fn fetch_max_extrinsic_weight<T: subxt::Config>(
	client: &subxt::OnlineClient<T>,
) -> Result<u64, Error> {
	let metadata = client.rpc().metadata().await?;
	let block_weights = metadata.pallet("System")?.constant("BlockWeights")?;
	let weights = BlockWeights::decode(&mut &block_weights.value[..])?;
	let extrinsic_weights = weights.per_class.get(DispatchClass::Normal);
	let max_extrinsic_weight = extrinsic_weights
		.max_extrinsic
		.or(extrinsic_weights.max_total)
		.unwrap_or(u64::MAX);
	Ok(max_extrinsic_weight)
}

pub unsafe fn unsafe_cast_to_jsonrpsee_client(
	client: &Arc<jsonrpsee_ws_client::WsClient>,
) -> Arc<jsonrpsee::core::client::Client> {
	unsafe_arc_cast::<_, _>(client.clone())
}
