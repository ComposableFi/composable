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