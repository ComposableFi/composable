use crate::{cache::Cache, feed::FeedNotification};
use futures::{
	stream::{Fuse, StreamExt},
	Stream,
};
use signal_hook_tokio::SignalsInfo;
use std::{convert::TryFrom, fmt::Debug};
use tokio::task::JoinHandle;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum FeedNotificationAction<K, V> {
	UpdateCache { key: K, value: V },
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
			FeedNotificationAction::UpdateCache { key, value } => state.insert(*key, *value),
		}
	}
}

impl<TFeed, TAsset, TPrice> TryFrom<FeedNotification<TFeed, TAsset, TPrice>>
	for FeedNotificationAction<TAsset, TPrice>
where
	TFeed: Debug,
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
		notification: FeedNotification<TFeed, TAsset, TPrice>,
	) -> Result<FeedNotificationAction<TAsset, TPrice>, Self::Error> {
		match notification {
			FeedNotification::Started { feed } => {
				log::info!("{:?} started successfully", feed);
				Err(())
			},
			FeedNotification::AssetOpened { feed, asset } => {
				log::info!("{:?} has opened a channel for {:?}", feed, asset);
				Err(())
			},
			FeedNotification::AssetClosed { feed, asset } => {
				log::info!("{:?} has closed a channel for {:?}", feed, asset);
				Err(())
			},
			FeedNotification::AssetPriceUpdated { asset, price, .. } =>
				Ok(FeedNotificationAction::UpdateCache { key: asset, value: price }),
			FeedNotification::Stopped { feed } => {
				log::info!("{:?} stopped", feed);
				Err(())
			},
		}
	}
}

pub struct Backend {
	pub shutdown_handle: JoinHandle<()>,
}

impl Backend {
	pub async fn new<TNotification, TTransition, TAsset, TPrice, TCache, TStream>(
		mut prices_cache: TCache,
		mut feed_channel: TStream,
		mut shutdown_trigger: Fuse<SignalsInfo>,
	) -> Backend
	where
		TStream: 'static + Stream<Item = TNotification> + Send + Unpin,
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
					message = feed_channel.next() => {
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
		asset::{Asset, VALID_ASSETS},
		backend::{FeedNotificationAction, Transition},
		cache::{PriceCache, ThreadSafePriceCache},
		feed::{
			Exponent, FeedIdentifier, FeedNotification, Price, TimeStamp, TimeStamped,
			TimeStampedPrice,
		},
	};
	use futures::stream::StreamExt;
	use signal_hook_tokio::Signals;
	use std::{
		collections::HashMap,
		convert::TryFrom,
		sync::{Arc, RwLock},
	};
	use tokio::sync::mpsc;
	use tokio_stream::wrappers::ReceiverStream;

	#[test]
	fn test_feed_notification_transition() {
		let feed = FeedIdentifier::Binance;
		let timestamped_price = TimeStamped {
			value: (Price(0xCAFEBABE), Exponent(0x1337)),
			timestamp: TimeStamp::now(),
		};
		VALID_ASSETS.iter().for_each(|&asset| {
			[
				(FeedNotification::AssetOpened { feed, asset }, None),
				(FeedNotification::AssetClosed { feed, asset }, None),
				(
					FeedNotification::AssetPriceUpdated { feed, asset, price: timestamped_price },
					Some((
						FeedNotificationAction::UpdateCache {
							key: asset,
							value: timestamped_price,
						},
						[(asset, timestamped_price)].iter().copied().collect(),
					)),
				),
			]
			.iter()
			.for_each(|(notification, expected)| {
				if let (Ok(actual_action), Some((expected_action, expected_state))) = (
					FeedNotificationAction::<Asset, TimeStampedPrice>::try_from(*notification),
					expected,
				) {
					assert_eq!(&actual_action, expected_action);
					let mut state = HashMap::new();
					actual_action.apply(&mut state);
					assert_eq!(&state, expected_state);
				}
			});
		});
	}

	#[tokio::test]
	async fn test_feed_backend() {
		let mk_price =
			|x, y| TimeStamped { value: (Price(x), Exponent(y)), timestamp: TimeStamp::now() };
		let (price1, price2, price3) = (mk_price(123, -3), mk_price(3134, -1), mk_price(93424, -4));
		let feed = FeedIdentifier::Binance;
		for &asset in VALID_ASSETS.iter() {
			let tests = [
				(
					vec![
						FeedNotification::AssetOpened { feed, asset },
						FeedNotification::AssetPriceUpdated { feed, asset, price: price1 },
						FeedNotification::AssetClosed { feed, asset },
					],
					[(asset, price1)],
				),
				(
					vec![
						FeedNotification::AssetOpened { feed, asset },
						FeedNotification::AssetPriceUpdated { feed, asset, price: price3 },
						FeedNotification::AssetPriceUpdated { feed, asset, price: price1 },
						FeedNotification::AssetPriceUpdated { feed, asset, price: price2 },
						FeedNotification::AssetClosed { feed, asset },
					],
					[(asset, price2)],
				),
				(
					vec![
						FeedNotification::AssetOpened { feed, asset },
						FeedNotification::AssetPriceUpdated { feed, asset, price: price2 },
						FeedNotification::AssetPriceUpdated { feed, asset, price: price1 },
						FeedNotification::AssetPriceUpdated { feed, asset, price: price3 },
						FeedNotification::AssetClosed { feed, asset },
					],
					[(asset, price3)],
				),
			];
			for (events, expected) in &tests {
				let prices_cache: ThreadSafePriceCache = Arc::new(RwLock::new(HashMap::new()));
				let (feed_in, feed_out) =
					mpsc::channel::<FeedNotification<FeedIdentifier, Asset, TimeStampedPrice>>(8);
				let signals = Signals::new(&[]).expect("could not create signals stream").fuse();
				let backend = Backend::new::<
					FeedNotification<FeedIdentifier, Asset, TimeStampedPrice>,
					FeedNotificationAction<Asset, TimeStampedPrice>,
					_,
					_,
					_,
					_,
				>(prices_cache.clone(), ReceiverStream::new(feed_out), signals)
				.await;

				for &event in events {
					feed_in.send(event).await.expect("could not send feed notification");
				}

				/* Drop the channel so that the backend exit and we join it.
				   This will ensure the events have been processed.
				*/
				drop(feed_in);
				backend.shutdown_handle.await.expect("could not join on backend handle");

				let prices_cache_r = prices_cache.read().expect("could not acquire read lock");
				assert_eq!(*prices_cache_r, expected.iter().copied().collect::<PriceCache>());
			}
		}
	}
}
