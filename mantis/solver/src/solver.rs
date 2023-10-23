
// #[derive(Clone, Debug)]
// struct OrderList {
//     value: Vec<Order>,
// }

// impl OrderList {
//     fn apply_filter<P>(&self, expr: P) -> Self
//     where
//         P: FnMut(&Order) -> bool,
//     {
//         OrderList {
//             value: self.value.iter().cloned().filter(expr).collect(),
//         }
//     }

//     fn buy(&self) -> Self {
//         self.apply_filter(|order| order.order_type == OrderType::BUY)
//     }

//     fn sell(&self) -> Self {
//         self.apply_filter(|order| order.order_type == OrderType::SELL)
//     }

//     fn pending(&self) -> Self {
//         self.apply_filter(|order| order.status == OrderStatus::PENDING)
//     }

//     fn filled(&self) -> Self {
//         self.apply_filter(|order| order.status != OrderStatus::PENDING)
//     }

//     fn is_acceptable_price(&self, price: Price) -> Self {
//         self.apply_filter(|order| order.is_acceptable_price(price))
//     }

//     fn amount_in(&self) -> BuyToken {
//         BuyToken(self.value.iter().map(|order| order.amount_in).sum())
//     }

//     fn amount_out(&self) -> SellToken {
//         SellToken(self.value.iter().map(|order| order.amount_out).sum())
//     }

//     fn amount_filled(&self) -> BuyToken {
//         BuyToken(self.value.iter().map(|order| order.amount_filled).sum())
//     }

//     fn token1_sum(&self, price: Price) -> BuyToken {
//         BuyToken(
//             self.value
//                 .iter()
//                 .map(|order| order.token1_at_price(price.0))
//                 .sum(),
//         )
//     }

//     fn id(&self, id: u128) -> Self {
//         self.apply_filter(|order| order.id == id)
//     }

//     fn all(&self) -> &Vec<Order> {
//         &self.value
//     }

//     fn clone(&self) -> Self {
//         OrderList {
//             value: self.value.iter().cloned().collect(),
//         }
//     }


//     fn compute_optimal_price(&self, num_range: i32) -> Price {
//         let mut optimal_price = Price(Decimal::new(-1, 0));
//         let mut max_volume = BuyToken(Decimal::new(-1, 0));
//         let min_price = self
//             .value
//             .iter()
//             .min_by(|a, b| {
//                 a.limit_price
//                     .partial_cmp(&b.limit_price)
//                     .unwrap_or(Ordering::Equal)
//             })
//             .map(|order| order.limit_price)
//             .unwrap_or(Decimal::new(0, 0));
//         let max_price = self
//             .value
//             .iter()
//             .max_by(|a, b| {
//                 a.limit_price
//                     .partial_cmp(&b.limit_price)
//                     .unwrap_or(Ordering::Equal)
//             })
//             .map(|order| order.limit_price)
//             .unwrap_or(Decimal::new(0, 0));

//         for i in 0..=num_range {
//             let price = min_price
//                 + (max_price - min_price) * Decimal::new(i as i64, 0)
//                     / Decimal::new(num_range as i64, 0);
//             let volume = self.volume_by_price(Price(price));
//             if volume.0 > max_volume.0 {
//                 optimal_price.0 = price;
//                 max_volume = volume;
//             }
//         }

//         optimal_price
//     }

//     fn volume_by_price(&self, price: Price) -> BuyToken {
//         let matched = self.is_acceptable_price(price);
//         min(
//             matched.buy().token1_sum(price),
//             matched.sell().token1_sum(price),
//         )
//     }

//     fn _resolve_predominant(
//         &mut self,
//         predominant_orders: &mut OrderList,
//         other_orders: &mut OrderList,
//         price: Price,
//     ) {
//         let mut filled = BuyToken(Decimal::new(0, 0));
//         for order in other_orders.value.iter_mut() {
//             order.fill(order.amount_in, price);
//         }
//         let other_volume = other_orders.amount_out();
//         for order in predominant_orders.value.iter_mut() {
//             if filled.0 + order.amount_in > other_volume.0 {
//                 order.fill(other_volume.0 - filled.0, price);
//                 break;
//             }
//             order.fill(order.amount_in, price);
//             filled.0 += order.amount_in;
//         }
//     }
// }


