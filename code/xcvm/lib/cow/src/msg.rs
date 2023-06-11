use bounded_collections::{BoundedVec, ConstU32};
use cosmwasm_std::{Coin, CosmosMsg};
use xcvm_core::{Network, Program, Instruction, DefaultXCVMPacket};


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

pub const ASSETS_LIMIT: u32 = 1;

pub enum Limit {
	// If true, then prefer to give as much as possible up to the limit of give and receive accordingly more. If false, then prefer to give as little as possible in order to receive as little as possible while receiving at least want.
	Maximal(bool)
}

pub struct Intention{
	/// maximum amount of assets to give from user xc accounts.
	want: BoundedVec<ConstU32<ASSETS_LIMIT>>,
	/// The minimum amount of assets which give should be exchanged for.
	give: BoundedeVec<ConstU32<ASSETS_LIMIT>>,
	
	limit : Limit,
}

pub struct Problem {

}

enum Solution {
	Execute { msgs: BoundedeVec<CosmosMsg<T>, ConstU32<8>> },	
	XcExecute { programs: BoundedeVec<DefaultXCVMPacket, ConstU32<8>> },	
}

pub struct Solver {

}