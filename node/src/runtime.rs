use assets_rpc::{Assets, AssetsApi};
use common::{AccountId, Balance, Index, OpaqueBlock};
use crowdloan_rewards_rpc::{CrowdloanRewards, CrowdloanRewardsApi};
use cumulus_primitives_core::CollectCollationInfo;
use lending_rpc::{Lending, LendingApi};
use pablo_rpc::{Pablo, PabloApi};
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
				io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client.clone())));
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client.clone())));
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.extend_with(AssetsApi::to_delegate(Assets::new(deps.client.clone())));
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
				io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(deps.client.clone())));
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(deps.client.clone())));
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.extend_with(CrowdloanRewardsApi::to_delegate(CrowdloanRewards::new(deps.client.clone())));
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
				io.extend_with(PabloApi::to_delegate(Pablo::new(deps.client.clone())));
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.extend_with(PabloApi::to_delegate(Pablo::new(deps.client.clone())));
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.extend_with(PabloApi::to_delegate(Pablo::new(deps.client.clone())));
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
				io.extend_with(LendingApi::to_delegate(Lending::new(deps.client.clone())));
			}
		}
	}
}
