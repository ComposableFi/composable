use crate::orderbook::*;
use crate::prelude::*;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct Solution<Id: Copy + PartialEq> {
    pub orders: OrderList<Id>,
    pub matched_price: Price,
    pub buy_volume: BuyToken,
    pub sell_volume: SellToken,
}

impl<Id: Copy + PartialEq> Solution<Id> {
    pub fn new(orders: Vec<Order<Id>>) -> Self {
        let mut order_list = OrderList {
            value: orders.clone(),
        };
        order_list.value.sort_by(|a, b| {
            a.limit_price
                .partial_cmp(&b.limit_price)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let matched_price = if !order_list.is_empty() {
            order_list.value[0].filled_price
        } else {
            dec!(0.0)
        };

        let buy_volume = order_list.sell().amount_out();
        let sell_volume = order_list.buy().amount_out();

        Self {
            orders: order_list,
            matched_price: Price(matched_price),
            buy_volume: buy_volume,
            sell_volume: sell_volume,
        }
    }

    pub fn sell_orders(&self) -> &OrderList<Id> {
        &self.orders.sell()
    }

    pub fn buy_orders(&self) -> &OrderList<Id> {
        &self.orders.buy()
    }

    pub fn check_constraints(&self) {
        const EPSILON: f64 = 1e-20;
        if !((self.buy_volume - self.orders.buy().amount_filled()).abs() < EPSILON
            || (self.sell_volume - self.orders.sell().amount_filled()).abs() < EPSILON)
        {
            panic!(
                "Error buy_volume: {} Buy amount filled: {} sell_volume: {} Buy amount filled: {}",
                self.buy_volume,
                self.orders.buy().amount_filled(),
                self.sell_volume,
                self.orders.sell().amount_filled()
            );
        }
    }

    pub fn print(&self) {
        println!("{} Start Solution {}", "#".repeat(20), "#".repeat(20));
        self.orders.print();
        println!(
            "\x1b[1mMatched Price {:.4} \tSell volume {:.4}\tBuy volume {:.4}\x1b[0m",
            self.matched_price, self.sell_volume, self.buy_volume
        );
        println!("{} End Solution {}", "#".repeat(20), "#".repeat(20));
    }

    fn match_orders(&mut self, price: Price) -> Solution<Id> {
        let mut orders = self.orders.clone();
        orders.value.sort_by(|a, b| {
            a.limit_price
                .partial_cmp(&b.limit_price)
                .unwrap_or(Ordering::Equal)
        });

        let matched = orders.is_acceptable_price(price);
        let buy_orders = matched.buy();
        let sell_orders = matched.sell();

        let buy_volume = buy_orders.token1_sum(price);
        let sell_volume = sell_orders.token1_sum(price);

        let is_buy_predominant = buy_volume > sell_volume;

        if is_buy_predominant {
            orders.resolve_predominant(
                &mut buy_orders.clone(),
                &mut sell_orders.clone(),
                price,
            );
        } else {
            orders.resolve_predominant(
                &mut sell_orders.clone(),
                &mut buy_orders.clone(),
                price,
            );
        }

        let mut solution = Solution {
            orders: matched.filled().clone(),
            matched_price: Price(Decimal::new(0, 0)),
            buy_volume: BuyToken(Decimal::new(0, 0)),
            sell_volume: SellToken(Decimal::new(0, 0)),
        };

        solution.check_constraints();
        solution
    }

    /// Assuming Order has a random method.
    pub fn random(num_orders: usize, next: fn() -> Id) -> Self {
        Self::new((0..num_orders).map(|_| Order::random(next())).collect())
    }
}
