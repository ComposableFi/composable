#[derive(clap::Parser)]
pub struct MantisArgs {
    /// the node hosting order contract
    #[arg(long)]
    pub centauri: String,
    /// the node with pools
    #[arg(long)]
    pub osmosis: String,

    /// the node with pools
    #[arg(long)]
    pub neutron: String,

    /// CVM contract on Centauri
    #[arg(long)]
    pub cvm_contract: String,
    
    /// Order contract on Centauri
    #[arg(long)]
    pub order_contract: String,

    /// tokens to send to order contract as problem
    #[arg(long)]
    pub simulate: Option<String>,

    /// wallet to use
    #[arg(long)]
    pub wallet: String,
}

impl MantisArgs {
    pub fn parsed() -> Self {
        use clap::Parser;
        let args = Self::parse();
        args
    }
}