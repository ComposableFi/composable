use beefy_light_client_primitives::{ClientState, MmrUpdateProof};
use beefy_primitives::known_payload_ids::MMR_ROOT_ID;
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
