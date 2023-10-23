//! Basic types with simple checks and domain, no heavy math or solving.
use crate::prelude::*;

pub type Amount = Decimal;

#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Default)]
pub struct BuyToken(pub Amount);
#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Default)]
pub struct SellToken(pub  Amount);

#[derive(Debug, Clone, Copy)]
pub struct Price(pub  Amount);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrderType {
    BUY,
    SELL,
}

impl OrderType {
    fn as_str(&self) -> &'static str {
        match self {
            OrderType::BUY => "Buy",
            OrderType::SELL => "Sell",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrderStatus {
    PENDING,
    PARTIALLY_FILLED,
    FILLED,
}

impl OrderStatus {
    fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::PENDING => "Pending",
            OrderStatus::PARTIALLY_FILLED => "Partial",
            OrderStatus::FILLED => "Filled",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderBookStatus {
    PENDING,
    MATCHED,
}

impl OrderBookStatus {
    fn as_str(&self) -> &'static str {
        match self {
            OrderBookStatus::PENDING => "Pending",
            OrderBookStatus::MATCHED => "Matched",
        }
    }
}

impl OrderType {
    fn is_acceptable_price(&self, price: Amount, limit_price: Amount) -> bool {
        match self {
            OrderType::SELL => price >= limit_price,
            OrderType::BUY => price <= limit_price,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order<Id : Copy + PartialEq> {
    pub amount_in: Amount,
    pub filled_price: Amount,
    pub order_type: OrderType,
    pub amount_out: Amount,
    pub amount_filled: Amount,
    pub status: OrderStatus,
    pub id: Id,
    pub limit_price: Amount,
}

impl<Id: Copy + PartialEq> Order<Id> {
    fn new(amount_in: Amount, limit_price: Amount, order_type: OrderType, id: Id) -> Self {
        Order {
            amount_in,
            filled_price: dec!(0.0),
            order_type,
            amount_out: dec!(0.0),
            amount_filled: dec!(0.0),
            status: OrderStatus::PENDING,
            id,
            limit_price,
        }
    }

    fn filled_price(&self) -> Amount {
        match self.order_type {
            OrderType::BUY => dec!(1.0) / self.filled_price,
            _ => self.filled_price,
        }
    }

    fn to_be_filled(&self) -> Amount {
        if self.status == OrderStatus::PARTIALLY_FILLED {
            self.amount_in - self.amount_out / self.filled_price()
        } else {
            dec!(0.0)
        }
    }

    fn is_acceptable_price(&self, price: Price) -> bool {
        self.order_type
            .is_acceptable_price(price.0, self.limit_price)
    }

    fn token1_at_price(&self, price: Amount) -> Amount {
        if self.order_type == OrderType::SELL {
            self.amount_in * price
        } else {
            self.amount_in
        }
    }

    fn fill(&mut self, volume: Amount, price: Price) {
        if volume == dec!(0.0) {
            return;
        } else if volume < dec!(0.0) {
            panic!("Negative volume {}", volume);
        }

        if volume > self.amount_in {
            panic!(
                "[{:?}] Volume trying to fill: {} Amount in the order: {} diff: {}",
                self.order_type,
                volume,
                self.amount_in,
                self.amount_in - volume
            );
        }

        self.filled_price = price.0;
        self.amount_out = volume * self.filled_price();
        self.amount_filled = volume;

        if volume < self.amount_in {
            self.status = OrderStatus::PARTIALLY_FILLED;
        } else {
            self.status = OrderStatus::FILLED;
        }

        self.check_constraints();
    }

    fn check_constraints(&self) {
        match self.status {
            OrderStatus::FILLED => {
                assert_eq!(
                    self.amount_out,
                    self.amount_in * self.filled_price(),
                    "Constraint check failed"
                );
            }
            OrderStatus::PARTIALLY_FILLED => {
                assert!(
                    self.amount_out < self.amount_in * self.filled_price(),
                    "Constraint check failed"
                );
            }
            _ => {
                if self.status != OrderStatus::PENDING {
                    assert_eq!(
                        self.amount_out,
                        self.amount_filled * self.filled_price(),
                        "Constraint check failed"
                    );
                } else {
                    assert_eq!(self.amount_out, dec!(0.0), "Constraint check failed");
                }
            }
        }
    }
}

impl<Id : Copy + PartialEq> Order<Id> {

    pub fn new_random(mean: f64, std: f64, volume_range: (u64, u64), id: Id) -> Self {
        let amount_in = rand::thread_rng().gen_range(volume_range.0..volume_range.1 + 1) as f64;
        let normal = rand_distr::Normal::new(mean, std).unwrap();
        let limit_price = normal.sample(&mut rand::thread_rng());

        let order_type = if rand::thread_rng().gen::<bool>() {
            OrderType::BUY
        } else {
            OrderType::SELL
        };

        Order::new(
            Decimal::from_f64_retain(amount_in).unwrap(),
            Decimal::from_f64_retain(limit_price).unwrap(),
            order_type,
            id,
        )
    }
}
