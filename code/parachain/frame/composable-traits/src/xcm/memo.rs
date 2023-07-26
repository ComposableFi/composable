use crate::prelude::*;

#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	Hash,
	codec::Encode,
	codec::Decode,
	scale_info::TypeInfo,
	Ord,
	PartialOrd,
	MaxEncodedLen,
	Debug,
)]
pub enum ChainHop {
	// SubstrateIbc is used to indicate whether the chain is a substrate chain
	/// if it is a substrate chain, it will use substrate address to send packet
	SubstrateIbc,
	/// CosmosIbc is used to indicate 
	/// whether the chain address should be converted with bech32 + chain name
	CosmosIbc,
	/// SubstrateXcm is used to indicate whether the chain is a substrate chain
	/// if it is none, it means send to relay-chain(polkadot/kusama/rococo)
	Xcm
}

#[derive(
	Copy,
	Clone,
	PartialEq,
	Eq,
	Hash,
	codec::Encode,
	codec::Decode,
	scale_info::TypeInfo,
	Ord,
	PartialOrd,
	MaxEncodedLen,
	Debug,
)]
pub struct ChainInfo {
	/// chain_id is used to indicate the chain id of chain
	pub chain_id: u32,
	/// Order is used to sort chains to compose routes in correct order
	pub order: u8,
	/// channel_id is used to indicate the channel id of chain
	pub channel_id: u64,        
	/// timestamp is used to indicate the timestamp of packet
	pub timestamp: Option<u64>,
	/// height is used to indicate the height of packet
	pub height: Option<u64>,
	/// retries is used to indicate the retries of packet
	pub retries: Option<u8>,    
	/// timeout is used to indicate the timeout of packet (seconds)
	pub timeout: Option<u64>,   

	/// chain_hop is used to indicate the chain hop of chain
	pub chain_hop: ChainHop,
	/// para_id is used to indicate the para id of chain
	/// if para id is none, it means send to relay-chain
	pub para_id: Option<u32>,
}
