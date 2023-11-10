use cosmwasm_std::{Deps, DepsMut};
use xc_core::service::dex::{ExchangeId, ExchangeItem};

use crate::{
	auth,
	batch::BatchResponse,
	error::{ContractError, Result},
	events::make_event,
	state::exchange::EXCHANGE,
};

pub(crate) fn get_by_id(deps: Deps, exchange_id: ExchangeId) -> Result<ExchangeItem> {
	EXCHANGE
		.may_load(deps.storage, exchange_id.0)?
		.ok_or(ContractError::ExchangeNotFound)
}

pub(crate) fn force_exchange(
	_: auth::Admin,
	deps: DepsMut,
	msg: ExchangeItem,
) -> Result<BatchResponse> {
	EXCHANGE.save(deps.storage, msg.exchange_id.0, &msg)?;
	Ok(BatchResponse::new().add_event(
		make_event("exchange.forced").add_attribute("exchange_od", msg.exchange_id.to_string()),
	))
}
