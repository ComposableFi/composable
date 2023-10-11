#![feature(result_flattening)]
mod error;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BankMsg, Coin, Order, StdError, Uint128, Uint64};
use cw_storage_plus::{Item, Map};
use itertools::*;
use sylvia::{
	contract,
	cw_std::{ensure, Response, StdResult},
	entry_points,
	types::{ExecCtx, InstantiateCtx, QueryCtx},
};
use xc_core::{service::dex::ExchangeId, shared::Displayed, NetworkId};

/// so this is just to make code easy to read, we will optimize later
use num_rational::BigRational;

use crate::error::ContractError;

pub type Amount = Displayed<u128>;
pub type OrderId = Displayed<u128>;
pub type Blocks = u32;

/// parts of a whole, numerator / denominator
pub type Ratio = (Uint64, Uint64);

#[cw_serde]
pub struct OrderSubMsg {
	/// denom users wants to get, it can be cw20, bank or cvm denoms
	/// minimum amount to get for given amount given (sure user wants more than at least `wants`)
	pub wants: Coin,

	/// how much blocks to wait for solution, if none, then cleaned up
	pub timeout: Blocks,
	/// if ok with partial fill, what is the minimum amount
	pub min_fill: Option<Ratio>,
}

#[cw_serde]
pub struct OrderItem {
	pub owner: Addr,
	pub msg: OrderSubMsg,
	pub given: Coin,
	pub order_id: Displayed<u128>,
}

/// price information will not be used on chain or deciding.
/// it will fill orders on chain as instructed
/// and check that max/min from orders respected
/// and sum all into volume. and compare solutions.
/// on chain cares each user gets what it wants and largest volume solution selected.
#[cw_serde]
pub struct SolutionSubMsg {
	pub cows: Vec<Cow>,
	/// must adhere Connection.fork_join_supported, for now it is always false (it restrict set of
	/// routes possible)
	pub routes: Vec<Route>,
}

/// how much of order to be solved by CoW.
/// difference with `Fill` to be solved by cross chain exchange
/// aggregate pool of all orders in solution is used to give user amount he wants.
#[cw_serde]
pub struct Cow {
	pub order_id: OrderId,
	/// how much of order to be solved by from bank for all aggregated cows
	pub cow_amount: Displayed<u128>,
	/// amount of order to be taken (100% in case of full fill, can be less in case of partial)
	pub taken: Option<Displayed<u128>>,
	/// amount user should get after order executed
	pub given: Displayed<u128>,
}

pub struct SolvedOrder {
	pub order: OrderItem,
	pub solution: Cow,
}

impl SolvedOrder {
	pub fn new(order: OrderItem, solution: Cow) -> StdResult<Self> {
		ensure!(
			order.msg.wants.amount.u128() >= solution.given.0,
			StdError::GenericErr { msg: "user limit was not satisfied".to_string() }
		);

		Ok(Self { order, solution })
	}

	pub fn cross_chain(&self) -> u128 {
		self.order.msg.wants.amount.u128() - self.solution.cow_amount.0
	}

	pub fn given(&self) -> &Coin {
		&self.order.given
	}

	pub fn wants(&self) -> &Coin {
		&self.order.msg.wants
	}

	pub fn owner(&self) -> &Addr {
		&self.order.owner
	}
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

pub struct OrderContract<'a> {
	pub orders: Map<'a, u128, OrderItem>,
	pub next_order_id: Item<'a, u128>,
}

#[entry_points]
#[contract]
impl OrderContract<'_> {
	pub fn new() -> Self {
		Self { orders: Map::new("orders"), next_order_id: Item::new("next_order_id") }
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
		let order_id = self.next_order_id.load(ctx.deps.storage).unwrap_or_default().into();
		let order = OrderItem { msg, given: funds.clone(), order_id, owner: ctx.info.sender };
		self.orders.save(ctx.deps.storage, order_id.0, &order)?;
		self.next_order_id.save(ctx.deps.storage, &(order_id.0 + 1))?;
		Ok(Response::default())
	}

	/// Provides solution for set of orders.
	/// All fully
	#[msg(exec)]
	pub fn solve(&self, ctx: ExecCtx, msg: SolutionSubMsg) -> StdResult<Response> {
		/// read all orders as solver provided
		let mut all_orders = msg
			.cows
			.iter()
			.map(|x| {
				self.orders
					.load(ctx.deps.storage, x.order_id.0)
					.map_err(|_| StdError::not_found("order"))
					.map(|order| SolvedOrder::new(order, x.clone()))
					.flatten()
			})
			.collect::<Result<Vec<SolvedOrder>, _>>()?;
		let at_least_one = all_orders.first().expect("at least one");

		/// unfortunately itertools std really:
		/// Disable to compile itertools using #![no_std]. This disables any items that depend on
		/// collections (like group_by, unique, kmerge, join and many more).
		let a = at_least_one.given().denom.clone();
		let b = at_least_one.wants().denom.clone();

		/// total in bank as put by all `order` calls
		/// very inefficient, but for now it is ok - let do logic, than make it secure, than
		/// efficient - or we never release so ignores fully Constant factors and sometimes n**2
		/// instead of n log n
		let mut a_total_in: u128 = all_orders
			.iter()
			.filter(|x| x.given().denom == a)
			.map(|x: &SolvedOrder| x.given().amount.u128())
			.sum();
		let mut b_total_in: u128 = all_orders
			.iter()
			.filter(|x| x.given().denom == b)
			.map(|x| x.given().amount.u128())
			.sum();

		/// so do all cows up to bank
		let mut transfers = vec![];
		for order in all_orders.iter_mut() {
			let cowed = order.solution.cow_amount;
			let amount = Coin { amount: cowed.0.into(), ..order.given().clone() };

			if amount.denom == a {
				a_total_in -= cowed.0;
			} else {
				b_total_in -= cowed.0;
			};
			transfers.push(BankMsg::Send {
				to_address: order.owner().to_string(),
				amount: vec![amount],
			});
		}

		Ok(Response::default().add_messages(transfers))
	}

	/// Simple get all orders
	#[msg(query)]
	pub fn get_all_orders(&self, ctx: QueryCtx) -> StdResult<Vec<OrderItem>> {
		self.orders
			.range_raw(ctx.deps.storage, None, None, Order::Ascending)
			.map(|r| r.map(|(_, order)| order))
			.collect::<StdResult<Vec<OrderItem>>>()
	}

	// next steps are:
	// 1. receive solution without verification
	// 2. solves cows
	// 3. send CVM program for cross chain
}
