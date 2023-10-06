use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Order, Uint128, Uint64, StdError};
use cw_storage_plus::{Item, Map};
use sylvia::{
	contract,
	cw_std::{ensure, Response, StdResult},
	entry_points,
	types::{ExecCtx, InstantiateCtx, QueryCtx},
};
use cosmwasm_std::Addr;
use xc_core::{service::dex::ExchangeId, NetworkId};


pub type Amount = u128;
pub type OrderId = u128;
pub type Blocks = u32;

/// parts of a whole, numerator / denominator
pub type Ratio = (Uint64, Uint64);

#[cw_serde]
pub struct OrderSubMsg {
	/// denom users wants to get, it can be cw20, bank or cvm denoms
	pub denom: String,
	/// minimum amount to get for given amount given (sure user wants more than at least `wants`)
	pub wants: u128,
	/// how much blocks to wait for solution, if none, then cleaned up
	pub timeout: Blocks,
	/// if ok with partial fill, what is the minimum amount
	pub min_fill: Option<Ratio>,
}

#[cw_serde]
pub struct OrderItem {
	pub owner : Addr,
	pub msg: OrderSubMsg,
	pub coin: Coin,
	pub order_id: u128,
}

/// price information will not be used on chain or deciding.
/// it will fill orders on chain as instructed
/// and check that max/min from orders respected
/// and sum all into volume. and compare solutions.
/// on chain cares each user gets what it wants and largest volume solution selected.
#[cw_serde]
pub struct SolutionSubMsg {
    pub cows: Vec<Cow>,
    pub fill: Vec<Fill>,
    /// must adhere Connection.fork_join_supported, for now it is always false (it restrict set of routes possible)
    pub routes: Vec<Route>,	
}

/// how much of order to be solved by CoW.
/// difference with `Fill` to be solved by cross chain exchange
/// aggregate pool of all orders in solution is used to give user amount he wants.
#[cw_serde]
pub struct Cow {
    pub order_id: OrderId,
    pub amount: u128,
}


#[cw_serde]
pub struct Route {
    // on this chain
    pub exchange: Vec<Exchange>,
    pub spawn: Vec<Spawn>,
}

#[cw_serde]
pub struct Spawn {
    pub to_chain: NetworkId,
    pub carry: Vec<Amount>,
    pub execute: Option<Route>,
}

#[cw_serde]
pub struct Exchange {
    pub pool_id: ExchangeId,
    pub give: Amount,
    pub want_min: Amount,
}

#[cw_serde]
pub struct Fill {
    pub order_id: OrderId,
    /// amount of order to be taken (100% in case of full fill, can be less in case of partial)
    pub taken: u128,
    /// amount user should get after order executed
    pub given: u128,
}


pub struct OrderContract<'a> {
	pub orders: Map<'a, u128, OrderItem>,
	pub next_order_id: Item<'a, u128>,
}

#[entry_points]
#[contract]
impl OrderContract<'_> {
	pub fn new() -> Self {
		Self { 
			orders: Map::new("orders"),
			next_order_id: Item::new("next_order_id"),
		 }
	}

	#[msg(instantiate)]
	pub fn instantiate(&self, _ctx: InstantiateCtx) -> StdResult<Response> {
		Ok(Response::default())
	}

	/// This contracts receives user order, takes ddos protection deposit (to protect solvers from
	/// spamming), and stores order for searchers.
	#[msg(exec)]
	pub fn order(&self, ctx: ExecCtx, msg: OrderSubMsg) -> StdResult<Response> {
		/// for now we just use bank for ics20 tokens
		let funds = ctx.info.funds.get(0).expect("there are some funds in order");
		

		/// just save order under incremented id
		let order_id = self.next_order_id.load(ctx.deps.storage).unwrap_or_default();
		let order = OrderItem { msg, coin: funds.clone(), order_id, owner: ctx.info.sender };
		self.orders.save(ctx.deps.storage, order_id, &order)?;
		self.next_order_id.save(ctx.deps.storage, &(order_id + 1))?;
		
		Ok(Response::default())
	}

	/// Provides solution for set of orders.
	/// All fully
	#[msg(exec)]
	pub fn solve(&self, ctx: ExecCtx, msg: OrderSubMsg) -> StdResult<Response> {
		todo!()
	}

	/// Simple get all orders
	#[msg(query)]
	pub fn get_all_orders(&self, ctx: QueryCtx) -> StdResult<Vec<OrderItem>> {
		self.orders
		 	.range_raw(ctx.deps.storage, None, None, Order::Ascending)
			 .map(|r| r.map(|(_, order)|  order))
			.collect::<StdResult<Vec<OrderItem>>>()
	}

	// next steps are:
	// 1. receive solution without verification
	// 2. solves cows
	// 3. send CVM program for cross chain
}
