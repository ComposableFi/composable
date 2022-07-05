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
use sp_arithmetic::{FixedPointNumber, FixedU128};
use std::collections::HashSet;
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct ComposableFeed;

impl ComposableFeed {
	pub async fn start(
		composable_node_url: String,
		assets: &HashSet<(Asset, Asset)>,
	) -> FeedResult<Feed<FeedIdentifier, Asset, TimeStampedPrice>> {
		let (sink, source) = mpsc::channel(CHANNEL_BUFFER_SIZE);
		// Notifiy feed started
		sink.send(FeedNotification::Started { feed: FeedIdentifier::Composable })
			.await
			.map_err(|_| FeedError::ChannelIsBroken)?;
		let api =
			ClientBuilder::new()
				.set_url(composable_node_url)
				.build()
				.await
				.map_err(|_| FeedError::NetworkFailure)?
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
			.map_err(|_| FeedError::ChannelIsBroken)?;
		}
		let sink = sink.clone();
		let assets = assets.clone();
		let api_clone = api.clone();

		let handle = tokio::spawn(async move {
			let mut twap_updated_events = api_clone
				.events()
				.subscribe()
				.await
				.map_err(|_| FeedError::NetworkFailure)?
				.filter_events::<(TwapUpdated,)>();
			// process all twap_updated event
			while let Some(twap_updated) = twap_updated_events.next().await {
				println!("TwapUpdated Event : {twap_updated:?}");
				if let Ok(twap_updated) = twap_updated {
					let event: TwapUpdated = twap_updated.event;
					let base = &event.twaps[0];
					let quote = &event.twaps[1];
					let base_asset = primitives::currency::CurrencyId(base.0 .0)
						.try_into()
						.map_err(|_| FeedError::NetworkFailure)?;

					let quote_asset = primitives::currency::CurrencyId(quote.0 .0)
						.try_into()
						.map_err(|_| FeedError::NetworkFailure)?;
					if assets.contains(&(base_asset, quote_asset)) {
						let _ = sink
							.send(FeedNotification::AssetPriceUpdated {
								feed: FeedIdentifier::Composable,
								asset: base_asset,
								price: TimeStamped {
									value: (
										Price(
											FixedU128::from_inner(base.1 .0)
												.saturating_mul_int(1_u64),
										),
										Exponent(0),
									),
									timestamp: TimeStamp::now(),
								},
							})
							.await;
					}
				}
			}
			Ok(())
		});
		Ok((handle, ReceiverStream::new(source)))
	}
}
