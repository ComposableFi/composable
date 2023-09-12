use cosmwasm_std::{Event, Addr};
use serde::{Serialize, Deserialize};
use xc_core::{service::dex::ExchangeId, InterpreterOrigin, shared};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.exchanged")]
pub struct CvmInterpreterExchanged {
	pub exchange_id: ExchangeId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.execution_started")]
pub struct CvmInterpreterExecutionStarted {
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.instantiated")]
pub struct CvmInterpreterInstantiated {
	pub interpreter_origin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.transferred")]
pub struct CvmInterpreterTransferred {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.owner.added")]
pub struct CvmInterpreterOwnerAdded {
    pub owner : Addr,
}



// beneath is something to be generate by macro

impl CvmInterpreterOwnerAdded {
        pub fn new(owner: &[Addr]) -> Event {
        Event::new("cvm.interpreter.owner.added")
            .add_attribute("owner", owner.to_string())
    }
}

impl CvmInterpreterExecutionStarted {
    pub fn new() -> Event {
        Event::new("cvm.interpreter.execution_started")
    }
}

impl CvmInterpreterTransferred {
    pub fn new() -> Event {
        Event::new("cvm.interpreter.transferred")
    }
}

impl CvmInterpreterInstantiated {
	pub fn new(interpreter_origin: &InterpreterOrigin) -> Event {
		Event::new("cvm.interpreter.instantiated")
			.add_attribute("interpreter_origin", shared::encode_base64(interpreter_origin).expect("origin is managed by"))
	}
}

impl CvmInterpreterExchanged {
	pub fn new(exchange_id: ExchangeId) -> Event {
		Event::new("cvm.interpreter.exchanged")
			.add_attribute("exchange_id", exchange_id.to_string())
	}
}
