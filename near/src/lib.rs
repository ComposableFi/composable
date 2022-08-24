use ibc::core::ics04_channel::packet::Packet;
use jsonrpsee::async_client::Client as RpcClient;
use near_primitives::types::AccountId;

mod error;
pub mod provider;

/// Implements the [`crate::Chain`] trait for NEAR.
pub struct Client {
	/// Near rpc client
	pub rpc_client: RpcClient,
	/// Core contract id
	pub contract_id: AccountId,
	/// Near's latest finalized height
	pub latest_near_height: Option<u32>,
	/// Commitment prefix
	pub commitment_prefix: Vec<u8>,
	/// Sent packet sequence cache
	pub packet_cache: Vec<Packet>,
}
