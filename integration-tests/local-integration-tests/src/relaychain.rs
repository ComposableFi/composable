//! utilities to work with relay chain and XCM transact calls into it

use common::AccountId;

pub type RelayBalance = u64;

mod kusama {
	use crate::*;

	/// The encoded index correspondes to Kusama's Runtime module configuration.
	/// https://github.com/paritytech/polkadot/blob/main/runtime/kusama/src/lib.rs#L1379
	#[derive(Encode, Decode, RuntimeDebug)]
	pub enum RelayChainCall<T: Config> {
		#[codec(index = 4)]
		Balances(BalancesCall<T>),
	}
}


pub fn balances_transfer_keep_alive(to: AccountId, amount : RelayBalance) -> 