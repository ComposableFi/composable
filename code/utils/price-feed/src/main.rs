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
	feed::{
		binance::BinanceFeed, composable::ComposableFeed, Exponent, FeedIdentifier,
		FeedNotification, TimeStampedPrice,
	},
	frontend::Frontend,
	opts::Opts,
};

use chrono::Duration;
use clap::Parser;
use futures::{
	future::join_all,
	stream::{Fuse, StreamExt},
};
use signal_hook::consts::signal::*;
use signal_hook_tokio::{Signals, SignalsInfo};
use tokio::sync::watch;

use std::{
	collections::HashMap,
	str::FromStr,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, RwLock,
	},
};

#[tokio::main]
async fn main() {
	env_logger::init();

	let opts = Opts::parse();

	let prices_cache: ThreadSafePriceCache = Arc::new(RwLock::new(HashMap::new()));

	// watch instead of oneshot to allow for multiple feeds to listen at once, instead of creating
	// multiple channels
	let (feed_shutdown_sender, feed_shutdown_receiver) = watch::channel(false);

	// used for binance feed
	// TODO(benluelo): Use the AtomicBool internally in the binance feed but use the watch channel
	// created above to send the shutdown message
	let keep_running = Arc::new(AtomicBool::new(true));

	let binance = BinanceFeed::start(
		keep_running.clone(),
		&[Asset::KSM].into_iter().collect(),
		Asset::from_str(&opts.quote_asset).expect("invalid quote asset"),
	)
	.await
	.map_err(|e| {
		log::error!("{:?}", e);
		std::process::exit(1);
	})
	.unwrap();

	let composable = ComposableFeed::start(
		feed_shutdown_receiver,
		opts.composable_node,
		&[(Asset::PICA, Asset::USDC)].into_iter().collect(),
	)
	.await
	.map_err(|e| {
		log::error!("{:?}", e);
		std::process::exit(1);
	})
	.unwrap();

	/* NOTE(hussein-aitlahcen):
		 Introducing a new feed is a matter of merge it with the existing ones.
		 A feed is a tuple of both a stream of notification along with joinable handle.

		 let new_feed = NewFeed::start(..);

		 ... merge(vec![..., new_feed])
	*/
	let (feeds_handle, feeds_source) = {
		let (handles, sources) = [binance, composable].into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
		(join_all(handles), futures::stream::select_all(sources))
	};

	let backend_shutdown_trigger: Fuse<SignalsInfo> = Signals::new(&[SIGTERM, SIGINT, SIGQUIT])
		.map_err(|e| {
			log::error!("{:?}", e);
			std::process::exit(1);
		})
		.unwrap()
		.fuse();

	let backend = Backend::new::<
		FeedNotification<FeedIdentifier, Asset, TimeStampedPrice>,
		FeedNotificationAction<Asset, TimeStampedPrice>,
		_,
		_,
		_,
		_,
	>(prices_cache.clone(), feeds_source, backend_shutdown_trigger)
	.await;

	let frontend = Frontend::new(
		&opts.listening_address,
		prices_cache,
		Duration::seconds(opts.cache_duration.into()),
		Exponent(opts.expected_exponent),
	)
	.await;

	backend.shutdown_handle.await.expect("oops, something went wrong");

	log::info!("backend terminated, notifying feeds of termination");
	keep_running.store(false, Ordering::SeqCst);

	feed_shutdown_sender.send(true).unwrap();

	log::info!("waiting for feeds to terminate");
	let _ = feeds_handle.await;

	log::info!("signaling warp for termination...");
	frontend.shutdown_trigger.send(()).expect("oops, something went wrong");

	log::info!("waiting for warp to terminate...");
	frontend.shutdown_handle.await.expect("oops, something went wrong");

	log::info!("farewell.");
}
