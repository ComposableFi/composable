use super::{Feed, FeedError, FeedResult};
use crate::{
	asset::Asset,
	feed::{
		composable_api, Exponent, FeedIdentifier, FeedNotification, Price, TimeStamp, TimeStamped,
		TimeStampedPrice, CHANNEL_BUFFER_SIZE,
	},
};
use std::collections::HashSet;
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct ComposableFeed;

impl ComposableFeed {
	pub async fn start(
		assets: &HashSet<(Asset, Asset)>,
	) -> FeedResult<Feed<FeedIdentifier, Asset, TimeStampedPrice>> {
		let (sink, source) = mpsc::channel(CHANNEL_BUFFER_SIZE);
		// Notifiy feed started
		sink.send(FeedNotification::Started { feed: FeedIdentifier::Composable })
			.await
			.map_err(|_| FeedError::ChannelIsBroken)?;
		let api =
			ClientBuilder::new()
				.build()
				.await?
				.to_runtime_api::<composable_api::api::RuntimeApi<
					DefaultConfig,
					PolkadotExtrinsicParams<DefaultConfig>,
				>>();
		let mut swapped_events = api
			.events()
			.subscribe()
			.await?
			.filter_events::<composable_api::api::pablo::events::Swapped>();

		for &(base, quote) in assets.iter() {
			sink.send(FeedNotification::AssetOpened {
				feed: FeedIdentifier::Composable,
				asset: base,
			})
			.await
			.map_err(|_| FeedError::ChannelIsBroken)?;
		}
		let sink = sink.clone();
		let sink1 = sink.clone();
		let assets = assets.clone();

		let handle = tokio::spawn(async move {
			// process all swapped event
			while let Some(swapped_event) = swapped_events.next().await {
				println!("Swapped Event : {swapped_event:?}");
				if let Ok(swapped) = swapped_event {
					let event: composable_api::api::pablo::events::Swapped = swapped.event;
					let base_asset =
						event.base_asset.try_into().map_err(|_| FeedError::NetworkFailure)?;
					let quote_asset =
						event.quote_asset.try_into().map_err(|_| FeedError::NetworkFailure)?;
					if assets.contains(&(base_asset, quote_asset)) {
						let _ = sink1.blocking_send(FeedNotification::AssetPriceUpdated {
							feed: FeedIdentifier::Composable,
							asset: base_asset,
							price: TimeStamped {
								value: (Price(event.base_amount as u64), Exponent(0)),
								timestamp: TimeStamp::now(),
							},
						});
					}
				}
			}
			Ok(())
		});
		Ok((handle, ReceiverStream::new(source)))
	}
}
