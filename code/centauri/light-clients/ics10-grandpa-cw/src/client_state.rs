use ics10_grandpa::client_state::ClientState;

use crate::ContractError;

pub fn validate_client_state<H: Clone>(
	client_state: &ClientState<H>,
	height: u64,
) -> Result<(), ContractError> {
	client_state.verify_height(todo!())?;
	Ok(())
}
