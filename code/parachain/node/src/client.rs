#[cfg(feature = "composable")]
use crate::service::ComposableExecutor;
#[cfg(feature = "dali")]
use crate::service::DaliExecutor;
use crate::{runtime::BaseHostRuntimeApis, service::PicassoExecutor};
pub use common::{AccountId, Balance, BlockNumber, Hash, Header, Index, OpaqueBlock as Block};
use sc_client_api::{Backend as BackendT, BlockchainEvents, KeyIterator};
use sc_executor::NativeElseWasmExecutor;
use sc_service::{TFullBackend, TFullClient};
use sp_api::{CallApiAt, NumberFor, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_consensus::BlockStatus;
use sp_runtime::{
	generic::{BlockId, SignedBlock},
	traits::{BlakeTwo256, Block as BlockT},
	Justifications,
};
use sp_storage::{ChildInfo, StorageData, StorageKey};
use std::sync::Arc;

pub type FullClient<RuntimeApi, Executor> =
	TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;
pub type FullBackend = TFullBackend<Block>;
pub(crate) type PicassoClient = FullClient<picasso_runtime::RuntimeApi, PicassoExecutor>;
#[cfg(feature = "composable")]
pub(crate) type ComposableClient = FullClient<composable_runtime::RuntimeApi, ComposableExecutor>;
#[cfg(feature = "dali")]
pub(crate) type DaliClient = FullClient<dali_runtime::RuntimeApi, DaliExecutor>;

/// A client instance of Picasso.
#[derive(Clone)]
pub enum Client {
	/// Picasso client type
	Picasso(Arc<PicassoClient>),
	/// Composable client type
	#[cfg(feature = "composable")]
	Composable(Arc<ComposableClient>),
	/// Dali client type
	#[cfg(feature = "dali")]
	Dali(Arc<DaliClient>),
}

/// Config that abstracts over all available client implementations.
///
/// For a concrete type there exists [`Client`].
pub trait AbstractClient<Block, Backend>:
	BlockchainEvents<Block>
	+ Sized
	+ Send
	+ Sync
	+ ProvideRuntimeApi<Block>
	+ HeaderBackend<Block>
	+ CallApiAt<Block, StateBackend = Backend::State>
where
	Block: BlockT,
	Backend: BackendT<Block>,
	Backend::State: sp_api::StateBackend<BlakeTwo256>,
	Self::Api: BaseHostRuntimeApis<StateBackend = Backend::State>,
{
}

impl<Block, Backend, Client> AbstractClient<Block, Backend> for Client
where
	Block: BlockT,
	Backend: BackendT<Block>,
	Backend::State: sp_api::StateBackend<BlakeTwo256>,
	Client: BlockchainEvents<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ Sized
		+ Send
		+ Sync
		+ CallApiAt<Block, StateBackend = Backend::State>,
	Client::Api: BaseHostRuntimeApis<StateBackend = Backend::State>,
{
}

impl From<Arc<PicassoClient>> for Client {
	fn from(client: Arc<PicassoClient>) -> Self {
		Self::Picasso(client)
	}
}

#[cfg(feature = "composable")]
impl From<Arc<ComposableClient>> for Client {
	fn from(client: Arc<ComposableClient>) -> Self {
		Self::Composable(client)
	}
}

#[cfg(feature = "dali")]
impl From<Arc<DaliClient>> for Client {
	fn from(client: Arc<DaliClient>) -> Self {
		Self::Dali(client)
	}
}

macro_rules! match_client {
	($self:ident, $method:ident($($param:ident),*)) => {
		match $self {
			Self::Picasso(client) => client.$method($($param),*),
			#[cfg(feature = "composable")]
			Self::Composable(client) => client.$method($($param),*),
			#[cfg(feature = "dali")]
			Self::Dali(client) => client.$method($($param),*),
		}
	};
}

impl sc_client_api::UsageProvider<Block> for Client {
	fn usage_info(&self) -> sc_client_api::ClientInfo<Block> {
		match_client!(self, usage_info())
	}
}

impl sc_client_api::BlockBackend<Block> for Client {
	fn block_body(
		&self,
		id: &BlockId<Block>,
	) -> sp_blockchain::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
		match_client!(self, block_body(id))
	}

	fn block_indexed_body(
		&self,
		id: &BlockId<Block>,
	) -> sp_blockchain::Result<Option<Vec<Vec<u8>>>> {
		match_client!(self, block_indexed_body(id))
	}

	fn block(&self, id: &BlockId<Block>) -> sp_blockchain::Result<Option<SignedBlock<Block>>> {
		match_client!(self, block(id))
	}

	fn block_status(&self, id: &BlockId<Block>) -> sp_blockchain::Result<BlockStatus> {
		match_client!(self, block_status(id))
	}

	fn justifications(&self, id: &BlockId<Block>) -> sp_blockchain::Result<Option<Justifications>> {
		match_client!(self, justifications(id))
	}

	fn block_hash(
		&self,
		number: NumberFor<Block>,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, block_hash(number))
	}

	fn indexed_transaction(
		&self,
		hash: &<Block as BlockT>::Hash,
	) -> sp_blockchain::Result<Option<Vec<u8>>> {
		match_client!(self, indexed_transaction(hash))
	}

	fn has_indexed_transaction(
		&self,
		hash: &<Block as BlockT>::Hash,
	) -> sp_blockchain::Result<bool> {
		match_client!(self, has_indexed_transaction(hash))
	}

	fn requires_full_sync(&self) -> bool {
		match self {
			Self::Picasso(client) => client.requires_full_sync(),
			#[cfg(feature = "dali")]
			Self::Dali(client) => client.requires_full_sync(),
			#[cfg(feature = "composable")]
			Self::Composable(client) => client.requires_full_sync(),
		}
	}
}

