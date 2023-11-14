//! Basic types with simple checks and domain, no heavy math or solving.
use crate::prelude::*;
use derive_more::{Display, From};
use strum_macros::AsRefStr;

pub type Amount = Decimal;

#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Default, From)]
#[repr(transparent)]
pub struct BuyToken(pub Amount);
#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Default, From)]
#[repr(transparent)]
pub struct SellToken(pub Amount);

#[derive(Debug, Clone, Copy, From, PartialEq, PartialOrd, Default, Display)]
#[repr(transparent)]
pub struct Price(pub Amount);

impl Price {
    pub fn new_float(amount: f64) -> Self {
        Price(Decimal::from_f64_retain(amount).unwrap())
    }
}

/// this is buy sell in terms of token1/token2 or A/B. just 2 sides of the orderbook.
/// not Buy and Sell orders which differ in limit definition(in limit vs out limit).
#[derive(Debug, PartialEq, Eq, Clone, Copy, AsRefStr, Display)]
pub enum OrderType {
    Buy,
    Sell,
}

impl OrderType {
    fn is_acceptable_price(&self, price: Amount, limit_price: Amount) -> bool {
        match self {
            OrderType::Sell => price >= limit_price,
            OrderType::Buy => price <= limit_price,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, AsRefStr, Display)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
}

#[derive(Debug, PartialEq, Eq, AsRefStr, Display)]
pub enum OrderBookStatus {
    Pending,
    Matched,
}

#[derive(Debug, Clone)]
pub struct Order<Id> {
    pub amount_in: Amount,
    pub filled_price: Amount,
    pub order_type: OrderType,
    pub amount_out: Amount,
    pub amount_filled: Amount,
    pub status: OrderStatus,
    pub id: Id,
    pub limit_price: Price,
}

impl<Id: Copy + PartialEq> Order<Id> {
    pub fn print(&self) {
        println!(
            "[{}]-{}- Limit Price: {:0.3}, In: {}, Filled: {}, Filled price: {}, Out: {}",
            self.order_type,
            self.status,
            self.limit_price,
            self.amount_in,
            self.amount_filled,
            self.filled_price,
            self.amount_out
        );
    }
    pub fn new(amount_in: Amount, limit_price: Price, order_type: OrderType, id: Id) -> Self {
        Order {
            amount_in,
            filled_price: dec!(0.0),
            order_type,
            amount_out: dec!(0.0),
            amount_filled: dec!(0.0),
            status: OrderStatus::Pending,
            id,
            limit_price,
        }
    }

    pub fn filled_price(&self) -> Amount {
        match self.order_type {
            OrderType::Buy => dec!(1.0) / self.filled_price,
            _ => self.filled_price,
        }
    }

    pub fn to_be_filled(&self) -> Amount {
        if self.status == OrderStatus::PartiallyFilled {
            self.amount_in - self.amount_out / self.filled_price()
        } else {
            dec!(0.0)
        }
    }

    pub fn is_acceptable_price(&self, price: Price) -> bool {
        self.order_type
            .is_acceptable_price(price.0, self.limit_price.0)
    }

    pub fn token1_at_price(&self, price: Amount) -> Amount {
        if self.order_type == OrderType::Sell {
            self.amount_in * price
        } else {
            self.amount_in
        }
    }

    pub fn fill(&mut self, volume: Amount, price: Price) {
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
            self.status = OrderStatus::PartiallyFilled;
        } else {
            self.status = OrderStatus::Filled;
        }

        self.check_constraints();
    }

    fn check_constraints(&self) {
        match self.status {
            OrderStatus::Filled => {
                assert_eq!(
                    self.amount_out,
                    self.amount_in * self.filled_price(),
                    "Constraint check failed"
                );
            }
            OrderStatus::PartiallyFilled => {
                assert!(
                    self.amount_out < self.amount_in * self.filled_price(),
                    "Constraint check failed"
                );
            }
            _ => {
                if self.status != OrderStatus::Pending {
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

impl<Id: Copy + PartialEq> Order<Id> {
    pub fn new_random(mean: f64, std: f64, volume_range: (u64, u64), id: Id) -> Self {
        let amount_in = rand::thread_rng().gen_range(volume_range.0..volume_range.1 + 1) as f64;
        let normal = rand_distr::Normal::new(mean, std).unwrap();
        let limit_price = normal.sample(&mut rand::thread_rng());

        let order_type = if rand::thread_rng().gen::<bool>() {
            OrderType::Buy
        } else {
            OrderType::Sell
        };

        Order::new(
            Decimal::from_f64_retain(amount_in).unwrap(),
            Price(Decimal::from_f64_retain(limit_price).unwrap()),
            order_type,
            id,
        )
    }
}
