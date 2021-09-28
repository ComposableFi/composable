use super::{
	FeedError, FeedIdentifier, FeedNotification, FeedResult, FeedSource, Price, TimeStamped,
	TimeStampedPrice,
};
use crate::{
	asset::{Asset, AssetPair, ConcatSymbol, Symbol},
	feed::{Exponent, TimeStamp, USD_CENT_EXPONENT},
};
use async_trait::async_trait;
use binance::websockets::{WebSockets, WebsocketEvent};
use std::{
	collections::{HashMap, HashSet},
	sync::{atomic::AtomicBool, Arc},
};
use tokio::task::JoinHandle;

pub const TOPIC_AGGREGATE_TRADE: &str = "aggTrade";

pub struct BinanceFeed {
	keep_running: Arc<AtomicBool>,
	handle: JoinHandle<Result<(), FeedError>>,
}

impl BinanceFeed {
	fn terminate(&self) {
		self.keep_running.store(false, std::sync::atomic::Ordering::Relaxed);
		self.handle.abort();
	}
}

#[async_trait]
impl FeedSource<FeedIdentifier, Asset, TimeStampedPrice> for BinanceFeed {
	type Parameters = ();

	async fn start(
		(): Self::Parameters,
		sink: tokio::sync::mpsc::Sender<FeedNotification<FeedIdentifier, Asset, TimeStampedPrice>>,
		assets: &HashSet<Asset>,
	) -> FeedResult<Self> {
		// Notifiy feed started
		sink.send(FeedNotification::Started { feed: FeedIdentifier::Binance })
			.await
			.map_err(|_| FeedError::ChannelIsBroken)?;

		let symbol_asset = assets
			.iter()
			.map(|&asset| {
				let asset_pair = AssetPair::new(asset, Asset::USDT)
					.unwrap_or_else(|| panic!("asset {:?} should be quotable in USDT", asset));
				ConcatSymbol::new(asset_pair).symbol()
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

		let keep_running = Arc::new(AtomicBool::new(true));
		let sink = sink.clone();
		let sink1 = sink.clone();
		let keep_running_clone = keep_running.clone();

		let assets = assets.clone();
		let handle = tokio::spawn(async move {
			for &asset in assets.iter() {
				sink.send(FeedNotification::AssetOpened { feed: FeedIdentifier::Binance, asset })
					.await
					.map_err(|_| FeedError::ChannelIsBroken)?;
			}

			/* NOTE(hussein-aitlahcen):
			   Unfortunately, the binance-rs crate doesn't support async...
			*/
			let event_loop_handle: JoinHandle<Result<(), FeedError>> =
				tokio::task::spawn_blocking(move || {
					let mut ws = WebSockets::new(|event: WebsocketEvent| {
						log::trace!("event: {:?}", event);
						if let WebsocketEvent::AggrTrades(trades) = event {
							let timestamp = TimeStamp::now();
							let price = str::parse::<f64>(trades.price.as_str())
								.expect("couldn't parse price");
							let Exponent(usd_cent_exponent) = USD_CENT_EXPONENT;
							let usd_cent_price =
								(price * f64::powf(10.0, i32::abs(usd_cent_exponent) as _)) as u64;
							// Find back the asset from the symbol.
							if let Some(&asset) = symbol_asset.get(&trades.symbol) {
								// Trigger a price update in USD cent
								let _ = sink1.blocking_send(FeedNotification::AssetPriceUpdated {
									feed: FeedIdentifier::Binance,
									asset,
									price: TimeStamped {
										value: (Price(usd_cent_price), USD_CENT_EXPONENT),
										timestamp,
									},
								});
							}
						}
						Ok(())
					});
					ws.connect_multiple_streams(&subscriptions)
						.map_err(|_| FeedError::NetworkFailure)?;
					log::trace!("running event loop");
					ws.event_loop(&keep_running_clone).map_err(|_| FeedError::NetworkFailure)?;
					log::trace!("closing subscription");
					Ok(())
				});

			// Make sure we trigger the AssetClosed/Stopped events
			// by not returning early.
			let e = event_loop_handle.await.map_err(|_| FeedError::NetworkFailure).and_then(|x| x);

			for &asset in assets.iter() {
				sink.send(FeedNotification::AssetClosed { feed: FeedIdentifier::Binance, asset })
					.await
					.map_err(|_| FeedError::ChannelIsBroken)?;
			}

			sink.send(FeedNotification::Stopped { feed: FeedIdentifier::Binance })
				.await
				.map_err(|_| FeedError::ChannelIsBroken)?;

			e
		});

		Ok(BinanceFeed { keep_running, handle })
	}

	async fn stop(&mut self) -> FeedResult<()> {
		self.terminate();
		Ok(())
	}
}