impl sc_client_api::StorageProvider<Block, FullBackend> for Client {
	fn storage(
		&self,
		id: &BlockId<Block>,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<StorageData>> {
		match_client!(self, storage(id, key))
	}

	fn storage_keys(
		&self,
		id: &BlockId<Block>,
		key_prefix: &StorageKey,
	) -> sp_blockchain::Result<Vec<StorageKey>> {
		match_client!(self, storage_keys(id, key_prefix))
	}

	fn storage_hash(
		&self,
		id: &BlockId<Block>,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, storage_hash(id, key))
	}

	fn storage_pairs(
		&self,
		id: &BlockId<Block>,
		key_prefix: &StorageKey,
	) -> sp_blockchain::Result<Vec<(StorageKey, StorageData)>> {
		match_client!(self, storage_pairs(id, key_prefix))
	}

	fn storage_keys_iter<'a>(
		&self,
		id: &BlockId<Block>,
		prefix: Option<&'a StorageKey>,
		start_key: Option<&StorageKey>,
	) -> sp_blockchain::Result<
		KeyIterator<'a, <FullBackend as sc_client_api::Backend<Block>>::State, Block>,
	> {
		match_client!(self, storage_keys_iter(id, prefix, start_key))
	}

	fn child_storage(
		&self,
		id: &BlockId<Block>,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<StorageData>> {
		match_client!(self, child_storage(id, child_info, key))
	}

	fn child_storage_keys(
		&self,
		id: &BlockId<Block>,
		child_info: &ChildInfo,
		key_prefix: &StorageKey,
	) -> sp_blockchain::Result<Vec<StorageKey>> {
		match_client!(self, child_storage_keys(id, child_info, key_prefix))
	}

	fn child_storage_keys_iter<'a>(
		&self,
		id: &BlockId<Block>,
		child_info: ChildInfo,
		prefix: Option<&'a StorageKey>,
		start_key: Option<&StorageKey>,
	) -> sp_blockchain::Result<
		KeyIterator<'a, <FullBackend as sc_client_api::Backend<Block>>::State, Block>,
	> {
		match_client!(self, child_storage_keys_iter(id, child_info, prefix, start_key))
	}

	fn child_storage_hash(
		&self,
		id: &BlockId<Block>,
		child_info: &ChildInfo,
		key: &StorageKey,
	) -> sp_blockchain::Result<Option<<Block as BlockT>::Hash>> {
		match_client!(self, child_storage_hash(id, child_info, key))
	}
}

impl sp_blockchain::HeaderBackend<Block> for Client {
	fn header(&self, id: BlockId<Block>) -> sp_blockchain::Result<Option<Header>> {
		let id = &id;
		match_client!(self, header(id))
	}

	fn info(&self) -> sp_blockchain::Info<Block> {
		match_client!(self, info())
	}

	fn status(&self, id: BlockId<Block>) -> sp_blockchain::Result<sp_blockchain::BlockStatus> {
		match_client!(self, status(id))
	}

	fn number(&self, hash: Hash) -> sp_blockchain::Result<Option<BlockNumber>> {
		match_client!(self, number(hash))
	}

	fn hash(&self, number: BlockNumber) -> sp_blockchain::Result<Option<Hash>> {
		match_client!(self, hash(number))
	}
}
