#![allow(clippy::disallowed_methods)] // does unwrap inside

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{wasm_execute, Addr, BankMsg, Coin, Event, Order, StdError, Uint128, Uint64};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use itertools::Itertools;
use sylvia::{
	contract,
	cw_std::{ensure, Response, StdResult},
	entry_points,
	types::{ExecCtx, InstantiateCtx, QueryCtx},
};

/// so this is just to make code easy to read, we will optimize later
use num_rational::BigRational;

// fix core in std in contract
// use xc_core::{service::dex::ExchangeId, shared::Displayed, NetworkId};
pub type ExchangeId = Uint128;
pub type Amount = u128;
pub type OrderId = u128;
pub type NetworkId = u32;
pub type Blocks = u32;

/// each pair waits ate least this amount of blocks before being decided
pub const BATCH_EPOCH: u32 = 1;

/// count of solutions at minimum which can be decided, just set 1 for ease of devtest
pub const MIN_SOLUTION_COUNT: u32 = 1;

/// parts of a whole, numerator / denominator
pub type Ratio = (Uint64, Uint64);

#[cw_serde]
pub struct OrderSubMsg {
	/// Amount is minimum amount to get for given amount (sure user wants more than `wants` and we
	/// try to achieve that). Denom users wants to get, it can be cw20, bank or this chain CVM
	/// asset identifier. Only local CVM identifiers are accepted.
	/// If target asset identifier on other chain, use `transfer` to identity it.
	/// Why this is the case? It allows to CoW with user wanted assets which is not on
	/// settlement(this) chain.
	pub wants: Coin,

	/// How offchain SDK must work with it?
	/// ```example
	/// Alice gives token 42 on this(settlement chain).
	/// But she wants token 123 on other chain.
	/// SDK reads all CVM configurations.
	/// And tells Alice that there are 2 routes of asset 123 to/from settlement chain.
	/// These routes are 666 and 777. Each asset has unique route to settlement chain in CVM configuration.
	/// Alice picks route 777.
	/// So SDK sends 42 token as given to  and 777 as wanted,
	/// but additionally with attached transfer route Alice picked.  
	/// ```
	/// This allow to to CoWs for assets not on this chain.
	pub transfer: Option<TransferRoute>,
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
	pub order_id: OrderId,
}

#[cw_serde]
pub struct SolutionItem {
	pub pair: (String, String),
	pub msg: SolutionSubMsg,
	/// at which block solution was added
	pub block_added: u64,
}
/// price information will not be used on chain or deciding.
/// it will fill orders on chain as instructed
/// and check that max/min from orders respected
/// and sum all into volume. and compare solutions.
/// on chain cares each user gets what it wants and largest volume solution selected.
#[cw_serde]
pub struct SolutionSubMsg {
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub cows: Vec<Cow>,
	/// must adhere Connection.fork_join_supported, for now it is always false (it restrict set of
	/// routes possible)
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub routes: Vec<ExchangeRoute>,

	/// after some time, solver will not commit to success
	pub timeout: Blocks,
}

/// after cows solved, need to route remaining cross chain
#[cw_serde]
pub struct RouteSubMsg {
	pub all_orders: Vec<SolvedOrder>,
	pub routes: Vec<ExchangeRoute>,
}

/// how much of order to be solved by CoW.
/// difference with `Fill` to be solved by cross chain exchange
/// aggregate pool of all orders in solution is used to give user amount he wants.
#[cw_serde]
pub struct Cow {
	pub order_id: Uint128,
	/// how much of order to be solved by from bank for all aggregated cows
	pub cow_amount: Uint128,
	/// amount of order to be taken (100% in case of full fill, can be less in case of partial)
	pub taken: Option<Uint128>,
	/// amount user should get after order executed
	pub given: Uint128,
}

#[cw_serde]
pub struct SolvedOrder {
	pub order: OrderItem,
	pub solution: Cow,
}

impl SolvedOrder {
	pub fn new(order: OrderItem, solution: Cow) -> StdResult<Self> {
		ensure!(
			order.msg.wants.amount.u128() <= solution.given.u128(),
			StdError::generic_err(format!(
				"user limit was not satisfied {:?} {:?} ",
				&order, &solution
			))
		);

		Ok(Self { order, solution })
	}

