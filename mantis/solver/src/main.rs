
mod solver;
mod prelude;
mod types;
mod orderbook;

fn main() {
    let order_ = types::Order::new_random(1.0, 0.05, (50, 150), 42);
}
