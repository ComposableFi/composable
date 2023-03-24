// NOTE(hussein-aitlahcen):
// avoid clippy issues because of suspended Pyth
#![allow(dead_code)]

pub mod binance;
pub mod composable;
#[allow(clippy::all)]
pub mod composable_api;
pub mod pyth;

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use tokio_stream::wrappers::ReceiverStream;

/// Default channels size used by feeds
pub const CHANNEL_BUFFER_SIZE: usize = 128;

#[derive(Serialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct TimeStamp(pub i64);

impl TimeStamp {
	/// Return the current timestamp represented by UNIX timestamp.
	pub fn now() -> Self {
		TimeStamp(Utc::now().timestamp())
	}
	pub fn elapsed_since(&self, previous: &TimeStamp) -> Duration {
		Duration::seconds(self.0 - previous.0)
	}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Price(pub(crate) u128);

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Exponent(pub(crate) i32);

/// A type that wrap a value and provide a timestamp for it.
#[derive(Serialize, PartialEq, Eq, Copy, Clone, Debug)]
pub struct TimeStamped<T> {
	pub value: T,
	pub timestamp: TimeStamp,
}

/// Convenient alias for timestamped price along with it's exponent.
pub type TimeStampedPrice = TimeStamped<(Price, Exponent)>;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum FeedNotification<F, A, P> {
	Started { feed: F },
	AssetOpened { feed: F, asset: A },
	AssetClosed { feed: F, asset: A },
	AssetPriceUpdated { feed: F, asset: A, price: P },
	Stopped { feed: F },
}

/// The feed identifiers.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum FeedIdentifier {
	Pyth,
	Binance,
	Composable,
}

/// The possible errors happening while feeds are running.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum FeedError {
	NetworkFailure,
	ChannelIsBroken,
	CannotDecodeEvent,
}

/// Wrapper type used to notify the possible FeedError
/// happening during a computation.
pub type FeedResult<T> = Result<T, FeedError>;

/// A feed stream that fire various notifications.
/// Generic over the identifier `F`, the asset `A` and the price `P`.
pub type FeedStream<F, A, P> = ReceiverStream<FeedNotification<F, A, P>>;

/// A joinable feed handle used to sync while shuting down.
pub type FeedHandle = JoinHandle<Result<(), FeedError>>;

/// A feed, represented as a product of a joinable shutdown handle and a notification stream.
pub type Feed<F, A, P> = (FeedHandle, FeedStream<F, A, P>);
