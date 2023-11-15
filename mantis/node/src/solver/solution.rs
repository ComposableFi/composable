use crate::prelude::*;
use crate::solver::orderbook::*;
use crate::solver::types::*;

#[derive(Clone, Debug)]
pub struct Solution<Id> {
    pub orders: OrderList<Id>,
    pub matched_price: Price,
    pub buy_volume: BuyToken,
    pub sell_volume: SellToken,
}

impl<Id: Copy + PartialEq + Debug> Solution<Id> {
    /// ensures orders are sorted
    pub fn new(mut orders: Vec<Order<Id>>) -> Self {
        orders.sort_by(|a, b| {
            a.limit_price
                .partial_cmp(&b.limit_price)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let orders = OrderList { value: orders };
        let matched_price = orders
            .value
            .get(0)
            .map(|x| x.filled_price)
            .unwrap_or_default();
        let buy_volume = orders.sell().amount_out().0.into();
        let sell_volume = orders.buy().amount_out().0.into();

        Self {
            orders,
            matched_price: Price(matched_price),
            buy_volume,
            sell_volume,
        }
    }

    pub fn sell_orders(&self) -> OrderList<Id> {
        self.orders.clone().sell()
    }

    pub fn buy_orders(&self) -> OrderList<Id> {
        self.orders.clone().buy()
    }

    pub fn check_constraints(&self) {
        const EPSILON: Decimal = dec!(1e-20);
        if !((self.buy_volume.0 - self.orders.buy().amount_filled().0).abs() < EPSILON
            || (self.sell_volume.0 - self.orders.sell().amount_filled().0).abs() < EPSILON)
        {
            panic!(
                "Error buy_volume: {} Buy amount filled: {} sell_volume: {} Buy amount filled: {}",
                self.buy_volume.0,
                self.orders.buy().amount_filled().0,
                self.sell_volume.0,
                self.orders.sell().amount_filled().0,
            );
        }
    }

    pub fn print(&self) {
        println!("{} Start Solution {}", "#".repeat(20), "#".repeat(20));
        self.orders.print();
        println!(
            "\x1b[1mMatched Price {:.4} \tSell volume {:.4}\tBuy volume {:.4}\x1b[0m",
            self.matched_price.0, self.sell_volume.0, self.buy_volume.0
        );
        println!("{} End Solution {}", "#".repeat(20), "#".repeat(20));
    }

    /// Fills order according `price`
    /// If order is not OK the provided `price`, not filled.
    pub fn match_orders(&mut self, price: Price) -> Solution<Id> {
        let mut orders = self.orders.clone();

        orders.value.sort_by(|a, b| {
            a.limit_price
                .partial_cmp(&b.limit_price)
                .unwrap_or(Ordering::Equal)
        });

        let matched = orders.is_acceptable_price(price);
        let mut buy_orders = matched.buy();
        let mut sell_orders = matched.sell();
        let buy_volume = buy_orders.token1_sum(price);
        let sell_volume = sell_orders.token1_sum(price);

        let is_buy_predominant = buy_volume > sell_volume;

        if is_buy_predominant {
            OrderList::resolve_predominant(&mut buy_orders, &mut sell_orders, price);
        } else {
            OrderList::resolve_predominant(&mut sell_orders, &mut buy_orders, price);
        }

        /// for now retaining "bad" design when solution has no price, there should no be solution without, will fix after testing correctness
        let matched = OrderList {
            value: [buy_orders.value, sell_orders.value]
                .concat()
                .into_iter()
                .collect(),
        };
        let mut solution = Solution::new(matched.filled().value);

        solution.check_constraints();
        solution
    }

    /// Assuming Order has a random method.
    pub fn new_random(
        num_orders: usize,
        mean: f64,
        std: f64,
        volume_range: (u64, u64),
        next: fn() -> Id,
    ) -> Self {
        Self::new(
            (0..num_orders)
                .map(|_| Order::new_random(mean, std, volume_range, next()))
                .collect(),
        )
    }
}