// #[derive(Clone, Debug)]
// struct Solution {
//     orders: OrderList,
//     matched_price: f64,
//     buy_volume: f64,
//     sell_volume: f64,
// }

// impl Solution {
//     pub fn new(orders: Vec<Order>) -> Self {
//         let mut order_list = OrderList {
//             value: orders.clone(),
//         };
//         order_list.value.sort_by(|a, b| a.limit_price.partial_cmp(&b.limit_price).unwrap_or(std::cmp::Ordering::Equal));

//         let matched_price = if !order_list.is_empty() {
//             order_list[0].filled_price
//         } else {
//             0.0
//         };

//         let buy_volume = order_list.sell().amount_out();
//         let sell_volume = order_list.buy().amount_out();

//         Self {
//             orders: order_list,
//             matched_price,
//             buy_volume,
//             sell_volume,
//         }
//     }

//     pub fn sell_orders(&self) -> &OrderList {
//         &self.orders.sell()
//     }

//     pub fn buy_orders(&self) -> &OrderList {
//         &self.orders.buy()
//     }

//     pub fn check_constraints(&self) {
//         const EPSILON: f64 = 1e-20;
//         if !((self.buy_volume - self.orders.buy().amount_filled()).abs() < EPSILON ||
//              (self.sell_volume - self.orders.sell().amount_filled()).abs() < EPSILON) {
//             panic!(
//                 "Error buy_volume: {} Buy amount filled: {} sell_volume: {} Buy amount filled: {}",
//                 self.buy_volume,
//                 self.orders.buy().amount_filled(),
//                 self.sell_volume,
//                 self.orders.sell().amount_filled()
//             );
//         }
//     }

//     pub fn print(&self) {
//         println!("{} Start Solution {}", "#".repeat(20), "#".repeat(20));
//         self.orders.print();
//         println!("\x1b[1mMatched Price {:.4} \tSell volume {:.4}\tBuy volume {:.4}\x1b[0m", 
//                  self.matched_price, self.sell_volume, self.buy_volume);
//         println!("{} End Solution {}", "#".repeat(20), "#".repeat(20));
//     }


//     // fn match_orders(&mut self, price: Price) -> Solution {
//     //     let mut orders = self.clone();
//     //     orders.value.sort_by(|a, b| {
//     //         a.limit_price
//     //             .partial_cmp(&b.limit_price)
//     //             .unwrap_or(Ordering::Equal)
//     //     });

//     //     let matched = orders.is_acceptable_price(price);
//     //     let buy_orders = matched.buy();
//     //     let sell_orders = matched.sell();

//     //     let buy_volume = buy_orders.token1_sum(price);
//     //     let sell_volume = sell_orders.token1_sum(price);

//     //     let is_buy_predominant = buy_volume > sell_volume;

//     //     if is_buy_predominant {
//     //         self._resolve_predominant(&mut buy_orders.clone(), &mut sell_orders.clone(), price);
//     //     } else {
//     //         self._resolve_predominant(&mut sell_orders.clone(), &mut buy_orders.clone(), price);
//     //     }

//     //     let mut solution = Solution {
//     //         orders: matched.filled().clone(),
//     //         matched_price: Price(Decimal::new(0, 0)),
//     //         buy_volume: BuyToken(Decimal::new(0, 0)),
//     //         sell_volume: SellToken(Decimal::new(0, 0)),
//     //     };

//     //     solution.check_constraints();
//     //     solution
//     // }

//     // Assuming Order has a random method. 
//     // pub fn random(num_orders: usize) -> Self {
//     //     Self::new((0..num_orders).map(|_| Order::random()).collect())
//     // }
// }

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
