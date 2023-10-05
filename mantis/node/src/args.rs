#[derive(clap::Parser)]
pub struct MantisArgs {
    /// the node hosting order contract
    #[arg(long)]
    pub order_rpc: String,
    /// address of the order contract on `order_rpc` chain
    #[arg(long)]
    pub order_contract: String,
}