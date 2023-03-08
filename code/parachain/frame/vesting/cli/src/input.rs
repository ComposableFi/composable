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
    pub address: String,
    /// unix timestamp
    pub window_moment_start: u64,
    /// unix time
    pub window_moment_period: u64,
    pub period_count: u16,
    /// amount
    pub per_period: u64,
}