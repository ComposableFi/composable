use crate::{cache::Cache, feed::FeedNotification};
use futures::stream::{Fuse, StreamExt};
use signal_hook_tokio::SignalsInfo;
use std::{convert::TryFrom, fmt::Debug};
use tokio::{sync::mpsc, task::JoinHandle};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum FeedNotificationAction<K, V> {
    UpdateCache(K, V),
}

pub trait Transition<S> {
    fn apply(&self, state: &mut S);
}

impl<TCache, TAsset, TPrice> Transition<TCache> for FeedNotificationAction<TAsset, TPrice>
where
    TCache: Cache<TAsset, TPrice>,
    TAsset: Copy,
    TPrice: Copy,
{
    fn apply(&self, state: &mut TCache) {
        match self {
            &FeedNotificationAction::UpdateCache(a, p) => state.insert(a, p),
        }
    }
}

impl<TAsset, TPrice> TryFrom<FeedNotification<TAsset, TPrice>>
    for FeedNotificationAction<TAsset, TPrice>
where
    TAsset: Debug + Copy,
    TPrice: Copy,
{
    type Error = ();
    /* TODO: how are we going to handles X feeds:
      - do we just expose every one of them from their own endpoint?
      - do we merge the prices (median?), if so, merging will depend on timestamp?
      On notification close, do we remove the price as we are no
      longer getting new prices?
    */
    fn try_from(
        notification: FeedNotification<TAsset, TPrice>,
    ) -> Result<FeedNotificationAction<TAsset, TPrice>, Self::Error> {
        match notification {
            FeedNotification::Opened(f, a) => {
                log::info!("{:?} has opened a channel for {:?}", f, a);
                Err(())
            }
            FeedNotification::Closed(f, a) => {
                log::info!("{:?} has closed a channel for {:?}", f, a);
                Err(())
            }
            FeedNotification::PriceUpdated(_, a, p) => {
                Ok(FeedNotificationAction::UpdateCache(a, p))
            }
        }
    }
}

pub struct Backend {
    pub shutdown_handle: JoinHandle<()>,
}

impl Backend {
    pub async fn new<TNotification, TTransition, TAsset, TPrice, TCache>(
        mut prices_cache: TCache,
        mut feed_channel: mpsc::Receiver<TNotification>,
        mut shutdown_trigger: Fuse<SignalsInfo>,
    ) -> Backend
    where
        TCache: 'static + Cache<TAsset, TPrice> + Send,
        TNotification: 'static + Send + Debug,
        TTransition: Transition<TCache> + TryFrom<TNotification>,
    {
        let backend = async move {
            'l: loop {
                tokio::select! {
                    _ = shutdown_trigger.next() => {
                        log::info!("terminating signal received.");
                        break 'l;
                    }
                    message = feed_channel.recv() => {
                        match message {
                            Some(notification) => {
                                log::debug!("notification received: {:?}", notification);
                                let _ = TTransition::try_from(notification)
                                    .map(|action| {
                                        action.apply(&mut prices_cache);
                                    });
                            }
                            None => {
                                log::info!("no more feed available... exiting handler.");
                                break 'l;
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

#[cfg(test)]
mod tests {
    use super::Backend;
    use crate::{
        asset::{AssetPair, VALID_ASSETPAIRS},
        backend::{FeedNotificationAction, Transition},
        cache::{PriceCache, ThreadSafePriceCache},
        feed::{Exponent, Feed, FeedNotification, Price, TimeStamp, TimeStamped, TimeStampedPrice},
    };
    use futures::stream::StreamExt;
    use signal_hook_tokio::Signals;
    use std::{
        collections::HashMap,
        convert::TryFrom,
        sync::{Arc, RwLock},
    };
    use tokio::sync::mpsc;

    #[test]
    fn test_feed_notification_transition() {
        let feed = Feed::Pyth;
        let timestamped_price = TimeStamped {
            value: (Price(0xCAFEBABE), Exponent(0x1337)),
            timestamp: TimeStamp(0xDEADC0DE),
        };
        VALID_ASSETPAIRS.iter().for_each(|&asset_pair| {
            [
                (FeedNotification::Opened(feed, asset_pair), None),
                (FeedNotification::Closed(feed, asset_pair), None),
                (
                    FeedNotification::PriceUpdated(feed, asset_pair, timestamped_price),
                    Some((
                        FeedNotificationAction::UpdateCache(asset_pair, timestamped_price),
                        [(asset_pair, timestamped_price)].iter().copied().collect(),
                    )),
                ),
            ]
            .iter()
            .for_each(|(notification, expected)| {
                match (
                    FeedNotificationAction::<AssetPair, TimeStampedPrice>::try_from(*notification),
                    expected,
                ) {
                    (Ok(actual_action), Some((expected_action, expected_state))) => {
                        assert_eq!(&actual_action, expected_action);
                        let mut state = HashMap::new();
                        actual_action.apply(&mut state);
                        assert_eq!(&state, expected_state);
                    }
                    _ => {
                        // No action = no transition
                    }
                }
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
        for &asset_pair in VALID_ASSETPAIRS.iter() {
            let tests = [
                (
                    vec![
                        FeedNotification::Opened(feed, asset_pair),
                        FeedNotification::PriceUpdated(feed, asset_pair, price1),
                        FeedNotification::Closed(feed, asset_pair),
                    ],
                    [(asset_pair, price1)],
                ),
                (
                    vec![
                        FeedNotification::Opened(feed, asset_pair),
                        FeedNotification::PriceUpdated(feed, asset_pair, price3),
                        FeedNotification::PriceUpdated(feed, asset_pair, price1),
                        FeedNotification::PriceUpdated(feed, asset_pair, price2),
                        FeedNotification::Closed(feed, asset_pair),
                    ],
                    [(asset_pair, price2)],
                ),
                (
                    vec![
                        FeedNotification::Opened(feed, asset_pair),
                        FeedNotification::PriceUpdated(feed, asset_pair, price2),
                        FeedNotification::PriceUpdated(feed, asset_pair, price1),
                        FeedNotification::PriceUpdated(feed, asset_pair, price3),
                        FeedNotification::Closed(feed, asset_pair),
                    ],
                    [(asset_pair, price3)],
                ),
            ];
            for (events, expected) in &tests {
                let prices_cache: ThreadSafePriceCache = Arc::new(RwLock::new(HashMap::new()));
                let (feed_in, feed_out) =
                    mpsc::channel::<FeedNotification<AssetPair, TimeStampedPrice>>(8);
                let signals = Signals::new(&[])
                    .expect("could not create signals stream")
                    .fuse();
                let backend = Backend::new::<
                    FeedNotification<AssetPair, TimeStampedPrice>,
                    FeedNotificationAction<AssetPair, TimeStampedPrice>,
                    _,
                    _,
                    _,
                >(prices_cache.clone(), feed_out, signals)
                .await;

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
