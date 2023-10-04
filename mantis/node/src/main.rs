use clap::Parser;

#[derive(clap::Parser)]
struct MantisArgs {
    /// the node hosting order contract
    #[arg(long)]
    order_rpc : String,
    /// address of the order contract on `order_rpc` chain
    #[arg(long)]
    order_contract : String,
}

fn main() {
    let args : MantisArgs = MantisArgs::parse();
    
    println!("Hello, world!");

}
