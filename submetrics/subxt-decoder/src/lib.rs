use std::{str::FromStr, io::Read};

use composable::parachain::api::preimage::events;
use jsonrpsee::types::Id;
use parity_scale_codec::Decode;
use serde::Deserialize;
use smoldot::json_rpc::methods::HexString;
use subxt::utils::{AccountId32, H256};

pub fn composable_decoder(chain_response: String) -> Vec<subchain_macro::composable::ChangeOfInterest> {
    use composable::parachain::api::runtime_types;
    use subchain_macro::composable::ChangeOfInterest;
    use composable::parachain::*;
    use composable::*;
    use composable::parachain::api::runtime_types::composable_runtime::RuntimeEvent;

    let mut result = Vec::new();
    log::debug!("{}", chain_response);

    let storage_item: Result<jsonrpsee::types::Response<String>, _> =
        serde_json::from_str(&chain_response);

    if let Ok(storage_item) = storage_item {
        let id = storage_item.id.clone();
        let storage_item = hex::decode(&storage_item.result.replace("0x", ""));
        if let Ok(data) = storage_item {
            let storage_item = <runtime_types::frame_system::AccountInfo<
                u32,
                runtime_types::pallet_balances::AccountData<u128>,
            >>::decode(&mut data.as_ref());

            if let Ok(storage_item) = storage_item {
                if let Id::Str(key) = id.clone() {                    
                    let account = hex::decode(
                        key.to_string()
                            .replace("0x", "")
                            .replace(system_account_prefix, ""),
                    )
                    .unwrap();
                    let account: Vec<_> = account.into_iter().skip(16).collect();
                    let account = AccountId32(account.try_into().unwrap());
                    result.push(ChangeOfInterest::SystemAccount(vec![(
                        account,
                        storage_item,
                    )]));
                }
            }

            let storage_item = <runtime_types::orml_tokens::AccountData<
                u128,
            >>::decode(&mut data.as_ref());

            if let Ok(storage_item) = storage_item {
                if let Id::Str(key) = id {
                   
                    let account = hex::decode(
                        key.to_string()
                            .replace("0x", "")
                            .replace(tokens_accounts_prefix, ""),
                    )
                    .unwrap();
                    let account: Vec<_> = account.into_iter().skip(16).collect();
                    let real_account = AccountId32(account.clone().into_iter().take(32).collect::<Vec<_>>().try_into().unwrap());
                    let stuff : Vec<_> = account.clone().into_iter().skip(32).skip(8).collect();
                    let asset_id = u128::decode(&mut stuff.as_ref()).unwrap();
                    
                    result.push(ChangeOfInterest::TokensAccounts(vec![(
                        real_account,
                        asset_id,
                        storage_item,
                    )]));
                }
            }
        }
    }

    let changeset: Result<
        jsonrpsee::types::SubscriptionResponse<subxt::rpc::types::StorageChangeSet<H256>>,
        _,
    > = serde_json::from_str(&chain_response);

    log::debug!("changeset {:?}", &changeset);
    
    if let Ok(events) = changeset {
        for (key, changeset) in events.params.result.changes {
            if key.0 == hex::decode(system_events).unwrap() {
                log::trace!("event");
                if let Some(events) = changeset {
                    let events: Result<
                        std::vec::Vec<
                            runtime_types::frame_system::EventRecord<
                                RuntimeEvent,
                                ::subxt::utils::H256,
                            >,
                        >,
                        _,
                    > = parity_scale_codec::Decode::decode(&mut events.as_ref());

                    match events {
                        Ok(events) => {
                            result.append(
                            &mut events
                                .into_iter()
                                .filter_map(|x| match x.event {
                                    RuntimeEvent::Ibc(x) => {
                                        Some(ChangeOfInterest::Ibc(x))
                                    }
                                    RuntimeEvent::Tokens(x) => {
                                        Some(ChangeOfInterest::Tokens(x))
                                    }
                                    RuntimeEvent::Balances(x) => {
                                        Some(ChangeOfInterest::Balances(x))
                                    }
                                    RuntimeEvent::PolkadotXcm(x) => {
                                        Some(ChangeOfInterest::PolkadotXcm(x))
                                    }
                                    RuntimeEvent::Ics20Fee(x) => {
                                        Some(ChangeOfInterest::Ics20Fee(x))
                                    }
                                    RuntimeEvent::PolkadotXcm(x) => {
                                        Some(ChangeOfInterest::PolkadotXcm(x))
                                    },
                                    _ => None,
                                })
                                .collect(),
                        );
                        }
                        Err(error) => log::error!("failed ot parse event{}", error),
                    }
                }
            } else {
                log::error!("failed to parse {:?}", &key);
            }
        }
    }
    return result;
}


