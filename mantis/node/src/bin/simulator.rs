use itertools::Itertools;
use mantis_node::solver::{orderbook::OrderList, solution::Solution, types::Order};
use mantis_node::{prelude::*, solver::types::Price};

fn main() {
    /// randomize price around 2.0 (ratio of 2 price tokens in pair)
    let orders = (1..100).map(|x| Order::new_random(2., 0.1, (50, 150), x));
    let orders = OrderList {
        value: orders.collect(),
    };
    orders.print();
    
    /// solves nothing as no really overlap of orders
    let mut solution = Solution::new(orders.value.clone());
    solution = solution.match_orders(Price::new_float(1.0));
    solution.print();

    /// solves some
    let mut solution = Solution::new(orders.value.clone());
    solution = solution.match_orders(Price::new_float(2.05));
    solution.print();

    /// finds maximal volume price
    let optimal_price = orders.compute_optimal_price(50);

    
    let mut solution = Solution::new(orders.value.clone());
    solution = solution.match_orders(optimal_price);
    solution.print();
}
