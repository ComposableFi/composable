pub mod ics20;
pub mod picasso;

use crate::{
	gateway::{self, RelativeTimeout},
	prelude::*,
	shared::XcPacket,
	AssetId, NetworkId,
};
use cosmwasm_std::{
	to_binary, Api, BlockInfo, CosmosMsg, Deps, IbcEndpoint, QueryRequest, StdResult, WasmMsg,
};

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
	pub packet: XcPacket,
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
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum TransportTrackerId {
	/// Allows to identify results of IBC packets
	Ibc { channel_id: ChannelId, sequence: u64 },
}

/// route is used to describe how to send a packet to another network
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub struct IbcIcs20Route {
	pub from_network: NetworkId,
	pub local_native_denom: String,
	pub channel_to_send_over: ChannelId,
	pub sender_gateway: Addr,
	/// the contract address of the gateway to send to assets
	pub gateway_to_send_to: Addr,
	pub counterparty_timeout: RelativeTimeout,
	pub ibc_ics_20_sender: IbcIcs20Sender,
	pub on_remote_asset: AssetId,
}

pub fn to_cw_message<T>(
	deps: Deps,
	api: &dyn Api,
	coin: Coin,
	route: IbcIcs20Route,
	packet: XcPacket,
	block: BlockInfo,
) -> StdResult<(CosmosMsg<T>, TransportTrackerId)> {
	let msg = gateway::ExecuteMsg::MessageHook(XcMessageData {
		from_network_id: route.from_network,
		packet,
	});
	let memo = SendMemo {
		inner: Memo {
			wasm: Some(Callback {
				contract: route.gateway_to_send_to.clone(),
				msg: serde_cw_value::to_value(msg).expect("can always serde"),
			}),
			forward: None,
		},
		ibc_callback: None,
	};
	let memo = serde_json_wasm::to_string(&memo).expect("any memo can be to string");
	api.debug(&format!("cvm::gateway::ibc::ics20::memo {}", &memo));
	match route.ibc_ics_20_sender {
		IbcIcs20Sender::SubstratePrecompile(addr) => {
			let transfer = picasso::IbcMsg::Transfer {
				channel_id: route.channel_to_send_over.clone(),
				to_address: route.gateway_to_send_to,
				amount: coin,
				timeout: route.counterparty_timeout.absolute(block),
				memo: Some(memo),
			};
			Ok((
				WasmMsg::Execute {
					contract_addr: addr.into(),
					msg: to_binary(&transfer)?,
					funds: <_>::default(), /* above message already approved to transfer coins
					                        * via polkadot pallets, no need to repeat it here */
				}
				.into(),
				TransportTrackerId::Ibc {
					channel_id: route.channel_to_send_over.clone(),
					sequence: 42, /* substrate has no interface to get sequence number for now,
					               * so it does not work on picasso */
				},
			))
		},
		IbcIcs20Sender::CosmosStargateIbcApplicationsTransferV1MsgTransfer => {
			// really
			// https://github.com/osmosis-labs/osmosis-rust/blob/main/packages/osmosis-std-derive/src/lib.rs
			// https://github.com/osmosis-labs/osmosis/blob/main/cosmwasm/packages/registry/src/proto.rs

			use ibc_proto::{
				cosmos::base::v1beta1::Coin,
				ibc::{
					applications::transfer::v1::MsgTransfer,
					core::client::v1::Height as ProofHeight,
				},
			};

			use prost::Message;
			let value = MsgTransfer {
				source_port: PortId::transfer().to_string(),
				source_channel: route.channel_to_send_over.to_string(),
				token: Some(Coin { denom: coin.denom, amount: coin.amount.to_string() }),
				sender: route.sender_gateway.to_string(),
				receiver: route.gateway_to_send_to.to_string(),
				timeout_height: route.counterparty_timeout.absolute(block.clone()).block().map(
					|x| ibc_proto::ibc::core::client::v1::Height {
						revision_height: x.height,
						revision_number: x.revision,
					},
				),
				timeout_timestamp: route
					.counterparty_timeout
					.absolute(block)
					.timestamp()
					.map(|x| x.nanos())
					.unwrap_or_default(),
				memo,
			};
			api.debug(&format!("cvm::gateway::ibc::ics20:: payload {:?}", &value));

			let value = value.encode_to_vec();
			let value = Binary::from(value);

			// already in latest ibc-rs, BUT it will take 2 days to merge updates(all no_std deps),
			// so copy paste for now.
			/// QueryNextSequenceSendRequest is the request type for the
			/// Query/QueryNextSequenceSend RPC method
			#[derive(::serde::Serialize, ::serde::Deserialize)]
			#[allow(clippy::derive_partial_eq_without_eq)]
			#[derive(Clone, PartialEq, ::prost::Message)]
			pub struct QueryNextSequenceSendRequest {
				/// port unique identifier
				#[prost(string, tag = "1")]
				pub port_id: ::prost::alloc::string::String,
				/// channel unique identifier
				#[prost(string, tag = "2")]
				pub channel_id: ::prost::alloc::string::String,
			}
			/// QueryNextSequenceSendResponse is the request type for the
			/// Query/QueryNextSequenceSend RPC method
			#[derive(::serde::Serialize, ::serde::Deserialize)]
			#[allow(clippy::derive_partial_eq_without_eq)]
			#[derive(Clone, PartialEq, ::prost::Message)]
			pub struct QueryNextSequenceSendResponse {
				/// next sequence send number
				#[prost(uint64, tag = "1")]
				pub next_sequence_send: u64,
				/// merkle proof of existence
				#[prost(bytes = "vec", tag = "2")]
				pub proof: ::prost::alloc::vec::Vec<u8>,
				/// height at which the proof was retrieved
				#[prost(message, optional, tag = "3")]
				pub proof_height: ::core::option::Option<ProofHeight>,
			}

			// https://github.com/cosmos/ibc-go/issues/4698
			let tracking_id = deps
				.querier
				.query::<QueryNextSequenceSendResponse>(&QueryRequest::Stargate {
					path: "/ibc.core.channel.v1.Query/NextSequenceSend".to_string(),
					data: to_binary(
						&QueryNextSequenceSendRequest {
							port_id: PortId::transfer().to_string(),
							channel_id: route.channel_to_send_over.to_string(),
						}
						.encode_to_vec(),
					)?,
				})
				.map(|x| x.next_sequence_send)
				// until https://composableprotocol.slack.com/archives/C04NTDSCBQR/p1695143647381079
				.unwrap_or_default();

			let tracking_id = TransportTrackerId::Ibc {
				channel_id: route.channel_to_send_over,
				sequence: tracking_id,
			};
			
			Ok((
				CosmosMsg::Stargate {
					type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
					value,
				},
				tracking_id,
			))
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
