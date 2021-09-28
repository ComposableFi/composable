//! Price function for auction.
//! Linear, step-wise exponential, and continuous exponential, others, configured from MakerDao

use composable_traits::loans::{DurationSeconds, LiftedFixedBalance};



struct LinearDecrease{
	/// Seconds after auction start when the price reaches zero [seconds]
	total_duration : DurationSeconds,
}

trait AuctionTimeCurveModel {
	/// return current auction price
	fn price(initial_price: LiftedFixedBalance, duration_since_start: DurationSeconds) -> LiftedFixedBalance;
}
