use super::{Feed, FeedError, FeedResult};
use crate::{
	asset::Asset,
	feed::{
		composable_api::api::pablo::events::TwapUpdated, Exponent, FeedIdentifier,
		FeedNotification, Price, TimeStamp, TimeStamped, TimeStampedPrice, CHANNEL_BUFFER_SIZE,
	},
};
use futures::StreamExt;
use std::collections::HashSet;
use subxt::{OnlineClient, PolkadotConfig};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct ComposableFeed;

impl ComposableFeed {
	pub async fn start(
		shutdown_message: tokio::sync::watch::Receiver<bool>,
		composable_node_url: String,
		assets: &HashSet<(Asset, Asset)>,
	) -> FeedResult<Feed<FeedIdentifier, Asset, TimeStampedPrice>> {
		let (sink, source) = mpsc::channel(CHANNEL_BUFFER_SIZE);
		sink.send(FeedNotification::Started { feed: FeedIdentifier::Composable })
			.await
			.map_err(|e| {
				log::error!("{}", e);
				FeedError::ChannelIsBroken
			})?;
		let api =
			OnlineClient::<PolkadotConfig>::from_url(composable_node_url)
				.await
				.map_err(|e| {
					log::error!("{}", e);
					FeedError::NetworkFailure
				})?;

		for &(base, _quote) in assets.iter() {
			sink.send(FeedNotification::AssetOpened {
				feed: FeedIdentifier::Composable,
				asset: base,
			})
			.await
			.map_err(|e| {
				log::error!("{}", e);
				FeedError::ChannelIsBroken
			})?;
		}

		let sink = sink.clone();
		let assets = assets.clone();

		let handle = tokio::spawn(async move {
			// Subscribe to finalized blocks.
			let mut block_sub = api.blocks().subscribe_finalized().await.map_err(|e| {
				log::error!("{}", e);
				FeedError::NetworkFailure
			})?;

			// Get each finalized block as it arrives.
			while let Some(block) = block_sub.next().await {
				let block = block.map_err(|e| {
					log::error!("{}", e);
					FeedError::NetworkFailure
				})?;

				// Ask for the events for this block.
				let events = block.events().await.map_err(|e| {
					log::error!("{}", e);
					FeedError::NetworkFailure
				})?;

				// Decode events
				for event in events.iter() {
					let event = event.map_err(|_| FeedError::CannotDecodeEvent)?;
					let maybe_twap_updated_event =
						event.as_event::<TwapUpdated>().map_err(|_| FeedError::CannotDecodeEvent)?;

					// If TwapUpdated event is found, handle it
					if let Some(twap_updated_event) = maybe_twap_updated_event {
						handle_twap_updated_event(twap_updated_event, &assets, &sink).await?;
					}
				}

				if *shutdown_message.borrow() {
					break;
				}
			}

			for &(base, _quote) in assets.iter() {
				sink.send(FeedNotification::AssetClosed {
					feed: FeedIdentifier::Composable,
					asset: base,
				})
				.await
				.map_err(|e| {
					log::error!("{}", e);
					FeedError::ChannelIsBroken
				})?;
			}

			sink.send(FeedNotification::Stopped { feed: FeedIdentifier::Composable })
				.await
				.map_err(|e| {
					log::error!("{}", e);
					FeedError::ChannelIsBroken
				})?;

			Ok(())
		});
		Ok((handle, ReceiverStream::new(source)))
	}
}

async fn handle_twap_updated_event(
	twap_updated_details: TwapUpdated,
	assets: &HashSet<(Asset, Asset)>,
	sink: &mpsc::Sender<FeedNotification<FeedIdentifier, Asset, TimeStamped<(Price, Exponent)>>>,
) -> Result<(), FeedError> {
	let (base_asset, base_price) = &twap_updated_details.twaps[0];
	let (quote_asset, _) = &twap_updated_details.twaps[1];
	let base_asset = primitives::currency::CurrencyId(base_asset.0).try_into().map_err(|e| {
		log::error!("{:?}", e);
		FeedError::NetworkFailure
	})?;
	let quote_asset = primitives::currency::CurrencyId(quote_asset.0).try_into().map_err(|e| {
		log::error!("{:?}", e);
		FeedError::NetworkFailure
	})?;
	Ok(if assets.contains(&(base_asset, quote_asset)) {
		sink.send(FeedNotification::AssetPriceUpdated {
			feed: FeedIdentifier::Composable,
			asset: base_asset,
			price: TimeStamped {
				value: (Price(base_price.0.try_into().unwrap()), Exponent(12)),
				timestamp: TimeStamp::now(),
			},
		})
		.await
		.map_err(|e| {
			log::error!("{}", e);
			FeedError::ChannelIsBroken
		})?;
	})
}
