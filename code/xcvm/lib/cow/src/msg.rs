use bounded_collections::{BoundedVec, ConstU32};
use cosmwasm_std::{Coin, CosmosMsg, Uint64};
use xcvm_core::{Network, Program, Instruction, DefaultXCVMPacket, AssetId};


pub enum ExecuteMsg {
	SubmitIntention {
		intention : BoundedVec<Intention, ConstU32<4>>,
	}
}

pub struct Batch {

}

pub struct CoinAt {
	network : Network,
	coin: Coin,
}

pub struct ResultAt {
	network : Network,
	coin: (Amount, AssetId),
}

pub const ASSETS_LIMIT: u32 = 1;

pub enum Limit {
	// If true, then prefer to give as much as possible up to the limit of give and receive accordingly more. If false, then prefer to give as little as possible in order to receive as little as possible while receiving at least want.
	Maximal(bool)
}


pub struct Intention{
	/// maximum amount of assets to give from user xc accounts.
	want: BoundedVec<CoinAt, ConstU32<ASSETS_LIMIT>>,
	/// The minimum amount of assets which give should be exchanged for.
	give: BoundedeVec<CoinAt, ConstU32<ASSETS_LIMIT>>,
	
	limit : Limit,
}

pub struct Problem {

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



pub struct Solver {

}