use clap::Parser;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// WS url to node
   #[arg(long)]
   client : String,

   /// Link to CSV file with schedule
   #[arg(long)]
   vesting_schedule: String,
}