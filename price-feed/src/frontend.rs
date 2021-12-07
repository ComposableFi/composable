use crate::{
	asset::{Asset, AssetIndex},
	cache::{PriceCache, ThreadSafePriceCache},
	feed::{Exponent, Price, TimeStamp, TimeStampedPrice},
};
use chrono::Duration;
use futures::channel::oneshot;
use serde::Serialize;
use std::{
	convert::TryFrom,
	net::SocketAddr,
	str::FromStr,
	sync::{Arc, RwLock},
};
use tokio::task::JoinHandle;
use warp::{
	hyper::StatusCode,
	reply::{self, Json, WithStatus},
	Filter,
};

#[derive(PartialEq, Eq, Serialize, Copy, Clone, Debug)]
#[serde(rename = "USD")]
#[repr(transparent)]
pub struct USDCentPrice(u64);

pub struct Frontend {
	pub shutdown_trigger: oneshot::Sender<()>,
	pub shutdown_handle: JoinHandle<()>,
}

impl Frontend {
	pub async fn new(listening_address: &str, prices_cache: Arc<RwLock<PriceCache>>) -> Self {
		let get_asset_id_endpoint =
			warp::path!("asset_id" / Asset).and(warp::get()).map(get_asset_id);

		let get_price_endpoint = warp::path!("price" / AssetIndex / u128).and(warp::get()).map(
			move |asset_index, _request_id| get_price(&prices_cache, asset_index, _request_id),
		);

		let (shutdown_trigger, shutdown) = oneshot::channel::<()>();
		let (_, server) = warp::serve(get_price_endpoint.or(get_asset_id_endpoint))
			.bind_with_graceful_shutdown(
				SocketAddr::from_str(listening_address).expect("invalid listening address."),
				async {
					shutdown.await.ok();
				},
			);

		let shutdown_handle = tokio::spawn(server);

		Frontend { shutdown_trigger, shutdown_handle }
	}
}

fn get_asset_id(x: Asset) -> WithStatus<Json> {
	match AssetIndex::try_from(x) {
		Ok(asset_index) => reply::with_status(reply::json(&asset_index), StatusCode::OK),
		Err(_) => reply::with_status(reply::json(&()), StatusCode::NOT_FOUND),
	}
}

/*
  The oracle pallet is expecting a price in USD cents.
  While this server handle any asset pair.
  It make this part of code very specific...
  Shouldn't we use the unit of value + exponent for any asset pair?

  Also, the price might be outdated, we added the timestamp value to it.
  Should the offchain worker handle this or should we put some kind of timeout
  here and wipe the cached value?
*/
fn get_price(
	prices: &ThreadSafePriceCache,
	asset_index: AssetIndex,
	_request_id: u128,
) -> WithStatus<Json> {
	// TODO: What is the request_id useful for (comming from oracle pallet)?
	match Asset::try_from(asset_index).and_then(|asset| {
		let now = TimeStamp::now();
		let max_cache_duration = Duration::seconds(10);
		prices
			.read()
			.expect("could not acquire read lock")
			.get(&asset)
			.copied()
			.and_then(|timestamped_price| {
				ensure_uptodate_price(&max_cache_duration, &now, &timestamped_price)
			})
			.map(get_usd_cent_price)
			.ok_or(())
	}) {
		Ok(usd_cent_price) => reply::with_status(reply::json(&usd_cent_price), StatusCode::OK),
		Err(_) => reply::with_status(reply::json(&()), StatusCode::NOT_FOUND),
	}
}

/*
  Ensure that the value was registered less than X seconds ago
*/
fn ensure_uptodate_price(
	&max_cache_duration: &Duration,
	current_timestamp: &TimeStamp,
	timestamped_price: &TimeStampedPrice,
) -> Option<(Price, Exponent)> {
	let elapsed = current_timestamp.elapsed_since(&timestamped_price.timestamp);
	if elapsed < max_cache_duration {
		Some(timestamped_price.value)
	} else {
		None
	}
}

fn get_usd_cent_price((Price(p), Exponent(q)): (Price, Exponent)) -> USDCentPrice {
	let usd_adjust_cent_exponent = q + 2;
	let usd_cent_price = match usd_adjust_cent_exponent.signum() {
		0 => p,
		1 => p * u64::pow(10u64, usd_adjust_cent_exponent as u32),
		-1 => p / u64::pow(10u64, usd_adjust_cent_exponent.abs() as u32),
		_ => unreachable!(),
	};
	USDCentPrice(usd_cent_price)
}

#[cfg(test)]
mod tests {
	use super::{get_usd_cent_price, USDCentPrice};
	use crate::{
		feed::{Exponent, Price, TimeStamp, TimeStamped},
		frontend::ensure_uptodate_price,
	};
	use chrono::Duration;

	#[test]
	fn test_ensure_uptodate_price() {
		let value = (Price(0x1337), Exponent(10));
		[
			(
				(
					Duration::seconds(1),
					TimeStamp(1),
					TimeStamped { value, timestamp: TimeStamp(0) },
				),
				None,
			),
			(
				(
					Duration::seconds(5),
					TimeStamp(6),
					TimeStamped { value, timestamp: TimeStamp(0) },
				),
				None,
			),
			(
				(
					Duration::seconds(20),
					TimeStamp(20),
					TimeStamped { value, timestamp: TimeStamp(1) },
				),
				Some(value),
			),
			(
				(
					Duration::seconds(10),
					TimeStamp(14),
					TimeStamped { value, timestamp: TimeStamp(5) },
				),
				Some(value),
			),
		]
		.iter()
		.for_each(|((max_cache_duration, current_timestamp, timestamped_price), expected)| {
			assert_eq!(
				ensure_uptodate_price(max_cache_duration, current_timestamp, timestamped_price),
				*expected
			);
		})
	}

	#[test]
	fn test_get_usd_cent_price() {
		[
			((Price(0xCAFEBABE), Exponent(-2)), USDCentPrice(0xCAFEBABE)),
			((Price(0xDEADBEEF), Exponent(2)), USDCentPrice(0xDEADBEEF * u64::pow(10, 2 + 2))),
			((Price(1), Exponent(0)), USDCentPrice(u64::pow(10, 2))),
			((Price(12), Exponent(-1)), USDCentPrice(12 * u64::pow(10, 1))),
			((Price(454000), Exponent(-6)), USDCentPrice(45)),
		]
		.iter()
		.for_each(|&(price, expected_usd_cent)| {
			assert_eq!(get_usd_cent_price(price), expected_usd_cent);
		});
	}
}
