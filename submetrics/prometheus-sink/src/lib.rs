#![allow(unused, unused_variables)]
use core::{iter, num::NonZeroU32};

use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures_util::StreamExt;
use hashbrown::{HashMap, HashSet};
use lazy_static::lazy_static;
use metrics::{counter, gauge, increment_counter, increment_gauge, register_counter};
use subxt::{storage::StorageKey, utils::AccountId32};

pub enum ChangeOfInterest {
    Composable(Vec<subchain_macro::composable::ChangeOfInterest>),
    Picasso(Vec<subchain_macro::picasso::ChangeOfInterest>),
}

pub async fn main(
    sink: UnboundedReceiver<ChangeOfInterest>,
    composable_request_sender: UnboundedSender<StorageKey>,
    picasso_request_sender: UnboundedSender<StorageKey>,
) {
    let mut stream = sink.enumerate();
    while let Some((_, events)) = stream.next().await {
        match events {
            ChangeOfInterest::Composable(events) => {
                handle_composable(events, &composable_request_sender, "composable", "2");
            }
            ChangeOfInterest::Picasso(events) => {
                handle_picasso(events, &picasso_request_sender, "picasso", "1");
            }
        }
    }
}

fn handle_picasso(
    events: Vec<subchain_macro::picasso::ChangeOfInterest>,
    request_sender: &UnboundedSender<StorageKey>,
    chain: &'static str,
    native_asset_id: &'static str,
) {
    use picasso::*;
    use subchain_macro::picasso::ChangeOfInterest;
    for event in events {
        match event {
    ChangeOfInterest::SystemAccount(event) =>{
for (account,i) in event {
    gauge!("substrate_storage_system_account_free", i.data.free as f64, "account" => account.to_string(), "chain" => chain, "asset_id" => native_asset_id);
}
    },
    ChangeOfInterest::Balances(event) => {
match event {
    parachain::api::runtime_types::pallet_balances::pallet::Event::Endowed { account, free_balance } => {
   increment_gauge!("substrate_events_balances_endowed",  free_balance as f64, "account" => account.to_string(), "asset_id" => native_asset_id);
   submetrics_core::request_system_account(request_sender, &account, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::DustLost { account, amount } => {
   increment_gauge!("substrate_events_balances_dust_lost",  amount as f64, "account" => account.to_string(), "asset_id" => native_asset_id);
   submetrics_core::request_system_account(request_sender, &account, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Transfer { from, to, amount } => {
   increment_gauge!("substrate_events_balances_transfer",  amount as f64, "from" => from.to_string(), "to" => to.to_string(), "asset_id" => native_asset_id, "chain" => chain);
   submetrics_core::request_system_account(request_sender, &from, system_account_prefix,);
   submetrics_core::request_system_account(request_sender, &to, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::BalanceSet { who, free, reserved } => {
   gauge!("substrate_storage_system_account_free", free as f64, "account" => who.to_string(), "chain" => chain, "asset_id" => native_asset_id);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Reserved { who, amount } => {
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Unreserved { who, amount } => {
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::ReserveRepatriated { from, to, amount, destination_status } => {
   submetrics_core::request_system_account(request_sender, &from, system_account_prefix,);
   submetrics_core::request_system_account(request_sender, &to, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Deposit { who, amount } => {
   increment_gauge!("substrate_events_balances_deposit",  amount as f64, "who" => who.to_string(), "asset_id" => native_asset_id, "chain" => chain);       
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Withdraw { who, amount } => {
   increment_gauge!("substrate_events_balances_withdraw",  amount as f64, "who" => who.to_string(), "asset_id" => native_asset_id, "chain" => chain);       
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);

    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Slashed { who, amount } => (),
}
    }
    ChangeOfInterest::Ibc(event) => {
match event {
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferCompleted {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   increment_gauge!("substrate_events_ibc_transfer_completed",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    },
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferInitiated {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   let from = String::from_utf8_lossy(&from).to_string();
   let to = String::from_utf8_lossy(&to).to_string();
   increment_gauge!("substrate_events_ibc_transfer_initiated",  amount as f64, "from" => from,  "to" => to, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    }
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferFailed {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   increment_gauge!("substrate_events_ibc_transfer_failed",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    },
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferTimeout {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   increment_gauge!("substrate_events_ibc_transfer_timeout",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::TokenReceived { from, to, ibc_denom, local_asset_id, amount, is_receiver_source, source_channel, destination_channel } => {
   increment_gauge!("substrate_events_ibc_transfer_received",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_receiver_source" => is_receiver_source.to_string(), "chain" => chain);     
    },
   parachain::api::runtime_types::pallet_ibc::pallet::Event::Events { events } => {
   for e in events {
       match e {
   Ok(a) => {
match a {
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::NewBlock { revision_height, revision_number } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::CreateClient { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::UpdateClient { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => {
increment_counter!("substrate_events_ibc_client_update","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::UpgradeClient { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::ClientMisbehaviour { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => {
increment_counter!("substrate_events_ibc_client_misbehaviour","chain" => chain);     
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenInitConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenConfirmConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenTryConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenAckConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenInitChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenConfirmChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenTryChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenAckChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::CloseInitChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => {
increment_counter!("substrate_events_ibc_channel_close_init","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::CloseConfirmChannel { revision_height, revision_number, channel_id, port_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::ReceivePacket { revision_height, revision_number, port_id, channel_id, dest_port, dest_channel, sequence } => {
increment_counter!("substrate_events_ibc_packet_receive","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::SendPacket { revision_height, revision_number, port_id, channel_id, dest_port, dest_channel, sequence } => {
increment_counter!("substrate_events_ibc_packet_sent","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::AcknowledgePacket { revision_height, revision_number, port_id, channel_id, sequence } => {
increment_counter!("substrate_events_ibc_packet_acknowledge","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::WriteAcknowledgement { revision_height, revision_number, port_id, channel_id, dest_port, dest_channel, sequence } => {
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::TimeoutPacket { revision_height, revision_number, port_id, channel_id, sequence } => {

increment_counter!("substrate_events_ibc_packet_timeout","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::TimeoutOnClosePacket { revision_height, revision_number, port_id, channel_id, sequence } => {
    },

    // what are these 3 events?
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::Empty => {},
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::ChainError => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::AppModule { kind, module_id } => (),
}
   },
   Err(error) => {
match error {
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics02Client { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics03Connection { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics04Channel { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics20FungibleTokenTransfer { message } => {

    },
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::UnknownMessageTypeUrl { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::MalformedMessageBytes { message } => (),
}
   },
       }
   }
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ChannelOpened { channel_id, port_id } => {},
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ParamsUpdated { send_enabled, receive_enabled } => {},
    parachain::api::runtime_types::pallet_ibc::pallet::Event::OnRecvPacketError { msg } => {
   increment_counter!("substrate_events_ibc_receive_packet_error", "chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ClientUpgradeSet => {},
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ClientFrozen { client_id, height, revision_number } => {
   increment_counter!("substrate_events_ibc_client_frozen", "chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::AssetAdminUpdated { admin_account } => {},
}
    },
    ChangeOfInterest::Tokens(events) => {
use parachain::api::runtime_types::orml_tokens::module::Event;
match events {
    Event::Endowed { currency_id, who, amount } => {
   increment_gauge!("substrate_events_tokens_endowed",  amount as f64, "who" => who.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   submetrics_core::request_tokens_account(request_sender, &who, tokens_accounts_prefix, currency_id.0);
    },
    Event::DustLost { currency_id, who, amount } => (),
    Event::Transfer { currency_id, from, to, amount } => {
   increment_gauge!("substrate_events_tokens_transfer",  amount as f64, "from" => from.to_string(), "to" => to.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   submetrics_core::request_tokens_account(request_sender, &to, tokens_accounts_prefix, currency_id.0);
    },
    Event::Reserved { currency_id, who, amount } => (),
    Event::Unreserved { currency_id, who, amount } => (),
    Event::ReserveRepatriated { currency_id, from, to, amount, status } => (),
    Event::BalanceSet { currency_id, who, free, reserved } => {
   increment_gauge!("substrate_events_tokens_balance_set",  free as f64, "who" => who.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   gauge!("substrate_storage_system_account_free", free as f64, "account" => who.to_string(), "chain" => chain, "asset_id" => native_asset_id);
    },
    Event::TotalIssuanceSet { currency_id, amount } => (),
    Event::Withdrawn { currency_id, who, amount } => {
   increment_gauge!("substrate_events_tokens_withdrawn",  amount as f64, "who" => who.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   submetrics_core::request_tokens_account(request_sender, &who, tokens_accounts_prefix, currency_id.0);
    },
    Event::Slashed { currency_id, who, free_amount, reserved_amount } => (),
    Event::Deposited { currency_id, who, amount } => (),
    Event::LockSet { lock_id, currency_id, who, amount } => (),
    Event::LockRemoved { lock_id, currency_id, who } => (),
    Event::Locked { currency_id, who, amount } => (),
    Event::Unlocked { currency_id, who, amount } => (),
}
    },
    ChangeOfInterest::TokensAccounts(events) => {
for (account,asset_id, i) in events {
    gauge!("substrate_storage_tokens_account_free", i.free as f64, "account" => account.to_string(), "chain" => chain, "asset_id" => asset_id.to_string());
}
    },
    ChangeOfInterest::Ics20Fee(events) => {
match events{
    parachain::api::runtime_types::pallet_ibc::ics20_fee::pallet::Event::IbcTransferFeeCollected { amount } => {
   increment_gauge!("substrate_events_ics20_fee_transfer_fee_collected",  amount as f64, "chain" => chain);
    },
}
    },
    _=> ()
}
    }
}

fn handle_composable(
    events: Vec<subchain_macro::composable::ChangeOfInterest>,
    request_sender: &UnboundedSender<StorageKey>,
    chain: &'static str,
    native_asset_id: &'static str,
) {
    use composable::*;
    use subchain_macro::composable::ChangeOfInterest;
    for event in events {
        match event {
    ChangeOfInterest::SystemAccount(event) =>{
for (account,i) in event {
    gauge!("substrate_storage_system_account_free", i.data.free as f64, "account" => account.to_string(), "chain" => chain, "asset_id" => native_asset_id);
}
    },
    ChangeOfInterest::Balances(event) => {
match event {
    parachain::api::runtime_types::pallet_balances::pallet::Event::Endowed { account, free_balance } => {
   increment_gauge!("substrate_events_balances_endowed",  free_balance as f64, "account" => account.to_string(), "asset_id" => native_asset_id);
   submetrics_core::request_system_account(request_sender, &account, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::DustLost { account, amount } => {
   increment_gauge!("substrate_events_balances_dust_lost",  amount as f64, "account" => account.to_string(), "asset_id" => native_asset_id);
   submetrics_core::request_system_account(request_sender, &account, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Transfer { from, to, amount } => {
   increment_gauge!("substrate_events_balances_transfer",  amount as f64, "from" => from.to_string(), "to" => to.to_string(), "asset_id" => native_asset_id, "chain" => chain);
   submetrics_core::request_system_account(request_sender, &from, system_account_prefix,);
   submetrics_core::request_system_account(request_sender, &to, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::BalanceSet { who, free, reserved } => {
   gauge!("substrate_storage_system_account_free", free as f64, "account" => who.to_string(), "chain" => chain, "asset_id" => native_asset_id);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Reserved { who, amount } => {
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Unreserved { who, amount } => {
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::ReserveRepatriated { from, to, amount, destination_status } => {
   submetrics_core::request_system_account(request_sender, &from, system_account_prefix,);
   submetrics_core::request_system_account(request_sender, &to, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Deposit { who, amount } => {
   increment_gauge!("substrate_events_balances_deposit",  amount as f64, "who" => who.to_string(), "asset_id" => native_asset_id, "chain" => chain);       
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);
    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Withdraw { who, amount } => {
   increment_gauge!("substrate_events_balances_withdraw",  amount as f64, "who" => who.to_string(), "asset_id" => native_asset_id, "chain" => chain);       
   submetrics_core::request_system_account(request_sender, &who, system_account_prefix,);

    },
    parachain::api::runtime_types::pallet_balances::pallet::Event::Slashed { who, amount } => (),
}
    }
    ChangeOfInterest::Ibc(event) => {
match event {
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferCompleted {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   increment_gauge!("substrate_events_ibc_transfer_completed",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    },
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferInitiated {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   let from = String::from_utf8_lossy(&from).to_string();
   let to = String::from_utf8_lossy(&to).to_string();
   increment_gauge!("substrate_events_ibc_transfer_initiated",  amount as f64, "from" => from,  "to" => to, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    }
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferFailed {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   increment_gauge!("substrate_events_ibc_transfer_failed",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    },
    parachain::api::runtime_types :: pallet_ibc :: pallet :: Event::TokenTransferTimeout {
   from,
   to,
   ibc_denom,
   local_asset_id,
   amount,
   is_sender_source,
   source_channel,
   destination_channel,
    } => {
   increment_gauge!("substrate_events_ibc_transfer_timeout",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_sender_source" => is_sender_source.to_string(), "chain" => chain);     
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::TokenReceived { from, to, ibc_denom, local_asset_id, amount, is_receiver_source, source_channel, destination_channel } => {
   increment_gauge!("substrate_events_ibc_transfer_received",  amount as f64, "from" => from.0,  "to" => to.0, "asset_id" => local_asset_id.unwrap().0.to_string(), "is_receiver_source" => is_receiver_source.to_string(), "chain" => chain);     
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::Events { events } => {
   for e in events {
       match e {
   Ok(a) => {
match a {
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::NewBlock { revision_height, revision_number } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::CreateClient { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::UpdateClient { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => {
increment_counter!("substrate_events_ibc_client_update","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::UpgradeClient { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::ClientMisbehaviour { client_id, client_type, revision_height, revision_number, consensus_height, consensus_revision_number } => {
increment_counter!("substrate_events_ibc_client_misbehaviour","chain" => chain);     
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenInitConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenConfirmConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenTryConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenAckConnection { revision_height, revision_number, connection_id, client_id, counterparty_connection_id, counterparty_client_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenInitChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenConfirmChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenTryChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::OpenAckChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::CloseInitChannel { revision_height, revision_number, port_id, channel_id, connection_id, counterparty_port_id, counterparty_channel_id } => {
increment_counter!("substrate_events_ibc_channel_close_init","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::CloseConfirmChannel { revision_height, revision_number, channel_id, port_id, connection_id, counterparty_port_id, counterparty_channel_id } => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::ReceivePacket { revision_height, revision_number, port_id, channel_id, dest_port, dest_channel, sequence } => {
let channel_id = String::from_utf8_lossy(&channel_id[..]).to_string();
let port_id = String::from_utf8_lossy(&port_id[..]).to_string();
increment_counter!("substrate_events_ibc_packet_receive","chain" => chain, "channel_id" => channel_id, "port_id" => port_id);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::SendPacket { revision_height, revision_number, port_id, channel_id, dest_port, dest_channel, sequence } => {
increment_counter!("substrate_events_ibc_packet_sent","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::AcknowledgePacket { revision_height, revision_number, port_id, channel_id, sequence } => {
increment_counter!("substrate_events_ibc_packet_acknowledge","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::WriteAcknowledgement { revision_height, revision_number, port_id, channel_id, dest_port, dest_channel, sequence } => {
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::TimeoutPacket { revision_height, revision_number, port_id, channel_id, sequence } => {
increment_counter!("substrate_events_ibc_packet_timeout","chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::TimeoutOnClosePacket { revision_height, revision_number, port_id, channel_id, sequence } => {
    },

    // what are these 3 events?
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::Empty => {},
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::ChainError => (),
    parachain::api::runtime_types::pallet_ibc::events::IbcEvent::AppModule { kind, module_id } => (),
}
   },
   Err(error) => {
match error {
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics02Client { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics03Connection { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics04Channel { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::Ics20FungibleTokenTransfer { message } => {

    },
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::UnknownMessageTypeUrl { message } => (),
    parachain::api::runtime_types::pallet_ibc::errors::IbcError::MalformedMessageBytes { message } => (),
}
   },
       }
   }
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ChannelOpened { channel_id, port_id } => {},
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ParamsUpdated { send_enabled, receive_enabled } => {},
    parachain::api::runtime_types::pallet_ibc::pallet::Event::OnRecvPacketError { msg } => {
   increment_counter!("substrate_events_ibc_receive_packet_error", "chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ClientUpgradeSet => {},
    parachain::api::runtime_types::pallet_ibc::pallet::Event::ClientFrozen { client_id, height, revision_number } => {
   increment_counter!("substrate_events_ibc_client_frozen", "chain" => chain);
    },
    parachain::api::runtime_types::pallet_ibc::pallet::Event::AssetAdminUpdated { admin_account } => {},
}
    },
    ChangeOfInterest::Tokens(events) => {
use parachain::api::runtime_types::orml_tokens::module::Event;
match events {
    Event::Endowed { currency_id, who, amount } => {
   increment_gauge!("substrate_events_tokens_endowed",  amount as f64, "who" => who.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   submetrics_core::request_tokens_account(request_sender, &who, tokens_accounts_prefix, currency_id.0);
    },
    Event::DustLost { currency_id, who, amount } => (),
    Event::Transfer { currency_id, from, to, amount } => {
   increment_gauge!("substrate_events_tokens_transfer",  amount as f64, "from" => from.to_string(), "to" => to.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   submetrics_core::request_tokens_account(request_sender, &to, tokens_accounts_prefix, currency_id.0);
    },
    Event::Reserved { currency_id, who, amount } => (),
    Event::Unreserved { currency_id, who, amount } => (),
    Event::ReserveRepatriated { currency_id, from, to, amount, status } => (),
    Event::BalanceSet { currency_id, who, free, reserved } => {
   increment_gauge!("substrate_events_tokens_balance_set",  free as f64, "who" => who.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   gauge!("substrate_storage_system_account_free", free as f64, "account" => who.to_string(), "chain" => chain, "asset_id" => native_asset_id);
    },
    Event::TotalIssuanceSet { currency_id, amount } => (),
    Event::Withdrawn { currency_id, who, amount } => {
   increment_gauge!("substrate_events_tokens_withdrawn",  amount as f64, "who" => who.to_string(), "asset_id" => currency_id.0.to_string(), "chain" => chain);
   submetrics_core::request_tokens_account(request_sender, &who, tokens_accounts_prefix, currency_id.0);
    },
    Event::Slashed { currency_id, who, free_amount, reserved_amount } => (),
    Event::Deposited { currency_id, who, amount } => (),
    Event::LockSet { lock_id, currency_id, who, amount } => (),
    Event::LockRemoved { lock_id, currency_id, who } => (),
    Event::Locked { currency_id, who, amount } => (),
    Event::Unlocked { currency_id, who, amount } => (),
}
    },
    ChangeOfInterest::TokensAccounts(events) => {
for (account,asset_id, i) in events {
    gauge!("substrate_storage_tokens_account_free", i.free as f64, "account" => account.to_string(), "chain" => chain, "asset_id" => asset_id.to_string());
}
    },
    ChangeOfInterest::Ics20Fee(events) => {
match events{
    parachain::api::runtime_types::pallet_ibc::ics20_fee::pallet::Event::IbcTransferFeeCollected { amount } => {
   increment_gauge!("substrate_events_ics20_fee_transfer_fee_collected",  amount as f64, "chain" => chain);
    },
}
    },
    _=> ()
}
    }
}

use axum::{extract::ConnectInfo, routing::get, Router};
use axum_prometheus::{
    metrics::describe_counter,
    metrics_exporter_prometheus::{Matcher, PrometheusBuilder},
    PrometheusMetricLayerBuilder, AXUM_HTTP_REQUESTS_DURATION_SECONDS, AXUM_HTTP_REQUESTS_PENDING,
    AXUM_HTTP_REQUESTS_TOTAL, PREFIXED_HTTP_REQUESTS_PENDING, PREFIXED_HTTP_REQUESTS_TOTAL,
    SECONDS_DURATION_BUCKETS,
};
use std::{net::SocketAddr, str::FromStr};

#[tokio::main]
pub async fn main_metrics() -> anyhow::Result<()> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_prefix("builder-example")
        .with_ignore_pattern("/metrics")
        .with_metrics_from_fn(|| {
            PrometheusBuilder::new()
                .set_buckets_for_metric(
                    Matcher::Full(AXUM_HTTP_REQUESTS_DURATION_SECONDS.to_string()),
                    SECONDS_DURATION_BUCKETS,
                )
                .unwrap()
                .install_recorder()
                .unwrap()
        })
        .build_pair();

    let app = Router::new()
        .route(
            "/",
            get(
                |ConnectInfo(remote_addr): ConnectInfo<SocketAddr>| async move {
                    format!("submetrics, {remote_addr:?}!\r\n")
                },
            ),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}