pub fn picasso_decoder(chain_response: String) -> Vec<subchain_macro::picasso::ChangeOfInterest> {
    use picasso::parachain::api::runtime_types;
    use subchain_macro::picasso::ChangeOfInterest;
    use picasso::parachain::*;
    use picasso::*;
    use picasso::parachain::api::runtime_types::picasso_runtime::RuntimeEvent;

    let mut result = Vec::new();
    log::debug!("{}", chain_response);

    let storage_item: Result<jsonrpsee::types::Response<String>, _> =
        serde_json::from_str(&chain_response);

    if let Ok(storage_item) = storage_item {
        let id = storage_item.id.clone();
        let storage_item = hex::decode(&storage_item.result.replace("0x", ""));
        if let Ok(data) = storage_item {
            let storage_item = <runtime_types::frame_system::AccountInfo<
                u32,
                runtime_types::pallet_balances::AccountData<u128>,
            >>::decode(&mut data.as_ref());

            if let Ok(storage_item) = storage_item {
                if let Id::Str(key) = id.clone() {                    
                    let account = hex::decode(
                        key.to_string()
                            .replace("0x", "")
                            .replace(system_account_prefix, ""),
                    )
                    .unwrap();
                    let account: Vec<_> = account.into_iter().skip(16).collect();
                    let account = AccountId32(account.try_into().unwrap());
                    result.push(ChangeOfInterest::SystemAccount(vec![(
                        account,
                        storage_item,
                    )]));
                }
            }

            let storage_item = <runtime_types::orml_tokens::AccountData<
                u128,
            >>::decode(&mut data.as_ref());

            if let Ok(storage_item) = storage_item {
                if let Id::Str(key) = id {
                   
                    let account = hex::decode(
                        key.to_string()
                            .replace("0x", "")
                            .replace(tokens_accounts_prefix, ""),
                    )
                    .unwrap();
                    let account: Vec<_> = account.into_iter().skip(16).collect();
                    let real_account = AccountId32(account.clone().into_iter().take(32).collect::<Vec<_>>().try_into().unwrap());
                    let stuff : Vec<_> = account.clone().into_iter().skip(32).skip(8).collect();
                    let asset_id = u128::decode(&mut stuff.as_ref()).unwrap();
                    
                    result.push(ChangeOfInterest::TokensAccounts(vec![(
                        real_account,
                        asset_id,
                        storage_item,
                    )]));
                }
            }
        }
    }

    let changeset: Result<
        jsonrpsee::types::SubscriptionResponse<subxt::rpc::types::StorageChangeSet<H256>>,
        _,
    > = serde_json::from_str(&chain_response);

    log::debug!("changeset {:?}", &changeset);
    
    if let Ok(events) = changeset {
        for (key, changeset) in events.params.result.changes {
            if key.0 == hex::decode(system_events).unwrap() {
                log::trace!("event");
                if let Some(events) = changeset {
                    let events: Result<
                        std::vec::Vec<
                            runtime_types::frame_system::EventRecord<
                                RuntimeEvent,
                                ::subxt::utils::H256,
                            >,
                        >,
                        _,
                    > = parity_scale_codec::Decode::decode(&mut events.as_ref());

                    match events {
                        Ok(events) => {
                            result.append(
                            &mut events
                                .into_iter()
                                .filter_map(|x| match x.event {
                                    RuntimeEvent::Ibc(x) => {
                                        Some(ChangeOfInterest::Ibc(x))
                                    }
                                    RuntimeEvent::Tokens(x) => {
                                        Some(ChangeOfInterest::Tokens(x))
                                    }
                                    RuntimeEvent::Balances(x) => {
                                        Some(ChangeOfInterest::Balances(x))
                                    }
                                    RuntimeEvent::PolkadotXcm(x) => {
                                        Some(ChangeOfInterest::PolkadotXcm(x))
                                    }
                                    RuntimeEvent::Ics20Fee(x) => {
                                        Some(ChangeOfInterest::Ics20Fee(x))
                                    }
                                    RuntimeEvent::PolkadotXcm(x) => {
                                        Some(ChangeOfInterest::PolkadotXcm(x))
                                    },
                                    _ => None,
                                })
                                .collect(),
                        );
                        }
                        Err(error) => log::error!("failed ot parse event{}", error),
                    }
                }
            } else {
                log::error!("failed to parse {:?}", &key);
            }
        }
    }
    return result;
}


