#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 3)]
        Indices(indices::Event),
        #[codec(index = 4)]
        Balances(balances::Event),
        #[codec(index = 7)]
        Offences(offences::Event),
        #[codec(index = 9)]
        Session(session::Event),
        #[codec(index = 10)]
        Grandpa(grandpa::Event),
        #[codec(index = 11)]
        ImOnline(im_online::Event),
        #[codec(index = 16)]
        ParaInclusion(para_inclusion::Event),
        #[codec(index = 19)]
        Paras(paras::Event),
        #[codec(index = 22)]
        Ump(ump::Event),
        #[codec(index = 23)]
        Hrmp(hrmp::Event),
        #[codec(index = 25)]
        ParasDisputes(paras_disputes::Event),
        #[codec(index = 26)]
        Registrar(registrar::Event),
        #[codec(index = 27)]
        Auctions(auctions::Event),
        #[codec(index = 28)]
        Crowdloan(crowdloan::Event),
        #[codec(index = 29)]
        Slots(slots::Event),
        #[codec(index = 31)]
        AssignedSlots(assigned_slots::Event),
        #[codec(index = 32)]
        Sudo(sudo::Event),
        #[codec(index = 36)]
        ValidatorManager(validator_manager::Event),
        #[codec(index = 43)]
        BridgeRococoMessages(bridge_rococo_messages::Event),
        #[codec(index = 44)]
        BridgeWococoMessages(bridge_wococo_messages::Event),
        #[codec(index = 45)]
        BridgeRococoMessagesDispatch(bridge_rococo_messages_dispatch::Event),
        #[codec(index = 46)]
        BridgeWococoMessagesDispatch(bridge_wococo_messages_dispatch::Event),
        #[codec(index = 80)]
        Collective(collective::Event),
        #[codec(index = 81)]
        Membership(membership::Event),
        #[codec(index = 90)]
        Utility(utility::Event),
        #[codec(index = 91)]
        Proxy(proxy::Event),
        #[codec(index = 92)]
        Multisig(multisig::Event),
        #[codec(index = 99)]
        XcmPallet(xcm_pallet::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct fill_block {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for fill_block {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_heap_pages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for set_heap_pages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_code {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_code_without_checks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code_without_checks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_storage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for set_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct kill_storage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for kill_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct kill_prefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for kill_prefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct remark_with_event {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark_with_event {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, fill_block, DispatchError>
                {
                    let call = fill_block { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remark, DispatchError>
                {
                    let call = remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_heap_pages, DispatchError>
                {
                    let call = set_heap_pages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_code, DispatchError>
                {
                    let call = set_code { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_code_without_checks,
                    DispatchError,
                > {
                    let call = set_code_without_checks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_storage, DispatchError>
                {
                    let call = set_storage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_storage, DispatchError>
                {
                    let call = kill_storage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_prefix, DispatchError>
                {
                    let call = kill_prefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remark_with_event, DispatchError>
                {
                    let call = remark_with_event { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KilledAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Remarked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub hash: ::subxt::sp_core::H256,
            }
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
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
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
                        runtime_types::rococo_runtime::Event,
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
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::BasicError,
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
                            runtime_types::rococo_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 242u8, 5u8, 42u8, 1u8, 0u8, 0u8, 0u8, 0u8, 32u8, 74u8, 169u8,
                            209u8, 1u8, 0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                            192u8, 118u8, 108u8, 143u8, 88u8, 1u8, 0u8, 0u8, 1u8, 0u8, 152u8,
                            247u8, 62u8, 93u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8, 192u8,
                            254u8, 190u8, 249u8, 204u8, 1u8, 0u8, 0u8, 1u8, 0u8, 32u8, 74u8, 169u8,
                            209u8, 1u8, 0u8, 0u8, 1u8, 0u8, 136u8, 82u8, 106u8, 116u8, 0u8, 0u8,
                            0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 60u8, 0u8, 0u8, 0u8, 80u8, 0u8, 0u8, 0u8, 80u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[96u8, 9u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            64u8, 120u8, 125u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 225u8, 245u8, 5u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            24u8, 114u8, 111u8, 99u8, 111u8, 99u8, 111u8, 72u8, 112u8, 97u8, 114u8,
                            105u8, 116u8, 121u8, 45u8, 114u8, 111u8, 99u8, 111u8, 99u8, 111u8,
                            45u8, 118u8, 50u8, 46u8, 48u8, 0u8, 0u8, 0u8, 0u8, 182u8, 35u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 80u8, 223u8, 106u8, 203u8, 104u8, 153u8, 7u8,
                            96u8, 155u8, 4u8, 0u8, 0u8, 0u8, 55u8, 227u8, 151u8, 252u8, 124u8,
                            145u8, 245u8, 228u8, 1u8, 0u8, 0u8, 0u8, 64u8, 254u8, 58u8, 212u8, 1u8,
                            248u8, 149u8, 154u8, 5u8, 0u8, 0u8, 0u8, 210u8, 188u8, 152u8, 151u8,
                            238u8, 208u8, 143u8, 21u8, 3u8, 0u8, 0u8, 0u8, 247u8, 139u8, 39u8,
                            139u8, 229u8, 63u8, 69u8, 76u8, 2u8, 0u8, 0u8, 0u8, 175u8, 44u8, 2u8,
                            151u8, 162u8, 62u8, 109u8, 61u8, 2u8, 0u8, 0u8, 0u8, 237u8, 153u8,
                            197u8, 172u8, 178u8, 94u8, 237u8, 245u8, 3u8, 0u8, 0u8, 0u8, 203u8,
                            202u8, 37u8, 227u8, 159u8, 20u8, 35u8, 135u8, 2u8, 0u8, 0u8, 0u8,
                            104u8, 122u8, 212u8, 74u8, 211u8, 127u8, 3u8, 194u8, 1u8, 0u8, 0u8,
                            0u8, 171u8, 60u8, 5u8, 114u8, 41u8, 31u8, 235u8, 139u8, 1u8, 0u8, 0u8,
                            0u8, 73u8, 234u8, 175u8, 27u8, 84u8, 138u8, 12u8, 176u8, 1u8, 0u8, 0u8,
                            0u8, 145u8, 213u8, 223u8, 24u8, 176u8, 210u8, 207u8, 88u8, 1u8, 0u8,
                            0u8, 0u8, 209u8, 250u8, 76u8, 185u8, 116u8, 16u8, 9u8, 23u8, 1u8, 0u8,
                            0u8, 0u8, 229u8, 189u8, 199u8, 82u8, 184u8, 236u8, 43u8, 161u8, 1u8,
                            0u8, 0u8, 0u8, 145u8, 163u8, 198u8, 146u8, 47u8, 208u8, 166u8, 8u8,
                            1u8, 0u8, 0u8, 0u8, 49u8, 40u8, 151u8, 200u8, 97u8, 161u8, 219u8,
                            206u8, 1u8, 0u8, 0u8, 0u8, 182u8, 151u8, 130u8, 196u8, 73u8, 148u8,
                            98u8, 229u8, 1u8, 0u8, 0u8, 0u8, 134u8, 80u8, 31u8, 141u8, 250u8,
                            128u8, 92u8, 111u8, 1u8, 0u8, 0u8, 0u8, 188u8, 157u8, 137u8, 144u8,
                            79u8, 91u8, 146u8, 63u8, 1u8, 0u8, 0u8, 0u8, 55u8, 200u8, 187u8, 19u8,
                            80u8, 169u8, 162u8, 168u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[42u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct report_equivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct report_equivocation_unsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation_unsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct plan_config_change {
                pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            impl ::subxt::Call for plan_config_change {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, report_equivocation, DispatchError>
                {
                    let call = report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation_unsigned,
                    DispatchError,
                > {
                    let call = report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, plan_config_change, DispatchError>
                {
                    let call = plan_config_change { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    [::core::primitive::u8; 32usize],
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
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
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = GenesisSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_epoch_config_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::BasicError,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::BasicError,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn epoch_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn expected_block_time(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[112u8, 23u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[160u8, 134u8, 1u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set, DispatchError>
                {
                    let call = set { now };
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
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[184u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod indices {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct claim {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for claim {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "claim";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct transfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct free {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for free {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "free";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_transfer {
                pub new: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
                pub freeze: ::core::primitive::bool,
            }
            impl ::subxt::Call for force_transfer {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct freeze {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for freeze {
                const PALLET: &'static str = "Indices";
                const FUNCTION: &'static str = "freeze";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn claim(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, claim, DispatchError>
                {
                    let call = claim { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer, DispatchError>
                {
                    let call = transfer { new, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn free(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, free, DispatchError>
                {
                    let call = free { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                    freeze: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_transfer, DispatchError>
                {
                    let call = force_transfer { new, index, freeze };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, freeze, DispatchError>
                {
                    let call = freeze { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_indices::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct IndexAssigned {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Event for IndexAssigned {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexAssigned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct IndexFreed {
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Event for IndexFreed {
                const PALLET: &'static str = "Indices";
                const EVENT: &'static str = "IndexFreed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct IndexFrozen {
                pub index: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
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
                    ::subxt::BasicError,
                > {
                    let entry = Accounts(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn accounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Accounts>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 16u8, 165u8, 212u8, 232u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_balance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_balance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_transfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct transfer_keep_alive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer_keep_alive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct transfer_all {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for transfer_all {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_unreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_unreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer, DispatchError>
                {
                    let call = transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_balance, DispatchError>
                {
                    let call = set_balance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_transfer, DispatchError>
                {
                    let call = force_transfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_keep_alive, DispatchError>
                {
                    let call = transfer_keep_alive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_all, DispatchError>
                {
                    let call = transfer_all { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_unreserve, DispatchError>
                {
                    let call = force_unreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReserveRepatriated {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Slashed {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
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
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
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
                    ::subxt::BasicError,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::BasicError>
                {
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
                    ::subxt::BasicError,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 228u8, 11u8, 84u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
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
                    ::subxt::BasicError,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 225u8, 245u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[5u8][..])?)
                }
                pub fn weight_to_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_support::weights::WeightToFeeCoefficient<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            4u8, 8u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_uncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for set_uncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_uncles, DispatchError>
                {
                    let call = set_uncles { new_uncles };
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
                    ::subxt::BasicError,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn uncle_generations(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod offences {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_offences::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Offence {
                pub kind: [::core::primitive::u8; 16usize],
                pub timeslot: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Event for Offence {
                const PALLET: &'static str = "Offences";
                const EVENT: &'static str = "Offence";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Reports(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Reports {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "Reports";
                type Value = runtime_types::sp_staking::offence::OffenceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    (::subxt::sp_core::crypto::AccountId32, ()),
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ConcurrentReportsIndex(
                pub [::core::primitive::u8; 16usize],
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for ConcurrentReportsIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ConcurrentReportsIndex";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ReportsByKindIndex(pub [::core::primitive::u8; 16usize]);
            impl ::subxt::StorageEntry for ReportsByKindIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ReportsByKindIndex";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
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
                pub async fn reports(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_staking::offence::OffenceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            (::subxt::sp_core::crypto::AccountId32, ()),
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Reports(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reports_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reports>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn concurrent_reports_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = ConcurrentReportsIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn concurrent_reports_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConcurrentReportsIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn reports_by_kind_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ReportsByKindIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reports_by_kind_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReportsByKindIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct HistoricalSessions(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for HistoricalSessions {
                const PALLET: &'static str = "Historical";
                const STORAGE: &'static str = "HistoricalSessions";
                type Value = (::subxt::sp_core::H256, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StoredRange;
            impl ::subxt::StorageEntry for StoredRange {
                const PALLET: &'static str = "Historical";
                const STORAGE: &'static str = "StoredRange";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
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
                pub async fn historical_sessions(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::subxt::sp_core::H256, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = HistoricalSessions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn historical_sessions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HistoricalSessions>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn stored_range(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = StoredRange;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_keys {
                pub keys: runtime_types::rococo_runtime::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct purge_keys;
            impl ::subxt::Call for purge_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_keys(
                    &self,
                    keys: runtime_types::rococo_runtime::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_keys, DispatchError>
                {
                    let call = set_keys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, purge_keys, DispatchError>
                {
                    let call = purge_keys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
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
                    runtime_types::rococo_runtime::SessionKeys,
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
                type Value = runtime_types::rococo_runtime::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner(
                pub runtime_types::sp_core::crypto::KeyTypeId,
                pub ::std::vec::Vec<::core::primitive::u8>,
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
                    ::subxt::BasicError,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::rococo_runtime::SessionKeys,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::rococo_runtime::SessionKeys>,
                    ::subxt::BasicError,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextKeys>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, KeyOwner>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct report_equivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct report_equivocation_unsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation_unsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct note_stalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for note_stalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, report_equivocation, DispatchError>
                {
                    let call = report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation_unsigned,
                    DispatchError,
                > {
                    let call = report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, note_stalled, DispatchError>
                {
                    let call = note_stalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
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
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[160u8, 134u8, 1u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            impl ::subxt::Call for heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, heartbeat, DispatchError>
                {
                    let call = heartbeat {
                        heartbeat,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct HeartbeatReceived {
                pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            }
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AllGood;
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SomeOffline {
                pub offline: ::std::vec::Vec<(::subxt::sp_core::crypto::AccountId32, ())>,
            }
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReceivedHeartbeats {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
                    runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct AuthoredBlocks(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
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
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthoredBlocks>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8][..],
                    )?)
                }
            }
        }
    }
    pub mod authority_discovery {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "AuthorityDiscovery";
                const STORAGE: &'static str = "Keys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_authority_discovery::app::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys;
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "AuthorityDiscovery";
                const STORAGE: &'static str = "NextKeys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_authority_discovery::app::Public,
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
                pub async fn keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_authority_discovery::app::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_authority_discovery::app::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod parachains_origin {
        use super::runtime_types;
    }
    pub mod configuration {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_validation_upgrade_cooldown {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_validation_upgrade_cooldown {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_validation_upgrade_cooldown";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_validation_upgrade_delay {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_validation_upgrade_delay {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_validation_upgrade_delay";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_code_retention_period {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_code_retention_period {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_code_retention_period";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_code_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_code_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_code_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_pov_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_pov_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_pov_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_head_data_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_head_data_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_head_data_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_parathread_cores {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_parathread_cores {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_parathread_cores";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_parathread_retries {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_parathread_retries {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_parathread_retries";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_group_rotation_frequency {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_group_rotation_frequency {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_group_rotation_frequency";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_chain_availability_period {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_chain_availability_period {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_chain_availability_period";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_thread_availability_period {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_thread_availability_period {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_thread_availability_period";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_scheduling_lookahead {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_scheduling_lookahead {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_scheduling_lookahead";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_max_validators_per_core {
                pub new: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for set_max_validators_per_core {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_validators_per_core";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_max_validators {
                pub new: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for set_max_validators {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_validators";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_dispute_period {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_dispute_period {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_period";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_dispute_post_conclusion_acceptance_period {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_dispute_post_conclusion_acceptance_period {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_post_conclusion_acceptance_period";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_dispute_max_spam_slots {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_dispute_max_spam_slots {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_max_spam_slots";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_dispute_conclusion_by_time_out_period {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_dispute_conclusion_by_time_out_period {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_dispute_conclusion_by_time_out_period";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_no_show_slots {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_no_show_slots {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_no_show_slots";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_n_delay_tranches {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_n_delay_tranches {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_n_delay_tranches";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_zeroth_delay_tranche_width {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_zeroth_delay_tranche_width {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_zeroth_delay_tranche_width";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_needed_approvals {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_needed_approvals {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_needed_approvals";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_relay_vrf_modulo_samples {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_relay_vrf_modulo_samples {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_relay_vrf_modulo_samples";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_upward_queue_count {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_upward_queue_count {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_queue_count";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_upward_queue_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_upward_queue_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_queue_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_downward_message_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_downward_message_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_downward_message_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_ump_service_total_weight {
                pub new: ::core::primitive::u64,
            }
            impl ::subxt::Call for set_ump_service_total_weight {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_ump_service_total_weight";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_upward_message_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_upward_message_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_message_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_max_upward_message_num_per_candidate {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_max_upward_message_num_per_candidate {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_max_upward_message_num_per_candidate";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_open_request_ttl {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_open_request_ttl {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_open_request_ttl";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_sender_deposit {
                pub new: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_hrmp_sender_deposit {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_sender_deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_recipient_deposit {
                pub new: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_hrmp_recipient_deposit {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_recipient_deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_channel_max_capacity {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_channel_max_capacity {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_capacity";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_channel_max_total_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_channel_max_total_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_total_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_max_parachain_inbound_channels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_max_parachain_inbound_channels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parachain_inbound_channels";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_max_parathread_inbound_channels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_max_parathread_inbound_channels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parathread_inbound_channels";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_channel_max_message_size {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_channel_max_message_size {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_channel_max_message_size";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_max_parachain_outbound_channels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_max_parachain_outbound_channels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parachain_outbound_channels";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_max_parathread_outbound_channels {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_max_parathread_outbound_channels {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_parathread_outbound_channels";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_hrmp_max_message_num_per_candidate {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_hrmp_max_message_num_per_candidate {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_hrmp_max_message_num_per_candidate";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_ump_max_individual_weight {
                pub new: ::core::primitive::u64,
            }
            impl ::subxt::Call for set_ump_max_individual_weight {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_ump_max_individual_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_pvf_checking_enabled {
                pub new: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_pvf_checking_enabled {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_pvf_checking_enabled";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_pvf_voting_ttl {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_pvf_voting_ttl {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_pvf_voting_ttl";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_minimum_validation_upgrade_delay {
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_minimum_validation_upgrade_delay {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_minimum_validation_upgrade_delay";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_bypass_consistency_check {
                pub new: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_bypass_consistency_check {
                const PALLET: &'static str = "Configuration";
                const FUNCTION: &'static str = "set_bypass_consistency_check";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_validation_upgrade_cooldown(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_validation_upgrade_cooldown,
                    DispatchError,
                > {
                    let call = set_validation_upgrade_cooldown { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validation_upgrade_delay(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_validation_upgrade_delay,
                    DispatchError,
                > {
                    let call = set_validation_upgrade_delay { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_retention_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_code_retention_period,
                    DispatchError,
                > {
                    let call = set_code_retention_period { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_code_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_max_code_size, DispatchError>
                {
                    let call = set_max_code_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_pov_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_max_pov_size, DispatchError>
                {
                    let call = set_max_pov_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_head_data_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_max_head_data_size, DispatchError>
                {
                    let call = set_max_head_data_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parathread_cores(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_parathread_cores, DispatchError>
                {
                    let call = set_parathread_cores { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_parathread_retries(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_parathread_retries, DispatchError>
                {
                    let call = set_parathread_retries { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_group_rotation_frequency(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_group_rotation_frequency,
                    DispatchError,
                > {
                    let call = set_group_rotation_frequency { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_chain_availability_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_chain_availability_period,
                    DispatchError,
                > {
                    let call = set_chain_availability_period { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_thread_availability_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_thread_availability_period,
                    DispatchError,
                > {
                    let call = set_thread_availability_period { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_scheduling_lookahead(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_scheduling_lookahead,
                    DispatchError,
                > {
                    let call = set_scheduling_lookahead { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_validators_per_core(
                    &self,
                    new: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_max_validators_per_core,
                    DispatchError,
                > {
                    let call = set_max_validators_per_core { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_validators(
                    &self,
                    new: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_max_validators, DispatchError>
                {
                    let call = set_max_validators { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_dispute_period, DispatchError>
                {
                    let call = set_dispute_period { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_post_conclusion_acceptance_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_dispute_post_conclusion_acceptance_period,
                    DispatchError,
                > {
                    let call = set_dispute_post_conclusion_acceptance_period { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_max_spam_slots(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_dispute_max_spam_slots,
                    DispatchError,
                > {
                    let call = set_dispute_max_spam_slots { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_dispute_conclusion_by_time_out_period(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_dispute_conclusion_by_time_out_period,
                    DispatchError,
                > {
                    let call = set_dispute_conclusion_by_time_out_period { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_no_show_slots(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_no_show_slots, DispatchError>
                {
                    let call = set_no_show_slots { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_n_delay_tranches(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_n_delay_tranches, DispatchError>
                {
                    let call = set_n_delay_tranches { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_zeroth_delay_tranche_width(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_zeroth_delay_tranche_width,
                    DispatchError,
                > {
                    let call = set_zeroth_delay_tranche_width { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_needed_approvals(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_needed_approvals, DispatchError>
                {
                    let call = set_needed_approvals { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_relay_vrf_modulo_samples(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_relay_vrf_modulo_samples,
                    DispatchError,
                > {
                    let call = set_relay_vrf_modulo_samples { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_queue_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_max_upward_queue_count,
                    DispatchError,
                > {
                    let call = set_max_upward_queue_count { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_queue_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_max_upward_queue_size,
                    DispatchError,
                > {
                    let call = set_max_upward_queue_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_downward_message_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_max_downward_message_size,
                    DispatchError,
                > {
                    let call = set_max_downward_message_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_ump_service_total_weight(
                    &self,
                    new: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_ump_service_total_weight,
                    DispatchError,
                > {
                    let call = set_ump_service_total_weight { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_message_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_max_upward_message_size,
                    DispatchError,
                > {
                    let call = set_max_upward_message_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_max_upward_message_num_per_candidate(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_max_upward_message_num_per_candidate,
                    DispatchError,
                > {
                    let call = set_max_upward_message_num_per_candidate { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_open_request_ttl(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_open_request_ttl,
                    DispatchError,
                > {
                    let call = set_hrmp_open_request_ttl { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_sender_deposit(
                    &self,
                    new: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_sender_deposit,
                    DispatchError,
                > {
                    let call = set_hrmp_sender_deposit { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_recipient_deposit(
                    &self,
                    new: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_recipient_deposit,
                    DispatchError,
                > {
                    let call = set_hrmp_recipient_deposit { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_capacity(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_channel_max_capacity,
                    DispatchError,
                > {
                    let call = set_hrmp_channel_max_capacity { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_total_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_channel_max_total_size,
                    DispatchError,
                > {
                    let call = set_hrmp_channel_max_total_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parachain_inbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_max_parachain_inbound_channels,
                    DispatchError,
                > {
                    let call = set_hrmp_max_parachain_inbound_channels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parathread_inbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_max_parathread_inbound_channels,
                    DispatchError,
                > {
                    let call = set_hrmp_max_parathread_inbound_channels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_channel_max_message_size(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_channel_max_message_size,
                    DispatchError,
                > {
                    let call = set_hrmp_channel_max_message_size { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parachain_outbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_max_parachain_outbound_channels,
                    DispatchError,
                > {
                    let call = set_hrmp_max_parachain_outbound_channels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_parathread_outbound_channels(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_max_parathread_outbound_channels,
                    DispatchError,
                > {
                    let call = set_hrmp_max_parathread_outbound_channels { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_hrmp_max_message_num_per_candidate(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_hrmp_max_message_num_per_candidate,
                    DispatchError,
                > {
                    let call = set_hrmp_max_message_num_per_candidate { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_ump_max_individual_weight(
                    &self,
                    new: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_ump_max_individual_weight,
                    DispatchError,
                > {
                    let call = set_ump_max_individual_weight { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_pvf_checking_enabled(
                    &self,
                    new: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_pvf_checking_enabled,
                    DispatchError,
                > {
                    let call = set_pvf_checking_enabled { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_pvf_voting_ttl(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_pvf_voting_ttl, DispatchError>
                {
                    let call = set_pvf_voting_ttl { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_minimum_validation_upgrade_delay(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_minimum_validation_upgrade_delay,
                    DispatchError,
                > {
                    let call = set_minimum_validation_upgrade_delay { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_bypass_consistency_check(
                    &self,
                    new: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_bypass_consistency_check,
                    DispatchError,
                > {
                    let call = set_bypass_consistency_check { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ActiveConfig;
            impl ::subxt::StorageEntry for ActiveConfig {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "ActiveConfig";
                type Value =
                    runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingConfig(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for PendingConfig {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "PendingConfig";
                type Value = runtime_types :: polkadot_runtime_parachains :: configuration :: migration :: v1 :: HostConfiguration < :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PendingConfigs;
            impl ::subxt::StorageEntry for PendingConfigs {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "PendingConfigs";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
                        ::core::primitive::u32,
                    >,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BypassConsistencyCheck;
            impl ::subxt::StorageEntry for BypassConsistencyCheck {
                const PALLET: &'static str = "Configuration";
                const STORAGE: &'static str = "BypassConsistencyCheck";
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
                pub async fn active_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ActiveConfig;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn pending_config (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: configuration :: migration :: v1 :: HostConfiguration < :: core :: primitive :: u32 > > , :: subxt :: BasicError >{
                    let entry = PendingConfig(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_config_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PendingConfig>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn pending_configs (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: std :: vec :: Vec < (:: core :: primitive :: u32 , runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < :: core :: primitive :: u32 > ,) > , :: subxt :: BasicError >{
                    let entry = PendingConfigs;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bypass_consistency_check(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = BypassConsistencyCheck;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod paras_shared {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct CurrentSessionIndex;
            impl ::subxt::StorageEntry for CurrentSessionIndex {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "CurrentSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveValidatorIndices;
            impl ::subxt::StorageEntry for ActiveValidatorIndices {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "ActiveValidatorIndices";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveValidatorKeys;
            impl ::subxt::StorageEntry for ActiveValidatorKeys {
                const PALLET: &'static str = "ParasShared";
                const STORAGE: &'static str = "ActiveValidatorKeys";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::validator_app::Public>;
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
                pub async fn current_session_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentSessionIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active_validator_indices(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    ::subxt::BasicError,
                > {
                    let entry = ActiveValidatorIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active_validator_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::validator_app::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = ActiveValidatorKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod para_inclusion {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CandidateBacked(
                pub runtime_types::polkadot_primitives::v1::CandidateReceipt<::subxt::sp_core::H256>,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
                pub runtime_types::polkadot_primitives::v1::GroupIndex,
            );
            impl ::subxt::Event for CandidateBacked {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateBacked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CandidateIncluded(
                pub runtime_types::polkadot_primitives::v1::CandidateReceipt<::subxt::sp_core::H256>,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
                pub runtime_types::polkadot_primitives::v1::GroupIndex,
            );
            impl ::subxt::Event for CandidateIncluded {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateIncluded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CandidateTimedOut(
                pub runtime_types::polkadot_primitives::v1::CandidateReceipt<::subxt::sp_core::H256>,
                pub runtime_types::polkadot_parachain::primitives::HeadData,
                pub runtime_types::polkadot_primitives::v1::CoreIndex,
            );
            impl ::subxt::Event for CandidateTimedOut {
                const PALLET: &'static str = "ParaInclusion";
                const EVENT: &'static str = "CandidateTimedOut";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AvailabilityBitfields(
                pub runtime_types::polkadot_primitives::v0::ValidatorIndex,
            );
            impl ::subxt::StorageEntry for AvailabilityBitfields {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "AvailabilityBitfields";
                type Value = runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PendingAvailability(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PendingAvailability {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "PendingAvailability";
                type Value = runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PendingAvailabilityCommitments(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for PendingAvailabilityCommitments {
                const PALLET: &'static str = "ParaInclusion";
                const STORAGE: &'static str = "PendingAvailabilityCommitments";
                type Value = runtime_types::polkadot_primitives::v1::CandidateCommitments<
                    ::core::primitive::u32,
                >;
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
                }                pub async fn availability_bitfields (& self , _0 : runtime_types :: polkadot_primitives :: v0 :: ValidatorIndex , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < :: core :: primitive :: u32 > > , :: subxt :: BasicError >{
                    let entry = AvailabilityBitfields(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn availability_bitfields_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AvailabilityBitfields>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn pending_availability (& self , _0 : runtime_types :: polkadot_parachain :: primitives :: Id , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: sp_core :: H256 , :: core :: primitive :: u32 > > , :: subxt :: BasicError >{
                    let entry = PendingAvailability(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_availability_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PendingAvailability>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn pending_availability_commitments(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::CandidateCommitments<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingAvailabilityCommitments(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_availability_commitments_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PendingAvailabilityCommitments>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod para_inherent {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct enter {
                pub data: runtime_types::polkadot_primitives::v1::InherentData<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for enter {
                const PALLET: &'static str = "ParaInherent";
                const FUNCTION: &'static str = "enter";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn enter(
                    &self,
                    data: runtime_types::polkadot_primitives::v1::InherentData<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, enter, DispatchError>
                {
                    let call = enter { data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Included;
            impl ::subxt::StorageEntry for Included {
                const PALLET: &'static str = "ParaInherent";
                const STORAGE: &'static str = "Included";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OnChainVotes;
            impl ::subxt::StorageEntry for OnChainVotes {
                const PALLET: &'static str = "ParaInherent";
                const STORAGE: &'static str = "OnChainVotes";
                type Value = runtime_types::polkadot_primitives::v1::ScrapedOnChainVotes<
                    ::subxt::sp_core::H256,
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
                pub async fn included(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = Included;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn on_chain_votes(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::ScrapedOnChainVotes<
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = OnChainVotes;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod para_scheduler {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct ValidatorGroups;
            impl ::subxt::StorageEntry for ValidatorGroups {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ValidatorGroups";
                type Value = ::std::vec::Vec<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParathreadQueue;
            impl ::subxt::StorageEntry for ParathreadQueue {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ParathreadQueue";
                type Value =
                    runtime_types::polkadot_runtime_parachains::scheduler::ParathreadClaimQueue;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AvailabilityCores;
            impl ::subxt::StorageEntry for AvailabilityCores {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "AvailabilityCores";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<runtime_types::polkadot_primitives::v1::CoreOccupied>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParathreadClaimIndex;
            impl ::subxt::StorageEntry for ParathreadClaimIndex {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "ParathreadClaimIndex";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SessionStartBlock;
            impl ::subxt::StorageEntry for SessionStartBlock {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "SessionStartBlock";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Scheduled;
            impl ::subxt::StorageEntry for Scheduled {
                const PALLET: &'static str = "ParaScheduler";
                const STORAGE: &'static str = "Scheduled";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_runtime_parachains::scheduler::CoreAssignment,
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
                pub async fn validator_groups(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ValidatorGroups;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parathread_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_runtime_parachains::scheduler::ParathreadClaimQueue,
                    ::subxt::BasicError,
                > {
                    let entry = ParathreadQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn availability_cores(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<
                            runtime_types::polkadot_primitives::v1::CoreOccupied,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = AvailabilityCores;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parathread_claim_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = ParathreadClaimIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn session_start_block(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = SessionStartBlock;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn scheduled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_runtime_parachains::scheduler::CoreAssignment,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Scheduled;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod paras {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_set_current_code {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for force_set_current_code {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_set_current_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_set_current_head {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
            }
            impl ::subxt::Call for force_set_current_head {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_set_current_head";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_schedule_code_upgrade {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                pub relay_parent_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_schedule_code_upgrade {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_schedule_code_upgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_note_new_head {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
            }
            impl ::subxt::Call for force_note_new_head {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_note_new_head";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_queue_action {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for force_queue_action {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "force_queue_action";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct add_trusted_validation_code {
                pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for add_trusted_validation_code {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "add_trusted_validation_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct poke_unused_validation_code {
                pub validation_code_hash:
                    runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            }
            impl ::subxt::Call for poke_unused_validation_code {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "poke_unused_validation_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct include_pvf_check_statement {
                pub stmt: runtime_types::polkadot_primitives::v2::PvfCheckStatement,
                pub signature: runtime_types::polkadot_primitives::v0::validator_app::Signature,
            }
            impl ::subxt::Call for include_pvf_check_statement {
                const PALLET: &'static str = "Paras";
                const FUNCTION: &'static str = "include_pvf_check_statement";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn force_set_current_code(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_set_current_code, DispatchError>
                {
                    let call = force_set_current_code { para, new_code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_current_head(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_set_current_head, DispatchError>
                {
                    let call = force_set_current_head { para, new_head };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_schedule_code_upgrade(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                    relay_parent_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_schedule_code_upgrade,
                    DispatchError,
                > {
                    let call = force_schedule_code_upgrade {
                        para,
                        new_code,
                        relay_parent_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_note_new_head(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_note_new_head, DispatchError>
                {
                    let call = force_note_new_head { para, new_head };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_queue_action(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_queue_action, DispatchError>
                {
                    let call = force_queue_action { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_trusted_validation_code(
                    &self,
                    validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    add_trusted_validation_code,
                    DispatchError,
                > {
                    let call = add_trusted_validation_code { validation_code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn poke_unused_validation_code(
                    &self,
                    validation_code_hash : runtime_types :: polkadot_parachain :: primitives :: ValidationCodeHash,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    poke_unused_validation_code,
                    DispatchError,
                > {
                    let call = poke_unused_validation_code {
                        validation_code_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn include_pvf_check_statement(
                    &self,
                    stmt: runtime_types::polkadot_primitives::v2::PvfCheckStatement,
                    signature: runtime_types::polkadot_primitives::v0::validator_app::Signature,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    include_pvf_check_statement,
                    DispatchError,
                > {
                    let call = include_pvf_check_statement { stmt, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::paras::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CurrentCodeUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for CurrentCodeUpdated {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CurrentCodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CurrentHeadUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for CurrentHeadUpdated {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CurrentHeadUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CodeUpgradeScheduled(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for CodeUpgradeScheduled {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "CodeUpgradeScheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewHeadNoted(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for NewHeadNoted {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "NewHeadNoted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ActionQueued(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for ActionQueued {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "ActionQueued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PvfCheckStarted(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for PvfCheckStarted {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "PvfCheckStarted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PvfCheckAccepted(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for PvfCheckAccepted {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "PvfCheckAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PvfCheckRejected(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for PvfCheckRejected {
                const PALLET: &'static str = "Paras";
                const EVENT: &'static str = "PvfCheckRejected";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PvfActiveVoteMap(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            );
            impl ::subxt::StorageEntry for PvfActiveVoteMap {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PvfActiveVoteMap";
                type Value =
                    runtime_types::polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PvfActiveVoteList;
            impl ::subxt::StorageEntry for PvfActiveVoteList {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PvfActiveVoteList";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Parachains;
            impl ::subxt::StorageEntry for Parachains {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "Parachains";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParaLifecycles(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for ParaLifecycles {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "ParaLifecycles";
                type Value = runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Heads(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Heads {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "Heads";
                type Value = runtime_types::polkadot_parachain::primitives::HeadData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CurrentCodeHash(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for CurrentCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CurrentCodeHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodeHash(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for PastCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodeHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodeMeta(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PastCodeMeta {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodeMeta";
                type Value = runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PastCodePruning;
            impl ::subxt::StorageEntry for PastCodePruning {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "PastCodePruning";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct FutureCodeUpgrades(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for FutureCodeUpgrades {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "FutureCodeUpgrades";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct FutureCodeHash(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for FutureCodeHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "FutureCodeHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCodeHash;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeGoAheadSignal(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for UpgradeGoAheadSignal {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeGoAheadSignal";
                type Value = runtime_types::polkadot_primitives::v1::UpgradeGoAhead;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeRestrictionSignal(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for UpgradeRestrictionSignal {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeRestrictionSignal";
                type Value = runtime_types::polkadot_primitives::v1::UpgradeRestriction;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpgradeCooldowns;
            impl ::subxt::StorageEntry for UpgradeCooldowns {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpgradeCooldowns";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpcomingUpgrades;
            impl ::subxt::StorageEntry for UpcomingUpgrades {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpcomingUpgrades";
                type Value = ::std::vec::Vec<(
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActionsQueue(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ActionsQueue {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "ActionsQueue";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct UpcomingParasGenesis(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for UpcomingParasGenesis {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "UpcomingParasGenesis";
                type Value = runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CodeByHashRefs(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            );
            impl ::subxt::StorageEntry for CodeByHashRefs {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CodeByHashRefs";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CodeByHash(
                pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            );
            impl ::subxt::StorageEntry for CodeByHash {
                const PALLET: &'static str = "Paras";
                const STORAGE: &'static str = "CodeByHash";
                type Value = runtime_types::polkadot_parachain::primitives::ValidationCode;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
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
                pub async fn pvf_active_vote_map(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PvfActiveVoteMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pvf_active_vote_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PvfActiveVoteMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn pvf_active_vote_list(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PvfActiveVoteList;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parachains(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = Parachains;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn para_lifecycles(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ParaLifecycles(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn para_lifecycles_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ParaLifecycles>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn heads(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_parachain::primitives::HeadData>,
                    ::subxt::BasicError,
                > {
                    let entry = Heads(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn heads_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Heads>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentCodeHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_code_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CurrentCodeHash>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn past_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PastCodeHash(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn past_code_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PastCodeHash>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn past_code_meta(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PastCodeMeta(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn past_code_meta_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PastCodeMeta>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn past_code_pruning(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = PastCodePruning;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn future_code_upgrades(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = FutureCodeUpgrades(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn future_code_upgrades_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, FutureCodeUpgrades>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn future_code_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = FutureCodeHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn future_code_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, FutureCodeHash>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn upgrade_go_ahead_signal(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_primitives::v1::UpgradeGoAhead>,
                    ::subxt::BasicError,
                > {
                    let entry = UpgradeGoAheadSignal(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgrade_go_ahead_signal_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpgradeGoAheadSignal>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn upgrade_restriction_signal(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::UpgradeRestriction,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UpgradeRestrictionSignal(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgrade_restriction_signal_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpgradeRestrictionSignal>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn upgrade_cooldowns(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = UpgradeCooldowns;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upcoming_upgrades(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = UpcomingUpgrades;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn actions_queue(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = ActionsQueue(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn actions_queue_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ActionsQueue>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn upcoming_paras_genesis(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UpcomingParasGenesis(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upcoming_paras_genesis_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UpcomingParasGenesis>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn code_by_hash_refs(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CodeByHashRefs(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn code_by_hash_refs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, CodeByHashRefs>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn code_by_hash(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CodeByHash(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn code_by_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, CodeByHash>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8][..],
                    )?)
                }
            }
        }
    }
    pub mod initializer {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct force_approve {
                pub up_to: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_approve {
                const PALLET: &'static str = "Initializer";
                const FUNCTION: &'static str = "force_approve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn force_approve(
                    &self,
                    up_to: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_approve, DispatchError>
                {
                    let call = force_approve { up_to };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HasInitialized;
            impl ::subxt::StorageEntry for HasInitialized {
                const PALLET: &'static str = "Initializer";
                const STORAGE: &'static str = "HasInitialized";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BufferedSessionChanges;
            impl ::subxt::StorageEntry for BufferedSessionChanges {
                const PALLET: &'static str = "Initializer";
                const STORAGE: &'static str = "BufferedSessionChanges";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_runtime_parachains::initializer::BufferedSessionChange,
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
                pub async fn has_initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = HasInitialized;
                    self.client.storage().fetch(&entry, hash).await
                }                pub async fn buffered_session_changes (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: std :: vec :: Vec < runtime_types :: polkadot_runtime_parachains :: initializer :: BufferedSessionChange > , :: subxt :: BasicError >{
                    let entry = BufferedSessionChanges;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod dmp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct DownwardMessageQueues(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for DownwardMessageQueues {
                const PALLET: &'static str = "Dmp";
                const STORAGE: &'static str = "DownwardMessageQueues";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DownwardMessageQueueHeads(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for DownwardMessageQueueHeads {
                const PALLET: &'static str = "Dmp";
                const STORAGE: &'static str = "DownwardMessageQueueHeads";
                type Value = ::subxt::sp_core::H256;
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
                pub async fn downward_message_queues(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = DownwardMessageQueues(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn downward_message_queues_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DownwardMessageQueues>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn downward_message_queue_heads(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = DownwardMessageQueueHeads(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn downward_message_queue_heads_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DownwardMessageQueueHeads>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod ump {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct service_overweight {
                pub index: ::core::primitive::u64,
                pub weight_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for service_overweight {
                const PALLET: &'static str = "Ump";
                const FUNCTION: &'static str = "service_overweight";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, service_overweight, DispatchError>
                {
                    let call = service_overweight {
                        index,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::ump::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for InvalidFormat {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for UnsupportedVersion {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ExecutedUpward(
                pub [::core::primitive::u8; 32usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::Event for ExecutedUpward {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "ExecutedUpward";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct WeightExhausted(
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for WeightExhausted {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "WeightExhausted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UpwardMessagesReceived(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for UpwardMessagesReceived {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "UpwardMessagesReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OverweightEnqueued(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for OverweightEnqueued {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OverweightServiced(pub ::core::primitive::u64, pub ::core::primitive::u64);
            impl ::subxt::Event for OverweightServiced {
                const PALLET: &'static str = "Ump";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RelayDispatchQueues(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for RelayDispatchQueues {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "RelayDispatchQueues";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct RelayDispatchQueueSize(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for RelayDispatchQueueSize {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "RelayDispatchQueueSize";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NeedsDispatch;
            impl ::subxt::StorageEntry for NeedsDispatch {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "NeedsDispatch";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextDispatchRoundStartWith;
            impl ::subxt::StorageEntry for NextDispatchRoundStartWith {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "NextDispatchRoundStartWith";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Overweight(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Overweight {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "Overweight";
                type Value = (
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::std::vec::Vec<::core::primitive::u8>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct OverweightCount;
            impl ::subxt::StorageEntry for OverweightCount {
                const PALLET: &'static str = "Ump";
                const STORAGE: &'static str = "OverweightCount";
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
                pub async fn relay_dispatch_queues(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = RelayDispatchQueues(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relay_dispatch_queues_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, RelayDispatchQueues>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn relay_dispatch_queue_size(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::BasicError,
                > {
                    let entry = RelayDispatchQueueSize(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relay_dispatch_queue_size_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, RelayDispatchQueueSize>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn needs_dispatch(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = NeedsDispatch;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_dispatch_round_start_with(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = NextDispatchRoundStartWith;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Overweight(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Overweight>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn overweight_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = OverweightCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod hrmp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct hrmp_init_open_channel {
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub proposed_max_capacity: ::core::primitive::u32,
                pub proposed_max_message_size: ::core::primitive::u32,
            }
            impl ::subxt::Call for hrmp_init_open_channel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_init_open_channel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct hrmp_accept_open_channel {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for hrmp_accept_open_channel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_accept_open_channel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct hrmp_close_channel {
                pub channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            }
            impl ::subxt::Call for hrmp_close_channel {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_close_channel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_clean_hrmp {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for force_clean_hrmp {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_clean_hrmp";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_process_hrmp_open;
            impl ::subxt::Call for force_process_hrmp_open {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_process_hrmp_open";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_process_hrmp_close;
            impl ::subxt::Call for force_process_hrmp_close {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "force_process_hrmp_close";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct hrmp_cancel_open_request {
                pub channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            }
            impl ::subxt::Call for hrmp_cancel_open_request {
                const PALLET: &'static str = "Hrmp";
                const FUNCTION: &'static str = "hrmp_cancel_open_request";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn hrmp_init_open_channel(
                    &self,
                    recipient: runtime_types::polkadot_parachain::primitives::Id,
                    proposed_max_capacity: ::core::primitive::u32,
                    proposed_max_message_size: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, hrmp_init_open_channel, DispatchError>
                {
                    let call = hrmp_init_open_channel {
                        recipient,
                        proposed_max_capacity,
                        proposed_max_message_size,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_accept_open_channel(
                    &self,
                    sender: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    hrmp_accept_open_channel,
                    DispatchError,
                > {
                    let call = hrmp_accept_open_channel { sender };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_close_channel(
                    &self,
                    channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, hrmp_close_channel, DispatchError>
                {
                    let call = hrmp_close_channel { channel_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clean_hrmp(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_clean_hrmp, DispatchError>
                {
                    let call = force_clean_hrmp { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_process_hrmp_open(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_process_hrmp_open,
                    DispatchError,
                > {
                    let call = force_process_hrmp_open {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_process_hrmp_close(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_process_hrmp_close,
                    DispatchError,
                > {
                    let call = force_process_hrmp_close {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn hrmp_cancel_open_request(
                    &self,
                    channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    hrmp_cancel_open_request,
                    DispatchError,
                > {
                    let call = hrmp_cancel_open_request { channel_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenChannelRequested(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for OpenChannelRequested {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelRequested";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenChannelCanceled(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::Event for OpenChannelCanceled {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelCanceled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpenChannelAccepted(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::Event for OpenChannelAccepted {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "OpenChannelAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ChannelClosed(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::Event for ChannelClosed {
                const PALLET: &'static str = "Hrmp";
                const EVENT: &'static str = "ChannelClosed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HrmpOpenChannelRequests(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpOpenChannelRequests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequests";
                type Value =
                    runtime_types::polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpOpenChannelRequestsList;
            impl ::subxt::StorageEntry for HrmpOpenChannelRequestsList {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequestsList";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpOpenChannelRequestCount(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpOpenChannelRequestCount {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpOpenChannelRequestCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpAcceptedChannelRequestCount(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpAcceptedChannelRequestCount {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpAcceptedChannelRequestCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpCloseChannelRequests(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpCloseChannelRequests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpCloseChannelRequests";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpCloseChannelRequestsList;
            impl ::subxt::StorageEntry for HrmpCloseChannelRequestsList {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpCloseChannelRequestsList";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpWatermarks(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for HrmpWatermarks {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpWatermarks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannels(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpChannels {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannels";
                type Value = runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpIngressChannelsIndex(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpIngressChannelsIndex {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpIngressChannelsIndex";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpEgressChannelsIndex(
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for HrmpEgressChannelsIndex {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpEgressChannelsIndex";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannelContents(
                pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
            );
            impl ::subxt::StorageEntry for HrmpChannelContents {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannelContents";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                        ::core::primitive::u32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct HrmpChannelDigests(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for HrmpChannelDigests {
                const PALLET: &'static str = "Hrmp";
                const STORAGE: &'static str = "HrmpChannelDigests";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                )>;
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
                pub async fn hrmp_open_channel_requests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpOpenChannelRequests(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_open_channel_requests_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpOpenChannelRequests>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_open_channel_requests_list(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpOpenChannelRequestsList;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_open_channel_request_count(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HrmpOpenChannelRequestCount(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_open_channel_request_count_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpOpenChannelRequestCount>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_accepted_channel_request_count(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HrmpAcceptedChannelRequestCount(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_accepted_channel_request_count_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpAcceptedChannelRequestCount>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_close_channel_requests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = HrmpCloseChannelRequests(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_close_channel_requests_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpCloseChannelRequests>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_close_channel_requests_list(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpCloseChannelRequestsList;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_watermarks(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpWatermarks(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_watermarks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpWatermarks>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_channels(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpChannels(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn hrmp_channels_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpChannels>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_ingress_channels_index(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpIngressChannelsIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_ingress_channels_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpIngressChannelsIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_egress_channels_index(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpEgressChannelsIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_egress_channels_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpEgressChannelsIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_channel_contents(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpChannelContents(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_channel_contents_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpChannelContents>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn hrmp_channel_digests(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpChannelDigests(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_channel_digests_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, HrmpChannelDigests>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod para_session_info {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct AssignmentKeysUnsafe;
            impl ::subxt::StorageEntry for AssignmentKeysUnsafe {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "AssignmentKeysUnsafe";
                type Value =
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v1::assignment_app::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EarliestStoredSession;
            impl ::subxt::StorageEntry for EarliestStoredSession {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "EarliestStoredSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Sessions(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Sessions {
                const PALLET: &'static str = "ParaSessionInfo";
                const STORAGE: &'static str = "Sessions";
                type Value = runtime_types::polkadot_primitives::v2::SessionInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
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
                pub async fn assignment_keys_unsafe(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v1::assignment_app::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = AssignmentKeysUnsafe;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn earliest_stored_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = EarliestStoredSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn sessions(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_primitives::v2::SessionInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = Sessions(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn sessions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Sessions>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod paras_disputes {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_unfreeze;
            impl ::subxt::Call for force_unfreeze {
                const PALLET: &'static str = "ParasDisputes";
                const FUNCTION: &'static str = "force_unfreeze";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn force_unfreeze(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_unfreeze, DispatchError>
                {
                    let call = force_unfreeze {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_parachains::disputes::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DisputeInitiated(
                pub runtime_types::polkadot_core_primitives::CandidateHash,
                pub runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation,
            );
            impl ::subxt::Event for DisputeInitiated {
                const PALLET: &'static str = "ParasDisputes";
                const EVENT: &'static str = "DisputeInitiated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DisputeConcluded(
                pub runtime_types::polkadot_core_primitives::CandidateHash,
                pub runtime_types::polkadot_runtime_parachains::disputes::DisputeResult,
            );
            impl ::subxt::Event for DisputeConcluded {
                const PALLET: &'static str = "ParasDisputes";
                const EVENT: &'static str = "DisputeConcluded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DisputeTimedOut(pub runtime_types::polkadot_core_primitives::CandidateHash);
            impl ::subxt::Event for DisputeTimedOut {
                const PALLET: &'static str = "ParasDisputes";
                const EVENT: &'static str = "DisputeTimedOut";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Revert(pub ::core::primitive::u32);
            impl ::subxt::Event for Revert {
                const PALLET: &'static str = "ParasDisputes";
                const EVENT: &'static str = "Revert";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct LastPrunedSession;
            impl ::subxt::StorageEntry for LastPrunedSession {
                const PALLET: &'static str = "ParasDisputes";
                const STORAGE: &'static str = "LastPrunedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Disputes(
                pub ::core::primitive::u32,
                pub runtime_types::polkadot_core_primitives::CandidateHash,
            );
            impl ::subxt::StorageEntry for Disputes {
                const PALLET: &'static str = "ParasDisputes";
                const STORAGE: &'static str = "Disputes";
                type Value =
                    runtime_types::polkadot_primitives::v1::DisputeState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Included(
                pub ::core::primitive::u32,
                pub runtime_types::polkadot_core_primitives::CandidateHash,
            );
            impl ::subxt::StorageEntry for Included {
                const PALLET: &'static str = "ParasDisputes";
                const STORAGE: &'static str = "Included";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct SpamSlots(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for SpamSlots {
                const PALLET: &'static str = "ParasDisputes";
                const STORAGE: &'static str = "SpamSlots";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Frozen;
            impl ::subxt::StorageEntry for Frozen {
                const PALLET: &'static str = "ParasDisputes";
                const STORAGE: &'static str = "Frozen";
                type Value = ::core::option::Option<::core::primitive::u32>;
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
                pub async fn last_pruned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = LastPrunedSession;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn disputes(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::polkadot_core_primitives::CandidateHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::DisputeState<
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Disputes(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn disputes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Disputes>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn included(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::polkadot_core_primitives::CandidateHash,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = Included(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn included_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Included>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn spam_slots(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u32>>,
                    ::subxt::BasicError,
                > {
                    let entry = SpamSlots(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn spam_slots_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SpamSlots>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn frozen(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = Frozen;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod registrar {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct register {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for register {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "register";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_register {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub deposit: ::core::primitive::u128,
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
            }
            impl ::subxt::Call for force_register {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "force_register";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct deregister {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for deregister {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "deregister";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct swap {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub other: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for swap {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "swap";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_remove_lock {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for force_remove_lock {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "force_remove_lock";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct reserve;
            impl ::subxt::Call for reserve {
                const PALLET: &'static str = "Registrar";
                const FUNCTION: &'static str = "reserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn register(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, register, DispatchError>
                {
                    let call = register {
                        id,
                        genesis_head,
                        validation_code,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_register(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_register, DispatchError>
                {
                    let call = force_register {
                        who,
                        deposit,
                        id,
                        genesis_head,
                        validation_code,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deregister(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deregister, DispatchError>
                {
                    let call = deregister { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn swap(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    other: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, swap, DispatchError>
                {
                    let call = swap { id, other };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_remove_lock(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_remove_lock, DispatchError>
                {
                    let call = force_remove_lock { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reserve(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, reserve, DispatchError>
                {
                    let call = reserve {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Registered(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Registered {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Registered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Deregistered(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Deregistered {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Deregistered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Reserved(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Registrar";
                const EVENT: &'static str = "Reserved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PendingSwap(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PendingSwap {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "PendingSwap";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Paras(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Paras {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "Paras";
                type Value = runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
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
            pub struct NextFreeParaId;
            impl ::subxt::StorageEntry for NextFreeParaId {
                const PALLET: &'static str = "Registrar";
                const STORAGE: &'static str = "NextFreeParaId";
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
                pub async fn pending_swap(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = PendingSwap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_swap_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PendingSwap>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn paras(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Paras(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn paras_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Paras>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_free_para_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::subxt::BasicError,
                > {
                    let entry = NextFreeParaId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn para_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 80u8, 57u8, 39u8, 140u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn data_deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            128u8, 240u8, 250u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod auctions {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct new_auction {
                #[codec(compact)]
                pub duration: ::core::primitive::u32,
                #[codec(compact)]
                pub lease_period_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for new_auction {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "new_auction";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct bid {
                #[codec(compact)]
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub auction_index: ::core::primitive::u32,
                #[codec(compact)]
                pub first_slot: ::core::primitive::u32,
                #[codec(compact)]
                pub last_slot: ::core::primitive::u32,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for bid {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "bid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct cancel_auction;
            impl ::subxt::Call for cancel_auction {
                const PALLET: &'static str = "Auctions";
                const FUNCTION: &'static str = "cancel_auction";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn new_auction(
                    &self,
                    duration: ::core::primitive::u32,
                    lease_period_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, new_auction, DispatchError>
                {
                    let call = new_auction {
                        duration,
                        lease_period_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bid(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    auction_index: ::core::primitive::u32,
                    first_slot: ::core::primitive::u32,
                    last_slot: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, bid, DispatchError>
                {
                    let call = bid {
                        para,
                        auction_index,
                        first_slot,
                        last_slot,
                        amount,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_auction(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_auction, DispatchError>
                {
                    let call = cancel_auction {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::auctions::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AuctionStarted(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for AuctionStarted {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "AuctionStarted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AuctionClosed(pub ::core::primitive::u32);
            impl ::subxt::Event for AuctionClosed {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "AuctionClosed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReserveConfiscated(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for ReserveConfiscated {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "ReserveConfiscated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BidAccepted(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u128,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for BidAccepted {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "BidAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct WinningOffset(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for WinningOffset {
                const PALLET: &'static str = "Auctions";
                const EVENT: &'static str = "WinningOffset";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AuctionCounter;
            impl ::subxt::StorageEntry for AuctionCounter {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "AuctionCounter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuctionInfo;
            impl ::subxt::StorageEntry for AuctionInfo {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "AuctionInfo";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedAmounts(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
            );
            impl ::subxt::StorageEntry for ReservedAmounts {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "ReservedAmounts";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Winning(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Winning {
                const PALLET: &'static str = "Auctions";
                const STORAGE: &'static str = "Winning";
                type Value = [::core::option::Option<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u128,
                )>; 36usize];
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
                pub async fn auction_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AuctionCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn auction_info(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = AuctionInfo;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_amounts(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = ReservedAmounts(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_amounts_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReservedAmounts>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn winning(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        [::core::option::Option<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                        )>; 36usize],
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Winning(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn winning_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Winning>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn ending_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[88u8, 2u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn sample_length(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[1u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn slot_range_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[36u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn lease_periods_per_slot(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[8u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod crowdloan {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct create {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub cap: ::core::primitive::u128,
                #[codec(compact)]
                pub first_period: ::core::primitive::u32,
                #[codec(compact)]
                pub last_period: ::core::primitive::u32,
                #[codec(compact)]
                pub end: ::core::primitive::u32,
                pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
            }
            impl ::subxt::Call for create {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct contribute {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
            }
            impl ::subxt::Call for contribute {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "contribute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for withdraw {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct refund {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for refund {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "refund";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct dissolve {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for dissolve {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "dissolve";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct edit {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                #[codec(compact)]
                pub cap: ::core::primitive::u128,
                #[codec(compact)]
                pub first_period: ::core::primitive::u32,
                #[codec(compact)]
                pub last_period: ::core::primitive::u32,
                #[codec(compact)]
                pub end: ::core::primitive::u32,
                pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
            }
            impl ::subxt::Call for edit {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "edit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct add_memo {
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                pub memo: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for add_memo {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "add_memo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct poke {
                pub index: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for poke {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "poke";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct contribute_all {
                #[codec(compact)]
                pub index: runtime_types::polkadot_parachain::primitives::Id,
                pub signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
            }
            impl ::subxt::Call for contribute_all {
                const PALLET: &'static str = "Crowdloan";
                const FUNCTION: &'static str = "contribute_all";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn create(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    cap: ::core::primitive::u128,
                    first_period: ::core::primitive::u32,
                    last_period: ::core::primitive::u32,
                    end: ::core::primitive::u32,
                    verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create, DispatchError>
                {
                    let call = create {
                        index,
                        cap,
                        first_period,
                        last_period,
                        end,
                        verifier,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn contribute(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    value: ::core::primitive::u128,
                    signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, contribute, DispatchError>
                {
                    let call = contribute {
                        index,
                        value,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, withdraw, DispatchError>
                {
                    let call = withdraw { who, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn refund(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, refund, DispatchError>
                {
                    let call = refund { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dissolve(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, dissolve, DispatchError>
                {
                    let call = dissolve { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn edit(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    cap: ::core::primitive::u128,
                    first_period: ::core::primitive::u32,
                    last_period: ::core::primitive::u32,
                    end: ::core::primitive::u32,
                    verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, edit, DispatchError>
                {
                    let call = edit {
                        index,
                        cap,
                        first_period,
                        last_period,
                        end,
                        verifier,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_memo(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    memo: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, add_memo, DispatchError>
                {
                    let call = add_memo { index, memo };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn poke(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, poke, DispatchError>
                {
                    let call = poke { index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn contribute_all(
                    &self,
                    index: runtime_types::polkadot_parachain::primitives::Id,
                    signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, contribute_all, DispatchError>
                {
                    let call = contribute_all { index, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::crowdloan::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Created(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Contributed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Contributed {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Contributed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Withdrew(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdrew {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Withdrew";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PartiallyRefunded(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for PartiallyRefunded {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "PartiallyRefunded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AllRefunded(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for AllRefunded {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "AllRefunded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Dissolved(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Dissolved {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Dissolved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct HandleBidResult(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for HandleBidResult {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "HandleBidResult";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Edited(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for Edited {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "Edited";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MemoUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for MemoUpdated {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "MemoUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AddedToNewRaise(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for AddedToNewRaise {
                const PALLET: &'static str = "Crowdloan";
                const EVENT: &'static str = "AddedToNewRaise";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Funds(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Funds {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "Funds";
                type Value = runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct NewRaise;
            impl ::subxt::StorageEntry for NewRaise {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "NewRaise";
                type Value = ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EndingsCount;
            impl ::subxt::StorageEntry for EndingsCount {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "EndingsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextTrieIndex;
            impl ::subxt::StorageEntry for NextTrieIndex {
                const PALLET: &'static str = "Crowdloan";
                const STORAGE: &'static str = "NextTrieIndex";
                type Value = ::core::primitive::u32;
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
                pub async fn funds(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Funds(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn funds_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Funds>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn new_raise(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
                    ::subxt::BasicError,
                > {
                    let entry = NewRaise;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn endings_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = EndingsCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_trie_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = NextTrieIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[112u8, 121u8, 47u8, 99u8, 102u8, 117u8, 110u8, 100u8][..],
                    )?)
                }
                pub fn min_contribution(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 16u8, 165u8, 212u8, 232u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn remove_keys_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[244u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod slots {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_lease {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
                pub leaser: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub period_begin: ::core::primitive::u32,
                pub period_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_lease {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "force_lease";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct clear_all_leases {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for clear_all_leases {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "clear_all_leases";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct trigger_onboard {
                pub para: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for trigger_onboard {
                const PALLET: &'static str = "Slots";
                const FUNCTION: &'static str = "trigger_onboard";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn force_lease(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                    leaser: ::subxt::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                    period_begin: ::core::primitive::u32,
                    period_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_lease, DispatchError>
                {
                    let call = force_lease {
                        para,
                        leaser,
                        amount,
                        period_begin,
                        period_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_all_leases(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, clear_all_leases, DispatchError>
                {
                    let call = clear_all_leases { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn trigger_onboard(
                    &self,
                    para: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, trigger_onboard, DispatchError>
                {
                    let call = trigger_onboard { para };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::slots::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewLeasePeriod(pub ::core::primitive::u32);
            impl ::subxt::Event for NewLeasePeriod {
                const PALLET: &'static str = "Slots";
                const EVENT: &'static str = "NewLeasePeriod";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Leased(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Leased {
                const PALLET: &'static str = "Slots";
                const EVENT: &'static str = "Leased";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Leases(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for Leases {
                const PALLET: &'static str = "Slots";
                const STORAGE: &'static str = "Leases";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                >;
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
                pub async fn leases(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Leases(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn leases_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Leases>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn lease_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[64u8, 56u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn lease_offset(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod paras_sudo_wrapper {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_schedule_para_initialize {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub genesis: runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
            }
            impl ::subxt::Call for sudo_schedule_para_initialize {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_para_initialize";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_schedule_para_cleanup {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for sudo_schedule_para_cleanup {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_para_cleanup";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_schedule_parathread_upgrade {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for sudo_schedule_parathread_upgrade {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_parathread_upgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_schedule_parachain_downgrade {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for sudo_schedule_parachain_downgrade {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_schedule_parachain_downgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_queue_downward_xcm {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub xcm: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
            }
            impl ::subxt::Call for sudo_queue_downward_xcm {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_queue_downward_xcm";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_establish_hrmp_channel {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub max_capacity: ::core::primitive::u32,
                pub max_message_size: ::core::primitive::u32,
            }
            impl ::subxt::Call for sudo_establish_hrmp_channel {
                const PALLET: &'static str = "ParasSudoWrapper";
                const FUNCTION: &'static str = "sudo_establish_hrmp_channel";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn sudo_schedule_para_initialize(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    genesis: runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_schedule_para_initialize,
                    DispatchError,
                > {
                    let call = sudo_schedule_para_initialize { id, genesis };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_schedule_para_cleanup(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_schedule_para_cleanup,
                    DispatchError,
                > {
                    let call = sudo_schedule_para_cleanup { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_schedule_parathread_upgrade(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_schedule_parathread_upgrade,
                    DispatchError,
                > {
                    let call = sudo_schedule_parathread_upgrade { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_schedule_parachain_downgrade(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_schedule_parachain_downgrade,
                    DispatchError,
                > {
                    let call = sudo_schedule_parachain_downgrade { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_queue_downward_xcm(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    xcm: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_queue_downward_xcm,
                    DispatchError,
                > {
                    let call = sudo_queue_downward_xcm {
                        id,
                        xcm: ::std::boxed::Box::new(xcm),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_establish_hrmp_channel(
                    &self,
                    sender: runtime_types::polkadot_parachain::primitives::Id,
                    recipient: runtime_types::polkadot_parachain::primitives::Id,
                    max_capacity: ::core::primitive::u32,
                    max_message_size: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_establish_hrmp_channel,
                    DispatchError,
                > {
                    let call = sudo_establish_hrmp_channel {
                        sender,
                        recipient,
                        max_capacity,
                        max_message_size,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
    }
    pub mod assigned_slots {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct assign_perm_parachain_slot {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for assign_perm_parachain_slot {
                const PALLET: &'static str = "AssignedSlots";
                const FUNCTION: &'static str = "assign_perm_parachain_slot";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct assign_temp_parachain_slot {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
                pub lease_period_start:
                    runtime_types::polkadot_runtime_common::assigned_slots::SlotLeasePeriodStart,
            }
            impl ::subxt::Call for assign_temp_parachain_slot {
                const PALLET: &'static str = "AssignedSlots";
                const FUNCTION: &'static str = "assign_temp_parachain_slot";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct unassign_parachain_slot {
                pub id: runtime_types::polkadot_parachain::primitives::Id,
            }
            impl ::subxt::Call for unassign_parachain_slot {
                const PALLET: &'static str = "AssignedSlots";
                const FUNCTION: &'static str = "unassign_parachain_slot";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn assign_perm_parachain_slot(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    assign_perm_parachain_slot,
                    DispatchError,
                > {
                    let call = assign_perm_parachain_slot { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn assign_temp_parachain_slot(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    lease_period_start : runtime_types :: polkadot_runtime_common :: assigned_slots :: SlotLeasePeriodStart,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    assign_temp_parachain_slot,
                    DispatchError,
                > {
                    let call = assign_temp_parachain_slot {
                        id,
                        lease_period_start,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unassign_parachain_slot(
                    &self,
                    id: runtime_types::polkadot_parachain::primitives::Id,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    unassign_parachain_slot,
                    DispatchError,
                > {
                    let call = unassign_parachain_slot { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::polkadot_runtime_common::assigned_slots::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PermanentSlotAssigned(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for PermanentSlotAssigned {
                const PALLET: &'static str = "AssignedSlots";
                const EVENT: &'static str = "PermanentSlotAssigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct TemporarySlotAssigned(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::Event for TemporarySlotAssigned {
                const PALLET: &'static str = "AssignedSlots";
                const EVENT: &'static str = "TemporarySlotAssigned";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PermanentSlots(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for PermanentSlots {
                const PALLET: &'static str = "AssignedSlots";
                const STORAGE: &'static str = "PermanentSlots";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PermanentSlotCount;
            impl ::subxt::StorageEntry for PermanentSlotCount {
                const PALLET: &'static str = "AssignedSlots";
                const STORAGE: &'static str = "PermanentSlotCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct TemporarySlots(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for TemporarySlots {
                const PALLET: &'static str = "AssignedSlots";
                const STORAGE: &'static str = "TemporarySlots";
                type Value =
                    runtime_types::polkadot_runtime_common::assigned_slots::ParachainTemporarySlot<
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
            pub struct TemporarySlotCount;
            impl ::subxt::StorageEntry for TemporarySlotCount {
                const PALLET: &'static str = "AssignedSlots";
                const STORAGE: &'static str = "TemporarySlotCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveTemporarySlotCount;
            impl ::subxt::StorageEntry for ActiveTemporarySlotCount {
                const PALLET: &'static str = "AssignedSlots";
                const STORAGE: &'static str = "ActiveTemporarySlotCount";
                type Value = ::core::primitive::u32;
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
                pub async fn permanent_slots(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = PermanentSlots(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn permanent_slots_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PermanentSlots>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn permanent_slot_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = PermanentSlotCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn temporary_slots (& self , _0 : runtime_types :: polkadot_parachain :: primitives :: Id , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: polkadot_runtime_common :: assigned_slots :: ParachainTemporarySlot < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , :: subxt :: BasicError >{
                    let entry = TemporarySlots(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn temporary_slots_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, TemporarySlots>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn temporary_slot_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = TemporarySlotCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn active_temporary_slot_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ActiveTemporarySlotCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn permanent_slot_lease_period_length(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[109u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn temporary_slot_lease_period_length(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[3u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_permanent_slots(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[25u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_temporary_slots(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[20u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_temporary_slot_per_lease_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[5u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo {
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_unchecked_weight {
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for sudo_unchecked_weight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_key {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for set_key {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct sudo_as {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for sudo_as {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::rococo_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo, DispatchError>
                {
                    let call = sudo {
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::rococo_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo_unchecked_weight, DispatchError>
                {
                    let call = sudo_unchecked_weight {
                        call: ::std::boxed::Box::new(call),
                        weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_key, DispatchError>
                {
                    let call = set_key { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::rococo_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo_as, DispatchError>
                {
                    let call = sudo_as {
                        who,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
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
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Key;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod mmr {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RootHash;
            impl ::subxt::StorageEntry for RootHash {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "RootHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NumberOfLeaves;
            impl ::subxt::StorageEntry for NumberOfLeaves {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "NumberOfLeaves";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nodes(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Nodes {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "Nodes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
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
                pub async fn root_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = RootHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn number_of_leaves(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = NumberOfLeaves;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nodes(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = Nodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nodes>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod beefy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "Authorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSetId;
            impl ::subxt::StorageEntry for ValidatorSetId {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "ValidatorSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
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
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = ValidatorSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod mmr_leaf {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct BeefyNextAuthorities;
            impl ::subxt::StorageEntry for BeefyNextAuthorities {
                const PALLET: &'static str = "MmrLeaf";
                const STORAGE: &'static str = "BeefyNextAuthorities";
                type Value = runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                    ::subxt::sp_core::H256,
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
                pub async fn beefy_next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::beefy_primitives::mmr::BeefyNextAuthoritySet<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = BeefyNextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod validator_manager {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct register_validators {
                pub validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for register_validators {
                const PALLET: &'static str = "ValidatorManager";
                const FUNCTION: &'static str = "register_validators";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct deregister_validators {
                pub validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for deregister_validators {
                const PALLET: &'static str = "ValidatorManager";
                const FUNCTION: &'static str = "deregister_validators";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn register_validators(
                    &self,
                    validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, register_validators, DispatchError>
                {
                    let call = register_validators { validators };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deregister_validators(
                    &self,
                    validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deregister_validators, DispatchError>
                {
                    let call = deregister_validators { validators };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::rococo_runtime::validator_manager::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ValidatorsRegistered(
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for ValidatorsRegistered {
                const PALLET: &'static str = "ValidatorManager";
                const EVENT: &'static str = "ValidatorsRegistered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ValidatorsDeregistered(
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for ValidatorsDeregistered {
                const PALLET: &'static str = "ValidatorManager";
                const EVENT: &'static str = "ValidatorsDeregistered";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ValidatorsToRetire;
            impl ::subxt::StorageEntry for ValidatorsToRetire {
                const PALLET: &'static str = "ValidatorManager";
                const STORAGE: &'static str = "ValidatorsToRetire";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorsToAdd;
            impl ::subxt::StorageEntry for ValidatorsToAdd {
                const PALLET: &'static str = "ValidatorManager";
                const STORAGE: &'static str = "ValidatorsToAdd";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
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
                pub async fn validators_to_retire(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = ValidatorsToRetire;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validators_to_add(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = ValidatorsToAdd;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod bridge_rococo_grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct submit_finality_proof {
                pub finality_target: ::std::boxed::Box<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
                pub justification:
                    runtime_types::bp_header_chain::justification::GrandpaJustification<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
            }
            impl ::subxt::Call for submit_finality_proof {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const FUNCTION: &'static str = "submit_finality_proof";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct initialize {
                pub init_data: runtime_types::bp_header_chain::InitializationData<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for initialize {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const FUNCTION: &'static str = "initialize";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_owner {
                pub new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_owner {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const FUNCTION: &'static str = "set_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_operational {
                pub operational: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_operational {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const FUNCTION: &'static str = "set_operational";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_finality_proof(
                    &self,
                    finality_target: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    justification : runtime_types :: bp_header_chain :: justification :: GrandpaJustification < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, submit_finality_proof, DispatchError>
                {
                    let call = submit_finality_proof {
                        finality_target: ::std::boxed::Box::new(finality_target),
                        justification,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn initialize(
                    &self,
                    init_data: runtime_types::bp_header_chain::InitializationData<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, initialize, DispatchError>
                {
                    let call = initialize { init_data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_owner(
                    &self,
                    new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_owner, DispatchError>
                {
                    let call = set_owner { new_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_operational(
                    &self,
                    operational: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_operational, DispatchError>
                {
                    let call = set_operational { operational };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RequestCount;
            impl ::subxt::StorageEntry for RequestCount {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "RequestCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InitialHash;
            impl ::subxt::StorageEntry for InitialHash {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "InitialHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BestFinalized;
            impl ::subxt::StorageEntry for BestFinalized {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "BestFinalized";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ImportedHashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ImportedHashes {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "ImportedHashes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ImportedHashesPointer;
            impl ::subxt::StorageEntry for ImportedHashesPointer {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "ImportedHashesPointer";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ImportedHeaders(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for ImportedHeaders {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "ImportedHeaders";
                type Value = runtime_types::sp_runtime::generic::header::Header<
                    ::core::primitive::u32,
                    runtime_types::sp_runtime::traits::BlakeTwo256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CurrentAuthoritySet;
            impl ::subxt::StorageEntry for CurrentAuthoritySet {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "CurrentAuthoritySet";
                type Value = runtime_types::bp_header_chain::AuthoritySet;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PalletOwner;
            impl ::subxt::StorageEntry for PalletOwner {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "PalletOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct IsHalted;
            impl ::subxt::StorageEntry for IsHalted {
                const PALLET: &'static str = "BridgeRococoGrandpa";
                const STORAGE: &'static str = "IsHalted";
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
                pub async fn request_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = RequestCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn initial_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = InitialHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn best_finalized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BestFinalized;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn imported_hashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = ImportedHashes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn imported_hashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ImportedHashes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn imported_hashes_pointer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ImportedHashesPointer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn imported_headers(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ImportedHeaders(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn imported_headers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ImportedHeaders>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_authority_set(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_header_chain::AuthoritySet,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentAuthoritySet;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pallet_owner(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOwner;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn is_halted(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = IsHalted;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_requests(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[96u8, 9u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn headers_to_keep(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 137u8, 1u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod bridge_wococo_grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct submit_finality_proof {
                pub finality_target: ::std::boxed::Box<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
                pub justification:
                    runtime_types::bp_header_chain::justification::GrandpaJustification<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
            }
            impl ::subxt::Call for submit_finality_proof {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const FUNCTION: &'static str = "submit_finality_proof";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct initialize {
                pub init_data: runtime_types::bp_header_chain::InitializationData<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for initialize {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const FUNCTION: &'static str = "initialize";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_owner {
                pub new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_owner {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const FUNCTION: &'static str = "set_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_operational {
                pub operational: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_operational {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const FUNCTION: &'static str = "set_operational";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_finality_proof(
                    &self,
                    finality_target: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    justification : runtime_types :: bp_header_chain :: justification :: GrandpaJustification < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, submit_finality_proof, DispatchError>
                {
                    let call = submit_finality_proof {
                        finality_target: ::std::boxed::Box::new(finality_target),
                        justification,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn initialize(
                    &self,
                    init_data: runtime_types::bp_header_chain::InitializationData<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, initialize, DispatchError>
                {
                    let call = initialize { init_data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_owner(
                    &self,
                    new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_owner, DispatchError>
                {
                    let call = set_owner { new_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_operational(
                    &self,
                    operational: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_operational, DispatchError>
                {
                    let call = set_operational { operational };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RequestCount;
            impl ::subxt::StorageEntry for RequestCount {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "RequestCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InitialHash;
            impl ::subxt::StorageEntry for InitialHash {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "InitialHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BestFinalized;
            impl ::subxt::StorageEntry for BestFinalized {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "BestFinalized";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ImportedHashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ImportedHashes {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "ImportedHashes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ImportedHashesPointer;
            impl ::subxt::StorageEntry for ImportedHashesPointer {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "ImportedHashesPointer";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ImportedHeaders(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for ImportedHeaders {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "ImportedHeaders";
                type Value = runtime_types::sp_runtime::generic::header::Header<
                    ::core::primitive::u32,
                    runtime_types::sp_runtime::traits::BlakeTwo256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CurrentAuthoritySet;
            impl ::subxt::StorageEntry for CurrentAuthoritySet {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "CurrentAuthoritySet";
                type Value = runtime_types::bp_header_chain::AuthoritySet;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PalletOwner;
            impl ::subxt::StorageEntry for PalletOwner {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "PalletOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct IsHalted;
            impl ::subxt::StorageEntry for IsHalted {
                const PALLET: &'static str = "BridgeWococoGrandpa";
                const STORAGE: &'static str = "IsHalted";
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
                pub async fn request_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = RequestCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn initial_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = InitialHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn best_finalized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BestFinalized;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn imported_hashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = ImportedHashes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn imported_hashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ImportedHashes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn imported_hashes_pointer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ImportedHashesPointer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn imported_headers(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ImportedHeaders(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn imported_headers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ImportedHeaders>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_authority_set(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_header_chain::AuthoritySet,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentAuthoritySet;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pallet_owner(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOwner;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn is_halted(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = IsHalted;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_requests(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[96u8, 9u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn headers_to_keep(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 137u8, 1u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod bridge_rococo_messages {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_owner {
                pub new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_owner {
                const PALLET: &'static str = "BridgeRococoMessages";
                const FUNCTION: &'static str = "set_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_operating_mode {
                pub operating_mode: runtime_types::bp_messages::OperatingMode,
            }
            impl ::subxt::Call for set_operating_mode {
                const PALLET: &'static str = "BridgeRococoMessages";
                const FUNCTION: &'static str = "set_operating_mode";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct update_pallet_parameter {
                pub parameter: (),
            }
            impl ::subxt::Call for update_pallet_parameter {
                const PALLET: &'static str = "BridgeRococoMessages";
                const FUNCTION: &'static str = "update_pallet_parameter";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct send_message {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub payload: runtime_types::bp_message_dispatch::MessagePayload<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_runtime::MultiSigner,
                    runtime_types::sp_runtime::MultiSignature,
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub delivery_and_dispatch_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for send_message {
                const PALLET: &'static str = "BridgeRococoMessages";
                const FUNCTION: &'static str = "send_message";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct increase_message_fee {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub nonce: ::core::primitive::u64,
                pub additional_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for increase_message_fee {
                const PALLET: &'static str = "BridgeRococoMessages";
                const FUNCTION: &'static str = "increase_message_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct receive_messages_proof { pub relayer_id_at_bridged_chain : :: subxt :: sp_core :: crypto :: AccountId32 , pub proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 > , pub messages_count : :: core :: primitive :: u32 , pub dispatch_weight : :: core :: primitive :: u64 , }
            impl ::subxt::Call for receive_messages_proof {
                const PALLET: &'static str = "BridgeRococoMessages";
                const FUNCTION: &'static str = "receive_messages_proof";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct receive_messages_delivery_proof { pub proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 > , pub relayers_state : runtime_types :: bp_messages :: UnrewardedRelayersState , }
            impl ::subxt::Call for receive_messages_delivery_proof {
                const PALLET: &'static str = "BridgeRococoMessages";
                const FUNCTION: &'static str = "receive_messages_delivery_proof";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_owner(
                    &self,
                    new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_owner, DispatchError>
                {
                    let call = set_owner { new_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_operating_mode(
                    &self,
                    operating_mode: runtime_types::bp_messages::OperatingMode,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_operating_mode, DispatchError>
                {
                    let call = set_operating_mode { operating_mode };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_pallet_parameter(
                    &self,
                    parameter: (),
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    update_pallet_parameter,
                    DispatchError,
                > {
                    let call = update_pallet_parameter { parameter };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn send_message(
                    &self,
                    lane_id: [::core::primitive::u8; 4usize],
                    payload: runtime_types::bp_message_dispatch::MessagePayload<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_runtime::MultiSigner,
                        runtime_types::sp_runtime::MultiSignature,
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    delivery_and_dispatch_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, send_message, DispatchError>
                {
                    let call = send_message {
                        lane_id,
                        payload,
                        delivery_and_dispatch_fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_message_fee(
                    &self,
                    lane_id: [::core::primitive::u8; 4usize],
                    nonce: ::core::primitive::u64,
                    additional_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, increase_message_fee, DispatchError>
                {
                    let call = increase_message_fee {
                        lane_id,
                        nonce,
                        additional_fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn receive_messages_proof(
                    &self,
                    relayer_id_at_bridged_chain: ::subxt::sp_core::crypto::AccountId32,
                    proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 >,
                    messages_count: ::core::primitive::u32,
                    dispatch_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, receive_messages_proof, DispatchError>
                {
                    let call = receive_messages_proof {
                        relayer_id_at_bridged_chain,
                        proof,
                        messages_count,
                        dispatch_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn receive_messages_delivery_proof(
                    &self,
                    proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 >,
                    relayers_state: runtime_types::bp_messages::UnrewardedRelayersState,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    receive_messages_delivery_proof,
                    DispatchError,
                > {
                    let call = receive_messages_delivery_proof {
                        proof,
                        relayers_state,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bridge_messages::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ParameterUpdated(pub ());
            impl ::subxt::Event for ParameterUpdated {
                const PALLET: &'static str = "BridgeRococoMessages";
                const EVENT: &'static str = "ParameterUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageAccepted(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageAccepted {
                const PALLET: &'static str = "BridgeRococoMessages";
                const EVENT: &'static str = "MessageAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessagesDelivered(
                pub [::core::primitive::u8; 4usize],
                pub runtime_types::bp_messages::DeliveredMessages,
            );
            impl ::subxt::Event for MessagesDelivered {
                const PALLET: &'static str = "BridgeRococoMessages";
                const EVENT: &'static str = "MessagesDelivered";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PalletOwner;
            impl ::subxt::StorageEntry for PalletOwner {
                const PALLET: &'static str = "BridgeRococoMessages";
                const STORAGE: &'static str = "PalletOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PalletOperatingMode;
            impl ::subxt::StorageEntry for PalletOperatingMode {
                const PALLET: &'static str = "BridgeRococoMessages";
                const STORAGE: &'static str = "PalletOperatingMode";
                type Value = runtime_types::bp_messages::OperatingMode;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InboundLanes(pub [::core::primitive::u8; 4usize]);
            impl ::subxt::StorageEntry for InboundLanes {
                const PALLET: &'static str = "BridgeRococoMessages";
                const STORAGE: &'static str = "InboundLanes";
                type Value = runtime_types::bp_messages::InboundLaneData<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct OutboundLanes(pub [::core::primitive::u8; 4usize]);
            impl ::subxt::StorageEntry for OutboundLanes {
                const PALLET: &'static str = "BridgeRococoMessages";
                const STORAGE: &'static str = "OutboundLanes";
                type Value = runtime_types::bp_messages::OutboundLaneData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct OutboundMessages(pub runtime_types::bp_messages::MessageKey);
            impl ::subxt::StorageEntry for OutboundMessages {
                const PALLET: &'static str = "BridgeRococoMessages";
                const STORAGE: &'static str = "OutboundMessages";
                type Value = runtime_types::bp_messages::MessageData<::core::primitive::u128>;
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
                pub async fn pallet_owner(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOwner;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pallet_operating_mode(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::OperatingMode,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOperatingMode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_lanes(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::InboundLaneData<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = InboundLanes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_lanes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InboundLanes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_lanes(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::OutboundLaneData,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundLanes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn outbound_lanes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundLanes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_messages(
                    &self,
                    _0: runtime_types::bp_messages::MessageKey,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::bp_messages::MessageData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundMessages(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn outbound_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundMessages>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn bridged_chain_id(
                    &self,
                ) -> ::core::result::Result<[::core::primitive::u8; 4usize], ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[114u8, 111u8, 99u8, 111u8][..],
                    )?)
                }
            }
        }
    }
    pub mod bridge_wococo_messages {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_owner {
                pub new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_owner {
                const PALLET: &'static str = "BridgeWococoMessages";
                const FUNCTION: &'static str = "set_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_operating_mode {
                pub operating_mode: runtime_types::bp_messages::OperatingMode,
            }
            impl ::subxt::Call for set_operating_mode {
                const PALLET: &'static str = "BridgeWococoMessages";
                const FUNCTION: &'static str = "set_operating_mode";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct update_pallet_parameter {
                pub parameter: (),
            }
            impl ::subxt::Call for update_pallet_parameter {
                const PALLET: &'static str = "BridgeWococoMessages";
                const FUNCTION: &'static str = "update_pallet_parameter";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct send_message {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub payload: runtime_types::bp_message_dispatch::MessagePayload<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_runtime::MultiSigner,
                    runtime_types::sp_runtime::MultiSignature,
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub delivery_and_dispatch_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for send_message {
                const PALLET: &'static str = "BridgeWococoMessages";
                const FUNCTION: &'static str = "send_message";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct increase_message_fee {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub nonce: ::core::primitive::u64,
                pub additional_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for increase_message_fee {
                const PALLET: &'static str = "BridgeWococoMessages";
                const FUNCTION: &'static str = "increase_message_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct receive_messages_proof { pub relayer_id_at_bridged_chain : :: subxt :: sp_core :: crypto :: AccountId32 , pub proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 > , pub messages_count : :: core :: primitive :: u32 , pub dispatch_weight : :: core :: primitive :: u64 , }
            impl ::subxt::Call for receive_messages_proof {
                const PALLET: &'static str = "BridgeWococoMessages";
                const FUNCTION: &'static str = "receive_messages_proof";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct receive_messages_delivery_proof { pub proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 > , pub relayers_state : runtime_types :: bp_messages :: UnrewardedRelayersState , }
            impl ::subxt::Call for receive_messages_delivery_proof {
                const PALLET: &'static str = "BridgeWococoMessages";
                const FUNCTION: &'static str = "receive_messages_delivery_proof";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_owner(
                    &self,
                    new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_owner, DispatchError>
                {
                    let call = set_owner { new_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_operating_mode(
                    &self,
                    operating_mode: runtime_types::bp_messages::OperatingMode,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_operating_mode, DispatchError>
                {
                    let call = set_operating_mode { operating_mode };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_pallet_parameter(
                    &self,
                    parameter: (),
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    update_pallet_parameter,
                    DispatchError,
                > {
                    let call = update_pallet_parameter { parameter };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn send_message(
                    &self,
                    lane_id: [::core::primitive::u8; 4usize],
                    payload: runtime_types::bp_message_dispatch::MessagePayload<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_runtime::MultiSigner,
                        runtime_types::sp_runtime::MultiSignature,
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    delivery_and_dispatch_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, send_message, DispatchError>
                {
                    let call = send_message {
                        lane_id,
                        payload,
                        delivery_and_dispatch_fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_message_fee(
                    &self,
                    lane_id: [::core::primitive::u8; 4usize],
                    nonce: ::core::primitive::u64,
                    additional_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, increase_message_fee, DispatchError>
                {
                    let call = increase_message_fee {
                        lane_id,
                        nonce,
                        additional_fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn receive_messages_proof(
                    &self,
                    relayer_id_at_bridged_chain: ::subxt::sp_core::crypto::AccountId32,
                    proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 >,
                    messages_count: ::core::primitive::u32,
                    dispatch_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, receive_messages_proof, DispatchError>
                {
                    let call = receive_messages_proof {
                        relayer_id_at_bridged_chain,
                        proof,
                        messages_count,
                        dispatch_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn receive_messages_delivery_proof(
                    &self,
                    proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 >,
                    relayers_state: runtime_types::bp_messages::UnrewardedRelayersState,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    receive_messages_delivery_proof,
                    DispatchError,
                > {
                    let call = receive_messages_delivery_proof {
                        proof,
                        relayers_state,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bridge_messages::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ParameterUpdated(pub ());
            impl ::subxt::Event for ParameterUpdated {
                const PALLET: &'static str = "BridgeWococoMessages";
                const EVENT: &'static str = "ParameterUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageAccepted(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageAccepted {
                const PALLET: &'static str = "BridgeWococoMessages";
                const EVENT: &'static str = "MessageAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessagesDelivered(
                pub [::core::primitive::u8; 4usize],
                pub runtime_types::bp_messages::DeliveredMessages,
            );
            impl ::subxt::Event for MessagesDelivered {
                const PALLET: &'static str = "BridgeWococoMessages";
                const EVENT: &'static str = "MessagesDelivered";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PalletOwner;
            impl ::subxt::StorageEntry for PalletOwner {
                const PALLET: &'static str = "BridgeWococoMessages";
                const STORAGE: &'static str = "PalletOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PalletOperatingMode;
            impl ::subxt::StorageEntry for PalletOperatingMode {
                const PALLET: &'static str = "BridgeWococoMessages";
                const STORAGE: &'static str = "PalletOperatingMode";
                type Value = runtime_types::bp_messages::OperatingMode;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InboundLanes(pub [::core::primitive::u8; 4usize]);
            impl ::subxt::StorageEntry for InboundLanes {
                const PALLET: &'static str = "BridgeWococoMessages";
                const STORAGE: &'static str = "InboundLanes";
                type Value = runtime_types::bp_messages::InboundLaneData<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct OutboundLanes(pub [::core::primitive::u8; 4usize]);
            impl ::subxt::StorageEntry for OutboundLanes {
                const PALLET: &'static str = "BridgeWococoMessages";
                const STORAGE: &'static str = "OutboundLanes";
                type Value = runtime_types::bp_messages::OutboundLaneData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct OutboundMessages(pub runtime_types::bp_messages::MessageKey);
            impl ::subxt::StorageEntry for OutboundMessages {
                const PALLET: &'static str = "BridgeWococoMessages";
                const STORAGE: &'static str = "OutboundMessages";
                type Value = runtime_types::bp_messages::MessageData<::core::primitive::u128>;
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
                pub async fn pallet_owner(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOwner;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pallet_operating_mode(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::OperatingMode,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOperatingMode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_lanes(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::InboundLaneData<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = InboundLanes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_lanes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InboundLanes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_lanes(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::OutboundLaneData,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundLanes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn outbound_lanes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundLanes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_messages(
                    &self,
                    _0: runtime_types::bp_messages::MessageKey,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::bp_messages::MessageData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundMessages(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn outbound_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundMessages>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn bridged_chain_id(
                    &self,
                ) -> ::core::result::Result<[::core::primitive::u8; 4usize], ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[119u8, 111u8, 99u8, 111u8][..],
                    )?)
                }
            }
        }
    }
    pub mod bridge_rococo_messages_dispatch {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_bridge_dispatch::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageRejected(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageRejected {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageRejected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageVersionSpecMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for MessageVersionSpecMismatch {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageVersionSpecMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageWeightMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageWeightMismatch {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageWeightMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageSignatureMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageSignatureMismatch {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageSignatureMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageCallDecodeFailed(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageCallDecodeFailed {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageCallDecodeFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageCallRejected(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageCallRejected {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageCallRejected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageDispatchPaymentFailed(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageDispatchPaymentFailed {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageDispatchPaymentFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageDispatched(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MessageDispatched {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "MessageDispatched";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct _Dummy;
            impl ::subxt::Event for _Dummy {
                const PALLET: &'static str = "BridgeRococoMessagesDispatch";
                const EVENT: &'static str = "_Dummy";
            }
        }
    }
    pub mod bridge_wococo_messages_dispatch {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_bridge_dispatch::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageRejected(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageRejected {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageRejected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageVersionSpecMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for MessageVersionSpecMismatch {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageVersionSpecMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageWeightMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageWeightMismatch {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageWeightMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageSignatureMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageSignatureMismatch {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageSignatureMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageCallDecodeFailed(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageCallDecodeFailed {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageCallDecodeFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageCallRejected(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageCallRejected {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageCallRejected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageDispatchPaymentFailed(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageDispatchPaymentFailed {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageDispatchPaymentFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageDispatched(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MessageDispatched {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "MessageDispatched";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct _Dummy;
            impl ::subxt::Event for _Dummy {
                const PALLET: &'static str = "BridgeWococoMessagesDispatch";
                const EVENT: &'static str = "_Dummy";
            }
        }
    }
    pub mod collective {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_members {
                pub new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                pub old_count: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_members {
                const PALLET: &'static str = "Collective";
                const FUNCTION: &'static str = "set_members";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct execute {
                pub proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for execute {
                const PALLET: &'static str = "Collective";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct propose {
                #[codec(compact)]
                pub threshold: ::core::primitive::u32,
                pub proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for propose {
                const PALLET: &'static str = "Collective";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct vote {
                pub proposal: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub approve: ::core::primitive::bool,
            }
            impl ::subxt::Call for vote {
                const PALLET: &'static str = "Collective";
                const FUNCTION: &'static str = "vote";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct close {
                pub proposal_hash: ::subxt::sp_core::H256,
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                #[codec(compact)]
                pub proposal_weight_bound: ::core::primitive::u64,
                #[codec(compact)]
                pub length_bound: ::core::primitive::u32,
            }
            impl ::subxt::Call for close {
                const PALLET: &'static str = "Collective";
                const FUNCTION: &'static str = "close";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct disapprove_proposal {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for disapprove_proposal {
                const PALLET: &'static str = "Collective";
                const FUNCTION: &'static str = "disapprove_proposal";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_members(
                    &self,
                    new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    old_count: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_members, DispatchError>
                {
                    let call = set_members {
                        new_members,
                        prime,
                        old_count,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    proposal: runtime_types::rococo_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, execute, DispatchError>
                {
                    let call = execute {
                        proposal: ::std::boxed::Box::new(proposal),
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn propose(
                    &self,
                    threshold: ::core::primitive::u32,
                    proposal: runtime_types::rococo_runtime::Call,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, propose, DispatchError>
                {
                    let call = propose {
                        threshold,
                        proposal: ::std::boxed::Box::new(proposal),
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn vote(
                    &self,
                    proposal: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, vote, DispatchError>
                {
                    let call = vote {
                        proposal,
                        index,
                        approve,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn close(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: ::core::primitive::u64,
                    length_bound: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, close, DispatchError>
                {
                    let call = close {
                        proposal_hash,
                        index,
                        proposal_weight_bound,
                        length_bound,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn disapprove_proposal(
                    &self,
                    proposal_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, disapprove_proposal, DispatchError>
                {
                    let call = disapprove_proposal { proposal_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_collective::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Proposed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub proposal_index: ::core::primitive::u32,
                pub proposal_hash: ::subxt::sp_core::H256,
                pub threshold: ::core::primitive::u32,
            }
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Voted {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub proposal_hash: ::subxt::sp_core::H256,
                pub voted: ::core::primitive::bool,
                pub yes: ::core::primitive::u32,
                pub no: ::core::primitive::u32,
            }
            impl ::subxt::Event for Voted {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Voted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Approved {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Approved {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Approved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Disapproved {
                pub proposal_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Disapproved {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Disapproved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Executed {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Executed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MemberExecuted {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for MemberExecuted {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "MemberExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Closed {
                pub proposal_hash: ::subxt::sp_core::H256,
                pub yes: ::core::primitive::u32,
                pub no: ::core::primitive::u32,
            }
            impl ::subxt::Event for Closed {
                const PALLET: &'static str = "Collective";
                const EVENT: &'static str = "Closed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proposals;
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "Collective";
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
                const PALLET: &'static str = "Collective";
                const STORAGE: &'static str = "ProposalOf";
                type Value = runtime_types::rococo_runtime::Call;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct Voting(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Voting {
                const PALLET: &'static str = "Collective";
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
                const PALLET: &'static str = "Collective";
                const STORAGE: &'static str = "ProposalCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Collective";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Prime;
            impl ::subxt::StorageEntry for Prime {
                const PALLET: &'static str = "Collective";
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
                    ::subxt::BasicError,
                > {
                    let entry = Proposals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposal_of(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::rococo_runtime::Call>,
                    ::subxt::BasicError,
                > {
                    let entry = ProposalOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposal_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ProposalOf>, ::subxt::BasicError>
                {
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
                    ::subxt::BasicError,
                > {
                    let entry = Voting(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn voting_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn members(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn prime(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Prime;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod membership {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct add_member {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for add_member {
                const PALLET: &'static str = "Membership";
                const FUNCTION: &'static str = "add_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct remove_member {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for remove_member {
                const PALLET: &'static str = "Membership";
                const FUNCTION: &'static str = "remove_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct swap_member {
                pub remove: ::subxt::sp_core::crypto::AccountId32,
                pub add: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for swap_member {
                const PALLET: &'static str = "Membership";
                const FUNCTION: &'static str = "swap_member";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct reset_members {
                pub members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for reset_members {
                const PALLET: &'static str = "Membership";
                const FUNCTION: &'static str = "reset_members";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct change_key {
                pub new: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for change_key {
                const PALLET: &'static str = "Membership";
                const FUNCTION: &'static str = "change_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct set_prime {
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for set_prime {
                const PALLET: &'static str = "Membership";
                const FUNCTION: &'static str = "set_prime";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct clear_prime;
            impl ::subxt::Call for clear_prime {
                const PALLET: &'static str = "Membership";
                const FUNCTION: &'static str = "clear_prime";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn add_member(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, add_member, DispatchError>
                {
                    let call = add_member { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_member(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_member, DispatchError>
                {
                    let call = remove_member { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn swap_member(
                    &self,
                    remove: ::subxt::sp_core::crypto::AccountId32,
                    add: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, swap_member, DispatchError>
                {
                    let call = swap_member { remove, add };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reset_members(
                    &self,
                    members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, reset_members, DispatchError>
                {
                    let call = reset_members { members };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn change_key(
                    &self,
                    new: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, change_key, DispatchError>
                {
                    let call = change_key { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_prime(
                    &self,
                    who: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_prime, DispatchError>
                {
                    let call = set_prime { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_prime(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, clear_prime, DispatchError>
                {
                    let call = clear_prime {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_membership::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MemberAdded;
            impl ::subxt::Event for MemberAdded {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MemberAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MemberRemoved;
            impl ::subxt::Event for MemberRemoved {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MemberRemoved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MembersSwapped;
            impl ::subxt::Event for MembersSwapped {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MembersSwapped";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MembersReset;
            impl ::subxt::Event for MembersReset {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "MembersReset";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct KeyChanged;
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Dummy;
            impl ::subxt::Event for Dummy {
                const PALLET: &'static str = "Membership";
                const EVENT: &'static str = "Dummy";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Members;
            impl ::subxt::StorageEntry for Members {
                const PALLET: &'static str = "Membership";
                const STORAGE: &'static str = "Members";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Prime;
            impl ::subxt::StorageEntry for Prime {
                const PALLET: &'static str = "Membership";
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
                    ::subxt::BasicError,
                > {
                    let entry = Members;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn prime(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Prime;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct batch {
                pub calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct as_derivative {
                pub index: ::core::primitive::u16,
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for as_derivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct batch_all {
                pub calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for batch_all {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct dispatch_as {
                pub as_origin: ::std::boxed::Box<runtime_types::rococo_runtime::OriginCaller>,
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for dispatch_as {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "dispatch_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, batch, DispatchError>
                {
                    let call = batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::rococo_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_derivative, DispatchError>
                {
                    let call = as_derivative {
                        index,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, batch_all, DispatchError>
                {
                    let call = batch_all { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dispatch_as(
                    &self,
                    as_origin: runtime_types::rococo_runtime::OriginCaller,
                    call: runtime_types::rococo_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, dispatch_as, DispatchError>
                {
                    let call = dispatch_as {
                        as_origin: ::std::boxed::Box::new(as_origin),
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BatchInterrupted {
                pub index: ::core::primitive::u32,
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BatchCompleted;
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ItemCompleted;
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DispatchedAs {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn batched_calls_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[170u8, 42u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod proxy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct proxy {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct add_proxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::rococo_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for add_proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "add_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct remove_proxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::rococo_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for remove_proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct remove_proxies;
            impl ::subxt::Call for remove_proxies {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxies";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct anonymous {
                pub proxy_type: runtime_types::rococo_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
                pub index: ::core::primitive::u16,
            }
            impl ::subxt::Call for anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct kill_anonymous {
                pub spawner: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::rococo_runtime::ProxyType,
                pub index: ::core::primitive::u16,
                #[codec(compact)]
                pub height: ::core::primitive::u32,
                #[codec(compact)]
                pub ext_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for kill_anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "kill_anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct announce {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for announce {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "announce";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct remove_announcement {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for remove_announcement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct reject_announcement {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for reject_announcement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "reject_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct proxy_announced {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type:
                    ::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for proxy_announced {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy_announced";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn proxy(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::rococo_runtime::ProxyType,
                    >,
                    call: runtime_types::rococo_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, proxy, DispatchError>
                {
                    let call = proxy {
                        real,
                        force_proxy_type,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::rococo_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, add_proxy, DispatchError>
                {
                    let call = add_proxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::rococo_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_proxy, DispatchError>
                {
                    let call = remove_proxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxies(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_proxies, DispatchError>
                {
                    let call = remove_proxies {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn anonymous(
                    &self,
                    proxy_type: runtime_types::rococo_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, anonymous, DispatchError>
                {
                    let call = anonymous {
                        proxy_type,
                        delay,
                        index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_anonymous(
                    &self,
                    spawner: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type: runtime_types::rococo_runtime::ProxyType,
                    index: ::core::primitive::u16,
                    height: ::core::primitive::u32,
                    ext_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_anonymous, DispatchError>
                {
                    let call = kill_anonymous {
                        spawner,
                        proxy_type,
                        index,
                        height,
                        ext_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn announce(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, announce, DispatchError>
                {
                    let call = announce { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_announcement(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_announcement, DispatchError>
                {
                    let call = remove_announcement { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_announcement(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, reject_announcement, DispatchError>
                {
                    let call = reject_announcement {
                        delegate,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn proxy_announced(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::rococo_runtime::ProxyType,
                    >,
                    call: runtime_types::rococo_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, proxy_announced, DispatchError>
                {
                    let call = proxy_announced {
                        delegate,
                        real,
                        force_proxy_type,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_proxy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ProxyExecuted {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for ProxyExecuted {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AnonymousCreated {
                pub anonymous: ::subxt::sp_core::crypto::AccountId32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::rococo_runtime::ProxyType,
                pub disambiguation_index: ::core::primitive::u16,
            }
            impl ::subxt::Event for AnonymousCreated {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "AnonymousCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Announced {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub proxy: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Announced {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "Announced";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ProxyAdded {
                pub delegator: ::subxt::sp_core::crypto::AccountId32,
                pub delegatee: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type: runtime_types::rococo_runtime::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProxyAdded {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyAdded";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proxies(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proxies {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Proxies";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::ProxyDefinition<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::rococo_runtime::ProxyType,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Announcements(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Announcements {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Announcements";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::Announcement<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
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
                pub async fn proxies(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::ProxyDefinition<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::rococo_runtime::ProxyType,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    ),
                    ::subxt::BasicError,
                > {
                    let entry = Proxies(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proxies_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proxies>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn announcements(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::Announcement<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    ),
                    ::subxt::BasicError,
                > {
                    let entry = Announcements(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announcements_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Announcements>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn proxy_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn proxy_deposit_factor(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_proxies(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[32u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_pending(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[32u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn announcement_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn announcement_deposit_factor(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            10u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct as_multi_threshold_1 {
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
            }
            impl ::subxt::Call for as_multi_threshold_1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold_1";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call: ::subxt::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct approve_as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for approve_as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct cancel_as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for cancel_as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "cancel_as_multi";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn as_multi_threshold_1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    call: runtime_types::rococo_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_multi_threshold_1, DispatchError>
                {
                    let call = as_multi_threshold_1 {
                        other_signatories,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::subxt::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_multi, DispatchError>
                {
                    let call = as_multi {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call,
                        store_call,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, approve_as_multi, DispatchError>
                {
                    let call = approve_as_multi {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_as_multi, DispatchError>
                {
                    let call = cancel_as_multi {
                        threshold,
                        other_signatories,
                        timepoint,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NewMultisig {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MultisigApproval {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MultisigExecuted {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MultisigCancelled {
                pub cancelling: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::subxt::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
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
                pub async fn multisigs(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Multisigs>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 188u8, 231u8, 218u8, 233u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn deposit_factor(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 16u8, 94u8, 95u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_signatories(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[100u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod xcm_pallet {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct send {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
            }
            impl ::subxt::Call for send {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "send";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct teleport_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for teleport_assets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "teleport_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct reserve_transfer_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for reserve_transfer_assets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "reserve_transfer_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct execute {
                pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for execute {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_xcm_version {
                pub location:
                    ::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
                pub xcm_version: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_xcm_version {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_default_xcm_version {
                pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for force_default_xcm_version {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_default_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_subscribe_version_notify {
                pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
            }
            impl ::subxt::Call for force_subscribe_version_notify {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_subscribe_version_notify";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct force_unsubscribe_version_notify {
                pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
            }
            impl ::subxt::Call for force_unsubscribe_version_notify {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "force_unsubscribe_version_notify";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct limited_reserve_transfer_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            impl ::subxt::Call for limited_reserve_transfer_assets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "limited_reserve_transfer_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct limited_teleport_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            impl ::subxt::Call for limited_teleport_assets {
                const PALLET: &'static str = "XcmPallet";
                const FUNCTION: &'static str = "limited_teleport_assets";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn send(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    message: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, send, DispatchError>
                {
                    let call = send {
                        dest: ::std::boxed::Box::new(dest),
                        message: ::std::boxed::Box::new(message),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, teleport_assets, DispatchError>
                {
                    let call = teleport_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    reserve_transfer_assets,
                    DispatchError,
                > {
                    let call = reserve_transfer_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    message: runtime_types::xcm::VersionedXcm,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, execute, DispatchError>
                {
                    let call = execute {
                        message: ::std::boxed::Box::new(message),
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_xcm_version(
                    &self,
                    location: runtime_types::xcm::v1::multilocation::MultiLocation,
                    xcm_version: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_xcm_version, DispatchError>
                {
                    let call = force_xcm_version {
                        location: ::std::boxed::Box::new(location),
                        xcm_version,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_default_xcm_version(
                    &self,
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_default_xcm_version,
                    DispatchError,
                > {
                    let call = force_default_xcm_version { maybe_xcm_version };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_subscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_subscribe_version_notify,
                    DispatchError,
                > {
                    let call = force_subscribe_version_notify {
                        location: ::std::boxed::Box::new(location),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unsubscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_unsubscribe_version_notify,
                    DispatchError,
                > {
                    let call = force_unsubscribe_version_notify {
                        location: ::std::boxed::Box::new(location),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn limited_reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    limited_reserve_transfer_assets,
                    DispatchError,
                > {
                    let call = limited_reserve_transfer_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
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
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    limited_teleport_assets,
                    DispatchError,
                > {
                    let call = limited_teleport_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
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
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
            impl ::subxt::Event for Attempted {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "Attempted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Sent(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v2::Xcm,
            );
            impl ::subxt::Event for Sent {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "Sent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnexpectedResponse(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for UnexpectedResponse {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "UnexpectedResponse";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ResponseReady(
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::Response,
            );
            impl ::subxt::Event for ResponseReady {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "ResponseReady";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Notified(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for Notified {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "Notified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NotifyOverweight(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyOverweight {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyOverweight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NotifyDispatchError(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDispatchError {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyDispatchError";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NotifyDecodeFailed(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDecodeFailed {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyDecodeFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InvalidResponder(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
            );
            impl ::subxt::Event for InvalidResponder {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "InvalidResponder";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InvalidResponderVersion(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for InvalidResponderVersion {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "InvalidResponderVersion";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ResponseTaken(pub ::core::primitive::u64);
            impl ::subxt::Event for ResponseTaken {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "ResponseTaken";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AssetsTrapped(
                pub ::subxt::sp_core::H256,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::VersionedMultiAssets,
            );
            impl ::subxt::Event for AssetsTrapped {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "AssetsTrapped";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct VersionChangeNotified(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for VersionChangeNotified {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "VersionChangeNotified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SupportedVersionChanged(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for SupportedVersionChanged {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "SupportedVersionChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NotifyTargetSendFail(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::traits::Error,
            );
            impl ::subxt::Event for NotifyTargetSendFail {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyTargetSendFail";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct NotifyTargetMigrationFail(
                pub runtime_types::xcm::VersionedMultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyTargetMigrationFail {
                const PALLET: &'static str = "XcmPallet";
                const EVENT: &'static str = "NotifyTargetMigrationFail";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct QueryCounter;
            impl ::subxt::StorageEntry for QueryCounter {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "QueryCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Queries(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Queries {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "Queries";
                type Value = runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct AssetTraps(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for AssetTraps {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "AssetTraps";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct SafeXcmVersion;
            impl ::subxt::StorageEntry for SafeXcmVersion {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "SafeXcmVersion";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SupportedVersion(
                pub ::core::primitive::u32,
                pub runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for SupportedVersion {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "SupportedVersion";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionNotifiers(
                pub ::core::primitive::u32,
                pub runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for VersionNotifiers {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "VersionNotifiers";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionNotifyTargets(
                pub ::core::primitive::u32,
                pub runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for VersionNotifyTargets {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "VersionNotifyTargets";
                type Value = (
                    ::core::primitive::u64,
                    ::core::primitive::u64,
                    ::core::primitive::u32,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionDiscoveryQueue;
            impl ::subxt::StorageEntry for VersionDiscoveryQueue {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "VersionDiscoveryQueue";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                    runtime_types::xcm::VersionedMultiLocation,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentMigration;
            impl ::subxt::StorageEntry for CurrentMigration {
                const PALLET: &'static str = "XcmPallet";
                const STORAGE: &'static str = "CurrentMigration";
                type Value = runtime_types::pallet_xcm::pallet::VersionMigrationStage;
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
                pub async fn query_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = QueryCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queries(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Queries(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn queries_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Queries>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset_traps(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AssetTraps(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_traps_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AssetTraps>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn safe_xcm_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SafeXcmVersion;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn supported_version(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SupportedVersion(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn supported_version_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SupportedVersion>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_notifiers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = VersionNotifiers(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn version_notifiers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VersionNotifiers>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_notify_targets(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = VersionNotifyTargets(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn version_notify_targets_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VersionNotifyTargets>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_discovery_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = VersionDiscoveryQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_migration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_xcm::pallet::VersionMigrationStage,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentMigration;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod beefy_primitives {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
            pub mod mmr {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BeefyNextAuthoritySet<_0> {
                    pub id: ::core::primitive::u64,
                    pub len: ::core::primitive::u32,
                    pub root: _0,
                }
            }
        }
        pub mod bitvec {
            use super::runtime_types;
            pub mod order {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Lsb0;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Msb0;
            }
        }
        pub mod bp_header_chain {
            use super::runtime_types;
            pub mod justification {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct GrandpaJustification<_0> {
                    pub round: ::core::primitive::u64,
                    pub commit: runtime_types::finality_grandpa::Commit<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                        runtime_types::sp_finality_grandpa::app::Signature,
                        runtime_types::sp_finality_grandpa::app::Public,
                    >,
                    pub votes_ancestries: ::std::vec::Vec<_0>,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AuthoritySet {
                pub authorities: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
                pub set_id: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InitializationData<_0> {
                pub header: ::std::boxed::Box<_0>,
                pub authority_list: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
                pub set_id: ::core::primitive::u64,
                pub is_halted: ::core::primitive::bool,
            }
        }
        pub mod bp_message_dispatch {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum CallOrigin<_0, _1, _2> {
                #[codec(index = 0)]
                SourceRoot,
                #[codec(index = 1)]
                TargetAccount(_0, _1, _2),
                #[codec(index = 2)]
                SourceAccount(_0),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessagePayload<_0, _1, _2, _3> {
                pub spec_version: ::core::primitive::u32,
                pub weight: ::core::primitive::u64,
                pub origin: runtime_types::bp_message_dispatch::CallOrigin<_0, _1, _2>,
                pub dispatch_fee_payment: runtime_types::bp_runtime::messages::DispatchFeePayment,
                pub call: _3,
            }
        }
        pub mod bp_messages {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct DeliveredMessages {
                pub begin: ::core::primitive::u64,
                pub end: ::core::primitive::u64,
                pub dispatch_results: ::subxt::bitvec::vec::BitVec<
                    ::subxt::bitvec::order::Msb0,
                    ::core::primitive::u8,
                >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InboundLaneData<_0> {
                pub relayers: ::std::vec::Vec<runtime_types::bp_messages::UnrewardedRelayer<_0>>,
                pub last_confirmed_nonce: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageData<_0> {
                pub payload: ::std::vec::Vec<::core::primitive::u8>,
                pub fee: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MessageKey {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub nonce: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum OperatingMode {
                #[codec(index = 0)]
                Normal,
                #[codec(index = 1)]
                RejectingOutboundMessages,
                #[codec(index = 2)]
                Halted,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OutboundLaneData {
                pub oldest_unpruned_nonce: ::core::primitive::u64,
                pub latest_received_nonce: ::core::primitive::u64,
                pub latest_generated_nonce: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnrewardedRelayer<_0> {
                pub relayer: _0,
                pub messages: runtime_types::bp_messages::DeliveredMessages,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct UnrewardedRelayersState {
                pub unrewarded_relayer_entries: ::core::primitive::u64,
                pub messages_in_oldest_entry: ::core::primitive::u64,
                pub total_messages: ::core::primitive::u64,
            }
        }
        pub mod bp_runtime {
            use super::runtime_types;
            pub mod messages {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum DispatchFeePayment {
                    #[codec(index = 0)]
                    AtSourceChain,
                    #[codec(index = 1)]
                    AtTargetChain,
                }
            }
        }
        pub mod bridge_runtime_common {
            use super::runtime_types;
            pub mod messages {
                use super::runtime_types;
                pub mod source {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct FromBridgedChainMessagesDeliveryProof<_0> {
                        pub bridged_header_hash: _0,
                        pub storage_proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                        pub lane: [::core::primitive::u8; 4usize],
                    }
                }
                pub mod target {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct FromBridgedChainMessagesProof<_0> {
                        pub bridged_header_hash: _0,
                        pub storage_proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                        pub lane: [::core::primitive::u8; 4usize],
                        pub nonces_start: ::core::primitive::u64,
                        pub nonces_end: ::core::primitive::u64,
                    }
                }
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Commit<_0, _1, _2, _3> {
                pub target_hash: _0,
                pub target_number: _1,
                pub precommits: ::std::vec::Vec<
                    runtime_types::finality_grandpa::SignedPrecommit<_0, _1, _2, _3>,
                >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SignedPrecommit<_0, _1, _2, _3> {
                pub precommit: runtime_types::finality_grandpa::Precommit<_0, _1>,
                pub signature: _2,
                pub id: _3,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct WrapperKeepOpaque<_0>(
                        #[codec(compact)] pub ::core::primitive::u32,
                        pub _0,
                    );
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct WrapperOpaque<_0>(
                        #[codec(compact)] pub ::core::primitive::u32,
                        pub _0,
                    );
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug,
                        )]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    KilledAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    Remarked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        hash: ::subxt::sp_core::H256,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(_0),
                #[codec(index = 2)]
                None,
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
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
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_beefy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {}
            }
        }
        pub mod pallet_bridge_dispatch {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    MessageRejected(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    ),
                    #[codec(index = 1)]
                    MessageVersionSpecMismatch(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 2)]
                    MessageWeightMismatch(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    MessageSignatureMismatch(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    ),
                    #[codec(index = 4)]
                    MessageCallDecodeFailed(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    ),
                    #[codec(index = 5)]
                    MessageCallRejected(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    ),
                    #[codec(index = 6)]
                    MessageDispatchPaymentFailed(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 7)]
                    MessageDispatched(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                    #[codec(index = 8)]
                    _Dummy,
                }
            }
        }
        pub mod pallet_bridge_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    submit_finality_proof {
                        finality_target: ::std::boxed::Box<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                        justification:
                            runtime_types::bp_header_chain::justification::GrandpaJustification<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                            >,
                    },
                    #[codec(index = 1)]
                    initialize {
                        init_data: runtime_types::bp_header_chain::InitializationData<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                    #[codec(index = 2)]
                    set_owner {
                        new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 3)]
                    set_operational {
                        operational: ::core::primitive::bool,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidJustification,
                    #[codec(index = 1)]
                    InvalidAuthoritySet,
                    #[codec(index = 2)]
                    TooManyRequests,
                    #[codec(index = 3)]
                    OldHeader,
                    #[codec(index = 4)]
                    UnknownHeader,
                    #[codec(index = 5)]
                    UnsupportedScheduledChange,
                    #[codec(index = 6)]
                    NotInitialized,
                    #[codec(index = 7)]
                    AlreadyInitialized,
                    #[codec(index = 8)]
                    Halted,
                    #[codec(index = 9)]
                    StorageRootMismatch,
                }
            }
        }
        pub mod pallet_bridge_messages {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    # [codec (index = 0)] set_owner { new_owner : :: core :: option :: Option < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 1)] set_operating_mode { operating_mode : runtime_types :: bp_messages :: OperatingMode , } , # [codec (index = 2)] update_pallet_parameter { parameter : () , } , # [codec (index = 3)] send_message { lane_id : [:: core :: primitive :: u8 ; 4usize] , payload : runtime_types :: bp_message_dispatch :: MessagePayload < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_runtime :: MultiSigner , runtime_types :: sp_runtime :: MultiSignature , :: std :: vec :: Vec < :: core :: primitive :: u8 > > , delivery_and_dispatch_fee : :: core :: primitive :: u128 , } , # [codec (index = 4)] increase_message_fee { lane_id : [:: core :: primitive :: u8 ; 4usize] , nonce : :: core :: primitive :: u64 , additional_fee : :: core :: primitive :: u128 , } , # [codec (index = 5)] receive_messages_proof { relayer_id_at_bridged_chain : :: subxt :: sp_core :: crypto :: AccountId32 , proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 > , messages_count : :: core :: primitive :: u32 , dispatch_weight : :: core :: primitive :: u64 , } , # [codec (index = 6)] receive_messages_delivery_proof { proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 > , relayers_state : runtime_types :: bp_messages :: UnrewardedRelayersState , } , }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    Halted,
                    #[codec(index = 1)]
                    MessageRejectedByChainVerifier,
                    #[codec(index = 2)]
                    MessageRejectedByLaneVerifier,
                    #[codec(index = 3)]
                    FailedToWithdrawMessageFee,
                    #[codec(index = 4)]
                    TooManyMessagesInTheProof,
                    #[codec(index = 5)]
                    InvalidMessagesProof,
                    #[codec(index = 6)]
                    InvalidMessagesDeliveryProof,
                    #[codec(index = 7)]
                    InvalidUnrewardedRelayers,
                    #[codec(index = 8)]
                    InvalidUnrewardedRelayersState,
                    #[codec(index = 9)]
                    MessageIsAlreadyDelivered,
                    #[codec(index = 10)]
                    MessageIsNotYetSent,
                    #[codec(index = 11)]
                    TryingToConfirmMoreMessagesThanExpected,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    ParameterUpdated(()),
                    #[codec(index = 1)]
                    MessageAccepted([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    #[codec(index = 2)]
                    MessagesDelivered(
                        [::core::primitive::u8; 4usize],
                        runtime_types::bp_messages::DeliveredMessages,
                    ),
                }
            }
        }
        pub mod pallet_collective {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_members {
                        new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                        old_count: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    execute {
                        proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                        #[codec(compact)]
                        length_bound: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    propose {
                        #[codec(compact)]
                        threshold: ::core::primitive::u32,
                        proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
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
                    disapprove_proposal {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Proposed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        proposal_index: ::core::primitive::u32,
                        proposal_hash: ::subxt::sp_core::H256,
                        threshold: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    Voted {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        proposal_hash: ::subxt::sp_core::H256,
                        voted: ::core::primitive::bool,
                        yes: ::core::primitive::u32,
                        no: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    Approved {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 3)]
                    Disapproved {
                        proposal_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 4)]
                    Executed {
                        proposal_hash: ::subxt::sp_core::H256,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 5)]
                    MemberExecuted {
                        proposal_hash: ::subxt::sp_core::H256,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 6)]
                    Closed {
                        proposal_hash: ::subxt::sp_core::H256,
                        yes: ::core::primitive::u32,
                        no: ::core::primitive::u32,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Members(::core::primitive::u32, ::core::primitive::u32),
                #[codec(index = 1)]
                Member(_0),
                #[codec(index = 2)]
                _Phantom,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Votes<_0, _1> {
                pub index: _1,
                pub threshold: _1,
                pub ayes: ::std::vec::Vec<_0>,
                pub nays: ::std::vec::Vec<_0>,
                pub end: _1,
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    HeartbeatReceived {
                        authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    },
                    #[codec(index = 1)]
                    AllGood,
                    #[codec(index = 2)]
                    SomeOffline {
                        offline: ::std::vec::Vec<(::subxt::sp_core::crypto::AccountId32, ())>,
                    },
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BoundedOpaqueNetworkState {
                pub peer_id:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                pub external_addresses:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_indices {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    IndexAssigned {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    IndexFreed { index: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    IndexFrozen {
                        index: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_membership {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    add_member {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    remove_member {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
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
                    change_key {
                        new: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    set_prime {
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    clear_prime,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    AlreadyMember,
                    #[codec(index = 1)]
                    NotMember,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::subxt::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
                        store_call: ::core::primitive::bool,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    TooManySignatories,
                    #[codec(index = 5)]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    NotFound,
                    #[codec(index = 8)]
                    NotOwner,
                    #[codec(index = 9)]
                    NoTimepoint,
                    #[codec(index = 10)]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    AlreadyStored,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewMultisig {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    MultisigApproval {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    MultisigExecuted {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    MultisigCancelled {
                        cancelling: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_offences {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Offence {
                        kind: [::core::primitive::u8; 16usize],
                        timeslot: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    proxy {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type:
                            ::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    add_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::rococo_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    remove_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::rococo_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    remove_proxies,
                    #[codec(index = 4)]
                    anonymous {
                        proxy_type: runtime_types::rococo_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                        index: ::core::primitive::u16,
                    },
                    #[codec(index = 5)]
                    kill_anonymous {
                        spawner: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::rococo_runtime::ProxyType,
                        index: ::core::primitive::u16,
                        #[codec(compact)]
                        height: ::core::primitive::u32,
                        #[codec(compact)]
                        ext_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    announce {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 7)]
                    remove_announcement {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 8)]
                    reject_announcement {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 9)]
                    proxy_announced {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type:
                            ::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooMany,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    NotProxy,
                    #[codec(index = 3)]
                    Unproxyable,
                    #[codec(index = 4)]
                    Duplicate,
                    #[codec(index = 5)]
                    NoPermission,
                    #[codec(index = 6)]
                    Unannounced,
                    #[codec(index = 7)]
                    NoSelfProxy,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    ProxyExecuted {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    AnonymousCreated {
                        anonymous: ::subxt::sp_core::crypto::AccountId32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::rococo_runtime::ProxyType,
                        disambiguation_index: ::core::primitive::u16,
                    },
                    #[codec(index = 2)]
                    Announced {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        proxy: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 3)]
                    ProxyAdded {
                        delegator: ::subxt::sp_core::crypto::AccountId32,
                        delegatee: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type: runtime_types::rococo_runtime::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    set_keys {
                        keys: runtime_types::rococo_runtime::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    purge_keys,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 2)]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    batch {
                        calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                    },
                    #[codec(index = 2)]
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
                    },
                    #[codec(index = 3)]
                    dispatch_as {
                        as_origin: ::std::boxed::Box<runtime_types::rococo_runtime::OriginCaller>,
                        call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCalls,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    BatchCompleted,
                    #[codec(index = 2)]
                    ItemCompleted,
                    #[codec(index = 3)]
                    DispatchedAs {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    Notified(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Origin {
                    #[codec(index = 0)]
                    Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
                    #[codec(index = 1)]
                    Response(runtime_types::xcm::v1::multilocation::MultiLocation),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum QueryStatus<_0> {
                    #[codec(index = 0)]
                    Pending {
                        responder: runtime_types::xcm::VersionedMultiLocation,
                        maybe_notify:
                            ::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
                        timeout: _0,
                    },
                    #[codec(index = 1)]
                    VersionNotifier {
                        origin: runtime_types::xcm::VersionedMultiLocation,
                        is_active: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    Ready {
                        response: runtime_types::xcm::VersionedResponse,
                        at: _0,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum VersionMigrationStage {
                    #[codec(index = 0)]
                    MigrateSupportedVersion,
                    #[codec(index = 1)]
                    MigrateVersionNotifiers,
                    #[codec(index = 2)]
                    NotifyCurrentTargets(
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ),
                    #[codec(index = 3)]
                    MigrateAndNotifyOldTargets,
                }
            }
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct CandidateHash(pub ::subxt::sp_core::H256);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct HrmpChannelId {
                    pub sender: runtime_types::polkadot_parachain::primitives::Id,
                    pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Id(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ValidationCode(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ValidationCodeHash(pub ::subxt::sp_core::H256);
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v0 {
                use super::runtime_types;
                pub mod collator_app {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                pub mod validator_app {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct ValidatorIndex(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum ValidityAttestation {
                    #[codec(index = 1)]
                    Implicit(runtime_types::polkadot_primitives::v0::validator_app::Signature),
                    #[codec(index = 2)]
                    Explicit(runtime_types::polkadot_primitives::v0::validator_app::Signature),
                }
            }
            pub mod v1 {
                use super::runtime_types;
                pub mod assignment_app {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
                pub mod signed {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct UncheckedSigned<_0, _1> {
                        pub payload: _0,
                        pub validator_index: runtime_types::polkadot_primitives::v0::ValidatorIndex,
                        pub signature:
                            runtime_types::polkadot_primitives::v0::validator_app::Signature,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AvailabilityBitfield(
                    pub  ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                );
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BackedCandidate<_0> {
                    pub candidate:
                        runtime_types::polkadot_primitives::v1::CommittedCandidateReceipt<_0>,
                    pub validity_votes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::ValidityAttestation,
                    >,
                    pub validator_indices: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct CandidateCommitments<_0> {
                    pub upward_messages: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub horizontal_messages: ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                    pub new_validation_code: ::core::option::Option<
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    >,
                    pub head_data: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub processed_downward_messages: _0,
                    pub hrmp_watermark: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct CandidateDescriptor<_0> {
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub relay_parent: _0,
                    pub collator: runtime_types::polkadot_primitives::v0::collator_app::Public,
                    pub persisted_validation_data_hash: _0,
                    pub pov_hash: _0,
                    pub erasure_root: _0,
                    pub signature: runtime_types::polkadot_primitives::v0::collator_app::Signature,
                    pub para_head: _0,
                    pub validation_code_hash:
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct CandidateReceipt<_0> {
                    pub descriptor: runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub commitments_hash: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct CommittedCandidateReceipt<_0> {
                    pub descriptor: runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub commitments: runtime_types::polkadot_primitives::v1::CandidateCommitments<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct CoreIndex(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum CoreOccupied {
                    #[codec(index = 0)]
                    Parathread(runtime_types::polkadot_primitives::v1::ParathreadEntry),
                    #[codec(index = 1)]
                    Parachain,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DisputeState<_0> {
                    pub validators_for: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub validators_against: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub start: _0,
                    pub concluded_at: ::core::option::Option<_0>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum DisputeStatement {
                    #[codec(index = 0)]
                    Valid(runtime_types::polkadot_primitives::v1::ValidDisputeStatementKind),
                    #[codec(index = 1)]
                    Invalid(runtime_types::polkadot_primitives::v1::InvalidDisputeStatementKind),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DisputeStatementSet {
                    pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    pub session: ::core::primitive::u32,
                    pub statements: ::std::vec::Vec<(
                        runtime_types::polkadot_primitives::v1::DisputeStatement,
                        runtime_types::polkadot_primitives::v0::ValidatorIndex,
                        runtime_types::polkadot_primitives::v0::validator_app::Signature,
                    )>,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct GroupIndex(pub ::core::primitive::u32);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct InherentData<_0> {
                    pub bitfields: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::signed::UncheckedSigned<
                            runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                            runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                        >,
                    >,
                    pub backed_candidates: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::BackedCandidate<
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    pub disputes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::DisputeStatementSet,
                    >,
                    pub parent_header: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum InvalidDisputeStatementKind {
                    #[codec(index = 0)]
                    Explicit,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ParathreadClaim(
                    pub runtime_types::polkadot_parachain::primitives::Id,
                    pub runtime_types::polkadot_primitives::v0::collator_app::Public,
                );
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ParathreadEntry {
                    pub claim: runtime_types::polkadot_primitives::v1::ParathreadClaim,
                    pub retries: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ScrapedOnChainVotes<_0> {
                    pub session: ::core::primitive::u32,
                    pub backing_validators_per_candidate: ::std::vec::Vec<(
                        runtime_types::polkadot_primitives::v1::CandidateReceipt<_0>,
                        ::std::vec::Vec<(
                            runtime_types::polkadot_primitives::v0::ValidatorIndex,
                            runtime_types::polkadot_primitives::v0::ValidityAttestation,
                        )>,
                    )>,
                    pub disputes: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::DisputeStatementSet,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum UpgradeGoAhead {
                    #[codec(index = 0)]
                    Abort,
                    #[codec(index = 1)]
                    GoAhead,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum UpgradeRestriction {
                    #[codec(index = 0)]
                    Present,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum ValidDisputeStatementKind {
                    #[codec(index = 0)]
                    Explicit,
                    #[codec(index = 1)]
                    BackingSeconded(::subxt::sp_core::H256),
                    #[codec(index = 2)]
                    BackingValid(::subxt::sp_core::H256),
                    #[codec(index = 3)]
                    ApprovalChecking,
                }
            }
            pub mod v2 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct PvfCheckStatement {
                    pub accept: ::core::primitive::bool,
                    pub subject: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    pub session_index: ::core::primitive::u32,
                    pub validator_index: runtime_types::polkadot_primitives::v0::ValidatorIndex,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct SessionInfo {
                    pub active_validator_indices:
                        ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    pub random_seed: [::core::primitive::u8; 32usize],
                    pub dispute_period: ::core::primitive::u32,
                    pub validators: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub discovery_keys:
                        ::std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
                    pub assignment_keys: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v1::assignment_app::Public,
                    >,
                    pub validator_groups: ::std::vec::Vec<
                        ::std::vec::Vec<runtime_types::polkadot_primitives::v0::ValidatorIndex>,
                    >,
                    pub n_cores: ::core::primitive::u32,
                    pub zeroth_delay_tranche_width: ::core::primitive::u32,
                    pub relay_vrf_modulo_samples: ::core::primitive::u32,
                    pub n_delay_tranches: ::core::primitive::u32,
                    pub no_show_slots: ::core::primitive::u32,
                    pub needed_approvals: ::core::primitive::u32,
                }
            }
        }
        pub mod polkadot_runtime_common {
            use super::runtime_types;
            pub mod assigned_slots {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        # [codec (index = 0)] assign_perm_parachain_slot { id : runtime_types :: polkadot_parachain :: primitives :: Id , } , # [codec (index = 1)] assign_temp_parachain_slot { id : runtime_types :: polkadot_parachain :: primitives :: Id , lease_period_start : runtime_types :: polkadot_runtime_common :: assigned_slots :: SlotLeasePeriodStart , } , # [codec (index = 2)] unassign_parachain_slot { id : runtime_types :: polkadot_parachain :: primitives :: Id , } , }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        ParaDoesntExist,
                        #[codec(index = 1)]
                        NotParathread,
                        #[codec(index = 2)]
                        CannotUpgrade,
                        #[codec(index = 3)]
                        CannotDowngrade,
                        #[codec(index = 4)]
                        SlotAlreadyAssigned,
                        #[codec(index = 5)]
                        SlotNotAssigned,
                        #[codec(index = 6)]
                        OngoingLeaseExists,
                        #[codec(index = 7)]
                        MaxPermanentSlotsExceeded,
                        #[codec(index = 8)]
                        MaxTemporarySlotsExceeded,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        PermanentSlotAssigned(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 1)]
                        TemporarySlotAssigned(runtime_types::polkadot_parachain::primitives::Id),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ParachainTemporarySlot<_0, _1> {
                    pub manager: _0,
                    pub period_begin: _1,
                    pub period_count: _1,
                    pub last_lease: ::core::option::Option<_1>,
                    pub lease_count: _1,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum SlotLeasePeriodStart {
                    #[codec(index = 0)]
                    Current,
                    #[codec(index = 1)]
                    Next,
                }
            }
            pub mod auctions {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        new_auction {
                            #[codec(compact)]
                            duration: ::core::primitive::u32,
                            #[codec(compact)]
                            lease_period_index: ::core::primitive::u32,
                        },
                        #[codec(index = 1)]
                        bid {
                            #[codec(compact)]
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            auction_index: ::core::primitive::u32,
                            #[codec(compact)]
                            first_slot: ::core::primitive::u32,
                            #[codec(compact)]
                            last_slot: ::core::primitive::u32,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 2)]
                        cancel_auction,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        AuctionInProgress,
                        #[codec(index = 1)]
                        LeasePeriodInPast,
                        #[codec(index = 2)]
                        ParaNotRegistered,
                        #[codec(index = 3)]
                        NotCurrentAuction,
                        #[codec(index = 4)]
                        NotAuction,
                        #[codec(index = 5)]
                        AuctionEnded,
                        #[codec(index = 6)]
                        AlreadyLeasedOut,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        AuctionStarted(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 1)]
                        AuctionClosed(::core::primitive::u32),
                        #[codec(index = 2)]
                        Reserved(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 3)]
                        Unreserved(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 4)]
                        ReserveConfiscated(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 5)]
                        BidAccepted(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 6)]
                        WinningOffset(::core::primitive::u32, ::core::primitive::u32),
                    }
                }
            }
            pub mod crowdloan {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        create {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            cap: ::core::primitive::u128,
                            #[codec(compact)]
                            first_period: ::core::primitive::u32,
                            #[codec(compact)]
                            last_period: ::core::primitive::u32,
                            #[codec(compact)]
                            end: ::core::primitive::u32,
                            verifier:
                                ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                        },
                        #[codec(index = 1)]
                        contribute {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            value: ::core::primitive::u128,
                            signature:
                                ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                        },
                        #[codec(index = 2)]
                        withdraw {
                            who: ::subxt::sp_core::crypto::AccountId32,
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 3)]
                        refund {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        dissolve {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 5)]
                        edit {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            #[codec(compact)]
                            cap: ::core::primitive::u128,
                            #[codec(compact)]
                            first_period: ::core::primitive::u32,
                            #[codec(compact)]
                            last_period: ::core::primitive::u32,
                            #[codec(compact)]
                            end: ::core::primitive::u32,
                            verifier:
                                ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                        },
                        #[codec(index = 6)]
                        add_memo {
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            memo: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 7)]
                        poke {
                            index: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 8)]
                        contribute_all {
                            #[codec(compact)]
                            index: runtime_types::polkadot_parachain::primitives::Id,
                            signature:
                                ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        FirstPeriodInPast,
                        #[codec(index = 1)]
                        FirstPeriodTooFarInFuture,
                        #[codec(index = 2)]
                        LastPeriodBeforeFirstPeriod,
                        #[codec(index = 3)]
                        LastPeriodTooFarInFuture,
                        #[codec(index = 4)]
                        CannotEndInPast,
                        #[codec(index = 5)]
                        EndTooFarInFuture,
                        #[codec(index = 6)]
                        Overflow,
                        #[codec(index = 7)]
                        ContributionTooSmall,
                        #[codec(index = 8)]
                        InvalidParaId,
                        #[codec(index = 9)]
                        CapExceeded,
                        #[codec(index = 10)]
                        ContributionPeriodOver,
                        #[codec(index = 11)]
                        InvalidOrigin,
                        #[codec(index = 12)]
                        NotParachain,
                        #[codec(index = 13)]
                        LeaseActive,
                        #[codec(index = 14)]
                        BidOrLeaseActive,
                        #[codec(index = 15)]
                        FundNotEnded,
                        #[codec(index = 16)]
                        NoContributions,
                        #[codec(index = 17)]
                        NotReadyToDissolve,
                        #[codec(index = 18)]
                        InvalidSignature,
                        #[codec(index = 19)]
                        MemoTooLarge,
                        #[codec(index = 20)]
                        AlreadyInNewRaise,
                        #[codec(index = 21)]
                        VrfDelayInProgress,
                        #[codec(index = 22)]
                        NoLeasePeriod,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        Created(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 1)]
                        Contributed(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 2)]
                        Withdrew(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u128,
                        ),
                        #[codec(index = 3)]
                        PartiallyRefunded(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 4)]
                        AllRefunded(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 5)]
                        Dissolved(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 6)]
                        HandleBidResult(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                        ),
                        #[codec(index = 7)]
                        Edited(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 8)]
                        MemoUpdated(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 9)]
                        AddedToNewRaise(runtime_types::polkadot_parachain::primitives::Id),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct FundInfo<_0, _1, _2, _3> {
                    pub depositor: _0,
                    pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                    pub deposit: _1,
                    pub raised: _1,
                    pub end: _2,
                    pub cap: _1,
                    pub last_contribution:
                        runtime_types::polkadot_runtime_common::crowdloan::LastContribution<_2>,
                    pub first_period: _2,
                    pub last_period: _2,
                    pub trie_index: _2,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum LastContribution<_0> {
                    #[codec(index = 0)]
                    Never,
                    #[codec(index = 1)]
                    PreEnding(_0),
                    #[codec(index = 2)]
                    Ending(_0),
                }
            }
            pub mod paras_registrar {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        register {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                            validation_code:
                                runtime_types::polkadot_parachain::primitives::ValidationCode,
                        },
                        #[codec(index = 1)]
                        force_register {
                            who: ::subxt::sp_core::crypto::AccountId32,
                            deposit: ::core::primitive::u128,
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                            validation_code:
                                runtime_types::polkadot_parachain::primitives::ValidationCode,
                        },
                        #[codec(index = 2)]
                        deregister {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 3)]
                        swap {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            other: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        force_remove_lock {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 5)]
                        reserve,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        NotRegistered,
                        #[codec(index = 1)]
                        AlreadyRegistered,
                        #[codec(index = 2)]
                        NotOwner,
                        #[codec(index = 3)]
                        CodeTooLarge,
                        #[codec(index = 4)]
                        HeadDataTooLarge,
                        #[codec(index = 5)]
                        NotParachain,
                        #[codec(index = 6)]
                        NotParathread,
                        #[codec(index = 7)]
                        CannotDeregister,
                        #[codec(index = 8)]
                        CannotDowngrade,
                        #[codec(index = 9)]
                        CannotUpgrade,
                        #[codec(index = 10)]
                        ParaLocked,
                        #[codec(index = 11)]
                        NotReserved,
                        #[codec(index = 12)]
                        EmptyCode,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        Registered(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                        #[codec(index = 1)]
                        Deregistered(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 2)]
                        Reserved(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ParaInfo<_0, _1> {
                    pub manager: _0,
                    pub deposit: _1,
                    pub locked: ::core::primitive::bool,
                }
            }
            pub mod paras_sudo_wrapper {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        sudo_schedule_para_initialize {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            genesis:
                                runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
                        },
                        #[codec(index = 1)]
                        sudo_schedule_para_cleanup {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 2)]
                        sudo_schedule_parathread_upgrade {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 3)]
                        sudo_schedule_parachain_downgrade {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        sudo_queue_downward_xcm {
                            id: runtime_types::polkadot_parachain::primitives::Id,
                            xcm: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                        },
                        #[codec(index = 5)]
                        sudo_establish_hrmp_channel {
                            sender: runtime_types::polkadot_parachain::primitives::Id,
                            recipient: runtime_types::polkadot_parachain::primitives::Id,
                            max_capacity: ::core::primitive::u32,
                            max_message_size: ::core::primitive::u32,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        ParaDoesntExist,
                        #[codec(index = 1)]
                        ParaAlreadyExists,
                        #[codec(index = 2)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 3)]
                        CouldntCleanup,
                        #[codec(index = 4)]
                        NotParathread,
                        #[codec(index = 5)]
                        NotParachain,
                        #[codec(index = 6)]
                        CannotUpgrade,
                        #[codec(index = 7)]
                        CannotDowngrade,
                    }
                }
            }
            pub mod slots {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        force_lease {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            leaser: ::subxt::sp_core::crypto::AccountId32,
                            amount: ::core::primitive::u128,
                            period_begin: ::core::primitive::u32,
                            period_count: ::core::primitive::u32,
                        },
                        #[codec(index = 1)]
                        clear_all_leases {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 2)]
                        trigger_onboard {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        ParaNotOnboarding,
                        #[codec(index = 1)]
                        LeaseError,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        NewLeasePeriod(::core::primitive::u32),
                        #[codec(index = 1)]
                        Leased(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        ),
                    }
                }
            }
        }
        pub mod polkadot_runtime_parachains {
            use super::runtime_types;
            pub mod configuration {
                use super::runtime_types;
                pub mod migration {
                    use super::runtime_types;
                    pub mod v1 {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug,
                        )]
                        pub struct HostConfiguration<_0> {
                            pub max_code_size: _0,
                            pub max_head_data_size: _0,
                            pub max_upward_queue_count: _0,
                            pub max_upward_queue_size: _0,
                            pub max_upward_message_size: _0,
                            pub max_upward_message_num_per_candidate: _0,
                            pub hrmp_max_message_num_per_candidate: _0,
                            pub validation_upgrade_frequency: _0,
                            pub validation_upgrade_delay: _0,
                            pub max_pov_size: _0,
                            pub max_downward_message_size: _0,
                            pub ump_service_total_weight: ::core::primitive::u64,
                            pub hrmp_max_parachain_outbound_channels: _0,
                            pub hrmp_max_parathread_outbound_channels: _0,
                            pub hrmp_sender_deposit: ::core::primitive::u128,
                            pub hrmp_recipient_deposit: ::core::primitive::u128,
                            pub hrmp_channel_max_capacity: _0,
                            pub hrmp_channel_max_total_size: _0,
                            pub hrmp_max_parachain_inbound_channels: _0,
                            pub hrmp_max_parathread_inbound_channels: _0,
                            pub hrmp_channel_max_message_size: _0,
                            pub code_retention_period: _0,
                            pub parathread_cores: _0,
                            pub parathread_retries: _0,
                            pub group_rotation_frequency: _0,
                            pub chain_availability_period: _0,
                            pub thread_availability_period: _0,
                            pub scheduling_lookahead: _0,
                            pub max_validators_per_core: ::core::option::Option<_0>,
                            pub max_validators: ::core::option::Option<_0>,
                            pub dispute_period: _0,
                            pub dispute_post_conclusion_acceptance_period: _0,
                            pub dispute_max_spam_slots: _0,
                            pub dispute_conclusion_by_time_out_period: _0,
                            pub no_show_slots: _0,
                            pub n_delay_tranches: _0,
                            pub zeroth_delay_tranche_width: _0,
                            pub needed_approvals: _0,
                            pub relay_vrf_modulo_samples: _0,
                            pub ump_max_individual_weight: ::core::primitive::u64,
                        }
                    }
                }
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        set_validation_upgrade_cooldown { new: ::core::primitive::u32 },
                        #[codec(index = 1)]
                        set_validation_upgrade_delay { new: ::core::primitive::u32 },
                        #[codec(index = 2)]
                        set_code_retention_period { new: ::core::primitive::u32 },
                        #[codec(index = 3)]
                        set_max_code_size { new: ::core::primitive::u32 },
                        #[codec(index = 4)]
                        set_max_pov_size { new: ::core::primitive::u32 },
                        #[codec(index = 5)]
                        set_max_head_data_size { new: ::core::primitive::u32 },
                        #[codec(index = 6)]
                        set_parathread_cores { new: ::core::primitive::u32 },
                        #[codec(index = 7)]
                        set_parathread_retries { new: ::core::primitive::u32 },
                        #[codec(index = 8)]
                        set_group_rotation_frequency { new: ::core::primitive::u32 },
                        #[codec(index = 9)]
                        set_chain_availability_period { new: ::core::primitive::u32 },
                        #[codec(index = 10)]
                        set_thread_availability_period { new: ::core::primitive::u32 },
                        #[codec(index = 11)]
                        set_scheduling_lookahead { new: ::core::primitive::u32 },
                        #[codec(index = 12)]
                        set_max_validators_per_core {
                            new: ::core::option::Option<::core::primitive::u32>,
                        },
                        #[codec(index = 13)]
                        set_max_validators {
                            new: ::core::option::Option<::core::primitive::u32>,
                        },
                        #[codec(index = 14)]
                        set_dispute_period { new: ::core::primitive::u32 },
                        #[codec(index = 15)]
                        set_dispute_post_conclusion_acceptance_period {
                            new: ::core::primitive::u32,
                        },
                        #[codec(index = 16)]
                        set_dispute_max_spam_slots { new: ::core::primitive::u32 },
                        #[codec(index = 17)]
                        set_dispute_conclusion_by_time_out_period { new: ::core::primitive::u32 },
                        #[codec(index = 18)]
                        set_no_show_slots { new: ::core::primitive::u32 },
                        #[codec(index = 19)]
                        set_n_delay_tranches { new: ::core::primitive::u32 },
                        #[codec(index = 20)]
                        set_zeroth_delay_tranche_width { new: ::core::primitive::u32 },
                        #[codec(index = 21)]
                        set_needed_approvals { new: ::core::primitive::u32 },
                        #[codec(index = 22)]
                        set_relay_vrf_modulo_samples { new: ::core::primitive::u32 },
                        #[codec(index = 23)]
                        set_max_upward_queue_count { new: ::core::primitive::u32 },
                        #[codec(index = 24)]
                        set_max_upward_queue_size { new: ::core::primitive::u32 },
                        #[codec(index = 25)]
                        set_max_downward_message_size { new: ::core::primitive::u32 },
                        #[codec(index = 26)]
                        set_ump_service_total_weight { new: ::core::primitive::u64 },
                        #[codec(index = 27)]
                        set_max_upward_message_size { new: ::core::primitive::u32 },
                        #[codec(index = 28)]
                        set_max_upward_message_num_per_candidate { new: ::core::primitive::u32 },
                        #[codec(index = 29)]
                        set_hrmp_open_request_ttl { new: ::core::primitive::u32 },
                        #[codec(index = 30)]
                        set_hrmp_sender_deposit { new: ::core::primitive::u128 },
                        #[codec(index = 31)]
                        set_hrmp_recipient_deposit { new: ::core::primitive::u128 },
                        #[codec(index = 32)]
                        set_hrmp_channel_max_capacity { new: ::core::primitive::u32 },
                        #[codec(index = 33)]
                        set_hrmp_channel_max_total_size { new: ::core::primitive::u32 },
                        #[codec(index = 34)]
                        set_hrmp_max_parachain_inbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 35)]
                        set_hrmp_max_parathread_inbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 36)]
                        set_hrmp_channel_max_message_size { new: ::core::primitive::u32 },
                        #[codec(index = 37)]
                        set_hrmp_max_parachain_outbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 38)]
                        set_hrmp_max_parathread_outbound_channels { new: ::core::primitive::u32 },
                        #[codec(index = 39)]
                        set_hrmp_max_message_num_per_candidate { new: ::core::primitive::u32 },
                        #[codec(index = 40)]
                        set_ump_max_individual_weight { new: ::core::primitive::u64 },
                        #[codec(index = 41)]
                        set_pvf_checking_enabled { new: ::core::primitive::bool },
                        #[codec(index = 42)]
                        set_pvf_voting_ttl { new: ::core::primitive::u32 },
                        #[codec(index = 43)]
                        set_minimum_validation_upgrade_delay { new: ::core::primitive::u32 },
                        #[codec(index = 44)]
                        set_bypass_consistency_check { new: ::core::primitive::bool },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        InvalidNewValue,
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct HostConfiguration<_0> {
                    pub max_code_size: _0,
                    pub max_head_data_size: _0,
                    pub max_upward_queue_count: _0,
                    pub max_upward_queue_size: _0,
                    pub max_upward_message_size: _0,
                    pub max_upward_message_num_per_candidate: _0,
                    pub hrmp_max_message_num_per_candidate: _0,
                    pub validation_upgrade_cooldown: _0,
                    pub validation_upgrade_delay: _0,
                    pub max_pov_size: _0,
                    pub max_downward_message_size: _0,
                    pub ump_service_total_weight: ::core::primitive::u64,
                    pub hrmp_max_parachain_outbound_channels: _0,
                    pub hrmp_max_parathread_outbound_channels: _0,
                    pub hrmp_sender_deposit: ::core::primitive::u128,
                    pub hrmp_recipient_deposit: ::core::primitive::u128,
                    pub hrmp_channel_max_capacity: _0,
                    pub hrmp_channel_max_total_size: _0,
                    pub hrmp_max_parachain_inbound_channels: _0,
                    pub hrmp_max_parathread_inbound_channels: _0,
                    pub hrmp_channel_max_message_size: _0,
                    pub code_retention_period: _0,
                    pub parathread_cores: _0,
                    pub parathread_retries: _0,
                    pub group_rotation_frequency: _0,
                    pub chain_availability_period: _0,
                    pub thread_availability_period: _0,
                    pub scheduling_lookahead: _0,
                    pub max_validators_per_core: ::core::option::Option<_0>,
                    pub max_validators: ::core::option::Option<_0>,
                    pub dispute_period: _0,
                    pub dispute_post_conclusion_acceptance_period: _0,
                    pub dispute_max_spam_slots: _0,
                    pub dispute_conclusion_by_time_out_period: _0,
                    pub no_show_slots: _0,
                    pub n_delay_tranches: _0,
                    pub zeroth_delay_tranche_width: _0,
                    pub needed_approvals: _0,
                    pub relay_vrf_modulo_samples: _0,
                    pub ump_max_individual_weight: ::core::primitive::u64,
                    pub pvf_checking_enabled: ::core::primitive::bool,
                    pub pvf_voting_ttl: _0,
                    pub minimum_validation_upgrade_delay: _0,
                }
            }
            pub mod disputes {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        force_unfreeze,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        DuplicateDisputeStatementSets,
                        #[codec(index = 1)]
                        AncientDisputeStatement,
                        #[codec(index = 2)]
                        ValidatorIndexOutOfBounds,
                        #[codec(index = 3)]
                        InvalidSignature,
                        #[codec(index = 4)]
                        DuplicateStatement,
                        #[codec(index = 5)]
                        PotentialSpam,
                        #[codec(index = 6)]
                        SingleSidedDispute,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        DisputeInitiated(
                            runtime_types::polkadot_core_primitives::CandidateHash,
                            runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation,
                        ),
                        #[codec(index = 1)]
                        DisputeConcluded(
                            runtime_types::polkadot_core_primitives::CandidateHash,
                            runtime_types::polkadot_runtime_parachains::disputes::DisputeResult,
                        ),
                        #[codec(index = 2)]
                        DisputeTimedOut(runtime_types::polkadot_core_primitives::CandidateHash),
                        #[codec(index = 3)]
                        Revert(::core::primitive::u32),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum DisputeLocation {
                    #[codec(index = 0)]
                    Local,
                    #[codec(index = 1)]
                    Remote,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum DisputeResult {
                    #[codec(index = 0)]
                    Valid,
                    #[codec(index = 1)]
                    Invalid,
                }
            }
            pub mod dmp {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {}
                }
            }
            pub mod hrmp {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        hrmp_init_open_channel {
                            recipient: runtime_types::polkadot_parachain::primitives::Id,
                            proposed_max_capacity: ::core::primitive::u32,
                            proposed_max_message_size: ::core::primitive::u32,
                        },
                        #[codec(index = 1)]
                        hrmp_accept_open_channel {
                            sender: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 2)]
                        hrmp_close_channel {
                            channel_id:
                                runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        },
                        #[codec(index = 3)]
                        force_clean_hrmp {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 4)]
                        force_process_hrmp_open,
                        #[codec(index = 5)]
                        force_process_hrmp_close,
                        #[codec(index = 6)]
                        hrmp_cancel_open_request {
                            channel_id:
                                runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        OpenHrmpChannelToSelf,
                        #[codec(index = 1)]
                        OpenHrmpChannelInvalidRecipient,
                        #[codec(index = 2)]
                        OpenHrmpChannelZeroCapacity,
                        #[codec(index = 3)]
                        OpenHrmpChannelCapacityExceedsLimit,
                        #[codec(index = 4)]
                        OpenHrmpChannelZeroMessageSize,
                        #[codec(index = 5)]
                        OpenHrmpChannelMessageSizeExceedsLimit,
                        #[codec(index = 6)]
                        OpenHrmpChannelAlreadyExists,
                        #[codec(index = 7)]
                        OpenHrmpChannelAlreadyRequested,
                        #[codec(index = 8)]
                        OpenHrmpChannelLimitExceeded,
                        #[codec(index = 9)]
                        AcceptHrmpChannelDoesntExist,
                        #[codec(index = 10)]
                        AcceptHrmpChannelAlreadyConfirmed,
                        #[codec(index = 11)]
                        AcceptHrmpChannelLimitExceeded,
                        #[codec(index = 12)]
                        CloseHrmpChannelUnauthorized,
                        #[codec(index = 13)]
                        CloseHrmpChannelDoesntExist,
                        #[codec(index = 14)]
                        CloseHrmpChannelAlreadyUnderway,
                        #[codec(index = 15)]
                        CancelHrmpOpenChannelUnauthorized,
                        #[codec(index = 16)]
                        OpenHrmpChannelDoesntExist,
                        #[codec(index = 17)]
                        OpenHrmpChannelAlreadyConfirmed,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        OpenChannelRequested(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 1)]
                        OpenChannelCanceled(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        ),
                        #[codec(index = 2)]
                        OpenChannelAccepted(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        #[codec(index = 3)]
                        ChannelClosed(
                            runtime_types::polkadot_parachain::primitives::Id,
                            runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct HrmpChannel {
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                    pub max_message_size: ::core::primitive::u32,
                    pub msg_count: ::core::primitive::u32,
                    pub total_size: ::core::primitive::u32,
                    pub mqc_head: ::core::option::Option<::subxt::sp_core::H256>,
                    pub sender_deposit: ::core::primitive::u128,
                    pub recipient_deposit: ::core::primitive::u128,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct HrmpOpenChannelRequest {
                    pub confirmed: ::core::primitive::bool,
                    pub _age: ::core::primitive::u32,
                    pub sender_deposit: ::core::primitive::u128,
                    pub max_message_size: ::core::primitive::u32,
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                }
            }
            pub mod inclusion {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {}
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        UnsortedOrDuplicateValidatorIndices,
                        #[codec(index = 1)]
                        UnsortedOrDuplicateDisputeStatementSet,
                        #[codec(index = 2)]
                        UnsortedOrDuplicateBackedCandidates,
                        #[codec(index = 3)]
                        UnexpectedRelayParent,
                        #[codec(index = 4)]
                        WrongBitfieldSize,
                        #[codec(index = 5)]
                        BitfieldAllZeros,
                        #[codec(index = 6)]
                        BitfieldDuplicateOrUnordered,
                        #[codec(index = 7)]
                        ValidatorIndexOutOfBounds,
                        #[codec(index = 8)]
                        InvalidBitfieldSignature,
                        #[codec(index = 9)]
                        UnscheduledCandidate,
                        #[codec(index = 10)]
                        CandidateScheduledBeforeParaFree,
                        #[codec(index = 11)]
                        WrongCollator,
                        #[codec(index = 12)]
                        ScheduledOutOfOrder,
                        #[codec(index = 13)]
                        HeadDataTooLarge,
                        #[codec(index = 14)]
                        PrematureCodeUpgrade,
                        #[codec(index = 15)]
                        NewCodeTooLarge,
                        #[codec(index = 16)]
                        CandidateNotInParentContext,
                        #[codec(index = 17)]
                        InvalidGroupIndex,
                        #[codec(index = 18)]
                        InsufficientBacking,
                        #[codec(index = 19)]
                        InvalidBacking,
                        #[codec(index = 20)]
                        NotCollatorSigned,
                        #[codec(index = 21)]
                        ValidationDataHashMismatch,
                        #[codec(index = 22)]
                        IncorrectDownwardMessageHandling,
                        #[codec(index = 23)]
                        InvalidUpwardMessages,
                        #[codec(index = 24)]
                        HrmpWatermarkMishandling,
                        #[codec(index = 25)]
                        InvalidOutboundHrmp,
                        #[codec(index = 26)]
                        InvalidValidationCodeHash,
                        #[codec(index = 27)]
                        ParaHeadMismatch,
                        #[codec(index = 28)]
                        BitfieldReferencesFreedCore,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        CandidateBacked(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                            runtime_types::polkadot_primitives::v1::GroupIndex,
                        ),
                        #[codec(index = 1)]
                        CandidateIncluded(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                            runtime_types::polkadot_primitives::v1::GroupIndex,
                        ),
                        #[codec(index = 2)]
                        CandidateTimedOut(
                            runtime_types::polkadot_primitives::v1::CandidateReceipt<
                                ::subxt::sp_core::H256,
                            >,
                            runtime_types::polkadot_parachain::primitives::HeadData,
                            runtime_types::polkadot_primitives::v1::CoreIndex,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AvailabilityBitfieldRecord<_0> {
                    pub bitfield: runtime_types::polkadot_primitives::v1::AvailabilityBitfield,
                    pub submitted_at: _0,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct CandidatePendingAvailability<_0, _1> {
                    pub core: runtime_types::polkadot_primitives::v1::CoreIndex,
                    pub hash: runtime_types::polkadot_core_primitives::CandidateHash,
                    pub descriptor: runtime_types::polkadot_primitives::v1::CandidateDescriptor<_0>,
                    pub availability_votes: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub backers: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub relay_parent_number: _1,
                    pub backed_in_number: _1,
                    pub backing_group: runtime_types::polkadot_primitives::v1::GroupIndex,
                }
            }
            pub mod initializer {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        force_approve { up_to: ::core::primitive::u32 },
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BufferedSessionChange {
                    pub validators: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub queued: ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v0::validator_app::Public,
                    >,
                    pub session_index: ::core::primitive::u32,
                }
            }
            pub mod origin {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Origin {
                        #[codec(index = 0)]
                        Parachain(runtime_types::polkadot_parachain::primitives::Id),
                    }
                }
            }
            pub mod paras {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        force_set_current_code {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                        },
                        #[codec(index = 1)]
                        force_set_current_head {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                        },
                        #[codec(index = 2)]
                        force_schedule_code_upgrade {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                            relay_parent_number: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        force_note_new_head {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                            new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                        },
                        #[codec(index = 4)]
                        force_queue_action {
                            para: runtime_types::polkadot_parachain::primitives::Id,
                        },
                        #[codec(index = 5)]
                        add_trusted_validation_code {
                            validation_code:
                                runtime_types::polkadot_parachain::primitives::ValidationCode,
                        },
                        #[codec(index = 6)]
                        poke_unused_validation_code {
                            validation_code_hash:
                                runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                        },
                        #[codec(index = 7)]
                        include_pvf_check_statement {
                            stmt: runtime_types::polkadot_primitives::v2::PvfCheckStatement,
                            signature:
                                runtime_types::polkadot_primitives::v0::validator_app::Signature,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        NotRegistered,
                        #[codec(index = 1)]
                        CannotOnboard,
                        #[codec(index = 2)]
                        CannotOffboard,
                        #[codec(index = 3)]
                        CannotUpgrade,
                        #[codec(index = 4)]
                        CannotDowngrade,
                        #[codec(index = 5)]
                        PvfCheckStatementStale,
                        #[codec(index = 6)]
                        PvfCheckStatementFuture,
                        #[codec(index = 7)]
                        PvfCheckValidatorIndexOutOfBounds,
                        #[codec(index = 8)]
                        PvfCheckInvalidSignature,
                        #[codec(index = 9)]
                        PvfCheckDoubleVote,
                        #[codec(index = 10)]
                        PvfCheckSubjectInvalid,
                        #[codec(index = 11)]
                        PvfCheckDisabled,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        CurrentCodeUpdated(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 1)]
                        CurrentHeadUpdated(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 2)]
                        CodeUpgradeScheduled(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 3)]
                        NewHeadNoted(runtime_types::polkadot_parachain::primitives::Id),
                        #[codec(index = 4)]
                        ActionQueued(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 5)]
                        PvfCheckStarted(
                            runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        #[codec(index = 6)]
                        PvfCheckAccepted(
                            runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                        #[codec(index = 7)]
                        PvfCheckRejected(
                            runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                            runtime_types::polkadot_parachain::primitives::Id,
                        ),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ParaGenesisArgs {
                    pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub validation_code:
                        runtime_types::polkadot_parachain::primitives::ValidationCode,
                    pub parachain: ::core::primitive::bool,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum ParaLifecycle {
                    #[codec(index = 0)]
                    Onboarding,
                    #[codec(index = 1)]
                    Parathread,
                    #[codec(index = 2)]
                    Parachain,
                    #[codec(index = 3)]
                    UpgradingParathread,
                    #[codec(index = 4)]
                    DowngradingParachain,
                    #[codec(index = 5)]
                    OffboardingParathread,
                    #[codec(index = 6)]
                    OffboardingParachain,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ParaPastCodeMeta<_0> {
                    pub upgrade_times: ::std::vec::Vec<
                        runtime_types::polkadot_runtime_parachains::paras::ReplacementTimes<_0>,
                    >,
                    pub last_pruned: ::core::option::Option<_0>,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct PvfCheckActiveVoteState<_0> {
                    pub votes_accept: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub votes_reject: ::subxt::bitvec::vec::BitVec<
                        ::subxt::bitvec::order::Lsb0,
                        ::core::primitive::u8,
                    >,
                    pub age: _0,
                    pub created_at: _0,
                    pub causes: ::std::vec::Vec<
                        runtime_types::polkadot_runtime_parachains::paras::PvfCheckCause<_0>,
                    >,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum PvfCheckCause<_0> {
                    #[codec(index = 0)]
                    Onboarding(runtime_types::polkadot_parachain::primitives::Id),
                    #[codec(index = 1)]
                    Upgrade {
                        id: runtime_types::polkadot_parachain::primitives::Id,
                        relay_parent_number: _0,
                    },
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ReplacementTimes<_0> {
                    pub expected_at: _0,
                    pub activated_at: _0,
                }
            }
            pub mod paras_inherent {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        enter {
                            data: runtime_types::polkadot_primitives::v1::InherentData<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                            >,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        TooManyInclusionInherents,
                        #[codec(index = 1)]
                        InvalidParentHeader,
                        #[codec(index = 2)]
                        CandidateConcludedInvalid,
                        #[codec(index = 3)]
                        InherentOverweight,
                        #[codec(index = 4)]
                        DisputeStatementsUnsortedOrDuplicates,
                        #[codec(index = 5)]
                        DisputeInvalid,
                    }
                }
            }
            pub mod scheduler {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum AssignmentKind {
                    #[codec(index = 0)]
                    Parachain,
                    #[codec(index = 1)]
                    Parathread(
                        runtime_types::polkadot_primitives::v0::collator_app::Public,
                        ::core::primitive::u32,
                    ),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct CoreAssignment {
                    pub core: runtime_types::polkadot_primitives::v1::CoreIndex,
                    pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                    pub kind: runtime_types::polkadot_runtime_parachains::scheduler::AssignmentKind,
                    pub group_idx: runtime_types::polkadot_primitives::v1::GroupIndex,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct ParathreadClaimQueue {
                    pub queue: ::std::vec::Vec<
                        runtime_types::polkadot_runtime_parachains::scheduler::QueuedParathread,
                    >,
                    pub next_core_offset: ::core::primitive::u32,
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct QueuedParathread {
                    pub claim: runtime_types::polkadot_primitives::v1::ParathreadEntry,
                    pub core_offset: ::core::primitive::u32,
                }
            }
            pub mod shared {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {}
                }
            }
            pub mod ump {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        service_overweight {
                            index: ::core::primitive::u64,
                            weight_limit: ::core::primitive::u64,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Error {
                        #[codec(index = 0)]
                        UnknownMessageIndex,
                        #[codec(index = 1)]
                        WeightOverLimit,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        InvalidFormat([::core::primitive::u8; 32usize]),
                        #[codec(index = 1)]
                        UnsupportedVersion([::core::primitive::u8; 32usize]),
                        #[codec(index = 2)]
                        ExecutedUpward(
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
                        UpwardMessagesReceived(
                            runtime_types::polkadot_parachain::primitives::Id,
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        ),
                        #[codec(index = 5)]
                        OverweightEnqueued(
                            runtime_types::polkadot_parachain::primitives::Id,
                            [::core::primitive::u8; 32usize],
                            ::core::primitive::u64,
                            ::core::primitive::u64,
                        ),
                        #[codec(index = 6)]
                        OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
                    }
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod rococo_runtime {
            use super::runtime_types;
            pub mod validator_manager {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Call {
                        #[codec(index = 0)]
                        register_validators {
                            validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        },
                        #[codec(index = 1)]
                        deregister_validators {
                            validators: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        },
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Event {
                        #[codec(index = 0)]
                        ValidatorsRegistered(
                            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        ),
                        #[codec(index = 1)]
                        ValidatorsDeregistered(
                            ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        ),
                    }
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Indices(runtime_types::pallet_indices::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 6)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 9)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 10)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 11)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 14)]
                Configuration(
                    runtime_types::polkadot_runtime_parachains::configuration::pallet::Call,
                ),
                #[codec(index = 15)]
                ParasShared(runtime_types::polkadot_runtime_parachains::shared::pallet::Call),
                #[codec(index = 16)]
                ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Call),
                #[codec(index = 17)]
                ParaInherent(
                    runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Call,
                ),
                #[codec(index = 19)]
                Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Call),
                #[codec(index = 20)]
                Initializer(runtime_types::polkadot_runtime_parachains::initializer::pallet::Call),
                #[codec(index = 21)]
                Dmp(runtime_types::polkadot_runtime_parachains::dmp::pallet::Call),
                #[codec(index = 22)]
                Ump(runtime_types::polkadot_runtime_parachains::ump::pallet::Call),
                #[codec(index = 23)]
                Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Call),
                #[codec(index = 25)]
                ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Call),
                #[codec(index = 26)]
                Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Call),
                #[codec(index = 27)]
                Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Call),
                #[codec(index = 28)]
                Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Call),
                #[codec(index = 29)]
                Slots(runtime_types::polkadot_runtime_common::slots::pallet::Call),
                #[codec(index = 30)]
                ParasSudoWrapper(
                    runtime_types::polkadot_runtime_common::paras_sudo_wrapper::pallet::Call,
                ),
                #[codec(index = 31)]
                AssignedSlots(runtime_types::polkadot_runtime_common::assigned_slots::pallet::Call),
                #[codec(index = 32)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 34)]
                Beefy(runtime_types::pallet_beefy::pallet::Call),
                #[codec(index = 36)]
                ValidatorManager(runtime_types::rococo_runtime::validator_manager::pallet::Call),
                #[codec(index = 40)]
                BridgeRococoGrandpa(runtime_types::pallet_bridge_grandpa::pallet::Call),
                #[codec(index = 41)]
                BridgeWococoGrandpa(runtime_types::pallet_bridge_grandpa::pallet::Call),
                #[codec(index = 43)]
                BridgeRococoMessages(runtime_types::pallet_bridge_messages::pallet::Call),
                #[codec(index = 44)]
                BridgeWococoMessages(runtime_types::pallet_bridge_messages::pallet::Call),
                #[codec(index = 80)]
                Collective(runtime_types::pallet_collective::pallet::Call),
                #[codec(index = 81)]
                Membership(runtime_types::pallet_membership::pallet::Call),
                #[codec(index = 90)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 91)]
                Proxy(runtime_types::pallet_proxy::pallet::Call),
                #[codec(index = 92)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 99)]
                XcmPallet(runtime_types::pallet_xcm::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 3)]
                Indices(runtime_types::pallet_indices::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 7)]
                Offences(runtime_types::pallet_offences::pallet::Event),
                #[codec(index = 9)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 10)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 11)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 16)]
                ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event),
                #[codec(index = 19)]
                Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Event),
                #[codec(index = 22)]
                Ump(runtime_types::polkadot_runtime_parachains::ump::pallet::Event),
                #[codec(index = 23)]
                Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event),
                #[codec(index = 25)]
                ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Event),
                #[codec(index = 26)]
                Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event),
                #[codec(index = 27)]
                Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Event),
                #[codec(index = 28)]
                Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Event),
                #[codec(index = 29)]
                Slots(runtime_types::polkadot_runtime_common::slots::pallet::Event),
                #[codec(index = 31)]
                AssignedSlots(
                    runtime_types::polkadot_runtime_common::assigned_slots::pallet::Event,
                ),
                #[codec(index = 32)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 36)]
                ValidatorManager(runtime_types::rococo_runtime::validator_manager::pallet::Event),
                #[codec(index = 43)]
                BridgeRococoMessages(runtime_types::pallet_bridge_messages::pallet::Event),
                #[codec(index = 44)]
                BridgeWococoMessages(runtime_types::pallet_bridge_messages::pallet::Event),
                #[codec(index = 45)]
                BridgeRococoMessagesDispatch(runtime_types::pallet_bridge_dispatch::pallet::Event),
                #[codec(index = 46)]
                BridgeWococoMessagesDispatch(runtime_types::pallet_bridge_dispatch::pallet::Event),
                #[codec(index = 80)]
                Collective(runtime_types::pallet_collective::pallet::Event),
                #[codec(index = 81)]
                Membership(runtime_types::pallet_membership::pallet::Event),
                #[codec(index = 90)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 91)]
                Proxy(runtime_types::pallet_proxy::pallet::Event),
                #[codec(index = 92)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 99)]
                XcmPallet(runtime_types::pallet_xcm::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_system::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
                ),
                #[codec(index = 13)]
                ParachainsOrigin(
                    runtime_types::polkadot_runtime_parachains::origin::pallet::Origin,
                ),
                #[codec(index = 80)]
                Collective(
                    runtime_types::pallet_collective::RawOrigin<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ),
                #[codec(index = 99)]
                XcmPallet(runtime_types::pallet_xcm::pallet::Origin),
                #[codec(index = 4)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum ProxyType {
                #[codec(index = 0)]
                Any,
                #[codec(index = 1)]
                CancelProxy,
                #[codec(index = 2)]
                Auction,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct Runtime;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct SessionKeys {
                pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                pub babe: runtime_types::sp_consensus_babe::app::Public,
                pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                pub para_validator: runtime_types::polkadot_primitives::v0::validator_app::Public,
                pub para_assignment: runtime_types::polkadot_primitives::v1::assignment_app::Public,
                pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
                pub beefy: runtime_types::beefy_primitives::crypto::Public,
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum DigestItem {
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
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _2, _1, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct BlakeTwo256;
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
        pub mod sp_session {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_staking {
            use super::runtime_types;
            pub mod offence {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: ::std::vec::Vec<_0>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod double_encoded {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct DoubleEncoded {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod v0 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum BodyId {
                        #[codec(index = 0)]
                        Unit,
                        #[codec(index = 1)]
                        Named(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 2)]
                        Index(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        Executive,
                        #[codec(index = 4)]
                        Technical,
                        #[codec(index = 5)]
                        Legislative,
                        #[codec(index = 6)]
                        Judicial,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parent,
                        #[codec(index = 1)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
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
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                        AllAbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 5)]
                        AllAbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                        },
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
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
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v1::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct MultiAssets(
                        pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
                    );
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v1::multilocation::Junctions,
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    Version(::core::primitive::u32),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                        MaxWeightInvalid,
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
                    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                    pub enum Outcome {
                        #[codec(index = 0)]
                        Complete(::core::primitive::u64),
                        #[codec(index = 1)]
                        Incomplete(
                            ::core::primitive::u64,
                            runtime_types::xcm::v2::traits::Error,
                        ),
                        #[codec(index = 2)]
                        Error(runtime_types::xcm::v2::traits::Error),
                    }
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                    Trap(#[codec(compact)] ::core::primitive::u64),
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(#[codec(compact)] ::core::primitive::u64),
                }
                #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
                pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum VersionedMultiAssets {
                #[codec(index = 0)]
                V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multiasset::MultiAssets),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum VersionedMultiLocation {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::multi_location::MultiLocation),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multilocation::MultiLocation),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
            pub enum VersionedResponse {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Response),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Response),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Response),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug)]
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
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = self::runtime_types::sp_runtime::DispatchError;
    pub struct ErrorDetails {
        pub pallet: &'static str,
        pub error: &'static str,
        pub docs: &'static str,
    }
    impl DispatchError {
        pub fn details(&self) -> Option<ErrorDetails> {
            if let Self::Module { index, error } = self {
                match (index , error) { (0u8 , 0u8) => Some (ErrorDetails { pallet : "System" , error : "InvalidSpecName" , docs : "The name of specification does not match between the current runtime\nand the new runtime." }) , (0u8 , 1u8) => Some (ErrorDetails { pallet : "System" , error : "SpecVersionNeedsToIncrease" , docs : "The specification version is not allowed to decrease between the current runtime\nand the new runtime." }) , (0u8 , 2u8) => Some (ErrorDetails { pallet : "System" , error : "FailedToExtractRuntimeVersion" , docs : "Failed to extract the runtime version from the new runtime.\n\nEither calling `Core_version` or decoding `RuntimeVersion` failed." }) , (0u8 , 3u8) => Some (ErrorDetails { pallet : "System" , error : "NonDefaultComposite" , docs : "Suicide called when the account has non-default composite data." }) , (0u8 , 4u8) => Some (ErrorDetails { pallet : "System" , error : "NonZeroRefCount" , docs : "There is a non-zero reference count preventing the account from being purged." }) , (0u8 , 5u8) => Some (ErrorDetails { pallet : "System" , error : "CallFiltered" , docs : "The origin filter prevent the call to be dispatched." }) , (1u8 , 0u8) => Some (ErrorDetails { pallet : "Babe" , error : "InvalidEquivocationProof" , docs : "An equivocation proof provided as part of an equivocation report is invalid." }) , (1u8 , 1u8) => Some (ErrorDetails { pallet : "Babe" , error : "InvalidKeyOwnershipProof" , docs : "A key ownership proof provided as part of an equivocation report is invalid." }) , (1u8 , 2u8) => Some (ErrorDetails { pallet : "Babe" , error : "DuplicateOffenceReport" , docs : "A given equivocation report is valid but already previously reported." }) , (3u8 , 0u8) => Some (ErrorDetails { pallet : "Indices" , error : "NotAssigned" , docs : "The index was not already assigned." }) , (3u8 , 1u8) => Some (ErrorDetails { pallet : "Indices" , error : "NotOwner" , docs : "The index is assigned to another account." }) , (3u8 , 2u8) => Some (ErrorDetails { pallet : "Indices" , error : "InUse" , docs : "The index was not available." }) , (3u8 , 3u8) => Some (ErrorDetails { pallet : "Indices" , error : "NotTransfer" , docs : "The source and destination accounts are identical." }) , (3u8 , 4u8) => Some (ErrorDetails { pallet : "Indices" , error : "Permanent" , docs : "The index is permanent and may not be freed/changed." }) , (4u8 , 0u8) => Some (ErrorDetails { pallet : "Balances" , error : "VestingBalance" , docs : "Vesting balance too high to send value" }) , (4u8 , 1u8) => Some (ErrorDetails { pallet : "Balances" , error : "LiquidityRestrictions" , docs : "Account liquidity restrictions prevent withdrawal" }) , (4u8 , 2u8) => Some (ErrorDetails { pallet : "Balances" , error : "InsufficientBalance" , docs : "Balance too low to send value" }) , (4u8 , 3u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistentialDeposit" , docs : "Value too low to create account due to existential deposit" }) , (4u8 , 4u8) => Some (ErrorDetails { pallet : "Balances" , error : "KeepAlive" , docs : "Transfer/payment would kill account" }) , (4u8 , 5u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistingVestingSchedule" , docs : "A vesting schedule already exists for this account" }) , (4u8 , 6u8) => Some (ErrorDetails { pallet : "Balances" , error : "DeadAccount" , docs : "Beneficiary account must pre-exist" }) , (4u8 , 7u8) => Some (ErrorDetails { pallet : "Balances" , error : "TooManyReserves" , docs : "Number of named reserves exceed MaxReserves" }) , (6u8 , 0u8) => Some (ErrorDetails { pallet : "Authorship" , error : "InvalidUncleParent" , docs : "The uncle parent not in the chain." }) , (6u8 , 1u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UnclesAlreadySet" , docs : "Uncles already set in the block." }) , (6u8 , 2u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooManyUncles" , docs : "Too many uncles." }) , (6u8 , 3u8) => Some (ErrorDetails { pallet : "Authorship" , error : "GenesisUncle" , docs : "The uncle is genesis." }) , (6u8 , 4u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooHighUncle" , docs : "The uncle is too high in chain." }) , (6u8 , 5u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UncleAlreadyIncluded" , docs : "The uncle is already included." }) , (6u8 , 6u8) => Some (ErrorDetails { pallet : "Authorship" , error : "OldUncle" , docs : "The uncle isn't recent enough to be included." }) , (9u8 , 0u8) => Some (ErrorDetails { pallet : "Session" , error : "InvalidProof" , docs : "Invalid ownership proof." }) , (9u8 , 1u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAssociatedValidatorId" , docs : "No associated validator ID for account." }) , (9u8 , 2u8) => Some (ErrorDetails { pallet : "Session" , error : "DuplicatedKey" , docs : "Registered duplicate key." }) , (9u8 , 3u8) => Some (ErrorDetails { pallet : "Session" , error : "NoKeys" , docs : "No keys are associated with this account." }) , (9u8 , 4u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAccount" , docs : "Key setting account is not live, so it's impossible to associate keys." }) , (10u8 , 0u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "PauseFailed" , docs : "Attempt to signal GRANDPA pause when the authority set isn't live\n(either paused or already pending pause)." }) , (10u8 , 1u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ResumeFailed" , docs : "Attempt to signal GRANDPA resume when the authority set isn't paused\n(either live or already pending resume)." }) , (10u8 , 2u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ChangePending" , docs : "Attempt to signal GRANDPA change with one already pending." }) , (10u8 , 3u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "TooSoon" , docs : "Cannot signal forced change so soon after last." }) , (10u8 , 4u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidKeyOwnershipProof" , docs : "A key ownership proof provided as part of an equivocation report is invalid." }) , (10u8 , 5u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidEquivocationProof" , docs : "An equivocation proof provided as part of an equivocation report is invalid." }) , (10u8 , 6u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "DuplicateOffenceReport" , docs : "A given equivocation report is valid but already previously reported." }) , (11u8 , 0u8) => Some (ErrorDetails { pallet : "ImOnline" , error : "InvalidKey" , docs : "Non existent public key." }) , (11u8 , 1u8) => Some (ErrorDetails { pallet : "ImOnline" , error : "DuplicatedHeartbeat" , docs : "Duplicated heartbeat." }) , (14u8 , 0u8) => Some (ErrorDetails { pallet : "Configuration" , error : "InvalidNewValue" , docs : "The new value for a configuration parameter is invalid." }) , (16u8 , 0u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "UnsortedOrDuplicateValidatorIndices" , docs : "Validator indices are out of order or contains duplicates." }) , (16u8 , 1u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "UnsortedOrDuplicateDisputeStatementSet" , docs : "Dispute statement sets are out of order or contain duplicates." }) , (16u8 , 2u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "UnsortedOrDuplicateBackedCandidates" , docs : "Backed candidates are out of order (core index) or contain duplicates." }) , (16u8 , 3u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "UnexpectedRelayParent" , docs : "A different relay parent was provided compared to the on-chain stored one." }) , (16u8 , 4u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "WrongBitfieldSize" , docs : "Availability bitfield has unexpected size." }) , (16u8 , 5u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "BitfieldAllZeros" , docs : "Bitfield consists of zeros only." }) , (16u8 , 6u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "BitfieldDuplicateOrUnordered" , docs : "Multiple bitfields submitted by same validator or validators out of order by index." }) , (16u8 , 7u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "ValidatorIndexOutOfBounds" , docs : "Validator index out of bounds." }) , (16u8 , 8u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "InvalidBitfieldSignature" , docs : "Invalid signature" }) , (16u8 , 9u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "UnscheduledCandidate" , docs : "Candidate submitted but para not scheduled." }) , (16u8 , 10u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "CandidateScheduledBeforeParaFree" , docs : "Candidate scheduled despite pending candidate already existing for the para." }) , (16u8 , 11u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "WrongCollator" , docs : "Candidate included with the wrong collator." }) , (16u8 , 12u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "ScheduledOutOfOrder" , docs : "Scheduled cores out of order." }) , (16u8 , 13u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "HeadDataTooLarge" , docs : "Head data exceeds the configured maximum." }) , (16u8 , 14u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "PrematureCodeUpgrade" , docs : "Code upgrade prematurely." }) , (16u8 , 15u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "NewCodeTooLarge" , docs : "Output code is too large" }) , (16u8 , 16u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "CandidateNotInParentContext" , docs : "Candidate not in parent context." }) , (16u8 , 17u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "InvalidGroupIndex" , docs : "Invalid group index in core assignment." }) , (16u8 , 18u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "InsufficientBacking" , docs : "Insufficient (non-majority) backing." }) , (16u8 , 19u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "InvalidBacking" , docs : "Invalid (bad signature, unknown validator, etc.) backing." }) , (16u8 , 20u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "NotCollatorSigned" , docs : "Collator did not sign PoV." }) , (16u8 , 21u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "ValidationDataHashMismatch" , docs : "The validation data hash does not match expected." }) , (16u8 , 22u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "IncorrectDownwardMessageHandling" , docs : "The downward message queue is not processed correctly." }) , (16u8 , 23u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "InvalidUpwardMessages" , docs : "At least one upward message sent does not pass the acceptance criteria." }) , (16u8 , 24u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "HrmpWatermarkMishandling" , docs : "The candidate didn't follow the rules of HRMP watermark advancement." }) , (16u8 , 25u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "InvalidOutboundHrmp" , docs : "The HRMP messages sent by the candidate is not valid." }) , (16u8 , 26u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "InvalidValidationCodeHash" , docs : "The validation code hash of the candidate is not valid." }) , (16u8 , 27u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "ParaHeadMismatch" , docs : "The `para_head` hash in the candidate descriptor doesn't match the hash of the actual para head in the\ncommitments." }) , (16u8 , 28u8) => Some (ErrorDetails { pallet : "ParaInclusion" , error : "BitfieldReferencesFreedCore" , docs : "A bitfield that references a freed core,\neither intentionally or as part of a concluded\ninvalid dispute." }) , (17u8 , 0u8) => Some (ErrorDetails { pallet : "ParaInherent" , error : "TooManyInclusionInherents" , docs : "Inclusion inherent called more than once per block." }) , (17u8 , 1u8) => Some (ErrorDetails { pallet : "ParaInherent" , error : "InvalidParentHeader" , docs : "The hash of the submitted parent header doesn't correspond to the saved block hash of\nthe parent." }) , (17u8 , 2u8) => Some (ErrorDetails { pallet : "ParaInherent" , error : "CandidateConcludedInvalid" , docs : "Disputed candidate that was concluded invalid." }) , (17u8 , 3u8) => Some (ErrorDetails { pallet : "ParaInherent" , error : "InherentOverweight" , docs : "The data given to the inherent will result in an overweight block." }) , (17u8 , 4u8) => Some (ErrorDetails { pallet : "ParaInherent" , error : "DisputeStatementsUnsortedOrDuplicates" , docs : "The ordering of dispute statements was invalid." }) , (17u8 , 5u8) => Some (ErrorDetails { pallet : "ParaInherent" , error : "DisputeInvalid" , docs : "A dispute statement was invalid." }) , (19u8 , 0u8) => Some (ErrorDetails { pallet : "Paras" , error : "NotRegistered" , docs : "Para is not registered in our system." }) , (19u8 , 1u8) => Some (ErrorDetails { pallet : "Paras" , error : "CannotOnboard" , docs : "Para cannot be onboarded because it is already tracked by our system." }) , (19u8 , 2u8) => Some (ErrorDetails { pallet : "Paras" , error : "CannotOffboard" , docs : "Para cannot be offboarded at this time." }) , (19u8 , 3u8) => Some (ErrorDetails { pallet : "Paras" , error : "CannotUpgrade" , docs : "Para cannot be upgraded to a parachain." }) , (19u8 , 4u8) => Some (ErrorDetails { pallet : "Paras" , error : "CannotDowngrade" , docs : "Para cannot be downgraded to a parathread." }) , (19u8 , 5u8) => Some (ErrorDetails { pallet : "Paras" , error : "PvfCheckStatementStale" , docs : "The statement for PVF pre-checking is stale." }) , (19u8 , 6u8) => Some (ErrorDetails { pallet : "Paras" , error : "PvfCheckStatementFuture" , docs : "The statement for PVF pre-checking is for a future session." }) , (19u8 , 7u8) => Some (ErrorDetails { pallet : "Paras" , error : "PvfCheckValidatorIndexOutOfBounds" , docs : "Claimed validator index is out of bounds." }) , (19u8 , 8u8) => Some (ErrorDetails { pallet : "Paras" , error : "PvfCheckInvalidSignature" , docs : "The signature for the PVF pre-checking is invalid." }) , (19u8 , 9u8) => Some (ErrorDetails { pallet : "Paras" , error : "PvfCheckDoubleVote" , docs : "The given validator already has cast a vote." }) , (19u8 , 10u8) => Some (ErrorDetails { pallet : "Paras" , error : "PvfCheckSubjectInvalid" , docs : "The given PVF does not exist at the moment of process a vote." }) , (19u8 , 11u8) => Some (ErrorDetails { pallet : "Paras" , error : "PvfCheckDisabled" , docs : "The PVF pre-checking statement cannot be included since the PVF pre-checking mechanism\nis disabled." }) , (22u8 , 0u8) => Some (ErrorDetails { pallet : "Ump" , error : "UnknownMessageIndex" , docs : "The message index given is unknown." }) , (22u8 , 1u8) => Some (ErrorDetails { pallet : "Ump" , error : "WeightOverLimit" , docs : "The amount of weight given is possibly not enough for executing the message." }) , (23u8 , 0u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelToSelf" , docs : "The sender tried to open a channel to themselves." }) , (23u8 , 1u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelInvalidRecipient" , docs : "The recipient is not a valid para." }) , (23u8 , 2u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelZeroCapacity" , docs : "The requested capacity is zero." }) , (23u8 , 3u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelCapacityExceedsLimit" , docs : "The requested capacity exceeds the global limit." }) , (23u8 , 4u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelZeroMessageSize" , docs : "The requested maximum message size is 0." }) , (23u8 , 5u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelMessageSizeExceedsLimit" , docs : "The open request requested the message size that exceeds the global limit." }) , (23u8 , 6u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelAlreadyExists" , docs : "The channel already exists" }) , (23u8 , 7u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelAlreadyRequested" , docs : "There is already a request to open the same channel." }) , (23u8 , 8u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelLimitExceeded" , docs : "The sender already has the maximum number of allowed outbound channels." }) , (23u8 , 9u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "AcceptHrmpChannelDoesntExist" , docs : "The channel from the sender to the origin doesn't exist." }) , (23u8 , 10u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "AcceptHrmpChannelAlreadyConfirmed" , docs : "The channel is already confirmed." }) , (23u8 , 11u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "AcceptHrmpChannelLimitExceeded" , docs : "The recipient already has the maximum number of allowed inbound channels." }) , (23u8 , 12u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "CloseHrmpChannelUnauthorized" , docs : "The origin tries to close a channel where it is neither the sender nor the recipient." }) , (23u8 , 13u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "CloseHrmpChannelDoesntExist" , docs : "The channel to be closed doesn't exist." }) , (23u8 , 14u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "CloseHrmpChannelAlreadyUnderway" , docs : "The channel close request is already requested." }) , (23u8 , 15u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "CancelHrmpOpenChannelUnauthorized" , docs : "Canceling is requested by neither the sender nor recipient of the open channel request." }) , (23u8 , 16u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelDoesntExist" , docs : "The open request doesn't exist." }) , (23u8 , 17u8) => Some (ErrorDetails { pallet : "Hrmp" , error : "OpenHrmpChannelAlreadyConfirmed" , docs : "Cannot cancel an HRMP open channel request because it is already confirmed." }) , (25u8 , 0u8) => Some (ErrorDetails { pallet : "ParasDisputes" , error : "DuplicateDisputeStatementSets" , docs : "Duplicate dispute statement sets provided." }) , (25u8 , 1u8) => Some (ErrorDetails { pallet : "ParasDisputes" , error : "AncientDisputeStatement" , docs : "Ancient dispute statement provided." }) , (25u8 , 2u8) => Some (ErrorDetails { pallet : "ParasDisputes" , error : "ValidatorIndexOutOfBounds" , docs : "Validator index on statement is out of bounds for session." }) , (25u8 , 3u8) => Some (ErrorDetails { pallet : "ParasDisputes" , error : "InvalidSignature" , docs : "Invalid signature on statement." }) , (25u8 , 4u8) => Some (ErrorDetails { pallet : "ParasDisputes" , error : "DuplicateStatement" , docs : "Validator vote submitted more than once to dispute." }) , (25u8 , 5u8) => Some (ErrorDetails { pallet : "ParasDisputes" , error : "PotentialSpam" , docs : "Too many spam slots used by some specific validator." }) , (25u8 , 6u8) => Some (ErrorDetails { pallet : "ParasDisputes" , error : "SingleSidedDispute" , docs : "A dispute where there are only votes on one side." }) , (26u8 , 0u8) => Some (ErrorDetails { pallet : "Registrar" , error : "NotRegistered" , docs : "The ID is not registered." }) , (26u8 , 1u8) => Some (ErrorDetails { pallet : "Registrar" , error : "AlreadyRegistered" , docs : "The ID is already registered." }) , (26u8 , 2u8) => Some (ErrorDetails { pallet : "Registrar" , error : "NotOwner" , docs : "The caller is not the owner of this Id." }) , (26u8 , 3u8) => Some (ErrorDetails { pallet : "Registrar" , error : "CodeTooLarge" , docs : "Invalid para code size." }) , (26u8 , 4u8) => Some (ErrorDetails { pallet : "Registrar" , error : "HeadDataTooLarge" , docs : "Invalid para head data size." }) , (26u8 , 5u8) => Some (ErrorDetails { pallet : "Registrar" , error : "NotParachain" , docs : "Para is not a Parachain." }) , (26u8 , 6u8) => Some (ErrorDetails { pallet : "Registrar" , error : "NotParathread" , docs : "Para is not a Parathread." }) , (26u8 , 7u8) => Some (ErrorDetails { pallet : "Registrar" , error : "CannotDeregister" , docs : "Cannot deregister para" }) , (26u8 , 8u8) => Some (ErrorDetails { pallet : "Registrar" , error : "CannotDowngrade" , docs : "Cannot schedule downgrade of parachain to parathread" }) , (26u8 , 9u8) => Some (ErrorDetails { pallet : "Registrar" , error : "CannotUpgrade" , docs : "Cannot schedule upgrade of parathread to parachain" }) , (26u8 , 10u8) => Some (ErrorDetails { pallet : "Registrar" , error : "ParaLocked" , docs : "Para is locked from manipulation by the manager. Must use parachain or relay chain governance." }) , (26u8 , 11u8) => Some (ErrorDetails { pallet : "Registrar" , error : "NotReserved" , docs : "The ID given for registration has not been reserved." }) , (26u8 , 12u8) => Some (ErrorDetails { pallet : "Registrar" , error : "EmptyCode" , docs : "Registering parachain with empty code is not allowed." }) , (27u8 , 0u8) => Some (ErrorDetails { pallet : "Auctions" , error : "AuctionInProgress" , docs : "This auction is already in progress." }) , (27u8 , 1u8) => Some (ErrorDetails { pallet : "Auctions" , error : "LeasePeriodInPast" , docs : "The lease period is in the past." }) , (27u8 , 2u8) => Some (ErrorDetails { pallet : "Auctions" , error : "ParaNotRegistered" , docs : "Para is not registered" }) , (27u8 , 3u8) => Some (ErrorDetails { pallet : "Auctions" , error : "NotCurrentAuction" , docs : "Not a current auction." }) , (27u8 , 4u8) => Some (ErrorDetails { pallet : "Auctions" , error : "NotAuction" , docs : "Not an auction." }) , (27u8 , 5u8) => Some (ErrorDetails { pallet : "Auctions" , error : "AuctionEnded" , docs : "Auction has already ended." }) , (27u8 , 6u8) => Some (ErrorDetails { pallet : "Auctions" , error : "AlreadyLeasedOut" , docs : "The para is already leased out for part of this range." }) , (28u8 , 0u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "FirstPeriodInPast" , docs : "The current lease period is more than the first lease period." }) , (28u8 , 1u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "FirstPeriodTooFarInFuture" , docs : "The first lease period needs to at least be less than 3 `max_value`." }) , (28u8 , 2u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "LastPeriodBeforeFirstPeriod" , docs : "Last lease period must be greater than first lease period." }) , (28u8 , 3u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "LastPeriodTooFarInFuture" , docs : "The last lease period cannot be more than 3 periods after the first period." }) , (28u8 , 4u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "CannotEndInPast" , docs : "The campaign ends before the current block number. The end must be in the future." }) , (28u8 , 5u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "EndTooFarInFuture" , docs : "The end date for this crowdloan is not sensible." }) , (28u8 , 6u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "Overflow" , docs : "There was an overflow." }) , (28u8 , 7u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "ContributionTooSmall" , docs : "The contribution was below the minimum, `MinContribution`." }) , (28u8 , 8u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "InvalidParaId" , docs : "Invalid fund index." }) , (28u8 , 9u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "CapExceeded" , docs : "Contributions exceed maximum amount." }) , (28u8 , 10u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "ContributionPeriodOver" , docs : "The contribution period has already ended." }) , (28u8 , 11u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "InvalidOrigin" , docs : "The origin of this call is invalid." }) , (28u8 , 12u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "NotParachain" , docs : "This crowdloan does not correspond to a parachain." }) , (28u8 , 13u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "LeaseActive" , docs : "This parachain lease is still active and retirement cannot yet begin." }) , (28u8 , 14u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "BidOrLeaseActive" , docs : "This parachain's bid or lease is still active and withdraw cannot yet begin." }) , (28u8 , 15u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "FundNotEnded" , docs : "The crowdloan has not yet ended." }) , (28u8 , 16u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "NoContributions" , docs : "There are no contributions stored in this crowdloan." }) , (28u8 , 17u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "NotReadyToDissolve" , docs : "The crowdloan is not ready to dissolve. Potentially still has a slot or in retirement period." }) , (28u8 , 18u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "InvalidSignature" , docs : "Invalid signature." }) , (28u8 , 19u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "MemoTooLarge" , docs : "The provided memo is too large." }) , (28u8 , 20u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "AlreadyInNewRaise" , docs : "The fund is already in `NewRaise`" }) , (28u8 , 21u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "VrfDelayInProgress" , docs : "No contributions allowed during the VRF delay" }) , (28u8 , 22u8) => Some (ErrorDetails { pallet : "Crowdloan" , error : "NoLeasePeriod" , docs : "A lease period has not started yet, due to an offset in the starting block." }) , (29u8 , 0u8) => Some (ErrorDetails { pallet : "Slots" , error : "ParaNotOnboarding" , docs : "The parachain ID is not onboarding." }) , (29u8 , 1u8) => Some (ErrorDetails { pallet : "Slots" , error : "LeaseError" , docs : "There was an error with the lease." }) , (30u8 , 0u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "ParaDoesntExist" , docs : "The specified parachain or parathread is not registered." }) , (30u8 , 1u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "ParaAlreadyExists" , docs : "The specified parachain or parathread is already registered." }) , (30u8 , 2u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "ExceedsMaxMessageSize" , docs : "A DMP message couldn't be sent because it exceeds the maximum size allowed for a downward\nmessage." }) , (30u8 , 3u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "CouldntCleanup" , docs : "Could not schedule para cleanup." }) , (30u8 , 4u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "NotParathread" , docs : "Not a parathread." }) , (30u8 , 5u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "NotParachain" , docs : "Not a parachain." }) , (30u8 , 6u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "CannotUpgrade" , docs : "Cannot upgrade parathread." }) , (30u8 , 7u8) => Some (ErrorDetails { pallet : "ParasSudoWrapper" , error : "CannotDowngrade" , docs : "Cannot downgrade parachain." }) , (31u8 , 0u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "ParaDoesntExist" , docs : "The specified parachain or parathread is not registered." }) , (31u8 , 1u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "NotParathread" , docs : "Not a parathread." }) , (31u8 , 2u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "CannotUpgrade" , docs : "Cannot upgrade parathread." }) , (31u8 , 3u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "CannotDowngrade" , docs : "Cannot downgrade parachain." }) , (31u8 , 4u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "SlotAlreadyAssigned" , docs : "Permanent or Temporary slot already assigned." }) , (31u8 , 5u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "SlotNotAssigned" , docs : "Permanent or Temporary slot has not been assigned." }) , (31u8 , 6u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "OngoingLeaseExists" , docs : "An ongoing lease already exists." }) , (31u8 , 7u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "MaxPermanentSlotsExceeded" , docs : "" }) , (31u8 , 8u8) => Some (ErrorDetails { pallet : "AssignedSlots" , error : "MaxTemporarySlotsExceeded" , docs : "" }) , (32u8 , 0u8) => Some (ErrorDetails { pallet : "Sudo" , error : "RequireSudo" , docs : "Sender must be the Sudo account" }) , (40u8 , 0u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "InvalidJustification" , docs : "The given justification is invalid for the given header." }) , (40u8 , 1u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "InvalidAuthoritySet" , docs : "The authority set from the underlying header chain is invalid." }) , (40u8 , 2u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "TooManyRequests" , docs : "There are too many requests for the current window to handle." }) , (40u8 , 3u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "OldHeader" , docs : "The header being imported is older than the best finalized header known to the pallet." }) , (40u8 , 4u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "UnknownHeader" , docs : "The header is unknown to the pallet." }) , (40u8 , 5u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "UnsupportedScheduledChange" , docs : "The scheduled authority set change found in the header is unsupported by the pallet.\n\nThis is the case for non-standard (e.g forced) authority set changes." }) , (40u8 , 6u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "NotInitialized" , docs : "The pallet is not yet initialized." }) , (40u8 , 7u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "AlreadyInitialized" , docs : "The pallet has already been initialized." }) , (40u8 , 8u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "Halted" , docs : "All pallet operations are halted." }) , (40u8 , 9u8) => Some (ErrorDetails { pallet : "BridgeRococoGrandpa" , error : "StorageRootMismatch" , docs : "The storage proof doesn't contains storage root. So it is invalid for given header." }) , (41u8 , 0u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "InvalidJustification" , docs : "The given justification is invalid for the given header." }) , (41u8 , 1u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "InvalidAuthoritySet" , docs : "The authority set from the underlying header chain is invalid." }) , (41u8 , 2u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "TooManyRequests" , docs : "There are too many requests for the current window to handle." }) , (41u8 , 3u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "OldHeader" , docs : "The header being imported is older than the best finalized header known to the pallet." }) , (41u8 , 4u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "UnknownHeader" , docs : "The header is unknown to the pallet." }) , (41u8 , 5u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "UnsupportedScheduledChange" , docs : "The scheduled authority set change found in the header is unsupported by the pallet.\n\nThis is the case for non-standard (e.g forced) authority set changes." }) , (41u8 , 6u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "NotInitialized" , docs : "The pallet is not yet initialized." }) , (41u8 , 7u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "AlreadyInitialized" , docs : "The pallet has already been initialized." }) , (41u8 , 8u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "Halted" , docs : "All pallet operations are halted." }) , (41u8 , 9u8) => Some (ErrorDetails { pallet : "BridgeWococoGrandpa" , error : "StorageRootMismatch" , docs : "The storage proof doesn't contains storage root. So it is invalid for given header." }) , (43u8 , 0u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "Halted" , docs : "All pallet operations are halted." }) , (43u8 , 1u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "MessageRejectedByChainVerifier" , docs : "Message has been treated as invalid by chain verifier." }) , (43u8 , 2u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "MessageRejectedByLaneVerifier" , docs : "Message has been treated as invalid by lane verifier." }) , (43u8 , 3u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "FailedToWithdrawMessageFee" , docs : "Submitter has failed to pay fee for delivering and dispatching messages." }) , (43u8 , 4u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "TooManyMessagesInTheProof" , docs : "The transaction brings too many messages." }) , (43u8 , 5u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "InvalidMessagesProof" , docs : "Invalid messages has been submitted." }) , (43u8 , 6u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "InvalidMessagesDeliveryProof" , docs : "Invalid messages delivery proof has been submitted." }) , (43u8 , 7u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "InvalidUnrewardedRelayers" , docs : "The bridged chain has invalid `UnrewardedRelayers` in its storage (fatal for the lane)." }) , (43u8 , 8u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "InvalidUnrewardedRelayersState" , docs : "The relayer has declared invalid unrewarded relayers state in the\n`receive_messages_delivery_proof` call." }) , (43u8 , 9u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "MessageIsAlreadyDelivered" , docs : "The message someone is trying to work with (i.e. increase fee) is already-delivered." }) , (43u8 , 10u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "MessageIsNotYetSent" , docs : "The message someone is trying to work with (i.e. increase fee) is not yet sent." }) , (43u8 , 11u8) => Some (ErrorDetails { pallet : "BridgeRococoMessages" , error : "TryingToConfirmMoreMessagesThanExpected" , docs : "The number of actually confirmed messages is going to be larger than the number of\nmessages in the proof. This may mean that this or bridged chain storage is corrupted." }) , (44u8 , 0u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "Halted" , docs : "All pallet operations are halted." }) , (44u8 , 1u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "MessageRejectedByChainVerifier" , docs : "Message has been treated as invalid by chain verifier." }) , (44u8 , 2u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "MessageRejectedByLaneVerifier" , docs : "Message has been treated as invalid by lane verifier." }) , (44u8 , 3u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "FailedToWithdrawMessageFee" , docs : "Submitter has failed to pay fee for delivering and dispatching messages." }) , (44u8 , 4u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "TooManyMessagesInTheProof" , docs : "The transaction brings too many messages." }) , (44u8 , 5u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "InvalidMessagesProof" , docs : "Invalid messages has been submitted." }) , (44u8 , 6u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "InvalidMessagesDeliveryProof" , docs : "Invalid messages delivery proof has been submitted." }) , (44u8 , 7u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "InvalidUnrewardedRelayers" , docs : "The bridged chain has invalid `UnrewardedRelayers` in its storage (fatal for the lane)." }) , (44u8 , 8u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "InvalidUnrewardedRelayersState" , docs : "The relayer has declared invalid unrewarded relayers state in the\n`receive_messages_delivery_proof` call." }) , (44u8 , 9u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "MessageIsAlreadyDelivered" , docs : "The message someone is trying to work with (i.e. increase fee) is already-delivered." }) , (44u8 , 10u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "MessageIsNotYetSent" , docs : "The message someone is trying to work with (i.e. increase fee) is not yet sent." }) , (44u8 , 11u8) => Some (ErrorDetails { pallet : "BridgeWococoMessages" , error : "TryingToConfirmMoreMessagesThanExpected" , docs : "The number of actually confirmed messages is going to be larger than the number of\nmessages in the proof. This may mean that this or bridged chain storage is corrupted." }) , (80u8 , 0u8) => Some (ErrorDetails { pallet : "Collective" , error : "NotMember" , docs : "Account is not a member" }) , (80u8 , 1u8) => Some (ErrorDetails { pallet : "Collective" , error : "DuplicateProposal" , docs : "Duplicate proposals not allowed" }) , (80u8 , 2u8) => Some (ErrorDetails { pallet : "Collective" , error : "ProposalMissing" , docs : "Proposal must exist" }) , (80u8 , 3u8) => Some (ErrorDetails { pallet : "Collective" , error : "WrongIndex" , docs : "Mismatched index" }) , (80u8 , 4u8) => Some (ErrorDetails { pallet : "Collective" , error : "DuplicateVote" , docs : "Duplicate vote ignored" }) , (80u8 , 5u8) => Some (ErrorDetails { pallet : "Collective" , error : "AlreadyInitialized" , docs : "Members are already initialized!" }) , (80u8 , 6u8) => Some (ErrorDetails { pallet : "Collective" , error : "TooEarly" , docs : "The close call was made too early, before the end of the voting." }) , (80u8 , 7u8) => Some (ErrorDetails { pallet : "Collective" , error : "TooManyProposals" , docs : "There can only be a maximum of `MaxProposals` active proposals." }) , (80u8 , 8u8) => Some (ErrorDetails { pallet : "Collective" , error : "WrongProposalWeight" , docs : "The given weight bound for the proposal was too low." }) , (80u8 , 9u8) => Some (ErrorDetails { pallet : "Collective" , error : "WrongProposalLength" , docs : "The given length bound for the proposal was too low." }) , (81u8 , 0u8) => Some (ErrorDetails { pallet : "Membership" , error : "AlreadyMember" , docs : "Already a member." }) , (81u8 , 1u8) => Some (ErrorDetails { pallet : "Membership" , error : "NotMember" , docs : "Not a member." }) , (90u8 , 0u8) => Some (ErrorDetails { pallet : "Utility" , error : "TooManyCalls" , docs : "Too many calls batched." }) , (91u8 , 0u8) => Some (ErrorDetails { pallet : "Proxy" , error : "TooMany" , docs : "There are too many proxies registered or too many announcements pending." }) , (91u8 , 1u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NotFound" , docs : "Proxy registration not found." }) , (91u8 , 2u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NotProxy" , docs : "Sender is not a proxy of the account to be proxied." }) , (91u8 , 3u8) => Some (ErrorDetails { pallet : "Proxy" , error : "Unproxyable" , docs : "A call which is incompatible with the proxy type's filter was attempted." }) , (91u8 , 4u8) => Some (ErrorDetails { pallet : "Proxy" , error : "Duplicate" , docs : "Account is already a proxy." }) , (91u8 , 5u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NoPermission" , docs : "Call may not be made by proxy because it may escalate its privileges." }) , (91u8 , 6u8) => Some (ErrorDetails { pallet : "Proxy" , error : "Unannounced" , docs : "Announcement, if made at all, was made too recently." }) , (91u8 , 7u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NoSelfProxy" , docs : "Cannot add self as proxy." }) , (92u8 , 0u8) => Some (ErrorDetails { pallet : "Multisig" , error : "MinimumThreshold" , docs : "Threshold must be 2 or greater." }) , (92u8 , 1u8) => Some (ErrorDetails { pallet : "Multisig" , error : "AlreadyApproved" , docs : "Call is already approved by this signatory." }) , (92u8 , 2u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NoApprovalsNeeded" , docs : "Call doesn't need any (more) approvals." }) , (92u8 , 3u8) => Some (ErrorDetails { pallet : "Multisig" , error : "TooFewSignatories" , docs : "There are too few signatories in the list." }) , (92u8 , 4u8) => Some (ErrorDetails { pallet : "Multisig" , error : "TooManySignatories" , docs : "There are too many signatories in the list." }) , (92u8 , 5u8) => Some (ErrorDetails { pallet : "Multisig" , error : "SignatoriesOutOfOrder" , docs : "The signatories were provided out of order; they should be ordered." }) , (92u8 , 6u8) => Some (ErrorDetails { pallet : "Multisig" , error : "SenderInSignatories" , docs : "The sender was contained in the other signatories; it shouldn't be." }) , (92u8 , 7u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NotFound" , docs : "Multisig operation not found when attempting to cancel." }) , (92u8 , 8u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NotOwner" , docs : "Only the account that originally created the multisig is able to cancel it." }) , (92u8 , 9u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NoTimepoint" , docs : "No timepoint was given, yet the multisig operation is already underway." }) , (92u8 , 10u8) => Some (ErrorDetails { pallet : "Multisig" , error : "WrongTimepoint" , docs : "A different timepoint was given to the multisig operation that is underway." }) , (92u8 , 11u8) => Some (ErrorDetails { pallet : "Multisig" , error : "UnexpectedTimepoint" , docs : "A timepoint was given, yet no multisig operation is underway." }) , (92u8 , 12u8) => Some (ErrorDetails { pallet : "Multisig" , error : "MaxWeightTooLow" , docs : "The maximum weight information provided was too low." }) , (92u8 , 13u8) => Some (ErrorDetails { pallet : "Multisig" , error : "AlreadyStored" , docs : "The data to be stored is already stored." }) , (99u8 , 0u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "Unreachable" , docs : "The desired destination was unreachable, generally because there is a no way of routing\nto it." }) , (99u8 , 1u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "SendFailure" , docs : "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps\na lack of space for buffering the message." }) , (99u8 , 2u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "Filtered" , docs : "The message execution fails the filter." }) , (99u8 , 3u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "UnweighableMessage" , docs : "The message's weight could not be determined." }) , (99u8 , 4u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "DestinationNotInvertible" , docs : "The destination `MultiLocation` provided cannot be inverted." }) , (99u8 , 5u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "Empty" , docs : "The assets to be sent are empty." }) , (99u8 , 6u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "CannotReanchor" , docs : "Could not re-anchor the assets to declare the fees for the destination chain." }) , (99u8 , 7u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "TooManyAssets" , docs : "Too many assets have been attempted for transfer." }) , (99u8 , 8u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "InvalidOrigin" , docs : "Origin is invalid for sending." }) , (99u8 , 9u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "BadVersion" , docs : "The version of the `Versioned` value used is not able to be interpreted." }) , (99u8 , 10u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "BadLocation" , docs : "The given location could not be used (e.g. because it cannot be expressed in the\ndesired version of XCM)." }) , (99u8 , 11u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "NoSubscription" , docs : "The referenced subscription could not be found." }) , (99u8 , 12u8) => Some (ErrorDetails { pallet : "XcmPallet" , error : "AlreadySubscribed" , docs : "The location is invalid since it already has a subscription from us." }) , _ => None }
            } else {
                None
            }
        }
    }
    #[doc = r" The default storage entry from which to fetch an account nonce, required for"]
    #[doc = r" constructing a transaction."]
    pub enum DefaultAccountData {}
    impl ::subxt::AccountData for DefaultAccountData {
        type StorageEntry = self::system::storage::Account;
        type AccountId = ::subxt::sp_core::crypto::AccountId32;
        type Index = ::core::primitive::u32;
        fn nonce(result: &<Self::StorageEntry as ::subxt::StorageEntry>::Value) -> Self::Index {
            result.nonce
        }
        fn storage_entry(account_id: Self::AccountId) -> Self::StorageEntry {
            self::system::storage::Account(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X, A = DefaultAccountData> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<T, X, A> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X, A> RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn constants(&'a self) -> ConstantsApi {
            ConstantsApi
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X, A> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn babe(&self) -> babe::constants::ConstantsApi {
            babe::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn indices(&self) -> indices::constants::ConstantsApi {
            indices::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi {
            authorship::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi {
            im_online::constants::ConstantsApi
        }
        pub fn paras(&self) -> paras::constants::ConstantsApi {
            paras::constants::ConstantsApi
        }
        pub fn registrar(&self) -> registrar::constants::ConstantsApi {
            registrar::constants::ConstantsApi
        }
        pub fn auctions(&self) -> auctions::constants::ConstantsApi {
            auctions::constants::ConstantsApi
        }
        pub fn crowdloan(&self) -> crowdloan::constants::ConstantsApi {
            crowdloan::constants::ConstantsApi
        }
        pub fn slots(&self) -> slots::constants::ConstantsApi {
            slots::constants::ConstantsApi
        }
        pub fn assigned_slots(&self) -> assigned_slots::constants::ConstantsApi {
            assigned_slots::constants::ConstantsApi
        }
        pub fn bridge_rococo_grandpa(&self) -> bridge_rococo_grandpa::constants::ConstantsApi {
            bridge_rococo_grandpa::constants::ConstantsApi
        }
        pub fn bridge_wococo_grandpa(&self) -> bridge_wococo_grandpa::constants::ConstantsApi {
            bridge_wococo_grandpa::constants::ConstantsApi
        }
        pub fn bridge_rococo_messages(&self) -> bridge_rococo_messages::constants::ConstantsApi {
            bridge_rococo_messages::constants::ConstantsApi
        }
        pub fn bridge_wococo_messages(&self) -> bridge_wococo_messages::constants::ConstantsApi {
            bridge_wococo_messages::constants::ConstantsApi
        }
        pub fn utility(&self) -> utility::constants::ConstantsApi {
            utility::constants::ConstantsApi
        }
        pub fn proxy(&self) -> proxy::constants::ConstantsApi {
            proxy::constants::ConstantsApi
        }
        pub fn multisig(&self) -> multisig::constants::ConstantsApi {
            multisig::constants::ConstantsApi
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn indices(&self) -> indices::storage::StorageApi<'a, T> {
            indices::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn offences(&self) -> offences::storage::StorageApi<'a, T> {
            offences::storage::StorageApi::new(self.client)
        }
        pub fn historical(&self) -> historical::storage::StorageApi<'a, T> {
            historical::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn authority_discovery(&self) -> authority_discovery::storage::StorageApi<'a, T> {
            authority_discovery::storage::StorageApi::new(self.client)
        }
        pub fn configuration(&self) -> configuration::storage::StorageApi<'a, T> {
            configuration::storage::StorageApi::new(self.client)
        }
        pub fn paras_shared(&self) -> paras_shared::storage::StorageApi<'a, T> {
            paras_shared::storage::StorageApi::new(self.client)
        }
        pub fn para_inclusion(&self) -> para_inclusion::storage::StorageApi<'a, T> {
            para_inclusion::storage::StorageApi::new(self.client)
        }
        pub fn para_inherent(&self) -> para_inherent::storage::StorageApi<'a, T> {
            para_inherent::storage::StorageApi::new(self.client)
        }
        pub fn para_scheduler(&self) -> para_scheduler::storage::StorageApi<'a, T> {
            para_scheduler::storage::StorageApi::new(self.client)
        }
        pub fn paras(&self) -> paras::storage::StorageApi<'a, T> {
            paras::storage::StorageApi::new(self.client)
        }
        pub fn initializer(&self) -> initializer::storage::StorageApi<'a, T> {
            initializer::storage::StorageApi::new(self.client)
        }
        pub fn dmp(&self) -> dmp::storage::StorageApi<'a, T> {
            dmp::storage::StorageApi::new(self.client)
        }
        pub fn ump(&self) -> ump::storage::StorageApi<'a, T> {
            ump::storage::StorageApi::new(self.client)
        }
        pub fn hrmp(&self) -> hrmp::storage::StorageApi<'a, T> {
            hrmp::storage::StorageApi::new(self.client)
        }
        pub fn para_session_info(&self) -> para_session_info::storage::StorageApi<'a, T> {
            para_session_info::storage::StorageApi::new(self.client)
        }
        pub fn paras_disputes(&self) -> paras_disputes::storage::StorageApi<'a, T> {
            paras_disputes::storage::StorageApi::new(self.client)
        }
        pub fn registrar(&self) -> registrar::storage::StorageApi<'a, T> {
            registrar::storage::StorageApi::new(self.client)
        }
        pub fn auctions(&self) -> auctions::storage::StorageApi<'a, T> {
            auctions::storage::StorageApi::new(self.client)
        }
        pub fn crowdloan(&self) -> crowdloan::storage::StorageApi<'a, T> {
            crowdloan::storage::StorageApi::new(self.client)
        }
        pub fn slots(&self) -> slots::storage::StorageApi<'a, T> {
            slots::storage::StorageApi::new(self.client)
        }
        pub fn assigned_slots(&self) -> assigned_slots::storage::StorageApi<'a, T> {
            assigned_slots::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn mmr(&self) -> mmr::storage::StorageApi<'a, T> {
            mmr::storage::StorageApi::new(self.client)
        }
        pub fn beefy(&self) -> beefy::storage::StorageApi<'a, T> {
            beefy::storage::StorageApi::new(self.client)
        }
        pub fn mmr_leaf(&self) -> mmr_leaf::storage::StorageApi<'a, T> {
            mmr_leaf::storage::StorageApi::new(self.client)
        }
        pub fn validator_manager(&self) -> validator_manager::storage::StorageApi<'a, T> {
            validator_manager::storage::StorageApi::new(self.client)
        }
        pub fn bridge_rococo_grandpa(&self) -> bridge_rococo_grandpa::storage::StorageApi<'a, T> {
            bridge_rococo_grandpa::storage::StorageApi::new(self.client)
        }
        pub fn bridge_wococo_grandpa(&self) -> bridge_wococo_grandpa::storage::StorageApi<'a, T> {
            bridge_wococo_grandpa::storage::StorageApi::new(self.client)
        }
        pub fn bridge_rococo_messages(&self) -> bridge_rococo_messages::storage::StorageApi<'a, T> {
            bridge_rococo_messages::storage::StorageApi::new(self.client)
        }
        pub fn bridge_wococo_messages(&self) -> bridge_wococo_messages::storage::StorageApi<'a, T> {
            bridge_wococo_messages::storage::StorageApi::new(self.client)
        }
        pub fn collective(&self) -> collective::storage::StorageApi<'a, T> {
            collective::storage::StorageApi::new(self.client)
        }
        pub fn membership(&self) -> membership::storage::StorageApi<'a, T> {
            membership::storage::StorageApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::storage::StorageApi<'a, T> {
            proxy::storage::StorageApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
            multisig::storage::StorageApi::new(self.client)
        }
        pub fn xcm_pallet(&self) -> xcm_pallet::storage::StorageApi<'a, T> {
            xcm_pallet::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<'a, T, X, A> TransactionApi<'a, T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X, A> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T, X, A> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X, A> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn indices(&self) -> indices::calls::TransactionApi<'a, T, X, A> {
            indices::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X, A> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T, X, A> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T, X, A> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X, A> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T, X, A> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn configuration(&self) -> configuration::calls::TransactionApi<'a, T, X, A> {
            configuration::calls::TransactionApi::new(self.client)
        }
        pub fn paras_shared(&self) -> paras_shared::calls::TransactionApi<'a, T, X, A> {
            paras_shared::calls::TransactionApi::new(self.client)
        }
        pub fn para_inclusion(&self) -> para_inclusion::calls::TransactionApi<'a, T, X, A> {
            para_inclusion::calls::TransactionApi::new(self.client)
        }
        pub fn para_inherent(&self) -> para_inherent::calls::TransactionApi<'a, T, X, A> {
            para_inherent::calls::TransactionApi::new(self.client)
        }
        pub fn paras(&self) -> paras::calls::TransactionApi<'a, T, X, A> {
            paras::calls::TransactionApi::new(self.client)
        }
        pub fn initializer(&self) -> initializer::calls::TransactionApi<'a, T, X, A> {
            initializer::calls::TransactionApi::new(self.client)
        }
        pub fn dmp(&self) -> dmp::calls::TransactionApi<'a, T, X, A> {
            dmp::calls::TransactionApi::new(self.client)
        }
        pub fn ump(&self) -> ump::calls::TransactionApi<'a, T, X, A> {
            ump::calls::TransactionApi::new(self.client)
        }
        pub fn hrmp(&self) -> hrmp::calls::TransactionApi<'a, T, X, A> {
            hrmp::calls::TransactionApi::new(self.client)
        }
        pub fn paras_disputes(&self) -> paras_disputes::calls::TransactionApi<'a, T, X, A> {
            paras_disputes::calls::TransactionApi::new(self.client)
        }
        pub fn registrar(&self) -> registrar::calls::TransactionApi<'a, T, X, A> {
            registrar::calls::TransactionApi::new(self.client)
        }
        pub fn auctions(&self) -> auctions::calls::TransactionApi<'a, T, X, A> {
            auctions::calls::TransactionApi::new(self.client)
        }
        pub fn crowdloan(&self) -> crowdloan::calls::TransactionApi<'a, T, X, A> {
            crowdloan::calls::TransactionApi::new(self.client)
        }
        pub fn slots(&self) -> slots::calls::TransactionApi<'a, T, X, A> {
            slots::calls::TransactionApi::new(self.client)
        }
        pub fn paras_sudo_wrapper(&self) -> paras_sudo_wrapper::calls::TransactionApi<'a, T, X, A> {
            paras_sudo_wrapper::calls::TransactionApi::new(self.client)
        }
        pub fn assigned_slots(&self) -> assigned_slots::calls::TransactionApi<'a, T, X, A> {
            assigned_slots::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X, A> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn beefy(&self) -> beefy::calls::TransactionApi<'a, T, X, A> {
            beefy::calls::TransactionApi::new(self.client)
        }
        pub fn validator_manager(&self) -> validator_manager::calls::TransactionApi<'a, T, X, A> {
            validator_manager::calls::TransactionApi::new(self.client)
        }
        pub fn bridge_rococo_grandpa(
            &self,
        ) -> bridge_rococo_grandpa::calls::TransactionApi<'a, T, X, A> {
            bridge_rococo_grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn bridge_wococo_grandpa(
            &self,
        ) -> bridge_wococo_grandpa::calls::TransactionApi<'a, T, X, A> {
            bridge_wococo_grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn bridge_rococo_messages(
            &self,
        ) -> bridge_rococo_messages::calls::TransactionApi<'a, T, X, A> {
            bridge_rococo_messages::calls::TransactionApi::new(self.client)
        }
        pub fn bridge_wococo_messages(
            &self,
        ) -> bridge_wococo_messages::calls::TransactionApi<'a, T, X, A> {
            bridge_wococo_messages::calls::TransactionApi::new(self.client)
        }
        pub fn collective(&self) -> collective::calls::TransactionApi<'a, T, X, A> {
            collective::calls::TransactionApi::new(self.client)
        }
        pub fn membership(&self) -> membership::calls::TransactionApi<'a, T, X, A> {
            membership::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T, X, A> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::calls::TransactionApi<'a, T, X, A> {
            proxy::calls::TransactionApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T, X, A> {
            multisig::calls::TransactionApi::new(self.client)
        }
        pub fn xcm_pallet(&self) -> xcm_pallet::calls::TransactionApi<'a, T, X, A> {
            xcm_pallet::calls::TransactionApi::new(self.client)
        }
    }
}
