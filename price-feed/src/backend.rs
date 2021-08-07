use crate::{
    asset::*,
    cache::{PriceCache, PriceCacheEntry},
    feed::{pyth::Pyth, FeedNotification},
};
use futures::stream::{Fuse, StreamExt};
use signal_hook_tokio::SignalsInfo;
use std::sync::{Arc, RwLock};
use tokio::{sync::mpsc, task::JoinHandle};
use url::Url;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum FeedNotificationAction {
    UpdateCache(AssetPairHash, PriceCacheEntry),
    NoOp,
}

pub struct Backend {
    pub shutdown_handle: JoinHandle<()>,
}

/* TODO: how are we going to handles X feeds:
 - do we just expose every one of them from their own endpoint?
 - do we merge the prices (median?), if so, merging will depend on timestamp?
 On notification close, do we remove the price as we are no
 longer getting new prices?
*/
fn feed_notification_action(feed_notification: &FeedNotification) -> FeedNotificationAction {
    match feed_notification {
        FeedNotification::Opened(f, a) => {
            log::info!("{:?} has opened a channel for {:?}", f, a);
            FeedNotificationAction::NoOp
        }
        FeedNotification::Closed(f, a) => {
            log::info!("{:?} has closed a channel for {:?}", f, a);
            FeedNotificationAction::NoOp
        }
        FeedNotification::PriceUpdated(_, a, p) => FeedNotificationAction::UpdateCache(
            *ASSETPAIR_HASHES.get(&a).expect("unknown asset pair hash"),
            *p,
        ),
    }
}

impl Backend {
    pub async fn new(
        prices_cache: Arc<RwLock<PriceCache>>,
        mut feed_channel: mpsc::Receiver<FeedNotification>,
        mut shutdown_trigger: Fuse<SignalsInfo>,
    ) -> Backend {
        let backend = async move {
            'a: loop {
                tokio::select! {
                    _ = shutdown_trigger.next() => {
                        log::info!("terminating signal received.");
                        break 'a;
                    }
                    message = feed_channel.recv() => {
                        match message {
                            Some(feed_notification) => {
                                log::debug!("notification received: {:?}", feed_notification);
                                match feed_notification_action(&feed_notification) {
                                    FeedNotificationAction::UpdateCache(k, v) => {
                                        prices_cache
                                            .write()
                                            .expect("failed to acquire write lock...")
                                            .insert(
                                                k,
                                                v,
                                            );
                                    }
                                    FeedNotificationAction::NoOp => {}
                                }
                            }
                            None => {
                                log::info!("no more feed available... exiting handler.");
                                break 'a;
                            }
                        }
                    }
                }
            }
        };

        let shutdown_handle = tokio::spawn(backend);

        Backend { shutdown_handle }
    }
}

// TODO: manage multiple feeds
pub async fn run_pyth_feed(pythd_host: &String) -> (Pyth, mpsc::Receiver<FeedNotification>) {
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

#[cfg(test)]
mod tests {
    use super::Backend;
    use crate::{
        asset::ASSETPAIR_HASHES,
        asset::VALID_ASSETPAIRS,
        backend::{feed_notification_action, FeedNotificationAction},
        cache::PriceCache,
        feed::{Exponent, Feed, FeedNotification, Price, TimeStamp, TimeStamped},
    };
    use futures::stream::StreamExt;
    use signal_hook_tokio::Signals;
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };
    use tokio::sync::mpsc;

    #[test]
    fn test_feed_notification_action() {
        let feed = Feed::Pyth;
        let timestamped_price = TimeStamped {
            value: (Price(0xCAFEBABE), Exponent(0x1337)),
            timestamp: TimeStamp(0xDEADC0DE),
        };
        VALID_ASSETPAIRS.iter().for_each(|&asset_pair| {
            let asset_pair_hash = asset_pair.hash();
            [
                (
                    FeedNotification::Opened(feed, asset_pair),
                    FeedNotificationAction::NoOp,
                ),
                (
                    FeedNotification::Closed(feed, asset_pair),
                    FeedNotificationAction::NoOp,
                ),
                (
                    FeedNotification::PriceUpdated(feed, asset_pair, timestamped_price),
                    FeedNotificationAction::UpdateCache(asset_pair_hash, timestamped_price),
                ),
            ]
            .iter()
            .for_each(|&(notification, expected_action)| {
                assert_eq!(feed_notification_action(&notification), expected_action);
            });
        });
    }

    #[tokio::test]
    async fn test_feed_backend() {
        let mk_price = |x, y, z| TimeStamped {
            value: (Price(x), Exponent(y)),
            timestamp: TimeStamp(z),
        };
        let (price1, price2, price3) = (
            mk_price(123, -3, 2),
            mk_price(3134, -1, 5),
            mk_price(93424, -4, 234),
        );
        let feed = Feed::Pyth;
        for (&asset_pair, &asset_pair_hash) in ASSETPAIR_HASHES.iter() {
            let tests = [
                (
                    vec![
                        FeedNotification::Opened(feed, asset_pair),
                        FeedNotification::PriceUpdated(feed, asset_pair, price1),
                        FeedNotification::Closed(feed, asset_pair),
                    ],
                    [(asset_pair_hash, price1)],
                ),
                (
                    vec![
                        FeedNotification::Opened(feed, asset_pair),
                        FeedNotification::PriceUpdated(feed, asset_pair, price3),
                        FeedNotification::PriceUpdated(feed, asset_pair, price1),
                        FeedNotification::PriceUpdated(feed, asset_pair, price2),
                        FeedNotification::Closed(feed, asset_pair),
                    ],
                    [(asset_pair_hash, price2)],
                ),
                (
                    vec![
                        FeedNotification::Opened(feed, asset_pair),
                        FeedNotification::PriceUpdated(feed, asset_pair, price2),
                        FeedNotification::PriceUpdated(feed, asset_pair, price1),
                        FeedNotification::PriceUpdated(feed, asset_pair, price3),
                        FeedNotification::Closed(feed, asset_pair),
                    ],
                    [(asset_pair_hash, price3)],
                ),
            ];
            for (events, expected) in &tests {
                let prices_cache: Arc<RwLock<PriceCache>> = Arc::new(RwLock::new(HashMap::new()));
                let (feed_in, feed_out) = mpsc::channel::<FeedNotification>(8);
                let signals = Signals::new(&[])
                    .expect("could not create signals stream")
                    .fuse();
                let backend = Backend::new(prices_cache.clone(), feed_out, signals).await;

                for &event in events {
                    feed_in
                        .send(event)
                        .await
                        .expect("could not send feed notification");
                }

                /* Drop the channel so that the backend exit and we join it.
                   This will ensure the events have been processed.
                */
                drop(feed_in);
                backend
                    .shutdown_handle
                    .await
                    .expect("could not join on backend handle");

                let prices_cache_r = prices_cache.read().expect("could not acquire read lock");
                assert_eq!(
                    *prices_cache_r,
                    expected.iter().copied().collect::<PriceCache>()
                );
            }
        }
    }
}
