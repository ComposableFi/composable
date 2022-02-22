//! utilities to work with relay chain and XCM transact calls into it

use support::RuntimeDebug;

use self::kusama::RelayChainCall;
use crate::prelude::*;

use {
    sp_runtime::traits::StaticLookup,
    frame_system::Config,
};

pub type RelayBalance = u64;

#[derive(Encode, Decode, RuntimeDebug)]
pub enum BalancesCall<T: Config> {
	#[codec(index = 3)]
	TransferKeepAlive(<T::Lookup as StaticLookup>::Source, #[codec(compact)] RelayBalance),
}

#[derive(Encode, Decode, RuntimeDebug)]
pub enum UtilityCall<RelayChainCall> {
	#[codec(index = 1)]
	AsDerivative(u16, RelayChainCall),
	#[codec(index = 2)]
	BatchAll(Vec<RelayChainCall>),
}

mod kusama {
    use crate::*;
    use prelude::*;

    use super::BalancesCall;
	/// The encoded index correspondes to Kusama's Runtime module configuration.
	/// https://github.com/paritytech/polkadot/blob/main/runtime/kusama/src/lib.rs#L1379
	#[derive(Encode, Decode, RuntimeDebug)]
	pub enum RelayChainCall<T: Config> {
		#[codec(index = 4)]
		Balances(BalancesCall<T>),
		#[codec(index = 24)]
		Utility(Box<UtilityCall<Self>>),
	}
}


pub fn balances_transfer_keep_alive<T:Config>(to: T::AccountId, amount : RelayBalance) -> RelayChainCall<T> {
	RelayChainCall::Balances(BalancesCall::TransferKeepAlive(T::Lookup::unlookup(to), amount))
} 

pub fn utility_as_derivative_call<T:Config>(call: RelayChainCall<T>, index: u16) -> RelayChainCall<T> {
	RelayChainCall::Utility(Box::new(UtilityCall::AsDerivative(index, call)))
}