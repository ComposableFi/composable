//! shared types across lending/liquidation/auctions pallets

use codec::FullCodec;
use frame_support::{pallet_prelude::MaybeSerializeDeserialize, traits::fungibles::{Mutate, Transfer}};

/// seconds
pub type DurationSeconds = u64;


pub trait DeFiComposablePallet {
	/// who.
	type AccountId;

	// what.
	type AssetId: FullCodec
		+ Eq
		+ PartialEq
		+ Copy
		+ MaybeSerializeDeserialize
		+ From<u128>
		+ Default;

	/// how much.
	type Balance;

	/// bank. vault owned - can transfer, cannot mint
	type Currency: Transfer<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>
		+ Mutate<Self::AccountId, Balance = Self::Balance, AssetId = <Self as Config>::AssetId>;
}
