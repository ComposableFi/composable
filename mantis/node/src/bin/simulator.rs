use itertools::Itertools;
use mantis_node::solver::{orderbook::OrderList, types::Order};

fn main() {
    let orders = (1..100).map(|x| Order::new_random(2., 0.2, (50, 150), x));
    let orders = OrderList {
        value: orders.collect(),
    };
    orders.print();
}
