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
    use crate::{
        asset::VALID_ASSETPAIRS,
        backend::{feed_notification_action, FeedNotificationAction},
        feed::{Exponent, Feed, FeedNotification, Price, TimeStamp, TimeStamped},
    };

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
}
