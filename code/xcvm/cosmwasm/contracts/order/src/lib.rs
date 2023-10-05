use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Order, Uint128, Uint64, StdError};
use cw_storage_plus::{Item, Map};
use sylvia::{
	contract,
	cw_std::{ensure, Response, StdResult},
	entry_points,
	types::{ExecCtx, InstantiateCtx, QueryCtx},
};

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
	pub msg: OrderSubMsg,
	pub coin: Coin,
order_id: u128,
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
		let order = OrderItem { msg, coin: funds.clone(), order_id, };
		self.orders.save(ctx.deps.storage, order_id, &order)?;
		self.next_order_id.save(ctx.deps.storage, &(order_id + 1))?;
		
		Ok(Response::default())
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
