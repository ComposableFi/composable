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
		binance::BinanceFeed, composable::ComposableFeed, Exponent, FeedHandle, FeedIdentifier,
		FeedNotification, FeedStream, TimeStampedPrice,
	},
	frontend::Frontend,
};
use chrono::Duration;
use futures::{future::join_all, stream::StreamExt};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::{
	collections::HashMap,
	str::FromStr,
	sync::{atomic::AtomicBool, Arc, RwLock},
};

#[tokio::main]
async fn main() {
	env_logger::init();

	let opts = opts::get_opts();

	let prices_cache: ThreadSafePriceCache = Arc::new(RwLock::new(HashMap::new()));

	let keep_running = Arc::new(AtomicBool::new(true));

	let binance = BinanceFeed::start(
		keep_running.clone(),
		&[Asset::KSM].iter().copied().collect(),
		Asset::from_str(&opts.quote_asset).expect("invalid quote asset"),
	)
	.await
	.expect("unable to start binance feed");

	let composable = ComposableFeed::start(
		opts.composable_node,
		&[(Asset::PICA, Asset::USDC)].iter().copied().collect(),
	)
	.await
	.expect("unable to start composable feed");

	let merge = |feeds: Vec<(FeedHandle, FeedStream<FeedIdentifier, Asset, TimeStampedPrice>)>| {
		let (handles, sources) = feeds.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
		(join_all(handles), futures::stream::select_all(sources))
	};

	/* NOTE(hussein-aitlahcen):
		 Introducing a new feed is a matter of merge it with the existing ones.
		 A feed is a tuple of both a stream of notification along with joinable handle.

		 let new_feed = NewFeed:start(..);

		 ... merge(vec![..., new_feed])
	*/
	let (feeds_handle, feeds_source) = merge(vec![binance, composable]);

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
		_,
	>(prices_cache.clone(), feeds_source, backend_shutdown_trigger)
	.await;

	let frontend = Frontend::new(
		&opts.listening_address,
		prices_cache,
		Duration::seconds(opts.cache_duration as _),
		Exponent(opts.expected_exponent),
	)
	.await;

	backend.shutdown_handle.await.expect("oops, something went wrong");

	log::info!("backend terminated, notifying feeds of termination");
	keep_running.store(false, std::sync::atomic::Ordering::Relaxed);

	log::info!("waiting for feeds to terminate");
	let _ = feeds_handle.await;

	log::info!("signaling warp for termination...");
	frontend.shutdown_trigger.send(()).expect("oops, something went wrong");

	log::info!("waiting for warp to terminate...");
	frontend.shutdown_handle.await.expect("oops, something went wrong");

	log::info!("farewell.");
}
