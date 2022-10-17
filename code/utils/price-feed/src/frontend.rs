use crate::{
	asset::Asset,
	cache::{Cache, PriceCache, ThreadSafePriceCache},
	feed::{Exponent, Price, TimeStamp, TimeStampedPrice},
};
use chrono::Duration;
use futures::channel::oneshot;
use primitives::currency::CurrencyId;
use serde::Serialize;
use std::{
	collections::HashMap,
	convert::TryFrom,
	net::SocketAddr,
	str::FromStr,
	sync::{Arc, RwLock},
};
use tokio::task::JoinHandle;
use warp::{hyper::StatusCode, reply, Filter, Rejection, Reply};

#[derive(PartialEq, Eq, Serialize, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct NormalizedPrice(u64);

pub struct Frontend {
	pub shutdown_trigger: oneshot::Sender<()>,
	pub shutdown_handle: JoinHandle<()>,
}

impl Frontend {
	pub async fn new(
		listening_address: &str,
		prices_cache: Arc<RwLock<PriceCache>>,
		cache_duration: Duration,
		expected_exponent: Exponent,
	) -> Self {
		let get_asset_id_endpoint = warp::path!("asset_id" / Asset)
			.and(warp::get())
			.and_then(move |asset_id| async move { get_asset_id(asset_id) });

		let get_price_endpoint =
			warp::path!("price" / CurrencyId)
				.and(warp::get())
				.and_then(move |currency_index| {
					let prices_cache_clone = prices_cache.clone();
					async move {
						get_price(
							prices_cache_clone,
							currency_index,
							cache_duration,
							expected_exponent,
						)
					}
				});

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

fn get_asset_id(x: Asset) -> Result<impl Reply, Rejection> {
	match CurrencyId::try_from(x) {
		Ok(currency_index) => Ok(reply::with_status(reply::json(&currency_index), StatusCode::OK)),
		Err(_) => Err(warp::reject::not_found()),
	}
}

fn get_price(
	prices: ThreadSafePriceCache,
	currency_index: CurrencyId,
	cache_duration: Duration,
	expected_exponent: Exponent,
) -> Result<impl Reply, Rejection> {
	match Asset::try_from(currency_index).and_then(|asset| {
		let now = TimeStamp::now();

		prices
			.get(&asset)
			.and_then(|timestamped_price| {
				ensure_uptodate_price(&cache_duration, &now, &timestamped_price)
			})
			.map(|(x, elapsed)| (normalize_price(expected_exponent, x), elapsed))
			.ok_or(())
	}) {
		// The oracle is expecting an object with the asset as key and it's price as value.
		Ok((normalized_price, elapsed)) => Ok(reply::with_header(
			reply::with_header(
				reply::with_status(
					reply::json(&HashMap::from([(
						format!("{}", currency_index),
						normalized_price,
					)])),
					StatusCode::OK,
				),
				"x-composable-cache-elapsed",
				format!("{}", elapsed),
			),
			"x-composable-cache-duration",
			format!("{}", cache_duration),
		)),
		Err(_) => Err(warp::reject::not_found()),
	}
}

/// Ensure that the price is not outdated.
fn ensure_uptodate_price(
	&max_cache_duration: &Duration,
	current_timestamp: &TimeStamp,
	timestamped_price: &TimeStampedPrice,
) -> Option<((Price, Exponent), Duration)> {
	let elapsed = current_timestamp.elapsed_since(&timestamped_price.timestamp);
	if elapsed < max_cache_duration {
		Some((timestamped_price.value, elapsed))
	} else {
		None
	}
}

/// Normalize the price to the expected exponent.
fn normalize_price(
	Exponent(expected_exponent): Exponent,
	(Price(p), Exponent(q)): (Price, Exponent),
) -> NormalizedPrice {
	// NOTE(hussein-aitlahcen): we want to go from x*10^q to x*10^expected_exponent
	let dt = expected_exponent - q;
	let power = u64::pow(10_u64, dt.abs() as u32);
	let normalized_price = match dt.signum() {
		0 => p,
		1 => p * power,
		-1 => p / power,
		_ => unreachable!(),
	};
	NormalizedPrice(normalized_price)
}

#[cfg(test)]
mod tests {
	use super::{normalize_price, NormalizedPrice};
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
				Some((value, Duration::seconds(19))),
			),
			(
				(
					Duration::seconds(10),
					TimeStamp(14),
					TimeStamped { value, timestamp: TimeStamp(5) },
				),
				Some((value, Duration::seconds(9))),
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
	fn test_get_normalized_price() {
		let expected_exponent = Exponent(2);
		[
			((Price(0xCAFEBABE), Exponent(-2)), NormalizedPrice(0xCAFEBABE * u64::pow(10, 4))),
			((Price(0xDEADBEEF), Exponent(2)), NormalizedPrice(0xDEADBEEF)),
			((Price(1), Exponent(0)), NormalizedPrice(u64::pow(10, 2))),
			((Price(12), Exponent(-1)), NormalizedPrice(12 * u64::pow(10, 3))),
			((Price(454000), Exponent(4)), NormalizedPrice(4540)),
		]
		.iter()
		.for_each(|&(price, expected_price)| {
			assert_eq!(normalize_price(expected_exponent, price), expected_price);
		});
	}
}
