use common::{AccountId, AccountIndex, Balance, Index, OpaqueBlock, PoolId};
use cumulus_primitives_core::CollectCollationInfo;
use pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi;
use sp_api::{ApiExt, Metadata, StateBackend};
use sp_block_builder::BlockBuilder;
use sp_consensus_aura::{sr25519, AuraApi};
use sp_offchain::OffchainWorkerApi;
use sp_runtime::traits::BlakeTwo256;
use sp_session::SessionKeys;
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use substrate_frame_rpc_system::AccountNonceApi;

/// Consider this a trait alias.
pub trait BaseHostRuntimeApis:
	TaggedTransactionQueue<OpaqueBlock>
	+ ApiExt<OpaqueBlock>
	+ BlockBuilder<OpaqueBlock>
	+ AccountNonceApi<OpaqueBlock, AccountId, Index>
	+ Metadata<OpaqueBlock>
	+ AuraApi<OpaqueBlock, sr25519::AuthorityId>
	+ OffchainWorkerApi<OpaqueBlock>
	+ SessionKeys<OpaqueBlock>
	+ CollectCollationInfo<OpaqueBlock>
	+ TransactionPaymentRuntimeApi<OpaqueBlock, Balance>
where
	<Self as ApiExt<OpaqueBlock>>::StateBackend: StateBackend<BlakeTwo256>,
{
}

impl<Api> BaseHostRuntimeApis for Api
where
	Api: TaggedTransactionQueue<OpaqueBlock>
		+ ApiExt<OpaqueBlock>
		+ BlockBuilder<OpaqueBlock>
		+ AccountNonceApi<OpaqueBlock, AccountId, Index>
		+ Metadata<OpaqueBlock>
		+ AuraApi<OpaqueBlock, sr25519::AuthorityId>
		+ OffchainWorkerApi<OpaqueBlock>
		+ SessionKeys<OpaqueBlock>
		+ CollectCollationInfo<OpaqueBlock>
		+ TransactionPaymentRuntimeApi<OpaqueBlock, Balance>,
	<Self as ApiExt<OpaqueBlock>>::StateBackend: StateBackend<BlakeTwo256>,
{
}

// pub trait ConstructRuntimeApis<RuntimeApi, Executor>
// where
// 	// Block: BlockT,
// 	RuntimeApi:
// 		ConstructRuntimeApi<OpaqueBlock, FullClient<RuntimeApi, Executor>> + Send + Sync + 'static,
// 	RuntimeApi::RuntimeApi:
// 		BaseHostRuntimeApis<StateBackend = StateBackendFor<FullBackend, OpaqueBlock>>,
// 	StateBackendFor<FullBackend, OpaqueBlock>: StateBackend<BlakeTwo256>,
// 	Executor: NativeExecutionDispatch + 'static,
// 	FullClient<RuntimeApi, Executor>: ProvideRuntimeApi<OpaqueBlock>
// 		+ HeaderBackend<OpaqueBlock>
// 		+ HeaderMetadata<OpaqueBlock, Error = sp_blockchain::Error>
// 		+ 'static
// 		+ Send
// 		+ Sync
// 		+ Sized,
// 	<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api:
// BaseHostRuntimeApis<StateBackend = StateBackendFor<FullBackend, OpaqueBlock>>
// 		+ ExtendWithAssetsApi<RuntimeApi, Executor>,
// {
// }

#[cfg(feature = "dali")]
pub mod dali {
	use assets_rpc::{Assets, AssetsApi};
	use common::OpaqueBlock;
	use polkadot_service::NativeExecutionDispatch;
	use sc_transaction_pool::FullPool;

	use crate::{client::FullClient, rpc::FullDeps, runtime::assets::ExtendWithAssetsApi};

	impl<Executor> ExtendWithAssetsApi<dali_runtime::RuntimeApi, Executor>
		for dali_runtime::RuntimeApiImpl<OpaqueBlock, FullClient<dali_runtime::RuntimeApi, Executor>>
	where
		Executor: NativeExecutionDispatch + 'static,
	{
		fn extend_with_assets_api(
			io: &mut jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>,
			deps: FullDeps<
				FullClient<dali_runtime::RuntimeApi, Executor>,
				FullPool<OpaqueBlock, FullClient<dali_runtime::RuntimeApi, Executor>>,
			>,
		) {
			io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client.clone())));
		}
	}
}

pub mod picasso {
	use assets_rpc::{Assets, AssetsApi};
	use common::OpaqueBlock;
	use polkadot_service::NativeExecutionDispatch;
	use sc_transaction_pool::FullPool;

