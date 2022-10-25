use cosmwasm_std::Deps;
use ics10_grandpa::client_state::ClientState;

use crate::{ContractError, CLIENT_STATE};

/// Retrieves raw bytes from storage and deserializes them into [`ClientState`]
pub fn get_client_state<H: Clone>(deps: Deps) -> Result<ClientState<H>, ContractError> {
	deps.storage
		.get(CLIENT_STATE)
		.ok_or_else(|| ContractError::StorageError)
		.and_then(deserialize_client_state)
}

fn deserialize_client_state<H: Clone>(
	client_state: Vec<u8>,
) -> Result<ClientState<H>, ContractError> {
	unimplemented!()
}
