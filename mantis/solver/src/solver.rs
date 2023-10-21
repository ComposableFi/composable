use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub type Amount = Decimal;

pub struct BuyToken(Amount);
pub struct SellToken(Amount);
pub struct Price(Amount);

#[derive(Debug, PartialEq, Eq)]
pub enum OrderType {
    BUY,
    SELL,
}

impl  OrderType {
    fn as_str(&self) -> &'static str {
        match self {
            OrderType::BUY => "Buy",
            OrderType::SELL => "Sell",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct Order {
    pub amount_in: Amount,
    pub filled_price: Amount,
    pub order_type: OrderType,
    pub amount_out: Amount,
    pub amount_filled: Amount,
    pub status: OrderStatus,
    pub id: u128,
    pub limit_price: Amount,
}

impl Order {
    fn new(
        amount_in: Amount,
        limit_price: Amount,
        order_type: OrderType,
        id: u128,
    ) -> Self {
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

    fn is_acceptable_price(&self, price: Amount) -> bool {
        self.order_type.is_acceptable_price(price, self.limit_price)
    }

    fn token1_at_price(&self, price: Amount) -> Amount {
        if self.order_type == OrderType::SELL {
            self.amount_in * price
        } else {
            self.amount_in
        }
    }

    fn fill(&mut self, volume: Amount, price: Amount) {
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

        self.filled_price = price;
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