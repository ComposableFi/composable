use assets_rpc::{Assets, AssetsApiServer};
use common::{AccountId, Balance, Index, OpaqueBlock};
use cosmwasm_rpc::{Cosmwasm, CosmwasmApiServer};
use crowdloan_rewards_rpc::{CrowdloanRewards, CrowdloanRewardsApiServer};
use cumulus_primitives_core::CollectCollationInfo;
use ibc_rpc::{IbcApiServer, IbcRpcHandler};
use lending_rpc::{Lending, LendingApiServer};
use pablo_rpc::{Pablo, PabloApiServer};
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
								$content:expr
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
						_io: &mut jsonrpsee::RpcModule<()>,
						_deps: FullDeps<
							FullClient<RuntimeApi, Executor>,
							FullPool<OpaqueBlock, FullClient<RuntimeApi, Executor>>,
						>,
					) -> Result<(), jsonrpsee::core::Error> {
						Ok(())
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
							$io: &mut jsonrpsee::RpcModule<()>,
							$deps: crate::rpc::FullDeps<
								crate::client::FullClient<$runtime_module::RuntimeApi, Executor>,
								sc_transaction_pool::FullPool<OpaqueBlock, crate::client::FullClient<$runtime_module::RuntimeApi, Executor>>,
							>,
						) -> Result<(), jsonrpsee::core::Error> {
							$content
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
				io.merge(Assets::new(deps.client).into_rpc())
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.merge(Assets::new(deps.client).into_rpc())
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.merge(Assets::new(deps.client).into_rpc())
			}
		}
	}

	mod crowdloan_rewards {
		pub trait ExtendWithCrowdloanRewardsApi {
			fn extend_with_crowdloan_rewards_api(io, deps) ;
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {
			fn (io, deps) {
				io.merge(CrowdloanRewards::new(deps.client).into_rpc())
			}
		}

		impl for picasso_runtime {
			fn (io, deps) {
				io.merge(CrowdloanRewards::new(deps.client).into_rpc())
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.merge(CrowdloanRewards::new(deps.client).into_rpc())
			}
		}
	}

	mod pablo {
		pub trait ExtendWithPabloApi {
			fn extend_with_pablo_api(io, deps);
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {}

		impl for picasso_runtime {
			fn (io, deps) {
				io.merge(Pablo::new(deps.client).into_rpc())
			}
		}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.merge(Pablo::new(deps.client).into_rpc())
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
				io.merge(Lending::new(deps.client).into_rpc())
			}
		}
	}

	mod cosmwasm {
		pub trait ExtendWithCosmwasmApi {
			fn extend_with_cosmwasm_api(io, deps) ;
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {}

		impl for picasso_runtime {}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.merge(Cosmwasm::new(deps.client).into_rpc())
			}
		}
	}

	mod ibc {
		pub trait ExtendWithIbcApi {
			fn extend_with_ibc_api(io, deps) ;
		}

		#[cfg(feature = "composable")]
		impl for composable_runtime {}

		impl for picasso_runtime {}

		#[cfg(feature = "dali")]
		impl for dali_runtime {
			fn (io, deps) {
				io.merge(IbcRpcHandler::new(deps.client.clone(), deps.chain_props).into_rpc())
			}
		}
	}
}
