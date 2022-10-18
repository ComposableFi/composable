use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

// ClientState interface related messages
// Reference: https://github.com/cosmos/ibc-go/blob/main/modules/core/exported/client.go#L36
#[cw_serde]
pub struct ClientTypeMsg {}

#[cw_serde]
pub struct GetLatestHeightsMsg {}

#[cw_serde]
pub struct ValidateMsg {}

#[cw_serde]
pub struct StatusMsg {
	// how do we handle ctx sdk.Context, clientStore sdk.KVStore, cdc codec.BinaryCodec
}

#[cw_serde]
pub struct ExportedMetadataMsg {
	// clientStore sdk.KVStore
}

#[cw_serde]
pub struct ZeroCustomFieldsMsg {}

#[cw_serde]
pub struct GetTimestampAtHeightMsg {}

#[cw_serde]
pub struct InitializeMsg {}

#[cw_serde]
pub struct VerifyMembershipMsg {
	height: Height,
	delayTimePeriod: u64,
	delayBlockPeriod: u64,
	proof: Vec<u8>,
	path: Vec<u8>,
	value: Vec<u8>,
}

#[cw_serde]
pub struct VerifyNonMembershipMsg {
	height: Height,
	delayTimePeriod: u64,
	delayBlockPeriod: u64,
	proof: Vec<u8>,
	path: Vec<u8>,
	value: Vec<u8>,
}

#[cw_serde]
pub struct VerifyClientMessage {
	client_msg: ClientMessage,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsg {
	client_msg: ClientMessage,
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsg {
	client_msg: ClientMessage,
}

#[cw_serde]
pub struct UpdateStateMsg {
	client_msg: ClientMessage,
}

#[cw_serde]
pub struct CheckSubstituteAndUpdateStateMsg {
	substitute_client_msg: ClientMessage,
}

#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsg {
	new_client: ClientState,
	new_cons_state: ConsensusState,
	proof_upgrade_client: Vec<u8>,
	proof_upgrade_cons_state: Vec<u8>,
}
