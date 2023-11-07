use rand_distr::num_traits::FromPrimitive;

use crate::prelude::*;
use crate::solver::orderbook::*;
use crate::solver::solution::Solution;
use crate::solver::types::*;

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
    pub fn new(
        orders: OrderList<Id>,
        target_price: Price,
        buy_token: BuyToken,
        sell_token: SellToken,
        solver_order_id: Id,
    ) -> Self {
        Self {
            orders,
            target_price,
            buy_token,
            sell_token,
            order: Order::new(dec!(0.0), Price(dec!(0.0)), OrderType::Buy, solver_order_id),
        }
    }

    pub fn limit_price(&self) -> Price {
        self.target_price
    }

    fn f_maximize(&self, order: &Order<Id>) -> Amount {
        match order.order_type {
            OrderType::Buy => {
                self.buy_token.0 - order.amount_filled
                    + (self.sell_token.0 + order.amount_out) * self.target_price.0
            }
            OrderType::Sell => {
                (self.buy_token.0 + order.amount_out) / self.target_price.0 + self.sell_token.0
                    - order.amount_filled
            }
        }
    }

    /// next_id - is used to generate solver order to match remaining amount via CFMM later
    pub fn solve(
        &mut self,
        num_orders: usize,
        next_id: fn() -> Id,
    ) -> Result<Solution<Id>, &'static str> {
        let original_price = self.orders.compute_optimal_price(50);
        let is_buy = original_price > self.target_price;
        let original_token_amount = if is_buy {
            self.buy_token.0
        } else {
            self.sell_token.0
        };

        let side = if is_buy {
            OrderType::Buy
        } else {
            OrderType::Sell
        };

        let orders: Vec<Order<_>> = (0..=num_orders)
            .map(|i| {
                self.order_for(
                    Amount::from_usize(i).expect("works") * original_token_amount
                        / Amount::from_usize(num_orders).expect("works"),
                    side,
                    next_id(),
                )
            })
            .collect();

        let mut max_value = dec!(0.0);
        let mut max_solution: Option<Solution<_>> = None;

        for order in &orders {
            let solution = self.match_ob_with_order(order)?;
            let introduced_orders = solution.orders.id(order.id);

            if let Some(introduced_order) = introduced_orders.value.first() {
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

    fn match_ob_with_order(&self, order: &Order<Id>) -> Result<Solution<Id>, &'static str> {
        let mut orderbook = self.orders.clone();
        orderbook.value.push(order.clone());
        orderbook
            .value
            .sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap());

        let optimal_price = orderbook.compute_optimal_price(50);
        Ok(Solution::new(orderbook.value).match_orders(optimal_price))
    }

    fn order_for(&self, amount: Decimal, order_type: OrderType, id: Id) -> Order<Id> {
        Order::new(amount, self.limit_price(), order_type, id)
    }
}
