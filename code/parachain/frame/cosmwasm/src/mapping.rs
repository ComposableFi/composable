//! this simply maps `ibc-rs` and `cosmwasm-std` types back and forth

use cosmwasm_vm::cosmwasm_std::{IbcChannel, IbcChannelOpenMsg, IbcEndpoint, IbcOrder as CwOrder};
use ibc::{
	applications::transfer::{Amount, PrefixedCoin, PrefixedDenom},
	core::{
		ics04_channel::{
			channel::{Counterparty, Order as IbcOrder},
			error::Error as IbcError,
			Version as IbcVersion,
		},
		ics24_host::identifier::{ChannelId, ConnectionId, PortId},
		ics26_routing::context::{
			Module as IbcModule, ModuleCallbackContext, ModuleId, ModuleOutputBuilder,
		},
	},
	signer::Signer as IbcSigner,
};

use crate::Config;
pub fn ibc_to_cw_channel_open<T: Config + Send + Sync>(
	channel_id: &ChannelId,
	port_id: &PortId,
	counterparty: &Counterparty,
	order: IbcOrder,
	version: &IbcVersion,
	connection_hops: &[ConnectionId],
) -> Result<IbcChannelOpenMsg, IbcError> {
	Ok(IbcChannelOpenMsg::OpenInit {
		channel: IbcChannel::new(
			IbcEndpoint { channel_id: channel_id.to_string(), port_id: port_id.to_string() },
			IbcEndpoint {
				port_id: counterparty.port_id.to_string(),
				channel_id: counterparty.channel_id.expect("channel").to_string(),
			},
			map_order(order)?,
			version.to_string(),
			connection_hops
				.get(0)
				.expect("by spec there is at least one connection; qed")
				.to_string(),
		),
	})
}

pub fn map_order(order: IbcOrder) -> Result<CwOrder, IbcError> {
	match order {
		IbcOrder::Unordered => Ok(CwOrder::Unordered),
		IbcOrder::Ordered => Ok(CwOrder::Ordered),
	}
}

pub fn to_cosmwasm_timeout_block(
	ibc::core::ics02_client::height::Height { revision_number, revision_height }: ibc::core::ics02_client::height::Height,
) -> cosmwasm_vm::cosmwasm_std::IbcTimeoutBlock {
	cosmwasm_vm::cosmwasm_std::IbcTimeoutBlock {
		revision: revision_number,
		height: revision_height,
	}
}

pub fn to_cosmwasm_timestamp(
	timestamp: ibc::timestamp::Timestamp,
) -> cosmwasm_vm::cosmwasm_std::Timestamp {
	cosmwasm_vm::cosmwasm_std::Timestamp::from_nanos(timestamp.nanoseconds())
}
