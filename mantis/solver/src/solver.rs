use crate::orderbook::*;
use crate::prelude::*;
use crate::solution::Solution;
use crate::types::*;

#[derive(Clone, Debug)]
pub struct Solver<Id> {
    orders: OrderList<Id>,
    target_price: Price,
    buy_token: BuyToken,
    sell_token: SellToken,
    order: Order<Id>,
}

impl<Id: Copy + PartialEq + Debug> Solver<Id> {
    /// solver_order_id - allows to provide own liquidity
    pub fn new(orders: OrderList<Id>, target_price: Price, buy_token: BuyToken, sell_token: f64, solver_order_id: Id) -> Self {
        Self {
            orders,
            target_price,
            buy_token,
            sell_token,
            order: Order::new(dec!(0.0), dec!(0.0), OrderType::Buy, solver_order_id),
        }
    }

    pub fn limit_price(&self) -> Price {
        self.target_price
    }

    fn f_maximize(&self, order: &Order<Id>) -> f64 {
        match order.order_type {
            OrderType::Buy => {
                self.buy_token
                    - order.amount_filled
                    + (self.sell_token + order.amount_out) * self.target_price
            }
            OrderType::Sell => {
                (self.buy_token + order.amount_out) / self.target_price + self.sell_token - order.amount_filled
            }
        }
    }

    pub fn solve(&mut self, num_orders: usize) -> Result<Solution<Id>, &'static str> {
        let original_price = self.orders.compute_optimal_price(50);
        let is_buy = original_price > self.target_price;
        let original_token_amount = if is_buy { self.buy_token } else { self.sell_token };

        let orders: Vec<Order> = (0..=num_orders)
            .map(|i| self.order_for(i as f64 * original_token_amount / num_orders as f64, is_buy))
            .collect();

        let mut max_value = 0.0;
        let mut max_solution: Option<Solution> = None;

        for order in &orders {
            let solution = self.match_ob_with_order(order)?;
            let introduced_orders = solution.orders.id(&order.id);

            if let Some(introduced_order) = introduced_orders.first() {
                let f_value = self.f_maximize(introduced_order);
                if max_value < f_value {
                    max_value = f_value;
                    max_solution = Some(solution);
                    self.order = introduced_order.clone();
                }
            }
        }

        max_solution.ok_or("No max solution found")
    }

    // fn match_ob_with_order(&self, order: &Order<Id>) -> Result<Solution<Id>, &'static str> {
    //     let mut orderbook = self.orders.clone();
    //     orderbook.value.push(order.clone());
    //     orderbook.value.sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap());

    //     let optimal_price = orderbook.compute_optimal_price(50);
    //     Ok(Solution(orderbook, optimal_price))
    // }

    // fn order_for(&self, amount: f64, order_type: OrderType) -> Order<Id> {
    //         Order::new(amount, self.limit_price(), order_type, 0)
    // }
}
