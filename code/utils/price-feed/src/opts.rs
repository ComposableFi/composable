use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Composable")]
pub struct Opts {
	/// Host address of the pythd server.
	#[clap(short, long, default_value = "http://127.0.0.1:8910")]
	pub pythd_host: String,

	/// Host address of the composable node.
	#[clap(long, default_value = "ws://127.0.0.1:9988")]
	pub composable_node: String,

	/// Listening address for the frontend.
	#[clap(short, long, default_value = "127.0.0.1:3001")]
	pub listening_address: String,

	/// Asset to be used as quote for pricing.
	#[clap(short, long, default_value = "USDT")]
	pub quote_asset: String,

	/// Price will be normalized to this exponent.
	#[clap(short, long, default_value = "12")]
	pub expected_exponent: i32,

	/// Duration, in seconds, before a price is evicted from the cache.
	#[clap(short, long, default_value = "10")]
	pub cache_duration: u32,
}
