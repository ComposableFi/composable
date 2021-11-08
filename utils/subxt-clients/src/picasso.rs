#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
	#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 2)]
		Sudo(sudo::Event),
		#[codec(index = 5)]
		Indices(indices::Event),
		#[codec(index = 6)]
		Balances(balances::Event),
		#[codec(index = 10)]
		ParachainSystem(parachain_system::Event),
		#[codec(index = 21)]
		CollatorSelection(collator_selection::Event),
		#[codec(index = 22)]
		Session(session::Event),
		#[codec(index = 30)]
		Council(council::Event),
		#[codec(index = 31)]
		CouncilMembership(council_membership::Event),
		#[codec(index = 32)]
		Treasury(treasury::Event),
		#[codec(index = 33)]
		Democracy(democracy::Event),
		#[codec(index = 34)]
		Scheduler(scheduler::Event),
		#[codec(index = 35)]
		Utility(utility::Event),
		#[codec(index = 40)]
		XcmpQueue(xcmp_queue::Event),
		#[codec(index = 41)]
		PolkadotXcm(polkadot_xcm::Event),
		#[codec(index = 42)]
		CumulusXcm(cumulus_xcm::Event),
		#[codec(index = 43)]
		DmpQueue(dmp_queue::Event),
		#[codec(index = 50)]
		Oracle(oracle::Event),
		#[codec(index = 51)]
		Tokens(tokens::Event),
		#[codec(index = 52)]
		Factory(factory::Event),
		#[codec(index = 53)]
		Vault(vault::Event),
		#[codec(index = 54)]
		Lending(lending::Event),
		#[codec(index = 55)]
		LiquidCrowdloan(liquid_crowdloan::Event),
		#[codec(index = 56)]
		Liquidations(liquidations::Event),
		#[codec(index = 57)]
		Auctions(auctions::Event),
		#[codec(index = 58)]
		Ping(ping::Event),
		#[codec(index = 59)]
		AssetsRegistry(assets_registry::Event),
		#[codec(index = 90)]
		Spambot(spambot::Event),
		#[codec(index = 100)]
		CallFilter(call_filter::Event),
	}
	pub mod system {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct FillBlock {
				pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			impl ::subxt::Call for FillBlock {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "fill_block";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Remark {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Remark {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "remark";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetHeapPages {
				pub pages: ::core::primitive::u64,
			}
			impl ::subxt::Call for SetHeapPages {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_heap_pages";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetCode {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SetCode {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_code";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetCodeWithoutChecks {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SetCodeWithoutChecks {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_code_without_checks";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetChangesTrieConfig {
				pub changes_trie_config: ::core::option::Option<
					runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
				>,
			}
			impl ::subxt::Call for SetChangesTrieConfig {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_changes_trie_config";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetStorage {
				pub items: ::std::vec::Vec<(
					::std::vec::Vec<::core::primitive::u8>,
					::std::vec::Vec<::core::primitive::u8>,
				)>,
			}
			impl ::subxt::Call for SetStorage {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_storage";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct KillStorage {
				pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
			}
			impl ::subxt::Call for KillStorage {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "kill_storage";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct KillPrefix {
				pub prefix: ::std::vec::Vec<::core::primitive::u8>,
				pub subkeys: ::core::primitive::u32,
			}
			impl ::subxt::Call for KillPrefix {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "kill_prefix";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RemarkWithEvent {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for RemarkWithEvent {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "remark_with_event";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn fill_block(
					&self,
					ratio: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::SubmittableExtrinsic<T, FillBlock> {
					let call = FillBlock { ratio };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Remark> {
					let call = Remark { remark };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::SubmittableExtrinsic<T, SetHeapPages> {
					let call = SetHeapPages { pages };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, SetCode> {
					let call = SetCode { code };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, SetCodeWithoutChecks> {
					let call = SetCodeWithoutChecks { code };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_changes_trie_config(
					&self,
					changes_trie_config: ::core::option::Option<
						runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
					>,
				) -> ::subxt::SubmittableExtrinsic<T, SetChangesTrieConfig> {
					let call = SetChangesTrieConfig { changes_trie_config };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::SubmittableExtrinsic<T, SetStorage> {
					let call = SetStorage { items };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::SubmittableExtrinsic<T, KillStorage> {
					let call = KillStorage { keys };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, KillPrefix> {
					let call = KillPrefix { prefix, subkeys };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, RemarkWithEvent> {
					let call = RemarkWithEvent { remark };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
			impl ::subxt::Event for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExtrinsicFailed(
				pub runtime_types::sp_runtime::DispatchError,
				pub runtime_types::frame_support::weights::DispatchInfo,
			);
			impl ::subxt::Event for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CodeUpdated {}
			impl ::subxt::Event for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Remarked(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::H256,
			);
			impl ::subxt::Event for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Account {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "Account";
				type Value = runtime_types::frame_system::AccountInfo<
					::core::primitive::u32,
					runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct ExtrinsicCount;
			impl ::subxt::StorageEntry for ExtrinsicCount {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "ExtrinsicCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct BlockWeight;
			impl ::subxt::StorageEntry for BlockWeight {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "BlockWeight";
				type Value =
					runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct AllExtrinsicsLen;
			impl ::subxt::StorageEntry for AllExtrinsicsLen {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "AllExtrinsicsLen";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct BlockHash(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for BlockHash {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "BlockHash";
				type Value = ::subxt::sp_core::H256;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct ExtrinsicData(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for ExtrinsicData {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "ExtrinsicData";
				type Value = ::std::vec::Vec<::core::primitive::u8>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct Number;
			impl ::subxt::StorageEntry for Number {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "Number";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ParentHash;
			impl ::subxt::StorageEntry for ParentHash {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "ParentHash";
				type Value = ::subxt::sp_core::H256;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Digest;
			impl ::subxt::StorageEntry for Digest {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "Digest";
				type Value =
					runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Events;
			impl ::subxt::StorageEntry for Events {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "Events";
				type Value = ::std::vec::Vec<
					runtime_types::frame_system::EventRecord<
						runtime_types::picasso_runtime::Event,
						::subxt::sp_core::H256,
					>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct EventCount;
			impl ::subxt::StorageEntry for EventCount {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "EventCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct EventTopics(pub ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for EventTopics {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "EventTopics";
				type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct LastRuntimeUpgrade;
			impl ::subxt::StorageEntry for LastRuntimeUpgrade {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "LastRuntimeUpgrade";
				type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct UpgradedToU32RefCount;
			impl ::subxt::StorageEntry for UpgradedToU32RefCount {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "UpgradedToU32RefCount";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct UpgradedToTripleRefCount;
			impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "UpgradedToTripleRefCount";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ExecutionPhase;
			impl ::subxt::StorageEntry for ExecutionPhase {
				const PALLET: &'static str = "System";
				const STORAGE: &'static str = "ExecutionPhase";
				type Value = runtime_types::frame_system::Phase;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn account(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					::subxt::Error,
				> {
					let entry = Account(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn account_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn extrinsic_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::Error,
				> {
					let entry = ExtrinsicCount;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn block_weight(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
					::subxt::Error,
				> {
					let entry = BlockWeight;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn all_extrinsics_len(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::Error,
				> {
					let entry = AllExtrinsicsLen;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn block_hash(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error> {
					let entry = BlockHash(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn block_hash_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn extrinsic_data(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
				{
					let entry = ExtrinsicData(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn extrinsic_data_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, ExtrinsicData>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
				pub async fn number(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = Number;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn parent_hash(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::Error> {
					let entry = ParentHash;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn digest(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>,
					::subxt::Error,
				> {
					let entry = Digest;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn events(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::picasso_runtime::Event,
							::subxt::sp_core::H256,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Events;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn event_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = EventCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn event_topics(
					&self,
					_0: ::subxt::sp_core::H256,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::Error,
				> {
					let entry = EventTopics(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn event_topics_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn last_runtime_upgrade(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
					::subxt::Error,
				> {
					let entry = LastRuntimeUpgrade;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn upgraded_to_u32_ref_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = UpgradedToU32RefCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn upgraded_to_triple_ref_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = UpgradedToTripleRefCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn execution_phase(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::frame_system::Phase>,
					::subxt::Error,
				> {
					let entry = ExecutionPhase;
					self.client.storage().fetch(&entry, hash).await
				}
			}
		}
	}
	pub mod timestamp {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Set {
				#[codec(compact)]
				pub now: ::core::primitive::u64,
			}
			impl ::subxt::Call for Set {
				const PALLET: &'static str = "Timestamp";
				const FUNCTION: &'static str = "set";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn set(
					&self,
					now: ::core::primitive::u64,
				) -> ::subxt::SubmittableExtrinsic<T, Set> {
					let call = Set { now };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Now;
			impl ::subxt::StorageEntry for Now {
				const PALLET: &'static str = "Timestamp";
				const STORAGE: &'static str = "Now";
				type Value = ::core::primitive::u64;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct DidUpdate;
			impl ::subxt::StorageEntry for DidUpdate {
				const PALLET: &'static str = "Timestamp";
				const STORAGE: &'static str = "DidUpdate";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn now(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error> {
					let entry = Now;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn did_update(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = DidUpdate;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod sudo {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Sudo {
				pub call: runtime_types::picasso_runtime::Call,
			}
			impl ::subxt::Call for Sudo {
				const PALLET: &'static str = "Sudo";
				const FUNCTION: &'static str = "sudo";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SudoUncheckedWeight {
				pub call: runtime_types::picasso_runtime::Call,
				pub weight: ::core::primitive::u64,
			}
			impl ::subxt::Call for SudoUncheckedWeight {
				const PALLET: &'static str = "Sudo";
				const FUNCTION: &'static str = "sudo_unchecked_weight";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetKey {
				pub new: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			impl ::subxt::Call for SetKey {
				const PALLET: &'static str = "Sudo";
				const FUNCTION: &'static str = "set_key";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SudoAs {
				pub who: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub call: runtime_types::picasso_runtime::Call,
			}
			impl ::subxt::Call for SudoAs {
				const PALLET: &'static str = "Sudo";
				const FUNCTION: &'static str = "sudo_as";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn sudo(
					&self,
					call: runtime_types::picasso_runtime::Call,
				) -> ::subxt::SubmittableExtrinsic<T, Sudo> {
					let call = Sudo { call };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::picasso_runtime::Call,
					weight: ::core::primitive::u64,
				) -> ::subxt::SubmittableExtrinsic<T, SudoUncheckedWeight> {
					let call = SudoUncheckedWeight { call, weight };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_key(
					&self,
					new: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::SubmittableExtrinsic<T, SetKey> {
					let call = SetKey { new };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn sudo_as(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::picasso_runtime::Call,
				) -> ::subxt::SubmittableExtrinsic<T, SudoAs> {
					let call = SudoAs { who, call };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Sudid(
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::Event for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct KeyChanged(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SudoAsDone(
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::Event for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Key;
			impl ::subxt::StorageEntry for Key {
				const PALLET: &'static str = "Sudo";
				const STORAGE: &'static str = "Key";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn key(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::sp_core::crypto::AccountId32, ::subxt::Error> {
					let entry = Key;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod randomness_collective_flip {
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct RandomMaterial;
			impl ::subxt::StorageEntry for RandomMaterial {
				const PALLET: &'static str = "RandomnessCollectiveFlip";
				const STORAGE: &'static str = "RandomMaterial";
				type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn random_material(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::std::vec::Vec<::subxt::sp_core::H256>, ::subxt::Error>
				{
					let entry = RandomMaterial;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct NextFeeMultiplier;
			impl ::subxt::StorageEntry for NextFeeMultiplier {
				const PALLET: &'static str = "TransactionPayment";
				const STORAGE: &'static str = "NextFeeMultiplier";
				type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageVersion;
			impl ::subxt::StorageEntry for StorageVersion {
				const PALLET: &'static str = "TransactionPayment";
				const STORAGE: &'static str = "StorageVersion";
				type Value = runtime_types::pallet_transaction_payment::Releases;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn next_fee_multiplier(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::Error,
				> {
					let entry = NextFeeMultiplier;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn storage_version(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_transaction_payment::Releases,
					::subxt::Error,
				> {
					let entry = StorageVersion;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod indices {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Claim {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Claim {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "claim";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Transfer {
				pub new: ::subxt::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Transfer {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Free {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Free {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "free";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceTransfer {
				pub new: ::subxt::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
				pub freeze: ::core::primitive::bool,
			}
			impl ::subxt::Call for ForceTransfer {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "force_transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Freeze {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Freeze {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "freeze";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn claim(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Claim> {
					let call = Claim { index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn transfer(
					&self,
					new: ::subxt::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
					let call = Transfer { new, index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn free(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Free> {
					let call = Free { index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_transfer(
					&self,
					new: ::subxt::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
					let call = ForceTransfer { new, index, freeze };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn freeze(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Freeze> {
					let call = Freeze { index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_indices::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct IndexAssigned(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for IndexAssigned {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexAssigned";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct IndexFreed(pub ::core::primitive::u32);
			impl ::subxt::Event for IndexFreed {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFreed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct IndexFrozen(
				pub ::core::primitive::u32,
				pub ::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::Event for IndexFrozen {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFrozen";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Accounts(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for Accounts {
				const PALLET: &'static str = "Indices";
				const STORAGE: &'static str = "Accounts";
				type Value = (
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u128,
					::core::primitive::bool,
				);
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn accounts(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					::subxt::Error,
				> {
					let entry = Accounts(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn accounts_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Accounts>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod balances {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Transfer {
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::Call for Transfer {
				const PALLET: &'static str = "Balances";
				const FUNCTION: &'static str = "transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetBalance {
				pub who: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			impl ::subxt::Call for SetBalance {
				const PALLET: &'static str = "Balances";
				const FUNCTION: &'static str = "set_balance";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceTransfer {
				pub source: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::Call for ForceTransfer {
				const PALLET: &'static str = "Balances";
				const FUNCTION: &'static str = "force_transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::Call for TransferKeepAlive {
				const PALLET: &'static str = "Balances";
				const FUNCTION: &'static str = "transfer_keep_alive";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct TransferAll {
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			impl ::subxt::Call for TransferAll {
				const PALLET: &'static str = "Balances";
				const FUNCTION: &'static str = "transfer_all";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceUnreserve {
				pub who: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for ForceUnreserve {
				const PALLET: &'static str = "Balances";
				const FUNCTION: &'static str = "force_unreserve";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn transfer(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
					let call = Transfer { dest, value };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_balance(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, SetBalance> {
					let call = SetBalance { who, new_free, new_reserved };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_transfer(
					&self,
					source: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
					let call = ForceTransfer { source, dest, value };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
					let call = TransferKeepAlive { dest, value };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn transfer_all(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::SubmittableExtrinsic<T, TransferAll> {
					let call = TransferAll { dest, keep_alive };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_unreserve(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, ForceUnreserve> {
					let call = ForceUnreserve { who, amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Endowed(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct DustLost(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Transfer(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BalanceSet(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Reserved(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Unreserved(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ReserveRepatriated(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			);
			impl ::subxt::Event for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Deposit(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Withdraw(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Slashed(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct TotalIssuance;
			impl ::subxt::StorageEntry for TotalIssuance {
				const PALLET: &'static str = "Balances";
				const STORAGE: &'static str = "TotalIssuance";
				type Value = ::core::primitive::u128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Account {
				const PALLET: &'static str = "Balances";
				const STORAGE: &'static str = "Account";
				type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Locks {
				const PALLET: &'static str = "Balances";
				const STORAGE: &'static str = "Locks";
				type Value =
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
					>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Reserves {
				const PALLET: &'static str = "Balances";
				const STORAGE: &'static str = "Reserves";
				type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
					runtime_types::pallet_balances::ReserveData<
						[::core::primitive::u8; 8usize],
						::core::primitive::u128,
					>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct StorageVersion;
			impl ::subxt::StorageEntry for StorageVersion {
				const PALLET: &'static str = "Balances";
				const STORAGE: &'static str = "StorageVersion";
				type Value = runtime_types::pallet_balances::Releases;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn total_issuance(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error> {
					let entry = TotalIssuance;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn account(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					::subxt::Error,
				> {
					let entry = Account(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn account_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn locks(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::Error,
				> {
					let entry = Locks(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn locks_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn reserves(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Reserves(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn reserves_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn storage_version(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<runtime_types::pallet_balances::Releases, ::subxt::Error>
				{
					let entry = StorageVersion;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod parachain_system {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetValidationData {
				pub data:
					runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
			}
			impl ::subxt::Call for SetValidationData {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "set_validation_data";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SudoSendUpwardMessage {
				pub message: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SudoSendUpwardMessage {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "sudo_send_upward_message";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AuthorizeUpgrade {
				pub code_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for AuthorizeUpgrade {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "authorize_upgrade";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct EnactAuthorizedUpgrade {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for EnactAuthorizedUpgrade {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "enact_authorized_upgrade";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
				) -> ::subxt::SubmittableExtrinsic<T, SetValidationData> {
					let call = SetValidationData { data };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn sudo_send_upward_message(
					&self,
					message: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, SudoSendUpwardMessage> {
					let call = SudoSendUpwardMessage { message };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn authorize_upgrade(
					&self,
					code_hash: ::subxt::sp_core::H256,
				) -> ::subxt::SubmittableExtrinsic<T, AuthorizeUpgrade> {
					let call = AuthorizeUpgrade { code_hash };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn enact_authorized_upgrade(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, EnactAuthorizedUpgrade> {
					let call = EnactAuthorizedUpgrade { code };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ValidationFunctionStored {}
			impl ::subxt::Event for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ValidationFunctionApplied(pub ::core::primitive::u32);
			impl ::subxt::Event for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ValidationFunctionDiscarded {}
			impl ::subxt::Event for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UpgradeAuthorized(pub ::subxt::sp_core::H256);
			impl ::subxt::Event for UpgradeAuthorized {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct DownwardMessagesReceived(pub ::core::primitive::u32);
			impl ::subxt::Event for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct DownwardMessagesProcessed(
				pub ::core::primitive::u64,
				pub ::subxt::sp_core::H256,
			);
			impl ::subxt::Event for DownwardMessagesProcessed {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesProcessed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct PendingValidationCode;
			impl ::subxt::StorageEntry for PendingValidationCode {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "PendingValidationCode";
				type Value = ::std::vec::Vec<::core::primitive::u8>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct NewValidationCode;
			impl ::subxt::StorageEntry for NewValidationCode {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "NewValidationCode";
				type Value = ::std::vec::Vec<::core::primitive::u8>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ValidationData;
			impl ::subxt::StorageEntry for ValidationData {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "ValidationData";
				type Value = runtime_types::polkadot_primitives::v1::PersistedValidationData<
					::subxt::sp_core::H256,
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct DidSetValidationCode;
			impl ::subxt::StorageEntry for DidSetValidationCode {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "DidSetValidationCode";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct UpgradeRestrictionSignal;
			impl ::subxt::StorageEntry for UpgradeRestrictionSignal {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "UpgradeRestrictionSignal";
				type Value = ::core::option::Option<
					runtime_types::polkadot_primitives::v1::UpgradeRestriction,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct RelevantMessagingState;
			impl ::subxt::StorageEntry for RelevantMessagingState {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "RelevantMessagingState";
				type Value = runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot ;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct HostConfiguration;
			impl ::subxt::StorageEntry for HostConfiguration {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "HostConfiguration";
				type Value = runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct LastDmqMqcHead;
			impl ::subxt::StorageEntry for LastDmqMqcHead {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "LastDmqMqcHead";
				type Value = runtime_types::cumulus_pallet_parachain_system::MessageQueueChain;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ProcessedDownwardMessages;
			impl ::subxt::StorageEntry for ProcessedDownwardMessages {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "ProcessedDownwardMessages";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct HrmpWatermark;
			impl ::subxt::StorageEntry for HrmpWatermark {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "HrmpWatermark";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct HrmpOutboundMessages;
			impl ::subxt::StorageEntry for HrmpOutboundMessages {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "HrmpOutboundMessages";
				type Value = ::std::vec::Vec<
					runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct UpwardMessages;
			impl ::subxt::StorageEntry for UpwardMessages {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "UpwardMessages";
				type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct PendingUpwardMessages;
			impl ::subxt::StorageEntry for PendingUpwardMessages {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "PendingUpwardMessages";
				type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct AnnouncedHrmpMessagesPerCandidate;
			impl ::subxt::StorageEntry for AnnouncedHrmpMessagesPerCandidate {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "AnnouncedHrmpMessagesPerCandidate";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ReservedXcmpWeightOverride;
			impl ::subxt::StorageEntry for ReservedXcmpWeightOverride {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "ReservedXcmpWeightOverride";
				type Value = ::core::primitive::u64;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ReservedDmpWeightOverride;
			impl ::subxt::StorageEntry for ReservedDmpWeightOverride {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "ReservedDmpWeightOverride";
				type Value = ::core::primitive::u64;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct AuthorizedUpgrade;
			impl ::subxt::StorageEntry for AuthorizedUpgrade {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "AuthorizedUpgrade";
				type Value = ::subxt::sp_core::H256;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn pending_validation_code(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
				{
					let entry = PendingValidationCode;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn new_validation_code(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::Error,
				> {
					let entry = NewValidationCode;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn validation_data(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::polkadot_primitives::v1::PersistedValidationData<
							::subxt::sp_core::H256,
							::core::primitive::u32,
						>,
					>,
					::subxt::Error,
				> {
					let entry = ValidationData;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn did_set_validation_code(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = DidSetValidationCode;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn upgrade_restriction_signal(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::polkadot_primitives::v1::UpgradeRestriction,
					>,
					::subxt::Error,
				> {
					let entry = UpgradeRestrictionSignal;
					self.client.storage().fetch_or_default(&entry, hash).await
				}				pub async fn relevant_messaging_state (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: Error >{
					let entry = RelevantMessagingState;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn host_configuration(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration,
					>,
					::subxt::Error,
				> {
					let entry = HostConfiguration;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn last_dmq_mqc_head(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
					::subxt::Error,
				> {
					let entry = LastDmqMqcHead;
					self.client.storage().fetch_or_default(&entry, hash).await
				}

				pub async fn processed_downward_messages(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = ProcessedDownwardMessages;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn hrmp_watermark(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = HrmpWatermark;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn hrmp_outbound_messages(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
							runtime_types::polkadot_parachain::primitives::Id,
						>,
					>,
					::subxt::Error,
				> {
					let entry = HrmpOutboundMessages;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn upward_messages(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::Error,
				> {
					let entry = UpwardMessages;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn pending_upward_messages(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::Error,
				> {
					let entry = PendingUpwardMessages;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn announced_hrmp_messages_per_candidate(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = AnnouncedHrmpMessagesPerCandidate;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn reserved_xcmp_weight_override(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u64>,
					::subxt::Error,
				> {
					let entry = ReservedXcmpWeightOverride;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn reserved_dmp_weight_override(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u64>,
					::subxt::Error,
				> {
					let entry = ReservedDmpWeightOverride;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn authorized_upgrade(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::H256>,
					::subxt::Error,
				> {
					let entry = AuthorizedUpgrade;
					self.client.storage().fetch(&entry, hash).await
				}
			}
		}
	}
	pub mod parachain_info {
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct ParachainId;
			impl ::subxt::StorageEntry for ParachainId {
				const PALLET: &'static str = "ParachainInfo";
				const STORAGE: &'static str = "ParachainId";
				type Value = runtime_types::polkadot_parachain::primitives::Id;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn parachain_id(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::polkadot_parachain::primitives::Id,
					::subxt::Error,
				> {
					let entry = ParachainId;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod authorship {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetUncles {
				pub new_uncles: ::std::vec::Vec<
					runtime_types::sp_runtime::generic::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
			}
			impl ::subxt::Call for SetUncles {
				const PALLET: &'static str = "Authorship";
				const FUNCTION: &'static str = "set_uncles";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn set_uncles(
					&self,
					new_uncles: ::std::vec::Vec<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				) -> ::subxt::SubmittableExtrinsic<T, SetUncles> {
					let call = SetUncles { new_uncles };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Uncles;
			impl ::subxt::StorageEntry for Uncles {
				const PALLET: &'static str = "Authorship";
				const STORAGE: &'static str = "Uncles";
				type Value = ::std::vec::Vec<
					runtime_types::pallet_authorship::UncleEntryItem<
						::core::primitive::u32,
						::subxt::sp_core::H256,
						::subxt::sp_core::crypto::AccountId32,
					>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Author;
			impl ::subxt::StorageEntry for Author {
				const PALLET: &'static str = "Authorship";
				const STORAGE: &'static str = "Author";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct DidSetUncles;
			impl ::subxt::StorageEntry for DidSetUncles {
				const PALLET: &'static str = "Authorship";
				const STORAGE: &'static str = "DidSetUncles";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn uncles(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::pallet_authorship::UncleEntryItem<
							::core::primitive::u32,
							::subxt::sp_core::H256,
							::subxt::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Uncles;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn author(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = Author;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn did_set_uncles(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = DidSetUncles;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod collator_selection {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetInvulnerables {
				pub new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::Call for SetInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "set_invulnerables";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetDesiredCandidates {
				pub max: ::core::primitive::u32,
			}
			impl ::subxt::Call for SetDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "set_desired_candidates";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetCandidacyBond {
				pub bond: ::core::primitive::u128,
			}
			impl ::subxt::Call for SetCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "set_candidacy_bond";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RegisterAsCandidate {}
			impl ::subxt::Call for RegisterAsCandidate {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "register_as_candidate";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct LeaveIntent {}
			impl ::subxt::Call for LeaveIntent {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "leave_intent";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
				) -> ::subxt::SubmittableExtrinsic<T, SetInvulnerables> {
					let call = SetInvulnerables { new };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, SetDesiredCandidates> {
					let call = SetDesiredCandidates { max };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, SetCandidacyBond> {
					let call = SetCandidacyBond { bond };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn register_as_candidate(
					&self,
				) -> ::subxt::SubmittableExtrinsic<T, RegisterAsCandidate> {
					let call = RegisterAsCandidate {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn leave_intent(&self) -> ::subxt::SubmittableExtrinsic<T, LeaveIntent> {
					let call = LeaveIntent {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NewInvulnerables(pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>);
			impl ::subxt::Event for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NewDesiredCandidates(pub ::core::primitive::u32);
			impl ::subxt::Event for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NewCandidacyBond(pub ::core::primitive::u128);
			impl ::subxt::Event for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CandidateAdded(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CandidateRemoved(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for CandidateRemoved {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Invulnerables;
			impl ::subxt::StorageEntry for Invulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const STORAGE: &'static str = "Invulnerables";
				type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Candidates;
			impl ::subxt::StorageEntry for Candidates {
				const PALLET: &'static str = "CollatorSelection";
				const STORAGE: &'static str = "Candidates";
				type Value = ::std::vec::Vec<
					runtime_types::pallet_collator_selection::pallet::CandidateInfo<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct LastAuthoredBlock(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for LastAuthoredBlock {
				const PALLET: &'static str = "CollatorSelection";
				const STORAGE: &'static str = "LastAuthoredBlock";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct DesiredCandidates;
			impl ::subxt::StorageEntry for DesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const STORAGE: &'static str = "DesiredCandidates";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct CandidacyBond;
			impl ::subxt::StorageEntry for CandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const STORAGE: &'static str = "CandidacyBond";
				type Value = ::core::primitive::u128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn invulnerables(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = Invulnerables;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn candidates(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::pallet_collator_selection::pallet::CandidateInfo<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Candidates;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn last_authored_block(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = LastAuthoredBlock(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn last_authored_block_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, LastAuthoredBlock>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
				pub async fn desired_candidates(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = DesiredCandidates;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn candidacy_bond(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error> {
					let entry = CandidacyBond;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod session {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetKeys {
				pub keys: runtime_types::picasso_runtime::opaque::SessionKeys,
				pub proof: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SetKeys {
				const PALLET: &'static str = "Session";
				const FUNCTION: &'static str = "set_keys";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PurgeKeys {}
			impl ::subxt::Call for PurgeKeys {
				const PALLET: &'static str = "Session";
				const FUNCTION: &'static str = "purge_keys";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn set_keys(
					&self,
					keys: runtime_types::picasso_runtime::opaque::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, SetKeys> {
					let call = SetKeys { keys, proof };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn purge_keys(&self) -> ::subxt::SubmittableExtrinsic<T, PurgeKeys> {
					let call = PurgeKeys {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NewSession(pub ::core::primitive::u32);
			impl ::subxt::Event for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Validators;
			impl ::subxt::StorageEntry for Validators {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "Validators";
				type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct CurrentIndex;
			impl ::subxt::StorageEntry for CurrentIndex {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "CurrentIndex";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct QueuedChanged;
			impl ::subxt::StorageEntry for QueuedChanged {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "QueuedChanged";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct QueuedKeys;
			impl ::subxt::StorageEntry for QueuedKeys {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "QueuedKeys";
				type Value = ::std::vec::Vec<(
					::subxt::sp_core::crypto::AccountId32,
					runtime_types::picasso_runtime::opaque::SessionKeys,
				)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct DisabledValidators;
			impl ::subxt::StorageEntry for DisabledValidators {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "DisabledValidators";
				type Value = ::std::vec::Vec<::core::primitive::u32>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for NextKeys {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "NextKeys";
				type Value = runtime_types::picasso_runtime::opaque::SessionKeys;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct KeyOwner(
				runtime_types::sp_core::crypto::KeyTypeId,
				::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::StorageEntry for KeyOwner {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "KeyOwner";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn validators(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = Validators;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn current_index(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = CurrentIndex;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn queued_changed(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = QueuedChanged;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn queued_keys(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						::subxt::sp_core::crypto::AccountId32,
						runtime_types::picasso_runtime::opaque::SessionKeys,
					)>,
					::subxt::Error,
				> {
					let entry = QueuedKeys;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn disabled_validators(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u32>, ::subxt::Error>
				{
					let entry = DisabledValidators;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn next_keys(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::picasso_runtime::opaque::SessionKeys>,
					::subxt::Error,
				> {
					let entry = NextKeys(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn next_keys_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextKeys>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn key_owner(
					&self,
					_0: runtime_types::sp_core::crypto::KeyTypeId,
					_1: ::std::vec::Vec<::core::primitive::u8>,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = KeyOwner(_0, _1);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn key_owner_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, KeyOwner>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod aura {
		use super::runtime_types;
		pub mod storage {
			use super::runtime_types;
			pub struct Authorities;
			impl ::subxt::StorageEntry for Authorities {
				const PALLET: &'static str = "Aura";
				const STORAGE: &'static str = "Authorities";
				type Value =
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct CurrentSlot;
			impl ::subxt::StorageEntry for CurrentSlot {
				const PALLET: &'static str = "Aura";
				const STORAGE: &'static str = "CurrentSlot";
				type Value = runtime_types::sp_consensus_slots::Slot;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn authorities(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::Error,
				> {
					let entry = Authorities;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn current_slot(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<runtime_types::sp_consensus_slots::Slot, ::subxt::Error>
				{
					let entry = CurrentSlot;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod aura_ext {
		use super::runtime_types;
	}
	pub mod council {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
				pub prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			impl ::subxt::Call for SetMembers {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "set_members";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Execute {
				pub proposal: runtime_types::picasso_runtime::Call,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for Execute {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "execute";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: runtime_types::picasso_runtime::Call,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for Propose {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "propose";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Vote {
				pub proposal: ::subxt::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			impl ::subxt::Call for Vote {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "vote";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Close {
				pub proposal_hash: ::subxt::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: ::core::primitive::u64,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for Close {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "close";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for DisapproveProposal {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "disapprove_proposal";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, SetMembers> {
					let call = SetMembers { new_members, prime, old_count };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn execute(
					&self,
					proposal: runtime_types::picasso_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Execute> {
					let call = Execute { proposal, length_bound };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::picasso_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Propose> {
					let call = Propose { threshold, proposal, length_bound };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn vote(
					&self,
					proposal: ::subxt::sp_core::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::SubmittableExtrinsic<T, Vote> {
					let call = Vote { proposal, index, approve };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn close(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: ::core::primitive::u64,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Close> {
					let call = Close { proposal_hash, index, proposal_weight_bound, length_bound };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> ::subxt::SubmittableExtrinsic<T, DisapproveProposal> {
					let call = DisapproveProposal { proposal_hash };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Proposed(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u32,
				pub ::subxt::sp_core::H256,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for Proposed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Voted(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::H256,
				pub ::core::primitive::bool,
				pub ::core::primitive::u32,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for Voted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Voted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Approved(pub ::subxt::sp_core::H256);
			impl ::subxt::Event for Approved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Approved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Disapproved(pub ::subxt::sp_core::H256);
			impl ::subxt::Event for Disapproved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Executed(
				pub ::subxt::sp_core::H256,
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::Event for Executed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Executed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct MemberExecuted(
				pub ::subxt::sp_core::H256,
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::Event for MemberExecuted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Closed(
				pub ::subxt::sp_core::H256,
				pub ::core::primitive::u32,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for Closed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Proposals;
			impl ::subxt::StorageEntry for Proposals {
				const PALLET: &'static str = "Council";
				const STORAGE: &'static str = "Proposals";
				type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
					::subxt::sp_core::H256,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ProposalOf(pub ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for ProposalOf {
				const PALLET: &'static str = "Council";
				const STORAGE: &'static str = "ProposalOf";
				type Value = runtime_types::picasso_runtime::Call;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Identity,
					)])
				}
			}
			pub struct Voting(pub ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Voting {
				const PALLET: &'static str = "Council";
				const STORAGE: &'static str = "Voting";
				type Value = runtime_types::pallet_collective::Votes<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Identity,
					)])
				}
			}
			pub struct ProposalCount;
			impl ::subxt::StorageEntry for ProposalCount {
				const PALLET: &'static str = "Council";
				const STORAGE: &'static str = "ProposalCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Members;
			impl ::subxt::StorageEntry for Members {
				const PALLET: &'static str = "Council";
				const STORAGE: &'static str = "Members";
				type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Prime;
			impl ::subxt::StorageEntry for Prime {
				const PALLET: &'static str = "Council";
				const STORAGE: &'static str = "Prime";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn proposals(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						::subxt::sp_core::H256,
					>,
					::subxt::Error,
				> {
					let entry = Proposals;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn proposal_of(
					&self,
					_0: ::subxt::sp_core::H256,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::picasso_runtime::Call>,
					::subxt::Error,
				> {
					let entry = ProposalOf(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn proposal_of_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, ProposalOf>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn voting(
					&self,
					_0: ::subxt::sp_core::H256,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_collective::Votes<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Voting(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn voting_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn proposal_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = ProposalCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn members(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = Members;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn prime(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = Prime;
					self.client.storage().fetch(&entry, hash).await
				}
			}
		}
	}
	pub mod council_membership {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AddMember {
				pub who: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for AddMember {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "add_member";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RemoveMember {
				pub who: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for RemoveMember {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "remove_member";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SwapMember {
				pub remove: ::subxt::sp_core::crypto::AccountId32,
				pub add: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for SwapMember {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "swap_member";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::Call for ResetMembers {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "reset_members";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ChangeKey {
				pub new: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for ChangeKey {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "change_key";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetPrime {
				pub who: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for SetPrime {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "set_prime";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ClearPrime {}
			impl ::subxt::Call for ClearPrime {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "clear_prime";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn add_member(
					&self,
					who: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, AddMember> {
					let call = AddMember { who };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn remove_member(
					&self,
					who: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, RemoveMember> {
					let call = RemoveMember { who };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn swap_member(
					&self,
					remove: ::subxt::sp_core::crypto::AccountId32,
					add: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, SwapMember> {
					let call = SwapMember { remove, add };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
				) -> ::subxt::SubmittableExtrinsic<T, ResetMembers> {
					let call = ResetMembers { members };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn change_key(
					&self,
					new: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, ChangeKey> {
					let call = ChangeKey { new };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_prime(
					&self,
					who: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, SetPrime> {
					let call = SetPrime { who };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn clear_prime(&self) -> ::subxt::SubmittableExtrinsic<T, ClearPrime> {
					let call = ClearPrime {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct MemberAdded {}
			impl ::subxt::Event for MemberAdded {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct MemberRemoved {}
			impl ::subxt::Event for MemberRemoved {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct MembersSwapped {}
			impl ::subxt::Event for MembersSwapped {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct MembersReset {}
			impl ::subxt::Event for MembersReset {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct KeyChanged {}
			impl ::subxt::Event for KeyChanged {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Dummy {}
			impl ::subxt::Event for Dummy {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Members;
			impl ::subxt::StorageEntry for Members {
				const PALLET: &'static str = "CouncilMembership";
				const STORAGE: &'static str = "Members";
				type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Prime;
			impl ::subxt::StorageEntry for Prime {
				const PALLET: &'static str = "CouncilMembership";
				const STORAGE: &'static str = "Prime";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn members(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = Members;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn prime(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = Prime;
					self.client.storage().fetch(&entry, hash).await
				}
			}
		}
	}
	pub mod treasury {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ProposeSpend {
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub beneficiary: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
			}
			impl ::subxt::Call for ProposeSpend {
				const PALLET: &'static str = "Treasury";
				const FUNCTION: &'static str = "propose_spend";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RejectProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			impl ::subxt::Call for RejectProposal {
				const PALLET: &'static str = "Treasury";
				const FUNCTION: &'static str = "reject_proposal";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ApproveProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			impl ::subxt::Call for ApproveProposal {
				const PALLET: &'static str = "Treasury";
				const FUNCTION: &'static str = "approve_proposal";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn propose_spend(
					&self,
					value: ::core::primitive::u128,
					beneficiary: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> ::subxt::SubmittableExtrinsic<T, ProposeSpend> {
					let call = ProposeSpend { value, beneficiary };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn reject_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, RejectProposal> {
					let call = RejectProposal { proposal_id };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn approve_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, ApproveProposal> {
					let call = ApproveProposal { proposal_id };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_treasury::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Proposed(pub ::core::primitive::u32);
			impl ::subxt::Event for Proposed {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Spending(pub ::core::primitive::u128);
			impl ::subxt::Event for Spending {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Spending";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Awarded(
				pub ::core::primitive::u32,
				pub ::core::primitive::u128,
				pub ::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::Event for Awarded {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Awarded";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Rejected(pub ::core::primitive::u32, pub ::core::primitive::u128);
			impl ::subxt::Event for Rejected {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rejected";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Burnt(pub ::core::primitive::u128);
			impl ::subxt::Event for Burnt {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Burnt";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Rollover(pub ::core::primitive::u128);
			impl ::subxt::Event for Rollover {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rollover";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Deposit(pub ::core::primitive::u128);
			impl ::subxt::Event for Deposit {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Deposit";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct ProposalCount;
			impl ::subxt::StorageEntry for ProposalCount {
				const PALLET: &'static str = "Treasury";
				const STORAGE: &'static str = "ProposalCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Proposals(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for Proposals {
				const PALLET: &'static str = "Treasury";
				const STORAGE: &'static str = "Proposals";
				type Value = runtime_types::pallet_treasury::Proposal<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u128,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct Approvals;
			impl ::subxt::StorageEntry for Approvals {
				const PALLET: &'static str = "Treasury";
				const STORAGE: &'static str = "Approvals";
				type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn proposal_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = ProposalCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn proposals(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_treasury::Proposal<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Proposals(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn proposals_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proposals>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn approvals(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						::core::primitive::u32,
					>,
					::subxt::Error,
				> {
					let entry = Approvals;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod democracy {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Propose {
				pub proposal_hash: ::subxt::sp_core::H256,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::Call for Propose {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "propose";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Second {
				#[codec(compact)]
				pub proposal: ::core::primitive::u32,
				#[codec(compact)]
				pub seconds_upper_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for Second {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "second";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Vote {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
				pub vote:
					runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
			}
			impl ::subxt::Call for Vote {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "vote";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct EmergencyCancel {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Call for EmergencyCancel {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "emergency_cancel";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExternalPropose {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for ExternalPropose {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "external_propose";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExternalProposeMajority {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for ExternalProposeMajority {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "external_propose_majority";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExternalProposeDefault {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for ExternalProposeDefault {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "external_propose_default";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct FastTrack {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub voting_period: ::core::primitive::u32,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::Call for FastTrack {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "fast_track";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct VetoExternal {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for VetoExternal {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "veto_external";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CancelReferendum {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Call for CancelReferendum {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "cancel_referendum";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CancelQueued {
				pub which: ::core::primitive::u32,
			}
			impl ::subxt::Call for CancelQueued {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "cancel_queued";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Delegate {
				pub to: ::subxt::sp_core::crypto::AccountId32,
				pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::Call for Delegate {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "delegate";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Undelegate {}
			impl ::subxt::Call for Undelegate {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "undelegate";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ClearPublicProposals {}
			impl ::subxt::Call for ClearPublicProposals {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "clear_public_proposals";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotePreimage {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NotePreimage {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_preimage";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotePreimageOperational {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NotePreimageOperational {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_preimage_operational";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NoteImminentPreimage {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NoteImminentPreimage {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_imminent_preimage";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NoteImminentPreimageOperational {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NoteImminentPreimageOperational {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_imminent_preimage_operational";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ReapPreimage {
				pub proposal_hash: ::subxt::sp_core::H256,
				#[codec(compact)]
				pub proposal_len_upper_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for ReapPreimage {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "reap_preimage";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Unlock {
				pub target: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for Unlock {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "unlock";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RemoveVote {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for RemoveVote {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "remove_vote";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RemoveOtherVote {
				pub target: ::subxt::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for RemoveOtherVote {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "remove_other_vote";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct EnactProposal {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for EnactProposal {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "enact_proposal";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Blacklist {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::Call for Blacklist {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "blacklist";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CancelProposal {
				#[codec(compact)]
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::Call for CancelProposal {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "cancel_proposal";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn propose(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					value: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Propose> {
					let call = Propose { proposal_hash, value };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn second(
					&self,
					proposal: ::core::primitive::u32,
					seconds_upper_bound: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Second> {
					let call = Second { proposal, seconds_upper_bound };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn vote(
					&self,
					ref_index: ::core::primitive::u32,
					vote: runtime_types::pallet_democracy::vote::AccountVote<
						::core::primitive::u128,
					>,
				) -> ::subxt::SubmittableExtrinsic<T, Vote> {
					let call = Vote { ref_index, vote };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn emergency_cancel(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, EmergencyCancel> {
					let call = EmergencyCancel { ref_index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn external_propose(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> ::subxt::SubmittableExtrinsic<T, ExternalPropose> {
					let call = ExternalPropose { proposal_hash };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn external_propose_majority(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> ::subxt::SubmittableExtrinsic<T, ExternalProposeMajority> {
					let call = ExternalProposeMajority { proposal_hash };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn external_propose_default(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> ::subxt::SubmittableExtrinsic<T, ExternalProposeDefault> {
					let call = ExternalProposeDefault { proposal_hash };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn fast_track(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					voting_period: ::core::primitive::u32,
					delay: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, FastTrack> {
					let call = FastTrack { proposal_hash, voting_period, delay };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn veto_external(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> ::subxt::SubmittableExtrinsic<T, VetoExternal> {
					let call = VetoExternal { proposal_hash };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn cancel_referendum(
					&self,
					ref_index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, CancelReferendum> {
					let call = CancelReferendum { ref_index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn cancel_queued(
					&self,
					which: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, CancelQueued> {
					let call = CancelQueued { which };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn delegate(
					&self,
					to: ::subxt::sp_core::crypto::AccountId32,
					conviction: runtime_types::pallet_democracy::conviction::Conviction,
					balance: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Delegate> {
					let call = Delegate { to, conviction, balance };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn undelegate(&self) -> ::subxt::SubmittableExtrinsic<T, Undelegate> {
					let call = Undelegate {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn clear_public_proposals(
					&self,
				) -> ::subxt::SubmittableExtrinsic<T, ClearPublicProposals> {
					let call = ClearPublicProposals {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn note_preimage(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, NotePreimage> {
					let call = NotePreimage { encoded_proposal };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn note_preimage_operational(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, NotePreimageOperational> {
					let call = NotePreimageOperational { encoded_proposal };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn note_imminent_preimage(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, NoteImminentPreimage> {
					let call = NoteImminentPreimage { encoded_proposal };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn note_imminent_preimage_operational(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, NoteImminentPreimageOperational> {
					let call = NoteImminentPreimageOperational { encoded_proposal };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn reap_preimage(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					proposal_len_upper_bound: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, ReapPreimage> {
					let call = ReapPreimage { proposal_hash, proposal_len_upper_bound };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn unlock(
					&self,
					target: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, Unlock> {
					let call = Unlock { target };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn remove_vote(
					&self,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, RemoveVote> {
					let call = RemoveVote { index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn remove_other_vote(
					&self,
					target: ::subxt::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, RemoveOtherVote> {
					let call = RemoveOtherVote { target, index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn enact_proposal(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, EnactProposal> {
					let call = EnactProposal { proposal_hash, index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn blacklist(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::SubmittableExtrinsic<T, Blacklist> {
					let call = Blacklist { proposal_hash, maybe_ref_index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn cancel_proposal(
					&self,
					prop_index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, CancelProposal> {
					let call = CancelProposal { prop_index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_democracy::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Proposed(pub ::core::primitive::u32, pub ::core::primitive::u128);
			impl ::subxt::Event for Proposed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Tabled(
				pub ::core::primitive::u32,
				pub ::core::primitive::u128,
				pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
			);
			impl ::subxt::Event for Tabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Tabled";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExternalTabled {}
			impl ::subxt::Event for ExternalTabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ExternalTabled";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Started(
				pub ::core::primitive::u32,
				pub runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
			);
			impl ::subxt::Event for Started {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Started";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Passed(pub ::core::primitive::u32);
			impl ::subxt::Event for Passed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Passed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotPassed(pub ::core::primitive::u32);
			impl ::subxt::Event for NotPassed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "NotPassed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Cancelled(pub ::core::primitive::u32);
			impl ::subxt::Event for Cancelled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Cancelled";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Executed(
				pub ::core::primitive::u32,
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::Event for Executed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Executed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Delegated(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::Event for Delegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Delegated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Undelegated(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for Undelegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Undelegated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Vetoed(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::H256,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for Vetoed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Vetoed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PreimageNoted(
				pub ::subxt::sp_core::H256,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for PreimageNoted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageNoted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PreimageUsed(
				pub ::subxt::sp_core::H256,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for PreimageUsed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageUsed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PreimageInvalid(pub ::subxt::sp_core::H256, pub ::core::primitive::u32);
			impl ::subxt::Event for PreimageInvalid {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageInvalid";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PreimageMissing(pub ::subxt::sp_core::H256, pub ::core::primitive::u32);
			impl ::subxt::Event for PreimageMissing {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageMissing";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PreimageReaped(
				pub ::subxt::sp_core::H256,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub ::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::Event for PreimageReaped {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageReaped";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Blacklisted(pub ::subxt::sp_core::H256);
			impl ::subxt::Event for Blacklisted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Blacklisted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct PublicPropCount;
			impl ::subxt::StorageEntry for PublicPropCount {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "PublicPropCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct PublicProps;
			impl ::subxt::StorageEntry for PublicProps {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "PublicProps";
				type Value = ::std::vec::Vec<(
					::core::primitive::u32,
					::subxt::sp_core::H256,
					::subxt::sp_core::crypto::AccountId32,
				)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct DepositOf(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for DepositOf {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "DepositOf";
				type Value = (
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::core::primitive::u128,
				);
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct Preimages(pub ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Preimages {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "Preimages";
				type Value = runtime_types::pallet_democracy::PreimageStatus<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u128,
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Identity,
					)])
				}
			}
			pub struct ReferendumCount;
			impl ::subxt::StorageEntry for ReferendumCount {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "ReferendumCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct LowestUnbaked;
			impl ::subxt::StorageEntry for LowestUnbaked {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "LowestUnbaked";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ReferendumInfoOf(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for ReferendumInfoOf {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "ReferendumInfoOf";
				type Value = runtime_types::pallet_democracy::types::ReferendumInfo<
					::core::primitive::u32,
					::subxt::sp_core::H256,
					::core::primitive::u128,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct VotingOf(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for VotingOf {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "VotingOf";
				type Value = runtime_types::pallet_democracy::vote::Voting<
					::core::primitive::u128,
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Locks {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "Locks";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct LastTabledWasExternal;
			impl ::subxt::StorageEntry for LastTabledWasExternal {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "LastTabledWasExternal";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct NextExternal;
			impl ::subxt::StorageEntry for NextExternal {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "NextExternal";
				type Value = (
					::subxt::sp_core::H256,
					runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
				);
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Blacklist(pub ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Blacklist {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "Blacklist";
				type Value = (
					::core::primitive::u32,
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
				);
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Identity,
					)])
				}
			}
			pub struct Cancellations(pub ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Cancellations {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "Cancellations";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Identity,
					)])
				}
			}
			pub struct StorageVersion;
			impl ::subxt::StorageEntry for StorageVersion {
				const PALLET: &'static str = "Democracy";
				const STORAGE: &'static str = "StorageVersion";
				type Value = runtime_types::pallet_democracy::Releases;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn public_prop_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = PublicPropCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn public_props(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						::core::primitive::u32,
						::subxt::sp_core::H256,
						::subxt::sp_core::crypto::AccountId32,
					)>,
					::subxt::Error,
				> {
					let entry = PublicProps;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn deposit_of(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
						::core::primitive::u128,
					)>,
					::subxt::Error,
				> {
					let entry = DepositOf(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn deposit_of_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, DepositOf>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn preimages(
					&self,
					_0: ::subxt::sp_core::H256,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_democracy::PreimageStatus<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Preimages(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn preimages_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Preimages>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn referendum_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = ReferendumCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn lowest_unbaked(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = LowestUnbaked;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn referendum_info_of(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_democracy::types::ReferendumInfo<
							::core::primitive::u32,
							::subxt::sp_core::H256,
							::core::primitive::u128,
						>,
					>,
					::subxt::Error,
				> {
					let entry = ReferendumInfoOf(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn referendum_info_of_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, ReferendumInfoOf>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
				pub async fn voting_of(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_democracy::vote::Voting<
						::core::primitive::u128,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					::subxt::Error,
				> {
					let entry = VotingOf(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn voting_of_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, VotingOf>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn locks(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::Error,
				> {
					let entry = Locks(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn locks_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn last_tabled_was_external(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = LastTabledWasExternal;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn next_external(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::subxt::sp_core::H256,
						runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					)>,
					::subxt::Error,
				> {
					let entry = NextExternal;
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn blacklist(
					&self,
					_0: ::subxt::sp_core::H256,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::core::primitive::u32,
						::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					)>,
					::subxt::Error,
				> {
					let entry = Blacklist(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn blacklist_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Blacklist>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn cancellations(
					&self,
					_0: ::subxt::sp_core::H256,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = Cancellations(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn cancellations_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Cancellations>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
				pub async fn storage_version(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::pallet_democracy::Releases>,
					::subxt::Error,
				> {
					let entry = StorageVersion;
					self.client.storage().fetch(&entry, hash).await
				}
			}
		}
	}
	pub mod scheduler {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Schedule {
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: runtime_types::picasso_runtime::Call,
			}
			impl ::subxt::Call for Schedule {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Cancel {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Cancel {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "cancel";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ScheduleNamed {
				pub id: ::std::vec::Vec<::core::primitive::u8>,
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: runtime_types::picasso_runtime::Call,
			}
			impl ::subxt::Call for ScheduleNamed {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule_named";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CancelNamed {
				pub id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for CancelNamed {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "cancel_named";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ScheduleAfter {
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: runtime_types::picasso_runtime::Call,
			}
			impl ::subxt::Call for ScheduleAfter {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule_after";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ScheduleNamedAfter {
				pub id: ::std::vec::Vec<::core::primitive::u8>,
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: runtime_types::picasso_runtime::Call,
			}
			impl ::subxt::Call for ScheduleNamedAfter {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule_named_after";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn schedule(
					&self,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::Call,
				) -> ::subxt::SubmittableExtrinsic<T, Schedule> {
					let call = Schedule { when, maybe_periodic, priority, call };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn cancel(
					&self,
					when: ::core::primitive::u32,
					index: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, Cancel> {
					let call = Cancel { when, index };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn schedule_named(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::Call,
				) -> ::subxt::SubmittableExtrinsic<T, ScheduleNamed> {
					let call = ScheduleNamed { id, when, maybe_periodic, priority, call };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn cancel_named(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, CancelNamed> {
					let call = CancelNamed { id };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn schedule_after(
					&self,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::Call,
				) -> ::subxt::SubmittableExtrinsic<T, ScheduleAfter> {
					let call = ScheduleAfter { after, maybe_periodic, priority, call };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn schedule_named_after(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::picasso_runtime::Call,
				) -> ::subxt::SubmittableExtrinsic<T, ScheduleNamedAfter> {
					let call = ScheduleNamedAfter { id, after, maybe_periodic, priority, call };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_scheduler::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Scheduled(pub ::core::primitive::u32, pub ::core::primitive::u32);
			impl ::subxt::Event for Scheduled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Scheduled";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Canceled(pub ::core::primitive::u32, pub ::core::primitive::u32);
			impl ::subxt::Event for Canceled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Canceled";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Dispatched(
				pub (::core::primitive::u32, ::core::primitive::u32),
				pub ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::Event for Dispatched {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Dispatched";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Agenda(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for Agenda {
				const PALLET: &'static str = "Scheduler";
				const STORAGE: &'static str = "Agenda";
				type Value = ::std::vec::Vec<
					::core::option::Option<
						runtime_types::pallet_scheduler::ScheduledV2<
							runtime_types::picasso_runtime::Call,
							::core::primitive::u32,
							runtime_types::picasso_runtime::OriginCaller,
							::subxt::sp_core::crypto::AccountId32,
						>,
					>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct Lookup(pub ::std::vec::Vec<::core::primitive::u8>);
			impl ::subxt::StorageEntry for Lookup {
				const PALLET: &'static str = "Scheduler";
				const STORAGE: &'static str = "Lookup";
				type Value = (::core::primitive::u32, ::core::primitive::u32);
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct StorageVersion;
			impl ::subxt::StorageEntry for StorageVersion {
				const PALLET: &'static str = "Scheduler";
				const STORAGE: &'static str = "StorageVersion";
				type Value = runtime_types::pallet_scheduler::Releases;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn agenda(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						::core::option::Option<
							runtime_types::pallet_scheduler::ScheduledV2<
								runtime_types::picasso_runtime::Call,
								::core::primitive::u32,
								runtime_types::picasso_runtime::OriginCaller,
								::subxt::sp_core::crypto::AccountId32,
							>,
						>,
					>,
					::subxt::Error,
				> {
					let entry = Agenda(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn agenda_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Agenda>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn lookup(
					&self,
					_0: ::std::vec::Vec<::core::primitive::u8>,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::Error,
				> {
					let entry = Lookup(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn lookup_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Lookup>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn storage_version(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<runtime_types::pallet_scheduler::Releases, ::subxt::Error>
				{
					let entry = StorageVersion;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod utility {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Batch {
				pub calls: ::std::vec::Vec<runtime_types::picasso_runtime::Call>,
			}
			impl ::subxt::Call for Batch {
				const PALLET: &'static str = "Utility";
				const FUNCTION: &'static str = "batch";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AsDerivative {
				pub index: ::core::primitive::u16,
				pub call: runtime_types::picasso_runtime::Call,
			}
			impl ::subxt::Call for AsDerivative {
				const PALLET: &'static str = "Utility";
				const FUNCTION: &'static str = "as_derivative";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BatchAll {
				pub calls: ::std::vec::Vec<runtime_types::picasso_runtime::Call>,
			}
			impl ::subxt::Call for BatchAll {
				const PALLET: &'static str = "Utility";
				const FUNCTION: &'static str = "batch_all";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::picasso_runtime::Call>,
				) -> ::subxt::SubmittableExtrinsic<T, Batch> {
					let call = Batch { calls };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::picasso_runtime::Call,
				) -> ::subxt::SubmittableExtrinsic<T, AsDerivative> {
					let call = AsDerivative { index, call };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::picasso_runtime::Call>,
				) -> ::subxt::SubmittableExtrinsic<T, BatchAll> {
					let call = BatchAll { calls };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_utility::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BatchInterrupted(
				pub ::core::primitive::u32,
				pub runtime_types::sp_runtime::DispatchError,
			);
			impl ::subxt::Event for BatchInterrupted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchInterrupted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BatchCompleted {}
			impl ::subxt::Event for BatchCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompleted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ItemCompleted {}
			impl ::subxt::Event for ItemCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemCompleted";
			}
		}
	}
	pub mod xcmp_queue {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Success(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for Success {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Success";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Fail(
				pub ::core::option::Option<::subxt::sp_core::H256>,
				pub runtime_types::xcm::v2::traits::Error,
			);
			impl ::subxt::Event for Fail {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Fail";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BadVersion(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for BadVersion {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadVersion";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BadFormat(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for BadFormat {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadFormat";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UpwardMessageSent(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for UpwardMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "UpwardMessageSent";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct XcmpMessageSent(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for XcmpMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "XcmpMessageSent";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct InboundXcmpStatus;
			impl ::subxt::StorageEntry for InboundXcmpStatus {
				const PALLET: &'static str = "XcmpQueue";
				const STORAGE: &'static str = "InboundXcmpStatus";
				type Value = ::std::vec::Vec<(
					runtime_types::polkadot_parachain::primitives::Id,
					runtime_types::cumulus_pallet_xcmp_queue::InboundStatus,
					::std::vec::Vec<(
						::core::primitive::u32,
						runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
					)>,
				)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct InboundXcmpMessages(
				runtime_types::polkadot_parachain::primitives::Id,
				::core::primitive::u32,
			);
			impl ::subxt::StorageEntry for InboundXcmpMessages {
				const PALLET: &'static str = "XcmpQueue";
				const STORAGE: &'static str = "InboundXcmpMessages";
				type Value = ::std::vec::Vec<::core::primitive::u8>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(
							&self.0,
							::subxt::StorageHasher::Blake2_128Concat,
						),
						::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
					])
				}
			}
			pub struct OutboundXcmpStatus;
			impl ::subxt::StorageEntry for OutboundXcmpStatus {
				const PALLET: &'static str = "XcmpQueue";
				const STORAGE: &'static str = "OutboundXcmpStatus";
				type Value = ::std::vec::Vec<(
					runtime_types::polkadot_parachain::primitives::Id,
					runtime_types::cumulus_pallet_xcmp_queue::OutboundStatus,
					::core::primitive::bool,
					::core::primitive::u16,
					::core::primitive::u16,
				)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct OutboundXcmpMessages(
				runtime_types::polkadot_parachain::primitives::Id,
				::core::primitive::u16,
			);
			impl ::subxt::StorageEntry for OutboundXcmpMessages {
				const PALLET: &'static str = "XcmpQueue";
				const STORAGE: &'static str = "OutboundXcmpMessages";
				type Value = ::std::vec::Vec<::core::primitive::u8>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(
							&self.0,
							::subxt::StorageHasher::Blake2_128Concat,
						),
						::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
					])
				}
			}
			pub struct SignalMessages(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::StorageEntry for SignalMessages {
				const PALLET: &'static str = "XcmpQueue";
				const STORAGE: &'static str = "SignalMessages";
				type Value = ::std::vec::Vec<::core::primitive::u8>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct QueueConfig;
			impl ::subxt::StorageEntry for QueueConfig {
				const PALLET: &'static str = "XcmpQueue";
				const STORAGE: &'static str = "QueueConfig";
				type Value = runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn inbound_xcmp_status(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::cumulus_pallet_xcmp_queue::InboundStatus,
						::std::vec::Vec<(
							::core::primitive::u32,
							runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
						)>,
					)>,
					::subxt::Error,
				> {
					let entry = InboundXcmpStatus;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn inbound_xcmp_messages(
					&self,
					_0: runtime_types::polkadot_parachain::primitives::Id,
					_1: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
				{
					let entry = InboundXcmpMessages(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn inbound_xcmp_messages_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, InboundXcmpMessages>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
				pub async fn outbound_xcmp_status(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::cumulus_pallet_xcmp_queue::OutboundStatus,
						::core::primitive::bool,
						::core::primitive::u16,
						::core::primitive::u16,
					)>,
					::subxt::Error,
				> {
					let entry = OutboundXcmpStatus;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn outbound_xcmp_messages(
					&self,
					_0: runtime_types::polkadot_parachain::primitives::Id,
					_1: ::core::primitive::u16,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
				{
					let entry = OutboundXcmpMessages(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn outbound_xcmp_messages_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, OutboundXcmpMessages>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
				pub async fn signal_messages(
					&self,
					_0: runtime_types::polkadot_parachain::primitives::Id,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::std::vec::Vec<::core::primitive::u8>, ::subxt::Error>
				{
					let entry = SignalMessages(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn signal_messages_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, SignalMessages>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
				pub async fn queue_config(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
					::subxt::Error,
				> {
					let entry = QueueConfig;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod polkadot_xcm {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Send {
				pub dest: runtime_types::xcm::VersionedMultiLocation,
				pub message: runtime_types::xcm::VersionedXcm,
			}
			impl ::subxt::Call for Send {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "send";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct TeleportAssets {
				pub dest: runtime_types::xcm::VersionedMultiLocation,
				pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
				pub assets: runtime_types::xcm::VersionedMultiAssets,
				pub fee_asset_item: ::core::primitive::u32,
			}
			impl ::subxt::Call for TeleportAssets {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "teleport_assets";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ReserveTransferAssets {
				pub dest: runtime_types::xcm::VersionedMultiLocation,
				pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
				pub assets: runtime_types::xcm::VersionedMultiAssets,
				pub fee_asset_item: ::core::primitive::u32,
			}
			impl ::subxt::Call for ReserveTransferAssets {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "reserve_transfer_assets";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Execute {
				pub message: runtime_types::xcm::VersionedXcm,
				pub max_weight: ::core::primitive::u64,
			}
			impl ::subxt::Call for Execute {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "execute";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceXcmVersion {
				pub location: runtime_types::xcm::v1::multilocation::MultiLocation,
				pub xcm_version: ::core::primitive::u32,
			}
			impl ::subxt::Call for ForceXcmVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "force_xcm_version";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceDefaultXcmVersion {
				pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::Call for ForceDefaultXcmVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "force_default_xcm_version";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceSubscribeVersionNotify {
				pub location: runtime_types::xcm::VersionedMultiLocation,
			}
			impl ::subxt::Call for ForceSubscribeVersionNotify {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "force_subscribe_version_notify";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceUnsubscribeVersionNotify {
				pub location: runtime_types::xcm::VersionedMultiLocation,
			}
			impl ::subxt::Call for ForceUnsubscribeVersionNotify {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "force_unsubscribe_version_notify";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct LimitedReserveTransferAssets {
				pub dest: runtime_types::xcm::VersionedMultiLocation,
				pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
				pub assets: runtime_types::xcm::VersionedMultiAssets,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			impl ::subxt::Call for LimitedReserveTransferAssets {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "limited_reserve_transfer_assets";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct LimitedTeleportAssets {
				pub dest: runtime_types::xcm::VersionedMultiLocation,
				pub beneficiary: runtime_types::xcm::VersionedMultiLocation,
				pub assets: runtime_types::xcm::VersionedMultiAssets,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			impl ::subxt::Call for LimitedTeleportAssets {
				const PALLET: &'static str = "PolkadotXcm";
				const FUNCTION: &'static str = "limited_teleport_assets";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::SubmittableExtrinsic<T, Send> {
					let call = Send { dest, message };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, TeleportAssets> {
					let call = TeleportAssets { dest, beneficiary, assets, fee_asset_item };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, ReserveTransferAssets> {
					let call = ReserveTransferAssets { dest, beneficiary, assets, fee_asset_item };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm,
					max_weight: ::core::primitive::u64,
				) -> ::subxt::SubmittableExtrinsic<T, Execute> {
					let call = Execute { message, max_weight };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_xcm_version(
					&self,
					location: runtime_types::xcm::v1::multilocation::MultiLocation,
					xcm_version: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, ForceXcmVersion> {
					let call = ForceXcmVersion { location, xcm_version };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::SubmittableExtrinsic<T, ForceDefaultXcmVersion> {
					let call = ForceDefaultXcmVersion { maybe_xcm_version };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::SubmittableExtrinsic<T, ForceSubscribeVersionNotify> {
					let call = ForceSubscribeVersionNotify { location };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::SubmittableExtrinsic<T, ForceUnsubscribeVersionNotify> {
					let call = ForceUnsubscribeVersionNotify { location };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::SubmittableExtrinsic<T, LimitedReserveTransferAssets> {
					let call = LimitedReserveTransferAssets {
						dest,
						beneficiary,
						assets,
						fee_asset_item,
						weight_limit,
					};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::SubmittableExtrinsic<T, LimitedTeleportAssets> {
					let call = LimitedTeleportAssets {
						dest,
						beneficiary,
						assets,
						fee_asset_item,
						weight_limit,
					};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
			impl ::subxt::Event for Attempted {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Sent(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v2::Xcm,
			);
			impl ::subxt::Event for Sent {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for UnexpectedResponse {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::Response,
			);
			impl ::subxt::Event for ResponseReady {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Notified(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::Event for Notified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "Notified";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotifyOverweight(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for NotifyOverweight {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotifyDispatchError(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::Event for NotifyDispatchError {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotifyDecodeFailed(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::Event for NotifyDecodeFailed {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct InvalidResponder(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
			);
			impl ::subxt::Event for InvalidResponder {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct InvalidResponderVersion(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for InvalidResponderVersion {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::Event for ResponseTaken {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AssetsTrapped(
				pub ::subxt::sp_core::H256,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::Event for AssetsTrapped {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for VersionChangeNotified {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for SupportedVersionChanged {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::traits::Error,
			);
			impl ::subxt::Event for NotifyTargetSendFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for NotifyTargetMigrationFail {
				const PALLET: &'static str = "PolkadotXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
		}
	}
	pub mod cumulus_xcm {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct InvalidFormat(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::Event for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::Event for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 8usize],
				pub runtime_types::xcm::v2::traits::Outcome,
			);
			impl ::subxt::Event for ExecutedDownward {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "ExecutedDownward";
			}
		}
	}
	pub mod dmp_queue {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: ::core::primitive::u64,
			}
			impl ::subxt::Call for ServiceOverweight {
				const PALLET: &'static str = "DmpQueue";
				const FUNCTION: &'static str = "service_overweight";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: ::core::primitive::u64,
				) -> ::subxt::SubmittableExtrinsic<T, ServiceOverweight> {
					let call = ServiceOverweight { index, weight_limit };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::Event for InvalidFormat {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::Event for UnsupportedVersion {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 32usize],
				pub runtime_types::xcm::v2::traits::Outcome,
			);
			impl ::subxt::Event for ExecutedDownward {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "ExecutedDownward";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct WeightExhausted(
				pub [::core::primitive::u8; 32usize],
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for WeightExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "WeightExhausted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct OverweightEnqueued(
				pub [::core::primitive::u8; 32usize],
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for OverweightEnqueued {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct OverweightServiced(pub ::core::primitive::u64, pub ::core::primitive::u64);
			impl ::subxt::Event for OverweightServiced {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightServiced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Configuration;
			impl ::subxt::StorageEntry for Configuration {
				const PALLET: &'static str = "DmpQueue";
				const STORAGE: &'static str = "Configuration";
				type Value = runtime_types::cumulus_pallet_dmp_queue::ConfigData;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct PageIndex;
			impl ::subxt::StorageEntry for PageIndex {
				const PALLET: &'static str = "DmpQueue";
				const STORAGE: &'static str = "PageIndex";
				type Value = runtime_types::cumulus_pallet_dmp_queue::PageIndexData;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Pages(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for Pages {
				const PALLET: &'static str = "DmpQueue";
				const STORAGE: &'static str = "Pages";
				type Value = ::std::vec::Vec<(
					::core::primitive::u32,
					::std::vec::Vec<::core::primitive::u8>,
				)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct Overweight(pub ::core::primitive::u64);
			impl ::subxt::StorageEntry for Overweight {
				const PALLET: &'static str = "DmpQueue";
				const STORAGE: &'static str = "Overweight";
				type Value = (::core::primitive::u32, ::std::vec::Vec<::core::primitive::u8>);
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn configuration(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_dmp_queue::ConfigData,
					::subxt::Error,
				> {
					let entry = Configuration;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn page_index(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
					::subxt::Error,
				> {
					let entry = PageIndex;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn pages(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::Error,
				> {
					let entry = Pages(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn pages_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Pages>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn overweight(
					&self,
					_0: ::core::primitive::u64,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::Error,
				> {
					let entry = Overweight(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn overweight_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Overweight>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod oracle {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AddAssetAndInfo {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
				pub threshold: runtime_types::sp_arithmetic::per_things::Percent,
				pub min_answers: ::core::primitive::u32,
				pub max_answers: ::core::primitive::u32,
			}
			impl ::subxt::Call for AddAssetAndInfo {
				const PALLET: &'static str = "Oracle";
				const FUNCTION: &'static str = "add_asset_and_info";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RequestPrice {
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::Call for RequestPrice {
				const PALLET: &'static str = "Oracle";
				const FUNCTION: &'static str = "request_price";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetSigner {
				pub signer: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for SetSigner {
				const PALLET: &'static str = "Oracle";
				const FUNCTION: &'static str = "set_signer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AddStake {
				pub stake: ::core::primitive::u128,
			}
			impl ::subxt::Call for AddStake {
				const PALLET: &'static str = "Oracle";
				const FUNCTION: &'static str = "add_stake";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RemoveStake {}
			impl ::subxt::Call for RemoveStake {
				const PALLET: &'static str = "Oracle";
				const FUNCTION: &'static str = "remove_stake";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ReclaimStake {}
			impl ::subxt::Call for ReclaimStake {
				const PALLET: &'static str = "Oracle";
				const FUNCTION: &'static str = "reclaim_stake";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SubmitPrice {
				pub price: ::core::primitive::u128,
				pub asset_id: runtime_types::primitives::currency::CurrencyId,
			}
			impl ::subxt::Call for SubmitPrice {
				const PALLET: &'static str = "Oracle";
				const FUNCTION: &'static str = "submit_price";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn add_asset_and_info(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
					threshold: runtime_types::sp_arithmetic::per_things::Percent,
					min_answers: ::core::primitive::u32,
					max_answers: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, AddAssetAndInfo> {
					let call = AddAssetAndInfo { asset_id, threshold, min_answers, max_answers };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn request_price(
					&self,
					asset_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::SubmittableExtrinsic<T, RequestPrice> {
					let call = RequestPrice { asset_id };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_signer(
					&self,
					signer: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, SetSigner> {
					let call = SetSigner { signer };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn add_stake(
					&self,
					stake: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, AddStake> {
					let call = AddStake { stake };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn remove_stake(&self) -> ::subxt::SubmittableExtrinsic<T, RemoveStake> {
					let call = RemoveStake {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn reclaim_stake(&self) -> ::subxt::SubmittableExtrinsic<T, ReclaimStake> {
					let call = ReclaimStake {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn submit_price(
					&self,
					price: ::core::primitive::u128,
					asset_id: runtime_types::primitives::currency::CurrencyId,
				) -> ::subxt::SubmittableExtrinsic<T, SubmitPrice> {
					let call = SubmitPrice { price, asset_id };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_oracle::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AssetInfoChange(
				pub runtime_types::primitives::currency::CurrencyId,
				pub runtime_types::sp_arithmetic::per_things::Percent,
				pub ::core::primitive::u32,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for AssetInfoChange {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "AssetInfoChange";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PriceRequested(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub runtime_types::primitives::currency::CurrencyId,
			);
			impl ::subxt::Event for PriceRequested {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "PriceRequested";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SignerSet(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::Event for SignerSet {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "SignerSet";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct StakeAdded(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for StakeAdded {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "StakeAdded";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct StakeRemoved(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for StakeRemoved {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "StakeRemoved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct StakeReclaimed(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for StakeReclaimed {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "StakeReclaimed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PriceSubmitted(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for PriceSubmitted {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "PriceSubmitted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UserSlashed(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for UserSlashed {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "UserSlashed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UserRewarded(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for UserRewarded {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "UserRewarded";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AnswerPruned(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for AnswerPruned {
				const PALLET: &'static str = "Oracle";
				const EVENT: &'static str = "AnswerPruned";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct AssetsCount;
			impl ::subxt::StorageEntry for AssetsCount {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "AssetsCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct SignerToController(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for SignerToController {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "SignerToController";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct ControllerToSigner(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for ControllerToSigner {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "ControllerToSigner";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct DeclaredWithdraws(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for DeclaredWithdraws {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "DeclaredWithdraws";
				type Value = runtime_types::pallet_oracle::pallet::Withdraw<
					::core::primitive::u128,
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct OracleStake(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for OracleStake {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "OracleStake";
				type Value = ::core::primitive::u128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct Prices(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::StorageEntry for Prices {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "Prices";
				type Value = runtime_types::pallet_oracle::pallet::Price<
					::core::primitive::u128,
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct PrePrices(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::StorageEntry for PrePrices {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "PrePrices";
				type Value = ::std::vec::Vec<
					runtime_types::pallet_oracle::pallet::PrePrice<
						::core::primitive::u128,
						::core::primitive::u32,
						::subxt::sp_core::crypto::AccountId32,
					>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct AssetsInfo(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::StorageEntry for AssetsInfo {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "AssetsInfo";
				type Value = runtime_types::pallet_oracle::pallet::AssetInfo<
					runtime_types::sp_arithmetic::per_things::Percent,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct Requested(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::StorageEntry for Requested {
				const PALLET: &'static str = "Oracle";
				const STORAGE: &'static str = "Requested";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn assets_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = AssetsCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn signer_to_controller(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = SignerToController(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn signer_to_controller_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, SignerToController>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
				pub async fn controller_to_signer(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::Error,
				> {
					let entry = ControllerToSigner(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn controller_to_signer_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, ControllerToSigner>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
				pub async fn declared_withdraws(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_oracle::pallet::Withdraw<
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					>,
					::subxt::Error,
				> {
					let entry = DeclaredWithdraws(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn declared_withdraws_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, DeclaredWithdraws>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
				pub async fn oracle_stake(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u128>,
					::subxt::Error,
				> {
					let entry = OracleStake(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn oracle_stake_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, OracleStake>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn prices(
					&self,
					_0: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_oracle::pallet::Price<
						::core::primitive::u128,
						::core::primitive::u32,
					>,
					::subxt::Error,
				> {
					let entry = Prices(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn prices_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Prices>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn pre_prices(
					&self,
					_0: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::pallet_oracle::pallet::PrePrice<
							::core::primitive::u128,
							::core::primitive::u32,
							::subxt::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::Error,
				> {
					let entry = PrePrices(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn pre_prices_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, PrePrices>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn assets_info(
					&self,
					_0: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_oracle::pallet::AssetInfo<
						runtime_types::sp_arithmetic::per_things::Percent,
					>,
					::subxt::Error,
				> {
					let entry = AssetsInfo(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn assets_info_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, AssetsInfo>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn requested(
					&self,
					_0: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::Error> {
					let entry = Requested(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn requested_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Requested>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod tokens {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Transfer {
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for Transfer {
				const PALLET: &'static str = "Tokens";
				const FUNCTION: &'static str = "transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct TransferAll {
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				pub keep_alive: ::core::primitive::bool,
			}
			impl ::subxt::Call for TransferAll {
				const PALLET: &'static str = "Tokens";
				const FUNCTION: &'static str = "transfer_all";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for TransferKeepAlive {
				const PALLET: &'static str = "Tokens";
				const FUNCTION: &'static str = "transfer_keep_alive";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForceTransfer {
				pub source: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub dest: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for ForceTransfer {
				const PALLET: &'static str = "Tokens";
				const FUNCTION: &'static str = "force_transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetBalance {
				pub who: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub currency_id: runtime_types::primitives::currency::CurrencyId,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			impl ::subxt::Call for SetBalance {
				const PALLET: &'static str = "Tokens";
				const FUNCTION: &'static str = "set_balance";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn transfer(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Transfer> {
					let call = Transfer { dest, currency_id, amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn transfer_all(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::SubmittableExtrinsic<T, TransferAll> {
					let call = TransferAll { dest, currency_id, keep_alive };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, TransferKeepAlive> {
					let call = TransferKeepAlive { dest, currency_id, amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn force_transfer(
					&self,
					source: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, ForceTransfer> {
					let call = ForceTransfer { source, dest, currency_id, amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_balance(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, SetBalance> {
					let call = SetBalance { who, currency_id, new_free, new_reserved };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::orml_tokens::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Endowed(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Endowed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct DustLost(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for DustLost {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Transfer(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Transfer {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Reserved(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Reserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Unreserved(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Unreserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BalanceSet(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for BalanceSet {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "BalanceSet";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct TotalIssuance(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::StorageEntry for TotalIssuance {
				const PALLET: &'static str = "Tokens";
				const STORAGE: &'static str = "TotalIssuance";
				type Value = ::core::primitive::u128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct Locks(
				::subxt::sp_core::crypto::AccountId32,
				runtime_types::primitives::currency::CurrencyId,
			);
			impl ::subxt::StorageEntry for Locks {
				const PALLET: &'static str = "Tokens";
				const STORAGE: &'static str = "Locks";
				type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
					runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(
							&self.0,
							::subxt::StorageHasher::Blake2_128Concat,
						),
						::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
					])
				}
			}
			pub struct Accounts(
				::subxt::sp_core::crypto::AccountId32,
				runtime_types::primitives::currency::CurrencyId,
			);
			impl ::subxt::StorageEntry for Accounts {
				const PALLET: &'static str = "Tokens";
				const STORAGE: &'static str = "Accounts";
				type Value = runtime_types::orml_tokens::AccountData<::core::primitive::u128>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(
							&self.0,
							::subxt::StorageHasher::Blake2_128Concat,
						),
						::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
					])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn total_issuance(
					&self,
					_0: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error> {
					let entry = TotalIssuance(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn total_issuance_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, TotalIssuance>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
				pub async fn locks(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					_1: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::Error,
				> {
					let entry = Locks(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn locks_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn accounts(
					&self,
					_0: ::subxt::sp_core::crypto::AccountId32,
					_1: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::orml_tokens::AccountData<::core::primitive::u128>,
					::subxt::Error,
				> {
					let entry = Accounts(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn accounts_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Accounts>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod factory {
		use super::runtime_types;
		pub type Event = runtime_types::pallet_currency_factory::pallet::Event;
		pub mod events {
			use super::runtime_types;
		}
		pub mod storage {
			use super::runtime_types;
			pub struct CurrencyCounter;
			impl ::subxt::StorageEntry for CurrencyCounter {
				const PALLET: &'static str = "Factory";
				const STORAGE: &'static str = "CurrencyCounter";
				type Value = runtime_types::primitives::currency::CurrencyId;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn currency_counter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::primitives::currency::CurrencyId,
					::subxt::Error,
				> {
					let entry = CurrencyCounter;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod vault {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Create {
				pub vault: runtime_types::composable_traits::vault::VaultConfig<
					::subxt::sp_core::crypto::AccountId32,
					runtime_types::primitives::currency::CurrencyId,
				>,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::Call for Create {
				const PALLET: &'static str = "Vault";
				const FUNCTION: &'static str = "create";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ClaimSurcharge {
				pub dest: ::core::primitive::u64,
				pub address: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::Call for ClaimSurcharge {
				const PALLET: &'static str = "Vault";
				const FUNCTION: &'static str = "claim_surcharge";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Deposit {
				pub vault: ::core::primitive::u64,
				pub asset_amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for Deposit {
				const PALLET: &'static str = "Vault";
				const FUNCTION: &'static str = "deposit";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Withdraw {
				pub vault: ::core::primitive::u64,
				pub lp_amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for Withdraw {
				const PALLET: &'static str = "Vault";
				const FUNCTION: &'static str = "withdraw";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct EmergencyShutdown {
				pub vault: ::core::primitive::u64,
			}
			impl ::subxt::Call for EmergencyShutdown {
				const PALLET: &'static str = "Vault";
				const FUNCTION: &'static str = "emergency_shutdown";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Start {
				pub vault: ::core::primitive::u64,
			}
			impl ::subxt::Call for Start {
				const PALLET: &'static str = "Vault";
				const FUNCTION: &'static str = "start";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn create(
					&self,
					vault: runtime_types::composable_traits::vault::VaultConfig<
						::subxt::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
					>,
					deposit: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Create> {
					let call = Create { vault, deposit };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn claim_surcharge(
					&self,
					dest: ::core::primitive::u64,
					address: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
				) -> ::subxt::SubmittableExtrinsic<T, ClaimSurcharge> {
					let call = ClaimSurcharge { dest, address };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn deposit(
					&self,
					vault: ::core::primitive::u64,
					asset_amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Deposit> {
					let call = Deposit { vault, asset_amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn withdraw(
					&self,
					vault: ::core::primitive::u64,
					lp_amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Withdraw> {
					let call = Withdraw { vault, lp_amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn emergency_shutdown(
					&self,
					vault: ::core::primitive::u64,
				) -> ::subxt::SubmittableExtrinsic<T, EmergencyShutdown> {
					let call = EmergencyShutdown { vault };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn start(
					&self,
					vault: ::core::primitive::u64,
				) -> ::subxt::SubmittableExtrinsic<T, Start> {
					let call = Start { vault };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_vault::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct VaultCreated {
				pub id: ::core::primitive::u64,
			}
			impl ::subxt::Event for VaultCreated {
				const PALLET: &'static str = "Vault";
				const EVENT: &'static str = "VaultCreated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Deposited {
				pub account: ::subxt::sp_core::crypto::AccountId32,
				pub asset_amount: ::core::primitive::u128,
				pub lp_amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Deposited {
				const PALLET: &'static str = "Vault";
				const EVENT: &'static str = "Deposited";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Withdrawn {
				pub account: ::subxt::sp_core::crypto::AccountId32,
				pub lp_amount: ::core::primitive::u128,
				pub asset_amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Withdrawn {
				const PALLET: &'static str = "Vault";
				const EVENT: &'static str = "Withdrawn";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct EmergencyShutdown {
				pub vault: ::core::primitive::u64,
			}
			impl ::subxt::Event for EmergencyShutdown {
				const PALLET: &'static str = "Vault";
				const EVENT: &'static str = "EmergencyShutdown";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct VaultStarted {
				pub vault: ::core::primitive::u64,
			}
			impl ::subxt::Event for VaultStarted {
				const PALLET: &'static str = "Vault";
				const EVENT: &'static str = "VaultStarted";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct VaultCount;
			impl ::subxt::StorageEntry for VaultCount {
				const PALLET: &'static str = "Vault";
				const STORAGE: &'static str = "VaultCount";
				type Value = ::core::primitive::u64;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Vaults(pub ::core::primitive::u64);
			impl ::subxt::StorageEntry for Vaults {
				const PALLET: &'static str = "Vault";
				const STORAGE: &'static str = "Vaults";
				type Value = runtime_types::pallet_vault::models::VaultInfo<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u128,
					runtime_types::primitives::currency::CurrencyId,
					::core::primitive::u32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct LpTokensToVaults(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::StorageEntry for LpTokensToVaults {
				const PALLET: &'static str = "Vault";
				const STORAGE: &'static str = "LpTokensToVaults";
				type Value = ::core::primitive::u64;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct Allocations(::core::primitive::u64, ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Allocations {
				const PALLET: &'static str = "Vault";
				const STORAGE: &'static str = "Allocations";
				type Value = runtime_types::sp_arithmetic::per_things::Perquintill;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(
							&self.0,
							::subxt::StorageHasher::Blake2_128Concat,
						),
						::subxt::StorageMapKey::new(
							&self.1,
							::subxt::StorageHasher::Blake2_128Concat,
						),
					])
				}
			}
			pub struct CapitalStructure(
				::core::primitive::u64,
				::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::StorageEntry for CapitalStructure {
				const PALLET: &'static str = "Vault";
				const STORAGE: &'static str = "CapitalStructure";
				type Value =
					runtime_types::pallet_vault::models::StrategyOverview<::core::primitive::u128>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(
							&self.0,
							::subxt::StorageHasher::Blake2_128Concat,
						),
						::subxt::StorageMapKey::new(
							&self.1,
							::subxt::StorageHasher::Blake2_128Concat,
						),
					])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn vault_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error> {
					let entry = VaultCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn vaults(
					&self,
					_0: ::core::primitive::u64,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_vault::models::VaultInfo<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u32,
					>,
					::subxt::Error,
				> {
					let entry = Vaults(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn vaults_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Vaults>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn lp_tokens_to_vaults(
					&self,
					_0: runtime_types::primitives::currency::CurrencyId,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error> {
					let entry = LpTokensToVaults(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn lp_tokens_to_vaults_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, LpTokensToVaults>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
				pub async fn allocations(
					&self,
					_0: ::core::primitive::u64,
					_1: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_arithmetic::per_things::Perquintill,
					::subxt::Error,
				> {
					let entry = Allocations(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn allocations_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Allocations>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn capital_structure(
					&self,
					_0: ::core::primitive::u64,
					_1: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_vault::models::StrategyOverview<::core::primitive::u128>,
					::subxt::Error,
				> {
					let entry = CapitalStructure(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn capital_structure_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, CapitalStructure>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod lending {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CreateNewMarket {
				pub borrow_asset_id: runtime_types::primitives::currency::CurrencyId,
				pub collateral_asset_id: runtime_types::primitives::currency::CurrencyId,
				pub reserved_factor: runtime_types::sp_arithmetic::per_things::Perquintill,
				pub collateral_factor: runtime_types::sp_arithmetic::fixed_point::FixedU128,
				pub under_collaterized_warn_percent:
					runtime_types::sp_arithmetic::per_things::Percent,
				pub interest_rate_model:
					runtime_types::composable_traits::rate_model::InterestRateModel,
			}
			impl ::subxt::Call for CreateNewMarket {
				const PALLET: &'static str = "Lending";
				const FUNCTION: &'static str = "create_new_market";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct DepositCollateral {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for DepositCollateral {
				const PALLET: &'static str = "Lending";
				const FUNCTION: &'static str = "deposit_collateral";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct WithdrawCollateral {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for WithdrawCollateral {
				const PALLET: &'static str = "Lending";
				const FUNCTION: &'static str = "withdraw_collateral";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Borrow {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub amount_to_borrow: ::core::primitive::u128,
			}
			impl ::subxt::Call for Borrow {
				const PALLET: &'static str = "Lending";
				const FUNCTION: &'static str = "borrow";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RepayBorrow {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub beneficiary: ::subxt::sp_core::crypto::AccountId32,
				pub repay_amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for RepayBorrow {
				const PALLET: &'static str = "Lending";
				const FUNCTION: &'static str = "repay_borrow";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Liquidate {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub borrower: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for Liquidate {
				const PALLET: &'static str = "Lending";
				const FUNCTION: &'static str = "liquidate";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn create_new_market(
					&self,
					borrow_asset_id: runtime_types::primitives::currency::CurrencyId,
					collateral_asset_id: runtime_types::primitives::currency::CurrencyId,
					reserved_factor: runtime_types::sp_arithmetic::per_things::Perquintill,
					collateral_factor: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					under_collaterized_warn_percent : runtime_types :: sp_arithmetic :: per_things :: Percent,
					interest_rate_model : runtime_types :: composable_traits :: rate_model :: InterestRateModel,
				) -> ::subxt::SubmittableExtrinsic<T, CreateNewMarket> {
					let call = CreateNewMarket {
						borrow_asset_id,
						collateral_asset_id,
						reserved_factor,
						collateral_factor,
						under_collaterized_warn_percent,
						interest_rate_model,
					};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn deposit_collateral(
					&self,
					market_id: runtime_types::pallet_lending::pallet::MarketIndex,
					amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, DepositCollateral> {
					let call = DepositCollateral { market_id, amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn withdraw_collateral(
					&self,
					market_id: runtime_types::pallet_lending::pallet::MarketIndex,
					amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, WithdrawCollateral> {
					let call = WithdrawCollateral { market_id, amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn borrow(
					&self,
					market_id: runtime_types::pallet_lending::pallet::MarketIndex,
					amount_to_borrow: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Borrow> {
					let call = Borrow { market_id, amount_to_borrow };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn repay_borrow(
					&self,
					market_id: runtime_types::pallet_lending::pallet::MarketIndex,
					beneficiary: ::subxt::sp_core::crypto::AccountId32,
					repay_amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, RepayBorrow> {
					let call = RepayBorrow { market_id, beneficiary, repay_amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn liquidate(
					&self,
					market_id: runtime_types::pallet_lending::pallet::MarketIndex,
					borrower: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, Liquidate> {
					let call = Liquidate { market_id, borrower };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_lending::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct NewMarketCreated {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub vault_id: ::core::primitive::u64,
				pub manager: ::subxt::sp_core::crypto::AccountId32,
				pub borrow_asset_id: runtime_types::primitives::currency::CurrencyId,
				pub collateral_asset_id: runtime_types::primitives::currency::CurrencyId,
				pub reserved_factor: runtime_types::sp_arithmetic::per_things::Perquintill,
				pub collateral_factor: runtime_types::sp_arithmetic::fixed_point::FixedU128,
			}
			impl ::subxt::Event for NewMarketCreated {
				const PALLET: &'static str = "Lending";
				const EVENT: &'static str = "NewMarketCreated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CollateralDeposited {
				pub sender: ::subxt::sp_core::crypto::AccountId32,
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for CollateralDeposited {
				const PALLET: &'static str = "Lending";
				const EVENT: &'static str = "CollateralDeposited";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct CollateralWithdrawed {
				pub sender: ::subxt::sp_core::crypto::AccountId32,
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for CollateralWithdrawed {
				const PALLET: &'static str = "Lending";
				const EVENT: &'static str = "CollateralWithdrawed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Borrowed {
				pub sender: ::subxt::sp_core::crypto::AccountId32,
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Borrowed {
				const PALLET: &'static str = "Lending";
				const EVENT: &'static str = "Borrowed";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RepaidBorrow {
				pub sender: ::subxt::sp_core::crypto::AccountId32,
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub beneficiary: ::subxt::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for RepaidBorrow {
				const PALLET: &'static str = "Lending";
				const EVENT: &'static str = "RepaidBorrow";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct LiquidationInitiated {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub account: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Event for LiquidationInitiated {
				const PALLET: &'static str = "Lending";
				const EVENT: &'static str = "LiquidationInitiated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SoonMayUnderCollaterized {
				pub market_id: runtime_types::pallet_lending::pallet::MarketIndex,
				pub account: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Event for SoonMayUnderCollaterized {
				const PALLET: &'static str = "Lending";
				const EVENT: &'static str = "SoonMayUnderCollaterized";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct LendingCount;
			impl ::subxt::StorageEntry for LendingCount {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "LendingCount";
				type Value = runtime_types::pallet_lending::pallet::MarketIndex;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Markets(pub runtime_types::pallet_lending::pallet::MarketIndex);
			impl ::subxt::StorageEntry for Markets {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "Markets";
				type Value = runtime_types::composable_traits::lending::MarketConfig<
					::core::primitive::u64,
					runtime_types::primitives::currency::CurrencyId,
					::subxt::sp_core::crypto::AccountId32,
				>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct DebtMarkets(pub runtime_types::pallet_lending::pallet::MarketIndex);
			impl ::subxt::StorageEntry for DebtMarkets {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "DebtMarkets";
				type Value = runtime_types::primitives::currency::CurrencyId;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct DebtIndex(
				runtime_types::pallet_lending::pallet::MarketIndex,
				::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::StorageEntry for DebtIndex {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "DebtIndex";
				type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
						::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
					])
				}
			}
			pub struct BorrowTimestamp(
				runtime_types::pallet_lending::pallet::MarketIndex,
				::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::StorageEntry for BorrowTimestamp {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "BorrowTimestamp";
				type Value = ::core::primitive::u64;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
						::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
					])
				}
			}
			pub struct BorrowIndex(pub runtime_types::pallet_lending::pallet::MarketIndex);
			impl ::subxt::StorageEntry for BorrowIndex {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "BorrowIndex";
				type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct AccountCollateral(
				runtime_types::pallet_lending::pallet::MarketIndex,
				::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::StorageEntry for AccountCollateral {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "AccountCollateral";
				type Value = ::core::primitive::u128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![
						::subxt::StorageMapKey::new(
							&self.0,
							::subxt::StorageHasher::Blake2_128Concat,
						),
						::subxt::StorageMapKey::new(
							&self.1,
							::subxt::StorageHasher::Blake2_128Concat,
						),
					])
				}
			}
			pub struct LastBlockTimestamp;
			impl ::subxt::StorageEntry for LastBlockTimestamp {
				const PALLET: &'static str = "Lending";
				const STORAGE: &'static str = "LastBlockTimestamp";
				type Value = ::core::primitive::u64;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn lending_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_lending::pallet::MarketIndex,
					::subxt::Error,
				> {
					let entry = LendingCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn markets(
					&self,
					_0: runtime_types::pallet_lending::pallet::MarketIndex,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::composable_traits::lending::MarketConfig<
						::core::primitive::u64,
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
					>,
					::subxt::Error,
				> {
					let entry = Markets(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn markets_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Markets>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn debt_markets(
					&self,
					_0: runtime_types::pallet_lending::pallet::MarketIndex,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::primitives::currency::CurrencyId,
					::subxt::Error,
				> {
					let entry = DebtMarkets(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn debt_markets_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, DebtMarkets>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn debt_index(
					&self,
					_0: runtime_types::pallet_lending::pallet::MarketIndex,
					_1: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::Error,
				> {
					let entry = DebtIndex(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn debt_index_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, DebtIndex>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn borrow_timestamp(
					&self,
					_0: runtime_types::pallet_lending::pallet::MarketIndex,
					_1: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u64>,
					::subxt::Error,
				> {
					let entry = BorrowTimestamp(_0, _1);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn borrow_timestamp_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, BorrowTimestamp>, ::subxt::Error>
				{
					self.client.storage().iter(hash).await
				}
				pub async fn borrow_index(
					&self,
					_0: runtime_types::pallet_lending::pallet::MarketIndex,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::Error,
				> {
					let entry = BorrowIndex(_0);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn borrow_index_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, BorrowIndex>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn account_collateral(
					&self,
					_0: runtime_types::pallet_lending::pallet::MarketIndex,
					_1: ::subxt::sp_core::crypto::AccountId32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::Error> {
					let entry = AccountCollateral(_0, _1);
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn account_collateral_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, AccountCollateral>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
				pub async fn last_block_timestamp(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u64, ::subxt::Error> {
					let entry = LastBlockTimestamp;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
			}
		}
	}
	pub mod liquid_crowdloan {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct MakeClaimable {}
			impl ::subxt::Call for MakeClaimable {
				const PALLET: &'static str = "LiquidCrowdloan";
				const FUNCTION: &'static str = "make_claimable";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Claim {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for Claim {
				const PALLET: &'static str = "LiquidCrowdloan";
				const FUNCTION: &'static str = "claim";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn make_claimable(&self) -> ::subxt::SubmittableExtrinsic<T, MakeClaimable> {
					let call = MakeClaimable {};
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn claim(
					&self,
					amount: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, Claim> {
					let call = Claim { amount };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_crowdloan_bonus::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Initiated(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::Event for Initiated {
				const PALLET: &'static str = "LiquidCrowdloan";
				const EVENT: &'static str = "Initiated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Claimed(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Claimed {
				const PALLET: &'static str = "LiquidCrowdloan";
				const EVENT: &'static str = "Claimed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct IsClaimable;
			impl ::subxt::StorageEntry for IsClaimable {
				const PALLET: &'static str = "LiquidCrowdloan";
				const STORAGE: &'static str = "IsClaimable";
				type Value = ::core::primitive::bool;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn is_claimable(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::bool>,
					::subxt::Error,
				> {
					let entry = IsClaimable;
					self.client.storage().fetch(&entry, hash).await
				}
			}
		}
	}
	pub mod liquidations {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct LiquidateMany {
				pub block_number: ::core::primitive::u32,
			}
			impl ::subxt::Call for LiquidateMany {
				const PALLET: &'static str = "Liquidations";
				const FUNCTION: &'static str = "liquidate_many";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn liquidate_many(
					&self,
					block_number: ::core::primitive::u32,
				) -> ::subxt::SubmittableExtrinsic<T, LiquidateMany> {
					let call = LiquidateMany { block_number };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_liquidations::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PositionWasSentToLiquidation {}
			impl ::subxt::Event for PositionWasSentToLiquidation {
				const PALLET: &'static str = "Liquidations";
				const EVENT: &'static str = "PositionWasSentToLiquidation";
			}
		}
	}
	pub mod auctions {
		use super::runtime_types;
		pub type Event = runtime_types::pallet_dutch_auctions::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AuctionWasStarted {
				pub order_id: ::core::primitive::u128,
			}
			impl ::subxt::Event for AuctionWasStarted {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "AuctionWasStarted";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AuctionSuccess {
				pub order_id: ::core::primitive::u128,
			}
			impl ::subxt::Event for AuctionSuccess {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "AuctionSuccess";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AuctionFatalFail {
				pub order_id: ::core::primitive::u128,
			}
			impl ::subxt::Event for AuctionFatalFail {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "AuctionFatalFail";
			}
		}
	}
	pub mod ping {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Start {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Start {
				const PALLET: &'static str = "Ping";
				const FUNCTION: &'static str = "start";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct StartMany {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub count: ::core::primitive::u32,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for StartMany {
				const PALLET: &'static str = "Ping";
				const FUNCTION: &'static str = "start_many";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Stop {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::Call for Stop {
				const PALLET: &'static str = "Ping";
				const FUNCTION: &'static str = "stop";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct StopAll {
				pub maybe_para:
					::core::option::Option<runtime_types::polkadot_parachain::primitives::Id>,
			}
			impl ::subxt::Call for StopAll {
				const PALLET: &'static str = "Ping";
				const FUNCTION: &'static str = "stop_all";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Ping {
				pub seq: ::core::primitive::u32,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Ping {
				const PALLET: &'static str = "Ping";
				const FUNCTION: &'static str = "ping";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Pong {
				pub seq: ::core::primitive::u32,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Pong {
				const PALLET: &'static str = "Ping";
				const FUNCTION: &'static str = "pong";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn start(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Start> {
					let call = Start { para, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn start_many(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					count: ::core::primitive::u32,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, StartMany> {
					let call = StartMany { para, count, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn stop(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::SubmittableExtrinsic<T, Stop> {
					let call = Stop { para };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn stop_all(
					&self,
					maybe_para: ::core::option::Option<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
				) -> ::subxt::SubmittableExtrinsic<T, StopAll> {
					let call = StopAll { maybe_para };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn ping(
					&self,
					seq: ::core::primitive::u32,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Ping> {
					let call = Ping { seq, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn pong(
					&self,
					seq: ::core::primitive::u32,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Pong> {
					let call = Pong { seq, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_ping::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PingSent(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for PingSent {
				const PALLET: &'static str = "Ping";
				const EVENT: &'static str = "PingSent";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Pinged(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for Pinged {
				const PALLET: &'static str = "Ping";
				const EVENT: &'static str = "Pinged";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PongSent(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for PongSent {
				const PALLET: &'static str = "Ping";
				const EVENT: &'static str = "PongSent";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Ponged(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for Ponged {
				const PALLET: &'static str = "Ping";
				const EVENT: &'static str = "Ponged";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ErrorSendingPing(
				pub runtime_types::xcm::v2::traits::SendError,
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for ErrorSendingPing {
				const PALLET: &'static str = "Ping";
				const EVENT: &'static str = "ErrorSendingPing";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ErrorSendingPong(
				pub runtime_types::xcm::v2::traits::SendError,
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for ErrorSendingPong {
				const PALLET: &'static str = "Ping";
				const EVENT: &'static str = "ErrorSendingPong";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UnknownPong(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for UnknownPong {
				const PALLET: &'static str = "Ping";
				const EVENT: &'static str = "UnknownPong";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Targets;
			impl ::subxt::StorageEntry for Targets {
				const PALLET: &'static str = "Ping";
				const STORAGE: &'static str = "Targets";
				type Value = ::std::vec::Vec<(
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<::core::primitive::u8>,
				)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct PingCount;
			impl ::subxt::StorageEntry for PingCount {
				const PALLET: &'static str = "Ping";
				const STORAGE: &'static str = "PingCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Pings(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for Pings {
				const PALLET: &'static str = "Ping";
				const STORAGE: &'static str = "Pings";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn targets(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::Error,
				> {
					let entry = Targets;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn ping_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = PingCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn pings(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::Error,
				> {
					let entry = Pings(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn pings_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Pings>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod assets_registry {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetLocalAdmin {
				pub local_admin: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for SetLocalAdmin {
				const PALLET: &'static str = "AssetsRegistry";
				const FUNCTION: &'static str = "set_local_admin";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct SetForeignAdmin {
				pub foreign_admin: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for SetForeignAdmin {
				const PALLET: &'static str = "AssetsRegistry";
				const FUNCTION: &'static str = "set_foreign_admin";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ApproveAssetsMappingCandidate {
				pub local_asset_id: ::core::primitive::u128,
				pub foreign_asset_id: ::core::primitive::u128,
			}
			impl ::subxt::Call for ApproveAssetsMappingCandidate {
				const PALLET: &'static str = "AssetsRegistry";
				const FUNCTION: &'static str = "approve_assets_mapping_candidate";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn set_local_admin(
					&self,
					local_admin: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, SetLocalAdmin> {
					let call = SetLocalAdmin { local_admin };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn set_foreign_admin(
					&self,
					foreign_admin: ::subxt::sp_core::crypto::AccountId32,
				) -> ::subxt::SubmittableExtrinsic<T, SetForeignAdmin> {
					let call = SetForeignAdmin { foreign_admin };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn approve_assets_mapping_candidate(
					&self,
					local_asset_id: ::core::primitive::u128,
					foreign_asset_id: ::core::primitive::u128,
				) -> ::subxt::SubmittableExtrinsic<T, ApproveAssetsMappingCandidate> {
					let call = ApproveAssetsMappingCandidate { local_asset_id, foreign_asset_id };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_assets_registry::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct LocalAdminUpdated(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for LocalAdminUpdated {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "LocalAdminUpdated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ForeignAdminUpdated(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for ForeignAdminUpdated {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "ForeignAdminUpdated";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AssetsMappingCandidateUpdated {
				pub local_asset_id: ::core::primitive::u128,
				pub foreign_asset_id: ::core::primitive::u128,
			}
			impl ::subxt::Event for AssetsMappingCandidateUpdated {
				const PALLET: &'static str = "AssetsRegistry";
				const EVENT: &'static str = "AssetsMappingCandidateUpdated";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct LocalAdmin;
			impl ::subxt::StorageEntry for LocalAdmin {
				const PALLET: &'static str = "AssetsRegistry";
				const STORAGE: &'static str = "LocalAdmin";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct ForeignAdmin;
			impl ::subxt::StorageEntry for ForeignAdmin {
				const PALLET: &'static str = "AssetsRegistry";
				const STORAGE: &'static str = "ForeignAdmin";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct LocalAsset(pub ::core::primitive::u128);
			impl ::subxt::StorageEntry for LocalAsset {
				const PALLET: &'static str = "AssetsRegistry";
				const STORAGE: &'static str = "LocalAsset";
				type Value = ::core::primitive::u128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct ForeignAsset(pub ::core::primitive::u128);
			impl ::subxt::StorageEntry for ForeignAsset {
				const PALLET: &'static str = "AssetsRegistry";
				const STORAGE: &'static str = "ForeignAsset";
				type Value = ::core::primitive::u128;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct AssetsMappingCandidates(::core::primitive::u128, ::core::primitive::u128);
			impl ::subxt::StorageEntry for AssetsMappingCandidates {
				const PALLET: &'static str = "AssetsRegistry";
				const STORAGE: &'static str = "AssetsMappingCandidates";
				type Value = runtime_types::pallet_assets_registry::pallet::CandidateStatus;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn local_admin(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::sp_core::crypto::AccountId32, ::subxt::Error> {
					let entry = LocalAdmin;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn foreign_admin(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::sp_core::crypto::AccountId32, ::subxt::Error> {
					let entry = ForeignAdmin;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn local_asset(
					&self,
					_0: ::core::primitive::u128,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u128>,
					::subxt::Error,
				> {
					let entry = LocalAsset(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn local_asset_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, LocalAsset>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn foreign_asset(
					&self,
					_0: ::core::primitive::u128,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u128>,
					::subxt::Error,
				> {
					let entry = ForeignAsset(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn foreign_asset_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, ForeignAsset>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
				pub async fn assets_mapping_candidates(
					&self,
					_0: ::core::primitive::u128,
					_1: ::core::primitive::u128,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_assets_registry::pallet::CandidateStatus,
					>,
					::subxt::Error,
				> {
					let entry = AssetsMappingCandidates(_0, _1);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn assets_mapping_candidates_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, AssetsMappingCandidates>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod spambot {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Start {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Start {
				const PALLET: &'static str = "Spambot";
				const FUNCTION: &'static str = "start";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct StartMany {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub count: ::core::primitive::u32,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for StartMany {
				const PALLET: &'static str = "Spambot";
				const FUNCTION: &'static str = "start_many";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Stop {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::Call for Stop {
				const PALLET: &'static str = "Spambot";
				const FUNCTION: &'static str = "stop";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct StopAll {
				pub maybe_para:
					::core::option::Option<runtime_types::polkadot_parachain::primitives::Id>,
			}
			impl ::subxt::Call for StopAll {
				const PALLET: &'static str = "Spambot";
				const FUNCTION: &'static str = "stop_all";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Ping {
				pub seq: ::core::primitive::u32,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Ping {
				const PALLET: &'static str = "Spambot";
				const FUNCTION: &'static str = "ping";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Pong {
				pub seq: ::core::primitive::u32,
				pub payload: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Pong {
				const PALLET: &'static str = "Spambot";
				const FUNCTION: &'static str = "pong";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn start(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Start> {
					let call = Start { para, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn start_many(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					count: ::core::primitive::u32,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, StartMany> {
					let call = StartMany { para, count, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn stop(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::SubmittableExtrinsic<T, Stop> {
					let call = Stop { para };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn stop_all(
					&self,
					maybe_para: ::core::option::Option<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
				) -> ::subxt::SubmittableExtrinsic<T, StopAll> {
					let call = StopAll { maybe_para };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn ping(
					&self,
					seq: ::core::primitive::u32,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Ping> {
					let call = Ping { seq, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn pong(
					&self,
					seq: ::core::primitive::u32,
					payload: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Pong> {
					let call = Pong { seq, payload };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::cumulus_ping::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PingSent(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for PingSent {
				const PALLET: &'static str = "Spambot";
				const EVENT: &'static str = "PingSent";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Pinged(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for Pinged {
				const PALLET: &'static str = "Spambot";
				const EVENT: &'static str = "Pinged";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PongSent(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for PongSent {
				const PALLET: &'static str = "Spambot";
				const EVENT: &'static str = "PongSent";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Ponged(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for Ponged {
				const PALLET: &'static str = "Spambot";
				const EVENT: &'static str = "Ponged";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ErrorSendingPing(
				pub runtime_types::xcm::v2::traits::SendError,
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for ErrorSendingPing {
				const PALLET: &'static str = "Spambot";
				const EVENT: &'static str = "ErrorSendingPing";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ErrorSendingPong(
				pub runtime_types::xcm::v2::traits::SendError,
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for ErrorSendingPong {
				const PALLET: &'static str = "Spambot";
				const EVENT: &'static str = "ErrorSendingPong";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct UnknownPong(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for UnknownPong {
				const PALLET: &'static str = "Spambot";
				const EVENT: &'static str = "UnknownPong";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct Targets;
			impl ::subxt::StorageEntry for Targets {
				const PALLET: &'static str = "Spambot";
				const STORAGE: &'static str = "Targets";
				type Value = ::std::vec::Vec<(
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<::core::primitive::u8>,
				)>;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct PingCount;
			impl ::subxt::StorageEntry for PingCount {
				const PALLET: &'static str = "Spambot";
				const STORAGE: &'static str = "PingCount";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Plain
				}
			}
			pub struct Pings(pub ::core::primitive::u32);
			impl ::subxt::StorageEntry for Pings {
				const PALLET: &'static str = "Spambot";
				const STORAGE: &'static str = "Pings";
				type Value = ::core::primitive::u32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Blake2_128Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn targets(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::Error,
				> {
					let entry = Targets;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn ping_count(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::Error> {
					let entry = PingCount;
					self.client.storage().fetch_or_default(&entry, hash).await
				}
				pub async fn pings(
					&self,
					_0: ::core::primitive::u32,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::Error,
				> {
					let entry = Pings(_0);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn pings_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Pings>, ::subxt::Error> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod call_filter {
		use super::runtime_types;
		pub mod calls {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Disable {
				pub pallet_name: ::std::vec::Vec<::core::primitive::u8>,
				pub function_name: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Disable {
				const PALLET: &'static str = "CallFilter";
				const FUNCTION: &'static str = "disable";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Enable {
				pub pallet_name: ::std::vec::Vec<::core::primitive::u8>,
				pub function_name: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Enable {
				const PALLET: &'static str = "CallFilter";
				const FUNCTION: &'static str = "enable";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> TransactionApi<'a, T>
			where
				T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn disable(
					&self,
					pallet_name: ::std::vec::Vec<::core::primitive::u8>,
					function_name: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Disable> {
					let call = Disable { pallet_name, function_name };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
				pub fn enable(
					&self,
					pallet_name: ::std::vec::Vec<::core::primitive::u8>,
					function_name: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::SubmittableExtrinsic<T, Enable> {
					let call = Enable { pallet_name, function_name };
					::subxt::SubmittableExtrinsic::new(self.client, call)
				}
			}
		}
		pub type Event = runtime_types::pallet_call_filter::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Disabled(
				pub ::std::vec::Vec<::core::primitive::u8>,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for Disabled {
				const PALLET: &'static str = "CallFilter";
				const EVENT: &'static str = "Disabled";
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct TransactionUnpaused(
				pub ::std::vec::Vec<::core::primitive::u8>,
				pub ::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::Event for TransactionUnpaused {
				const PALLET: &'static str = "CallFilter";
				const EVENT: &'static str = "TransactionUnpaused";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct PausedTransactions(
				::std::vec::Vec<::core::primitive::u8>,
				::std::vec::Vec<::core::primitive::u8>,
			);
			impl ::subxt::StorageEntry for PausedTransactions {
				const PALLET: &'static str = "CallFilter";
				const STORAGE: &'static str = "PausedTransactions";
				type Value = ();
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct StorageApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub async fn paused_transactions(
					&self,
					_0: ::std::vec::Vec<::core::primitive::u8>,
					_1: ::std::vec::Vec<::core::primitive::u8>,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::option::Option<()>, ::subxt::Error> {
					let entry = PausedTransactions(_0, _1);
					self.client.storage().fetch(&entry, hash).await
				}
				pub async fn paused_transactions_iter(
					&self,
					hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, PausedTransactions>,
					::subxt::Error,
				> {
					self.client.storage().iter(hash).await
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod composable_traits {
			use super::runtime_types;
			pub mod lending {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct MarketConfig<_0, _1, _2> {
					pub manager: _2,
					pub borrow: _0,
					pub collateral: _1,
					pub collateral_factor: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					pub interest_rate_model:
						runtime_types::composable_traits::rate_model::InterestRateModel,
					pub under_collaterized_warn_percent:
						runtime_types::sp_arithmetic::per_things::Percent,
				}
			}
			pub mod rate_model {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct CurveModel {
					pub base_rate: runtime_types::sp_arithmetic::fixed_point::FixedU128,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct DoubleExponentModel {
					pub coefficients: [::core::primitive::u8; 16usize],
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct DynamicPIDControllerModel {
					pub kp: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					pub ki: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					pub kd: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					pub et_1: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					pub it_1: runtime_types::sp_arithmetic::fixed_point::FixedI128,
					pub ir_t_1: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					pub uo: runtime_types::sp_arithmetic::fixed_point::FixedU128,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum InterestRateModel {
					#[codec(index = 0)]
					Jump(runtime_types::composable_traits::rate_model::JumpModel),
					#[codec(index = 1)]
					Curve(runtime_types::composable_traits::rate_model::CurveModel),
					#[codec(index = 2)]
					DynamicPIDController(
						runtime_types::composable_traits::rate_model::DynamicPIDControllerModel,
					),
					#[codec(index = 3)]
					DoubleExponent(
						runtime_types::composable_traits::rate_model::DoubleExponentModel,
					),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct JumpModel {
					pub base_rate: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					pub jump_rate: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					pub full_rate: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					pub target_utilization: runtime_types::sp_arithmetic::per_things::Percent,
				}
			}
			pub mod vault {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Deposit<_0, _1> {
					#[codec(index = 0)]
					Existential,
					#[codec(index = 1)]
					Rent { amount: _0, at: _1 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct VaultConfig<_0, _1> {
					pub asset_id: _1,
					pub reserved: runtime_types::sp_arithmetic::per_things::Perquintill,
					pub manager: _0,
					pub strategies: ::std::collections::BTreeMap<
						_0,
						runtime_types::sp_arithmetic::per_things::Perquintill,
					>,
				}
			}
		}
		pub mod cumulus_pallet_dmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: ::core::primitive::u64,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					OverLimit,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec(index = 1)]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec(index = 2)]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::xcm::v2::traits::Outcome,
					),
					#[codec(index = 3)]
					WeightExhausted(
						[::core::primitive::u8; 32usize],
						::core::primitive::u64,
						::core::primitive::u64,
					),
					#[codec(index = 4)]
					OverweightEnqueued(
						[::core::primitive::u8; 32usize],
						::core::primitive::u64,
						::core::primitive::u64,
					),
					#[codec(index = 5)]
					OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
				}
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Encode,
				:: subxt :: codec :: Decode,
			)]
			pub struct ConfigData {
				pub max_individual: ::core::primitive::u64,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PageIndexData {
				pub begin_used: ::core::primitive::u32,
				pub end_used: ::core::primitive::u32,
				pub overweight_count: ::core::primitive::u64,
			}
		}
		pub mod cumulus_pallet_parachain_system {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					# [codec (index = 0)] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					OverlappingUpgrades,
					#[codec(index = 1)]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					TooBig,
					#[codec(index = 3)]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					NotScheduled,
					#[codec(index = 6)]
					NothingAuthorized,
					#[codec(index = 7)]
					Unauthorized,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					ValidationFunctionStored,
					#[codec(index = 1)]
					ValidationFunctionApplied(::core::primitive::u32),
					#[codec(index = 2)]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					UpgradeAuthorized(::subxt::sp_core::H256),
					#[codec(index = 4)]
					DownwardMessagesReceived(::core::primitive::u32),
					#[codec(index = 5)]
					DownwardMessagesProcessed(::core::primitive::u64, ::subxt::sp_core::H256),
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct MessagingStateSnapshot {
					pub dmq_mqc_head: ::subxt::sp_core::H256,
					pub relay_dispatch_queue_size: (::core::primitive::u32, ::core::primitive::u32),
					pub ingress_channels: ::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
					)>,
					pub egress_channels: ::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
					)>,
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct MessageQueueChain(pub ::subxt::sp_core::H256);
		}
		pub mod cumulus_pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					InvalidFormat([::core::primitive::u8; 8usize]),
					#[codec(index = 1)]
					UnsupportedVersion([::core::primitive::u8; 8usize]),
					#[codec(index = 2)]
					ExecutedDownward(
						[::core::primitive::u8; 8usize],
						runtime_types::xcm::v2::traits::Outcome,
					),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Origin {
					#[codec(index = 0)]
					Relay,
					#[codec(index = 1)]
					SiblingParachain(runtime_types::polkadot_parachain::primitives::Id),
				}
			}
		}
		pub mod cumulus_pallet_xcmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					FailedToSend,
					#[codec(index = 1)]
					BadXcmOrigin,
					#[codec(index = 2)]
					BadXcm,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Success(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 1)]
					Fail(
						::core::option::Option<::subxt::sp_core::H256>,
						runtime_types::xcm::v2::traits::Error,
					),
					#[codec(index = 2)]
					BadVersion(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 3)]
					BadFormat(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 4)]
					UpwardMessageSent(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 5)]
					XcmpMessageSent(::core::option::Option<::subxt::sp_core::H256>),
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum InboundStatus {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum OutboundStatus {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
				pub threshold_weight: ::core::primitive::u64,
				pub weight_restrict_decay: ::core::primitive::u64,
			}
		}
		pub mod cumulus_ping {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					start {
						para: runtime_types::polkadot_parachain::primitives::Id,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					start_many {
						para: runtime_types::polkadot_parachain::primitives::Id,
						count: ::core::primitive::u32,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					stop { para: runtime_types::polkadot_parachain::primitives::Id },
					#[codec(index = 3)]
					stop_all {
						maybe_para: ::core::option::Option<
							runtime_types::polkadot_parachain::primitives::Id,
						>,
					},
					#[codec(index = 4)]
					ping {
						seq: ::core::primitive::u32,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 5)]
					pong {
						seq: ::core::primitive::u32,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					PingSent(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 1)]
					Pinged(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 2)]
					PongSent(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 3)]
					Ponged(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
						::core::primitive::u32,
					),
					#[codec(index = 4)]
					ErrorSendingPing(
						runtime_types::xcm::v2::traits::SendError,
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 5)]
					ErrorSendingPong(
						runtime_types::xcm::v2::traits::SendError,
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 6)]
					UnknownPong(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
				}
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ParachainInherentData {
				pub validation_data:
					runtime_types::polkadot_primitives::v1::PersistedValidationData<
						::subxt::sp_core::H256,
						::core::primitive::u32,
					>,
				pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
				pub downward_messages: ::std::vec::Vec<
					runtime_types::polkadot_core_primitives::InboundDownwardMessage<
						::core::primitive::u32,
					>,
				>,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod storage {
				use super::runtime_types;
				pub mod bounded_vec {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
				pub mod weak_bounded_vec {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
						pub enum BalanceStatus {
							#[codec(index = 0)]
							Free,
							#[codec(index = 1)]
							Reserved,
						}
					}
				}
			}
			pub mod weights {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct DispatchInfo {
					pub weight: ::core::primitive::u64,
					pub class: runtime_types::frame_support::weights::DispatchClass,
					pub pays_fee: runtime_types::frame_support::weights::Pays,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct RuntimeDbWeight {
					pub read: ::core::primitive::u64,
					pub write: ::core::primitive::u64,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct WeightToFeeCoefficient<_0> {
					pub coeff_integer: _0,
					pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
					pub negative: ::core::primitive::bool,
					pub degree: ::core::primitive::u8,
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct CheckGenesis {}
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct CheckNonce(pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct CheckSpecVersion {}
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct CheckTxVersion {}
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct CheckWeight {}
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::weights::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct BlockWeights {
					pub base_block: ::core::primitive::u64,
					pub max_block: ::core::primitive::u64,
					pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct WeightsPerClass {
					pub base_extrinsic: ::core::primitive::u64,
					pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
					pub max_total: ::core::option::Option<::core::primitive::u64>,
					pub reserved: ::core::option::Option<::core::primitive::u64>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					fill_block { ratio: runtime_types::sp_arithmetic::per_things::Perbill },
					#[codec(index = 1)]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 2)]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 3)]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 5)]
					set_changes_trie_config {
						changes_trie_config: ::core::option::Option<
							runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
						>,
					},
					#[codec(index = 6)]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 7)]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 8)]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					InvalidSpecName,
					#[codec(index = 1)]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					NonDefaultComposite,
					#[codec(index = 4)]
					NonZeroRefCount,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
					#[codec(index = 1)]
					ExtrinsicFailed(
						runtime_types::sp_runtime::DispatchError,
						runtime_types::frame_support::weights::DispatchInfo,
					),
					#[codec(index = 2)]
					CodeUpdated,
					#[codec(index = 3)]
					NewAccount(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 4)]
					KilledAccount(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 5)]
					Remarked(::subxt::sp_core::crypto::AccountId32, ::subxt::sp_core::H256),
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Root,
				#[codec(index = 1)]
				Signed(_0),
				#[codec(index = 2)]
				None,
			}
		}
		pub mod orml_tokens {
			use super::runtime_types;
			pub mod module {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					transfer {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					transfer_all {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					transfer_keep_alive {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					force_transfer {
						source: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					set_balance {
						who: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					BalanceTooLow,
					#[codec(index = 1)]
					AmountIntoBalanceFailed,
					#[codec(index = 2)]
					LiquidityRestrictions,
					#[codec(index = 3)]
					MaxLocksExceeded,
					#[codec(index = 4)]
					KeepAlive,
					#[codec(index = 5)]
					ExistentialDeposit,
					#[codec(index = 6)]
					DeadAccount,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Endowed(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 1)]
					DustLost(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 2)]
					Transfer(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 3)]
					Reserved(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 4)]
					Unreserved(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 5)]
					BalanceSet(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::u128,
					),
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub frozen: _0,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
			}
		}
		pub mod pallet_assets_registry {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					set_local_admin { local_admin: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 1)]
					set_foreign_admin { foreign_admin: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 2)]
					approve_assets_mapping_candidate {
						local_asset_id: ::core::primitive::u128,
						foreign_asset_id: ::core::primitive::u128,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum CandidateStatus {
					#[codec(index = 0)]
					LocalAdminApproved,
					#[codec(index = 1)]
					ForeignAdminApproved,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					OnlyAllowedForAdmins,
					#[codec(index = 1)]
					LocalAssetIdAlreadyUsed,
					#[codec(index = 2)]
					ForeignAssetIdAlreadyUsed,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					LocalAdminUpdated(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 1)]
					ForeignAdminUpdated(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 2)]
					AssetsMappingCandidateUpdated {
						local_asset_id: ::core::primitive::u128,
						foreign_asset_id: ::core::primitive::u128,
					},
				}
			}
		}
		pub mod pallet_authorship {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					set_uncles {
						new_uncles: ::std::vec::Vec<
							runtime_types::sp_runtime::generic::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					InvalidUncleParent,
					#[codec(index = 1)]
					UnclesAlreadySet,
					#[codec(index = 2)]
					TooManyUncles,
					#[codec(index = 3)]
					GenesisUncle,
					#[codec(index = 4)]
					TooHighUncle,
					#[codec(index = 5)]
					UncleAlreadyIncluded,
					#[codec(index = 6)]
					OldUncle,
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum UncleEntryItem<_0, _1, _2> {
				#[codec(index = 0)]
				InclusionHeight(_0),
				#[codec(index = 1)]
				Uncle(_1, ::core::option::Option<_2>),
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					transfer {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					set_balance {
						who: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						new_free: ::core::primitive::u128,
						#[codec(compact)]
						new_reserved: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					force_transfer {
						source: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					transfer_keep_alive {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					transfer_all {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					force_unreserve {
						who: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					VestingBalance,
					#[codec(index = 1)]
					LiquidityRestrictions,
					#[codec(index = 2)]
					InsufficientBalance,
					#[codec(index = 3)]
					ExistentialDeposit,
					#[codec(index = 4)]
					KeepAlive,
					#[codec(index = 5)]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					DeadAccount,
					#[codec(index = 7)]
					TooManyReserves,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Endowed(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 1)]
					DustLost(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 2)]
					Transfer(
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 3)]
					BalanceSet(
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::u128,
					),
					#[codec(index = 4)]
					Reserved(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 5)]
					Unreserved(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 6)]
					ReserveRepatriated(
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					),
					#[codec(index = 7)]
					Deposit(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 8)]
					Withdraw(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 9)]
					Slashed(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
				pub reasons: runtime_types::pallet_balances::Reasons,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Reasons {
				#[codec(index = 0)]
				Fee,
				#[codec(index = 1)]
				Misc,
				#[codec(index = 2)]
				All,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Releases {
				#[codec(index = 0)]
				V1_0_0,
				#[codec(index = 1)]
				V2_0_0,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod pallet_call_filter {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					disable {
						pallet_name: ::std::vec::Vec<::core::primitive::u8>,
						function_name: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					enable {
						pallet_name: ::std::vec::Vec<::core::primitive::u8>,
						function_name: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					CannotPause,
					#[codec(index = 1)]
					InvalidCharacter,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Disabled(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 1)]
					TransactionUnpaused(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					),
				}
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					set_invulnerables {
						new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 1)]
					set_desired_candidates { max: ::core::primitive::u32 },
					#[codec(index = 2)]
					set_candidacy_bond { bond: ::core::primitive::u128 },
					#[codec(index = 3)]
					register_as_candidate,
					#[codec(index = 4)]
					leave_intent,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					TooManyCandidates,
					#[codec(index = 1)]
					TooFewCandidates,
					#[codec(index = 2)]
					Unknown,
					#[codec(index = 3)]
					Permission,
					#[codec(index = 4)]
					AlreadyCandidate,
					#[codec(index = 5)]
					NotCandidate,
					#[codec(index = 6)]
					AlreadyInvulnerable,
					#[codec(index = 7)]
					NoAssociatedValidatorId,
					#[codec(index = 8)]
					ValidatorNotRegistered,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					NewInvulnerables(::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>),
					#[codec(index = 1)]
					NewDesiredCandidates(::core::primitive::u32),
					#[codec(index = 2)]
					NewCandidacyBond(::core::primitive::u128),
					#[codec(index = 3)]
					CandidateAdded(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 4)]
					CandidateRemoved(::subxt::sp_core::crypto::AccountId32),
				}
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					set_members {
						new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					execute {
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					vote {
						proposal: ::subxt::sp_core::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					close {
						proposal_hash: ::subxt::sp_core::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						#[codec(compact)]
						proposal_weight_bound: ::core::primitive::u64,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					disapprove_proposal { proposal_hash: ::subxt::sp_core::H256 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					NotMember,
					#[codec(index = 1)]
					DuplicateProposal,
					#[codec(index = 2)]
					ProposalMissing,
					#[codec(index = 3)]
					WrongIndex,
					#[codec(index = 4)]
					DuplicateVote,
					#[codec(index = 5)]
					AlreadyInitialized,
					#[codec(index = 6)]
					TooEarly,
					#[codec(index = 7)]
					TooManyProposals,
					#[codec(index = 8)]
					WrongProposalWeight,
					#[codec(index = 9)]
					WrongProposalLength,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Proposed(
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
						::subxt::sp_core::H256,
						::core::primitive::u32,
					),
					#[codec(index = 1)]
					Voted(
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::H256,
						::core::primitive::bool,
						::core::primitive::u32,
						::core::primitive::u32,
					),
					#[codec(index = 2)]
					Approved(::subxt::sp_core::H256),
					#[codec(index = 3)]
					Disapproved(::subxt::sp_core::H256),
					#[codec(index = 4)]
					Executed(
						::subxt::sp_core::H256,
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
					#[codec(index = 5)]
					MemberExecuted(
						::subxt::sp_core::H256,
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
					#[codec(index = 6)]
					Closed(::subxt::sp_core::H256, ::core::primitive::u32, ::core::primitive::u32),
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Votes<_0, _1> {
				pub index: _1,
				pub threshold: _1,
				pub ayes: ::std::vec::Vec<_0>,
				pub nays: ::std::vec::Vec<_0>,
				pub end: _1,
			}
		}
		pub mod pallet_crowdloan_bonus {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					make_claimable,
					#[codec(index = 1)]
					claim { amount: ::core::primitive::u128 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					AlreadyInitiated,
					#[codec(index = 1)]
					NotClaimable,
					#[codec(index = 2)]
					EmptyPot,
					#[codec(index = 3)]
					InsufficientTokens,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Initiated(runtime_types::primitives::currency::CurrencyId),
					#[codec(index = 1)]
					Claimed(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
				}
			}
		}
		pub mod pallet_currency_factory {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {}
			}
		}
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Conviction {
					#[codec(index = 0)]
					None,
					#[codec(index = 1)]
					Locked1x,
					#[codec(index = 2)]
					Locked2x,
					#[codec(index = 3)]
					Locked3x,
					#[codec(index = 4)]
					Locked4x,
					#[codec(index = 5)]
					Locked5x,
					#[codec(index = 6)]
					Locked6x,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					propose {
						proposal_hash: ::subxt::sp_core::H256,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					second {
						#[codec(compact)]
						proposal: ::core::primitive::u32,
						#[codec(compact)]
						seconds_upper_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					vote {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 3)]
					emergency_cancel { ref_index: ::core::primitive::u32 },
					#[codec(index = 4)]
					external_propose { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 5)]
					external_propose_majority { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 6)]
					external_propose_default { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 7)]
					fast_track {
						proposal_hash: ::subxt::sp_core::H256,
						voting_period: ::core::primitive::u32,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					veto_external { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 9)]
					cancel_referendum {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					cancel_queued { which: ::core::primitive::u32 },
					#[codec(index = 11)]
					delegate {
						to: ::subxt::sp_core::crypto::AccountId32,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					undelegate,
					#[codec(index = 13)]
					clear_public_proposals,
					#[codec(index = 14)]
					note_preimage { encoded_proposal: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 15)]
					note_preimage_operational {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 16)]
					note_imminent_preimage {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 17)]
					note_imminent_preimage_operational {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 18)]
					reap_preimage {
						proposal_hash: ::subxt::sp_core::H256,
						#[codec(compact)]
						proposal_len_upper_bound: ::core::primitive::u32,
					},
					#[codec(index = 19)]
					unlock { target: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 20)]
					remove_vote { index: ::core::primitive::u32 },
					#[codec(index = 21)]
					remove_other_vote {
						target: ::subxt::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 22)]
					enact_proposal {
						proposal_hash: ::subxt::sp_core::H256,
						index: ::core::primitive::u32,
					},
					#[codec(index = 23)]
					blacklist {
						proposal_hash: ::subxt::sp_core::H256,
						maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 24)]
					cancel_proposal {
						#[codec(compact)]
						prop_index: ::core::primitive::u32,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					ValueLow,
					#[codec(index = 1)]
					ProposalMissing,
					#[codec(index = 2)]
					AlreadyCanceled,
					#[codec(index = 3)]
					DuplicateProposal,
					#[codec(index = 4)]
					ProposalBlacklisted,
					#[codec(index = 5)]
					NotSimpleMajority,
					#[codec(index = 6)]
					InvalidHash,
					#[codec(index = 7)]
					NoProposal,
					#[codec(index = 8)]
					AlreadyVetoed,
					#[codec(index = 9)]
					DuplicatePreimage,
					#[codec(index = 10)]
					NotImminent,
					#[codec(index = 11)]
					TooEarly,
					#[codec(index = 12)]
					Imminent,
					#[codec(index = 13)]
					PreimageMissing,
					#[codec(index = 14)]
					ReferendumInvalid,
					#[codec(index = 15)]
					PreimageInvalid,
					#[codec(index = 16)]
					NoneWaiting,
					#[codec(index = 17)]
					NotVoter,
					#[codec(index = 18)]
					NoPermission,
					#[codec(index = 19)]
					AlreadyDelegating,
					#[codec(index = 20)]
					InsufficientFunds,
					#[codec(index = 21)]
					NotDelegating,
					#[codec(index = 22)]
					VotesExist,
					#[codec(index = 23)]
					InstantNotAllowed,
					#[codec(index = 24)]
					Nonsense,
					#[codec(index = 25)]
					WrongUpperBound,
					#[codec(index = 26)]
					MaxVotesReached,
					#[codec(index = 27)]
					TooManyProposals,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Proposed(::core::primitive::u32, ::core::primitive::u128),
					#[codec(index = 1)]
					Tabled(
						::core::primitive::u32,
						::core::primitive::u128,
						::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					),
					#[codec(index = 2)]
					ExternalTabled,
					#[codec(index = 3)]
					Started(
						::core::primitive::u32,
						runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					),
					#[codec(index = 4)]
					Passed(::core::primitive::u32),
					#[codec(index = 5)]
					NotPassed(::core::primitive::u32),
					#[codec(index = 6)]
					Cancelled(::core::primitive::u32),
					#[codec(index = 7)]
					Executed(
						::core::primitive::u32,
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
					#[codec(index = 8)]
					Delegated(
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::crypto::AccountId32,
					),
					#[codec(index = 9)]
					Undelegated(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 10)]
					Vetoed(
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::H256,
						::core::primitive::u32,
					),
					#[codec(index = 11)]
					PreimageNoted(
						::subxt::sp_core::H256,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 12)]
					PreimageUsed(
						::subxt::sp_core::H256,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 13)]
					PreimageInvalid(::subxt::sp_core::H256, ::core::primitive::u32),
					#[codec(index = 14)]
					PreimageMissing(::subxt::sp_core::H256, ::core::primitive::u32),
					#[codec(index = 15)]
					PreimageReaped(
						::subxt::sp_core::H256,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::subxt::sp_core::crypto::AccountId32,
					),
					#[codec(index = 16)]
					Blacklisted(::subxt::sp_core::H256),
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum ReferendumInfo<_0, _1, _2> {
					#[codec(index = 0)]
					Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
					#[codec(index = 1)]
					Finished { approved: ::core::primitive::bool, end: _0 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct ReferendumStatus<_0, _1, _2> {
					pub end: _0,
					pub proposal_hash: _1,
					pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					pub delay: _0,
					pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub turnout: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard { vote: runtime_types::pallet_democracy::vote::Vote, balance: _0 },
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct Vote(::core::primitive::u8);
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Voting<_0, _1, _2> {
					#[codec(index = 0)]
					Direct {
						votes: ::std::vec::Vec<(
							_2,
							runtime_types::pallet_democracy::vote::AccountVote<_0>,
						)>,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
					#[codec(index = 1)]
					Delegating {
						balance: _0,
						target: _1,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
						prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
					},
				}
			}
			pub mod vote_threshold {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum VoteThreshold {
					#[codec(index = 0)]
					SuperMajorityApprove,
					#[codec(index = 1)]
					SuperMajorityAgainst,
					#[codec(index = 2)]
					SimpleMajority,
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum PreimageStatus<_0, _1, _2> {
				#[codec(index = 0)]
				Missing(_2),
				#[codec(index = 1)]
				Available {
					data: ::std::vec::Vec<::core::primitive::u8>,
					provider: _0,
					deposit: _1,
					since: _2,
					expiry: ::core::option::Option<_2>,
				},
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Releases {
				#[codec(index = 0)]
				V1,
			}
		}
		pub mod pallet_dutch_auctions {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					CannotWithdrawAmountEqualToDesiredAuction,
					#[codec(index = 1)]
					EitherTooMuchOfAuctions,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					AuctionWasStarted { order_id: ::core::primitive::u128 },
					#[codec(index = 1)]
					AuctionSuccess { order_id: ::core::primitive::u128 },
					#[codec(index = 2)]
					AuctionFatalFail { order_id: ::core::primitive::u128 },
				}
			}
		}
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					claim { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					transfer {
						new: ::subxt::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					force_transfer {
						new: ::subxt::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					freeze { index: ::core::primitive::u32 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					NotAssigned,
					#[codec(index = 1)]
					NotOwner,
					#[codec(index = 2)]
					InUse,
					#[codec(index = 3)]
					NotTransfer,
					#[codec(index = 4)]
					Permanent,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					IndexAssigned(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u32),
					#[codec(index = 1)]
					IndexFreed(::core::primitive::u32),
					#[codec(index = 2)]
					IndexFrozen(::core::primitive::u32, ::subxt::sp_core::crypto::AccountId32),
				}
			}
		}
		pub mod pallet_lending {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					create_new_market {
						borrow_asset_id: runtime_types::primitives::currency::CurrencyId,
						collateral_asset_id: runtime_types::primitives::currency::CurrencyId,
						reserved_factor: runtime_types::sp_arithmetic::per_things::Perquintill,
						collateral_factor: runtime_types::sp_arithmetic::fixed_point::FixedU128,
						under_collaterized_warn_percent:
							runtime_types::sp_arithmetic::per_things::Percent,
						interest_rate_model:
							runtime_types::composable_traits::rate_model::InterestRateModel,
					},
					#[codec(index = 1)]
					deposit_collateral {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					withdraw_collateral {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					borrow {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						amount_to_borrow: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					repay_borrow {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						beneficiary: ::subxt::sp_core::crypto::AccountId32,
						repay_amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					liquidate {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						borrower: ::subxt::sp_core::crypto::AccountId32,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					Overflow,
					#[codec(index = 1)]
					Underflow,
					#[codec(index = 2)]
					VaultNotFound,
					#[codec(index = 3)]
					AssetWithoutPrice,
					#[codec(index = 4)]
					MarketDoesNotExist,
					#[codec(index = 5)]
					CollateralDepositFailed,
					#[codec(index = 6)]
					MarketCollateralWasNotDepositedByAccount,
					#[codec(index = 7)]
					CollateralFactorIsLessOrEqualOne,
					#[codec(index = 8)]
					MarketAndAccountPairNotFound,
					#[codec(index = 9)]
					NotEnoughCollateralToBorrowAmount,
					#[codec(index = 10)]
					MarketIsClosing,
					#[codec(index = 11)]
					InvalidTimestampOnBorrowRequest,
					#[codec(index = 12)]
					NotEnoughBorrowAsset,
					#[codec(index = 13)]
					NotEnoughCollateral,
					#[codec(index = 14)]
					TransferFailed,
					#[codec(index = 15)]
					CannotWithdrawFromProvidedBorrowAccount,
					#[codec(index = 16)]
					CannotRepayMoreThanBorrowAmount,
					#[codec(index = 17)]
					BorrowRateDoesNotExist,
					#[codec(index = 18)]
					BorrowIndexDoesNotExist,
					#[codec(index = 19)]
					BorrowAndRepayInSameBlockIsNotSupported,
					#[codec(index = 20)]
					BorrowDoesNotExist,
					#[codec(index = 21)]
					RepayAmountMustBeGraterThanZero,
					#[codec(index = 22)]
					ExceedLendingCount,
					#[codec(index = 23)]
					LiquidationFailed,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					NewMarketCreated {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						vault_id: ::core::primitive::u64,
						manager: ::subxt::sp_core::crypto::AccountId32,
						borrow_asset_id: runtime_types::primitives::currency::CurrencyId,
						collateral_asset_id: runtime_types::primitives::currency::CurrencyId,
						reserved_factor: runtime_types::sp_arithmetic::per_things::Perquintill,
						collateral_factor: runtime_types::sp_arithmetic::fixed_point::FixedU128,
					},
					#[codec(index = 1)]
					CollateralDeposited {
						sender: ::subxt::sp_core::crypto::AccountId32,
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					CollateralWithdrawed {
						sender: ::subxt::sp_core::crypto::AccountId32,
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					Borrowed {
						sender: ::subxt::sp_core::crypto::AccountId32,
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					RepaidBorrow {
						sender: ::subxt::sp_core::crypto::AccountId32,
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						beneficiary: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					LiquidationInitiated {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						account: ::subxt::sp_core::crypto::AccountId32,
					},
					#[codec(index = 6)]
					SoonMayUnderCollaterized {
						market_id: runtime_types::pallet_lending::pallet::MarketIndex,
						account: ::subxt::sp_core::crypto::AccountId32,
					},
				}
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct MarketIndex(pub ::core::primitive::u32);
			}
		}
		pub mod pallet_liquidations {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					liquidate_many { block_number: ::core::primitive::u32 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					PositionWasSentToLiquidation,
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					add_member { who: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 1)]
					remove_member { who: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 2)]
					swap_member {
						remove: ::subxt::sp_core::crypto::AccountId32,
						add: ::subxt::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					reset_members {
						members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 4)]
					change_key { new: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 5)]
					set_prime { who: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 6)]
					clear_prime,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					AlreadyMember,
					#[codec(index = 1)]
					NotMember,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					MemberAdded,
					#[codec(index = 1)]
					MemberRemoved,
					#[codec(index = 2)]
					MembersSwapped,
					#[codec(index = 3)]
					MembersReset,
					#[codec(index = 4)]
					KeyChanged,
					#[codec(index = 5)]
					Dummy,
				}
			}
		}
		pub mod pallet_oracle {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct AssetInfo<_0> {
					pub threshold: _0,
					pub min_answers: ::core::primitive::u32,
					pub max_answers: ::core::primitive::u32,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					add_asset_and_info {
						asset_id: runtime_types::primitives::currency::CurrencyId,
						threshold: runtime_types::sp_arithmetic::per_things::Percent,
						min_answers: ::core::primitive::u32,
						max_answers: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					request_price { asset_id: runtime_types::primitives::currency::CurrencyId },
					#[codec(index = 2)]
					set_signer { signer: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 3)]
					add_stake { stake: ::core::primitive::u128 },
					#[codec(index = 4)]
					remove_stake,
					#[codec(index = 5)]
					reclaim_stake,
					#[codec(index = 6)]
					submit_price {
						price: ::core::primitive::u128,
						asset_id: runtime_types::primitives::currency::CurrencyId,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					Unknown,
					#[codec(index = 1)]
					NoPermission,
					#[codec(index = 2)]
					NoStake,
					#[codec(index = 3)]
					StakeLocked,
					#[codec(index = 4)]
					NotEnoughStake,
					#[codec(index = 5)]
					NotEnoughFunds,
					#[codec(index = 6)]
					InvalidAssetId,
					#[codec(index = 7)]
					AlreadySubmitted,
					#[codec(index = 8)]
					MaxPrices,
					#[codec(index = 9)]
					PriceNotRequested,
					#[codec(index = 10)]
					UnsetSigner,
					#[codec(index = 11)]
					AlreadySet,
					#[codec(index = 12)]
					UnsetController,
					#[codec(index = 13)]
					ControllerUsed,
					#[codec(index = 14)]
					SignerUsed,
					#[codec(index = 15)]
					AvoidPanic,
					#[codec(index = 16)]
					ExceedMaxAnswers,
					#[codec(index = 17)]
					InvalidMinAnswers,
					#[codec(index = 18)]
					MaxAnswersLessThanMinAnswers,
					#[codec(index = 19)]
					ExceedThreshold,
					#[codec(index = 20)]
					ExceedAssetsCount,
					#[codec(index = 21)]
					PriceNotFound,
					#[codec(index = 22)]
					ExceedStake,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					AssetInfoChange(
						runtime_types::primitives::currency::CurrencyId,
						runtime_types::sp_arithmetic::per_things::Percent,
						::core::primitive::u32,
						::core::primitive::u32,
					),
					#[codec(index = 1)]
					PriceRequested(
						::subxt::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
					),
					#[codec(index = 2)]
					SignerSet(
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::crypto::AccountId32,
					),
					#[codec(index = 3)]
					StakeAdded(
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::u128,
					),
					#[codec(index = 4)]
					StakeRemoved(
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::u32,
					),
					#[codec(index = 5)]
					StakeReclaimed(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
					#[codec(index = 6)]
					PriceSubmitted(
						::subxt::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					),
					#[codec(index = 7)]
					UserSlashed(
						::subxt::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					),
					#[codec(index = 8)]
					UserRewarded(
						::subxt::sp_core::crypto::AccountId32,
						runtime_types::primitives::currency::CurrencyId,
						::core::primitive::u128,
					),
					#[codec(index = 9)]
					AnswerPruned(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct PrePrice<_0, _1, _2> {
					pub price: _0,
					pub block: _1,
					pub who: _2,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Price<_0, _1> {
					pub price: _0,
					pub block: _1,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Withdraw<_0, _1> {
					pub stake: _0,
					pub unlock_block: _1,
				}
			}
		}
		pub mod pallet_ping {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					start {
						para: runtime_types::polkadot_parachain::primitives::Id,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					start_many {
						para: runtime_types::polkadot_parachain::primitives::Id,
						count: ::core::primitive::u32,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 2)]
					stop { para: runtime_types::polkadot_parachain::primitives::Id },
					#[codec(index = 3)]
					stop_all {
						maybe_para: ::core::option::Option<
							runtime_types::polkadot_parachain::primitives::Id,
						>,
					},
					#[codec(index = 4)]
					ping {
						seq: ::core::primitive::u32,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 5)]
					pong {
						seq: ::core::primitive::u32,
						payload: ::std::vec::Vec<::core::primitive::u8>,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					PingSent(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 1)]
					Pinged(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 2)]
					PongSent(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 3)]
					Ponged(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
						::core::primitive::u32,
					),
					#[codec(index = 4)]
					ErrorSendingPing(
						runtime_types::xcm::v2::traits::SendError,
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 5)]
					ErrorSendingPong(
						runtime_types::xcm::v2::traits::SendError,
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
					#[codec(index = 6)]
					UnknownPong(
						runtime_types::polkadot_parachain::primitives::Id,
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					),
				}
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
					},
					#[codec(index = 1)]
					cancel { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					schedule_named {
						id: ::std::vec::Vec<::core::primitive::u8>,
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
					},
					#[codec(index = 3)]
					cancel_named { id: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
					},
					#[codec(index = 5)]
					schedule_named_after {
						id: ::std::vec::Vec<::core::primitive::u8>,
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					FailedToSchedule,
					#[codec(index = 1)]
					NotFound,
					#[codec(index = 2)]
					TargetBlockNumberInPast,
					#[codec(index = 3)]
					RescheduleNoChange,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Scheduled(::core::primitive::u32, ::core::primitive::u32),
					#[codec(index = 1)]
					Canceled(::core::primitive::u32, ::core::primitive::u32),
					#[codec(index = 2)]
					Dispatched(
						(::core::primitive::u32, ::core::primitive::u32),
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Releases {
				#[codec(index = 0)]
				V1,
				#[codec(index = 1)]
				V2,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ScheduledV2<_0, _1, _2, _3> {
				pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
				pub priority: ::core::primitive::u8,
				pub call: _0,
				pub maybe_periodic: ::core::option::Option<(_1, _1)>,
				pub origin: _2,
				#[codec(skip)]
				pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
			}
		}
		pub mod pallet_session {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					set_keys {
						keys: runtime_types::picasso_runtime::opaque::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					purge_keys,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					InvalidProof,
					#[codec(index = 1)]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					DuplicatedKey,
					#[codec(index = 3)]
					NoKeys,
					#[codec(index = 4)]
					NoAccount,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					NewSession(::core::primitive::u32),
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					sudo { call: ::std::boxed::Box<runtime_types::picasso_runtime::Call> },
					#[codec(index = 1)]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
						weight: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					set_key {
						new: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					sudo_as {
						who: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					RequireSudo,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Sudid(::core::result::Result<(), runtime_types::sp_runtime::DispatchError>),
					#[codec(index = 1)]
					KeyChanged(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 2)]
					SudoAsDone(
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
				}
			}
		}
		pub mod pallet_timestamp {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct ChargeTransactionPayment(pub ::core::primitive::u128);
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Releases {
				#[codec(index = 0)]
				V1Ancient,
				#[codec(index = 1)]
				V2,
			}
		}
		pub mod pallet_treasury {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					approve_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					InvalidIndex,
					#[codec(index = 2)]
					TooManyApprovals,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Proposed(::core::primitive::u32),
					#[codec(index = 1)]
					Spending(::core::primitive::u128),
					#[codec(index = 2)]
					Awarded(
						::core::primitive::u32,
						::core::primitive::u128,
						::subxt::sp_core::crypto::AccountId32,
					),
					#[codec(index = 3)]
					Rejected(::core::primitive::u32, ::core::primitive::u128),
					#[codec(index = 4)]
					Burnt(::core::primitive::u128),
					#[codec(index = 5)]
					Rollover(::core::primitive::u128),
					#[codec(index = 6)]
					Deposit(::core::primitive::u128),
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Proposal<_0, _1> {
				pub proposer: _0,
				pub value: _1,
				pub beneficiary: _0,
				pub bond: _1,
			}
		}
		pub mod pallet_utility {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					batch { calls: ::std::vec::Vec<runtime_types::picasso_runtime::Call> },
					#[codec(index = 1)]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::picasso_runtime::Call>,
					},
					#[codec(index = 2)]
					batch_all { calls: ::std::vec::Vec<runtime_types::picasso_runtime::Call> },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					TooManyCalls,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					BatchInterrupted(
						::core::primitive::u32,
						runtime_types::sp_runtime::DispatchError,
					),
					#[codec(index = 1)]
					BatchCompleted,
					#[codec(index = 2)]
					ItemCompleted,
				}
			}
		}
		pub mod pallet_vault {
			use super::runtime_types;
			pub mod capabilities {
				use super::runtime_types;
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct Capabilities {
					pub bits: ::core::primitive::u32,
				}
			}
			pub mod models {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct StrategyOverview<_0> {
					pub balance: _0,
					pub lifetime_withdrawn: _0,
					pub lifetime_deposited: _0,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct VaultInfo<_0, _1, _2, _3> {
					pub asset_id: _2,
					pub lp_token_id: _2,
					pub manager: _0,
					pub deposit: runtime_types::composable_traits::vault::Deposit<_1, _3>,
					pub capabilities: runtime_types::pallet_vault::capabilities::Capabilities,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					create {
						vault: runtime_types::composable_traits::vault::VaultConfig<
							::subxt::sp_core::crypto::AccountId32,
							runtime_types::primitives::currency::CurrencyId,
						>,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					claim_surcharge {
						dest: ::core::primitive::u64,
						address: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 2)]
					deposit { vault: ::core::primitive::u64, asset_amount: ::core::primitive::u128 },
					#[codec(index = 3)]
					withdraw { vault: ::core::primitive::u64, lp_amount: ::core::primitive::u128 },
					#[codec(index = 4)]
					emergency_shutdown { vault: ::core::primitive::u64 },
					#[codec(index = 5)]
					start { vault: ::core::primitive::u64 },
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					CannotCreateAsset,
					#[codec(index = 1)]
					TransferFromFailed,
					#[codec(index = 2)]
					MintFailed,
					#[codec(index = 3)]
					InsufficientLpTokens,
					#[codec(index = 4)]
					VaultDoesNotExist,
					#[codec(index = 5)]
					NoFreeVaultAllocation,
					#[codec(index = 6)]
					AllocationMustSumToOne,
					#[codec(index = 7)]
					TooManyStrategies,
					#[codec(index = 8)]
					OverflowError,
					#[codec(index = 9)]
					InsufficientFunds,
					#[codec(index = 10)]
					AmountMustGteMinimumDeposit,
					#[codec(index = 11)]
					AmountMustGteMinimumWithdrawal,
					#[codec(index = 12)]
					NotEnoughLiquidity,
					#[codec(index = 13)]
					InsufficientCreationDeposit,
					#[codec(index = 14)]
					InvalidSurchargeClaim,
					#[codec(index = 15)]
					NotVaultLpToken,
					#[codec(index = 16)]
					DepositsHalted,
					#[codec(index = 17)]
					WithdrawalsHalted,
					#[codec(index = 18)]
					OnlyManagerCanDoThisOperation,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					VaultCreated { id: ::core::primitive::u64 },
					#[codec(index = 1)]
					Deposited {
						account: ::subxt::sp_core::crypto::AccountId32,
						asset_amount: ::core::primitive::u128,
						lp_amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					Withdrawn {
						account: ::subxt::sp_core::crypto::AccountId32,
						lp_amount: ::core::primitive::u128,
						asset_amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					EmergencyShutdown { vault: ::core::primitive::u64 },
					#[codec(index = 4)]
					VaultStarted { vault: ::core::primitive::u64 },
				}
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 7)]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 8)]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 9)]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Error {
					#[codec(index = 0)]
					Unreachable,
					#[codec(index = 1)]
					SendFailure,
					#[codec(index = 2)]
					Filtered,
					#[codec(index = 3)]
					UnweighableMessage,
					#[codec(index = 4)]
					DestinationNotInvertible,
					#[codec(index = 5)]
					Empty,
					#[codec(index = 6)]
					CannotReanchor,
					#[codec(index = 7)]
					TooManyAssets,
					#[codec(index = 8)]
					InvalidOrigin,
					#[codec(index = 9)]
					BadVersion,
					#[codec(index = 10)]
					BadLocation,
					#[codec(index = 11)]
					NoSubscription,
					#[codec(index = 12)]
					AlreadySubscribed,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Event {
					#[codec(index = 0)]
					Attempted(runtime_types::xcm::v2::traits::Outcome),
					#[codec(index = 1)]
					Sent(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v2::Xcm,
					),
					#[codec(index = 2)]
					UnexpectedResponse(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 3)]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
					#[codec(index = 4)]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec(index = 5)]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						::core::primitive::u64,
						::core::primitive::u64,
					),
					#[codec(index = 6)]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 7)]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 8)]
					InvalidResponder(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v1::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 9)]
					InvalidResponderVersion(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 10)]
					ResponseTaken(::core::primitive::u64),
					#[codec(index = 11)]
					AssetsTrapped(
						::subxt::sp_core::H256,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec(index = 12)]
					VersionChangeNotified(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 13)]
					SupportedVersionChanged(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 14)]
					NotifyTargetSendFail(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v2::traits::Error,
					),
					#[codec(index = 15)]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Origin {
					#[codec(index = 0)]
					Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
					#[codec(index = 1)]
					Response(runtime_types::xcm::v1::multilocation::MultiLocation),
				}
			}
		}
		pub mod picasso_runtime {
			use super::runtime_types;
			pub mod opaque {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct SessionKeys {
					pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
				}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Call {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Call),
				#[codec(index = 1)]
				Timestamp(runtime_types::pallet_timestamp::pallet::Call),
				#[codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Call),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Call),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Call),
				#[codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
				#[codec(index = 20)]
				Authorship(runtime_types::pallet_authorship::pallet::Call),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Call),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Call),
				#[codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Call),
				#[codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Call),
				#[codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Call),
				#[codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Call),
				#[codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Call),
				#[codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
				#[codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec(index = 50)]
				Oracle(runtime_types::pallet_oracle::pallet::Call),
				#[codec(index = 51)]
				Tokens(runtime_types::orml_tokens::module::Call),
				#[codec(index = 53)]
				Vault(runtime_types::pallet_vault::pallet::Call),
				#[codec(index = 54)]
				Lending(runtime_types::pallet_lending::pallet::Call),
				#[codec(index = 55)]
				LiquidCrowdloan(runtime_types::pallet_crowdloan_bonus::pallet::Call),
				#[codec(index = 56)]
				Liquidations(runtime_types::pallet_liquidations::pallet::Call),
				#[codec(index = 58)]
				Ping(runtime_types::pallet_ping::pallet::Call),
				#[codec(index = 59)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Call),
				#[codec(index = 90)]
				Spambot(runtime_types::cumulus_ping::pallet::Call),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Call),
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Event {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 2)]
				Sudo(runtime_types::pallet_sudo::pallet::Event),
				#[codec(index = 5)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 6)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 10)]
				ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
				#[codec(index = 21)]
				CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
				#[codec(index = 22)]
				Session(runtime_types::pallet_session::pallet::Event),
				#[codec(index = 30)]
				Council(runtime_types::pallet_collective::pallet::Event),
				#[codec(index = 31)]
				CouncilMembership(runtime_types::pallet_membership::pallet::Event),
				#[codec(index = 32)]
				Treasury(runtime_types::pallet_treasury::pallet::Event),
				#[codec(index = 33)]
				Democracy(runtime_types::pallet_democracy::pallet::Event),
				#[codec(index = 34)]
				Scheduler(runtime_types::pallet_scheduler::pallet::Event),
				#[codec(index = 35)]
				Utility(runtime_types::pallet_utility::pallet::Event),
				#[codec(index = 40)]
				XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
				#[codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec(index = 50)]
				Oracle(runtime_types::pallet_oracle::pallet::Event),
				#[codec(index = 51)]
				Tokens(runtime_types::orml_tokens::module::Event),
				#[codec(index = 52)]
				Factory(runtime_types::pallet_currency_factory::pallet::Event),
				#[codec(index = 53)]
				Vault(runtime_types::pallet_vault::pallet::Event),
				#[codec(index = 54)]
				Lending(runtime_types::pallet_lending::pallet::Event),
				#[codec(index = 55)]
				LiquidCrowdloan(runtime_types::pallet_crowdloan_bonus::pallet::Event),
				#[codec(index = 56)]
				Liquidations(runtime_types::pallet_liquidations::pallet::Event),
				#[codec(index = 57)]
				Auctions(runtime_types::pallet_dutch_auctions::pallet::Event),
				#[codec(index = 58)]
				Ping(runtime_types::pallet_ping::pallet::Event),
				#[codec(index = 59)]
				AssetsRegistry(runtime_types::pallet_assets_registry::pallet::Event),
				#[codec(index = 90)]
				Spambot(runtime_types::cumulus_ping::pallet::Event),
				#[codec(index = 100)]
				CallFilter(runtime_types::pallet_call_filter::pallet::Event),
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum OriginCaller {
				#[codec(index = 0)]
				system(
					runtime_types::frame_system::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
				),
				#[codec(index = 30)]
				Council(
					runtime_types::pallet_collective::RawOrigin<
						::subxt::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 41)]
				PolkadotXcm(runtime_types::pallet_xcm::pallet::Origin),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
				#[codec(index = 4)]
				Void(runtime_types::sp_core::Void),
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct Runtime {}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum XcmpMessageFormat {
					#[codec(index = 0)]
					ConcatenatedVersionedXcm,
					#[codec(index = 1)]
					ConcatenatedEncodedBlob,
					#[codec(index = 2)]
					Signals,
				}
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v1 {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct AbridgedHostConfiguration {
					pub max_code_size: ::core::primitive::u32,
					pub max_head_data_size: ::core::primitive::u32,
					pub max_upward_queue_count: ::core::primitive::u32,
					pub max_upward_queue_size: ::core::primitive::u32,
					pub max_upward_message_size: ::core::primitive::u32,
					pub max_upward_message_num_per_candidate: ::core::primitive::u32,
					pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
					pub validation_upgrade_frequency: ::core::primitive::u32,
					pub validation_upgrade_delay: ::core::primitive::u32,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::sp_core::H256>,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: _1,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct H256(pub [::core::primitive::u8; 32usize]);
		}
		pub mod primitives {
			use super::runtime_types;
			pub mod currency {
				use super::runtime_types;
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct CurrencyId(pub ::core::primitive::u128);
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct FixedI128(pub ::core::primitive::i128);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct Percent(pub ::core::primitive::u8);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct Permill(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Encode,
					:: subxt :: codec :: Decode,
				)]
				pub struct Perquintill(pub ::core::primitive::u64);
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Encode,
				:: subxt :: codec :: Decode,
			)]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod changes_trie {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct ChangesTrieConfiguration {
					pub digest_interval: ::core::primitive::u32,
					pub digest_levels: ::core::primitive::u32,
				}
			}
			pub mod crypto {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum Void {}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum ChangesTrieSignal {
						#[codec(index = 0)]
						NewConfiguration(
							::core::option::Option<
								runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
							>,
						),
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct Digest<_0> {
						pub logs: ::std::vec::Vec<
							runtime_types::sp_runtime::generic::digest::DigestItem<_0>,
						>,
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum DigestItem<_0> {
						#[codec(index = 2)]
						ChangesTrieRoot(_0),
						#[codec(index = 6)]
						PreRuntime(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 4)]
						Consensus(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 5)]
						Seal(
							[::core::primitive::u8; 4usize],
							::std::vec::Vec<::core::primitive::u8>,
						),
						#[codec(index = 7)]
						ChangesTrieSignal(
							runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
						),
						#[codec(index = 0)]
						Other(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						RuntimeEnvironmentUpdated,
					}
				}
				pub mod era {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Era {
						#[codec(index = 0)]
						Immortal,
						#[codec(index = 1)]
						Mortal1(::core::primitive::u8),
						#[codec(index = 2)]
						Mortal2(::core::primitive::u8),
						#[codec(index = 3)]
						Mortal3(::core::primitive::u8),
						#[codec(index = 4)]
						Mortal4(::core::primitive::u8),
						#[codec(index = 5)]
						Mortal5(::core::primitive::u8),
						#[codec(index = 6)]
						Mortal6(::core::primitive::u8),
						#[codec(index = 7)]
						Mortal7(::core::primitive::u8),
						#[codec(index = 8)]
						Mortal8(::core::primitive::u8),
						#[codec(index = 9)]
						Mortal9(::core::primitive::u8),
						#[codec(index = 10)]
						Mortal10(::core::primitive::u8),
						#[codec(index = 11)]
						Mortal11(::core::primitive::u8),
						#[codec(index = 12)]
						Mortal12(::core::primitive::u8),
						#[codec(index = 13)]
						Mortal13(::core::primitive::u8),
						#[codec(index = 14)]
						Mortal14(::core::primitive::u8),
						#[codec(index = 15)]
						Mortal15(::core::primitive::u8),
						#[codec(index = 16)]
						Mortal16(::core::primitive::u8),
						#[codec(index = 17)]
						Mortal17(::core::primitive::u8),
						#[codec(index = 18)]
						Mortal18(::core::primitive::u8),
						#[codec(index = 19)]
						Mortal19(::core::primitive::u8),
						#[codec(index = 20)]
						Mortal20(::core::primitive::u8),
						#[codec(index = 21)]
						Mortal21(::core::primitive::u8),
						#[codec(index = 22)]
						Mortal22(::core::primitive::u8),
						#[codec(index = 23)]
						Mortal23(::core::primitive::u8),
						#[codec(index = 24)]
						Mortal24(::core::primitive::u8),
						#[codec(index = 25)]
						Mortal25(::core::primitive::u8),
						#[codec(index = 26)]
						Mortal26(::core::primitive::u8),
						#[codec(index = 27)]
						Mortal27(::core::primitive::u8),
						#[codec(index = 28)]
						Mortal28(::core::primitive::u8),
						#[codec(index = 29)]
						Mortal29(::core::primitive::u8),
						#[codec(index = 30)]
						Mortal30(::core::primitive::u8),
						#[codec(index = 31)]
						Mortal31(::core::primitive::u8),
						#[codec(index = 32)]
						Mortal32(::core::primitive::u8),
						#[codec(index = 33)]
						Mortal33(::core::primitive::u8),
						#[codec(index = 34)]
						Mortal34(::core::primitive::u8),
						#[codec(index = 35)]
						Mortal35(::core::primitive::u8),
						#[codec(index = 36)]
						Mortal36(::core::primitive::u8),
						#[codec(index = 37)]
						Mortal37(::core::primitive::u8),
						#[codec(index = 38)]
						Mortal38(::core::primitive::u8),
						#[codec(index = 39)]
						Mortal39(::core::primitive::u8),
						#[codec(index = 40)]
						Mortal40(::core::primitive::u8),
						#[codec(index = 41)]
						Mortal41(::core::primitive::u8),
						#[codec(index = 42)]
						Mortal42(::core::primitive::u8),
						#[codec(index = 43)]
						Mortal43(::core::primitive::u8),
						#[codec(index = 44)]
						Mortal44(::core::primitive::u8),
						#[codec(index = 45)]
						Mortal45(::core::primitive::u8),
						#[codec(index = 46)]
						Mortal46(::core::primitive::u8),
						#[codec(index = 47)]
						Mortal47(::core::primitive::u8),
						#[codec(index = 48)]
						Mortal48(::core::primitive::u8),
						#[codec(index = 49)]
						Mortal49(::core::primitive::u8),
						#[codec(index = 50)]
						Mortal50(::core::primitive::u8),
						#[codec(index = 51)]
						Mortal51(::core::primitive::u8),
						#[codec(index = 52)]
						Mortal52(::core::primitive::u8),
						#[codec(index = 53)]
						Mortal53(::core::primitive::u8),
						#[codec(index = 54)]
						Mortal54(::core::primitive::u8),
						#[codec(index = 55)]
						Mortal55(::core::primitive::u8),
						#[codec(index = 56)]
						Mortal56(::core::primitive::u8),
						#[codec(index = 57)]
						Mortal57(::core::primitive::u8),
						#[codec(index = 58)]
						Mortal58(::core::primitive::u8),
						#[codec(index = 59)]
						Mortal59(::core::primitive::u8),
						#[codec(index = 60)]
						Mortal60(::core::primitive::u8),
						#[codec(index = 61)]
						Mortal61(::core::primitive::u8),
						#[codec(index = 62)]
						Mortal62(::core::primitive::u8),
						#[codec(index = 63)]
						Mortal63(::core::primitive::u8),
						#[codec(index = 64)]
						Mortal64(::core::primitive::u8),
						#[codec(index = 65)]
						Mortal65(::core::primitive::u8),
						#[codec(index = 66)]
						Mortal66(::core::primitive::u8),
						#[codec(index = 67)]
						Mortal67(::core::primitive::u8),
						#[codec(index = 68)]
						Mortal68(::core::primitive::u8),
						#[codec(index = 69)]
						Mortal69(::core::primitive::u8),
						#[codec(index = 70)]
						Mortal70(::core::primitive::u8),
						#[codec(index = 71)]
						Mortal71(::core::primitive::u8),
						#[codec(index = 72)]
						Mortal72(::core::primitive::u8),
						#[codec(index = 73)]
						Mortal73(::core::primitive::u8),
						#[codec(index = 74)]
						Mortal74(::core::primitive::u8),
						#[codec(index = 75)]
						Mortal75(::core::primitive::u8),
						#[codec(index = 76)]
						Mortal76(::core::primitive::u8),
						#[codec(index = 77)]
						Mortal77(::core::primitive::u8),
						#[codec(index = 78)]
						Mortal78(::core::primitive::u8),
						#[codec(index = 79)]
						Mortal79(::core::primitive::u8),
						#[codec(index = 80)]
						Mortal80(::core::primitive::u8),
						#[codec(index = 81)]
						Mortal81(::core::primitive::u8),
						#[codec(index = 82)]
						Mortal82(::core::primitive::u8),
						#[codec(index = 83)]
						Mortal83(::core::primitive::u8),
						#[codec(index = 84)]
						Mortal84(::core::primitive::u8),
						#[codec(index = 85)]
						Mortal85(::core::primitive::u8),
						#[codec(index = 86)]
						Mortal86(::core::primitive::u8),
						#[codec(index = 87)]
						Mortal87(::core::primitive::u8),
						#[codec(index = 88)]
						Mortal88(::core::primitive::u8),
						#[codec(index = 89)]
						Mortal89(::core::primitive::u8),
						#[codec(index = 90)]
						Mortal90(::core::primitive::u8),
						#[codec(index = 91)]
						Mortal91(::core::primitive::u8),
						#[codec(index = 92)]
						Mortal92(::core::primitive::u8),
						#[codec(index = 93)]
						Mortal93(::core::primitive::u8),
						#[codec(index = 94)]
						Mortal94(::core::primitive::u8),
						#[codec(index = 95)]
						Mortal95(::core::primitive::u8),
						#[codec(index = 96)]
						Mortal96(::core::primitive::u8),
						#[codec(index = 97)]
						Mortal97(::core::primitive::u8),
						#[codec(index = 98)]
						Mortal98(::core::primitive::u8),
						#[codec(index = 99)]
						Mortal99(::core::primitive::u8),
						#[codec(index = 100)]
						Mortal100(::core::primitive::u8),
						#[codec(index = 101)]
						Mortal101(::core::primitive::u8),
						#[codec(index = 102)]
						Mortal102(::core::primitive::u8),
						#[codec(index = 103)]
						Mortal103(::core::primitive::u8),
						#[codec(index = 104)]
						Mortal104(::core::primitive::u8),
						#[codec(index = 105)]
						Mortal105(::core::primitive::u8),
						#[codec(index = 106)]
						Mortal106(::core::primitive::u8),
						#[codec(index = 107)]
						Mortal107(::core::primitive::u8),
						#[codec(index = 108)]
						Mortal108(::core::primitive::u8),
						#[codec(index = 109)]
						Mortal109(::core::primitive::u8),
						#[codec(index = 110)]
						Mortal110(::core::primitive::u8),
						#[codec(index = 111)]
						Mortal111(::core::primitive::u8),
						#[codec(index = 112)]
						Mortal112(::core::primitive::u8),
						#[codec(index = 113)]
						Mortal113(::core::primitive::u8),
						#[codec(index = 114)]
						Mortal114(::core::primitive::u8),
						#[codec(index = 115)]
						Mortal115(::core::primitive::u8),
						#[codec(index = 116)]
						Mortal116(::core::primitive::u8),
						#[codec(index = 117)]
						Mortal117(::core::primitive::u8),
						#[codec(index = 118)]
						Mortal118(::core::primitive::u8),
						#[codec(index = 119)]
						Mortal119(::core::primitive::u8),
						#[codec(index = 120)]
						Mortal120(::core::primitive::u8),
						#[codec(index = 121)]
						Mortal121(::core::primitive::u8),
						#[codec(index = 122)]
						Mortal122(::core::primitive::u8),
						#[codec(index = 123)]
						Mortal123(::core::primitive::u8),
						#[codec(index = 124)]
						Mortal124(::core::primitive::u8),
						#[codec(index = 125)]
						Mortal125(::core::primitive::u8),
						#[codec(index = 126)]
						Mortal126(::core::primitive::u8),
						#[codec(index = 127)]
						Mortal127(::core::primitive::u8),
						#[codec(index = 128)]
						Mortal128(::core::primitive::u8),
						#[codec(index = 129)]
						Mortal129(::core::primitive::u8),
						#[codec(index = 130)]
						Mortal130(::core::primitive::u8),
						#[codec(index = 131)]
						Mortal131(::core::primitive::u8),
						#[codec(index = 132)]
						Mortal132(::core::primitive::u8),
						#[codec(index = 133)]
						Mortal133(::core::primitive::u8),
						#[codec(index = 134)]
						Mortal134(::core::primitive::u8),
						#[codec(index = 135)]
						Mortal135(::core::primitive::u8),
						#[codec(index = 136)]
						Mortal136(::core::primitive::u8),
						#[codec(index = 137)]
						Mortal137(::core::primitive::u8),
						#[codec(index = 138)]
						Mortal138(::core::primitive::u8),
						#[codec(index = 139)]
						Mortal139(::core::primitive::u8),
						#[codec(index = 140)]
						Mortal140(::core::primitive::u8),
						#[codec(index = 141)]
						Mortal141(::core::primitive::u8),
						#[codec(index = 142)]
						Mortal142(::core::primitive::u8),
						#[codec(index = 143)]
						Mortal143(::core::primitive::u8),
						#[codec(index = 144)]
						Mortal144(::core::primitive::u8),
						#[codec(index = 145)]
						Mortal145(::core::primitive::u8),
						#[codec(index = 146)]
						Mortal146(::core::primitive::u8),
						#[codec(index = 147)]
						Mortal147(::core::primitive::u8),
						#[codec(index = 148)]
						Mortal148(::core::primitive::u8),
						#[codec(index = 149)]
						Mortal149(::core::primitive::u8),
						#[codec(index = 150)]
						Mortal150(::core::primitive::u8),
						#[codec(index = 151)]
						Mortal151(::core::primitive::u8),
						#[codec(index = 152)]
						Mortal152(::core::primitive::u8),
						#[codec(index = 153)]
						Mortal153(::core::primitive::u8),
						#[codec(index = 154)]
						Mortal154(::core::primitive::u8),
						#[codec(index = 155)]
						Mortal155(::core::primitive::u8),
						#[codec(index = 156)]
						Mortal156(::core::primitive::u8),
						#[codec(index = 157)]
						Mortal157(::core::primitive::u8),
						#[codec(index = 158)]
						Mortal158(::core::primitive::u8),
						#[codec(index = 159)]
						Mortal159(::core::primitive::u8),
						#[codec(index = 160)]
						Mortal160(::core::primitive::u8),
						#[codec(index = 161)]
						Mortal161(::core::primitive::u8),
						#[codec(index = 162)]
						Mortal162(::core::primitive::u8),
						#[codec(index = 163)]
						Mortal163(::core::primitive::u8),
						#[codec(index = 164)]
						Mortal164(::core::primitive::u8),
						#[codec(index = 165)]
						Mortal165(::core::primitive::u8),
						#[codec(index = 166)]
						Mortal166(::core::primitive::u8),
						#[codec(index = 167)]
						Mortal167(::core::primitive::u8),
						#[codec(index = 168)]
						Mortal168(::core::primitive::u8),
						#[codec(index = 169)]
						Mortal169(::core::primitive::u8),
						#[codec(index = 170)]
						Mortal170(::core::primitive::u8),
						#[codec(index = 171)]
						Mortal171(::core::primitive::u8),
						#[codec(index = 172)]
						Mortal172(::core::primitive::u8),
						#[codec(index = 173)]
						Mortal173(::core::primitive::u8),
						#[codec(index = 174)]
						Mortal174(::core::primitive::u8),
						#[codec(index = 175)]
						Mortal175(::core::primitive::u8),
						#[codec(index = 176)]
						Mortal176(::core::primitive::u8),
						#[codec(index = 177)]
						Mortal177(::core::primitive::u8),
						#[codec(index = 178)]
						Mortal178(::core::primitive::u8),
						#[codec(index = 179)]
						Mortal179(::core::primitive::u8),
						#[codec(index = 180)]
						Mortal180(::core::primitive::u8),
						#[codec(index = 181)]
						Mortal181(::core::primitive::u8),
						#[codec(index = 182)]
						Mortal182(::core::primitive::u8),
						#[codec(index = 183)]
						Mortal183(::core::primitive::u8),
						#[codec(index = 184)]
						Mortal184(::core::primitive::u8),
						#[codec(index = 185)]
						Mortal185(::core::primitive::u8),
						#[codec(index = 186)]
						Mortal186(::core::primitive::u8),
						#[codec(index = 187)]
						Mortal187(::core::primitive::u8),
						#[codec(index = 188)]
						Mortal188(::core::primitive::u8),
						#[codec(index = 189)]
						Mortal189(::core::primitive::u8),
						#[codec(index = 190)]
						Mortal190(::core::primitive::u8),
						#[codec(index = 191)]
						Mortal191(::core::primitive::u8),
						#[codec(index = 192)]
						Mortal192(::core::primitive::u8),
						#[codec(index = 193)]
						Mortal193(::core::primitive::u8),
						#[codec(index = 194)]
						Mortal194(::core::primitive::u8),
						#[codec(index = 195)]
						Mortal195(::core::primitive::u8),
						#[codec(index = 196)]
						Mortal196(::core::primitive::u8),
						#[codec(index = 197)]
						Mortal197(::core::primitive::u8),
						#[codec(index = 198)]
						Mortal198(::core::primitive::u8),
						#[codec(index = 199)]
						Mortal199(::core::primitive::u8),
						#[codec(index = 200)]
						Mortal200(::core::primitive::u8),
						#[codec(index = 201)]
						Mortal201(::core::primitive::u8),
						#[codec(index = 202)]
						Mortal202(::core::primitive::u8),
						#[codec(index = 203)]
						Mortal203(::core::primitive::u8),
						#[codec(index = 204)]
						Mortal204(::core::primitive::u8),
						#[codec(index = 205)]
						Mortal205(::core::primitive::u8),
						#[codec(index = 206)]
						Mortal206(::core::primitive::u8),
						#[codec(index = 207)]
						Mortal207(::core::primitive::u8),
						#[codec(index = 208)]
						Mortal208(::core::primitive::u8),
						#[codec(index = 209)]
						Mortal209(::core::primitive::u8),
						#[codec(index = 210)]
						Mortal210(::core::primitive::u8),
						#[codec(index = 211)]
						Mortal211(::core::primitive::u8),
						#[codec(index = 212)]
						Mortal212(::core::primitive::u8),
						#[codec(index = 213)]
						Mortal213(::core::primitive::u8),
						#[codec(index = 214)]
						Mortal214(::core::primitive::u8),
						#[codec(index = 215)]
						Mortal215(::core::primitive::u8),
						#[codec(index = 216)]
						Mortal216(::core::primitive::u8),
						#[codec(index = 217)]
						Mortal217(::core::primitive::u8),
						#[codec(index = 218)]
						Mortal218(::core::primitive::u8),
						#[codec(index = 219)]
						Mortal219(::core::primitive::u8),
						#[codec(index = 220)]
						Mortal220(::core::primitive::u8),
						#[codec(index = 221)]
						Mortal221(::core::primitive::u8),
						#[codec(index = 222)]
						Mortal222(::core::primitive::u8),
						#[codec(index = 223)]
						Mortal223(::core::primitive::u8),
						#[codec(index = 224)]
						Mortal224(::core::primitive::u8),
						#[codec(index = 225)]
						Mortal225(::core::primitive::u8),
						#[codec(index = 226)]
						Mortal226(::core::primitive::u8),
						#[codec(index = 227)]
						Mortal227(::core::primitive::u8),
						#[codec(index = 228)]
						Mortal228(::core::primitive::u8),
						#[codec(index = 229)]
						Mortal229(::core::primitive::u8),
						#[codec(index = 230)]
						Mortal230(::core::primitive::u8),
						#[codec(index = 231)]
						Mortal231(::core::primitive::u8),
						#[codec(index = 232)]
						Mortal232(::core::primitive::u8),
						#[codec(index = 233)]
						Mortal233(::core::primitive::u8),
						#[codec(index = 234)]
						Mortal234(::core::primitive::u8),
						#[codec(index = 235)]
						Mortal235(::core::primitive::u8),
						#[codec(index = 236)]
						Mortal236(::core::primitive::u8),
						#[codec(index = 237)]
						Mortal237(::core::primitive::u8),
						#[codec(index = 238)]
						Mortal238(::core::primitive::u8),
						#[codec(index = 239)]
						Mortal239(::core::primitive::u8),
						#[codec(index = 240)]
						Mortal240(::core::primitive::u8),
						#[codec(index = 241)]
						Mortal241(::core::primitive::u8),
						#[codec(index = 242)]
						Mortal242(::core::primitive::u8),
						#[codec(index = 243)]
						Mortal243(::core::primitive::u8),
						#[codec(index = 244)]
						Mortal244(::core::primitive::u8),
						#[codec(index = 245)]
						Mortal245(::core::primitive::u8),
						#[codec(index = 246)]
						Mortal246(::core::primitive::u8),
						#[codec(index = 247)]
						Mortal247(::core::primitive::u8),
						#[codec(index = 248)]
						Mortal248(::core::primitive::u8),
						#[codec(index = 249)]
						Mortal249(::core::primitive::u8),
						#[codec(index = 250)]
						Mortal250(::core::primitive::u8),
						#[codec(index = 251)]
						Mortal251(::core::primitive::u8),
						#[codec(index = 252)]
						Mortal252(::core::primitive::u8),
						#[codec(index = 253)]
						Mortal253(::core::primitive::u8),
						#[codec(index = 254)]
						Mortal254(::core::primitive::u8),
						#[codec(index = 255)]
						Mortal255(::core::primitive::u8),
					}
				}
				pub mod header {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct Header<_0, _1> {
						pub parent_hash: ::subxt::sp_core::H256,
						#[codec(compact)]
						pub number: _0,
						pub state_root: ::subxt::sp_core::H256,
						pub extrinsics_root: ::subxt::sp_core::H256,
						pub digest: runtime_types::sp_runtime::generic::digest::Digest<
							::subxt::sp_core::H256,
						>,
						#[codec(skip)]
						pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
					}
				}
				pub mod unchecked_extrinsic {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						::std::vec::Vec<::core::primitive::u8>,
						#[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
					);
				}
			}
			pub mod multiaddress {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum MultiAddress<_0, _1> {
					#[codec(index = 0)]
					Id(_0),
					#[codec(index = 1)]
					Index(_1),
					#[codec(index = 2)]
					Raw(::std::vec::Vec<::core::primitive::u8>),
					#[codec(index = 3)]
					Address32([::core::primitive::u8; 32usize]),
					#[codec(index = 4)]
					Address20([::core::primitive::u8; 20usize]),
				}
			}
			pub mod traits {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct BlakeTwo256 {}
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module { index: ::core::primitive::u8, error: ::core::primitive::u8 },
				#[codec(index = 4)]
				ConsumerRemaining,
				#[codec(index = 5)]
				NoProviders,
				#[codec(index = 6)]
				Token(runtime_types::sp_runtime::TokenError),
				#[codec(index = 7)]
				Arithmetic(runtime_types::sp_runtime::ArithmeticError),
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum TokenError {
				#[codec(index = 0)]
				NoFunds,
				#[codec(index = 1)]
				WouldDie,
				#[codec(index = 2)]
				BelowMinimum,
				#[codec(index = 3)]
				CannotCreate,
				#[codec(index = 4)]
				UnknownAsset,
				#[codec(index = 5)]
				Frozen,
				#[codec(index = 6)]
				Unsupported,
			}
		}
		pub mod sp_trie {
			use super::runtime_types;
			pub mod storage_proof {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub struct RuntimeVersion {
				pub spec_name: ::std::string::String,
				pub impl_name: ::std::string::String,
				pub authoring_version: ::core::primitive::u32,
				pub spec_version: ::core::primitive::u32,
				pub impl_version: ::core::primitive::u32,
				pub apis:
					::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
				pub transaction_version: ::core::primitive::u32,
			}
		}
		pub mod xcm {
			use super::runtime_types;
			pub mod double_encoded {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v0 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Named(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 2)]
						Index(::core::primitive::u32),
						#[codec(index = 3)]
						Executive,
						#[codec(index = 4)]
						Technical,
						#[codec(index = 5)]
						Legislative,
						#[codec(index = 6)]
						Judicial,
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum BodyPart {
						#[codec(index = 0)]
						Voice,
						#[codec(index = 1)]
						Members {
							#[codec(compact)]
							count: ::core::primitive::u32,
						},
						#[codec(index = 2)]
						Fraction {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						AtLeastProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						MoreThanProportion {
							#[codec(compact)]
							nom: ::core::primitive::u32,
							#[codec(compact)]
							denom: ::core::primitive::u32,
						},
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Junction {
						#[codec(index = 0)]
						Parent,
						#[codec(index = 1)]
						Parachain(::core::primitive::u32),
						#[codec(index = 2)]
						AccountId32 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 3)]
						AccountIndex64 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 4)]
						AccountKey20 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 5)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 6)]
						GeneralIndex(::core::primitive::u128),
						#[codec(index = 7)]
						GeneralKey(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 8)]
						OnlyChild,
						#[codec(index = 9)]
						Plurality {
							id: runtime_types::xcm::v0::junction::BodyId,
							part: runtime_types::xcm::v0::junction::BodyPart,
						},
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum NetworkId {
						#[codec(index = 0)]
						Any,
						#[codec(index = 1)]
						Named(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
					}
				}
				pub mod multi_asset {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum MultiAsset {
						#[codec(index = 0)]
						None,
						#[codec(index = 1)]
						All,
						#[codec(index = 2)]
						AllFungible,
						#[codec(index = 3)]
						AllNonFungible,
						#[codec(index = 4)]
						AllAbstractFungible { id: ::std::vec::Vec<::core::primitive::u8> },
						#[codec(index = 5)]
						AllAbstractNonFungible { class: ::std::vec::Vec<::core::primitive::u8> },
						#[codec(index = 6)]
						AllConcreteFungible {
							id: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 7)]
						AllConcreteNonFungible {
							class: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 8)]
						AbstractFungible {
							id: ::std::vec::Vec<::core::primitive::u8>,
							#[codec(compact)]
							amount: ::core::primitive::u128,
						},
						#[codec(index = 9)]
						AbstractNonFungible {
							class: ::std::vec::Vec<::core::primitive::u8>,
							instance: runtime_types::xcm::v1::multiasset::AssetInstance,
						},
						#[codec(index = 10)]
						ConcreteFungible {
							id: runtime_types::xcm::v0::multi_location::MultiLocation,
							#[codec(compact)]
							amount: ::core::primitive::u128,
						},
						#[codec(index = 11)]
						ConcreteNonFungible {
							class: runtime_types::xcm::v0::multi_location::MultiLocation,
							instance: runtime_types::xcm::v1::multiasset::AssetInstance,
						},
					}
				}
				pub mod multi_location {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum MultiLocation {
						#[codec(index = 0)]
						Null,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v0::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
							runtime_types::xcm::v0::junction::Junction,
						),
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Order {
						#[codec(index = 0)]
						Null,
						#[codec(index = 1)]
						DepositAsset {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
						},
						#[codec(index = 2)]
						DepositReserveAsset {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 3)]
						ExchangeAsset {
							give: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							receive:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						},
						#[codec(index = 4)]
						InitiateReserveWithdraw {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							reserve: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 5)]
						InitiateTeleport {
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
						},
						#[codec(index = 6)]
						QueryHolding {
							#[codec(compact)]
							query_id: ::core::primitive::u64,
							dest: runtime_types::xcm::v0::multi_location::MultiLocation,
							assets:
								::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						},
						#[codec(index = 7)]
						BuyExecution {
							fees: runtime_types::xcm::v0::multi_asset::MultiAsset,
							weight: ::core::primitive::u64,
							debt: ::core::primitive::u64,
							halt_on_error: ::core::primitive::bool,
							xcm: ::std::vec::Vec<runtime_types::xcm::v0::Xcm>,
						},
					}
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum OriginKind {
					#[codec(index = 0)]
					Native,
					#[codec(index = 1)]
					SovereignAccount,
					#[codec(index = 2)]
					Superuser,
					#[codec(index = 3)]
					Xcm,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Xcm {
					#[codec(index = 0)]
					WithdrawAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 1)]
					ReserveAssetDeposit {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 2)]
					TeleportAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v0::Response,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						dest: runtime_types::xcm::v0::multi_location::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
						dest: runtime_types::xcm::v0::multi_location::MultiLocation,
						effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					RelayedFrom {
						who: runtime_types::xcm::v0::multi_location::MultiLocation,
						message: ::std::boxed::Box<runtime_types::xcm::v0::Xcm>,
					},
				}
			}
			pub mod v1 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Junction {
						#[codec(index = 0)]
						Parachain(::core::primitive::u32),
						#[codec(index = 1)]
						AccountId32 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							id: [::core::primitive::u8; 32usize],
						},
						#[codec(index = 2)]
						AccountIndex64 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							#[codec(compact)]
							index: ::core::primitive::u64,
						},
						#[codec(index = 3)]
						AccountKey20 {
							network: runtime_types::xcm::v0::junction::NetworkId,
							key: [::core::primitive::u8; 20usize],
						},
						#[codec(index = 4)]
						PalletInstance(::core::primitive::u8),
						#[codec(index = 5)]
						GeneralIndex(::core::primitive::u128),
						#[codec(index = 6)]
						GeneralKey(::std::vec::Vec<::core::primitive::u8>),
						#[codec(index = 7)]
						OnlyChild,
						#[codec(index = 8)]
						Plurality {
							id: runtime_types::xcm::v0::junction::BodyId,
							part: runtime_types::xcm::v0::junction::BodyPart,
						},
					}
				}
				pub mod multiasset {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum AssetInstance {
						#[codec(index = 0)]
						Undefined,
						#[codec(index = 1)]
						Index(::core::primitive::u128),
						#[codec(index = 2)]
						Array4([::core::primitive::u8; 4usize]),
						#[codec(index = 3)]
						Array8([::core::primitive::u8; 8usize]),
						#[codec(index = 4)]
						Array16([::core::primitive::u8; 16usize]),
						#[codec(index = 5)]
						Array32([::core::primitive::u8; 32usize]),
						#[codec(index = 6)]
						Blob(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v1::multiasset::AssetId,
						pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
					);
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum WildMultiAsset {
						#[codec(index = 0)]
						All,
						#[codec(index = 1)]
						AllOf {
							id: runtime_types::xcm::v1::multiasset::AssetId,
							fun: runtime_types::xcm::v1::multiasset::WildFungibility,
						},
					}
				}
				pub mod multilocation {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Junctions {
						#[codec(index = 0)]
						Here,
						#[codec(index = 1)]
						X1(runtime_types::xcm::v1::junction::Junction),
						#[codec(index = 2)]
						X2(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 3)]
						X3(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 4)]
						X4(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 5)]
						X5(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 6)]
						X6(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 7)]
						X7(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
						#[codec(index = 8)]
						X8(
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
							runtime_types::xcm::v1::junction::Junction,
						),
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v1::multilocation::Junctions,
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Order {
						#[codec(index = 0)]
						Noop,
						#[codec(index = 1)]
						DepositAsset {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							max_assets: ::core::primitive::u32,
							beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
						},
						#[codec(index = 2)]
						DepositReserveAsset {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							max_assets: ::core::primitive::u32,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 3)]
						ExchangeAsset {
							give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							receive: runtime_types::xcm::v1::multiasset::MultiAssets,
						},
						#[codec(index = 4)]
						InitiateReserveWithdraw {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 5)]
						InitiateTeleport {
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
						},
						#[codec(index = 6)]
						QueryHolding {
							#[codec(compact)]
							query_id: ::core::primitive::u64,
							dest: runtime_types::xcm::v1::multilocation::MultiLocation,
							assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						},
						#[codec(index = 7)]
						BuyExecution {
							fees: runtime_types::xcm::v1::multiasset::MultiAsset,
							weight: ::core::primitive::u64,
							debt: ::core::primitive::u64,
							halt_on_error: ::core::primitive::bool,
							instructions: ::std::vec::Vec<runtime_types::xcm::v1::Xcm>,
						},
					}
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					Version(::core::primitive::u32),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Xcm {
					#[codec(index = 0)]
					WithdrawAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 1)]
					ReserveAssetDeposited {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 2)]
					ReceiveTeleportedAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v1::Response,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					RelayedFrom {
						who: runtime_types::xcm::v1::multilocation::Junctions,
						message: ::std::boxed::Box<runtime_types::xcm::v1::Xcm>,
					},
					#[codec(index = 11)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 12)]
					UnsubscribeVersion,
				}
			}
			pub mod v2 {
				use super::runtime_types;
				pub mod traits {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Error {
						#[codec(index = 0)]
						Overflow,
						#[codec(index = 1)]
						Unimplemented,
						#[codec(index = 2)]
						UntrustedReserveLocation,
						#[codec(index = 3)]
						UntrustedTeleportLocation,
						#[codec(index = 4)]
						MultiLocationFull,
						#[codec(index = 5)]
						MultiLocationNotInvertible,
						#[codec(index = 6)]
						BadOrigin,
						#[codec(index = 7)]
						InvalidLocation,
						#[codec(index = 8)]
						AssetNotFound,
						#[codec(index = 9)]
						FailedToTransactAsset,
						#[codec(index = 10)]
						NotWithdrawable,
						#[codec(index = 11)]
						LocationCannotHold,
						#[codec(index = 12)]
						ExceedsMaxMessageSize,
						#[codec(index = 13)]
						DestinationUnsupported,
						#[codec(index = 14)]
						Transport,
						#[codec(index = 15)]
						Unroutable,
						#[codec(index = 16)]
						UnknownClaim,
						#[codec(index = 17)]
						FailedToDecode,
						#[codec(index = 18)]
						TooMuchWeightRequired,
						#[codec(index = 19)]
						NotHoldingFees,
						#[codec(index = 20)]
						TooExpensive,
						#[codec(index = 21)]
						Trap(::core::primitive::u64),
						#[codec(index = 22)]
						UnhandledXcmVersion,
						#[codec(index = 23)]
						WeightLimitReached(::core::primitive::u64),
						#[codec(index = 24)]
						Barrier,
						#[codec(index = 25)]
						WeightNotComputable,
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(::core::primitive::u64),
						#[codec(index = 1)]
						Incomplete(::core::primitive::u64, runtime_types::xcm::v2::traits::Error),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v2::traits::Error),
					}
					#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
					pub enum SendError {
						#[codec(index = 0)]
						CannotReachDestination(
							runtime_types::xcm::v1::multilocation::MultiLocation,
							runtime_types::xcm::v2::Xcm,
						),
						#[codec(index = 1)]
						Transport,
						#[codec(index = 2)]
						Unroutable,
						#[codec(index = 3)]
						DestinationUnsupported,
						#[codec(index = 4)]
						ExceedsMaxMessageSize,
					}
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Instruction {
					#[codec(index = 0)]
					WithdrawAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					ReserveAssetDeposited(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 2)]
					ReceiveTeleportedAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 3)]
					QueryResponse {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						response: runtime_types::xcm::v2::Response,
						#[codec(compact)]
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					TransferAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 5)]
					TransferReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 6)]
					Transact {
						origin_type: runtime_types::xcm::v0::OriginKind,
						#[codec(compact)]
						require_weight_at_most: ::core::primitive::u64,
						call: runtime_types::xcm::double_encoded::DoubleEncoded,
					},
					#[codec(index = 7)]
					HrmpNewChannelOpenRequest {
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						max_message_size: ::core::primitive::u32,
						#[codec(compact)]
						max_capacity: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					HrmpChannelAccepted {
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 9)]
					HrmpChannelClosing {
						#[codec(compact)]
						initiator: ::core::primitive::u32,
						#[codec(compact)]
						sender: ::core::primitive::u32,
						#[codec(compact)]
						recipient: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					ClearOrigin,
					#[codec(index = 11)]
					DescendOrigin(runtime_types::xcm::v1::multilocation::Junctions),
					#[codec(index = 12)]
					ReportError {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 13)]
					DepositAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 14)]
					DepositReserveAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_assets: ::core::primitive::u32,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 15)]
					ExchangeAsset {
						give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						receive: runtime_types::xcm::v1::multiasset::MultiAssets,
					},
					#[codec(index = 16)]
					InitiateReserveWithdraw {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 17)]
					InitiateTeleport {
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						xcm: runtime_types::xcm::v2::Xcm,
					},
					#[codec(index = 18)]
					QueryHolding {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						dest: runtime_types::xcm::v1::multilocation::MultiLocation,
						assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 19)]
					BuyExecution {
						fees: runtime_types::xcm::v1::multiasset::MultiAsset,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 20)]
					RefundSurplus,
					#[codec(index = 21)]
					SetErrorHandler(runtime_types::xcm::v2::Xcm),
					#[codec(index = 22)]
					SetAppendix(runtime_types::xcm::v2::Xcm),
					#[codec(index = 23)]
					ClearError,
					#[codec(index = 24)]
					ClaimAsset {
						assets: runtime_types::xcm::v1::multiasset::MultiAssets,
						ticket: runtime_types::xcm::v1::multilocation::MultiLocation,
					},
					#[codec(index = 25)]
					Trap(::core::primitive::u64),
					#[codec(index = 26)]
					SubscribeVersion {
						#[codec(compact)]
						query_id: ::core::primitive::u64,
						#[codec(compact)]
						max_response_weight: ::core::primitive::u64,
					},
					#[codec(index = 27)]
					UnsubscribeVersion,
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum Response {
					#[codec(index = 0)]
					Null,
					#[codec(index = 1)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 2)]
					ExecutionResult(
						::core::option::Option<(
							::core::primitive::u32,
							runtime_types::xcm::v2::traits::Error,
						)>,
					),
					#[codec(index = 3)]
					Version(::core::primitive::u32),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(::core::primitive::u64),
				}
				#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum VersionedMultiAssets {
				#[codec(index = 0)]
				V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multiasset::MultiAssets),
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum VersionedMultiLocation {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::multi_location::MultiLocation),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multilocation::MultiLocation),
			}
			#[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode)]
			pub enum VersionedXcm {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::Xcm),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::Xcm),
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Xcm),
			}
		}
	}
	#[doc = r" Default configuration of common types for a target Substrate runtime."]
	#[derive(Clone, Debug, Default, Eq, PartialEq)]
	pub struct DefaultConfig;
	impl ::subxt::Config for DefaultConfig {
		type Index = u32;
		type BlockNumber = u32;
		type Hash = ::subxt::sp_core::H256;
		type Hashing = ::subxt::sp_runtime::traits::BlakeTwo256;
		type AccountId = ::subxt::sp_runtime::AccountId32;
		type Address = ::subxt::sp_runtime::MultiAddress<Self::AccountId, u32>;
		type Header = ::subxt::sp_runtime::generic::Header<
			Self::BlockNumber,
			::subxt::sp_runtime::traits::BlakeTwo256,
		>;
		type Signature = ::subxt::sp_runtime::MultiSignature;
		type Extrinsic = ::subxt::sp_runtime::OpaqueExtrinsic;
	}
	impl ::subxt::ExtrinsicExtraData<DefaultConfig> for DefaultConfig {
		type AccountData = AccountData;
		type Extra = ::subxt::DefaultExtra<DefaultConfig>;
	}
	pub type AccountData = self::system::storage::Account;
	impl ::subxt::AccountData<DefaultConfig> for AccountData {
		fn nonce(
			result: &<Self as ::subxt::StorageEntry>::Value,
		) -> <DefaultConfig as ::subxt::Config>::Index {
			result.nonce
		}
		fn storage_entry(account_id: <DefaultConfig as ::subxt::Config>::AccountId) -> Self {
			Self(account_id)
		}
	}
	pub struct RuntimeApi<T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
		pub client: ::subxt::Client<T>,
	}
	impl<T> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T>
	where
		T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
	{
		fn from(client: ::subxt::Client<T>) -> Self {
			Self { client }
		}
	}
	impl<'a, T> RuntimeApi<T>
	where
		T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
	{
		pub fn storage(&'a self) -> StorageApi<'a, T> {
			StorageApi { client: &self.client }
		}
		pub fn tx(&'a self) -> TransactionApi<'a, T> {
			TransactionApi { client: &self.client }
		}
	}
	pub struct StorageApi<'a, T>
	where
		T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
	{
		client: &'a ::subxt::Client<T>,
	}
	impl<'a, T> StorageApi<'a, T>
	where
		T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
	{
		pub fn system(&self) -> system::storage::StorageApi<'a, T> {
			system::storage::StorageApi::new(self.client)
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
			timestamp::storage::StorageApi::new(self.client)
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
			sudo::storage::StorageApi::new(self.client)
		}
		pub fn randomness_collective_flip(
			&self,
		) -> randomness_collective_flip::storage::StorageApi<'a, T> {
			randomness_collective_flip::storage::StorageApi::new(self.client)
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
			transaction_payment::storage::StorageApi::new(self.client)
		}
		pub fn indices(&self) -> indices::storage::StorageApi<'a, T> {
			indices::storage::StorageApi::new(self.client)
		}
		pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
			balances::storage::StorageApi::new(self.client)
		}
		pub fn parachain_system(&self) -> parachain_system::storage::StorageApi<'a, T> {
			parachain_system::storage::StorageApi::new(self.client)
		}
		pub fn parachain_info(&self) -> parachain_info::storage::StorageApi<'a, T> {
			parachain_info::storage::StorageApi::new(self.client)
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
			authorship::storage::StorageApi::new(self.client)
		}
		pub fn collator_selection(&self) -> collator_selection::storage::StorageApi<'a, T> {
			collator_selection::storage::StorageApi::new(self.client)
		}
		pub fn session(&self) -> session::storage::StorageApi<'a, T> {
			session::storage::StorageApi::new(self.client)
		}
		pub fn aura(&self) -> aura::storage::StorageApi<'a, T> {
			aura::storage::StorageApi::new(self.client)
		}
		pub fn council(&self) -> council::storage::StorageApi<'a, T> {
			council::storage::StorageApi::new(self.client)
		}
		pub fn council_membership(&self) -> council_membership::storage::StorageApi<'a, T> {
			council_membership::storage::StorageApi::new(self.client)
		}
		pub fn treasury(&self) -> treasury::storage::StorageApi<'a, T> {
			treasury::storage::StorageApi::new(self.client)
		}
		pub fn democracy(&self) -> democracy::storage::StorageApi<'a, T> {
			democracy::storage::StorageApi::new(self.client)
		}
		pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
			scheduler::storage::StorageApi::new(self.client)
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi<'a, T> {
			xcmp_queue::storage::StorageApi::new(self.client)
		}
		pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi<'a, T> {
			dmp_queue::storage::StorageApi::new(self.client)
		}
		pub fn oracle(&self) -> oracle::storage::StorageApi<'a, T> {
			oracle::storage::StorageApi::new(self.client)
		}
		pub fn tokens(&self) -> tokens::storage::StorageApi<'a, T> {
			tokens::storage::StorageApi::new(self.client)
		}
		pub fn factory(&self) -> factory::storage::StorageApi<'a, T> {
			factory::storage::StorageApi::new(self.client)
		}
		pub fn vault(&self) -> vault::storage::StorageApi<'a, T> {
			vault::storage::StorageApi::new(self.client)
		}
		pub fn lending(&self) -> lending::storage::StorageApi<'a, T> {
			lending::storage::StorageApi::new(self.client)
		}
		pub fn liquid_crowdloan(&self) -> liquid_crowdloan::storage::StorageApi<'a, T> {
			liquid_crowdloan::storage::StorageApi::new(self.client)
		}
		pub fn ping(&self) -> ping::storage::StorageApi<'a, T> {
			ping::storage::StorageApi::new(self.client)
		}
		pub fn assets_registry(&self) -> assets_registry::storage::StorageApi<'a, T> {
			assets_registry::storage::StorageApi::new(self.client)
		}
		pub fn spambot(&self) -> spambot::storage::StorageApi<'a, T> {
			spambot::storage::StorageApi::new(self.client)
		}
		pub fn call_filter(&self) -> call_filter::storage::StorageApi<'a, T> {
			call_filter::storage::StorageApi::new(self.client)
		}
	}
	pub struct TransactionApi<'a, T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>> {
		client: &'a ::subxt::Client<T>,
	}
	impl<'a, T> TransactionApi<'a, T>
	where
		T: ::subxt::Config + ::subxt::ExtrinsicExtraData<T>,
	{
		pub fn system(&self) -> system::calls::TransactionApi<'a, T> {
			system::calls::TransactionApi::new(self.client)
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T> {
			timestamp::calls::TransactionApi::new(self.client)
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T> {
			sudo::calls::TransactionApi::new(self.client)
		}
		pub fn indices(&self) -> indices::calls::TransactionApi<'a, T> {
			indices::calls::TransactionApi::new(self.client)
		}
		pub fn balances(&self) -> balances::calls::TransactionApi<'a, T> {
			balances::calls::TransactionApi::new(self.client)
		}
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi<'a, T> {
			parachain_system::calls::TransactionApi::new(self.client)
		}
		pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T> {
			authorship::calls::TransactionApi::new(self.client)
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi<'a, T> {
			collator_selection::calls::TransactionApi::new(self.client)
		}
		pub fn session(&self) -> session::calls::TransactionApi<'a, T> {
			session::calls::TransactionApi::new(self.client)
		}
		pub fn council(&self) -> council::calls::TransactionApi<'a, T> {
			council::calls::TransactionApi::new(self.client)
		}
		pub fn council_membership(&self) -> council_membership::calls::TransactionApi<'a, T> {
			council_membership::calls::TransactionApi::new(self.client)
		}
		pub fn treasury(&self) -> treasury::calls::TransactionApi<'a, T> {
			treasury::calls::TransactionApi::new(self.client)
		}
		pub fn democracy(&self) -> democracy::calls::TransactionApi<'a, T> {
			democracy::calls::TransactionApi::new(self.client)
		}
		pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T> {
			scheduler::calls::TransactionApi::new(self.client)
		}
		pub fn utility(&self) -> utility::calls::TransactionApi<'a, T> {
			utility::calls::TransactionApi::new(self.client)
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi<'a, T> {
			xcmp_queue::calls::TransactionApi::new(self.client)
		}
		pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi<'a, T> {
			polkadot_xcm::calls::TransactionApi::new(self.client)
		}
		pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi<'a, T> {
			cumulus_xcm::calls::TransactionApi::new(self.client)
		}
		pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi<'a, T> {
			dmp_queue::calls::TransactionApi::new(self.client)
		}
		pub fn oracle(&self) -> oracle::calls::TransactionApi<'a, T> {
			oracle::calls::TransactionApi::new(self.client)
		}
		pub fn tokens(&self) -> tokens::calls::TransactionApi<'a, T> {
			tokens::calls::TransactionApi::new(self.client)
		}
		pub fn vault(&self) -> vault::calls::TransactionApi<'a, T> {
			vault::calls::TransactionApi::new(self.client)
		}
		pub fn lending(&self) -> lending::calls::TransactionApi<'a, T> {
			lending::calls::TransactionApi::new(self.client)
		}
		pub fn liquid_crowdloan(&self) -> liquid_crowdloan::calls::TransactionApi<'a, T> {
			liquid_crowdloan::calls::TransactionApi::new(self.client)
		}
		pub fn liquidations(&self) -> liquidations::calls::TransactionApi<'a, T> {
			liquidations::calls::TransactionApi::new(self.client)
		}
		pub fn ping(&self) -> ping::calls::TransactionApi<'a, T> {
			ping::calls::TransactionApi::new(self.client)
		}
		pub fn assets_registry(&self) -> assets_registry::calls::TransactionApi<'a, T> {
			assets_registry::calls::TransactionApi::new(self.client)
		}
		pub fn spambot(&self) -> spambot::calls::TransactionApi<'a, T> {
			spambot::calls::TransactionApi::new(self.client)
		}
		pub fn call_filter(&self) -> call_filter::calls::TransactionApi<'a, T> {
			call_filter::calls::TransactionApi::new(self.client)
		}
	}
}
