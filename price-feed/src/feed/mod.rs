pub mod binance;
pub mod pyth;

use std::collections::HashSet;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

pub const USD_CENT_EXPONENT: Exponent = Exponent(-2);

#[derive(Serialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct TimeStamp(pub i64);

impl TimeStamp {
	pub fn now() -> Self {
		TimeStamp(Utc::now().timestamp())
	}
	pub fn elapsed_since(&self, previous: &TimeStamp) -> Duration {
		Duration::seconds(self.0 - previous.0)
	}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Price(pub(crate) u64);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Exponent(pub(crate) i32);

#[derive(Serialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct TimeStamped<T> {
	pub value: T,
	pub timestamp: TimeStamp,
}

pub type TimeStampedPrice = TimeStamped<(Price, Exponent)>;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum FeedNotification<F, A, P> {
	Started { feed: F },
	AssetOpened { feed: F, asset: A },
	AssetClosed { feed: F, asset: A },
	AssetPriceUpdated { feed: F, asset: A, price: P },
	Stopped { feed: F },
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum FeedIdentifier {
	Pyth,
	Binance,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum FeedError {
	NetworkFailure,
	ChannelIsBroken,
}

pub type FeedResult<T> = Result<T, FeedError>;

#[async_trait]
pub trait FeedSource<F, A, P>
where
	Self: Sized,
{
	type Parameters;
	async fn start(
		parameters: Self::Parameters,
		sink: mpsc::Sender<FeedNotification<F, A, P>>,
		assets: &HashSet<A>,
	) -> FeedResult<Self>;
	async fn stop(&mut self) -> FeedResult<()>;
}
