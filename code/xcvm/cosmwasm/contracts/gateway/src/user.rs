use crate::prelude::*;

pub(crate) fn force_instantiate(auth: crate::auth::Auth<crate::auth::policy::Admin>, deps: cosmwasm_std::DepsMut<'_>, network_id: xc_core::NetworkId, user_origin: String, salt: cosmwasm_std::Binary) -> Result<cosmwasm_std::Response, crate::error::ContractError> {
    
    NETWORK.save(deps.storage, msg.network_id, &msg)?;
	Ok(Response::new().add_event(
		make_event("network.forced").add_attribute("network_id", msg.network_id.to_string()),
	))
}

