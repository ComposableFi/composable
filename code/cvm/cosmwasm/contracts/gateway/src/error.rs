use cosmwasm_std::{IbcOrder, Response, StdError};
use ibc_rs_scale::core::ics24_host::identifier::IdentifierError;
use thiserror::Error;
use xc_core::{AssetId, NetworkId};

pub type Result<T = Response, E = ContractError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ContractError {
	#[error("{0}")]
	Std(#[from] StdError),
	#[error("Caller is not authorised to take this action.")]
	NotAuthorized,
	#[error("IBC channel version mismatch {0}.")]
	InvalidIbcVersion(String),
	#[error("Unexpected IBC channel ordering {0:?}.")]
	InvalidIbcOrdering(IbcOrder),
	#[error("An invalid XCVM packet has been received.")]
	InvalidIbcXcvmPacket,
	#[error("No IBC channel is opened to the target network.")]
	UnsupportedNetwork,
	#[error("Ics20 not found")]
	ICS20NotFound,
	#[error("Failed ibc transfer {0}")]
	FailedIBCTransfer(String),
	#[error("Could not serialize to JSON")]
	FailedToSerialize,
	#[error("Asset not been found in the registry.")]
	AssetNotFound,
	#[error("Exchange not been found in the registry.")]
	ExchangeNotFound,
	#[error("The contract must be initialized first.")]
	NotInitialized,
	#[error("An overflow occurred.")]
	ArithmeticOverflow,
	#[error("Not enough funds to cover the operation.")]
	InsufficientFunds,
	#[error("Program funds denom mapping to host not found")]
	ProgramFundsDenomMappingToHostNotFound,
	#[error("Program amount not equal to host amount")]
	ProgramAmountNotEqualToHostAmount,
	#[error("{0}")]
	Protobuf(xc_core::proto::DecodeError),
	#[error("An invalid ACK was provided, this MUST be impossible.")]
	InvalidAck,
	#[error("An unknown reply ID was provided, this MUST be impossible.")]
	UnknownReply,
	#[error("The provided channel has not been previously opened.")]
	UnknownChannel,
	#[error("The asset is already registered.")]
	AlreadyRegistered,
	#[error("Route not found.")]
	RouteNotFound,
	#[error("{0}")]
	Bech32(bech32_no_std::Error),
	#[error("{0}")]
	Serde(#[from] serde_json_wasm::ser::Error),
	#[error("Assets non transferrable")]
	AssetsNonTransferrable,
	#[error("Cannot transfer assets")]
	CannotTransferAssets,
	#[error("Program cannot be handled by destination")]
	ProgramCannotBeHandledByDestination,
	#[error("Not implemented")]
	NotImplemented,
	#[error("{0}")]
	IbcIdentifier(IdentifierError),
	#[error("Network config")]
	NetworkConfig,
	#[error("Unknown target network")]
	UnknownTargetNetwork,
	#[error("No connection information from this {0} to other network {1}")]
	NoConnectionInformationFromThisToOtherNetwork(NetworkId, NetworkId),

	#[error("Asset {0} not found by id")]
	AssetNotFoundById(AssetId),
	#[error("Asset {0} cannot be transferred to network {1}")]
	AssetCannotBeTransferredToNetwork(AssetId, NetworkId),
	#[error("Gateway for network {0} not found")]
	GatewayForNetworkNotFound(NetworkId),
	#[error("Anonymous calls can do only limitet set of actions")]
	AnonymousCallsCanDoOnlyLimitedSetOfActions,
	#[error("Runtime unsupported on network")]
	RuntimeUnsupportedOnNetwork,
	#[error("Badly configured route because this chain can send only from cosmwasm")]
	BadlyConfiguredRouteBecauseThisChainCanSendOnlyFromCosmwasm,
	#[error("Account in program is not mappable to this chain")]
	AccountInProgramIsNotMappableToThisChain,
}

impl From<xc_core::proto::DecodeError> for ContractError {
	fn from(value: xc_core::proto::DecodeError) -> Self {
		Self::Protobuf(value)
	}
}

impl From<bech32_no_std::Error> for ContractError {
	fn from(value: bech32_no_std::Error) -> Self {
		Self::Bech32(value)
	}
}

impl From<IdentifierError> for ContractError {
	fn from(value: IdentifierError) -> Self {
		Self::IbcIdentifier(value)
	}
}
