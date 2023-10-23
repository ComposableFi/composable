



// #[derive(Clone, Debug)]
// struct CFMM {
//     r0: f64,
//     r1: f64,
//     chain_id: i32,
//     fee: f64,
// }

// impl CFMM {
//     pub fn new(r0: f64, r1: f64, chain_id: i32, fee: f64) -> Self {
//         Self {
//             r0,
//             r1,
//             chain_id,
//             fee,
//         }
//     }

//     pub fn gamma(&self) -> f64 {
//         1.0 - self.fee
//     }

//     pub fn set_gamma(&mut self, value: f64) {
//         self.fee = 1.0 - value;
//     }

//     pub fn sell(&mut self, delta: f64, simulate: bool) -> f64 {
//         let amount_out = self.swap(delta, self.r1, self.r0);
//         if !simulate {
//             self.r0 -= amount_out;
//             self.r1 += delta;
//         }
//         amount_out
//     }

//     pub fn buy(&mut self, delta: f64, simulate: bool) -> f64 {
//         let amount_out = self.swap(delta, self.r0, self.r1);
//         if !simulate {
//             self.r1 -= amount_out;
//             self.r0 += delta;
//         }
//         amount_out
//     }

//     fn swap(&self, delta: f64, in_reserve: f64, out_reserve: f64) -> f64 {
//         out_reserve - in_reserve * out_reserve / (in_reserve + self.gamma() * delta)
//     }

//     pub fn price(&self) -> f64 {
//         self.r0 / self.r1
//     }

//     pub fn random(r0_range: (f64, f64), r1_range: (f64, f64)) -> Self {
//         let mut rng = rand::thread_rng();
//         let r0 = rng.gen_range(r0_range.0..r0_range.1);
//         let r1 = rng.gen_range(r1_range.0..r1_range.1);
//         CFMM::new(r0, r1, 1, 0.03)
//     }
// }


// // struct Mechanism {
// //     orderbooks: Vec<Solution>,
// // }

// // impl Mechanism {
// //     pub fn new() -> Self {
// //         Self {
// //             orderbooks: Vec::new(),
// //         }
// //     }

// //     pub fn submit_orderbook(&mut self, orderbook: Solution) {
// //         assert!(!orderbook.is_empty(), "Orderbook must not be empty");
// //         self.orderbooks.push(orderbook);
// //     }
// // }


// // pub struct CFMMSolver {
// //     cfmm: CFMM,
// //     orders: Solution,
// //     buy_token: f64, // Assuming a simple type for illustration
// //     sell_token: f64,
// //     _optimal_price: f64,
// // }

// // impl CFMMSolver {
// //     pub fn new(cfmm: CFMM, ob: Solution, buy_token: f64, sell_token: f64) -> Self {
// //         let _optimal_price = ob.compute_optimal_price();
// //         Self {
// //             cfmm,
// //             orders: ob,
// //             buy_token,
// //             sell_token,
// //             _optimal_price,
// //         }
// //     }

// //     pub fn target_price(&self) -> f64 {
// //         1.0 / self.cfmm.price
// //     }

// //     pub fn limit_price(&self) -> f64 {
// //         if self._optimal_price < self.cfmm.price {
// //             self._optimal_price * 1.1
// //         } else {
// //             self._optimal_price / 1.1
// //         }
// //     }

// //     pub fn profit(&self, order: &Order) -> f64 {
// //         let obtained = order.amount_out;
// //         let result = match order.order_type {
// //             OrderType::Buy => self.cfmm.sell(obtained, true),
// //             OrderType::Sell => self.cfmm.buy(obtained, true),
// //         };
// //         result - order.amount_filled
// //     }
// // }

// // pub struct CFMMProfitSolver {
// //     inner: CFMMSolver,
// // }

// // impl CFMMProfitSolver {
// //     pub fn f_maximize(&self, order: &Order) -> f64 {
// //         self.inner.profit(order)
// //     }
// // }

// // pub struct CFMMVolumeSolver {
// //     inner: CFMMSolver,
// // }


// // pub struct Solver {
// //     orders: OrderList,
// //     target_price: f64,
// //     buy_token: f64,
// //     sell_token: f64,
// //     order: Order,
// // }

// // impl Solver {
// //     pub fn new(orders: OrderList, target_price: f64, buy_token: f64, sell_token: f64) -> Self {
// //         Self {
// //             orders,
// //             target_price,
// //             buy_token,
// //             sell_token,
// //             order: Order::new(0.0, 0.0, OrderType::Buy, "fake-order".to_string()),
// //         }
// //     }

// //     pub fn limit_price(&self) -> f64 {
// //         self.target_price
// //     }

// //     fn f_maximize(&self, order: &Order) -> f64 {
// //         match order.order_type {
// //             OrderType::Buy => {
// //                 self.buy_token
// //                     - order.amount_filled
// //                     + (self.sell_token + order.amount_out) * self.target_price
// //             }
// //             OrderType::Sell => {
// //                 (self.buy_token + order.amount_out) / self.target_price + self.sell_token - order.amount_filled
// //             }
// //         }
// //     }

// //     pub fn solve(&mut self, num_orders: usize) -> Result<Solution, &'static str> {
// //         let original_price = self.orders.compute_optimal_price();
// //         let is_buy = original_price > self.target_price;
// //         let original_token_amount = if is_buy { self.buy_token } else { self.sell_token };

// //         let orders: Vec<Order> = (0..=num_orders)
// //             .map(|i| self.order_for(i as f64 * original_token_amount / num_orders as f64, is_buy))
// //             .collect();

// //         let mut max_value = 0.0;
// //         let mut max_solution: Option<Solution> = None;

// //         for order in &orders {
// //             let solution = self.match_ob_with_order(order)?;
// //             let introduced_orders = solution.orders.id(&order.id);

// //             if let Some(introduced_order) = introduced_orders.first() {
// //                 let f_value = self.f_maximize(introduced_order);
// //                 if max_value < f_value {
// //                     max_value = f_value;
// //                     max_solution = Some(solution);
// //                     self.order = introduced_order.clone();
// //                 }
// //             }
// //         }

// //         max_solution.ok_or("No max solution found")
// //     }

// //     fn match_ob_with_order(&self, order: &Order) -> Result<Solution, &'static str> {
// //         let mut orderbook = self.orders.clone();
// //         orderbook.value.push(order.clone());
// //         orderbook.value.sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap());

// //         let optimal_price = orderbook.compute_optimal_price();
// //         orderbook.match_orders(optimal_price)
// //     }

// //     fn order_for(&self, amount: f64, is_buy: bool) -> Order {
// //         if is_buy {
// //             Order::new(amount, self.limit_price(), OrderType::Buy, "solver".to_string())
// //         } else {
// //             Order::new(amount, self.limit_price(), OrderType::Sell, "solver".to_string())
// //         }
// //     }
// // }
