#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instructions {
    #[prost(message, repeated, tag="1")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(oneof="instruction::Instruction", tags="1, 2, 3")]
    pub instruction: ::core::option::Option<instruction::Instruction>,
}
/// Nested message and enum types in `Instruction`.
pub mod instruction {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Instruction {
        #[prost(message, tag="1")]
        Transfer(super::Transfer),
        #[prost(message, tag="2")]
        Bridge(super::Bridge),
        #[prost(message, tag="3")]
        Call(super::Call),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(message, optional, tag="1")]
    pub destination: ::core::option::Option<Account>,
    /// (Asset, Amount)
    #[prost(btree_map="uint32, uint64", tag="2")]
    pub assets: ::prost::alloc::collections::BTreeMap<u32, u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bridge {
    #[prost(uint32, tag="1")]
    pub network: u32,
    /// (Asset, Amount)
    #[prost(btree_map="uint32, uint64", tag="2")]
    pub assets: ::prost::alloc::collections::BTreeMap<u32, u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Call {
    #[prost(bytes="vec", tag="1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(oneof="account::Account", tags="1, 2")]
    pub account: ::core::option::Option<account::Account>,
}
/// Nested message and enum types in `Account`.
pub mod account {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Account {
        #[prost(string, tag="1")]
        Named(::prost::alloc::string::String),
        #[prost(bytes, tag="2")]
        Addressed(::prost::alloc::vec::Vec<u8>),
    }
}
