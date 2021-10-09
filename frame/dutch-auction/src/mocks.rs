//! may consider implementing similar to for mock

use composable_traits::{auction::DutchAuction, dex::Orderbook};
use sp_runtime::DispatchError;
//https://github.com/PacktPublishing/Blockchain-Development-for-Finance-Projects/blob/master/Chapter%208/contracts/Orderbook.sol

pub fn start_default_auction<
	B: Orderbook<AssetId = u32, Balance = u32, AccountId = u32, OrderId = u32>,
	T: DutchAuction<
		OrderId = u32,
		Orderbook = B,
		AccountId = u32,
		AssetId = u32,
		Balance = u32,
		Order = u32,
	>,
>(
	auction: T,
) -> Result<T::OrderId, DispatchError> {
	const TOKEN_A: u32 = 1;
	const TOKEN_B: u32 = 2;
	const ALICE: u32 = 42;
	T::start(&ALICE, TOKEN_A, &10, TOKEN_B, &13, 1_000_000, 1_000, <_>::default())
}

// will cover with tests as soon as cross chain auction design known
