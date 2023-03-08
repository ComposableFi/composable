use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// WS url to node
   #[arg(long)]
   pub client : String,

   /// Link to CSV file with schedule
   #[arg(long)]
   pub schedule: String,

   /// Private sudo key
   #[arg(long)]
   pub key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Record {
    address: String,
    /// unix timestamp
    window_moment_start: u64,
    /// unix time
    window_moment_period: u64,
    period_count: u16,
    /// amount
    per_period: u64,
}