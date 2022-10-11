use super::{
	Feed, FeedError, FeedIdentifier, FeedNotification, FeedResult, Price, TimeStamped,
	TimeStampedPrice,
};
use crate::{
	asset::{Asset, AssetPair, ConcatSymbol},
	feed::{Exponent, TimeStamp, CHANNEL_BUFFER_SIZE},
};
use binance::websockets::{WebSockets, WebsocketEvent};
use std::{
	collections::{HashMap, HashSet},
	sync::{atomic::AtomicBool, Arc},
};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_stream::wrappers::ReceiverStream;

pub const TOPIC_AGGREGATE_TRADE: &str = "aggTrade";

pub struct BinanceFeed;

impl BinanceFeed {
	pub async fn start(
		keep_running: Arc<AtomicBool>,
		assets: &HashSet<Asset>,
		quote_asset: Asset,
	) -> FeedResult<Feed<FeedIdentifier, Asset, TimeStampedPrice>> {
		let (sink, source) = mpsc::channel(CHANNEL_BUFFER_SIZE);

		// Notify feed started
		sink.send(FeedNotification::Started { feed: FeedIdentifier::Binance })
			.await
			.map_err(|e| {
				log::error!("{}", e);
				FeedError::ChannelIsBroken
			})?;

		let symbol_asset = assets
			.iter()
			.map(|&asset| {
				let asset_pair = AssetPair::new(asset, quote_asset).unwrap_or_else(|| {
					panic!("asset {:?} should be quotable in {:?}", asset, quote_asset)
				});
				format!("{}", ConcatSymbol::new(asset_pair))
			})
			.zip(assets.iter().copied())
			.collect::<HashMap<_, _>>();

		// Only listen to aggregate trades events
		let subscriptions = symbol_asset
			.keys()
			/* NOTE(hussein-aitlahcen):
				 It look like binance is expecting the symbol to be in lowercase
			*/
			.map(|symbol| format!("{}@{}", symbol.to_ascii_lowercase(), TOPIC_AGGREGATE_TRADE))
			.collect::<Vec<_>>();

		let sink_clone = sink.clone();
		let keep_running_clone = keep_running.clone();
		let assets = assets.clone();

		let handle = tokio::spawn(async move {
			for &asset in assets.iter() {
				sink.send(FeedNotification::AssetOpened { feed: FeedIdentifier::Binance, asset })
					.await
					.map_err(|e| {
						log::error!("{}", e);
						FeedError::ChannelIsBroken
					})?;
			}

			/* NOTE(hussein-aitlahcen):
			   Unfortunately, the binance-rs crate doesn't support async...
			*/
			let event_loop_handle: JoinHandle<Result<(), FeedError>> =
				tokio::task::spawn_blocking(move || {
					let sink = sink_clone.clone();
					let mut ws = WebSockets::new(|event: WebsocketEvent| {
						log::trace!("event: {:?}", event);
						if let WebsocketEvent::AggrTrades(trades) = event {
							let timestamp = TimeStamp::now();
							let price = str::parse::<f64>(trades.price.as_str())
								.expect("couldn't parse price");
							// Binance send a f64 price in USD. We normalize it to USD cent.
							let usd_cent_price = (price * 100.0) as u64;
							// Find back the asset from the symbol.
							if let Some(&asset) = symbol_asset.get(&trades.symbol) {
								// Trigger a price update in USD cent
								let _ = sink.blocking_send(FeedNotification::AssetPriceUpdated {
									feed: FeedIdentifier::Binance,
									asset,
									price: TimeStamped {
										value: (Price(usd_cent_price), Exponent(2)),
										timestamp,
									},
								});
							}
						}
						Ok(())
					});
					log::debug!("connecting to binance");
					ws.connect_multiple_streams(&subscriptions).map_err(|e| {
						log::error!("{}", e);
						FeedError::NetworkFailure
					})?;
					log::debug!("running event loop");
					ws.event_loop(&keep_running_clone).map_err(|e| {
						log::error!("{}", e);
						FeedError::NetworkFailure
					})?;
					log::debug!("closing subscription");
					Ok(())
				});

			// Make sure we trigger the AssetClosed/Stopped events
			// by not returning early.
			let e = event_loop_handle
				.await
				.map_err(|e| {
					log::error!("{}", e);
					FeedError::NetworkFailure
				})
				.and_then(|x| x);

			for &asset in assets.iter() {
				sink.send(FeedNotification::AssetClosed { feed: FeedIdentifier::Binance, asset })
					.await
					.map_err(|e| {
						log::error!("{}", e);
						FeedError::ChannelIsBroken
					})?;
			}

			sink.send(FeedNotification::Stopped { feed: FeedIdentifier::Binance })
				.await
				.map_err(|e| {
					log::error!("{}", e);
					FeedError::ChannelIsBroken
				})?;

			e
		});

		Ok((handle, ReceiverStream::new(source)))
	}
}
