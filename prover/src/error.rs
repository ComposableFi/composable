#[derive(sp_std::fmt::Debug, derive_more::From)]
pub enum Error {
    /// subxt error
    Subxt(subxt::BasicError),
    /// subxt rpc error
    SubxtRRpc(subxt::rpc::RpcError),
    /// Trie error
    TrieProof(Box<sp_trie::TrieError<sp_trie::LayoutV0<sp_runtime::traits::BlakeTwo256>>>),
    /// Custom
    Custom(String),
    /// Codec error
    Codec(codec::Error),
}