#[cfg(test)]
mod test {
    use std::str::FromStr;

    use subxt::{
        config::substrate::BlakeTwo256,
        utils::{AccountId32, H256},
    };

    use composable::parachain::{self, api::runtime_types};

    #[test]
    fn tokens_decode() {
        use composable::parachain::*;
        let real = hex::decode("000000000000000001000000000000007f635f78170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let storage_item = <runtime_types::frame_system::AccountInfo<
        u32,
        runtime_types::pallet_balances::AccountData<u128>,
    >>::decode(&mut real.as_ref());
    }

    #[test]
    fn account_key_decode() {
        use composable::parachain::*;
        let real = hex::decode("0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9978b127a950425e07500a99d2d31ff6eb8e39e87c0fec96f7d012d31a4c27b44bfb504ab359662112e4270e380c84341".to_string().replace("0x", "").replace(composable::system_account_prefix, "")).unwrap();
        let real: Vec<_> = real.into_iter().skip(16).collect();
        AccountId32(real.try_into().unwrap());
    }

    #[test]
    fn prefix() {
        let real = "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da95a3fb8de4321e12fad081eaeece61bc56d6f646c506f745374616b650000000000000000000000000000000000000000";
        let account = "62qUEaQqPiAcWAG12fT7qXxpiSRBdHcKdeL7VU5Xrg98Kkrf";

        let account = AccountId32::from_str(account).unwrap();
        let key = format!(
            "{}{}{}",
            composable::system_account_prefix,
            hex::encode(sp_core_hashing::blake2_128(&account.0)),
            hex::encode(account.0)
        );
        panic!("{}", key);
    }

    #[test]
    fn decode() {
        let events = r#"{"jsonrpc":"2.0","method":"state_storage","params":{"subscription":"0","result":{"block":"0x0ebcd4c1f46d2b7b71a2311d507045eda8cfeca34c17af8d4ec228665dafb207","changes":[["0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7","0x0c0206026d6f646c506f745374616b6500000000000000000000000000000000000000005858ff9beaa2acafd65b549ac0fb49a60f9181ecaa058be59014505882a92223cf0982000908000000000000000000000000000000000000c2f69f3f000200000001000000000002c0cd1700020100"]]}}}"#;

        let events: jsonrpsee::types::SubscriptionResponse<
            subxt::rpc::types::StorageChangeSet<H256>,
        > = serde_json::from_str(events).unwrap();
        let events = events.params.result.changes;
        for (key, events) in events {
            let events: std::vec::Vec<
                runtime_types::frame_system::EventRecord<
                    runtime_types::composable_runtime::RuntimeEvent,
                    ::subxt::utils::H256,
                >,
            > = parity_scale_codec::Decode::decode(&mut events.unwrap().as_ref()).unwrap();
            println!("{:?}", events)
        }
    }
}
