//! Solving just order book without cross chain routing.

use crate::prelude::*;
use crate::types::*;

#[derive(Clone, Debug)]
struct OrderList<Id: Copy> {
    value: Vec<Order<Id>>,
}

impl<Id: Copy> OrderList<Id> {
    fn apply_filter<P>(&self, expr: P) -> Self
    where
        P: FnMut(&Order) -> bool,
    {
        OrderList {
            value: self.value.iter().cloned().filter(expr).collect(),
        }
    }

    fn buy(&self) -> Self {
        self.apply_filter(|order| order.order_type == OrderType::BUY)
    }

    fn sell(&self) -> Self {
        self.apply_filter(|order| order.order_type == OrderType::SELL)
    }

    fn pending(&self) -> Self {
        self.apply_filter(|order| order.status == OrderStatus::PENDING)
    }

    fn filled(&self) -> Self {
        self.apply_filter(|order| order.status != OrderStatus::PENDING)
    }

    fn is_acceptable_price(&self, price: Price) -> Self {
        self.apply_filter(|order| order.is_acceptable_price(price))
    }

    fn amount_in(&self) -> BuyToken {
        BuyToken(self.value.iter().map(|order| order.amount_in).sum())
    }

    fn amount_out(&self) -> SellToken {
        SellToken(self.value.iter().map(|order| order.amount_out).sum())
    }

    fn amount_filled(&self) -> BuyToken {
        BuyToken(self.value.iter().map(|order| order.amount_filled).sum())
    }

    fn token1_sum(&self, price: Price) -> BuyToken {
        BuyToken(
            self.value
                .iter()
                .map(|order| order.token1_at_price(price.0))
                .sum(),
        )
    }

    fn id(&self, id: u128) -> Self {
        self.apply_filter(|order| order.id == id)
    }

    fn all(&self) -> &Vec<Order> {
        &self.value
    }

    fn clone(&self) -> Self {
        OrderList {
            value: self.value.iter().cloned().collect(),
        }
    }


    fn compute_optimal_price(&self, num_range: i32) -> Price {
        let mut optimal_price = Price(Decimal::new(-1, 0));
        let mut max_volume = BuyToken(Decimal::new(-1, 0));
        let min_price = self
            .value
            .iter()
            .min_by(|a, b| {
                a.limit_price
                    .partial_cmp(&b.limit_price)
                    .unwrap_or(Ordering::Equal)
            })
            .map(|order| order.limit_price)
            .unwrap_or(Decimal::new(0, 0));
        let max_price = self
            .value
            .iter()
            .max_by(|a, b| {
                a.limit_price
                    .partial_cmp(&b.limit_price)
                    .unwrap_or(Ordering::Equal)
            })
            .map(|order| order.limit_price)
            .unwrap_or(Decimal::new(0, 0));

        for i in 0..=num_range {
            let price = min_price
                + (max_price - min_price) * Decimal::new(i as i64, 0)
                    / Decimal::new(num_range as i64, 0);
            let volume = self.volume_by_price(Price(price));
            if volume.0 > max_volume.0 {
                optimal_price.0 = price;
                max_volume = volume;
            }
        }

        optimal_price
    }

    fn volume_by_price(&self, price: Price) -> BuyToken {
        let matched = self.is_acceptable_price(price);
        min(
            matched.buy().token1_sum(price),
            matched.sell().token1_sum(price),
        )
    }

    fn _resolve_predominant(
        &mut self,
        predominant_orders: &mut OrderList,
        other_orders: &mut OrderList,
        price: Price,
    ) {
        let mut filled = BuyToken(Decimal::new(0, 0));
        for order in other_orders.value.iter_mut() {
            order.fill(order.amount_in, price);
        }
        let other_volume = other_orders.amount_out();
        for order in predominant_orders.value.iter_mut() {
            if filled.0 + order.amount_in > other_volume.0 {
                order.fill(other_volume.0 - filled.0, price);
                break;
            }
            order.fill(order.amount_in, price);
            filled.0 += order.amount_in;
        }
    }
}
