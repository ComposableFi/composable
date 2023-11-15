//! Solving just order book without cross chain routing.
use crate::prelude::*;
use crate::solver::types::*;

#[derive(Clone, Debug)]
pub struct OrderList<Id> {
    pub value: Vec<Order<Id>>,
}

impl<Id: Copy + PartialEq + Debug> OrderList<Id> {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    fn apply_filter<P>(&self, expr: P) -> Self
    where
        P: FnMut(&Order<Id>) -> bool,
    {
        OrderList {
            value: self.value.iter().cloned().filter(expr).collect(),
        }
    }

    pub fn buy(&self) -> Self {
        self.apply_filter(|order| order.order_type == OrderType::Buy)
    }

    pub fn sell(&self) -> Self {
        self.apply_filter(|order| order.order_type == OrderType::Sell)
    }

    pub fn pending(&self) -> Self {
        self.apply_filter(|order| order.status == OrderStatus::Pending)
    }

    pub fn filled(&self) -> Self {
        self.apply_filter(|order| order.status != OrderStatus::Pending)
    }

    pub fn is_acceptable_price(&self, price: Price) -> Self {
        self.apply_filter(|order| order.is_acceptable_price(price))
    }

    pub fn amount_in(&self) -> BuyToken {
        BuyToken(self.value.iter().map(|order| order.amount_in).sum())
    }

    pub fn amount_out(&self) -> SellToken {
        SellToken(self.value.iter().map(|order| order.amount_out).sum())
    }

    pub fn amount_filled(&self) -> BuyToken {
        BuyToken(self.value.iter().map(|order| order.amount_filled).sum())
    }

    pub fn token1_sum(&self, price: Price) -> BuyToken {
        BuyToken(
            self.value
                .iter()
                .map(|order| order.token1_at_price(price.0))
                .sum(),
        )
    }

    pub fn id(&self, id: Id) -> Self {
        self.apply_filter(|order| order.id == id)
    }

    pub fn all(&self) -> &Vec<Order<Id>> {
        &self.value
    }

    /// Computes the optimal price that will maximize the transacted volume in batch auction.
    /// Finds the price in which $max(x*y)$ is satisfied according limit.
    /// `num_range` - amid min and max price, how many quantization point to consider.
    /// All price outliers are pruned to avoid computational attack.
    pub fn compute_optimal_price(&self, num_range: u16) -> Price {
        let mut optimal_price = Price(Decimal::new(-1, 0));
        let mut max_volume = BuyToken(Decimal::new(-1, 0));
        let (min_price, max_price) = match self.value.iter().map(|x| x.limit_price).minmax() {
            itertools::MinMaxResult::NoElements => <_>::default(),
            itertools::MinMaxResult::OneElement(x) => (x, x),
            itertools::MinMaxResult::MinMax(min, max) => (min, max),
        };
        for i in 0..=num_range {
            let price = min_price.0
                + (max_price.0 - min_price.0) * Decimal::new(i.into(), 0)
                    / Decimal::new(num_range.into(), 0);
            let volume = self.volume_by_price(Price(price));
            if volume.0 > max_volume.0 {
                optimal_price = Price(price);
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

    pub fn print(&self) {
        for order in self.buy().value.iter() {
            order.print();
        }
        println!("----------");
        for order in self.sell().value.iter() {
            order.print();
        }
        println!("----------");
    }

    pub fn resolve_predominant(
        predominant_orders: &mut OrderList<Id>,
        other_orders: &mut OrderList<Id>,
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
