use common::{AccountId, Balance, Index, OpaqueBlock as Block, PoolId};
use primitives::currency::CurrencyId;
use sp_runtime::traits::BlakeTwo256;
use sp_session::SessionKeys;
use sp_transaction_pool::runtime_api::TaggedTransactionQueue;
use substrate_frame_rpc_system::AccountNonceApi;

/// Consider this a trait alias.
pub trait HostRuntimeApis:
	sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ sp_api::ApiExt<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
	+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
	+ assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance>
	+ crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi<Block, AccountId, Balance>
	+ pablo_runtime_api::PabloRuntimeApi<Block, PoolId, CurrencyId, Balance>
	+ sp_api::Metadata<Block>
	+ sp_consensus_aura::AuraApi<Block, sp_consensus_aura::sr25519::AuthorityId>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ cumulus_primitives_core::CollectCollationInfo<Block>
	+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
where
	<Self as ApiExt<OpaqueBlock>>::StateBackend: StateBackend<BlakeTwo256>,
{
}

impl<Api> BaseHostRuntimeApis for Api
where
	Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::ApiExt<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>
		+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
		+ assets_runtime_api::AssetsRuntimeApi<Block, CurrencyId, AccountId, Balance>
		+ crowdloan_rewards_runtime_api::CrowdloanRewardsRuntimeApi<Block, AccountId, Balance>
		+ pablo_runtime_api::PabloRuntimeApi<Block, PoolId, CurrencyId, Balance>
		+ sp_api::Metadata<Block>
		+ sp_consensus_aura::AuraApi<Block, sp_consensus_aura::sr25519::AuthorityId>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

macro_rules! define_trait {
	(
		$(
			mod $mod:ident {
				pub trait $Trait:ident {
					fn $fn:ident(io, deps);
				}

				$(
					$(#[cfg(feature = $feature:literal)])?
					impl for $runtime_module:ident {
						$(
							fn ($io: ident, $deps: ident) {
								$content:expr;
							}
						)?
					}
				)*
			}
		)+
	) => {
		$(
			pub mod $mod {
				use common::OpaqueBlock;
				use polkadot_cli::ProvideRuntimeApi;
				use polkadot_service::HeaderBackend;
				use sc_transaction_pool::FullPool;
				use sp_blockchain::HeaderMetadata;

				use crate::{client::FullClient, rpc::FullDeps, runtime::BaseHostRuntimeApis};

				pub trait $Trait<RuntimeApi, Executor>
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
					#[doc=concat!(" Extends the given [`jsonrpc_core::MetaIoHandler`] with the ", stringify!($mod), " runtime api.")]
					#[doc=""]
					#[doc=" The default implementation does nothing, to allow for usage with runtimes that don't"]
					#[doc=" implement the API."]
					fn $fn(
						_io: &mut jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>,
						_deps: FullDeps<
							FullClient<RuntimeApi, Executor>,
							FullPool<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
						>,
					) {
					}
				}
			}

			$(
				$(#[cfg(feature = $feature)])?
				impl<Executor> $mod::$Trait<$runtime_module::RuntimeApi, Executor>
					for $runtime_module::RuntimeApiImpl<
						common::OpaqueBlock,
						crate::client::FullClient<$runtime_module::RuntimeApi, Executor>,
					> where
					Executor: sc_executor::NativeExecutionDispatch + 'static,
				{
					$(
						fn $fn(
							$io: &mut jsonrpc_core::MetaIoHandler<sc_rpc::Metadata>,
							$deps: crate::rpc::FullDeps<
								crate::client::FullClient<$runtime_module::RuntimeApi, Executor>,
								sc_transaction_pool::FullPool<OpaqueBlock, crate::client::FullClient<$runtime_module::RuntimeApi, Executor>>,
							>,
						) {
							$content;
						}
					)?
				}
			)*
		)+
	};
}

define_trait! {
	mod assets {
		pub trait ExtendWithAssetsApi {
			fn extend_with_assets_api(io, deps);
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {
			fn (io, deps) {
				io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client)));
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client)));
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client)));
			}
		}
	}

	mod crowdloan_rewards {
		pub trait ExtendWithCrowdloanRewardsApi {
			fn extend_with_crowdloan_rewards_api(io, deps);
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {
			fn (io, deps) {
				io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(deps.client)));
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(deps.client)));
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(deps.client)));
			}
		}
	}

	mod pablo {
		pub trait ExtendWithPabloApi {
			fn extend_with_pablo_api(io, deps);
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {
			fn (io, deps) {
				io.extend_with(PabloApi::to_delegate(Pablo::new(deps.client)));
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.extend_with(PabloApi::to_delegate(Pablo::new(deps.client)));
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.extend_with(PabloApi::to_delegate(Pablo::new(deps.client)));
			}
		}
	}

	mod lending {
		pub trait ExtendWithLendingApi {
			fn extend_with_lending_api(io, deps);
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {}

		impl for picasso_runtime {}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.extend_with(LendingApi::to_delegate(Lending::new(deps.client)));
			}
		}
	}
}
