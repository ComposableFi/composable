
pub struct Solver {
    orders: OrderList,
    target_price: f64,
    buy_token: f64,
    sell_token: f64,
    order: Order,
}

impl Solver {
    pub fn new(orders: OrderList, target_price: f64, buy_token: f64, sell_token: f64) -> Self {
        Self {
            orders,
            target_price,
            buy_token,
            sell_token,
            order: Order::new(0.0, 0.0, OrderType::Buy, "fake-order".to_string()),
        }
    }

    pub fn limit_price(&self) -> f64 {
        self.target_price
    }

    fn f_maximize(&self, order: &Order) -> f64 {
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

    pub fn solve(&mut self, num_orders: usize) -> Result<Solution, &'static str> {
        let original_price = self.orders.compute_optimal_price();
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

    fn match_ob_with_order(&self, order: &Order) -> Result<Solution, &'static str> {
        let mut orderbook = self.orders.clone();
        orderbook.value.push(order.clone());
        orderbook.value.sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap());

        let optimal_price = orderbook.compute_optimal_price();
        orderbook.match_orders(optimal_price)
    }

    fn order_for(&self, amount: f64, is_buy: bool) -> Order {
        if is_buy {
            Order::new(amount, self.limit_price(), OrderType::Buy, "solver".to_string())
        } else {
            Order::new(amount, self.limit_price(), OrderType::Sell, "solver".to_string())
        }
    }
}
