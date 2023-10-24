mod orderbook;
mod prelude;
mod solution;
mod types;
mod solver;

fn main() {
    let order_ = types::Order::new_random(1.0, 0.05, (50, 150), 42);
    // to be continued...
}
