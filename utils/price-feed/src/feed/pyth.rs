use std::{
	collections::HashSet,
	sync::{atomic::AtomicBool, Arc},
};

use super::{
	Feed, FeedError, FeedIdentifier, FeedNotification, FeedResult, Price, TimeStamped,
	TimeStampedPrice, CHANNEL_BUFFER_SIZE,
};
use crate::{
	asset::{Asset, AssetPair, SlashSymbol},
	feed::{Exponent, TimeStamp},
};
use futures::stream::StreamExt;
use jsonrpc_client_transports::{
	transports::ws::connect, RpcError, TypedClient, TypedSubscriptionStream,
};
use serde::{Deserialize, Serialize};
use tokio::{
	sync::mpsc::{self, error::SendError},
	task::JoinHandle,
};
use tokio_stream::wrappers::ReceiverStream;
use url::Url;

pub type PythFeedNotification = FeedNotification<FeedIdentifier, Asset, TimeStampedPrice>;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PythSymbolStatus {
	Trading,
	Halted,
	Unknown,
}

#[derive(Copy, Clone, Debug, Deserialize)]
struct PythNotifyPrice {
	status: PythSymbolStatus,
	price: Price,
}

#[derive(Clone, Debug, Deserialize)]
struct PythProductPrice {
	account: String,
	price_exponent: Exponent,
}

#[derive(Clone, Debug, Deserialize)]
struct PythProductAttributes {
	symbol: String,
}

#[derive(Clone, Debug, Deserialize)]
struct PythProduct {
	account: String,
	attr_dict: PythProductAttributes,
	price: Vec<PythProductPrice>,
}

#[derive(Debug)]
pub enum PythError {
	RpcError(RpcError),
	ChannelError(SendError<PythFeedNotification>),
}

#[derive(Serialize)]
struct PythSubscribeParams {
	account: String,
}

pub struct Pyth {
	client: TypedClient,
	handles: Vec<JoinHandle<Result<(), PythError>>>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum PythNotifyPriceAction {
	YieldFeedNotification(PythFeedNotification),
}

fn notify_price_action(
	asset: Asset,
	product_price: &PythProductPrice,
	notify_price: &PythNotifyPrice,
	timestamp: &TimeStamp,
) -> Option<PythNotifyPriceAction> {
	match notify_price.status {
		PythSymbolStatus::Trading => Some(PythNotifyPriceAction::YieldFeedNotification(
			FeedNotification::AssetPriceUpdated {
				feed: FeedIdentifier::Pyth,
				asset,
				price: TimeStamped {
					value: (notify_price.price, product_price.price_exponent),
					timestamp: *timestamp,
				},
			},
		)),
		PythSymbolStatus::Halted => None,
		PythSymbolStatus::Unknown => None,
	}
}

impl Pyth {
	async fn new(url: &url::Url) -> Result<Pyth, PythError> {
		let client = connect::<TypedClient>(url).await.map_err(PythError::RpcError)?;
		Ok(Pyth { client, handles: Vec::new() })
	}

	async fn get_product_list(&self) -> Result<Vec<PythProduct>, PythError> {
		self.client
			.call_method::<(), Vec<PythProduct>>("get_product_list", "", ())
			.await
			.map_err(PythError::RpcError)
	}

