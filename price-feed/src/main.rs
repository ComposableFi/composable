mod asset;
mod feed;
mod opts;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
use crate::{
    asset::{Asset, AssetPair},
    feed::{pyth::Pyth, FeedNotification, Price, TimeStamped},
};
use futures::stream::StreamExt;
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

async fn run_http_frontend(
    listening_address: &String,
    prices: Arc<RwLock<HashMap<AssetPair, TimeStamped<Price>>>>,
) -> (oneshot::Sender<()>, JoinHandle<()>) {
    let get_price = warp::path!("price" / Asset / Asset)
        .and(warp::get())
        .map(move |x, y| {
            let asset_pair = (x, y);
            match prices.read().unwrap().get(&asset_pair) {
                Some(latest_price) => with_status(json(latest_price), StatusCode::OK),
                // TODO: how to 404 without content???
                None => with_status(json(&"NOT_FOUND"), StatusCode::NOT_FOUND),
            }
        });

    // Channel that will allow warp to gracefully shutdown when a signal is comming.
    let (tx, rx) = oneshot::channel::<()>();
    let (_, server) = warp::serve(get_price).bind_with_graceful_shutdown(
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
    log::info!("successfully connected to pyth-client.");
    for &asset in [Asset::BTC, Asset::ETH, Asset::LTC].iter() {
        pyth.subscribe_to_asset(&feed_in, (asset, Asset::USD))
            .await
            .expect("failed to subscribe to asset");
    }

    (pyth, feed_out)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = opts::get_opts();

    let (pyth, mut feed_out) = create_subscriptions(&opts.pythd_host).await;

    let prices: Arc<RwLock<HashMap<AssetPair, TimeStamped<Price>>>> =
        Arc::new(RwLock::new(HashMap::new()));

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
                        log::info!("notification received: {:?}", notification);
                        /* TODO: how are we going to handles X feeds:
                            - do we just expose every one of them from their own endpoint?
                            - do we merge the prices (median?), if so, merging will depend on timestamp?
                          On notification close, do we remove the price as we are no
                             longer getting new prices?
                        */
                        if let FeedNotification::PriceUpdated(_, a, p) = notification {
                            prices.write().expect("failed to acquire write lock...").insert(a, p);
                        };
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
