use crate::{
    asset::{Asset, AssetPair, AssetPairHash},
    cache::{PriceCache, ThreadSafePriceCache},
    feed::{Exponent, Price, TimeStamped},
};
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

pub struct Frontend {
    pub shutdown_trigger: oneshot::Sender<()>,
    pub shutdown_handle: JoinHandle<()>,
}

impl Frontend {
    pub async fn new(listening_address: &String, prices_cache: Arc<RwLock<PriceCache>>) -> Self {
        let get_asset_id_endpoint = warp::path!("asset_id" / Asset / Asset)
            .and(warp::get())
            .map(get_asset_id);

        let get_price_endpoint = warp::path!("price" / AssetPairHash / u128)
            .and(warp::get())
            .map(move |asset_pair_hash, _request_id| {
                get_price(&prices_cache, asset_pair_hash, _request_id)
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

        Frontend {
            shutdown_trigger,
            shutdown_handle,
        }
    }
}

fn get_asset_id(x: Asset, y: Asset) -> WithStatus<Json> {
    match AssetPairHash::try_from(AssetPair::new(x, y)) {
        Ok(valid_asset_pair_hash) => {
            reply::with_status(reply::json(&valid_asset_pair_hash), StatusCode::OK)
        }
        Err(_) => reply::with_status(reply::json(&()), StatusCode::BAD_REQUEST),
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
    asset_pair_hash: AssetPairHash,
    _request_id: u128,
) -> WithStatus<Json> {
    // TODO: What is the request_id useful for (comming from oracle pallet)?
    match AssetPair::try_from(asset_pair_hash).and_then(|x| {
        prices
            .read()
            .expect("could not acquire read lock")
            .get(&x)
            .copied()
            .ok_or(())
    }) {
        Ok(TimeStamped {
            value: (Price(p), Exponent(q)),
            timestamp: _,
        }) => {
            #[derive(Serialize, Copy, Clone, Debug)]
            #[serde(rename_all = "UPPERCASE")]
            #[repr(transparent)]
            pub struct USDPrice {
                pub usd: Price,
            }
            let usd_adjust_cent_exponent = q + 2;
            let usd_price = Price(p / u64::pow(10u64, i32::abs(usd_adjust_cent_exponent) as u32));

            reply::with_status(reply::json(&USDPrice { usd: usd_price }), StatusCode::OK)
        }
        Err(_) => reply::with_status(reply::json(&()), StatusCode::NOT_FOUND),
    }
}
