use super::{Feed, FeedError, FeedResult};
use crate::{
	asset::Asset,
	feed::{
		composable_api::{self, api::pablo::events::TwapUpdated},
		Exponent, FeedIdentifier, FeedNotification, Price, TimeStamp, TimeStamped,
		TimeStampedPrice, CHANNEL_BUFFER_SIZE,
	},
};
use futures::StreamExt;
use std::collections::HashSet;
use subxt::{
	events::FilteredEventDetails, sp_core::H256, ClientBuilder, DefaultConfig,
	PolkadotExtrinsicParams,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct ComposableFeed;

impl ComposableFeed {
	pub async fn start(
		mut shutdown_message: tokio::sync::watch::Receiver<bool>,
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
			ClientBuilder::new()
				.set_url(composable_node_url)
				.build()
				.await
				.map_err(|e| {
					log::error!("{}", e);
					FeedError::NetworkFailure
				})?
				.to_runtime_api::<composable_api::api::RuntimeApi<
					DefaultConfig,
					PolkadotExtrinsicParams<DefaultConfig>,
				>>();

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
		let api_clone = api.clone();

		let handle = tokio::spawn(async move {
			let mut twap_updated_events = api_clone
				.events()
				.subscribe()
				.await
				.map_err(|e| {
					log::error!("{}", e);
					FeedError::NetworkFailure
				})?
				.filter_events::<(TwapUpdated,)>()
				.fuse();

			loop {
				tokio::select! {
					biased;

					_ = shutdown_message.changed() => {
						if *shutdown_message.borrow() {
							break;
						}
					}

					twap_updated_details = twap_updated_events.select_next_some() => {
						if let Ok(twap_updated_details) = twap_updated_details {
							handle_twap_updated_event(twap_updated_details, &assets, &sink).await?;
						}
					}
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
	twap_updated_details: FilteredEventDetails<H256, TwapUpdated>,
	assets: &HashSet<(Asset, Asset)>,
	sink: &mpsc::Sender<FeedNotification<FeedIdentifier, Asset, TimeStamped<(Price, Exponent)>>>,
) -> Result<(), FeedError> {
	let event: TwapUpdated = twap_updated_details.event;
	let (base_asset, base_price) = &event.twaps[0];
	let (quote_asset, _) = &event.twaps[1];
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
