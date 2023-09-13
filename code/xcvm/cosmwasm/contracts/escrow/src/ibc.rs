use crate::{
	error::{ContractError, Result},
	msg, state,
};
use cosmwasm_std::{CosmosMsg, IbcMsg, IbcTimeout, IbcTimeoutBlock, Storage};

use xc_core::proto::Isomorphism;

/// Makes a CosmosMsg sending given packet to accounts contract.
///
/// Depending whether accounts contract runs on local chain or remotely, the
/// message is either a local execute message or an IBC send packet message.
/// Returns and error if no accounts contract is configured.
pub(crate) fn make_accounts_message(
	storage: &dyn Storage,
	packet: msg::accounts::Packet,
) -> Result<CosmosMsg> {
	match state::Config::load(storage)?.accounts_contract {
		state::AccountsContract::Local(addr) => {
			let msg = msg::accounts::ExecuteMsg::LocalPacket(packet);
			cosmwasm_std::wasm_execute(addr, &msg, Vec::new())
				.map(CosmosMsg::from)
				.map_err(ContractError::from)
		},
		state::AccountsContract::Remote(channel) => {
			let msg = IbcMsg::SendPacket {
				channel_id: channel,
				data: cosmwasm_std::Binary::from(packet.encode()),
				// TODO: should be a parameter or configuration
				timeout: IbcTimeout::with_block(IbcTimeoutBlock { revision: 0, height: 10000 }),
			};
			Ok(IbcMsg::from(msg).into())
		},
		state::AccountsContract::None => Err(ContractError::NoAccountsContract),
	}
}

pub(crate) fn decode<T: Isomorphism>(data: &[u8]) -> Result<T> {
	T::decode(data).map_err(|_| ContractError::InvalidPacket)
}