	pub fn cross_chain(&self) -> u128 {
		self.order.msg.wants.amount.u128() - self.solution.cow_amount.u128()
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

/// Route which may spawn on the way.
#[cw_serde]
pub struct ExchangeRoute {
	// on this chain
	pub exchange: Vec<Exchange>,
	pub spawn: Vec<Spawn<ExchangeRoute>>,
}

/// Purely transfer route.
#[cw_serde]
pub struct TransferRoute {
	pub spawn: Vec<Spawn<TransferRoute>>,
}

/// Abstracted out route of underlying encoding on specific transport.
/// In the end of route, amount is always put onto user CVM executor.
#[cw_serde]
pub struct Spawn<Route> {
	pub to_chain: NetworkId,
	pub carry: Vec<Uint128>,
	pub execute: Option<Route>,
}

#[cw_serde]
pub struct Exchange {
	pub pool_id: ExchangeId,
	pub give: Uint128,
	pub want_min: Uint128,
}

pub struct OrderContract<'a> {
	pub orders: Map<'a, u128, OrderItem>,
	/// (a,b,solver)
	pub solutions: IndexedMap<'a, &'a (String, String, Addr), SolutionItem, SolutionIndexes<'a>>,
	pub next_order_id: Item<'a, u128>,
}

/// so we need to have several solution per pair to pick one best
pub struct SolutionIndexes<'a> {
	pub pair: MultiIndex<'a, (String, String), SolutionItem, (String, String, Addr)>,
}

impl<'a> IndexList<SolutionItem> for SolutionIndexes<'a> {
	fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<SolutionItem>> + '_> {
		let v: Vec<&dyn Index<SolutionItem>> = vec![&self.pair];
		Box::new(v.into_iter())
	}
}

pub fn solution<'a>(
) -> IndexedMap<'a, &'a (String, String, Addr), SolutionItem, SolutionIndexes<'a>> {
	let indexes = SolutionIndexes {
		pair: MultiIndex::new(|_pk: &[u8], d: &SolutionItem| d.pair.clone(), "pair_solver", "pair"),
	};
	IndexedMap::new("solutions", indexes)
}

impl Default for OrderContract<'_> {
	fn default() -> Self {
		Self {
			orders: Map::new("orders"),
			next_order_id: Item::new("next_order_id"),
			solutions: solution(),
		}
	}
}

