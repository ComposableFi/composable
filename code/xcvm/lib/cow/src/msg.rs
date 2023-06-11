use bounded_collections::{BoundedVec, ConstU32};
use cosmwasm_std::{Addr, Coin, CosmosMsg, Uint64};
use xcvm_core::{
	AssetId, CallOrigin, DefaultXCVMPacket, Instruction, Network, Program, UserOrigin,
};

enum Decision {
	Approve,
	/// if there was solution, users lists it lock
	/// if no solution yet, he get all back to prevent ddos solvers
	Cancel,
	Reject,
}

pub enum ExecuteMsg {
	SubmitIntention { intention: BoundedVec<Intention, ConstU32<4>> },
	Decide { problem_id: String },
	/// user escrows his funds on interpeter on relevant chains and collects data on this chain
	ProveFunds {}
}

pub struct Batch {}

pub struct CoinAt {
	network: Network,
	coin: Coin,
}

pub struct ResultAt {
	network: Network,
	coin: (Amount, AssetId),
}

pub const ASSETS_LIMIT: u32 = 1;

pub enum Limit {
	// If true, then prefer to give as much as possible up to the limit of give and receive
	// accordingly more. If false, then prefer to give as little as possible in order to receive
	// as little as possible while receiving at least want.
	Maximal(bool),
}

pub struct Intention {
	/// maximum amount of assets to give from user xc accounts.
	want: BoundedVec<CoinAt, ConstU32<ASSETS_LIMIT>>,
	/// The minimum amount of assets which give should be exchanged for.
	give: BoundedeVec<CoinAt, ConstU32<ASSETS_LIMIT>>,

	limit: Limit,

	tip: BoundedeVec<CoinAt, ConstU32<8>>,
}

pub struct Problem {
	origin: UserOrigin,
	intention: Intention,
}

enum Solution {
	Execute { msgs: BoundedeVec<CosmosMsg<T>, ConstU32<8>> },
	XcExecute { programs: BoundedeVec<DefaultXCVMPacket, ConstU32<8>> },
}

/// Solution requires funds on chains to be settled before solution can be executed
/// for this well defined transfers program is send.
/// And than send well defined program to collect assets back from to desired location
struct Setup {
	funds: BoundedVec<CoinAt, ConstU32<ASSETS_LIMIT>>,
	collect: BoundedVec<ResultAt, ConstU32<ASSETS_LIMIT>>,
}

/// whitelisted set of contracts which can be used
struct Whitelist {
	network: Network,
	contract: Addr,
}

pub struct Solver {}