	use crate::{client::FullClient, rpc::FullDeps, runtime::assets::ExtendWithAssetsApi};

	impl<Executor> ExtendWithAssetsApi<picasso_runtime::RuntimeApi, Executor>
		for picasso_runtime::RuntimeApiImpl<OpaqueBlock, FullClient<picasso_runtime::RuntimeApi, Executor>>
	where
		Executor: NativeExecutionDispatch + 'static,
	{
		fn extend_with_assets_api(
			io: &mut jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>,
			deps: FullDeps<
				FullClient<picasso_runtime::RuntimeApi, Executor>,
				FullPool<OpaqueBlock, FullClient<picasso_runtime::RuntimeApi, Executor>>,
			>,
		) {
			io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client.clone())));
		}
	}
}

#[cfg(feature = "composable")]
mod composable {
	use assets_rpc::{Assets, AssetsApi};
	use common::OpaqueBlock;
	use polkadot_service::NativeExecutionDispatch;

	use sc_transaction_pool::FullPool;

	use crate::{client::FullClient, rpc::FullDeps, runtime::assets::ExtendWithAssetsApi};

	impl<Executor> ExtendWithAssetsApi<composable_runtime::RuntimeApi, Executor>
		for composable_runtime::RuntimeApiImpl<
			OpaqueBlock,
			FullClient<composable_runtime::RuntimeApi, Executor>,
		> where
		Executor: NativeExecutionDispatch + 'static,
	{
		fn extend_with_assets_api(
			io: &mut jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>,
			deps: FullDeps<
				FullClient<composable_runtime::RuntimeApi, Executor>,
				FullPool<OpaqueBlock, FullClient<composable_runtime::RuntimeApi, Executor>>,
			>,
		) {
			io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client.clone())));
		}
	}
}

pub mod assets {
	use common::OpaqueBlock;
	use polkadot_cli::ProvideRuntimeApi;
	use polkadot_service::HeaderBackend;
	use sc_executor::NativeExecutionDispatch;
	use sc_transaction_pool::FullPool;
	use sp_blockchain::HeaderMetadata;

	use crate::{client::FullClient, rpc::FullDeps, runtime::BaseHostRuntimeApis};

	pub trait ExtendWithAssetsApi<RuntimeApi, Executor>
	where
		FullClient<RuntimeApi, Executor>: ProvideRuntimeApi<OpaqueBlock>
			+ HeaderBackend<OpaqueBlock>
			+ HeaderMetadata<OpaqueBlock, Error = sp_blockchain::Error>
			+ 'static
			+ Send
			+ Sync
			+ Sized,
		<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api:
			BaseHostRuntimeApis,
		Executor: NativeExecutionDispatch,
		RuntimeApi: Send + Sync,
	{
		/// Extends the given [`jsonrpc_core::MetaIoHandler`] with the [`AssetsApi`] runtime api.
		///
		/// The default implementation does nothing, to allow for usage with runtimes that don't
		/// implement the API.
		fn extend_with_assets_api(
			_io: &mut jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>,
			_deps: FullDeps<
				FullClient<RuntimeApi, Executor>,
				FullPool<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
			>,
		) {
		}
	}
}

pub mod crowdloan_rewards {
	use common::OpaqueBlock;
	use polkadot_cli::ProvideRuntimeApi;
	use polkadot_service::HeaderBackend;
	use sc_transaction_pool::FullPool;
	use sp_blockchain::HeaderMetadata;

	use crate::{client::FullClient, rpc::FullDeps, runtime::BaseHostRuntimeApis};

	pub trait ExtendWithCrowdloanRewardsApi<RuntimeApi, Executor>
	where
		FullClient<RuntimeApi, Executor>: ProvideRuntimeApi<OpaqueBlock>
			+ HeaderBackend<OpaqueBlock>
			+ HeaderMetadata<OpaqueBlock, Error = sp_blockchain::Error>
			+ 'static
			+ Send
			+ Sync
			+ Sized,
		<FullClient<RuntimeApi, Executor> as ProvideRuntimeApi<OpaqueBlock>>::Api:
			BaseHostRuntimeApis,
		Executor: sc_executor::NativeExecutionDispatch,
		RuntimeApi: Send + Sync,
	{
		/// Extends the given [`jsonrpc_core::MetaIoHandler`] with the [`AssetsApi`] runtime api.
		///
		/// The default implementation does nothing, to allow for usage with runtimes that don't
		/// implement the API.
		fn extend_with_crowdloan_rewards_api(
			_io: jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>,
			_deps: FullDeps<
				FullClient<RuntimeApi, Executor>,
				FullPool<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
			>,
		) {
		}
	}
}
