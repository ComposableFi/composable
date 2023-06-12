use alloc::string::String;
use bounded_collections::{BoundedVec, ConstU32};
use cosmwasm_std::{Addr, Coin, CosmosMsg, Uint64};
use xcvm_core::{
	AssetId, CallOrigin, DefaultXCVMPacket, Instruction, Network, Program, UserOrigin,
};

enum Decision {
	Approve{solution_id : Option<String> },
	/// if there was solution, users lists it lock
	/// if no solution yet, he get all back to prevent ddos solvers
	Cancel,
	Reject,
}


pub enum ExecuteMsg {
	SubmitIntention { intention: BoundedVec<Intention, ConstU32<4>> },	
	/// user can cancel his intention
	Decide { intention_id: String, decision: Decision },
	/// user escrows his funds on interpreter on relevant chains and collects data on this chain
	ProveFunds {},
	// submit solutions of problems
	SubmitSolutions { solutions: BoundedVec<(Solution, BoundedVec<Problem, ConstU32<32>>)> },

	/// one may preload liquidity on various chains so that if solution uses that liqudity for limited operations set you likely get more 
	PreloadLiquidity { },
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
	Slippage,
}

pub struct Intention {
	/// maximum amount of assets to give from user xc accounts.
	want: BoundedVec<CoinAt, ConstU32<ASSETS_LIMIT>>,
	/// The minimum amount of assets which give should be exchanged for.
	give: BoundedeVec<CoinAt, ConstU32<ASSETS_LIMIT>>,

	limit: Limit,

	tip: BoundedeVec<CoinAt, ConstU32<8>>,
	
	solution_executuin_block_offset: Option<Uint64>,
	canceltaion_block_offset: Option<Uint32>,
}

pub struct Problem {
	origin: UserOrigin,
	intention: Intention,
}


pub struct Solution {
	program : SolutionProgram,
	/// fees this solution will take for execution on best effort according limits
	fee :  BoundedVec<CoinAt, ConstU32<ASSETS_LIMIT>>,
	/// solution can solve full or only only part of intention
	solved: BoundedeVec<CoinAt, ConstU32<8>>,
}

pub enum SolutionProgram {
	// set of cross chain swaps routed over the chains over swap adapter on each chain for whitelisted contracts
	SwapRoute {},
	Execute { msgs: BoundedeVec<CosmosMsg<T>, ConstU32<8>> },
	XcExecute { programs: BoundedeVec<DefaultXCVMPacket, ConstU32<8>> },
	
	/// set of well know dexes executed via XCVM swap command
	PermissonedRouter {}
}

/// Solution requires funds on chains to be settled before solution can be executed
/// for this well defined transfers program is send.
/// And than send well defined program to collect assets back from to desired location
struct Setup {
	funds: BoundedVec<CoinAt, ConstU32<ASSETS_LIMIT>>,
	collect: BoundedVec<ResultAt, ConstU32<ASSETS_LIMIT>>,
}

/// whitelisted set of contracts and calls can be made to relevant contracts
struct Whitelist {
	network: Network,
	contract: Addr,

}

pub struct Solver {}
