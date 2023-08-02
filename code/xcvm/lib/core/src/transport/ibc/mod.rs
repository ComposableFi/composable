pub mod ics20;
pub mod picasso;

use crate::{prelude::*, proto::Encodable, shared::XcPacket, AssetId, NetworkId};
use cosmwasm_std::{to_binary, CosmosMsg, IbcEndpoint, IbcTimeout, StdResult, WasmMsg};

use ibc_rs_scale::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};

use self::ics20::{
	hook::{Callback, IBCLifecycleComplete},
	Memo, SendMemo,
};

/// This message should be send as part of wasm termination memo.
/// So that can match it to sender hash and know what channel and origin was used to send message.
/// All information here is not secured until compared with existing secured data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct XcMessageData {
	pub from_network_id: NetworkId,
	pub data: Binary,
}

/// Message type for `sudo` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum SudoMsg {
	#[serde(rename = "ibc_lifecycle_complete")]
	IBCLifecycleComplete(IBCLifecycleComplete),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct IbcRoute {
	pub from_network: NetworkId,
	pub local_native_denom: String,
	pub channel_to_send_over: ChannelId,
	pub sender_gateway: Addr,
	pub gateway_to_send_to: Addr,
	pub counterparty_timeout: IbcTimeout,
	pub ibc_ics_20_sender: IbcIcs20Sender,
	pub on_remote_asset: AssetId,
}

pub fn to_cw_message<T>(coin: Coin, route: IbcRoute, packet: XcPacket) -> StdResult<CosmosMsg<T>> {
	let memo =
		XcMessageData { from_network_id: route.from_network, data: Binary::from(packet.encode()) };
	let memo = SendMemo {
		inner: Memo {
			wasm: Some(Callback {
				contract: route.gateway_to_send_to.clone(),

				msg: serde_cw_value::to_value(&memo).expect("can always serde"),
			}),
			forward: None,
		},
		ibc_callback: None,
	};
	let memo = serde_json_wasm::to_string(&memo).expect("any memo can be to string");

	match route.ibc_ics_20_sender {
		IbcIcs20Sender::SubstratePrecompile(addr) => {
			let transfer = picasso::IbcMsg::Transfer {
				channel_id: route.channel_to_send_over.clone(),
				to_address: route.gateway_to_send_to,
				amount: coin,
				timeout: route.counterparty_timeout,
				memo: Some(memo),
			};
			Ok(WasmMsg::Execute {
				contract_addr: addr.into(),
				msg: to_binary(&transfer)?,
				funds: <_>::default(),
			}
			.into())
		},
		IbcIcs20Sender::CosmosStargateIbcApplicationsTransferV1MsgTransfer => {
			// really
			// https://github.com/osmosis-labs/osmosis-rust/blob/main/packages/osmosis-std-derive/src/lib.rs
			// https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/packages/registry/src/proto.rs
			use ibc_proto::{
				cosmos::base::v1beta1::Coin, ibc::applications::transfer::v1::MsgTransfer,
			};

			use prost::Message;
			let value = MsgTransfer {
				source_port: PortId::transfer().to_string(),
				source_channel: route.channel_to_send_over.to_string(),
				token: Some(Coin { denom: coin.denom, amount: coin.amount.to_string() }),
				sender: route.sender_gateway.to_string(),
				receiver: route.gateway_to_send_to.to_string(),
				timeout_height: route.counterparty_timeout.block().map(|x| {
					ibc_proto::ibc::core::client::v1::Height {
						revision_height: x.height,
						revision_number: x.revision,
					}
				}),
				timeout_timestamp: route
					.counterparty_timeout
					.timestamp()
					.map(|x| x.seconds())
					.unwrap_or_default(),
				memo,
			}
			.encode_to_vec();
			let value = Binary::from(value);
			Ok(CosmosMsg::Stargate {
				type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
				value,
			}
			.into())
		},

		IbcIcs20Sender::CosmWasmStd1_3 =>
			Err(cosmwasm_std::StdError::GenericErr { msg: "NotSupported".to_string() }),
	}
}

/// Information associated with an IBC channel.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct ChannelInfo {
	/// id of this channel
	pub id: ChannelId,
	/// the remote channel/port we connect to
	pub counterparty_endpoint: IbcEndpoint,
	/// the connection this exists on (you can use to query client/consensus info)
	pub connection_id: ConnectionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum IbcIcs20Sender {
	SubstratePrecompile(Addr),
	CosmosStargateIbcApplicationsTransferV1MsgTransfer,
	CosmWasmStd1_3,
}
