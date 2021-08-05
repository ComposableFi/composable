mod asset;
mod feed;
mod opts;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate lazy_static;

use crate::{
    asset::*,
    feed::{pyth::Pyth, FeedNotification, Price, TimeStamped},
};
use feed::Exponent;
use futures::stream::StreamExt;
use serde::Serialize;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::{
    collections::HashMap,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock},
};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use url::Url;
use warp::{
    hyper::StatusCode,
    reply::{json, with_status},
    Filter,
};

type PriceMap = HashMap<AssetPairHash, TimeStamped<(Price, Exponent)>>;

async fn run_http_frontend(
    listening_address: &String,
    prices: Arc<RwLock<PriceMap>>,
) -> (oneshot::Sender<()>, JoinHandle<()>) {
    let get_asset_id = warp::path!("asset_id" / Asset / Asset)
        .and(warp::get())
        .map(|x, y| json(&to_hash(&(x, y))));

    let get_price = warp::path!("price" / AssetPairHash / u128)
        .and(warp::get())
        .map(move |asset_pair_hash, _request_id| {
            // TODO: What is the request_id useful for (comming from oracle pallet)?
            match prices.read().unwrap().get(&asset_pair_hash) {
                Some(&TimeStamped {
                    value: (Price(p), Exponent(q)),
                    timestamp: _,
                }) => {
                    /*
                     The oracle pallet is expecting a price in USD cents.
                     While this server handle any asset pair.
                     It make this part of code very specific...
                     Shouldn't we use the unit of value + exponent for any asset pair?
                    */
                    #[derive(Serialize, Copy, Clone, Debug)]
                    #[repr(transparent)]
                    pub struct USDPrice {
                        pub USD: Price,
                    }
                    let usd_unit_exponent = q + 2;
                    let usd_price = Price(p / u64::pow(10u64, i32::abs(usd_unit_exponent) as u32));

                    with_status(json(&USDPrice { USD: usd_price }), StatusCode::OK)
                }
                None => with_status(json(&()), StatusCode::NOT_FOUND),
            }
        });

    // Channel that will allow warp to gracefully shutdown when a signal is comming.
    let (tx, rx) = oneshot::channel::<()>();
    let (_, server) = warp::serve(get_price.or(get_asset_id)).bind_with_graceful_shutdown(
        SocketAddr::from_str(listening_address).expect("invalid listening address."),
        async {
            rx.await.ok();
        },
    );

    // Allow us to join the shutdown later.
    let server_handle = tokio::spawn(server);

    (tx, server_handle)
}

// TODO: manage multiple feeds
async fn create_subscriptions(pythd_host: &String) -> (Pyth, mpsc::Receiver<FeedNotification>) {
    /* Its important to drop the initial feed_in as it will be cloned for all subsequent tasks
       The received won't get notified if all cloned senders are closed but not the 'main' one.
    */
    let (feed_in, feed_out) = mpsc::channel::<FeedNotification>(128);

    let mut pyth = Pyth::new(&Url::parse(&pythd_host).expect("invalid pythd host address."))
        .await
        .expect("connection to pythd failed");

    // TODO: subscribe to all asset prices? cli configurable?
    log::info!("successfully connected to pythd.");
    for asset_pair in VALID_ASSETPAIRS.iter() {
        pyth.subscribe_to_asset(&feed_in, asset_pair)
            .await
            .expect("failed to subscribe to asset");
    }

    (pyth, feed_out)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = opts::get_opts();

    for (asset_pair, asset_pair_hash) in ASSETPAIR_HASHES.iter() {
        log::info!("AssetPair{:?} = AssetId({:?})", asset_pair, asset_pair_hash);
    }

    let (pyth, mut feed_out) = create_subscriptions(&opts.pythd_host).await;

    let prices: Arc<RwLock<PriceMap>> = Arc::new(RwLock::new(HashMap::new()));

    let (server_shutdown, server_handle) =
        run_http_frontend(&opts.listening_address, prices.clone()).await;

    let mut signals = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])
        .expect("could not create signals stream")
        .fuse();

    let terminate = async {
        log::info!("terminating pyth subscriptions...");
        pyth.terminate().await;
        log::info!("signaling warp for termination...");
        server_shutdown.send(()).unwrap();
        log::info!("waiting for warp to terminate...");
        server_handle.await.unwrap();
    };

    'a: loop {
        tokio::select! {
            _ = signals.next() => {
                log::info!("terminating signal received.");
                terminate.await;
                break 'a;
            }
            message = feed_out.recv() => {
                match message {
                    Some(notification) => {
                        log::debug!("notification received: {:?}", notification);
                        /* TODO: how are we going to handles X feeds:
                            - do we just expose every one of them from their own endpoint?
                            - do we merge the prices (median?), if so, merging will depend on timestamp?
                          On notification close, do we remove the price as we are no
                             longer getting new prices?
                        */
                        match notification {
                            FeedNotification::Opened(f, a) => {
                                log::info!("{:?} has opened a channel for {:?}", f, a);
                            }
                            FeedNotification::Closed(f, a) => {
                                log::info!("{:?} has closed a channel for {:?}", f, a);
                            }
                            FeedNotification::PriceUpdated(_, a, p) => {
                                prices
                                    .write()
                                    .expect("failed to acquire write lock...")
                                    .insert(*ASSETPAIR_HASHES.get(&a).expect("impossible"), p);
                            }
                        }
                    }
                    None => {
                        log::info!("no more feed available... exiting handler.");
                        terminate.await;
                        break 'a;
                    }
                }
            }
        }
    }

    log::info!("farewell.");
}
