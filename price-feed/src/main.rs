mod asset;
mod backend;
mod cache;
mod feed;
mod frontend;
mod opts;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
#[macro_use]
extern crate lazy_static;

use crate::{
	asset::Asset,
	backend::{Backend, FeedNotificationAction},
	cache::ThreadSafePriceCache,
	feed::{binance::BinanceFeed, FeedIdentifier, FeedNotification, FeedSource, TimeStampedPrice},
	frontend::Frontend,
};
use futures::stream::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};
use tokio::sync::mpsc;

pub type DefaultFeedNotification = FeedNotification<FeedIdentifier, Asset, TimeStampedPrice>;

const CHANNEL_BUFFER_SIZE: usize = 128;

#[tokio::main]
async fn main() {
	env_logger::init();

	let opts = opts::get_opts();

	let prices_cache: ThreadSafePriceCache = Arc::new(RwLock::new(HashMap::new()));

	let (sink, source) = mpsc::channel::<DefaultFeedNotification>(CHANNEL_BUFFER_SIZE);

	let mut binance = BinanceFeed::start(
		(),
		sink.clone(),
		&[Asset::BTC, Asset::ETH, Asset::LTC, Asset::ADA, Asset::DOT]
			.iter()
			.copied()
			.collect(),
	)
	.await
	.expect("unable to start binance feed");

	let backend_shutdown_trigger: futures::stream::Fuse<signal_hook_tokio::SignalsInfo> =
		Signals::new(&[SIGTERM, SIGINT, SIGQUIT])
			.expect("could not create signals stream")
			.fuse();

	let backend = Backend::new::<
		FeedNotification<FeedIdentifier, Asset, TimeStampedPrice>,
		FeedNotificationAction<Asset, TimeStampedPrice>,
		_,
		_,
		_,
	>(prices_cache.clone(), source, backend_shutdown_trigger)
	.await;

	let frontend = Frontend::new(&opts.listening_address, prices_cache).await;

	backend.shutdown_handle.await.expect("oop, something went wrong");

	log::info!("backend terminated, dropping subscriptions");
	binance.stop().await.expect("could not stop binance");

	log::info!("signaling warp for termination...");
	frontend.shutdown_trigger.send(()).expect("oop, something went wrong");

	log::info!("waiting for warp to terminate...");
	frontend.shutdown_handle.await.expect("oop, something went wrong");

	log::info!("farewell.");
}
