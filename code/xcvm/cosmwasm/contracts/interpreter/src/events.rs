//! Evens emitted by the contract
use cosmwasm_std::Event;
use xc_core::service::dex::ExchangeId;


/// Event emitted when successful exchange happened
pub fn make_exchanged_event(exchange_id : ExchangeId) -> Event {
    Event::new("cvm.interpreter.exchanged")
        .add_attribute("exchange_id", exchange_id.to_string())        
}  