#[entry_points]
#[contract]
impl OrderContract<'_> {
	pub fn new() -> Self {
		Self::default()
	}
	#[msg(instantiate)]
	pub fn instantiate(
		&self,
		_ctx: InstantiateCtx, /*	i think we would need admin, gateway, scoring/fee parameters */
	) -> StdResult<Response> {
		Ok(Response::default())
	}

	/// This contracts receives user order, takes ddos protection deposit (to protect solvers from
	/// spamming), and stores order for searchers.
	#[msg(exec)]
	pub fn order(&self, ctx: ExecCtx, msg: OrderSubMsg) -> StdResult<Response> {
		// for now we just use bank for ics20 tokens
		let funds = ctx.info.funds.get(0).expect("there are some funds in order");

		// just save order under incremented id
		let order_id = self.next_order_id.load(ctx.deps.storage).unwrap_or_default();
		let order = OrderItem { msg, given: funds.clone(), order_id, owner: ctx.info.sender };
		self.orders.save(ctx.deps.storage, order_id, &order)?;
		self.next_order_id.save(ctx.deps.storage, &(order_id + 1))?;
		let order_created =
			Event::new("mantis-order-created").add_attribute("order_id", order_id.to_string());
		ctx.deps.api.debug(&format!("mantis::order::created: {:?}", order));
		Ok(Response::default().add_event(order_created))
	}

	#[msg(exec)]
	pub fn route(&self, ctx: ExecCtx, _msg: RouteSubMsg) -> StdResult<Response> {
		ensure!(
			ctx.info.sender == ctx.env.contract.address,
			StdError::GenericErr { msg: "only self can call this".to_string() }
		);
		unimplemented!(
			"so here we add route execution tracking to storage and map route to CVM program"
		)
	}

	/// Provides solution for set of orders.
	/// All fully
	#[msg(exec)]
	pub fn solve(&self, ctx: ExecCtx, msg: SolutionSubMsg) -> StdResult<Response> {
		// read all orders as solver provided
		let mut all_orders = self.merge_solution_with_orders(&msg, &ctx)?;
		let at_least_one = all_orders.first().expect("at least one");

		// normalize pair
		let mut ab = [at_least_one.given().denom.clone(), at_least_one.wants().denom.clone()];
		ab.sort();
		let [a, b] = ab;

		// add solution to total solutions
		let possible_solution =
			SolutionItem { pair: (a.clone(), b.clone()), msg, block_added: ctx.env.block.height };

		self.solutions.save(
			ctx.deps.storage,
			&(a.clone(), b.clone(), ctx.info.sender.clone()),
			&possible_solution,
		)?;
		let solution_upserted = Event::new("mantis-solution-upserted")
			.add_attribute("pair", &format!("{}{}", a, b))
			.add_attribute("solver", &ctx.info.sender.to_string());
		ctx.deps
			.api
			.debug(&format!("mantis::solution::upserted {:?}", &solution_upserted));

		// get all solution for pair
		let all_solutions: Result<Vec<SolutionItem>, _> = self
			.solutions
			.idx
			.pair
			.prefix((a.clone(), b.clone()))
			.range(ctx.deps.storage, None, None, Order::Ascending)
			.map(|r| r.map(|(_, solution)| solution))
			.collect();
		let all_solutions = all_solutions?;
		ctx.deps.api.debug(&format!("mantis::solutions::current {:?}", all_solutions));

		// pick up optimal solution with solves with bank
		let mut a_in = 0;
		let mut b_in = 0;
		let mut transfers = vec![];
		let mut solution_item = possible_solution;
		for solution in all_solutions {
			let alternative_all_orders = self.merge_solution_with_orders(&solution.msg, &ctx)?;
			let a_total_in: u128 = alternative_all_orders
				.iter()
				.filter(|x| x.given().denom == a)
				.map(|x: &SolvedOrder| x.given().amount.u128())
				.sum();
			let b_total_in: u128 = alternative_all_orders
				.iter()
				.filter(|x| x.given().denom == b)
				.map(|x| x.given().amount.u128())
				.sum();
			// so do all cows up to bank
			let alternative_transfers = solves_cows_via_bank(
				alternative_all_orders.clone(),
				a.clone(),
				a_total_in,
				b_total_in,
			);
			ctx.deps
				.api
				.debug(&format!("mantis::solutions::alternative {:?}", &alternative_transfers));
			if let Ok(alternative_transfers) = alternative_transfers {
				if a_total_in * b_total_in > a_in * b_in {
					a_in = a_total_in;
					b_in = b_total_in;
					all_orders = alternative_all_orders;
					transfers = alternative_transfers;
					solution_item = solution;
				}
			}
		}

		// send remaining for settlement
		let route = wasm_execute(
			ctx.env.contract.address,
			&ExecMsg::route(RouteSubMsg { all_orders, routes: solution_item.msg.routes }),
			vec![],
		)?;

		let solution_chosen = Event::new("mantis-solution-chosen")
			.add_attribute("pair", format!("{}{}", a, b))
			.add_attribute("solver", ctx.info.sender.to_string());
		ctx.deps.api.debug(&format!("mantis-solution-chosen: {:?}", &solution_chosen));
		Ok(Response::default()
			.add_messages(transfers)
			.add_message(route)
			.add_event(solution_upserted)
			.add_event(solution_chosen))
	}

	fn merge_solution_with_orders(
		&self,
		msg: &SolutionSubMsg,
		ctx: &ExecCtx<'_>,
	) -> Result<Vec<SolvedOrder>, StdError> {
		let all_orders = msg
			.cows
			.iter()
			.map(|x| {
				self.orders
					.load(ctx.deps.storage, x.order_id.u128())
					.map_err(|_| StdError::not_found("order"))
					.and_then(|order| SolvedOrder::new(order, x.clone()))
			})
			.collect::<Result<Vec<SolvedOrder>, _>>()?;
		Ok(all_orders)
	}

	/// Simple get all orders
	#[msg(query)]
	pub fn get_all_orders(&self, ctx: QueryCtx) -> StdResult<Vec<OrderItem>> {
		self.orders
			.range_raw(ctx.deps.storage, None, None, Order::Ascending)
			.map(|r| r.map(|(_, order)| order))
			.collect::<StdResult<Vec<OrderItem>>>()
	}

	#[msg(query)]
	pub fn get_all_solutions(&self, ctx: QueryCtx) -> StdResult<Vec<SolutionItem>> {
		self.solutions
			.idx
			.pair
			.range_raw(ctx.deps.storage, None, None, Order::Ascending)
			.map(|r| r.map(|(_, x)| x))
			.collect()
	}
}

fn solves_cows_via_bank(
	mut all_orders: Vec<SolvedOrder>,
	a: String,
	a_total_in: u128,
	b_total_in: u128,
) -> Result<Vec<BankMsg>, StdError> {
	let mut a_total_in = BigRational::from_integer(a_total_in.into());
	let mut b_total_in = BigRational::from_integer(b_total_in.into());
	let mut transfers = vec![];
	for order in all_orders.iter_mut() {
		let cowed = order.solution.cow_amount;
		let amount = Coin { amount: cowed.into(), ..order.given().clone() };

		// so if not enough was deposited as was taken from original orders, it will fails - so
		// solver cannot rob the bank
		if amount.denom == a {
			a_total_in -= BigRational::from_integer(cowed.u128().into());
		} else {
			b_total_in -= BigRational::from_integer(cowed.u128().into());
		};
		transfers
			.push(BankMsg::Send { to_address: order.owner().to_string(), amount: vec![amount] });
	}
	if a_total_in < BigRational::default() || b_total_in < BigRational::default() {
		return Err(StdError::generic_err("SolutionForCowsViaBankIsNotBalanced"));
	}
	Ok(transfers)
}
