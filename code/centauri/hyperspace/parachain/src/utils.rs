use crate::Error;
use beefy_light_client_primitives::{ClientState, MmrUpdateProof};
use std::sync::Arc;

use beefy_primitives::known_payload_ids::MMR_ROOT_ID;
use beefy_prover::helpers::unsafe_arc_cast;
use codec::Decode;
use frame_support::weights::DispatchClass;
use frame_system::limits::BlockWeights;
use sp_core::H256;

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