	async fn subscribe(
		&mut self,
		keep_running: Arc<AtomicBool>,
		sink: mpsc::Sender<PythFeedNotification>,
		asset_pair: AssetPair,
		product_price: PythProductPrice,
	) -> Result<(), PythError> {
		log::info!(
			"Subscribing to asset pair {:?} from account {:?}",
			asset_pair,
			product_price.account
		);
		let mut stream: TypedSubscriptionStream<PythNotifyPrice> = self
			.client
			.subscribe(
				"subscribe_price",
				[PythSubscribeParams { account: product_price.account.to_string() }],
				"notify_price",
				"",
				"",
			)
			.map_err(PythError::RpcError)?;
		let join_handle = tokio::spawn(async move {
			sink.send(FeedNotification::AssetOpened {
				feed: FeedIdentifier::Pyth,
				asset: asset_pair.0,
			})
			.await
			.map_err(PythError::ChannelError)?;
			'a: loop {
				if !keep_running.load(std::sync::atomic::Ordering::Relaxed) {
					break 'a
				}
				match stream.next().await {
					Some(Ok(notify_price)) => {
						log::debug!("received notify_price, {:?}, {:?}", asset_pair, notify_price);
						let timestamp = TimeStamp::now();
						#[allow(clippy::single_match)]
						match notify_price_action(
							asset_pair.0,
							&product_price,
							&notify_price,
							&timestamp,
						) {
							Some(PythNotifyPriceAction::YieldFeedNotification(
								feed_notification,
							)) => {
								sink.send(feed_notification)
									.await
									.map_err(PythError::ChannelError)?;
							},
							None => {
								// TODO: should we close the feed if the received price don't yield
								// a price update? e.g. the SymbolStatus != Trading
							},
						}
					},
					Some(Err(e)) => {
						log::error!("unexpected rpc error: {:?}", e);
						break 'a
					},
					None => break 'a,
				}
			}
			sink.send(FeedNotification::AssetClosed {
				feed: FeedIdentifier::Pyth,
				asset: asset_pair.0,
			})
			.await
			.map_err(PythError::ChannelError)?;
			Ok(())
		});
		self.handles.push(join_handle);
		Ok(())
	}

	async fn subscribe_to_asset(
		&mut self,
		keep_running: Arc<AtomicBool>,
		sink: &mpsc::Sender<PythFeedNotification>,
		asset_pair: &AssetPair,
	) -> Result<(), PythError> {
		let asset_pair_symbol = format!("{}", SlashSymbol::new(*asset_pair));
		let product_prices = self
			.get_product_list()
			.await?
			.iter()
			.filter(|p| p.attr_dict.symbol == asset_pair_symbol)
			.flat_map(|p| p.price.clone())
			.collect::<Vec<_>>();
		log::info!("Accounts for {:?}: {:?}", asset_pair_symbol, product_prices);
		for product_price in product_prices {
			self.subscribe(keep_running.clone(), sink.clone(), *asset_pair, product_price)
				.await?
		}
		Ok(())
	}

	async fn terminate(&self) {
		self.handles.iter().for_each(drop);
	}
}

pub struct PythFeed;

impl PythFeed {
	pub async fn start(
		url: Url,
		keep_running: Arc<AtomicBool>,
		assets: &HashSet<Asset>,
	) -> FeedResult<Feed<FeedIdentifier, Asset, TimeStampedPrice>> {
		let mut pyth = Pyth::new(&url).await.map_err(|_| FeedError::NetworkFailure)?;

		let (sink, source) = mpsc::channel(CHANNEL_BUFFER_SIZE);

		sink.send(FeedNotification::Started { feed: FeedIdentifier::Pyth })
			.await
			.map_err(|_| FeedError::ChannelIsBroken)?;

		for &asset in assets.iter() {
			if let Some(asset_pair) = AssetPair::new(asset, Asset::USD) {
				pyth.subscribe_to_asset(keep_running.clone(), &sink, &asset_pair)
					.await
					.expect("failed to subscribe to asset");
			}
		}

		let handle = tokio::spawn(async move {
			let _ = &pyth;
			futures::future::join_all(pyth.handles).await;
			Ok(())
		});

		Ok((handle, ReceiverStream::new(source)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{asset::*, feed::*};

	#[test]
	fn test_notify_price_action() {
		let account = "irrelevant".to_string();
		let product_price = PythProductPrice { account, price_exponent: Exponent(0x1337) };
		let price = Price(0xCAFEBABE);
		let timestamp = TimeStamp::now();
		VALID_ASSETS.iter().for_each(|&asset| {
			[
				(PythSymbolStatus::Halted, None),
				(PythSymbolStatus::Unknown, None),
				(
					PythSymbolStatus::Trading,
					Some(PythNotifyPriceAction::YieldFeedNotification(
						FeedNotification::AssetPriceUpdated {
							feed: FeedIdentifier::Pyth,
							asset,
							price: TimeStamped {
								value: (price, product_price.price_exponent),
								timestamp,
							},
						},
					)),
				),
			]
			.iter()
			.for_each(|&(status, expected_action)| {
				let notify_price = PythNotifyPrice { status, price };
				assert_eq!(
					expected_action,
					notify_price_action(asset, &product_price, &notify_price, &timestamp)
				)
			});
		});
	}
}
