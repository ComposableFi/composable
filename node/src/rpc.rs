//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use primitives::currency::CurrencyId;
use std::sync::Arc;

use assets_rpc::{Assets, AssetsApi};
use common::{AccountId, AccountIndex, Balance};
use crowdloan_rewards_rpc::{CrowdloanRewards, CrowdloanRewardsApi};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_runtime::traits::Block;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create<C, P, B>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
	B: Block,
	C: ProvideRuntimeApi<B>,
	C: HeaderBackend<B> + HeaderMetadata<B, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<B, AccountId, AccountIndex>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<B, Balance>,
	C::Api: assets_runtime_api::AssetsRuntimeApi<B, CurrencyId, AccountId, Balance>,
	C::Api: crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi<B, AccountId, Balance>,
	C::Api: BlockBuilder<B>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use substrate_frame_rpc_system::{FullSystem, SystemApi};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps { client, pool, deny_unsafe } = deps;

	io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe)));

	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));

	io.extend_with(AssetsApi::to_delegate(Assets::new(client.clone())));

	io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(client)));

	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

	io
}
