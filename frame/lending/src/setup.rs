//! most usable things, 20% of all , for tests
/// test should be freely use external API
pub use codec::{Codec, Decode, Encode, FullCodec, MaxEncodedLen};

use composable_traits::defi::{CurrencyPair, DeFiComposableConfig};
use frame_benchmarking::whitelisted_caller;
use frame_support::traits::Hooks;
use frame_system::RawOrigin;

use crate::{self as pallet_lending, currency::*};

pub fn whitelisted_origin<T: frame_system::Config>() -> RawOrigin<T::AccountId> {
	let caller: T::AccountId = whitelisted_caller();
	RawOrigin::Signed(caller)
}

pub type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;

pub fn assets<T>() -> CurrencyPair<AssetIdOf<T>>
where
	T: frame_system::Config + composable_traits::defi::DeFiComposableConfig,
{
	let a = USDT::ID.encode();
	let b = BTC::ID.encode();
	CurrencyPair::new(
		AssetIdOf::<T>::decode(&mut &a[..]).unwrap(),
		AssetIdOf::<T>::decode(&mut &b[..]).unwrap(),
	)
}

pub fn produce_block<
	T: frame_system::Config + pallet_lending::Config + pallet_timestamp::Config,
>(
	n: T::BlockNumber,
	time: T::Moment,
) {
	frame_system::Pallet::<T>::set_block_number(n);
	<pallet_lending::Pallet<T> as Hooks<T::BlockNumber>>::on_initialize(n);
	<pallet_timestamp::Pallet<T>>::set_timestamp(time);
}
