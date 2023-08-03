use crate::prelude::*;
use cosmwasm_std::IbcTimeout;
use ibc_rs_scale::core::ics24_host::identifier::ChannelId;
pub const IBC_PRECOMPILE: &str = "5EYCAe5g89aboD4c8naVbgG6izsMBCgtoCB9TUHiJiH2yVow";

/// These are messages in the IBC lifecycle. Only usable by IBC-enabled contracts
/// (contracts that directly speak the IBC protocol via 6 entry points)
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum IbcMsg {
	/// Sends bank tokens owned by the contract to the given address on another chain.
	/// The channel must already be established between the ibctransfer module on this chain
	/// and a matching module on the remote chain.
	/// We cannot select the port_id, this is whatever the local chain has bound the ibctransfer
	/// module to.
	Transfer {
		/// exisiting channel to send the tokens over
		channel_id: ChannelId,
		/// address on the remote chain to receive these tokens
		to_address: Addr,
		/// packet data only supports one coin
		/// https://github.com/cosmos/cosmos-sdk/blob/v0.40.0/proto/ibc/applications/transfer/v1/transfer.proto#L11-L20
		amount: Coin,
		/// when packet times out, measured on remote chain
		timeout: IbcTimeout,
		memo: Option<String>,
	},
}
