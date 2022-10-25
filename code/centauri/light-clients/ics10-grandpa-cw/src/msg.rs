use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::types::Height;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
	ValidateMsg(ValidateMsg),
	StatusMsg(StatusMsg),
	ExportedMetadataMsg(ExportedMetadataMsg),
	ZeroCustomFieldsMsg(ZeroCustomFieldsMsg),
	GetTimestampAtHeightMsg(GetTimestampAtHeightMsg),
	InitializeMsg(InitializeMsg),
	VerifyMembershipMsg(VerifyMembershipMsg),
	VerifyClientMessage(VerifyClientMessage),
	CheckForMisbehaviourMsg(CheckForMisbehaviourMsg),
	UpdateStateOnMisbehaviourMsg(UpdateStateOnMisbehaviourMsg),
	UpdateStateMsg(UpdateStateMsg),
	CheckSubstituteAndUpdateStateMsg(CheckSubstituteAndUpdateStateMsg),
	VerifyUpgradeAndUpdateStateMsg(VerifyUpgradeAndUpdateStateMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
	ClientTypeMsg(ClientTypeMsg),
	GetLatestHeightsMsg(GetLatestHeightsMsg),
}

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
	height: u64,
	delayTimePeriod: u64,
	delayBlockPeriod: u64,
	proof: Vec<u8>,
	path: Vec<u8>,
	value: Vec<u8>,
}

#[cw_serde]
pub struct VerifyClientMessage {
	client_msg: Vec<u8>,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsg {
	client_msg: Vec<u8>,
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsg {
	client_msg: Vec<u8>,
}

#[cw_serde]
pub struct UpdateStateMsg {
	client_msg: Vec<u8>,
}

#[cw_serde]
pub struct CheckSubstituteAndUpdateStateMsg {
	substitute_client_msg: Vec<u8>,
}

#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsg {
	new_client: Vec<u8>,
	new_cons_state: Vec<u8>,
	proof_upgrade_client: Vec<u8>,
	proof_upgrade_cons_state: Vec<u8>,
}
