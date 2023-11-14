use itertools::Itertools;
use mantis_node::solver::{orderbook::OrderList, solution::Solution, types::Order};
use mantis_node::{prelude::*, solver::types::Price};

fn main() {
    let orders = (1..100).map(|x| Order::new_random(2., 0.2, (50, 150), x));
    let orders = OrderList {
        value: orders.collect(),
    };
    orders.print();

    let mut solution = Solution::new(orders.value.clone());
    solution.match_orders(Price::new_float(1.0));
    solution.print();

    let mut solution = Solution::new(orders.value.clone());
    solution.match_orders(Price::new_float(2.0));
    solution.print();
}
