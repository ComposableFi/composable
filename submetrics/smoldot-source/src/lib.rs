use core::{iter, num::NonZeroU32};
use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures_util::StreamExt;
use hashbrown::HashMap;
use jsonrpsee::types::Id;
use smoldot::{executor::host::StorageKey, json_rpc::methods::HexString};
use smoldot_light::ChainId;

type StoragePrefix = String;

#[derive(serde::Serialize)]
struct GetStorageParams {
    key: HexString,
}

#[derive(serde::Serialize)]
struct StateSubscribeStorage {
    list: [HexString],
}

pub async fn composable_polkadot(
    sink: UnboundedSender<String>,
    mut storage_requests: UnboundedReceiver<subxt::storage::StorageKey>,
) {
    const deployment: &str = "composable-polkadot";
    const para_spec : &str = include_str!("../../../code/parachain/node/src/res/composable.json");
    const relay_spec: &str = include_str!(
        "../../../../../../github.com/smol-dot/smoldot/demo-chain-specs/polkadot.json"
    );

    let mut client = smoldot_light::Client::new(
        smoldot_light::platform::async_std::AsyncStdTcpWebSocket::new(
            env!("CARGO_PKG_NAME").into(),
            env!("CARGO_PKG_VERSION").into(),
        ),
    );

    let smoldot_light::AddChainSuccess {
        chain_id: relay_chain_id,
        json_rpc_responses: relay_json_rpc_responses,
    } = client
        .add_chain(smoldot_light::AddChainConfig {
            specification: relay_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: NonZeroU32::new(128).unwrap(),
                max_subscriptions: 1024,
            },
            potential_relay_chains: iter::empty(),
            database_content: "",
            user_data: (),
        })
        .unwrap();

    let smoldot_light::AddChainSuccess {
        chain_id,
        json_rpc_responses,
    } = client
        .add_chain(smoldot_light::AddChainConfig {
            specification: para_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: NonZeroU32::new(128).unwrap(),
                max_subscriptions: 1024,
            },
            potential_relay_chains: [relay_chain_id].into_iter(),
            database_content: "",
            user_data: (),
        })
        .unwrap();

    let mut json_rpc_responses = json_rpc_responses.unwrap();

    let mut counter: u64 = 1;

    client
        .json_rpc_request(
r#"{"id":1,"jsonrpc":"2.0","method":"state_subscribeStorage","params": { "list": ["0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7", "0x99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d51", "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9", "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9004325711314fc9a69f6de0d037dd9126e61132120b0eb4a95015ecb49a30a7893b34709c590f2414148f5c8e4460516"] }}"#,
chain_id,
        )
        .unwrap();

    log::info!("relay_chain_id {:?}", relay_chain_id);
    log::info!("chain_id {:?}", chain_id);
    loop {
        while let Ok(Some(next)) = storage_requests.try_next() {
            log::debug!("query");
            let key = GetStorageParams {
                key: HexString(next.as_ref().to_vec()),
            };
            let key = serde_json::value::to_raw_value(&key).unwrap();
            let request = jsonrpsee::types::Request::new(
                "state_getStorage".into(),
                Some(key.as_ref()),
                Id::Str(HexString(next.as_ref().to_vec()).to_string().into()),
            );
            client
                .json_rpc_request(serde_json::to_string(&request).unwrap(), chain_id)
                .unwrap();
        }
        log::info!("{}", deployment);
        let response = json_rpc_responses.next().await.unwrap();
        sink.unbounded_send(response).unwrap();
    }
}

pub async fn picasso_kusama(
    sink: UnboundedSender<String>,
    mut storage_requests: UnboundedReceiver<subxt::storage::StorageKey>,
) {
    const deployment: &str = "picasso-kusama";
    const para_spec : &str = include_str!("../../../code/parachain/node/src/res/picasso.json");
    const relay_spec: &str = include_str!(
        "../../../../../../github.com/smol-dot/smoldot/demo-chain-specs/kusama.json"
    );

    let mut client = smoldot_light::Client::new(
        smoldot_light::platform::async_std::AsyncStdTcpWebSocket::new(
            env!("CARGO_PKG_NAME").into(),
            env!("CARGO_PKG_VERSION").into(),
        ),
    );

    let smoldot_light::AddChainSuccess {
        chain_id: relay_chain_id,
        json_rpc_responses: relay_json_rpc_responses,
    } = client
        .add_chain(smoldot_light::AddChainConfig {
            specification: relay_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: NonZeroU32::new(128).unwrap(),
                max_subscriptions: 1024,
            },
            potential_relay_chains: iter::empty(),
            database_content: "",
            user_data: (),
        })
        .unwrap();

    let smoldot_light::AddChainSuccess {
        chain_id,
        json_rpc_responses,
    } = client
        .add_chain(smoldot_light::AddChainConfig {
            specification: para_spec,
            json_rpc: smoldot_light::AddChainConfigJsonRpc::Enabled {
                max_pending_requests: NonZeroU32::new(128).unwrap(),
                max_subscriptions: 1024,
            },
            potential_relay_chains: [relay_chain_id].into_iter(),
            database_content: "",
            user_data: (),
        })
        .unwrap();

    let mut json_rpc_responses = json_rpc_responses.unwrap();

    let mut counter: u64 = 1;

    client
        .json_rpc_request(
r#"{"id":1,"jsonrpc":"2.0","method":"state_subscribeStorage","params": { "list": ["0x26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7", "0x99971b5749ac43e0235e41b0d37869188ee7418a6531173d60d1f6a82d8f4d51", "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9", "0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9004325711314fc9a69f6de0d037dd9126e61132120b0eb4a95015ecb49a30a7893b34709c590f2414148f5c8e4460516"] }}"#,
chain_id,
        )
        .unwrap();

    log::info!("relay_chain_id {:?}", relay_chain_id);
    log::info!("chain_id {:?}", chain_id);
    loop {
        while let Ok(Some(next)) = storage_requests.try_next() {
            log::debug!("query");
            let key = GetStorageParams {
                key: HexString(next.as_ref().to_vec()),
            };
            let key = serde_json::value::to_raw_value(&key).unwrap();
            let request = jsonrpsee::types::Request::new(
                "state_getStorage".into(),
                Some(key.as_ref()),
                Id::Str(HexString(next.as_ref().to_vec()).to_string().into()),
            );
            client
                .json_rpc_request(serde_json::to_string(&request).unwrap(), chain_id)
                .unwrap();
        }
        log::info!("{}", deployment);
        let response = json_rpc_responses.next().await.unwrap();
        sink.unbounded_send(response).unwrap();
    }
}