use ibc::core::ics02_client::context::ClientTypes;
use pallet_ibc::light_clients::{AnyClient, AnyClientMessage, AnyClientState, AnyConsensusState};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LocalClientTypes;

impl ClientTypes for LocalClientTypes {
	type AnyClientMessage = AnyClientMessage;
	type AnyClientState = AnyClientState;
	type AnyConsensusState = AnyConsensusState;
	type ClientDef = AnyClient;
}
