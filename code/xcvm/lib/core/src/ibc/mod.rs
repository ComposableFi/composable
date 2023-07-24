pub mod hook;
pub mod picasso;

use crate::{prelude::*, AssetId, IbcIcs20Sender, NetworkId};
use cosmwasm_std::{to_binary, CosmosMsg, IbcTimeout, StdResult, WasmMsg};

use self::hook::IBCLifecycleComplete;
use ibc_rs_scale::core::ics24_host::identifier::ChannelId;

/// see https://github.com/osmosis-labs/osmosis/tree/main/x/ibc-hooks
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct WasmMemo {
	pub contract: String,
	pub msg: Binary,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ibc_callback: Option<String>,
}

/// This message should be send as part of wasm termination memo.
/// So that can match it to sender hash and know what channel and origin was used to send message.
/// All information here is not secured until compared with existing secured data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub struct Ics20MessageHook {
	pub from_network_id: NetworkId,
	pub data: Binary,
}

/// Message type for `sudo` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
//#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum SudoMsg {
	#[serde(rename = "ibc_lifecycle_complete")]
	IBCLifecycleComplete(IBCLifecycleComplete),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct IbcRoute {
	pub from_network: NetworkId,
	pub local_native_denom: String,
	pub channel_to_send_to: ChannelId,
	pub gateway_to_send_to: String,
	pub counterparty_timeout: IbcTimeout,
	pub ibc_ics_20_sender: IbcIcs20Sender,
	pub on_remote_asset: AssetId,
}

pub fn to_cw_message(memo: String, coin: Coin, route: IbcRoute) -> StdResult<CosmosMsg> {
	let transfer = picasso::IbcMsg::Transfer {
		channel_id: route.channel_to_send_to.clone(),
		to_address: route.gateway_to_send_to,
		amount: coin,
		timeout: route.counterparty_timeout,
		memo: Some(memo),
	};
	match route.ibc_ics_20_sender {
		IbcIcs20Sender::SubstratePrecompile(addr) => Ok(WasmMsg::Execute {
			contract_addr: addr.into(),
			msg: to_binary(&transfer)?,
			funds: <_>::default(),
		}
		.into()),
		_ => unimplemented!(),
	}
}
