#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Program {
    #[prost(message, optional, tag="1")]
    pub instructions: ::core::option::Option<Instructions>,
}
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
        Call(super::Call),
        #[prost(message, tag="3")]
        Spawn(super::Spawn),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(message, optional, tag="1")]
    pub destination: ::core::option::Option<Account>,
    /// (Asset, Amount)
    #[prost(btree_map="uint32, message", tag="2")]
    pub assets: ::prost::alloc::collections::BTreeMap<u32, U128>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Call {
    #[prost(bytes="vec", tag="1")]
    pub encoded: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spawn {
    #[prost(uint32, tag="1")]
    pub network: u32,
    /// (Asset, Amount)
    #[prost(btree_map="uint32, message", tag="2")]
    pub assets: ::prost::alloc::collections::BTreeMap<u32, U128>,
    #[prost(message, repeated, tag="3")]
    pub program: ::prost::alloc::vec::Vec<Instruction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(bytes="vec", tag="1")]
    pub encoded: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct U128 {
    #[prost(bytes="vec", tag="1")]
    pub encoded: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct U256 {
    #[prost(bytes="vec", tag="1")]
    pub encoded: ::prost::alloc::vec::Vec<u8>,
}
