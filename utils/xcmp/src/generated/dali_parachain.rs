#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
	use super::api as root_mod;
	pub static PALLETS: [&str; 26usize] = [
		"System",
		"Timestamp",
		"Sudo",
		"RandomnessCollectiveFlip",
		"TransactionPayment",
		"Indices",
		"Balances",
		"ParachainSystem",
		"ParachainInfo",
		"Authorship",
		"CollatorSelection",
		"Session",
		"Aura",
		"AuraExt",
		"Council",
		"CouncilMembership",
		"Treasury",
		"Democracy",
		"Scheduler",
		"Utility",
		"XcmpQueue",
		"RelayerXcm",
		"CumulusXcm",
		"DmpQueue",
		"LiquidCrowdloan",
		"Tokens",
	];
	#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
		RelayerXcm(relayer_xcm::Event),
		#[codec(index = 42)]
		CumulusXcm(cumulus_xcm::Event),
		#[codec(index = 43)]
		DmpQueue(dmp_queue::Event),
		#[codec(index = 50)]
		LiquidCrowdloan(liquid_crowdloan::Event),
		#[codec(index = 52)]
		Tokens(tokens::Event),
	}
	pub mod system {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct FillBlock {
				pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			impl ::subxt::Call for FillBlock {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "fill_block";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Remark {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for Remark {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "remark";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct SetHeapPages {
				pub pages: ::core::primitive::u64,
			}
			impl ::subxt::Call for SetHeapPages {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_heap_pages";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SetCode {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SetCode {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_code";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SetCodeWithoutChecks {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SetCodeWithoutChecks {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "set_code_without_checks";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct KillStorage {
				pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
			}
			impl ::subxt::Call for KillStorage {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "kill_storage";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct KillPrefix {
				pub prefix: ::std::vec::Vec<::core::primitive::u8>,
				pub subkeys: ::core::primitive::u32,
			}
			impl ::subxt::Call for KillPrefix {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "kill_prefix";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct RemarkWithEvent {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for RemarkWithEvent {
				const PALLET: &'static str = "System";
				const FUNCTION: &'static str = "remark_with_event";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "A dispatch that will fill the block weight up to the given ratio."]
				pub fn fill_block(
					&self,
					ratio: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						FillBlock,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<FillBlock>()? ==
						[
							228u8, 117u8, 251u8, 95u8, 47u8, 56u8, 32u8, 177u8, 191u8, 72u8, 75u8,
							23u8, 193u8, 175u8, 227u8, 218u8, 127u8, 94u8, 114u8, 110u8, 215u8,
							61u8, 162u8, 102u8, 73u8, 89u8, 218u8, 148u8, 59u8, 73u8, 59u8, 149u8,
						] {
						let call = FillBlock { ratio };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`"]
				#[doc = "# </weight>"]
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Remark, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Remark>()? ==
						[
							186u8, 79u8, 33u8, 199u8, 216u8, 115u8, 19u8, 146u8, 220u8, 174u8,
							98u8, 61u8, 179u8, 230u8, 40u8, 70u8, 22u8, 251u8, 77u8, 62u8, 133u8,
							80u8, 186u8, 70u8, 135u8, 172u8, 178u8, 241u8, 69u8, 106u8, 235u8,
							140u8,
						] {
						let call = Remark { remark };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetHeapPages,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetHeapPages>()? ==
						[
							77u8, 138u8, 122u8, 55u8, 179u8, 101u8, 60u8, 137u8, 173u8, 39u8, 28u8,
							36u8, 237u8, 243u8, 232u8, 162u8, 76u8, 176u8, 135u8, 58u8, 60u8,
							177u8, 105u8, 136u8, 94u8, 53u8, 26u8, 31u8, 41u8, 156u8, 228u8, 241u8,
						] {
						let call = SetHeapPages { pages };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set the new runtime code."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
				#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
				#[doc = "  expensive)."]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
				#[doc = "expensive. We will treat this as a full block."]
				#[doc = "# </weight>"]
				pub fn set_code(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetCode,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetCode>()? ==
						[
							35u8, 75u8, 103u8, 203u8, 91u8, 141u8, 77u8, 95u8, 37u8, 157u8, 107u8,
							240u8, 54u8, 242u8, 245u8, 205u8, 104u8, 165u8, 177u8, 37u8, 86u8,
							197u8, 28u8, 202u8, 121u8, 159u8, 18u8, 204u8, 237u8, 117u8, 141u8,
							131u8,
						] {
						let call = SetCode { code };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set the new runtime code without doing any checks of the given `code`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(C)` where `C` length of `code`"]
				#[doc = "- 1 storage write (codec `O(C)`)."]
				#[doc = "- 1 digest item."]
				#[doc = "- 1 event."]
				#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
				#[doc = "block. # </weight>"]
				pub fn set_code_without_checks(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetCodeWithoutChecks,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetCodeWithoutChecks>()? ==
						[
							150u8, 148u8, 119u8, 129u8, 77u8, 216u8, 135u8, 187u8, 127u8, 24u8,
							238u8, 15u8, 227u8, 229u8, 191u8, 217u8, 106u8, 129u8, 149u8, 79u8,
							154u8, 78u8, 53u8, 159u8, 89u8, 69u8, 103u8, 197u8, 93u8, 161u8, 134u8,
							17u8,
						] {
						let call = SetCodeWithoutChecks { code };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set some items of storage."]
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetStorage,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetStorage>()? ==
						[
							197u8, 12u8, 119u8, 205u8, 152u8, 103u8, 211u8, 170u8, 146u8, 253u8,
							25u8, 56u8, 180u8, 146u8, 74u8, 75u8, 38u8, 108u8, 212u8, 154u8, 23u8,
							22u8, 148u8, 175u8, 107u8, 186u8, 222u8, 13u8, 149u8, 132u8, 204u8,
							217u8,
						] {
						let call = SetStorage { items };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Kill some items from storage."]
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						KillStorage,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<KillStorage>()? ==
						[
							154u8, 115u8, 185u8, 20u8, 126u8, 90u8, 222u8, 131u8, 199u8, 57u8,
							184u8, 226u8, 43u8, 245u8, 161u8, 176u8, 194u8, 123u8, 139u8, 97u8,
							97u8, 94u8, 47u8, 64u8, 204u8, 96u8, 190u8, 94u8, 216u8, 237u8, 69u8,
							51u8,
						] {
						let call = KillStorage { keys };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						KillPrefix,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<KillPrefix>()? ==
						[
							214u8, 101u8, 191u8, 241u8, 1u8, 241u8, 144u8, 116u8, 246u8, 199u8,
							159u8, 249u8, 155u8, 164u8, 220u8, 221u8, 75u8, 33u8, 204u8, 3u8,
							255u8, 201u8, 187u8, 238u8, 181u8, 213u8, 41u8, 105u8, 234u8, 120u8,
							202u8, 115u8,
						] {
						let call = KillPrefix { prefix, subkeys };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Make some on-chain remark and emit event."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(b)` where b is the length of the remark."]
				#[doc = "- 1 event."]
				#[doc = "# </weight>"]
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						RemarkWithEvent,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<RemarkWithEvent>()? ==
						[
							171u8, 82u8, 75u8, 237u8, 69u8, 197u8, 223u8, 125u8, 123u8, 51u8,
							241u8, 35u8, 202u8, 210u8, 227u8, 109u8, 1u8, 241u8, 255u8, 63u8, 33u8,
							115u8, 156u8, 239u8, 97u8, 76u8, 193u8, 35u8, 74u8, 199u8, 43u8, 255u8,
						] {
						let call = RemarkWithEvent { remark };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An extrinsic completed successfully. \\[info\\]"]
			pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
			impl ::subxt::Event for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An extrinsic failed. \\[error, info\\]"]
			pub struct ExtrinsicFailed(
				pub runtime_types::sp_runtime::DispatchError,
				pub runtime_types::frame_support::weights::DispatchInfo,
			);
			impl ::subxt::Event for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::Event for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A new \\[account\\] was created."]
			pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An \\[account\\] was reaped."]
			pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::Event for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "On on-chain remark happened. \\[origin, remark_hash\\]"]
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
			pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Account<'_> {
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
			pub struct BlockHash<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for BlockHash<'_> {
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
			pub struct ExtrinsicData<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for ExtrinsicData<'_> {
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
						runtime_types::composable_runtime::Event,
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
			pub struct EventTopics<'a>(pub &'a ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for EventTopics<'_> {
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
				#[doc = " The full account information for a particular account ID."]
				pub async fn account(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_system::AccountInfo<
						::core::primitive::u32,
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Account>()? ==
						[
							224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8, 130u8,
							23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8, 88u8, 71u8,
							168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8, 195u8, 46u8,
						] {
						let entry = Account(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The full account information for a particular account ID."]
				pub async fn account_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Account>()? ==
						[
							224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8, 130u8,
							23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8, 88u8, 71u8,
							168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8, 195u8, 46u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Total extrinsics count for the current block."]
				pub async fn extrinsic_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ExtrinsicCount>()? ==
						[
							223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
							222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
							41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
						] {
						let entry = ExtrinsicCount;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The current weight for the block."]
				pub async fn block_weight(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<BlockWeight>()? ==
						[
							2u8, 236u8, 190u8, 174u8, 244u8, 98u8, 194u8, 168u8, 89u8, 208u8, 7u8,
							45u8, 175u8, 171u8, 177u8, 121u8, 215u8, 190u8, 184u8, 195u8, 49u8,
							133u8, 44u8, 1u8, 181u8, 215u8, 89u8, 84u8, 255u8, 16u8, 57u8, 152u8,
						] {
						let entry = BlockWeight;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub async fn all_extrinsics_len(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<AllExtrinsicsLen>()? ==
						[
							202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
							254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
							219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
							134u8, 60u8,
						] {
						let entry = AllExtrinsicsLen;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Map of block numbers to block hashes."]
				pub async fn block_hash(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<BlockHash>()? ==
						[
							24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8, 24u8,
							236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8, 111u8,
							141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8, 246u8,
							120u8,
						] {
						let entry = BlockHash(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Map of block numbers to block hashes."]
				pub async fn block_hash_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, BlockHash<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<BlockHash>()? ==
						[
							24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8, 24u8,
							236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8, 111u8,
							141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8, 246u8,
							120u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub async fn extrinsic_data(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ExtrinsicData>()? ==
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						] {
						let entry = ExtrinsicData(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub async fn extrinsic_data_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, ExtrinsicData<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ExtrinsicData>()? ==
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub async fn number(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<Number>()? ==
						[
							228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
							246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
							36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
							164u8, 191u8,
						] {
						let entry = Number;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Hash of the previous block."]
				pub async fn parent_hash(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<ParentHash>()? ==
						[
							194u8, 221u8, 147u8, 22u8, 68u8, 141u8, 32u8, 6u8, 202u8, 39u8, 164u8,
							184u8, 69u8, 126u8, 190u8, 101u8, 215u8, 27u8, 127u8, 157u8, 200u8,
							69u8, 170u8, 139u8, 232u8, 27u8, 254u8, 181u8, 183u8, 105u8, 111u8,
							177u8,
						] {
						let entry = ParentHash;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Digest of the current block, also part of the block header."]
				pub async fn digest(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_runtime::generic::digest::Digest,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Digest>()? ==
						[
							10u8, 176u8, 13u8, 228u8, 226u8, 42u8, 210u8, 151u8, 107u8, 212u8,
							136u8, 15u8, 38u8, 182u8, 225u8, 12u8, 250u8, 56u8, 193u8, 243u8,
							219u8, 113u8, 95u8, 233u8, 21u8, 229u8, 125u8, 146u8, 92u8, 250u8,
							32u8, 168u8,
						] {
						let entry = Digest;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: This storage item is explicitly unbounded since it is never intended to be read"]
				#[doc = " from within the runtime."]
				pub async fn events(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::frame_system::EventRecord<
							runtime_types::composable_runtime::Event,
							::subxt::sp_core::H256,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Events>()? ==
						[
							103u8, 52u8, 86u8, 92u8, 190u8, 85u8, 150u8, 239u8, 32u8, 63u8, 146u8,
							35u8, 83u8, 98u8, 201u8, 244u8, 215u8, 133u8, 155u8, 238u8, 49u8, 75u8,
							113u8, 209u8, 7u8, 222u8, 57u8, 78u8, 29u8, 84u8, 255u8, 197u8,
						] {
						let entry = Events;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub async fn event_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<EventCount>()? ==
						[
							236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
							203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
							161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
							112u8,
						] {
						let entry = EventCount;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub async fn event_topics(
					&self,
					_0: &::subxt::sp_core::H256,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<EventTopics>()? ==
						[
							231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
							140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8, 93u8,
							178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8, 245u8, 179u8,
						] {
						let entry = EventTopics(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
				#[doc = " of events in the `<Events<T>>` list."]
				#[doc = ""]
				#[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
				#[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
				#[doc = " in case of changes fetch the list of events of interest."]
				#[doc = ""]
				#[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
				#[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
				#[doc = " no notification will be triggered thus the event might be lost."]
				pub async fn event_topics_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, EventTopics<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<EventTopics>()? ==
						[
							231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
							140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8, 93u8,
							178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8, 245u8, 179u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub async fn last_runtime_upgrade(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<LastRuntimeUpgrade>()? ==
						[
							219u8, 153u8, 158u8, 38u8, 45u8, 65u8, 151u8, 137u8, 53u8, 76u8, 11u8,
							181u8, 218u8, 248u8, 125u8, 190u8, 100u8, 240u8, 173u8, 75u8, 179u8,
							137u8, 198u8, 197u8, 248u8, 185u8, 118u8, 58u8, 42u8, 165u8, 125u8,
							119u8,
						] {
						let entry = LastRuntimeUpgrade;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub async fn upgraded_to_u32_ref_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<UpgradedToU32RefCount>()? ==
						[
							171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
							178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
							83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
						] {
						let entry = UpgradedToU32RefCount;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub async fn upgraded_to_triple_ref_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<UpgradedToTripleRefCount>()? ==
						[
							90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
							210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
							155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
							185u8,
						] {
						let entry = UpgradedToTripleRefCount;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The execution phase of the block."]
				pub async fn execution_phase(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::frame_system::Phase>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ExecutionPhase>()? ==
						[
							174u8, 13u8, 230u8, 220u8, 239u8, 161u8, 172u8, 122u8, 188u8, 95u8,
							141u8, 118u8, 91u8, 158u8, 111u8, 145u8, 243u8, 173u8, 226u8, 212u8,
							187u8, 118u8, 94u8, 132u8, 221u8, 244u8, 61u8, 148u8, 217u8, 30u8,
							238u8, 225u8,
						] {
						let entry = ExecutionPhase;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::core::result::Result<
					runtime_types::frame_system::limits::BlockWeights,
					::subxt::BasicError,
				> {
					if self.client.metadata().constant_hash("System", "BlockWeights")? ==
						[
							171u8, 219u8, 233u8, 26u8, 8u8, 82u8, 126u8, 26u8, 45u8, 242u8, 95u8,
							241u8, 173u8, 95u8, 182u8, 49u8, 162u8, 240u8, 151u8, 9u8, 49u8, 197u8,
							203u8, 181u8, 118u8, 90u8, 209u8, 38u8, 23u8, 22u8, 164u8, 1u8,
						] {
						let pallet = self.client.metadata().pallet("System")?;
						let constant = pallet.constant("BlockWeights")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::core::result::Result<
					runtime_types::frame_system::limits::BlockLength,
					::subxt::BasicError,
				> {
					if self.client.metadata().constant_hash("System", "BlockLength")? ==
						[
							120u8, 249u8, 182u8, 103u8, 246u8, 214u8, 149u8, 44u8, 42u8, 64u8, 2u8,
							56u8, 157u8, 184u8, 43u8, 195u8, 214u8, 251u8, 207u8, 207u8, 249u8,
							105u8, 203u8, 108u8, 179u8, 93u8, 93u8, 246u8, 40u8, 175u8, 160u8,
							114u8,
						] {
						let pallet = self.client.metadata().pallet("System")?;
						let constant = pallet.constant("BlockLength")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("System", "BlockHashCount")? ==
						[
							26u8, 201u8, 14u8, 127u8, 151u8, 212u8, 14u8, 28u8, 184u8, 180u8, 96u8,
							223u8, 210u8, 69u8, 176u8, 187u8, 183u8, 124u8, 4u8, 13u8, 0u8, 241u8,
							151u8, 202u8, 41u8, 152u8, 230u8, 247u8, 138u8, 23u8, 132u8, 49u8,
						] {
						let pallet = self.client.metadata().pallet("System")?;
						let constant = pallet.constant("BlockHashCount")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::core::result::Result<
					runtime_types::frame_support::weights::RuntimeDbWeight,
					::subxt::BasicError,
				> {
					if self.client.metadata().constant_hash("System", "DbWeight")? ==
						[
							203u8, 8u8, 106u8, 152u8, 74u8, 132u8, 2u8, 132u8, 244u8, 106u8, 147u8,
							12u8, 93u8, 80u8, 61u8, 158u8, 172u8, 178u8, 228u8, 125u8, 213u8,
							102u8, 75u8, 210u8, 64u8, 185u8, 204u8, 84u8, 10u8, 164u8, 204u8, 62u8,
						] {
						let pallet = self.client.metadata().pallet("System")?;
						let constant = pallet.constant("DbWeight")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Get the chain's current version."]
				pub fn version(
					&self,
				) -> ::core::result::Result<
					runtime_types::sp_version::RuntimeVersion,
					::subxt::BasicError,
				> {
					if self.client.metadata().constant_hash("System", "Version")? ==
						[
							6u8, 250u8, 253u8, 235u8, 6u8, 80u8, 26u8, 105u8, 75u8, 8u8, 176u8,
							114u8, 116u8, 201u8, 196u8, 148u8, 160u8, 193u8, 227u8, 198u8, 210u8,
							62u8, 104u8, 125u8, 21u8, 234u8, 219u8, 25u8, 200u8, 95u8, 213u8,
							183u8,
						] {
						let pallet = self.client.metadata().pallet("System")?;
						let constant = pallet.constant("Version")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The designated SS85 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(
					&self,
				) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("System", "SS58Prefix")? ==
						[
							142u8, 135u8, 76u8, 135u8, 231u8, 49u8, 207u8, 182u8, 149u8, 160u8,
							180u8, 179u8, 7u8, 108u8, 105u8, 145u8, 33u8, 184u8, 191u8, 144u8,
							28u8, 87u8, 231u8, 186u8, 210u8, 233u8, 228u8, 60u8, 106u8, 19u8,
							137u8, 136u8,
						] {
						let pallet = self.client.metadata().pallet("System")?;
						let constant = pallet.constant("SS58Prefix")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod timestamp {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Set {
				#[codec(compact)]
				pub now: ::core::primitive::u64,
			}
			impl ::subxt::Call for Set {
				const PALLET: &'static str = "Timestamp";
				const FUNCTION: &'static str = "set";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Set the current time."]
				#[doc = ""]
				#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase, if this call hasn't been invoked by that time."]
				#[doc = ""]
				#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
				#[doc = "`MinimumPeriod`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
				#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
				#[doc = "  `on_finalize`)"]
				#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
				#[doc = "# </weight>"]
				pub fn set(
					&self,
					now: ::core::primitive::u64,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Set, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Set>()? ==
						[
							191u8, 73u8, 102u8, 150u8, 65u8, 157u8, 172u8, 194u8, 7u8, 72u8, 1u8,
							35u8, 54u8, 99u8, 245u8, 139u8, 40u8, 136u8, 245u8, 53u8, 167u8, 100u8,
							143u8, 244u8, 160u8, 5u8, 18u8, 130u8, 77u8, 160u8, 227u8, 51u8,
						] {
						let call = Set { now };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
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
				#[doc = " Current time for the current block."]
				pub async fn now(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<Now>()? ==
						[
							148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
							221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
							185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
						] {
						let entry = Now;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Did the timestamp get updated in this block?"]
				pub async fn did_update(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<DidUpdate>()? ==
						[
							70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
							13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
							27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
						] {
						let entry = DidUpdate;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
				#[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
				#[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
				#[doc = " double this period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Timestamp", "MinimumPeriod")? ==
						[
							224u8, 163u8, 2u8, 57u8, 22u8, 120u8, 36u8, 145u8, 190u8, 20u8, 13u8,
							182u8, 94u8, 250u8, 138u8, 166u8, 43u8, 184u8, 117u8, 174u8, 236u8,
							84u8, 149u8, 87u8, 176u8, 229u8, 213u8, 83u8, 187u8, 168u8, 16u8,
							210u8,
						] {
						let pallet = self.client.metadata().pallet("Timestamp")?;
						let constant = pallet.constant("MinimumPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod sudo {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Sudo {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for Sudo {
				const PALLET: &'static str = "Sudo";
				const FUNCTION: &'static str = "sudo";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SudoUncheckedWeight {
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
				pub weight: ::core::primitive::u64,
			}
			impl ::subxt::Call for SudoUncheckedWeight {
				const PALLET: &'static str = "Sudo";
				const FUNCTION: &'static str = "sudo_unchecked_weight";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SudoAs {
				pub who: ::subxt::sp_runtime::MultiAddress<
					::subxt::sp_core::crypto::AccountId32,
					::core::primitive::u32,
				>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for SudoAs {
				const PALLET: &'static str = "Sudo";
				const FUNCTION: &'static str = "sudo_as";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo(
					&self,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Sudo, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Sudo>()? ==
						[
							158u8, 233u8, 172u8, 118u8, 34u8, 27u8, 176u8, 80u8, 114u8, 60u8, 29u8,
							203u8, 27u8, 98u8, 80u8, 128u8, 174u8, 49u8, 92u8, 227u8, 37u8, 51u8,
							209u8, 59u8, 21u8, 231u8, 241u8, 161u8, 68u8, 121u8, 104u8, 149u8,
						] {
						let call = Sudo { call: ::std::boxed::Box::new(call) };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
				#[doc = "This function does not check the weight of the call, and instead allows the"]
				#[doc = "Sudo user to specify the weight of the call."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- The weight of this call is defined by the caller."]
				#[doc = "# </weight>"]
				pub fn sudo_unchecked_weight(
					&self,
					call: runtime_types::composable_runtime::Call,
					weight: ::core::primitive::u64,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SudoUncheckedWeight,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SudoUncheckedWeight>()? ==
						[
							87u8, 64u8, 129u8, 118u8, 201u8, 190u8, 125u8, 150u8, 205u8, 226u8,
							190u8, 169u8, 42u8, 155u8, 103u8, 252u8, 16u8, 2u8, 197u8, 144u8,
							246u8, 219u8, 172u8, 180u8, 101u8, 25u8, 221u8, 145u8, 147u8, 225u8,
							159u8, 46u8,
						] {
						let call =
							SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
				#[doc = "key."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB change."]
				#[doc = "# </weight>"]
				pub fn set_key(
					&self,
					new: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, SetKey, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetKey>()? ==
						[
							142u8, 228u8, 169u8, 153u8, 89u8, 247u8, 116u8, 76u8, 245u8, 199u8,
							2u8, 131u8, 195u8, 249u8, 201u8, 178u8, 212u8, 253u8, 144u8, 48u8,
							172u8, 25u8, 184u8, 72u8, 32u8, 150u8, 161u8, 116u8, 72u8, 162u8,
							236u8, 131u8,
						] {
						let call = SetKey { new };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
				#[doc = "a given account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + 10,000."]
				#[doc = "# </weight>"]
				pub fn sudo_as(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, SudoAs, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SudoAs>()? ==
						[
							27u8, 0u8, 105u8, 8u8, 208u8, 179u8, 63u8, 82u8, 124u8, 113u8, 17u8,
							87u8, 70u8, 227u8, 8u8, 120u8, 141u8, 165u8, 143u8, 55u8, 240u8, 126u8,
							33u8, 149u8, 29u8, 147u8, 10u8, 159u8, 195u8, 194u8, 216u8, 16u8,
						] {
						let call = SudoAs { who, call: ::std::boxed::Box::new(call) };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::Event for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied."]
			pub struct KeyChanged {
				pub new_sudoer: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Event for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A sudo just took place. \\[result\\]"]
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
				#[doc = " The `AccountId` of the sudo key."]
				pub async fn key(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::sp_core::crypto::AccountId32,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Key>()? ==
						[
							27u8, 145u8, 220u8, 130u8, 100u8, 142u8, 37u8, 251u8, 22u8, 117u8,
							53u8, 141u8, 203u8, 109u8, 197u8, 210u8, 164u8, 142u8, 250u8, 229u8,
							155u8, 177u8, 63u8, 167u8, 166u8, 197u8, 109u8, 42u8, 5u8, 117u8,
							156u8, 12u8,
						] {
						let entry = Key;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod randomness_collective_flip {
		use super::{root_mod, runtime_types};
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
				#[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
				#[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
				#[doc = " the oldest hash."]
				pub async fn random_material(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::H256>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<RandomMaterial>()? ==
						[
							40u8, 29u8, 216u8, 175u8, 194u8, 61u8, 254u8, 46u8, 29u8, 39u8, 76u8,
							190u8, 110u8, 149u8, 60u8, 177u8, 79u8, 88u8, 248u8, 199u8, 208u8,
							246u8, 37u8, 166u8, 84u8, 58u8, 120u8, 6u8, 98u8, 83u8, 192u8, 36u8,
						] {
						let entry = RandomMaterial;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
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
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_arithmetic::fixed_point::FixedU128,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<NextFeeMultiplier>()? ==
						[
							232u8, 48u8, 68u8, 202u8, 209u8, 29u8, 249u8, 71u8, 0u8, 84u8, 229u8,
							250u8, 176u8, 203u8, 27u8, 26u8, 34u8, 55u8, 83u8, 183u8, 224u8, 40u8,
							62u8, 127u8, 131u8, 88u8, 128u8, 9u8, 56u8, 178u8, 31u8, 183u8,
						] {
						let entry = NextFeeMultiplier;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub async fn storage_version(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_transaction_payment::Releases,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<StorageVersion>()? ==
						[
							219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
							200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
							58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
						] {
						let entry = StorageVersion;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The fee to be paid for making a transaction; the per-byte portion."]
				pub fn transaction_byte_fee(
					&self,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self
						.client
						.metadata()
						.constant_hash("TransactionPayment", "TransactionByteFee")? ==
						[
							6u8, 197u8, 50u8, 211u8, 213u8, 44u8, 51u8, 100u8, 4u8, 33u8, 97u8,
							123u8, 151u8, 220u8, 230u8, 3u8, 4u8, 131u8, 29u8, 252u8, 233u8, 79u8,
							178u8, 252u8, 84u8, 189u8, 79u8, 97u8, 198u8, 76u8, 242u8, 154u8,
						] {
						let pallet = self.client.metadata().pallet("TransactionPayment")?;
						let constant = pallet.constant("TransactionByteFee")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
				#[doc = " `priority`"]
				#[doc = ""]
				#[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
				#[doc = " added to a tip component in regular `priority` calculations."]
				#[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
				#[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
				#[doc = ""]
				#[doc = " ```rust,ignore"]
				#[doc = " // For `Normal`"]
				#[doc = " let priority = priority_calc(tip);"]
				#[doc = ""]
				#[doc = " // For `Operational`"]
				#[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
				#[doc = " let priority = priority_calc(tip + virtual_tip);"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
				#[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
				#[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
				#[doc = " transactions."]
				pub fn operational_fee_multiplier(
					&self,
				) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError> {
					if self
						.client
						.metadata()
						.constant_hash("TransactionPayment", "OperationalFeeMultiplier")? ==
						[
							161u8, 232u8, 150u8, 43u8, 106u8, 83u8, 56u8, 248u8, 54u8, 123u8,
							244u8, 73u8, 5u8, 49u8, 245u8, 150u8, 70u8, 92u8, 158u8, 207u8, 127u8,
							115u8, 211u8, 21u8, 24u8, 136u8, 89u8, 44u8, 151u8, 211u8, 235u8,
							196u8,
						] {
						let pallet = self.client.metadata().pallet("TransactionPayment")?;
						let constant = pallet.constant("OperationalFeeMultiplier")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The polynomial that is applied in order to derive fee from weight."]
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
					if self.client.metadata().constant_hash("TransactionPayment", "WeightToFee")? ==
						[
							149u8, 0u8, 74u8, 92u8, 102u8, 215u8, 163u8, 188u8, 115u8, 9u8, 121u8,
							173u8, 219u8, 107u8, 145u8, 36u8, 73u8, 167u8, 82u8, 8u8, 181u8, 2u8,
							219u8, 93u8, 147u8, 48u8, 54u8, 19u8, 96u8, 224u8, 65u8, 30u8,
						] {
						let pallet = self.client.metadata().pallet("TransactionPayment")?;
						let constant = pallet.constant("WeightToFee")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod indices {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct Claim {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Claim {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "claim";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Transfer {
				pub new: ::subxt::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Transfer {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "transfer";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct Free {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Free {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "free";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ForceTransfer {
				pub new: ::subxt::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
				pub freeze: ::core::primitive::bool,
			}
			impl ::subxt::Call for ForceTransfer {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "force_transfer";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct Freeze {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Freeze {
				const PALLET: &'static str = "Indices";
				const FUNCTION: &'static str = "freeze";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Assign an previously unassigned index."]
				#[doc = ""]
				#[doc = "Payment: `Deposit` is reserved from the sender account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be claimed. This must not be in use."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
				pub fn claim(
					&self,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Claim, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Claim>()? ==
						[
							27u8, 4u8, 108u8, 55u8, 23u8, 109u8, 175u8, 25u8, 201u8, 230u8, 228u8,
							51u8, 164u8, 15u8, 79u8, 10u8, 219u8, 182u8, 242u8, 102u8, 164u8,
							148u8, 39u8, 91u8, 106u8, 197u8, 29u8, 190u8, 178u8, 221u8, 16u8, 87u8,
						] {
						let call = Claim { index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
				#[doc = "is effectively transferred to the new account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
				#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One transfer operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
				#[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
				#[doc = "# </weight>"]
				pub fn transfer(
					&self,
					new: ::subxt::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Transfer,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Transfer>()? ==
						[
							124u8, 83u8, 33u8, 230u8, 23u8, 70u8, 83u8, 59u8, 76u8, 100u8, 219u8,
							100u8, 165u8, 163u8, 102u8, 193u8, 11u8, 22u8, 30u8, 125u8, 114u8,
							28u8, 61u8, 156u8, 38u8, 170u8, 129u8, 74u8, 187u8, 28u8, 33u8, 65u8,
						] {
						let call = Transfer { new, index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Free up an index owned by the sender."]
				#[doc = ""]
				#[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
				#[doc = ""]
				#[doc = "- `index`: the index to be freed. This must be owned by the sender."]
				#[doc = ""]
				#[doc = "Emits `IndexFreed` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- One reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
				pub fn free(
					&self,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Free, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Free>()? ==
						[
							153u8, 143u8, 162u8, 33u8, 229u8, 3u8, 159u8, 153u8, 111u8, 100u8,
							160u8, 250u8, 227u8, 24u8, 157u8, 226u8, 173u8, 39u8, 25u8, 200u8,
							137u8, 147u8, 232u8, 213u8, 182u8, 49u8, 142u8, 250u8, 139u8, 155u8,
							84u8, 214u8,
						] {
						let call = Free { index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
				#[doc = "held, then any deposit is reimbursed to its current owner."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `index`: the index to be (re-)assigned."]
				#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
				#[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
				#[doc = ""]
				#[doc = "Emits `IndexAssigned` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- Up to one reserve operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
				#[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
				#[doc = "# </weight>"]
				pub fn force_transfer(
					&self,
					new: ::subxt::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceTransfer,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceTransfer>()? ==
						[
							181u8, 143u8, 90u8, 135u8, 132u8, 11u8, 145u8, 85u8, 4u8, 211u8, 56u8,
							110u8, 213u8, 153u8, 224u8, 106u8, 198u8, 250u8, 130u8, 253u8, 72u8,
							58u8, 133u8, 150u8, 102u8, 119u8, 177u8, 175u8, 77u8, 106u8, 253u8,
							99u8,
						] {
						let call = ForceTransfer { new, index, freeze };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
				#[doc = "deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
				#[doc = "non-frozen account `index`."]
				#[doc = ""]
				#[doc = "- `index`: the index to be frozen in place."]
				#[doc = ""]
				#[doc = "Emits `IndexFrozen` if successful."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`."]
				#[doc = "- One storage mutation (codec `O(1)`)."]
				#[doc = "- Up to one slash operation."]
				#[doc = "- One event."]
				#[doc = "-------------------"]
				#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
				#[doc = "# </weight>"]
				pub fn freeze(
					&self,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Freeze, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Freeze>()? ==
						[
							204u8, 127u8, 214u8, 137u8, 138u8, 28u8, 171u8, 169u8, 184u8, 164u8,
							235u8, 114u8, 132u8, 176u8, 14u8, 207u8, 72u8, 39u8, 179u8, 231u8,
							137u8, 243u8, 242u8, 57u8, 89u8, 57u8, 213u8, 210u8, 87u8, 12u8, 253u8,
							159u8,
						] {
						let call = Freeze { index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_indices::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A account index was assigned."]
			pub struct IndexAssigned {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Event for IndexAssigned {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexAssigned";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "A account index has been freed up (unassigned)."]
			pub struct IndexFreed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Event for IndexFreed {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFreed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A account index has been frozen to its current account ID."]
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
			pub struct Accounts<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for Accounts<'_> {
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
				#[doc = " The lookup from index to account."]
				pub async fn accounts(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Accounts>()? ==
						[
							105u8, 208u8, 81u8, 30u8, 157u8, 108u8, 22u8, 122u8, 152u8, 220u8,
							40u8, 97u8, 255u8, 166u8, 222u8, 11u8, 81u8, 245u8, 143u8, 79u8, 57u8,
							19u8, 174u8, 164u8, 220u8, 59u8, 77u8, 117u8, 39u8, 72u8, 251u8, 234u8,
						] {
						let entry = Accounts(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The lookup from index to account."]
				pub async fn accounts_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Accounts<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Accounts>()? ==
						[
							105u8, 208u8, 81u8, 30u8, 157u8, 108u8, 22u8, 122u8, 152u8, 220u8,
							40u8, 97u8, 255u8, 166u8, 222u8, 11u8, 81u8, 245u8, 143u8, 79u8, 57u8,
							19u8, 174u8, 164u8, 220u8, 59u8, 77u8, 117u8, 39u8, 72u8, 251u8, 234u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The deposit needed for reserving an index."]
				pub fn deposit(
					&self,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Indices", "Deposit")? ==
						[
							173u8, 234u8, 239u8, 32u8, 129u8, 136u8, 106u8, 151u8, 229u8, 225u8,
							70u8, 47u8, 115u8, 214u8, 158u8, 215u8, 155u8, 20u8, 132u8, 204u8,
							168u8, 20u8, 135u8, 164u8, 129u8, 243u8, 20u8, 100u8, 47u8, 228u8, 4u8,
							248u8,
						] {
						let pallet = self.client.metadata().pallet("Indices")?;
						let constant = pallet.constant("Deposit")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod balances {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "It will decrease the total issuance of the system by the `TransferFee`."]
				#[doc = "If the sender's account is below the existential deposit as a result"]
				#[doc = "of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
				#[doc = "  types. See related functions below."]
				#[doc = "- It contains a limited number of reads and writes internally and no complex"]
				#[doc = "  computation."]
				#[doc = ""]
				#[doc = "Related functions:"]
				#[doc = ""]
				#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
				#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
				#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
				#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
				#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
				#[doc = "    that the transfer will not kill the origin account."]
				#[doc = "---------------------------------"]
				#[doc = "- Origin account is already in memory, so no DB operations for them."]
				#[doc = "# </weight>"]
				pub fn transfer(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Transfer,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Transfer>()? ==
						[
							51u8, 127u8, 65u8, 149u8, 186u8, 25u8, 125u8, 225u8, 172u8, 243u8,
							144u8, 156u8, 86u8, 150u8, 89u8, 114u8, 9u8, 142u8, 44u8, 98u8, 24u8,
							252u8, 83u8, 64u8, 78u8, 247u8, 136u8, 130u8, 203u8, 10u8, 206u8, 48u8,
						] {
						let call = Transfer { dest, value };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set the balances of a given account."]
				#[doc = ""]
				#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
				#[doc = "also decrease the total issuance of the system (`TotalIssuance`)."]
				#[doc = "If the new free or reserved balance is below the existential deposit,"]
				#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn set_balance(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetBalance,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetBalance>()? ==
						[
							126u8, 224u8, 173u8, 235u8, 17u8, 214u8, 51u8, 73u8, 132u8, 184u8,
							52u8, 124u8, 147u8, 120u8, 186u8, 82u8, 247u8, 199u8, 89u8, 31u8,
							111u8, 94u8, 224u8, 130u8, 198u8, 2u8, 60u8, 0u8, 16u8, 248u8, 243u8,
							216u8,
						] {
						let call = SetBalance { who, new_free, new_reserved };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
				#[doc = "specified."]
				#[doc = "# <weight>"]
				#[doc = "- Same as transfer, but additional read and write because the source account is not"]
				#[doc = "  assumed to be in the overlay."]
				#[doc = "# </weight>"]
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
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceTransfer,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceTransfer>()? ==
						[
							39u8, 171u8, 216u8, 52u8, 120u8, 195u8, 8u8, 202u8, 157u8, 154u8,
							191u8, 235u8, 163u8, 121u8, 132u8, 119u8, 166u8, 162u8, 163u8, 68u8,
							144u8, 193u8, 97u8, 194u8, 130u8, 136u8, 234u8, 84u8, 177u8, 134u8,
							0u8, 232u8,
						] {
						let call = ForceTransfer { source, dest, value };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
				#[doc = "origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer`] instead."]
				#[doc = ""]
				#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					value: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						TransferKeepAlive,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<TransferKeepAlive>()? ==
						[
							81u8, 224u8, 225u8, 42u8, 20u8, 198u8, 176u8, 165u8, 166u8, 150u8,
							143u8, 162u8, 202u8, 240u8, 59u8, 171u8, 17u8, 168u8, 211u8, 217u8,
							137u8, 108u8, 207u8, 95u8, 221u8, 51u8, 152u8, 4u8, 208u8, 79u8, 251u8,
							29u8,
						] {
						let call = TransferKeepAlive { dest, value };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Transfer the entire transferable balance from the caller account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
				#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
				#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
				#[doc = "you might need to prepare the account by removing any reference counters, storage"]
				#[doc = "deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be Signed."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
				#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
				#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true). # <weight>"]
				#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
				#[doc = "  #</weight>"]
				pub fn transfer_all(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					keep_alive: ::core::primitive::bool,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						TransferAll,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<TransferAll>()? ==
						[
							48u8, 241u8, 202u8, 6u8, 29u8, 207u8, 104u8, 141u8, 218u8, 18u8, 127u8,
							214u8, 99u8, 196u8, 39u8, 229u8, 120u8, 123u8, 130u8, 56u8, 129u8,
							169u8, 149u8, 62u8, 221u8, 108u8, 55u8, 201u8, 106u8, 36u8, 255u8,
							85u8,
						] {
						let call = TransferAll { dest, keep_alive };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					amount: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceUnreserve,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceUnreserve>()? ==
						[
							4u8, 231u8, 55u8, 137u8, 114u8, 76u8, 44u8, 166u8, 28u8, 224u8, 22u8,
							92u8, 76u8, 124u8, 219u8, 29u8, 204u8, 207u8, 179u8, 134u8, 93u8,
							137u8, 33u8, 178u8, 174u8, 106u8, 132u8, 204u8, 180u8, 122u8, 162u8,
							110u8,
						] {
						let call = ForceUnreserve { who, amount };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: ::subxt::sp_core::crypto::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::Event for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: ::subxt::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: ::subxt::sp_core::crypto::AccountId32,
				pub to: ::subxt::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::Event for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Event for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
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
			pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Account<'_> {
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
			pub struct Locks<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Locks<'_> {
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
			pub struct Reserves<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Reserves<'_> {
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
				#[doc = " The total units issued in the system."]
				pub async fn total_issuance(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<TotalIssuance>()? ==
						[
							1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
							156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
							20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
						] {
						let entry = TotalIssuance;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The balance of an account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub async fn account(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Account>()? ==
						[
							129u8, 169u8, 171u8, 206u8, 229u8, 178u8, 69u8, 118u8, 199u8, 64u8,
							254u8, 67u8, 16u8, 154u8, 160u8, 197u8, 177u8, 161u8, 148u8, 199u8,
							78u8, 219u8, 187u8, 83u8, 99u8, 110u8, 207u8, 252u8, 243u8, 39u8, 46u8,
							106u8,
						] {
						let entry = Account(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The balance of an account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub async fn account_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Account>()? ==
						[
							129u8, 169u8, 171u8, 206u8, 229u8, 178u8, 69u8, 118u8, 199u8, 64u8,
							254u8, 67u8, 16u8, 154u8, 160u8, 197u8, 177u8, 161u8, 148u8, 199u8,
							78u8, 219u8, 187u8, 83u8, 99u8, 110u8, 207u8, 252u8, 243u8, 39u8, 46u8,
							106u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub async fn locks(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Locks>()? ==
						[
							31u8, 76u8, 213u8, 60u8, 86u8, 11u8, 155u8, 151u8, 33u8, 212u8, 74u8,
							89u8, 174u8, 74u8, 195u8, 107u8, 29u8, 163u8, 178u8, 34u8, 209u8, 8u8,
							201u8, 237u8, 77u8, 99u8, 205u8, 212u8, 236u8, 132u8, 2u8, 252u8,
						] {
						let entry = Locks(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub async fn locks_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Locks>()? ==
						[
							31u8, 76u8, 213u8, 60u8, 86u8, 11u8, 155u8, 151u8, 33u8, 212u8, 74u8,
							89u8, 174u8, 74u8, 195u8, 107u8, 29u8, 163u8, 178u8, 34u8, 209u8, 8u8,
							201u8, 237u8, 77u8, 99u8, 205u8, 212u8, 236u8, 132u8, 2u8, 252u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Named reserves on some account balances."]
				pub async fn reserves(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						runtime_types::pallet_balances::ReserveData<
							[::core::primitive::u8; 8usize],
							::core::primitive::u128,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Reserves>()? ==
						[
							103u8, 6u8, 69u8, 151u8, 81u8, 40u8, 146u8, 113u8, 56u8, 239u8, 104u8,
							31u8, 168u8, 242u8, 141u8, 121u8, 213u8, 213u8, 114u8, 63u8, 62u8,
							47u8, 91u8, 119u8, 57u8, 91u8, 95u8, 81u8, 19u8, 208u8, 59u8, 146u8,
						] {
						let entry = Reserves(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Named reserves on some account balances."]
				pub async fn reserves_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Reserves<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Reserves>()? ==
						[
							103u8, 6u8, 69u8, 151u8, 81u8, 40u8, 146u8, 113u8, 56u8, 239u8, 104u8,
							31u8, 168u8, 242u8, 141u8, 121u8, 213u8, 213u8, 114u8, 63u8, 62u8,
							47u8, 91u8, 119u8, 57u8, 91u8, 95u8, 81u8, 19u8, 208u8, 59u8, 146u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " This is set to v2.0.0 for new networks."]
				pub async fn storage_version(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_balances::Releases,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<StorageVersion>()? ==
						[
							135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
							235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
							174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
							173u8, 96u8,
						] {
						let entry = StorageVersion;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The minimum amount required to keep an account open."]
				pub fn existential_deposit(
					&self,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Balances", "ExistentialDeposit")? ==
						[
							47u8, 23u8, 126u8, 122u8, 190u8, 106u8, 109u8, 151u8, 159u8, 1u8, 60u8,
							29u8, 128u8, 35u8, 34u8, 194u8, 245u8, 105u8, 14u8, 200u8, 102u8, 78u8,
							61u8, 138u8, 240u8, 186u8, 71u8, 219u8, 130u8, 118u8, 110u8, 251u8,
						] {
						let pallet = self.client.metadata().pallet("Balances")?;
						let constant = pallet.constant("ExistentialDeposit")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				pub fn max_locks(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Balances", "MaxLocks")? ==
						[
							250u8, 58u8, 19u8, 15u8, 35u8, 113u8, 227u8, 89u8, 39u8, 75u8, 21u8,
							108u8, 202u8, 32u8, 163u8, 167u8, 207u8, 233u8, 69u8, 151u8, 53u8,
							164u8, 230u8, 16u8, 14u8, 22u8, 172u8, 46u8, 36u8, 216u8, 29u8, 1u8,
						] {
						let pallet = self.client.metadata().pallet("Balances")?;
						let constant = pallet.constant("MaxLocks")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				pub fn max_reserves(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Balances", "MaxReserves")? ==
						[
							95u8, 163u8, 254u8, 186u8, 158u8, 222u8, 45u8, 163u8, 130u8, 111u8,
							59u8, 232u8, 163u8, 210u8, 243u8, 112u8, 38u8, 103u8, 252u8, 120u8,
							141u8, 104u8, 20u8, 200u8, 128u8, 65u8, 56u8, 145u8, 247u8, 95u8, 82u8,
							42u8,
						] {
						let pallet = self.client.metadata().pallet("Balances")?;
						let constant = pallet.constant("MaxReserves")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod parachain_system {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SetValidationData {
				pub data:
					runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
			}
			impl ::subxt::Call for SetValidationData {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "set_validation_data";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SudoSendUpwardMessage {
				pub message: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SudoSendUpwardMessage {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "sudo_send_upward_message";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct AuthorizeUpgrade {
				pub code_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for AuthorizeUpgrade {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "authorize_upgrade";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct EnactAuthorizedUpgrade {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for EnactAuthorizedUpgrade {
				const PALLET: &'static str = "ParachainSystem";
				const FUNCTION: &'static str = "enact_authorized_upgrade";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Set the current validation data."]
				#[doc = ""]
				#[doc = "This should be invoked exactly once per block. It will panic at the finalization"]
				#[doc = "phase if the call was not invoked."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Inherent`"]
				#[doc = ""]
				#[doc = "As a side effect, this function upgrades the current validation function"]
				#[doc = "if the appropriate time has come."]
				pub fn set_validation_data(
					&self,
					data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetValidationData,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetValidationData>()? ==
						[
							198u8, 82u8, 225u8, 120u8, 11u8, 189u8, 136u8, 67u8, 31u8, 232u8, 15u8,
							170u8, 5u8, 99u8, 176u8, 254u8, 236u8, 189u8, 18u8, 66u8, 64u8, 92u8,
							61u8, 173u8, 19u8, 56u8, 189u8, 73u8, 148u8, 215u8, 219u8, 157u8,
						] {
						let call = SetValidationData { data };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub fn sudo_send_upward_message(
					&self,
					message: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SudoSendUpwardMessage,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SudoSendUpwardMessage>()? ==
						[
							49u8, 161u8, 67u8, 34u8, 72u8, 150u8, 52u8, 109u8, 63u8, 41u8, 41u8,
							245u8, 33u8, 194u8, 43u8, 148u8, 187u8, 59u8, 184u8, 121u8, 200u8,
							167u8, 235u8, 215u8, 208u8, 44u8, 85u8, 233u8, 177u8, 198u8, 157u8,
							134u8,
						] {
						let call = SudoSendUpwardMessage { message };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub fn authorize_upgrade(
					&self,
					code_hash: ::subxt::sp_core::H256,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						AuthorizeUpgrade,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<AuthorizeUpgrade>()? ==
						[
							202u8, 225u8, 158u8, 88u8, 109u8, 6u8, 3u8, 61u8, 208u8, 170u8, 97u8,
							83u8, 5u8, 132u8, 199u8, 62u8, 195u8, 53u8, 233u8, 8u8, 235u8, 112u8,
							186u8, 240u8, 223u8, 81u8, 181u8, 0u8, 151u8, 85u8, 153u8, 6u8,
						] {
						let call = AuthorizeUpgrade { code_hash };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub fn enact_authorized_upgrade(
					&self,
					code: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						EnactAuthorizedUpgrade,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<EnactAuthorizedUpgrade>()? ==
						[
							210u8, 136u8, 47u8, 250u8, 122u8, 10u8, 208u8, 233u8, 122u8, 131u8,
							215u8, 134u8, 114u8, 143u8, 181u8, 32u8, 30u8, 221u8, 179u8, 84u8,
							98u8, 243u8, 136u8, 67u8, 206u8, 243u8, 190u8, 238u8, 177u8, 226u8,
							144u8, 103u8,
						] {
						let call = EnactAuthorizedUpgrade { code };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The validation function has been scheduled to apply."]
			pub struct ValidationFunctionStored;
			impl ::subxt::Event for ValidationFunctionStored {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionStored";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "The validation function was applied as of the contained relay chain block number."]
			pub struct ValidationFunctionApplied(pub ::core::primitive::u32);
			impl ::subxt::Event for ValidationFunctionApplied {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionApplied";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The relay-chain aborted the upgrade process."]
			pub struct ValidationFunctionDiscarded;
			impl ::subxt::Event for ValidationFunctionDiscarded {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "ValidationFunctionDiscarded";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An upgrade has been authorized."]
			pub struct UpgradeAuthorized(pub ::subxt::sp_core::H256);
			impl ::subxt::Event for UpgradeAuthorized {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "UpgradeAuthorized";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some downward messages have been received and will be processed."]
			#[doc = "\\[ count \\]"]
			pub struct DownwardMessagesReceived(pub ::core::primitive::u32);
			impl ::subxt::Event for DownwardMessagesReceived {
				const PALLET: &'static str = "ParachainSystem";
				const EVENT: &'static str = "DownwardMessagesReceived";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward messages were processed using the given weight."]
			#[doc = "\\[ weight_used, result_mqc_head \\]"]
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
			pub struct LastHrmpMqcHeads;
			impl ::subxt::StorageEntry for LastHrmpMqcHeads {
				const PALLET: &'static str = "ParachainSystem";
				const STORAGE: &'static str = "LastHrmpMqcHeads";
				type Value = ::subxt::KeyedVec<
					runtime_types::polkadot_parachain::primitives::Id,
					runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
				>;
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
				#[doc = " In case of a scheduled upgrade, this storage field contains the validation code to be applied."]
				#[doc = ""]
				#[doc = " As soon as the relay chain gives us the go-ahead signal, we will overwrite the [`:code`][well_known_keys::CODE]"]
				#[doc = " which will result the next block process with the new validation code. This concludes the upgrade process."]
				#[doc = ""]
				#[doc = " [well_known_keys::CODE]: sp_core::storage::well_known_keys::CODE"]
				pub async fn pending_validation_code(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<PendingValidationCode>()? ==
						[
							162u8, 35u8, 108u8, 76u8, 160u8, 93u8, 215u8, 84u8, 20u8, 249u8, 57u8,
							187u8, 88u8, 161u8, 15u8, 131u8, 213u8, 89u8, 140u8, 20u8, 227u8,
							204u8, 79u8, 176u8, 114u8, 119u8, 8u8, 7u8, 64u8, 15u8, 90u8, 92u8,
						] {
						let entry = PendingValidationCode;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Validation code that is set by the parachain and is to be communicated to collator and"]
				#[doc = " consequently the relay-chain."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block if no other pallet already set"]
				#[doc = " the value."]
				pub async fn new_validation_code(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<NewValidationCode>()? ==
						[
							224u8, 174u8, 53u8, 106u8, 240u8, 49u8, 48u8, 79u8, 219u8, 74u8, 142u8,
							166u8, 92u8, 204u8, 244u8, 200u8, 43u8, 169u8, 177u8, 207u8, 190u8,
							106u8, 180u8, 65u8, 245u8, 131u8, 134u8, 4u8, 53u8, 45u8, 76u8, 3u8,
						] {
						let entry = NewValidationCode;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The [`PersistedValidationData`] set for this block."]
				#[doc = " This value is expected to be set only once per block and it's never stored"]
				#[doc = " in the trie."]
				pub async fn validation_data(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::polkadot_primitives::v1::PersistedValidationData<
							::subxt::sp_core::H256,
							::core::primitive::u32,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ValidationData>()? ==
						[
							172u8, 221u8, 162u8, 67u8, 105u8, 190u8, 232u8, 237u8, 46u8, 36u8,
							254u8, 224u8, 88u8, 242u8, 45u8, 20u8, 213u8, 211u8, 15u8, 25u8, 231u8,
							17u8, 41u8, 68u8, 33u8, 137u8, 127u8, 245u8, 219u8, 227u8, 101u8, 55u8,
						] {
						let entry = ValidationData;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Were the validation data set to notify the relay chain?"]
				pub async fn did_set_validation_code(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<DidSetValidationCode>()? ==
						[
							89u8, 83u8, 74u8, 174u8, 234u8, 188u8, 149u8, 78u8, 140u8, 17u8, 92u8,
							165u8, 243u8, 87u8, 59u8, 97u8, 135u8, 81u8, 192u8, 86u8, 193u8, 187u8,
							113u8, 22u8, 108u8, 83u8, 242u8, 208u8, 174u8, 40u8, 49u8, 245u8,
						] {
						let entry = DidSetValidationCode;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " An option which indicates if the relay-chain restricts signalling a validation code upgrade."]
				#[doc = " In other words, if this is `Some` and [`NewValidationCode`] is `Some` then the produced"]
				#[doc = " candidate will be invalid."]
				#[doc = ""]
				#[doc = " This storage item is a mirror of the corresponding value for the current parachain from the"]
				#[doc = " relay-chain. This value is ephemeral which means it doesn't hit the storage. This value is"]
				#[doc = " set after the inherent."]
				pub async fn upgrade_restriction_signal(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::polkadot_primitives::v1::UpgradeRestriction,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<UpgradeRestrictionSignal>()? ==
						[
							61u8, 3u8, 26u8, 6u8, 88u8, 114u8, 109u8, 63u8, 7u8, 115u8, 245u8,
							198u8, 73u8, 234u8, 28u8, 228u8, 126u8, 27u8, 151u8, 18u8, 133u8, 54u8,
							144u8, 149u8, 246u8, 43u8, 83u8, 47u8, 77u8, 238u8, 10u8, 196u8,
						] {
						let entry = UpgradeRestrictionSignal;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The snapshot of some state related to messaging relevant to the current parachain as per"]
				#[doc = " the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]				pub async fn relevant_messaging_state (& self , block_hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: BasicError >{
					if self.client.metadata().storage_hash::<RelevantMessagingState>()? ==
						[
							10u8, 168u8, 63u8, 15u8, 50u8, 249u8, 199u8, 12u8, 123u8, 226u8, 71u8,
							245u8, 201u8, 1u8, 203u8, 39u8, 36u8, 92u8, 238u8, 226u8, 252u8, 199u8,
							82u8, 110u8, 90u8, 224u8, 175u8, 160u8, 214u8, 130u8, 151u8, 33u8,
						] {
						let entry = RelevantMessagingState;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The parachain host configuration that was obtained from the relay parent."]
				#[doc = ""]
				#[doc = " This field is meant to be updated each block with the validation data inherent. Therefore,"]
				#[doc = " before processing of the inherent, e.g. in `on_initialize` this data may be stale."]
				#[doc = ""]
				#[doc = " This data is also absent from the genesis."]
				pub async fn host_configuration(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<HostConfiguration>()? ==
						[
							93u8, 232u8, 157u8, 77u8, 159u8, 246u8, 12u8, 196u8, 68u8, 192u8, 0u8,
							137u8, 142u8, 55u8, 131u8, 75u8, 250u8, 224u8, 113u8, 181u8, 137u8,
							216u8, 68u8, 137u8, 97u8, 63u8, 181u8, 21u8, 214u8, 30u8, 83u8, 255u8,
						] {
						let entry = HostConfiguration;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The last downward message queue chain head we have observed."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub async fn last_dmq_mqc_head(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<LastDmqMqcHead>()? ==
						[
							1u8, 201u8, 86u8, 218u8, 174u8, 105u8, 70u8, 87u8, 109u8, 91u8, 95u8,
							246u8, 252u8, 48u8, 61u8, 73u8, 34u8, 73u8, 42u8, 47u8, 57u8, 225u8,
							240u8, 143u8, 185u8, 100u8, 71u8, 13u8, 46u8, 152u8, 74u8, 143u8,
						] {
						let entry = LastDmqMqcHead;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The message queue chain heads we have observed per each channel incoming channel."]
				#[doc = ""]
				#[doc = " This value is loaded before and saved after processing inbound downward messages carried"]
				#[doc = " by the system inherent."]
				pub async fn last_hrmp_mqc_heads(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyedVec<
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::cumulus_pallet_parachain_system::MessageQueueChain,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<LastHrmpMqcHeads>()? ==
						[
							228u8, 142u8, 229u8, 142u8, 234u8, 203u8, 56u8, 62u8, 233u8, 157u8,
							133u8, 124u8, 155u8, 153u8, 158u8, 55u8, 47u8, 215u8, 189u8, 15u8, 2u8,
							177u8, 247u8, 156u8, 172u8, 38u8, 13u8, 185u8, 253u8, 211u8, 201u8,
							200u8,
						] {
						let entry = LastHrmpMqcHeads;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Number of downward messages processed in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub async fn processed_downward_messages(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<ProcessedDownwardMessages>()? ==
						[
							48u8, 177u8, 84u8, 228u8, 101u8, 235u8, 181u8, 27u8, 66u8, 55u8, 50u8,
							146u8, 245u8, 223u8, 77u8, 132u8, 178u8, 80u8, 74u8, 90u8, 166u8, 81u8,
							109u8, 25u8, 91u8, 69u8, 5u8, 69u8, 123u8, 197u8, 160u8, 146u8,
						] {
						let entry = ProcessedDownwardMessages;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " HRMP watermark that was set in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub async fn hrmp_watermark(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<HrmpWatermark>()? ==
						[
							189u8, 59u8, 183u8, 195u8, 69u8, 185u8, 241u8, 226u8, 62u8, 204u8,
							230u8, 77u8, 102u8, 75u8, 86u8, 157u8, 249u8, 140u8, 219u8, 72u8, 94u8,
							64u8, 176u8, 72u8, 34u8, 205u8, 114u8, 103u8, 231u8, 233u8, 206u8,
							111u8,
						] {
						let entry = HrmpWatermark;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " HRMP messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub async fn hrmp_outbound_messages(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
							runtime_types::polkadot_parachain::primitives::Id,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<HrmpOutboundMessages>()? ==
						[
							117u8, 161u8, 141u8, 250u8, 88u8, 41u8, 18u8, 251u8, 19u8, 199u8,
							211u8, 142u8, 42u8, 81u8, 27u8, 17u8, 225u8, 31u8, 62u8, 248u8, 13u8,
							154u8, 129u8, 243u8, 7u8, 44u8, 221u8, 249u8, 218u8, 6u8, 75u8, 210u8,
						] {
						let entry = HrmpOutboundMessages;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Upward messages that were sent in a block."]
				#[doc = ""]
				#[doc = " This will be cleared in `on_initialize` of each new block."]
				pub async fn upward_messages(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<UpwardMessages>()? ==
						[
							129u8, 208u8, 187u8, 36u8, 48u8, 108u8, 135u8, 56u8, 204u8, 60u8,
							100u8, 158u8, 113u8, 238u8, 46u8, 92u8, 228u8, 41u8, 178u8, 177u8,
							208u8, 195u8, 148u8, 149u8, 127u8, 21u8, 93u8, 92u8, 29u8, 115u8, 10u8,
							248u8,
						] {
						let entry = UpwardMessages;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Upward messages that are still pending and not yet send to the relay chain."]
				pub async fn pending_upward_messages(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<PendingUpwardMessages>()? ==
						[
							223u8, 46u8, 224u8, 227u8, 222u8, 119u8, 225u8, 244u8, 59u8, 87u8,
							127u8, 19u8, 217u8, 237u8, 103u8, 61u8, 6u8, 210u8, 107u8, 201u8,
							117u8, 25u8, 85u8, 248u8, 36u8, 231u8, 28u8, 202u8, 41u8, 140u8, 208u8,
							254u8,
						] {
						let entry = PendingUpwardMessages;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The number of HRMP messages we observed in `on_initialize` and thus used that number for"]
				#[doc = " announcing the weight of `on_initialize` and `on_finalize`."]
				pub async fn announced_hrmp_messages_per_candidate(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<AnnouncedHrmpMessagesPerCandidate>()? ==
						[
							132u8, 61u8, 162u8, 129u8, 251u8, 243u8, 20u8, 144u8, 162u8, 73u8,
							237u8, 51u8, 248u8, 41u8, 127u8, 171u8, 180u8, 79u8, 137u8, 23u8, 66u8,
							134u8, 106u8, 222u8, 182u8, 154u8, 0u8, 145u8, 184u8, 156u8, 36u8,
							97u8,
						] {
						let entry = AnnouncedHrmpMessagesPerCandidate;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The weight we reserve at the beginning of the block for processing XCMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub async fn reserved_xcmp_weight_override(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u64>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ReservedXcmpWeightOverride>()? ==
						[
							250u8, 177u8, 18u8, 183u8, 23u8, 84u8, 14u8, 178u8, 92u8, 60u8, 210u8,
							155u8, 63u8, 58u8, 105u8, 196u8, 184u8, 235u8, 145u8, 11u8, 215u8,
							121u8, 60u8, 140u8, 14u8, 50u8, 185u8, 101u8, 210u8, 230u8, 180u8,
							250u8,
						] {
						let entry = ReservedXcmpWeightOverride;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The weight we reserve at the beginning of the block for processing DMP messages. This"]
				#[doc = " overrides the amount set in the Config trait."]
				pub async fn reserved_dmp_weight_override(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u64>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ReservedDmpWeightOverride>()? ==
						[
							20u8, 145u8, 152u8, 245u8, 73u8, 101u8, 125u8, 190u8, 151u8, 180u8,
							22u8, 157u8, 58u8, 115u8, 165u8, 167u8, 117u8, 166u8, 201u8, 10u8,
							206u8, 255u8, 206u8, 40u8, 40u8, 63u8, 228u8, 53u8, 58u8, 47u8, 121u8,
							76u8,
						] {
						let entry = ReservedDmpWeightOverride;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The next authorized upgrade, if there is one."]
				pub async fn authorized_upgrade(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::H256>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<AuthorizedUpgrade>()? ==
						[
							162u8, 253u8, 211u8, 241u8, 158u8, 137u8, 172u8, 159u8, 193u8, 182u8,
							156u8, 218u8, 210u8, 80u8, 149u8, 115u8, 199u8, 196u8, 61u8, 205u8,
							167u8, 24u8, 183u8, 183u8, 19u8, 199u8, 55u8, 75u8, 142u8, 125u8,
							192u8, 202u8,
						] {
						let entry = AuthorizedUpgrade;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod parachain_info {
		use super::{root_mod, runtime_types};
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
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::polkadot_parachain::primitives::Id,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ParachainId>()? ==
						[
							108u8, 199u8, 129u8, 85u8, 169u8, 54u8, 7u8, 32u8, 128u8, 184u8, 180u8,
							179u8, 182u8, 212u8, 83u8, 150u8, 166u8, 154u8, 160u8, 103u8, 137u8,
							115u8, 60u8, 124u8, 68u8, 90u8, 80u8, 100u8, 13u8, 187u8, 22u8, 137u8,
						] {
						let entry = ParachainId;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod authorship {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Provide a set of uncles."]
				pub fn set_uncles(
					&self,
					new_uncles: ::std::vec::Vec<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetUncles,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetUncles>()? ==
						[
							5u8, 56u8, 71u8, 152u8, 103u8, 232u8, 101u8, 171u8, 200u8, 2u8, 177u8,
							102u8, 0u8, 93u8, 210u8, 90u8, 56u8, 151u8, 5u8, 235u8, 227u8, 197u8,
							189u8, 248u8, 2u8, 71u8, 49u8, 220u8, 212u8, 253u8, 235u8, 67u8,
						] {
						let call = SetUncles { new_uncles };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
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
				#[doc = " Uncles"]
				pub async fn uncles(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
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
					if self.client.metadata().storage_hash::<Uncles>()? ==
						[
							71u8, 135u8, 85u8, 172u8, 221u8, 165u8, 212u8, 2u8, 208u8, 50u8, 9u8,
							92u8, 251u8, 25u8, 194u8, 123u8, 210u8, 4u8, 148u8, 30u8, 20u8, 146u8,
							21u8, 210u8, 138u8, 128u8, 144u8, 152u8, 97u8, 57u8, 205u8, 231u8,
						] {
						let entry = Uncles;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Author of current block."]
				pub async fn author(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Author>()? ==
						[
							191u8, 57u8, 3u8, 242u8, 220u8, 123u8, 103u8, 215u8, 149u8, 120u8,
							20u8, 139u8, 146u8, 234u8, 180u8, 105u8, 129u8, 128u8, 114u8, 147u8,
							114u8, 236u8, 23u8, 21u8, 15u8, 250u8, 180u8, 19u8, 177u8, 145u8, 77u8,
							228u8,
						] {
						let entry = Author;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Whether uncles were already set in this block."]
				pub async fn did_set_uncles(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<DidSetUncles>()? ==
						[
							64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
							126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
							83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
						] {
						let entry = DidSetUncles;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The number of blocks back we should accept uncles."]
				#[doc = " This means that we will deal with uncle-parents that are"]
				#[doc = " `UncleGenerations + 1` before `now`."]
				pub fn uncle_generations(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Authorship", "UncleGenerations")? ==
						[
							0u8, 72u8, 57u8, 175u8, 222u8, 143u8, 191u8, 33u8, 163u8, 157u8, 202u8,
							83u8, 186u8, 103u8, 162u8, 103u8, 227u8, 158u8, 239u8, 212u8, 205u8,
							193u8, 226u8, 138u8, 5u8, 220u8, 221u8, 42u8, 7u8, 146u8, 173u8, 205u8,
						] {
						let pallet = self.client.metadata().pallet("Authorship")?;
						let constant = pallet.constant("UncleGenerations")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod collator_selection {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SetInvulnerables {
				pub new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::Call for SetInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "set_invulnerables";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct SetDesiredCandidates {
				pub max: ::core::primitive::u32,
			}
			impl ::subxt::Call for SetDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "set_desired_candidates";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct SetCandidacyBond {
				pub bond: ::core::primitive::u128,
			}
			impl ::subxt::Call for SetCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "set_candidacy_bond";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct RegisterAsCandidate;
			impl ::subxt::Call for RegisterAsCandidate {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "register_as_candidate";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct LeaveIntent;
			impl ::subxt::Call for LeaveIntent {
				const PALLET: &'static str = "CollatorSelection";
				const FUNCTION: &'static str = "leave_intent";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				pub fn set_invulnerables(
					&self,
					new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetInvulnerables,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetInvulnerables>()? ==
						[
							147u8, 148u8, 230u8, 132u8, 108u8, 68u8, 229u8, 176u8, 25u8, 161u8,
							84u8, 123u8, 218u8, 134u8, 175u8, 81u8, 164u8, 126u8, 223u8, 105u8,
							212u8, 180u8, 65u8, 98u8, 254u8, 0u8, 252u8, 18u8, 159u8, 77u8, 51u8,
							150u8,
						] {
						let call = SetInvulnerables { new };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub fn set_desired_candidates(
					&self,
					max: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetDesiredCandidates,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetDesiredCandidates>()? ==
						[
							167u8, 161u8, 161u8, 25u8, 27u8, 34u8, 137u8, 171u8, 89u8, 126u8, 89u8,
							114u8, 219u8, 176u8, 83u8, 124u8, 156u8, 49u8, 188u8, 209u8, 39u8,
							138u8, 180u8, 47u8, 33u8, 162u8, 222u8, 74u8, 130u8, 117u8, 173u8,
							16u8,
						] {
						let call = SetDesiredCandidates { max };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub fn set_candidacy_bond(
					&self,
					bond: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetCandidacyBond,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetCandidacyBond>()? ==
						[
							128u8, 221u8, 132u8, 1u8, 215u8, 113u8, 188u8, 201u8, 197u8, 40u8,
							84u8, 60u8, 151u8, 237u8, 145u8, 70u8, 102u8, 48u8, 2u8, 2u8, 36u8,
							1u8, 99u8, 17u8, 98u8, 195u8, 131u8, 27u8, 47u8, 209u8, 197u8, 132u8,
						] {
						let call = SetCandidacyBond { bond };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub fn register_as_candidate(
					&self,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						RegisterAsCandidate,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<RegisterAsCandidate>()? ==
						[
							63u8, 11u8, 114u8, 142u8, 89u8, 78u8, 120u8, 214u8, 22u8, 215u8, 125u8,
							60u8, 203u8, 89u8, 141u8, 126u8, 124u8, 167u8, 70u8, 240u8, 85u8,
							253u8, 34u8, 245u8, 67u8, 46u8, 240u8, 195u8, 57u8, 81u8, 138u8, 69u8,
						] {
						let call = RegisterAsCandidate {};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				pub fn leave_intent(
					&self,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						LeaveIntent,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<LeaveIntent>()? ==
						[
							217u8, 3u8, 35u8, 71u8, 152u8, 203u8, 203u8, 212u8, 25u8, 113u8, 158u8,
							124u8, 161u8, 154u8, 32u8, 47u8, 116u8, 134u8, 11u8, 201u8, 154u8,
							40u8, 138u8, 163u8, 184u8, 188u8, 33u8, 237u8, 219u8, 40u8, 63u8,
							221u8,
						] {
						let call = LeaveIntent {};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct NewInvulnerables(pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>);
			impl ::subxt::Event for NewInvulnerables {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewInvulnerables";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct NewDesiredCandidates(pub ::core::primitive::u32);
			impl ::subxt::Event for NewDesiredCandidates {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewDesiredCandidates";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct NewCandidacyBond(pub ::core::primitive::u128);
			impl ::subxt::Event for NewCandidacyBond {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "NewCandidacyBond";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct CandidateAdded(
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for CandidateAdded {
				const PALLET: &'static str = "CollatorSelection";
				const EVENT: &'static str = "CandidateAdded";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			pub struct LastAuthoredBlock<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for LastAuthoredBlock<'_> {
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
				#[doc = " The invulnerable, fixed collators."]
				pub async fn invulnerables(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Invulnerables>()? ==
						[
							103u8, 93u8, 29u8, 166u8, 244u8, 19u8, 78u8, 182u8, 235u8, 37u8, 199u8,
							127u8, 211u8, 124u8, 168u8, 145u8, 111u8, 251u8, 33u8, 36u8, 167u8,
							119u8, 124u8, 206u8, 205u8, 14u8, 186u8, 68u8, 16u8, 150u8, 45u8,
							158u8,
						] {
						let entry = Invulnerables;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The (community, limited) collation candidates."]
				pub async fn candidates(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						runtime_types::pallet_collator_selection::pallet::CandidateInfo<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Candidates>()? ==
						[
							249u8, 206u8, 40u8, 15u8, 9u8, 10u8, 173u8, 200u8, 141u8, 154u8, 232u8,
							31u8, 20u8, 129u8, 99u8, 96u8, 187u8, 128u8, 10u8, 208u8, 198u8, 123u8,
							197u8, 219u8, 242u8, 23u8, 110u8, 34u8, 220u8, 224u8, 68u8, 161u8,
						] {
						let entry = Candidates;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Last block authored by collator."]
				pub async fn last_authored_block(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<LastAuthoredBlock>()? ==
						[
							31u8, 13u8, 209u8, 126u8, 134u8, 83u8, 102u8, 173u8, 231u8, 231u8,
							65u8, 155u8, 145u8, 135u8, 166u8, 140u8, 35u8, 32u8, 150u8, 177u8,
							205u8, 188u8, 216u8, 226u8, 121u8, 190u8, 41u8, 179u8, 244u8, 254u8,
							54u8, 149u8,
						] {
						let entry = LastAuthoredBlock(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Last block authored by collator."]
				pub async fn last_authored_block_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, LastAuthoredBlock<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<LastAuthoredBlock>()? ==
						[
							31u8, 13u8, 209u8, 126u8, 134u8, 83u8, 102u8, 173u8, 231u8, 231u8,
							65u8, 155u8, 145u8, 135u8, 166u8, 140u8, 35u8, 32u8, 150u8, 177u8,
							205u8, 188u8, 216u8, 226u8, 121u8, 190u8, 41u8, 179u8, 244u8, 254u8,
							54u8, 149u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Desired number of candidates."]
				#[doc = ""]
				#[doc = " This should ideally always be less than [`Config::MaxCandidates`] for weights to be correct."]
				pub async fn desired_candidates(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<DesiredCandidates>()? ==
						[
							161u8, 170u8, 254u8, 76u8, 112u8, 146u8, 144u8, 7u8, 177u8, 152u8,
							146u8, 60u8, 143u8, 237u8, 1u8, 168u8, 176u8, 33u8, 103u8, 35u8, 39u8,
							233u8, 107u8, 253u8, 47u8, 183u8, 11u8, 86u8, 230u8, 13u8, 127u8,
							133u8,
						] {
						let entry = DesiredCandidates;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Fixed deposit bond for each candidate."]
				pub async fn candidacy_bond(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<CandidacyBond>()? ==
						[
							1u8, 153u8, 211u8, 74u8, 138u8, 178u8, 81u8, 9u8, 205u8, 117u8, 102u8,
							182u8, 56u8, 184u8, 56u8, 62u8, 193u8, 82u8, 224u8, 218u8, 253u8,
							194u8, 250u8, 55u8, 220u8, 107u8, 157u8, 175u8, 62u8, 35u8, 224u8,
							183u8,
						] {
						let entry = CandidacyBond;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SetKeys {
				pub keys: runtime_types::composable_runtime::opaque::SessionKeys,
				pub proof: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for SetKeys {
				const PALLET: &'static str = "Session";
				const FUNCTION: &'static str = "set_keys";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct PurgeKeys;
			impl ::subxt::Call for PurgeKeys {
				const PALLET: &'static str = "Session";
				const FUNCTION: &'static str = "purge_keys";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Sets the session key(s) of the function caller to `keys`."]
				#[doc = "Allows an account to set its session key prior to becoming a validator."]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be signed."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
				#[doc = "  `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
				#[doc = "- DbWrites: `origin account`, `NextKeys`"]
				#[doc = "- DbReads per key id: `KeyOwner`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn set_keys(
					&self,
					keys: runtime_types::composable_runtime::opaque::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetKeys,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetKeys>()? ==
						[
							240u8, 174u8, 244u8, 95u8, 15u8, 73u8, 108u8, 183u8, 164u8, 138u8,
							122u8, 226u8, 146u8, 189u8, 211u8, 93u8, 251u8, 222u8, 161u8, 232u8,
							253u8, 4u8, 150u8, 5u8, 169u8, 142u8, 85u8, 137u8, 227u8, 90u8, 183u8,
							204u8,
						] {
						let call = SetKeys { keys, proof };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Removes any session key(s) of the function caller."]
				#[doc = ""]
				#[doc = "This doesn't take effect until the next session."]
				#[doc = ""]
				#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
				#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
				#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
				#[doc = "usually means being a stash account)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
				#[doc = "  of `T::Keys::key_ids()` which is fixed."]
				#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
				#[doc = "- DbWrites: `NextKeys`, `origin account`"]
				#[doc = "- DbWrites per key id: `KeyOwner`"]
				#[doc = "# </weight>"]
				pub fn purge_keys(
					&self,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						PurgeKeys,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<PurgeKeys>()? ==
						[
							200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
							35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
							71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
						] {
						let call = PurgeKeys {};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
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
					runtime_types::composable_runtime::opaque::SessionKeys,
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
			pub struct NextKeys<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for NextKeys<'_> {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "NextKeys";
				type Value = runtime_types::composable_runtime::opaque::SessionKeys;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Twox64Concat,
					)])
				}
			}
			pub struct KeyOwner<'a>(
				pub &'a runtime_types::sp_core::crypto::KeyTypeId,
				pub &'a [::core::primitive::u8],
			);
			impl ::subxt::StorageEntry for KeyOwner<'_> {
				const PALLET: &'static str = "Session";
				const STORAGE: &'static str = "KeyOwner";
				type Value = ::subxt::sp_core::crypto::AccountId32;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&(&self.0, &self.1),
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
				#[doc = " The current set of validators."]
				pub async fn validators(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Validators>()? ==
						[
							186u8, 248u8, 234u8, 74u8, 245u8, 141u8, 90u8, 152u8, 226u8, 220u8,
							255u8, 104u8, 174u8, 1u8, 37u8, 152u8, 23u8, 208u8, 25u8, 49u8, 33u8,
							253u8, 254u8, 251u8, 141u8, 16u8, 18u8, 175u8, 196u8, 188u8, 163u8,
							209u8,
						] {
						let entry = Validators;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Current index of the session."]
				pub async fn current_index(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<CurrentIndex>()? ==
						[
							148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
							183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
							159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
							130u8, 40u8,
						] {
						let entry = CurrentIndex;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub async fn queued_changed(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<QueuedChanged>()? ==
						[
							105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
							244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
							247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
						] {
						let entry = QueuedChanged;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub async fn queued_keys(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						::subxt::sp_core::crypto::AccountId32,
						runtime_types::composable_runtime::opaque::SessionKeys,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<QueuedKeys>()? ==
						[
							77u8, 154u8, 63u8, 8u8, 160u8, 17u8, 30u8, 118u8, 131u8, 61u8, 176u8,
							206u8, 180u8, 74u8, 125u8, 233u8, 186u8, 12u8, 229u8, 72u8, 143u8,
							62u8, 18u8, 29u8, 94u8, 25u8, 70u8, 168u8, 90u8, 104u8, 107u8, 75u8,
						] {
						let entry = QueuedKeys;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub async fn disabled_validators(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::core::primitive::u32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<DisabledValidators>()? ==
						[
							135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
							189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
							110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
							84u8,
						] {
						let entry = DisabledValidators;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The next session keys for a validator."]
				pub async fn next_keys(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::composable_runtime::opaque::SessionKeys>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<NextKeys>()? ==
						[
							91u8, 120u8, 219u8, 231u8, 50u8, 104u8, 67u8, 140u8, 26u8, 194u8,
							130u8, 248u8, 34u8, 225u8, 102u8, 242u8, 31u8, 98u8, 135u8, 106u8,
							49u8, 171u8, 119u8, 69u8, 227u8, 172u8, 137u8, 60u8, 137u8, 155u8,
							71u8, 75u8,
						] {
						let entry = NextKeys(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The next session keys for a validator."]
				pub async fn next_keys_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, NextKeys<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<NextKeys>()? ==
						[
							91u8, 120u8, 219u8, 231u8, 50u8, 104u8, 67u8, 140u8, 26u8, 194u8,
							130u8, 248u8, 34u8, 225u8, 102u8, 242u8, 31u8, 98u8, 135u8, 106u8,
							49u8, 171u8, 119u8, 69u8, 227u8, 172u8, 137u8, 60u8, 137u8, 155u8,
							71u8, 75u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub async fn key_owner(
					&self,
					_0: &runtime_types::sp_core::crypto::KeyTypeId,
					_1: &[::core::primitive::u8],
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<KeyOwner>()? ==
						[
							49u8, 245u8, 212u8, 141u8, 211u8, 208u8, 109u8, 102u8, 249u8, 161u8,
							41u8, 93u8, 220u8, 230u8, 14u8, 59u8, 251u8, 176u8, 33u8, 127u8, 93u8,
							149u8, 205u8, 229u8, 113u8, 129u8, 162u8, 177u8, 155u8, 216u8, 151u8,
							57u8,
						] {
						let entry = KeyOwner(_0, _1);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub async fn key_owner_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, KeyOwner<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<KeyOwner>()? ==
						[
							49u8, 245u8, 212u8, 141u8, 211u8, 208u8, 109u8, 102u8, 249u8, 161u8,
							41u8, 93u8, 220u8, 230u8, 14u8, 59u8, 251u8, 176u8, 33u8, 127u8, 93u8,
							149u8, 205u8, 229u8, 113u8, 129u8, 162u8, 177u8, 155u8, 216u8, 151u8,
							57u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod aura {
		use super::{root_mod, runtime_types};
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
				#[doc = " The current authority set."]
				pub async fn authorities(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
						runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Authorities>()? ==
						[
							168u8, 101u8, 224u8, 96u8, 254u8, 152u8, 213u8, 141u8, 46u8, 181u8,
							131u8, 23u8, 218u8, 24u8, 145u8, 111u8, 161u8, 192u8, 253u8, 29u8,
							128u8, 92u8, 125u8, 159u8, 242u8, 144u8, 253u8, 174u8, 50u8, 190u8,
							148u8, 193u8,
						] {
						let entry = Authorities;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The current slot of this block."]
				#[doc = ""]
				#[doc = " This will be set in `on_initialize`."]
				pub async fn current_slot(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::sp_consensus_slots::Slot,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<CurrentSlot>()? ==
						[
							233u8, 102u8, 77u8, 99u8, 103u8, 50u8, 151u8, 229u8, 46u8, 226u8,
							181u8, 37u8, 117u8, 204u8, 234u8, 120u8, 116u8, 166u8, 80u8, 188u8,
							92u8, 154u8, 137u8, 150u8, 79u8, 164u8, 29u8, 203u8, 2u8, 51u8, 123u8,
							104u8,
						] {
						let entry = CurrentSlot;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod aura_ext {
		use super::{root_mod, runtime_types};
	}
	pub mod council {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
				pub prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			impl ::subxt::Call for SetMembers {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "set_members";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for Execute {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "execute";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for Propose {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "propose";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for DisapproveProposal {
				const PALLET: &'static str = "Council";
				const FUNCTION: &'static str = "disapprove_proposal";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Set the collective's membership."]
				#[doc = ""]
				#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
				#[doc = "- `prime`: The prime member whose vote sets the default."]
				#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
				#[doc = "  weight estimation."]
				#[doc = ""]
				#[doc = "Requires root origin."]
				#[doc = ""]
				#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
				#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
				#[doc = ""]
				#[doc = "# WARNING:"]
				#[doc = ""]
				#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
				#[doc = "implementation of the trait [`ChangeMembers`]."]
				#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
				#[doc = "with other logic managing the member set."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(MP + N)` where:"]
				#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
				#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
				#[doc = "  - `P` proposals-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
				#[doc = "    members"]
				#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
				#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
				#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
				#[doc = "# </weight>"]
				pub fn set_members(
					&self,
					new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetMembers,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetMembers>()? ==
						[
							228u8, 186u8, 17u8, 12u8, 231u8, 231u8, 139u8, 15u8, 96u8, 200u8, 68u8,
							27u8, 61u8, 106u8, 245u8, 199u8, 120u8, 141u8, 95u8, 215u8, 36u8, 49u8,
							0u8, 163u8, 172u8, 252u8, 221u8, 9u8, 1u8, 222u8, 44u8, 214u8,
						] {
						let call = SetMembers { new_members, prime, old_count };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Dispatch a proposal from a member using the `Member` origin."]
				#[doc = ""]
				#[doc = "Origin must be a member of the collective."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
				#[doc = "  `proposal`"]
				#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn execute(
					&self,
					proposal: runtime_types::composable_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Execute,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Execute>()? ==
						[
							24u8, 118u8, 155u8, 80u8, 107u8, 152u8, 182u8, 84u8, 63u8, 93u8, 91u8,
							2u8, 125u8, 175u8, 29u8, 124u8, 2u8, 132u8, 100u8, 31u8, 174u8, 254u8,
							174u8, 134u8, 219u8, 238u8, 59u8, 95u8, 221u8, 5u8, 198u8, 242u8,
						] {
						let call =
							Execute { proposal: ::std::boxed::Box::new(proposal), length_bound };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Add a new proposal to either be voted on or executed directly."]
				#[doc = ""]
				#[doc = "Requires the sender to be member."]
				#[doc = ""]
				#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
				#[doc = "or put up for voting."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - branching is influenced by `threshold` where:"]
				#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
				#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
				#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
				#[doc = "  - DB accesses influenced by `threshold`:"]
				#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
				#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
				#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
				#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
				#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
				#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
				#[doc = "  - 1 event"]
				#[doc = "# </weight>"]
				pub fn propose(
					&self,
					threshold: ::core::primitive::u32,
					proposal: runtime_types::composable_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Propose,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Propose>()? ==
						[
							100u8, 108u8, 11u8, 34u8, 198u8, 201u8, 232u8, 44u8, 121u8, 206u8,
							240u8, 131u8, 134u8, 23u8, 5u8, 130u8, 8u8, 95u8, 226u8, 37u8, 221u8,
							116u8, 70u8, 49u8, 181u8, 167u8, 151u8, 221u8, 183u8, 197u8, 181u8,
							134u8,
						] {
						let call = Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Add an aye or nay vote for the sender to the given proposal."]
				#[doc = ""]
				#[doc = "Requires the sender to be a member."]
				#[doc = ""]
				#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
				#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
				#[doc = "fee."]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
				#[doc = "- DB:"]
				#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
				#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
				#[doc = "- 1 event"]
				#[doc = "# </weight>"]
				pub fn vote(
					&self,
					proposal: ::subxt::sp_core::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Vote, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Vote>()? ==
						[
							184u8, 236u8, 80u8, 133u8, 26u8, 207u8, 3u8, 2u8, 120u8, 27u8, 38u8,
							135u8, 195u8, 86u8, 169u8, 229u8, 125u8, 253u8, 220u8, 120u8, 231u8,
							181u8, 101u8, 84u8, 151u8, 161u8, 39u8, 154u8, 183u8, 142u8, 165u8,
							161u8,
						] {
						let call = Vote { proposal, index, approve };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
				#[doc = ""]
				#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
				#[doc = ""]
				#[doc = "If called before the end of the voting period it will only close the vote if it is"]
				#[doc = "has enough votes to be approved or disapproved."]
				#[doc = ""]
				#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
				#[doc = "unless there is a prime member set and the prime member cast an approval."]
				#[doc = ""]
				#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
				#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
				#[doc = ""]
				#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
				#[doc = "proposal."]
				#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
				#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "## Weight"]
				#[doc = "- `O(B + M + P1 + P2)` where:"]
				#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
				#[doc = "  - `M` is members-count (code- and governance-bounded)"]
				#[doc = "  - `P1` is the complexity of `proposal` preimage."]
				#[doc = "  - `P2` is proposal-count (code-bounded)"]
				#[doc = "- DB:"]
				#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
				#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
				#[doc = "   `O(P2)`)"]
				#[doc = " - any mutations done while executing `proposal` (`P1`)"]
				#[doc = "- up to 3 events"]
				#[doc = "# </weight>"]
				pub fn close(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: ::core::primitive::u64,
					length_bound: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Close, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Close>()? ==
						[
							242u8, 208u8, 108u8, 202u8, 24u8, 139u8, 8u8, 150u8, 108u8, 217u8,
							30u8, 209u8, 178u8, 1u8, 80u8, 25u8, 154u8, 146u8, 173u8, 172u8, 227u8,
							4u8, 140u8, 228u8, 58u8, 221u8, 189u8, 135u8, 203u8, 69u8, 105u8, 47u8,
						] {
						let call =
							Close { proposal_hash, index, proposal_weight_bound, length_bound };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
				#[doc = "state."]
				#[doc = ""]
				#[doc = "Must be called by the Root origin."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Complexity: O(P) where P is the number of max proposals"]
				#[doc = "DB Weight:"]
				#[doc = "* Reads: Proposals"]
				#[doc = "* Writes: Voting, Proposals, ProposalOf"]
				#[doc = "# </weight>"]
				pub fn disapprove_proposal(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						DisapproveProposal,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<DisapproveProposal>()? ==
						[
							199u8, 113u8, 221u8, 167u8, 60u8, 241u8, 77u8, 166u8, 205u8, 191u8,
							183u8, 121u8, 191u8, 206u8, 230u8, 212u8, 215u8, 219u8, 30u8, 51u8,
							123u8, 18u8, 17u8, 218u8, 77u8, 227u8, 197u8, 95u8, 232u8, 59u8, 169u8,
							133u8,
						] {
						let call = DisapproveProposal { proposal_hash };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
			#[doc = "`MemberCount`)."]
			pub struct Proposed {
				pub account: ::subxt::sp_core::crypto::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::sp_core::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::Event for Proposed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A motion (given hash) has been voted on by given account, leaving"]
			#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
			pub struct Voted {
				pub account: ::subxt::sp_core::crypto::AccountId32,
				pub proposal_hash: ::subxt::sp_core::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::Event for Voted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Voted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A motion was approved by the required threshold."]
			pub struct Approved {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Event for Approved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Approved";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A motion was not approved by the required threshold."]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Event for Disapproved {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
			pub struct Executed {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::Event for Executed {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "Executed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::Event for MemberExecuted {
				const PALLET: &'static str = "Council";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
			pub struct Closed {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
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
			pub struct ProposalOf<'a>(pub &'a ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for ProposalOf<'_> {
				const PALLET: &'static str = "Council";
				const STORAGE: &'static str = "ProposalOf";
				type Value = runtime_types::composable_runtime::Call;
				fn key(&self) -> ::subxt::StorageEntryKey {
					::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
						&self.0,
						::subxt::StorageHasher::Identity,
					)])
				}
			}
			pub struct Voting<'a>(pub &'a ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Voting<'_> {
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
				#[doc = " The hashes of the active proposals."]
				pub async fn proposals(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						::subxt::sp_core::H256,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Proposals>()? ==
						[
							174u8, 75u8, 108u8, 245u8, 86u8, 50u8, 107u8, 212u8, 244u8, 113u8,
							232u8, 168u8, 194u8, 33u8, 247u8, 97u8, 54u8, 115u8, 236u8, 189u8,
							59u8, 2u8, 252u8, 84u8, 199u8, 127u8, 197u8, 72u8, 23u8, 1u8, 118u8,
							95u8,
						] {
						let entry = Proposals;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub async fn proposal_of(
					&self,
					_0: &::subxt::sp_core::H256,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::composable_runtime::Call>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ProposalOf>()? ==
						[
							166u8, 244u8, 117u8, 141u8, 101u8, 4u8, 1u8, 80u8, 229u8, 76u8, 114u8,
							160u8, 46u8, 77u8, 205u8, 147u8, 0u8, 171u8, 47u8, 131u8, 149u8, 43u8,
							68u8, 106u8, 66u8, 194u8, 138u8, 174u8, 213u8, 246u8, 161u8, 122u8,
						] {
						let entry = ProposalOf(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub async fn proposal_of_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, ProposalOf<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ProposalOf>()? ==
						[
							166u8, 244u8, 117u8, 141u8, 101u8, 4u8, 1u8, 80u8, 229u8, 76u8, 114u8,
							160u8, 46u8, 77u8, 205u8, 147u8, 0u8, 171u8, 47u8, 131u8, 149u8, 43u8,
							68u8, 106u8, 66u8, 194u8, 138u8, 174u8, 213u8, 246u8, 161u8, 122u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub async fn voting(
					&self,
					_0: &::subxt::sp_core::H256,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_collective::Votes<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Voting>()? ==
						[
							145u8, 223u8, 203u8, 2u8, 137u8, 33u8, 22u8, 239u8, 175u8, 149u8,
							254u8, 185u8, 0u8, 139u8, 71u8, 134u8, 109u8, 95u8, 45u8, 75u8, 33u8,
							228u8, 127u8, 67u8, 53u8, 119u8, 188u8, 198u8, 11u8, 92u8, 4u8, 177u8,
						] {
						let entry = Voting(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub async fn voting_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Voting<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Voting>()? ==
						[
							145u8, 223u8, 203u8, 2u8, 137u8, 33u8, 22u8, 239u8, 175u8, 149u8,
							254u8, 185u8, 0u8, 139u8, 71u8, 134u8, 109u8, 95u8, 45u8, 75u8, 33u8,
							228u8, 127u8, 67u8, 53u8, 119u8, 188u8, 198u8, 11u8, 92u8, 4u8, 177u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Proposals so far."]
				pub async fn proposal_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<ProposalCount>()? ==
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						] {
						let entry = ProposalCount;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The current members of the collective. This is stored sorted (just by value)."]
				pub async fn members(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Members>()? ==
						[
							136u8, 91u8, 140u8, 173u8, 238u8, 221u8, 4u8, 132u8, 238u8, 99u8,
							195u8, 142u8, 10u8, 35u8, 210u8, 227u8, 22u8, 72u8, 218u8, 222u8,
							227u8, 51u8, 55u8, 31u8, 252u8, 78u8, 195u8, 11u8, 195u8, 242u8, 171u8,
							75u8,
						] {
						let entry = Members;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
				pub async fn prime(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Prime>()? ==
						[
							70u8, 101u8, 20u8, 160u8, 173u8, 87u8, 190u8, 85u8, 60u8, 249u8, 144u8,
							77u8, 175u8, 195u8, 51u8, 196u8, 234u8, 62u8, 243u8, 199u8, 126u8,
							12u8, 88u8, 252u8, 1u8, 210u8, 65u8, 210u8, 33u8, 19u8, 222u8, 11u8,
						] {
						let entry = Prime;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod council_membership {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct AddMember {
				pub who: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for AddMember {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "add_member";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct RemoveMember {
				pub who: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for RemoveMember {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "remove_member";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SwapMember {
				pub remove: ::subxt::sp_core::crypto::AccountId32,
				pub add: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for SwapMember {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "swap_member";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::Call for ResetMembers {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "reset_members";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ChangeKey {
				pub new: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for ChangeKey {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "change_key";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct SetPrime {
				pub who: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for SetPrime {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "set_prime";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ClearPrime;
			impl ::subxt::Call for ClearPrime {
				const PALLET: &'static str = "CouncilMembership";
				const FUNCTION: &'static str = "clear_prime";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub fn add_member(
					&self,
					who: ::subxt::sp_core::crypto::AccountId32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						AddMember,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<AddMember>()? ==
						[
							1u8, 149u8, 115u8, 222u8, 93u8, 9u8, 208u8, 58u8, 22u8, 148u8, 215u8,
							141u8, 204u8, 48u8, 107u8, 210u8, 202u8, 165u8, 43u8, 159u8, 45u8,
							161u8, 255u8, 127u8, 225u8, 100u8, 161u8, 195u8, 197u8, 206u8, 57u8,
							166u8,
						] {
						let call = AddMember { who };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub fn remove_member(
					&self,
					who: ::subxt::sp_core::crypto::AccountId32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						RemoveMember,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<RemoveMember>()? ==
						[
							137u8, 249u8, 148u8, 139u8, 147u8, 47u8, 226u8, 228u8, 139u8, 219u8,
							109u8, 128u8, 254u8, 51u8, 227u8, 154u8, 105u8, 91u8, 229u8, 69u8,
							217u8, 241u8, 107u8, 229u8, 41u8, 202u8, 228u8, 227u8, 160u8, 162u8,
							45u8, 211u8,
						] {
						let call = RemoveMember { who };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub fn swap_member(
					&self,
					remove: ::subxt::sp_core::crypto::AccountId32,
					add: ::subxt::sp_core::crypto::AccountId32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SwapMember,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SwapMember>()? ==
						[
							159u8, 62u8, 254u8, 117u8, 56u8, 185u8, 99u8, 29u8, 146u8, 210u8, 40u8,
							77u8, 169u8, 224u8, 215u8, 34u8, 106u8, 95u8, 204u8, 109u8, 72u8, 67u8,
							11u8, 183u8, 33u8, 84u8, 133u8, 4u8, 5u8, 13u8, 188u8, 123u8,
						] {
						let call = SwapMember { remove, add };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ResetMembers,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ResetMembers>()? ==
						[
							246u8, 84u8, 91u8, 191u8, 61u8, 245u8, 171u8, 80u8, 18u8, 120u8, 61u8,
							86u8, 23u8, 115u8, 161u8, 203u8, 128u8, 34u8, 166u8, 128u8, 33u8, 28u8,
							229u8, 81u8, 103u8, 217u8, 173u8, 151u8, 31u8, 118u8, 151u8, 217u8,
						] {
						let call = ResetMembers { members };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub fn change_key(
					&self,
					new: ::subxt::sp_core::crypto::AccountId32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ChangeKey,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ChangeKey>()? ==
						[
							198u8, 93u8, 41u8, 52u8, 241u8, 11u8, 225u8, 82u8, 30u8, 114u8, 111u8,
							204u8, 13u8, 31u8, 34u8, 82u8, 171u8, 58u8, 180u8, 65u8, 3u8, 246u8,
							33u8, 167u8, 200u8, 23u8, 150u8, 235u8, 130u8, 172u8, 202u8, 216u8,
						] {
						let call = ChangeKey { new };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn set_prime(
					&self,
					who: ::subxt::sp_core::crypto::AccountId32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetPrime,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetPrime>()? ==
						[
							185u8, 53u8, 61u8, 154u8, 234u8, 77u8, 195u8, 126u8, 19u8, 39u8, 78u8,
							205u8, 109u8, 210u8, 137u8, 245u8, 128u8, 110u8, 2u8, 201u8, 20u8,
							153u8, 146u8, 177u8, 4u8, 144u8, 229u8, 125u8, 91u8, 131u8, 199u8,
							15u8,
						] {
						let call = SetPrime { who };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn clear_prime(
					&self,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ClearPrime,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ClearPrime>()? ==
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						] {
						let call = ClearPrime {};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::Event for MemberAdded {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::Event for MemberRemoved {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::Event for MembersSwapped {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::Event for MembersReset {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::Event for KeyChanged {
				const PALLET: &'static str = "CouncilMembership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
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
				#[doc = " The current membership, stored as an ordered Vec."]
				pub async fn members(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Members>()? ==
						[
							136u8, 91u8, 140u8, 173u8, 238u8, 221u8, 4u8, 132u8, 238u8, 99u8,
							195u8, 142u8, 10u8, 35u8, 210u8, 227u8, 22u8, 72u8, 218u8, 222u8,
							227u8, 51u8, 55u8, 31u8, 252u8, 78u8, 195u8, 11u8, 195u8, 242u8, 171u8,
							75u8,
						] {
						let entry = Members;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The current prime member, if one exists."]
				pub async fn prime(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Prime>()? ==
						[
							70u8, 101u8, 20u8, 160u8, 173u8, 87u8, 190u8, 85u8, 60u8, 249u8, 144u8,
							77u8, 175u8, 195u8, 51u8, 196u8, 234u8, 62u8, 243u8, 199u8, 126u8,
							12u8, 88u8, 252u8, 1u8, 210u8, 65u8, 210u8, 33u8, 19u8, 222u8, 11u8,
						] {
						let entry = Prime;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod treasury {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct RejectProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			impl ::subxt::Call for RejectProposal {
				const PALLET: &'static str = "Treasury";
				const FUNCTION: &'static str = "reject_proposal";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ApproveProposal {
				#[codec(compact)]
				pub proposal_id: ::core::primitive::u32,
			}
			impl ::subxt::Call for ApproveProposal {
				const PALLET: &'static str = "Treasury";
				const FUNCTION: &'static str = "approve_proposal";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
				#[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
				#[doc = "proposal is awarded."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)"]
				#[doc = "- DbReads: `ProposalCount`, `origin account`"]
				#[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
				#[doc = "# </weight>"]
				pub fn propose_spend(
					&self,
					value: ::core::primitive::u128,
					beneficiary: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ProposeSpend,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ProposeSpend>()? ==
						[
							28u8, 133u8, 252u8, 219u8, 50u8, 86u8, 29u8, 221u8, 188u8, 4u8, 33u8,
							236u8, 171u8, 116u8, 77u8, 164u8, 7u8, 55u8, 138u8, 37u8, 51u8, 213u8,
							224u8, 6u8, 25u8, 245u8, 137u8, 136u8, 196u8, 86u8, 28u8, 154u8,
						] {
						let call = ProposeSpend { value, beneficiary };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Reject a proposed spend. The original deposit will be slashed."]
				#[doc = ""]
				#[doc = "May only be called from `T::RejectOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)"]
				#[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
				#[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
				#[doc = "# </weight>"]
				pub fn reject_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						RejectProposal,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<RejectProposal>()? ==
						[
							153u8, 238u8, 223u8, 212u8, 86u8, 178u8, 184u8, 150u8, 117u8, 91u8,
							69u8, 30u8, 196u8, 134u8, 56u8, 54u8, 236u8, 145u8, 202u8, 139u8,
							135u8, 254u8, 80u8, 189u8, 40u8, 56u8, 148u8, 108u8, 42u8, 118u8, 74u8,
							242u8,
						] {
						let call = RejectProposal { proposal_id };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
				#[doc = "and the original deposit will be returned."]
				#[doc = ""]
				#[doc = "May only be called from `T::ApproveOrigin`."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(1)."]
				#[doc = "- DbReads: `Proposals`, `Approvals`"]
				#[doc = "- DbWrite: `Approvals`"]
				#[doc = "# </weight>"]
				pub fn approve_proposal(
					&self,
					proposal_id: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ApproveProposal,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ApproveProposal>()? ==
						[
							191u8, 81u8, 78u8, 230u8, 230u8, 192u8, 144u8, 232u8, 81u8, 70u8,
							227u8, 212u8, 194u8, 228u8, 231u8, 147u8, 57u8, 222u8, 156u8, 77u8,
							173u8, 60u8, 92u8, 84u8, 255u8, 64u8, 240u8, 45u8, 131u8, 200u8, 206u8,
							231u8,
						] {
						let call = ApproveProposal { proposal_id };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_treasury::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "New proposal. \\[proposal_index\\]"]
			pub struct Proposed(pub ::core::primitive::u32);
			impl ::subxt::Event for Proposed {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "We have ended a spend period and will now allocate funds. \\[budget_remaining\\]"]
			pub struct Spending(pub ::core::primitive::u128);
			impl ::subxt::Event for Spending {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Spending";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some funds have been allocated. \\[proposal_index, award, beneficiary\\]"]
			pub struct Awarded(
				pub ::core::primitive::u32,
				pub ::core::primitive::u128,
				pub ::subxt::sp_core::crypto::AccountId32,
			);
			impl ::subxt::Event for Awarded {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Awarded";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal was rejected; funds were slashed. \\[proposal_index, slashed\\]"]
			pub struct Rejected(pub ::core::primitive::u32, pub ::core::primitive::u128);
			impl ::subxt::Event for Rejected {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rejected";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some of our funds have been burnt. \\[burn\\]"]
			pub struct Burnt(pub ::core::primitive::u128);
			impl ::subxt::Event for Burnt {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Burnt";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
			#[doc = "\\[budget_remaining\\]"]
			pub struct Rollover(pub ::core::primitive::u128);
			impl ::subxt::Event for Rollover {
				const PALLET: &'static str = "Treasury";
				const EVENT: &'static str = "Rollover";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "Some funds have been deposited. \\[deposit\\]"]
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
			pub struct Proposals<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for Proposals<'_> {
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
				#[doc = " Number of proposals that have been made."]
				pub async fn proposal_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<ProposalCount>()? ==
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						] {
						let entry = ProposalCount;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Proposals that have been made."]
				pub async fn proposals(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_treasury::Proposal<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Proposals>()? ==
						[
							46u8, 242u8, 203u8, 56u8, 166u8, 200u8, 95u8, 110u8, 47u8, 71u8, 71u8,
							45u8, 12u8, 93u8, 222u8, 120u8, 40u8, 130u8, 29u8, 236u8, 189u8, 49u8,
							115u8, 238u8, 135u8, 64u8, 252u8, 171u8, 29u8, 229u8, 63u8, 31u8,
						] {
						let entry = Proposals(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Proposals that have been made."]
				pub async fn proposals_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Proposals<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Proposals>()? ==
						[
							46u8, 242u8, 203u8, 56u8, 166u8, 200u8, 95u8, 110u8, 47u8, 71u8, 71u8,
							45u8, 12u8, 93u8, 222u8, 120u8, 40u8, 130u8, 29u8, 236u8, 189u8, 49u8,
							115u8, 238u8, 135u8, 64u8, 252u8, 171u8, 29u8, 229u8, 63u8, 31u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Proposal indices that have been approved but not yet awarded."]
				pub async fn approvals(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						::core::primitive::u32,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Approvals>()? ==
						[
							152u8, 185u8, 127u8, 54u8, 169u8, 155u8, 124u8, 22u8, 142u8, 132u8,
							254u8, 197u8, 162u8, 152u8, 15u8, 18u8, 192u8, 138u8, 196u8, 231u8,
							234u8, 178u8, 111u8, 181u8, 20u8, 131u8, 149u8, 36u8, 222u8, 4u8,
							119u8, 135u8,
						] {
						let entry = Approvals;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " Fraction of a proposal's value that should be bonded in order to place the proposal."]
				#[doc = " An accepted proposal gets these back. A rejected proposal does not."]
				pub fn proposal_bond(
					&self,
				) -> ::core::result::Result<
					runtime_types::sp_arithmetic::per_things::Permill,
					::subxt::BasicError,
				> {
					if self.client.metadata().constant_hash("Treasury", "ProposalBond")? ==
						[
							254u8, 112u8, 56u8, 108u8, 71u8, 90u8, 128u8, 114u8, 54u8, 239u8, 87u8,
							235u8, 71u8, 56u8, 11u8, 132u8, 179u8, 134u8, 115u8, 139u8, 109u8,
							136u8, 59u8, 69u8, 108u8, 160u8, 18u8, 120u8, 34u8, 213u8, 166u8, 13u8,
						] {
						let pallet = self.client.metadata().pallet("Treasury")?;
						let constant = pallet.constant("ProposalBond")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Minimum amount of funds that should be placed in a deposit for making a proposal."]
				pub fn proposal_bond_minimum(
					&self,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Treasury", "ProposalBondMinimum")? ==
						[
							74u8, 247u8, 241u8, 153u8, 187u8, 209u8, 93u8, 143u8, 54u8, 172u8,
							209u8, 136u8, 121u8, 161u8, 201u8, 146u8, 41u8, 127u8, 209u8, 246u8,
							38u8, 46u8, 182u8, 92u8, 201u8, 184u8, 189u8, 80u8, 231u8, 30u8, 185u8,
							38u8,
						] {
						let pallet = self.client.metadata().pallet("Treasury")?;
						let constant = pallet.constant("ProposalBondMinimum")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Period between successive spends."]
				pub fn spend_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Treasury", "SpendPeriod")? ==
						[
							106u8, 24u8, 10u8, 227u8, 158u8, 110u8, 248u8, 16u8, 18u8, 227u8,
							219u8, 123u8, 105u8, 207u8, 255u8, 111u8, 78u8, 66u8, 65u8, 247u8,
							72u8, 82u8, 23u8, 112u8, 47u8, 149u8, 61u8, 255u8, 19u8, 104u8, 48u8,
							188u8,
						] {
						let pallet = self.client.metadata().pallet("Treasury")?;
						let constant = pallet.constant("SpendPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Percentage of spare funds (if any) that are burnt per spend period."]
				pub fn burn(
					&self,
				) -> ::core::result::Result<
					runtime_types::sp_arithmetic::per_things::Permill,
					::subxt::BasicError,
				> {
					if self.client.metadata().constant_hash("Treasury", "Burn")? ==
						[
							175u8, 244u8, 202u8, 183u8, 30u8, 101u8, 60u8, 205u8, 78u8, 55u8,
							138u8, 212u8, 254u8, 224u8, 88u8, 83u8, 244u8, 168u8, 90u8, 228u8,
							99u8, 130u8, 249u8, 104u8, 94u8, 160u8, 197u8, 38u8, 185u8, 166u8,
							105u8, 12u8,
						] {
						let pallet = self.client.metadata().pallet("Treasury")?;
						let constant = pallet.constant("Burn")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The treasury's pallet id, used for deriving its sovereign account ID."]
				pub fn pallet_id(
					&self,
				) -> ::core::result::Result<
					runtime_types::frame_support::PalletId,
					::subxt::BasicError,
				> {
					if self.client.metadata().constant_hash("Treasury", "PalletId")? ==
						[
							215u8, 38u8, 251u8, 239u8, 48u8, 208u8, 254u8, 213u8, 157u8, 17u8,
							124u8, 203u8, 241u8, 60u8, 16u8, 113u8, 71u8, 85u8, 123u8, 186u8,
							182u8, 163u8, 95u8, 0u8, 122u8, 200u8, 129u8, 147u8, 145u8, 31u8, 24u8,
							41u8,
						] {
						let pallet = self.client.metadata().pallet("Treasury")?;
						let constant = pallet.constant("PalletId")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The maximum number of approvals that can wait in the spending queue."]
				pub fn max_approvals(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Treasury", "MaxApprovals")? ==
						[
							25u8, 185u8, 151u8, 218u8, 193u8, 64u8, 148u8, 243u8, 190u8, 70u8,
							31u8, 23u8, 181u8, 189u8, 127u8, 10u8, 142u8, 24u8, 148u8, 0u8, 250u8,
							127u8, 128u8, 131u8, 6u8, 129u8, 154u8, 118u8, 238u8, 176u8, 197u8,
							51u8,
						] {
						let pallet = self.client.metadata().pallet("Treasury")?;
						let constant = pallet.constant("MaxApprovals")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod democracy {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Propose {
				pub proposal_hash: ::subxt::sp_core::H256,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			impl ::subxt::Call for Propose {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "propose";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct EmergencyCancel {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Call for EmergencyCancel {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "emergency_cancel";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ExternalPropose {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for ExternalPropose {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "external_propose";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ExternalProposeMajority {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for ExternalProposeMajority {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "external_propose_majority";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ExternalProposeDefault {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for ExternalProposeDefault {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "external_propose_default";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct FastTrack {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub voting_period: ::core::primitive::u32,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::Call for FastTrack {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "fast_track";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct VetoExternal {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
			impl ::subxt::Call for VetoExternal {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "veto_external";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct CancelReferendum {
				#[codec(compact)]
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Call for CancelReferendum {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "cancel_referendum";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct CancelQueued {
				pub which: ::core::primitive::u32,
			}
			impl ::subxt::Call for CancelQueued {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "cancel_queued";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Delegate {
				pub to: ::subxt::sp_core::crypto::AccountId32,
				pub conviction: runtime_types::pallet_democracy::conviction::Conviction,
				pub balance: ::core::primitive::u128,
			}
			impl ::subxt::Call for Delegate {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "delegate";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Undelegate;
			impl ::subxt::Call for Undelegate {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "undelegate";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ClearPublicProposals;
			impl ::subxt::Call for ClearPublicProposals {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "clear_public_proposals";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct NotePreimage {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NotePreimage {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_preimage";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct NotePreimageOperational {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NotePreimageOperational {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_preimage_operational";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct NoteImminentPreimage {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NoteImminentPreimage {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_imminent_preimage";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct NoteImminentPreimageOperational {
				pub encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for NoteImminentPreimageOperational {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "note_imminent_preimage_operational";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ReapPreimage {
				pub proposal_hash: ::subxt::sp_core::H256,
				#[codec(compact)]
				pub proposal_len_upper_bound: ::core::primitive::u32,
			}
			impl ::subxt::Call for ReapPreimage {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "reap_preimage";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Unlock {
				pub target: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Call for Unlock {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "unlock";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct RemoveVote {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for RemoveVote {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "remove_vote";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct RemoveOtherVote {
				pub target: ::subxt::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for RemoveOtherVote {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "remove_other_vote";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct EnactProposal {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for EnactProposal {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "enact_proposal";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Blacklist {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::Call for Blacklist {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "blacklist";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct CancelProposal {
				#[codec(compact)]
				pub prop_index: ::core::primitive::u32,
			}
			impl ::subxt::Call for CancelProposal {
				const PALLET: &'static str = "Democracy";
				const FUNCTION: &'static str = "cancel_proposal";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Propose a sensitive action to be taken."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the sender must"]
				#[doc = "have funds to cover the deposit."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The hash of the proposal preimage."]
				#[doc = "- `value`: The amount of deposit (must be at least `MinimumDeposit`)."]
				#[doc = ""]
				#[doc = "Emits `Proposed`."]
				#[doc = ""]
				#[doc = "Weight: `O(p)`"]
				pub fn propose(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					value: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Propose,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Propose>()? ==
						[
							149u8, 60u8, 16u8, 143u8, 114u8, 16u8, 124u8, 96u8, 97u8, 5u8, 176u8,
							137u8, 188u8, 164u8, 65u8, 145u8, 142u8, 104u8, 74u8, 120u8, 248u8,
							90u8, 109u8, 112u8, 29u8, 226u8, 208u8, 230u8, 101u8, 8u8, 79u8, 12u8,
						] {
						let call = Propose { proposal_hash, value };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Signals agreement with a particular proposal."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the sender"]
				#[doc = "must have funds to cover the deposit, equal to the original deposit."]
				#[doc = ""]
				#[doc = "- `proposal`: The index of the proposal to second."]
				#[doc = "- `seconds_upper_bound`: an upper bound on the current number of seconds on this"]
				#[doc = "  proposal. Extrinsic is weighted according to this value with no refund."]
				#[doc = ""]
				#[doc = "Weight: `O(S)` where S is the number of seconds a proposal already has."]
				pub fn second(
					&self,
					proposal: ::core::primitive::u32,
					seconds_upper_bound: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Second, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Second>()? ==
						[
							37u8, 226u8, 138u8, 26u8, 138u8, 46u8, 39u8, 147u8, 22u8, 32u8, 245u8,
							40u8, 49u8, 228u8, 218u8, 225u8, 72u8, 89u8, 37u8, 90u8, 132u8, 31u8,
							52u8, 22u8, 234u8, 124u8, 254u8, 223u8, 56u8, 215u8, 255u8, 79u8,
						] {
						let call = Second { proposal, seconds_upper_bound };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
				#[doc = "otherwise it is a vote to keep the status quo."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `ref_index`: The index of the referendum to vote for."]
				#[doc = "- `vote`: The vote configuration."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter has voted on."]
				pub fn vote(
					&self,
					ref_index: ::core::primitive::u32,
					vote: runtime_types::pallet_democracy::vote::AccountVote<
						::core::primitive::u128,
					>,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Vote, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Vote>()? ==
						[
							1u8, 235u8, 77u8, 58u8, 54u8, 224u8, 30u8, 168u8, 150u8, 169u8, 20u8,
							172u8, 137u8, 191u8, 189u8, 184u8, 28u8, 118u8, 204u8, 233u8, 146u8,
							212u8, 45u8, 139u8, 58u8, 175u8, 231u8, 169u8, 43u8, 164u8, 149u8,
							16u8,
						] {
						let call = Vote { ref_index, vote };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Schedule an emergency cancellation of a referendum. Cannot happen twice to the same"]
				#[doc = "referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `CancellationOrigin`."]
				#[doc = ""]
				#[doc = "-`ref_index`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`."]
				pub fn emergency_cancel(
					&self,
					ref_index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						EmergencyCancel,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<EmergencyCancel>()? ==
						[
							4u8, 129u8, 205u8, 102u8, 202u8, 197u8, 75u8, 155u8, 24u8, 125u8,
							157u8, 73u8, 50u8, 243u8, 173u8, 103u8, 49u8, 60u8, 50u8, 63u8, 54u8,
							40u8, 34u8, 227u8, 29u8, 247u8, 179u8, 102u8, 107u8, 177u8, 117u8,
							161u8,
						] {
						let call = EmergencyCancel { ref_index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Schedule a referendum to be tabled once it is legal to schedule an external"]
				#[doc = "referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `ExternalOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Weight: `O(V)` with V number of vetoers in the blacklist of proposal."]
				#[doc = "  Decoding vec of length V. Charged as maximum"]
				pub fn external_propose(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ExternalPropose,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ExternalPropose>()? ==
						[
							50u8, 82u8, 155u8, 206u8, 57u8, 61u8, 64u8, 43u8, 30u8, 71u8, 89u8,
							91u8, 221u8, 46u8, 15u8, 222u8, 15u8, 211u8, 56u8, 176u8, 84u8, 225u8,
							192u8, 92u8, 253u8, 56u8, 207u8, 29u8, 252u8, 77u8, 245u8, 113u8,
						] {
						let call = ExternalPropose { proposal_hash };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Schedule a majority-carries referendum to be tabled next once it is legal to schedule"]
				#[doc = "an external referendum."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `ExternalMajorityOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
				#[doc = "pre-scheduled `external_propose` call."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn external_propose_majority(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ExternalProposeMajority,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ExternalProposeMajority>()? ==
						[
							18u8, 92u8, 204u8, 120u8, 189u8, 60u8, 223u8, 166u8, 213u8, 49u8, 20u8,
							131u8, 202u8, 1u8, 87u8, 226u8, 168u8, 156u8, 144u8, 110u8, 118u8,
							125u8, 81u8, 111u8, 229u8, 244u8, 89u8, 93u8, 202u8, 140u8, 16u8,
							220u8,
						] {
						let call = ExternalProposeMajority { proposal_hash };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Schedule a negative-turnout-bias referendum to be tabled next once it is legal to"]
				#[doc = "schedule an external referendum."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `ExternalDefaultOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
				#[doc = ""]
				#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
				#[doc = "pre-scheduled `external_propose` call."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn external_propose_default(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ExternalProposeDefault,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ExternalProposeDefault>()? ==
						[
							51u8, 75u8, 236u8, 51u8, 53u8, 39u8, 26u8, 231u8, 212u8, 191u8, 175u8,
							233u8, 181u8, 156u8, 210u8, 221u8, 181u8, 182u8, 113u8, 69u8, 171u8,
							70u8, 219u8, 133u8, 88u8, 78u8, 87u8, 228u8, 177u8, 53u8, 111u8, 115u8,
						] {
						let call = ExternalProposeDefault { proposal_hash };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Schedule the currently externally-proposed majority-carries referendum to be tabled"]
				#[doc = "immediately. If there is no externally-proposed referendum currently, or if there is one"]
				#[doc = "but it is not a majority-carries referendum then it fails."]
				#[doc = ""]
				#[doc = "The dispatch of this call must be `FastTrackOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The hash of the current external proposal."]
				#[doc = "- `voting_period`: The period that is allowed for voting on this proposal. Increased to"]
				#[doc = "  `FastTrackVotingPeriod` if too low."]
				#[doc = "- `delay`: The number of block after voting has ended in approval and this should be"]
				#[doc = "  enacted. This doesn't have a minimum amount."]
				#[doc = ""]
				#[doc = "Emits `Started`."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`"]
				pub fn fast_track(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					voting_period: ::core::primitive::u32,
					delay: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						FastTrack,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<FastTrack>()? ==
						[
							232u8, 255u8, 150u8, 13u8, 151u8, 28u8, 253u8, 37u8, 183u8, 127u8,
							53u8, 228u8, 160u8, 11u8, 223u8, 48u8, 74u8, 5u8, 37u8, 3u8, 84u8,
							224u8, 79u8, 172u8, 120u8, 220u8, 158u8, 191u8, 127u8, 55u8, 126u8,
							135u8,
						] {
						let call = FastTrack { proposal_hash, voting_period, delay };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Veto and blacklist the external proposal hash."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `VetoOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of the proposal to veto and blacklist."]
				#[doc = ""]
				#[doc = "Emits `Vetoed`."]
				#[doc = ""]
				#[doc = "Weight: `O(V + log(V))` where V is number of `existing vetoers`"]
				pub fn veto_external(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						VetoExternal,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<VetoExternal>()? ==
						[
							230u8, 207u8, 43u8, 137u8, 173u8, 97u8, 143u8, 183u8, 193u8, 78u8,
							252u8, 104u8, 237u8, 32u8, 151u8, 164u8, 91u8, 247u8, 233u8, 36u8,
							198u8, 88u8, 63u8, 176u8, 77u8, 87u8, 26u8, 242u8, 211u8, 47u8, 193u8,
							180u8,
						] {
						let call = VetoExternal { proposal_hash };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Remove a referendum."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `ref_index`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "# Weight: `O(1)`."]
				pub fn cancel_referendum(
					&self,
					ref_index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						CancelReferendum,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<CancelReferendum>()? ==
						[
							107u8, 144u8, 114u8, 224u8, 39u8, 217u8, 156u8, 202u8, 62u8, 4u8,
							196u8, 63u8, 145u8, 196u8, 107u8, 241u8, 3u8, 61u8, 202u8, 20u8, 123u8,
							158u8, 153u8, 45u8, 192u8, 192u8, 244u8, 42u8, 224u8, 23u8, 243u8,
							225u8,
						] {
						let call = CancelReferendum { ref_index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Cancel a proposal queued for enactment."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `which`: The index of the referendum to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(D)` where `D` is the items in the dispatch queue. Weighted as `D = 10`."]
				pub fn cancel_queued(
					&self,
					which: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						CancelQueued,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<CancelQueued>()? ==
						[
							130u8, 218u8, 212u8, 143u8, 89u8, 134u8, 207u8, 161u8, 165u8, 202u8,
							237u8, 237u8, 81u8, 125u8, 165u8, 147u8, 222u8, 198u8, 236u8, 1u8,
							223u8, 74u8, 200u8, 6u8, 208u8, 128u8, 215u8, 50u8, 46u8, 117u8, 16u8,
							143u8,
						] {
						let call = CancelQueued { which };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Delegate the voting power (with some given conviction) of the sending account."]
				#[doc = ""]
				#[doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"]
				#[doc = "time appropriate for the conviction's lock period."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"]
				#[doc = "  - be delegating already; or"]
				#[doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"]
				#[doc = "    through `reap_vote` or `unvote`)."]
				#[doc = ""]
				#[doc = "- `to`: The account whose voting the `target` account's voting power will follow."]
				#[doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"]
				#[doc = "  account is undelegated, the funds will be locked for the corresponding period."]
				#[doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"]
				#[doc = "  be more than the account's current balance."]
				#[doc = ""]
				#[doc = "Emits `Delegated`."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
				#[doc = "  voted on. Weight is charged as if maximum votes."]
				pub fn delegate(
					&self,
					to: ::subxt::sp_core::crypto::AccountId32,
					conviction: runtime_types::pallet_democracy::conviction::Conviction,
					balance: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Delegate,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Delegate>()? ==
						[
							33u8, 155u8, 180u8, 53u8, 39u8, 251u8, 59u8, 100u8, 16u8, 124u8, 209u8,
							40u8, 42u8, 152u8, 3u8, 109u8, 97u8, 211u8, 129u8, 151u8, 82u8, 45u8,
							16u8, 98u8, 114u8, 250u8, 145u8, 176u8, 244u8, 39u8, 64u8, 11u8,
						] {
						let call = Delegate { to, conviction, balance };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Undelegate the voting power of the sending account."]
				#[doc = ""]
				#[doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"]
				#[doc = "of the conviction with which the delegation was issued."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"]
				#[doc = "currently delegating."]
				#[doc = ""]
				#[doc = "Emits `Undelegated`."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
				#[doc = "  voted on. Weight is charged as if maximum votes."]
				pub fn undelegate(
					&self,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Undelegate,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Undelegate>()? ==
						[
							165u8, 40u8, 183u8, 209u8, 57u8, 153u8, 111u8, 29u8, 114u8, 109u8,
							107u8, 235u8, 97u8, 61u8, 53u8, 155u8, 44u8, 245u8, 28u8, 220u8, 56u8,
							134u8, 43u8, 122u8, 248u8, 156u8, 191u8, 154u8, 4u8, 121u8, 152u8,
							153u8,
						] {
						let call = Undelegate {};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Clears all public proposals."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Root_."]
				#[doc = ""]
				#[doc = "Weight: `O(1)`."]
				pub fn clear_public_proposals(
					&self,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ClearPublicProposals,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ClearPublicProposals>()? ==
						[
							59u8, 126u8, 254u8, 223u8, 252u8, 225u8, 75u8, 185u8, 188u8, 181u8,
							42u8, 179u8, 211u8, 73u8, 12u8, 141u8, 243u8, 197u8, 46u8, 130u8,
							215u8, 196u8, 225u8, 88u8, 48u8, 199u8, 231u8, 249u8, 195u8, 53u8,
							184u8, 204u8,
						] {
						let call = ClearPublicProposals {};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Register the preimage for an upcoming proposal. This doesn't require the proposal to be"]
				#[doc = "in the dispatch queue but does require a deposit, returned once enacted."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `encoded_proposal`: The preimage of a proposal."]
				#[doc = ""]
				#[doc = "Emits `PreimageNoted`."]
				#[doc = ""]
				#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
				pub fn note_preimage(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						NotePreimage,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<NotePreimage>()? ==
						[
							121u8, 179u8, 204u8, 32u8, 104u8, 133u8, 99u8, 153u8, 226u8, 190u8,
							89u8, 121u8, 232u8, 154u8, 89u8, 133u8, 124u8, 222u8, 237u8, 39u8,
							50u8, 128u8, 80u8, 115u8, 186u8, 180u8, 151u8, 139u8, 73u8, 112u8,
							148u8, 232u8,
						] {
						let call = NotePreimage { encoded_proposal };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Same as `note_preimage` but origin is `OperationalPreimageOrigin`."]
				pub fn note_preimage_operational(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						NotePreimageOperational,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<NotePreimageOperational>()? ==
						[
							102u8, 20u8, 213u8, 32u8, 64u8, 28u8, 150u8, 241u8, 173u8, 182u8,
							201u8, 70u8, 52u8, 211u8, 95u8, 211u8, 127u8, 12u8, 249u8, 57u8, 128u8,
							64u8, 185u8, 239u8, 255u8, 191u8, 203u8, 222u8, 123u8, 187u8, 106u8,
							12u8,
						] {
						let call = NotePreimageOperational { encoded_proposal };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Register the preimage for an upcoming proposal. This requires the proposal to be"]
				#[doc = "in the dispatch queue. No deposit is needed. When this call is successful, i.e."]
				#[doc = "the preimage has not been uploaded before and matches some imminent proposal,"]
				#[doc = "no fee is paid."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `encoded_proposal`: The preimage of a proposal."]
				#[doc = ""]
				#[doc = "Emits `PreimageNoted`."]
				#[doc = ""]
				#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
				pub fn note_imminent_preimage(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						NoteImminentPreimage,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<NoteImminentPreimage>()? ==
						[
							240u8, 77u8, 42u8, 178u8, 110u8, 117u8, 152u8, 158u8, 64u8, 26u8, 49u8,
							37u8, 177u8, 178u8, 203u8, 227u8, 23u8, 251u8, 242u8, 112u8, 184u8,
							234u8, 95u8, 73u8, 86u8, 37u8, 148u8, 150u8, 6u8, 50u8, 239u8, 64u8,
						] {
						let call = NoteImminentPreimage { encoded_proposal };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Same as `note_imminent_preimage` but origin is `OperationalPreimageOrigin`."]
				pub fn note_imminent_preimage_operational(
					&self,
					encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						NoteImminentPreimageOperational,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<NoteImminentPreimageOperational>()? ==
						[
							119u8, 17u8, 140u8, 81u8, 7u8, 103u8, 162u8, 112u8, 160u8, 179u8,
							116u8, 34u8, 126u8, 150u8, 64u8, 117u8, 93u8, 225u8, 197u8, 40u8, 62u8,
							238u8, 174u8, 63u8, 148u8, 248u8, 214u8, 212u8, 228u8, 86u8, 87u8,
							195u8,
						] {
						let call = NoteImminentPreimageOperational { encoded_proposal };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Remove an expired proposal preimage and collect the deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The preimage hash of a proposal."]
				#[doc = "- `proposal_length_upper_bound`: an upper bound on length of the proposal. Extrinsic is"]
				#[doc = "  weighted according to this value with no refund."]
				#[doc = ""]
				#[doc = "This will only work after `VotingPeriod` blocks from the time that the preimage was"]
				#[doc = "noted, if it's the same account doing it. If it's a different account, then it'll only"]
				#[doc = "work an additional `EnactmentPeriod` later."]
				#[doc = ""]
				#[doc = "Emits `PreimageReaped`."]
				#[doc = ""]
				#[doc = "Weight: `O(D)` where D is length of proposal."]
				pub fn reap_preimage(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					proposal_len_upper_bound: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ReapPreimage,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ReapPreimage>()? ==
						[
							45u8, 191u8, 46u8, 19u8, 87u8, 216u8, 48u8, 29u8, 124u8, 205u8, 39u8,
							178u8, 158u8, 95u8, 163u8, 116u8, 232u8, 58u8, 6u8, 242u8, 52u8, 215u8,
							251u8, 49u8, 1u8, 234u8, 99u8, 142u8, 76u8, 182u8, 134u8, 173u8,
						] {
						let call = ReapPreimage { proposal_hash, proposal_len_upper_bound };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Unlock tokens that have an expired lock."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `target`: The account to remove the lock on."]
				#[doc = ""]
				#[doc = "Weight: `O(R)` with R number of vote of target."]
				pub fn unlock(
					&self,
					target: ::subxt::sp_core::crypto::AccountId32,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Unlock, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Unlock>()? ==
						[
							106u8, 17u8, 189u8, 71u8, 208u8, 26u8, 49u8, 71u8, 162u8, 196u8, 126u8,
							192u8, 242u8, 239u8, 77u8, 196u8, 62u8, 171u8, 58u8, 176u8, 157u8,
							81u8, 65u8, 246u8, 210u8, 43u8, 1u8, 226u8, 143u8, 149u8, 210u8, 192u8,
						] {
						let call = Unlock { target };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Remove a vote for a referendum."]
				#[doc = ""]
				#[doc = "If:"]
				#[doc = "- the referendum was cancelled, or"]
				#[doc = "- the referendum is ongoing, or"]
				#[doc = "- the referendum has ended such that"]
				#[doc = "  - the vote of the account was in opposition to the result; or"]
				#[doc = "  - there was no conviction to the account's vote; or"]
				#[doc = "  - the account made a split vote"]
				#[doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"]
				#[doc = "funds being available."]
				#[doc = ""]
				#[doc = "If, however, the referendum has ended and:"]
				#[doc = "- it finished corresponding to the vote of the account, and"]
				#[doc = "- the account made a standard vote with conviction, and"]
				#[doc = "- the lock period of the conviction is not over"]
				#[doc = "...then the lock will be aggregated into the overall account's lock, which may involve"]
				#[doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"]
				#[doc = "of both the amount locked and the time is it locked for)."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"]
				#[doc = "registered for referendum `index`."]
				#[doc = ""]
				#[doc = "- `index`: The index of referendum of the vote to be removed."]
				#[doc = ""]
				#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
				#[doc = "  Weight is calculated for the maximum number of vote."]
				pub fn remove_vote(
					&self,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						RemoveVote,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<RemoveVote>()? ==
						[
							33u8, 72u8, 14u8, 166u8, 152u8, 18u8, 232u8, 153u8, 163u8, 96u8, 146u8,
							180u8, 98u8, 155u8, 119u8, 75u8, 247u8, 175u8, 246u8, 183u8, 182u8,
							108u8, 250u8, 80u8, 148u8, 86u8, 255u8, 59u8, 93u8, 197u8, 209u8,
							226u8,
						] {
						let call = RemoveVote { index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Remove a vote for a referendum."]
				#[doc = ""]
				#[doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"]
				#[doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"]
				#[doc = "either because the referendum was cancelled, because the voter lost the referendum or"]
				#[doc = "because the conviction period is over."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `target`: The account of the vote to be removed; this account must have voted for"]
				#[doc = "  referendum `index`."]
				#[doc = "- `index`: The index of referendum of the vote to be removed."]
				#[doc = ""]
				#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
				#[doc = "  Weight is calculated for the maximum number of vote."]
				pub fn remove_other_vote(
					&self,
					target: ::subxt::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						RemoveOtherVote,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<RemoveOtherVote>()? ==
						[
							43u8, 194u8, 32u8, 219u8, 87u8, 143u8, 240u8, 34u8, 236u8, 232u8,
							128u8, 7u8, 99u8, 113u8, 106u8, 124u8, 92u8, 115u8, 75u8, 228u8, 39u8,
							234u8, 192u8, 134u8, 69u8, 109u8, 119u8, 133u8, 194u8, 110u8, 167u8,
							244u8,
						] {
						let call = RemoveOtherVote { target, index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Enact a proposal from a referendum. For now we just make the weight be the maximum."]
				pub fn enact_proposal(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						EnactProposal,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<EnactProposal>()? ==
						[
							246u8, 188u8, 9u8, 244u8, 56u8, 81u8, 201u8, 59u8, 212u8, 11u8, 204u8,
							7u8, 173u8, 7u8, 212u8, 34u8, 173u8, 248u8, 83u8, 225u8, 209u8, 105u8,
							249u8, 167u8, 243u8, 49u8, 119u8, 167u8, 28u8, 31u8, 60u8, 75u8,
						] {
						let call = EnactProposal { proposal_hash, index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Permanently place a proposal into the blacklist. This prevents it from ever being"]
				#[doc = "proposed again."]
				#[doc = ""]
				#[doc = "If called on a queued public or external proposal, then this will result in it being"]
				#[doc = "removed. If the `ref_index` supplied is an active referendum with the proposal hash,"]
				#[doc = "then it will be cancelled."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `BlacklistOrigin`."]
				#[doc = ""]
				#[doc = "- `proposal_hash`: The proposal hash to blacklist permanently."]
				#[doc = "- `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be"]
				#[doc = "cancelled."]
				#[doc = ""]
				#[doc = "Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a"]
				#[doc = "  reasonable value)."]
				pub fn blacklist(
					&self,
					proposal_hash: ::subxt::sp_core::H256,
					maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Blacklist,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Blacklist>()? ==
						[
							105u8, 99u8, 153u8, 150u8, 122u8, 234u8, 105u8, 238u8, 152u8, 152u8,
							121u8, 181u8, 133u8, 246u8, 159u8, 35u8, 8u8, 65u8, 15u8, 203u8, 206u8,
							75u8, 28u8, 214u8, 111u8, 26u8, 40u8, 141u8, 68u8, 57u8, 217u8, 244u8,
						] {
						let call = Blacklist { proposal_hash, maybe_ref_index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Remove a proposal."]
				#[doc = ""]
				#[doc = "The dispatch origin of this call must be `CancelProposalOrigin`."]
				#[doc = ""]
				#[doc = "- `prop_index`: The index of the proposal to cancel."]
				#[doc = ""]
				#[doc = "Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`"]
				pub fn cancel_proposal(
					&self,
					prop_index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						CancelProposal,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<CancelProposal>()? ==
						[
							26u8, 117u8, 180u8, 24u8, 12u8, 177u8, 77u8, 254u8, 113u8, 53u8, 146u8,
							48u8, 164u8, 255u8, 45u8, 205u8, 207u8, 46u8, 74u8, 184u8, 73u8, 95u8,
							216u8, 190u8, 240u8, 64u8, 121u8, 104u8, 147u8, 141u8, 128u8, 82u8,
						] {
						let call = CancelProposal { prop_index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_democracy::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A motion has been proposed by a public account."]
			pub struct Proposed {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::Event for Proposed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A public proposal has been tabled for referendum vote."]
			pub struct Tabled {
				pub proposal_index: ::core::primitive::u32,
				pub deposit: ::core::primitive::u128,
				pub depositors: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::Event for Tabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Tabled";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An external proposal has been tabled."]
			pub struct ExternalTabled;
			impl ::subxt::Event for ExternalTabled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "ExternalTabled";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A referendum has begun."]
			pub struct Started {
				pub ref_index: ::core::primitive::u32,
				pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
			}
			impl ::subxt::Event for Started {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Started";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "A proposal has been approved by referendum."]
			pub struct Passed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Event for Passed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Passed";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "A proposal has been rejected by referendum."]
			pub struct NotPassed {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Event for NotPassed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "NotPassed";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "A referendum has been cancelled."]
			pub struct Cancelled {
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Event for Cancelled {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Cancelled";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal has been enacted."]
			pub struct Executed {
				pub ref_index: ::core::primitive::u32,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::Event for Executed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Executed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An account has delegated their vote to another account."]
			pub struct Delegated {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub target: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Event for Delegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Delegated";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An account has cancelled a previous delegation operation."]
			pub struct Undelegated {
				pub account: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Event for Undelegated {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Undelegated";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An external proposal has been vetoed."]
			pub struct Vetoed {
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub proposal_hash: ::subxt::sp_core::H256,
				pub until: ::core::primitive::u32,
			}
			impl ::subxt::Event for Vetoed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "Vetoed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal's preimage was noted, and the deposit taken."]
			pub struct PreimageNoted {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub who: ::subxt::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::Event for PreimageNoted {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageNoted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal preimage was removed and used (the deposit was returned)."]
			pub struct PreimageUsed {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub provider: ::subxt::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
			}
			impl ::subxt::Event for PreimageUsed {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageUsed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal could not be executed because its preimage was invalid."]
			pub struct PreimageInvalid {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Event for PreimageInvalid {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageInvalid";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal could not be executed because its preimage was missing."]
			pub struct PreimageMissing {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub ref_index: ::core::primitive::u32,
			}
			impl ::subxt::Event for PreimageMissing {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageMissing";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A registered preimage was removed and the deposit collected by the reaper."]
			pub struct PreimageReaped {
				pub proposal_hash: ::subxt::sp_core::H256,
				pub provider: ::subxt::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
				pub reaper: ::subxt::sp_core::crypto::AccountId32,
			}
			impl ::subxt::Event for PreimageReaped {
				const PALLET: &'static str = "Democracy";
				const EVENT: &'static str = "PreimageReaped";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A proposal_hash has been blacklisted permanently."]
			pub struct Blacklisted {
				pub proposal_hash: ::subxt::sp_core::H256,
			}
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
			pub struct DepositOf<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for DepositOf<'_> {
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
			pub struct Preimages<'a>(pub &'a ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Preimages<'_> {
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
			pub struct ReferendumInfoOf<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for ReferendumInfoOf<'_> {
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
			pub struct VotingOf<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for VotingOf<'_> {
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
			pub struct Locks<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
			impl ::subxt::StorageEntry for Locks<'_> {
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
			pub struct Blacklist<'a>(pub &'a ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Blacklist<'_> {
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
			pub struct Cancellations<'a>(pub &'a ::subxt::sp_core::H256);
			impl ::subxt::StorageEntry for Cancellations<'_> {
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
				#[doc = " The number of (public) proposals that have been made so far."]
				pub async fn public_prop_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<PublicPropCount>()? ==
						[
							91u8, 14u8, 171u8, 94u8, 37u8, 157u8, 46u8, 157u8, 254u8, 13u8, 68u8,
							144u8, 23u8, 146u8, 128u8, 159u8, 9u8, 174u8, 74u8, 174u8, 218u8,
							197u8, 23u8, 235u8, 152u8, 226u8, 216u8, 4u8, 120u8, 121u8, 27u8,
							138u8,
						] {
						let entry = PublicPropCount;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The public proposals. Unsorted. The second item is the proposal's hash."]
				pub async fn public_props(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						::core::primitive::u32,
						::subxt::sp_core::H256,
						::subxt::sp_core::crypto::AccountId32,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<PublicProps>()? ==
						[
							78u8, 208u8, 211u8, 20u8, 85u8, 237u8, 161u8, 149u8, 99u8, 158u8, 6u8,
							54u8, 204u8, 228u8, 132u8, 10u8, 75u8, 247u8, 148u8, 155u8, 101u8,
							183u8, 58u8, 169u8, 21u8, 172u8, 10u8, 110u8, 130u8, 74u8, 88u8, 52u8,
						] {
						let entry = PublicProps;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Those who have locked a deposit."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: Safe, as increasing integer keys are safe."]
				pub async fn deposit_of(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
						::core::primitive::u128,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<DepositOf>()? ==
						[
							116u8, 57u8, 200u8, 96u8, 150u8, 62u8, 162u8, 169u8, 28u8, 18u8, 134u8,
							161u8, 210u8, 217u8, 80u8, 225u8, 22u8, 185u8, 177u8, 166u8, 243u8,
							232u8, 193u8, 64u8, 170u8, 89u8, 216u8, 198u8, 43u8, 102u8, 178u8,
							55u8,
						] {
						let entry = DepositOf(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Those who have locked a deposit."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: Safe, as increasing integer keys are safe."]
				pub async fn deposit_of_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, DepositOf<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<DepositOf>()? ==
						[
							116u8, 57u8, 200u8, 96u8, 150u8, 62u8, 162u8, 169u8, 28u8, 18u8, 134u8,
							161u8, 210u8, 217u8, 80u8, 225u8, 22u8, 185u8, 177u8, 166u8, 243u8,
							232u8, 193u8, 64u8, 170u8, 89u8, 216u8, 198u8, 43u8, 102u8, 178u8,
							55u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Map of hashes to the proposal preimage, along with who registered it and their deposit."]
				#[doc = " The block number is the block at which it was deposited."]
				pub async fn preimages(
					&self,
					_0: &::subxt::sp_core::H256,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_democracy::PreimageStatus<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Preimages>()? ==
						[
							20u8, 82u8, 223u8, 51u8, 178u8, 115u8, 71u8, 83u8, 23u8, 15u8, 85u8,
							66u8, 0u8, 69u8, 68u8, 20u8, 28u8, 159u8, 74u8, 41u8, 225u8, 145u8,
							247u8, 23u8, 36u8, 155u8, 101u8, 229u8, 27u8, 24u8, 93u8, 215u8,
						] {
						let entry = Preimages(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Map of hashes to the proposal preimage, along with who registered it and their deposit."]
				#[doc = " The block number is the block at which it was deposited."]
				pub async fn preimages_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Preimages<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Preimages>()? ==
						[
							20u8, 82u8, 223u8, 51u8, 178u8, 115u8, 71u8, 83u8, 23u8, 15u8, 85u8,
							66u8, 0u8, 69u8, 68u8, 20u8, 28u8, 159u8, 74u8, 41u8, 225u8, 145u8,
							247u8, 23u8, 36u8, 155u8, 101u8, 229u8, 27u8, 24u8, 93u8, 215u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The next free referendum index, aka the number of referenda started so far."]
				pub async fn referendum_count(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<ReferendumCount>()? ==
						[
							153u8, 210u8, 106u8, 244u8, 156u8, 70u8, 124u8, 251u8, 123u8, 75u8,
							7u8, 189u8, 199u8, 145u8, 95u8, 119u8, 137u8, 11u8, 240u8, 160u8,
							151u8, 248u8, 229u8, 231u8, 89u8, 222u8, 18u8, 237u8, 144u8, 78u8,
							99u8, 58u8,
						] {
						let entry = ReferendumCount;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The lowest referendum index representing an unbaked referendum. Equal to"]
				#[doc = " `ReferendumCount` if there isn't a unbaked referendum."]
				pub async fn lowest_unbaked(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<LowestUnbaked>()? ==
						[
							4u8, 51u8, 108u8, 11u8, 48u8, 165u8, 19u8, 251u8, 182u8, 76u8, 163u8,
							73u8, 227u8, 2u8, 212u8, 74u8, 128u8, 27u8, 165u8, 164u8, 111u8, 22u8,
							209u8, 190u8, 103u8, 7u8, 116u8, 16u8, 160u8, 144u8, 123u8, 64u8,
						] {
						let entry = LowestUnbaked;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Information concerning any given referendum."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as indexes are not under an attacker’s control."]
				pub async fn referendum_info_of(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<
						runtime_types::pallet_democracy::types::ReferendumInfo<
							::core::primitive::u32,
							::subxt::sp_core::H256,
							::core::primitive::u128,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ReferendumInfoOf>()? ==
						[
							112u8, 206u8, 173u8, 93u8, 255u8, 76u8, 85u8, 122u8, 24u8, 97u8, 177u8,
							67u8, 44u8, 143u8, 53u8, 159u8, 206u8, 135u8, 63u8, 74u8, 230u8, 47u8,
							27u8, 224u8, 138u8, 217u8, 194u8, 229u8, 148u8, 249u8, 230u8, 114u8,
						] {
						let entry = ReferendumInfoOf(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Information concerning any given referendum."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as indexes are not under an attacker’s control."]
				pub async fn referendum_info_of_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, ReferendumInfoOf<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<ReferendumInfoOf>()? ==
						[
							112u8, 206u8, 173u8, 93u8, 255u8, 76u8, 85u8, 122u8, 24u8, 97u8, 177u8,
							67u8, 44u8, 143u8, 53u8, 159u8, 206u8, 135u8, 63u8, 74u8, 230u8, 47u8,
							27u8, 224u8, 138u8, 217u8, 194u8, 229u8, 148u8, 249u8, 230u8, 114u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " All votes for a particular voter. We store the balance for the number of votes that we"]
				#[doc = " have recorded. The second item is the total amount of delegations, that will be added."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId`s are crypto hashes anyway."]
				pub async fn voting_of(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_democracy::vote::Voting<
						::core::primitive::u128,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<VotingOf>()? ==
						[
							194u8, 13u8, 151u8, 207u8, 194u8, 79u8, 233u8, 214u8, 193u8, 52u8,
							78u8, 62u8, 71u8, 35u8, 139u8, 11u8, 41u8, 163u8, 143u8, 156u8, 236u8,
							207u8, 132u8, 138u8, 2u8, 176u8, 56u8, 224u8, 67u8, 39u8, 190u8, 13u8,
						] {
						let entry = VotingOf(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " All votes for a particular voter. We store the balance for the number of votes that we"]
				#[doc = " have recorded. The second item is the total amount of delegations, that will be added."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: SAFE as `AccountId`s are crypto hashes anyway."]
				pub async fn voting_of_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, VotingOf<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<VotingOf>()? ==
						[
							194u8, 13u8, 151u8, 207u8, 194u8, 79u8, 233u8, 214u8, 193u8, 52u8,
							78u8, 62u8, 71u8, 35u8, 139u8, 11u8, 41u8, 163u8, 143u8, 156u8, 236u8,
							207u8, 132u8, 138u8, 2u8, 176u8, 56u8, 224u8, 67u8, 39u8, 190u8, 13u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Accounts for which there are locks in action which may be removed at some point in the"]
				#[doc = " future. The value is the block number at which the lock expires and may be removed."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub async fn locks(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::u32>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Locks>()? ==
						[
							89u8, 243u8, 33u8, 115u8, 101u8, 81u8, 187u8, 200u8, 147u8, 255u8,
							82u8, 11u8, 122u8, 37u8, 247u8, 192u8, 30u8, 187u8, 19u8, 229u8, 128u8,
							229u8, 104u8, 66u8, 233u8, 229u8, 149u8, 128u8, 95u8, 52u8, 201u8,
							145u8,
						] {
						let entry = Locks(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Accounts for which there are locks in action which may be removed at some point in the"]
				#[doc = " future. The value is the block number at which the lock expires and may be removed."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: OK ― `AccountId` is a secure hash."]
				pub async fn locks_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Locks>()? ==
						[
							89u8, 243u8, 33u8, 115u8, 101u8, 81u8, 187u8, 200u8, 147u8, 255u8,
							82u8, 11u8, 122u8, 37u8, 247u8, 192u8, 30u8, 187u8, 19u8, 229u8, 128u8,
							229u8, 104u8, 66u8, 233u8, 229u8, 149u8, 128u8, 95u8, 52u8, 201u8,
							145u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " True if the last referendum tabled was submitted externally. False if it was a public"]
				#[doc = " proposal."]
				pub async fn last_tabled_was_external(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<LastTabledWasExternal>()? ==
						[
							3u8, 67u8, 106u8, 1u8, 89u8, 204u8, 4u8, 145u8, 121u8, 44u8, 34u8,
							76u8, 18u8, 206u8, 65u8, 214u8, 222u8, 82u8, 31u8, 223u8, 144u8, 169u8,
							17u8, 6u8, 138u8, 36u8, 113u8, 155u8, 241u8, 106u8, 189u8, 218u8,
						] {
						let entry = LastTabledWasExternal;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The referendum to be tabled whenever it would be valid to table an external proposal."]
				#[doc = " This happens when a referendum needs to be tabled and one of two conditions are met:"]
				#[doc = " - `LastTabledWasExternal` is `false`; or"]
				#[doc = " - `PublicProps` is empty."]
				pub async fn next_external(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::subxt::sp_core::H256,
						runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<NextExternal>()? ==
						[
							167u8, 226u8, 113u8, 10u8, 12u8, 157u8, 190u8, 117u8, 233u8, 177u8,
							254u8, 126u8, 2u8, 55u8, 100u8, 249u8, 78u8, 127u8, 148u8, 239u8,
							193u8, 246u8, 123u8, 58u8, 150u8, 132u8, 209u8, 228u8, 105u8, 195u8,
							217u8, 99u8,
						] {
						let entry = NextExternal;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " A record of who vetoed what. Maps proposal hash to a possible existent block number"]
				#[doc = " (until when it may not be resubmitted) and who vetoed it."]
				pub async fn blacklist(
					&self,
					_0: &::subxt::sp_core::H256,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::core::primitive::u32,
						::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Blacklist>()? ==
						[
							9u8, 76u8, 174u8, 143u8, 210u8, 103u8, 197u8, 219u8, 152u8, 134u8,
							67u8, 78u8, 109u8, 39u8, 246u8, 214u8, 3u8, 51u8, 69u8, 208u8, 32u8,
							69u8, 247u8, 14u8, 236u8, 37u8, 112u8, 226u8, 146u8, 169u8, 153u8,
							217u8,
						] {
						let entry = Blacklist(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " A record of who vetoed what. Maps proposal hash to a possible existent block number"]
				#[doc = " (until when it may not be resubmitted) and who vetoed it."]
				pub async fn blacklist_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Blacklist<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Blacklist>()? ==
						[
							9u8, 76u8, 174u8, 143u8, 210u8, 103u8, 197u8, 219u8, 152u8, 134u8,
							67u8, 78u8, 109u8, 39u8, 246u8, 214u8, 3u8, 51u8, 69u8, 208u8, 32u8,
							69u8, 247u8, 14u8, 236u8, 37u8, 112u8, 226u8, 146u8, 169u8, 153u8,
							217u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Record of all proposals that have been subject to emergency cancellation."]
				pub async fn cancellations(
					&self,
					_0: &::subxt::sp_core::H256,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<Cancellations>()? ==
						[
							176u8, 55u8, 142u8, 79u8, 35u8, 110u8, 215u8, 163u8, 134u8, 172u8,
							171u8, 71u8, 180u8, 175u8, 7u8, 29u8, 126u8, 141u8, 236u8, 234u8,
							214u8, 132u8, 192u8, 197u8, 205u8, 31u8, 106u8, 122u8, 204u8, 71u8,
							155u8, 18u8,
						] {
						let entry = Cancellations(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Record of all proposals that have been subject to emergency cancellation."]
				pub async fn cancellations_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Cancellations<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Cancellations>()? ==
						[
							176u8, 55u8, 142u8, 79u8, 35u8, 110u8, 215u8, 163u8, 134u8, 172u8,
							171u8, 71u8, 180u8, 175u8, 7u8, 29u8, 126u8, 141u8, 236u8, 234u8,
							214u8, 132u8, 192u8, 197u8, 205u8, 31u8, 106u8, 122u8, 204u8, 71u8,
							155u8, 18u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " New networks start with last version."]
				pub async fn storage_version(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<runtime_types::pallet_democracy::Releases>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<StorageVersion>()? ==
						[
							39u8, 219u8, 134u8, 64u8, 250u8, 96u8, 95u8, 156u8, 100u8, 236u8, 18u8,
							78u8, 59u8, 146u8, 5u8, 245u8, 113u8, 125u8, 220u8, 140u8, 125u8, 5u8,
							194u8, 134u8, 248u8, 95u8, 250u8, 108u8, 142u8, 230u8, 21u8, 120u8,
						] {
						let entry = StorageVersion;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The period between a proposal being approved and enacted."]
				#[doc = ""]
				#[doc = " It should generally be a little more than the unstake period to ensure that"]
				#[doc = " voting stakers have an opportunity to remove themselves from the system in the case"]
				#[doc = " where they are on the losing side of a vote."]
				pub fn enactment_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "EnactmentPeriod")? ==
						[
							253u8, 67u8, 165u8, 237u8, 244u8, 21u8, 109u8, 170u8, 142u8, 183u8,
							41u8, 182u8, 101u8, 237u8, 67u8, 154u8, 228u8, 14u8, 229u8, 154u8,
							109u8, 21u8, 146u8, 31u8, 214u8, 169u8, 17u8, 160u8, 130u8, 186u8,
							114u8, 40u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("EnactmentPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " How often (in blocks) new public referenda are launched."]
				pub fn launch_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "LaunchPeriod")? ==
						[
							245u8, 237u8, 56u8, 109u8, 232u8, 150u8, 223u8, 171u8, 23u8, 64u8,
							50u8, 121u8, 167u8, 27u8, 147u8, 211u8, 244u8, 31u8, 94u8, 133u8,
							157u8, 189u8, 228u8, 8u8, 154u8, 138u8, 31u8, 184u8, 0u8, 247u8, 80u8,
							40u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("LaunchPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " How often (in blocks) to check for new votes."]
				pub fn voting_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "VotingPeriod")? ==
						[
							171u8, 175u8, 8u8, 228u8, 60u8, 233u8, 241u8, 79u8, 126u8, 7u8, 91u8,
							193u8, 111u8, 213u8, 171u8, 254u8, 239u8, 84u8, 216u8, 218u8, 140u8,
							137u8, 165u8, 197u8, 197u8, 219u8, 235u8, 250u8, 156u8, 52u8, 141u8,
							231u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("VotingPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The minimum period of vote locking."]
				#[doc = ""]
				#[doc = " It should be no shorter than enactment period to ensure that in the case of an approval,"]
				#[doc = " those successful voters are locked into the consequences that their votes entail."]
				pub fn vote_locking_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "VoteLockingPeriod")? ==
						[
							0u8, 77u8, 4u8, 208u8, 241u8, 242u8, 197u8, 163u8, 193u8, 241u8, 59u8,
							147u8, 113u8, 125u8, 62u8, 30u8, 6u8, 158u8, 243u8, 57u8, 162u8, 41u8,
							68u8, 22u8, 227u8, 4u8, 242u8, 253u8, 203u8, 147u8, 183u8, 109u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("VoteLockingPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The minimum amount to be used as a deposit for a public referendum proposal."]
				pub fn minimum_deposit(
					&self,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "MinimumDeposit")? ==
						[
							121u8, 234u8, 23u8, 29u8, 240u8, 133u8, 174u8, 115u8, 158u8, 231u8,
							16u8, 152u8, 230u8, 36u8, 211u8, 29u8, 232u8, 222u8, 8u8, 19u8, 140u8,
							236u8, 140u8, 225u8, 144u8, 190u8, 110u8, 76u8, 220u8, 219u8, 81u8,
							130u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("MinimumDeposit")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Indicator for whether an emergency origin is even allowed to happen. Some chains may"]
				#[doc = " want to set this permanently to `false`, others may want to condition it on things such"]
				#[doc = " as an upgrade having happened recently."]
				pub fn instant_allowed(
					&self,
				) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "InstantAllowed")? ==
						[
							66u8, 19u8, 43u8, 75u8, 149u8, 2u8, 157u8, 136u8, 33u8, 102u8, 57u8,
							127u8, 246u8, 72u8, 14u8, 94u8, 240u8, 2u8, 162u8, 86u8, 232u8, 70u8,
							22u8, 133u8, 209u8, 205u8, 115u8, 236u8, 17u8, 9u8, 37u8, 14u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("InstantAllowed")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Minimum voting period allowed for a fast-track referendum."]
				pub fn fast_track_voting_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "FastTrackVotingPeriod")? ==
						[
							71u8, 48u8, 212u8, 167u8, 97u8, 240u8, 205u8, 66u8, 106u8, 13u8, 53u8,
							104u8, 141u8, 228u8, 124u8, 14u8, 185u8, 138u8, 30u8, 168u8, 192u8,
							78u8, 16u8, 7u8, 31u8, 126u8, 120u8, 79u8, 57u8, 207u8, 171u8, 37u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("FastTrackVotingPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Period in blocks where an external proposal may not be re-submitted after being vetoed."]
				pub fn cooloff_period(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "CooloffPeriod")? ==
						[
							231u8, 13u8, 169u8, 211u8, 179u8, 85u8, 187u8, 112u8, 204u8, 125u8,
							248u8, 22u8, 161u8, 217u8, 211u8, 93u8, 187u8, 28u8, 129u8, 171u8,
							88u8, 37u8, 161u8, 180u8, 87u8, 207u8, 132u8, 111u8, 66u8, 194u8, 38u8,
							5u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("CooloffPeriod")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The amount of balance that must be deposited per byte of preimage stored."]
				pub fn preimage_byte_deposit(
					&self,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "PreimageByteDeposit")? ==
						[
							40u8, 225u8, 79u8, 2u8, 3u8, 167u8, 159u8, 232u8, 81u8, 38u8, 248u8,
							132u8, 236u8, 203u8, 88u8, 133u8, 112u8, 47u8, 141u8, 153u8, 104u8,
							87u8, 191u8, 102u8, 65u8, 32u8, 120u8, 166u8, 188u8, 23u8, 34u8, 202u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("PreimageByteDeposit")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The maximum number of votes for an account."]
				#[doc = ""]
				#[doc = " Also used to compute weight, an overly big value can"]
				#[doc = " lead to extrinsic with very big weight: see `delegate` for instance."]
				pub fn max_votes(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "MaxVotes")? ==
						[
							218u8, 111u8, 73u8, 160u8, 254u8, 247u8, 22u8, 113u8, 78u8, 79u8,
							145u8, 255u8, 29u8, 155u8, 89u8, 144u8, 4u8, 167u8, 134u8, 190u8,
							232u8, 124u8, 36u8, 207u8, 7u8, 204u8, 40u8, 32u8, 38u8, 216u8, 249u8,
							29u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("MaxVotes")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The maximum number of public proposals that can exist at any time."]
				pub fn max_proposals(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Democracy", "MaxProposals")? ==
						[
							125u8, 103u8, 31u8, 211u8, 29u8, 50u8, 100u8, 13u8, 229u8, 120u8,
							216u8, 228u8, 4u8, 121u8, 229u8, 90u8, 172u8, 228u8, 86u8, 73u8, 64u8,
							153u8, 249u8, 48u8, 232u8, 150u8, 150u8, 65u8, 205u8, 182u8, 12u8,
							81u8,
						] {
						let pallet = self.client.metadata().pallet("Democracy")?;
						let constant = pallet.constant("MaxProposals")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod scheduler {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Schedule {
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for Schedule {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Cancel {
				pub when: ::core::primitive::u32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::Call for Cancel {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "cancel";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ScheduleNamed {
				pub id: ::std::vec::Vec<::core::primitive::u8>,
				pub when: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for ScheduleNamed {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule_named";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct CancelNamed {
				pub id: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::Call for CancelNamed {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "cancel_named";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ScheduleAfter {
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for ScheduleAfter {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule_after";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ScheduleNamedAfter {
				pub id: ::std::vec::Vec<::core::primitive::u8>,
				pub after: ::core::primitive::u32,
				pub maybe_periodic:
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
				pub priority: ::core::primitive::u8,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for ScheduleNamedAfter {
				const PALLET: &'static str = "Scheduler";
				const FUNCTION: &'static str = "schedule_named_after";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Anonymously schedule a task."]
				pub fn schedule(
					&self,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Schedule,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Schedule>()? ==
						[
							165u8, 196u8, 176u8, 186u8, 47u8, 133u8, 116u8, 174u8, 169u8, 87u8,
							23u8, 102u8, 237u8, 189u8, 2u8, 202u8, 203u8, 24u8, 119u8, 71u8, 6u8,
							235u8, 214u8, 117u8, 215u8, 53u8, 236u8, 16u8, 23u8, 217u8, 123u8,
							249u8,
						] {
						let call = Schedule {
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Cancel an anonymously scheduled task."]
				pub fn cancel(
					&self,
					when: ::core::primitive::u32,
					index: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Cancel, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Cancel>()? ==
						[
							118u8, 0u8, 188u8, 218u8, 148u8, 86u8, 139u8, 15u8, 3u8, 161u8, 6u8,
							150u8, 46u8, 32u8, 85u8, 179u8, 106u8, 113u8, 240u8, 115u8, 167u8,
							114u8, 243u8, 69u8, 103u8, 60u8, 99u8, 135u8, 21u8, 8u8, 19u8, 225u8,
						] {
						let call = Cancel { when, index };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Schedule a named task."]
				pub fn schedule_named(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
					when: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ScheduleNamed,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ScheduleNamed>()? ==
						[
							234u8, 243u8, 52u8, 2u8, 83u8, 159u8, 133u8, 57u8, 162u8, 157u8, 15u8,
							233u8, 235u8, 141u8, 141u8, 190u8, 92u8, 62u8, 67u8, 190u8, 109u8,
							172u8, 24u8, 84u8, 22u8, 28u8, 23u8, 125u8, 204u8, 154u8, 177u8, 88u8,
						] {
						let call = ScheduleNamed {
							id,
							when,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Cancel a named scheduled task."]
				pub fn cancel_named(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						CancelNamed,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<CancelNamed>()? ==
						[
							118u8, 221u8, 232u8, 126u8, 67u8, 134u8, 33u8, 7u8, 224u8, 110u8,
							181u8, 18u8, 57u8, 39u8, 15u8, 64u8, 90u8, 132u8, 2u8, 238u8, 19u8,
							241u8, 194u8, 120u8, 5u8, 109u8, 74u8, 205u8, 42u8, 244u8, 99u8, 54u8,
						] {
						let call = CancelNamed { id };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Anonymously schedule a task after a delay."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`schedule`]."]
				#[doc = "# </weight>"]
				pub fn schedule_after(
					&self,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ScheduleAfter,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ScheduleAfter>()? ==
						[
							217u8, 27u8, 63u8, 33u8, 82u8, 67u8, 154u8, 156u8, 92u8, 23u8, 129u8,
							129u8, 222u8, 100u8, 194u8, 168u8, 153u8, 126u8, 212u8, 214u8, 131u8,
							46u8, 4u8, 244u8, 168u8, 201u8, 225u8, 48u8, 83u8, 21u8, 144u8, 12u8,
						] {
						let call = ScheduleAfter {
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Schedule a named task after a delay."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Same as [`schedule_named`](Self::schedule_named)."]
				#[doc = "# </weight>"]
				pub fn schedule_named_after(
					&self,
					id: ::std::vec::Vec<::core::primitive::u8>,
					after: ::core::primitive::u32,
					maybe_periodic: ::core::option::Option<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					priority: ::core::primitive::u8,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ScheduleNamedAfter,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ScheduleNamedAfter>()? ==
						[
							126u8, 50u8, 31u8, 23u8, 177u8, 154u8, 106u8, 171u8, 196u8, 217u8,
							95u8, 114u8, 118u8, 200u8, 60u8, 105u8, 65u8, 229u8, 230u8, 90u8,
							218u8, 130u8, 187u8, 133u8, 211u8, 161u8, 8u8, 248u8, 246u8, 121u8,
							183u8, 14u8,
						] {
						let call = ScheduleNamedAfter {
							id,
							after,
							maybe_periodic,
							priority,
							call: ::std::boxed::Box::new(call),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_scheduler::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Scheduled some task. \\[when, index\\]"]
			pub struct Scheduled(pub ::core::primitive::u32, pub ::core::primitive::u32);
			impl ::subxt::Event for Scheduled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Scheduled";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Canceled some task. \\[when, index\\]"]
			pub struct Canceled(pub ::core::primitive::u32, pub ::core::primitive::u32);
			impl ::subxt::Event for Canceled {
				const PALLET: &'static str = "Scheduler";
				const EVENT: &'static str = "Canceled";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Dispatched some task. \\[task, id, result\\]"]
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
			pub struct Agenda<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for Agenda<'_> {
				const PALLET: &'static str = "Scheduler";
				const STORAGE: &'static str = "Agenda";
				type Value = ::std::vec::Vec<
					::core::option::Option<
						runtime_types::pallet_scheduler::ScheduledV2<
							runtime_types::composable_runtime::Call,
							::core::primitive::u32,
							runtime_types::composable_runtime::OriginCaller,
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
			pub struct Lookup<'a>(pub &'a [::core::primitive::u8]);
			impl ::subxt::StorageEntry for Lookup<'_> {
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
				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub async fn agenda(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<
						::core::option::Option<
							runtime_types::pallet_scheduler::ScheduledV2<
								runtime_types::composable_runtime::Call,
								::core::primitive::u32,
								runtime_types::composable_runtime::OriginCaller,
								::subxt::sp_core::crypto::AccountId32,
							>,
						>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Agenda>()? ==
						[
							254u8, 162u8, 68u8, 253u8, 87u8, 82u8, 214u8, 169u8, 63u8, 170u8,
							143u8, 1u8, 44u8, 201u8, 139u8, 65u8, 229u8, 14u8, 67u8, 215u8, 23u8,
							219u8, 112u8, 192u8, 205u8, 121u8, 38u8, 151u8, 218u8, 248u8, 162u8,
							131u8,
						] {
						let entry = Agenda(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Items to be executed, indexed by the block number that they should be executed on."]
				pub async fn agenda_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Agenda<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Agenda>()? ==
						[
							254u8, 162u8, 68u8, 253u8, 87u8, 82u8, 214u8, 169u8, 63u8, 170u8,
							143u8, 1u8, 44u8, 201u8, 139u8, 65u8, 229u8, 14u8, 67u8, 215u8, 23u8,
							219u8, 112u8, 192u8, 205u8, 121u8, 38u8, 151u8, 218u8, 248u8, 162u8,
							131u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Lookup from identity to the block number and index of the task."]
				pub async fn lookup(
					&self,
					_0: &[::core::primitive::u8],
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Lookup>()? ==
						[
							56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
							180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8, 36u8,
							202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8, 62u8, 166u8,
						] {
						let entry = Lookup(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Lookup from identity to the block number and index of the task."]
				pub async fn lookup_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Lookup<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Lookup>()? ==
						[
							56u8, 105u8, 156u8, 110u8, 251u8, 141u8, 219u8, 56u8, 131u8, 57u8,
							180u8, 33u8, 48u8, 30u8, 193u8, 194u8, 169u8, 182u8, 168u8, 43u8, 36u8,
							202u8, 222u8, 182u8, 41u8, 216u8, 222u8, 1u8, 72u8, 165u8, 62u8, 166u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " New networks start with last version."]
				pub async fn storage_version(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::pallet_scheduler::Releases,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<StorageVersion>()? ==
						[
							181u8, 185u8, 153u8, 214u8, 175u8, 17u8, 111u8, 241u8, 124u8, 242u8,
							171u8, 99u8, 193u8, 23u8, 251u8, 248u8, 150u8, 241u8, 249u8, 142u8,
							234u8, 209u8, 246u8, 39u8, 232u8, 192u8, 44u8, 121u8, 63u8, 14u8,
							245u8, 110u8,
						] {
						let entry = StorageVersion;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The maximum weight that may be scheduled per block for any dispatchables of less"]
				#[doc = " priority than `schedule::HARD_DEADLINE`."]
				pub fn maximum_weight(
					&self,
				) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Scheduler", "MaximumWeight")? ==
						[
							230u8, 169u8, 152u8, 214u8, 255u8, 61u8, 176u8, 69u8, 211u8, 100u8,
							217u8, 192u8, 188u8, 247u8, 181u8, 157u8, 38u8, 122u8, 75u8, 206u8,
							246u8, 8u8, 161u8, 175u8, 73u8, 182u8, 204u8, 242u8, 227u8, 3u8, 231u8,
							254u8,
						] {
						let pallet = self.client.metadata().pallet("Scheduler")?;
						let constant = pallet.constant("MaximumWeight")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The maximum number of scheduled calls in the queue for a single block."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				pub fn max_scheduled_per_block(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Scheduler", "MaxScheduledPerBlock")? ==
						[
							64u8, 25u8, 128u8, 202u8, 165u8, 97u8, 30u8, 196u8, 174u8, 132u8,
							139u8, 223u8, 88u8, 20u8, 228u8, 203u8, 253u8, 201u8, 83u8, 157u8,
							161u8, 120u8, 187u8, 165u8, 4u8, 64u8, 184u8, 34u8, 28u8, 129u8, 136u8,
							13u8,
						] {
						let pallet = self.client.metadata().pallet("Scheduler")?;
						let constant = pallet.constant("MaxScheduledPerBlock")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod utility {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Batch {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for Batch {
				const PALLET: &'static str = "Utility";
				const FUNCTION: &'static str = "batch";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct AsDerivative {
				pub index: ::core::primitive::u16,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for AsDerivative {
				const PALLET: &'static str = "Utility";
				const FUNCTION: &'static str = "as_derivative";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct BatchAll {
				pub calls: ::std::vec::Vec<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for BatchAll {
				const PALLET: &'static str = "Utility";
				const FUNCTION: &'static str = "batch_all";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct DispatchAs {
				pub as_origin: ::std::boxed::Box<runtime_types::composable_runtime::OriginCaller>,
				pub call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
			}
			impl ::subxt::Call for DispatchAs {
				const PALLET: &'static str = "Utility";
				const FUNCTION: &'static str = "dispatch_as";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Send a batch of dispatch calls."]
				#[doc = ""]
				#[doc = "May be called from any origin."]
				#[doc = ""]
				#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
				#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
				#[doc = ""]
				#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
				#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
				#[doc = "# </weight>"]
				#[doc = ""]
				#[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
				#[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
				#[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
				#[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
				#[doc = "event is deposited."]
				pub fn batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::Call>,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Batch, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Batch>()? ==
						[
							224u8, 154u8, 60u8, 225u8, 36u8, 100u8, 126u8, 166u8, 55u8, 8u8, 59u8,
							33u8, 209u8, 41u8, 107u8, 95u8, 33u8, 139u8, 139u8, 210u8, 75u8, 67u8,
							127u8, 215u8, 216u8, 219u8, 182u8, 159u8, 208u8, 13u8, 165u8, 124u8,
						] {
						let call = Batch { calls };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Send a call through an indexed pseudonym of the sender."]
				#[doc = ""]
				#[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
				#[doc = "use the same filter as the origin of this call."]
				#[doc = ""]
				#[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
				#[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
				#[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
				#[doc = "in the Multisig pallet instead."]
				#[doc = ""]
				#[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				pub fn as_derivative(
					&self,
					index: ::core::primitive::u16,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						AsDerivative,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<AsDerivative>()? ==
						[
							23u8, 119u8, 118u8, 141u8, 135u8, 9u8, 10u8, 24u8, 107u8, 245u8, 118u8,
							25u8, 92u8, 37u8, 149u8, 77u8, 205u8, 85u8, 111u8, 0u8, 96u8, 4u8,
							97u8, 206u8, 141u8, 63u8, 33u8, 246u8, 242u8, 36u8, 41u8, 30u8,
						] {
						let call = AsDerivative { index, call: ::std::boxed::Box::new(call) };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Send a batch of dispatch calls and atomically execute them."]
				#[doc = "The whole transaction will rollback and fail if any of the calls failed."]
				#[doc = ""]
				#[doc = "May be called from any origin."]
				#[doc = ""]
				#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
				#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
				#[doc = ""]
				#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
				#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
				#[doc = "# </weight>"]
				pub fn batch_all(
					&self,
					calls: ::std::vec::Vec<runtime_types::composable_runtime::Call>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						BatchAll,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<BatchAll>()? ==
						[
							165u8, 115u8, 84u8, 204u8, 208u8, 228u8, 222u8, 135u8, 254u8, 210u8,
							72u8, 78u8, 142u8, 205u8, 33u8, 4u8, 126u8, 86u8, 66u8, 243u8, 59u8,
							86u8, 163u8, 241u8, 213u8, 79u8, 122u8, 106u8, 58u8, 156u8, 68u8,
							224u8,
						] {
						let call = BatchAll { calls };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Dispatches a function call with a provided origin."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- O(1)."]
				#[doc = "- Limited storage reads."]
				#[doc = "- One DB write (event)."]
				#[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
				#[doc = "# </weight>"]
				pub fn dispatch_as(
					&self,
					as_origin: runtime_types::composable_runtime::OriginCaller,
					call: runtime_types::composable_runtime::Call,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						DispatchAs,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<DispatchAs>()? ==
						[
							64u8, 73u8, 90u8, 63u8, 114u8, 204u8, 75u8, 180u8, 57u8, 74u8, 132u8,
							92u8, 88u8, 221u8, 83u8, 7u8, 225u8, 202u8, 55u8, 24u8, 171u8, 74u8,
							218u8, 69u8, 12u8, 223u8, 57u8, 20u8, 104u8, 49u8, 6u8, 205u8,
						] {
						let call = DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_utility::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
			#[doc = "well as the error."]
			pub struct BatchInterrupted {
				pub index: ::core::primitive::u32,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::Event for BatchInterrupted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchInterrupted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Batch of dispatches completed fully with no error."]
			pub struct BatchCompleted;
			impl ::subxt::Event for BatchCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompleted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A single item within a Batch of dispatches has completed with no error."]
			pub struct ItemCompleted;
			impl ::subxt::Event for ItemCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemCompleted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A call was dispatched. \\[result\\]"]
			pub struct DispatchedAs(
				pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			);
			impl ::subxt::Event for DispatchedAs {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "DispatchedAs";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				#[doc = " The limit on the number of batched calls."]
				pub fn batched_calls_limit(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Utility", "batched_calls_limit")? ==
						[
							230u8, 161u8, 6u8, 191u8, 162u8, 108u8, 149u8, 245u8, 68u8, 101u8,
							120u8, 129u8, 140u8, 51u8, 77u8, 97u8, 30u8, 155u8, 115u8, 70u8, 72u8,
							235u8, 251u8, 192u8, 5u8, 8u8, 188u8, 72u8, 132u8, 227u8, 44u8, 2u8,
						] {
						let pallet = self.client.metadata().pallet("Utility")?;
						let constant = pallet.constant("batched_calls_limit")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod xcmp_queue {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some XCM was executed ok."]
			pub struct Success(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for Success {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Success";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some XCM failed."]
			pub struct Fail(
				pub ::core::option::Option<::subxt::sp_core::H256>,
				pub runtime_types::xcm::v2::traits::Error,
			);
			impl ::subxt::Event for Fail {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "Fail";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Bad XCM version used."]
			pub struct BadVersion(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for BadVersion {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadVersion";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Bad XCM format used."]
			pub struct BadFormat(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for BadFormat {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "BadFormat";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An upward message was sent to the relay chain."]
			pub struct UpwardMessageSent(pub ::core::option::Option<::subxt::sp_core::H256>);
			impl ::subxt::Event for UpwardMessageSent {
				const PALLET: &'static str = "XcmpQueue";
				const EVENT: &'static str = "UpwardMessageSent";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An HRMP message was sent to a sibling parachain."]
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
			pub struct InboundXcmpMessages<'a>(
				pub &'a runtime_types::polkadot_parachain::primitives::Id,
				pub &'a ::core::primitive::u32,
			);
			impl ::subxt::StorageEntry for InboundXcmpMessages<'_> {
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
			pub struct OutboundXcmpMessages<'a>(
				pub &'a runtime_types::polkadot_parachain::primitives::Id,
				pub &'a ::core::primitive::u16,
			);
			impl ::subxt::StorageEntry for OutboundXcmpMessages<'_> {
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
			pub struct SignalMessages<'a>(
				pub &'a runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::StorageEntry for SignalMessages<'_> {
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
				#[doc = " Status of the inbound XCMP channels."]
				pub async fn inbound_xcmp_status(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::cumulus_pallet_xcmp_queue::InboundStatus,
						::std::vec::Vec<(
							::core::primitive::u32,
							runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
						)>,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<InboundXcmpStatus>()? ==
						[
							148u8, 247u8, 147u8, 31u8, 112u8, 54u8, 112u8, 63u8, 176u8, 191u8,
							199u8, 202u8, 158u8, 233u8, 239u8, 252u8, 49u8, 126u8, 47u8, 165u8,
							67u8, 146u8, 235u8, 28u8, 22u8, 248u8, 230u8, 117u8, 187u8, 152u8,
							107u8, 137u8,
						] {
						let entry = InboundXcmpStatus;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
				pub async fn inbound_xcmp_messages(
					&self,
					_0: &runtime_types::polkadot_parachain::primitives::Id,
					_1: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<InboundXcmpMessages>()? ==
						[
							40u8, 210u8, 247u8, 211u8, 86u8, 173u8, 196u8, 82u8, 179u8, 253u8,
							101u8, 113u8, 231u8, 237u8, 92u8, 59u8, 169u8, 175u8, 19u8, 18u8,
							197u8, 159u8, 146u8, 203u8, 34u8, 42u8, 158u8, 137u8, 20u8, 230u8,
							150u8, 7u8,
						] {
						let entry = InboundXcmpMessages(_0, _1);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Inbound aggregate XCMP messages. It can only be one per ParaId/block."]
				pub async fn inbound_xcmp_messages_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, InboundXcmpMessages<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<InboundXcmpMessages>()? ==
						[
							40u8, 210u8, 247u8, 211u8, 86u8, 173u8, 196u8, 82u8, 179u8, 253u8,
							101u8, 113u8, 231u8, 237u8, 92u8, 59u8, 169u8, 175u8, 19u8, 18u8,
							197u8, 159u8, 146u8, 203u8, 34u8, 42u8, 158u8, 137u8, 20u8, 230u8,
							150u8, 7u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The non-empty XCMP channels in order of becoming non-empty, and the index of the first"]
				#[doc = " and last outbound message. If the two indices are equal, then it indicates an empty"]
				#[doc = " queue and there must be a non-`Ok` `OutboundStatus`. We assume queues grow no greater"]
				#[doc = " than 65535 items. Queue indices for normal messages begin at one; zero is reserved in"]
				#[doc = " case of the need to send a high-priority signal message this block."]
				#[doc = " The bool is true if there is a signal message waiting to be sent."]
				pub async fn outbound_xcmp_status(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						runtime_types::polkadot_parachain::primitives::Id,
						runtime_types::cumulus_pallet_xcmp_queue::OutboundStatus,
						::core::primitive::bool,
						::core::primitive::u16,
						::core::primitive::u16,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<OutboundXcmpStatus>()? ==
						[
							53u8, 40u8, 96u8, 194u8, 88u8, 137u8, 82u8, 195u8, 139u8, 252u8, 36u8,
							217u8, 187u8, 174u8, 75u8, 66u8, 50u8, 172u8, 158u8, 85u8, 29u8, 198u8,
							194u8, 54u8, 120u8, 9u8, 208u8, 223u8, 121u8, 207u8, 103u8, 153u8,
						] {
						let entry = OutboundXcmpStatus;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub async fn outbound_xcmp_messages(
					&self,
					_0: &runtime_types::polkadot_parachain::primitives::Id,
					_1: &::core::primitive::u16,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<OutboundXcmpMessages>()? ==
						[
							212u8, 72u8, 186u8, 88u8, 124u8, 179u8, 78u8, 181u8, 148u8, 165u8,
							18u8, 215u8, 169u8, 42u8, 235u8, 246u8, 54u8, 52u8, 74u8, 13u8, 198u8,
							7u8, 97u8, 120u8, 182u8, 157u8, 4u8, 82u8, 223u8, 160u8, 170u8, 11u8,
						] {
						let entry = OutboundXcmpMessages(_0, _1);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The messages outbound in a given XCMP channel."]
				pub async fn outbound_xcmp_messages_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, OutboundXcmpMessages<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<OutboundXcmpMessages>()? ==
						[
							212u8, 72u8, 186u8, 88u8, 124u8, 179u8, 78u8, 181u8, 148u8, 165u8,
							18u8, 215u8, 169u8, 42u8, 235u8, 246u8, 54u8, 52u8, 74u8, 13u8, 198u8,
							7u8, 97u8, 120u8, 182u8, 157u8, 4u8, 82u8, 223u8, 160u8, 170u8, 11u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub async fn signal_messages(
					&self,
					_0: &runtime_types::polkadot_parachain::primitives::Id,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<::core::primitive::u8>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<SignalMessages>()? ==
						[
							103u8, 138u8, 202u8, 122u8, 216u8, 218u8, 251u8, 206u8, 7u8, 34u8,
							207u8, 21u8, 150u8, 7u8, 19u8, 247u8, 217u8, 27u8, 122u8, 242u8, 217u8,
							117u8, 90u8, 227u8, 247u8, 189u8, 91u8, 99u8, 94u8, 143u8, 252u8, 75u8,
						] {
						let entry = SignalMessages(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Any signal messages waiting to be sent."]
				pub async fn signal_messages_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, SignalMessages<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<SignalMessages>()? ==
						[
							103u8, 138u8, 202u8, 122u8, 216u8, 218u8, 251u8, 206u8, 7u8, 34u8,
							207u8, 21u8, 150u8, 7u8, 19u8, 247u8, 217u8, 27u8, 122u8, 242u8, 217u8,
							117u8, 90u8, 227u8, 247u8, 189u8, 91u8, 99u8, 94u8, 143u8, 252u8, 75u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The configuration which controls the dynamics of the outbound queue."]
				pub async fn queue_config(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<QueueConfig>()? ==
						[
							166u8, 70u8, 234u8, 240u8, 144u8, 245u8, 116u8, 18u8, 187u8, 129u8,
							3u8, 221u8, 13u8, 222u8, 98u8, 42u8, 64u8, 178u8, 185u8, 164u8, 66u8,
							1u8, 17u8, 74u8, 185u8, 118u8, 104u8, 29u8, 99u8, 152u8, 132u8, 25u8,
						] {
						let entry = QueueConfig;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod relayer_xcm {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Send {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
			}
			impl ::subxt::Call for Send {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "send";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct TeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			impl ::subxt::Call for TeleportAssets {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "teleport_assets";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			impl ::subxt::Call for ReserveTransferAssets {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "reserve_transfer_assets";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Execute {
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
				pub max_weight: ::core::primitive::u64,
			}
			impl ::subxt::Call for Execute {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "execute";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ForceXcmVersion {
				pub location:
					::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
				pub xcm_version: ::core::primitive::u32,
			}
			impl ::subxt::Call for ForceXcmVersion {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "force_xcm_version";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ForceDefaultXcmVersion {
				pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
			}
			impl ::subxt::Call for ForceDefaultXcmVersion {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "force_default_xcm_version";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ForceSubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			impl ::subxt::Call for ForceSubscribeVersionNotify {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "force_subscribe_version_notify";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ForceUnsubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			impl ::subxt::Call for ForceUnsubscribeVersionNotify {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "force_unsubscribe_version_notify";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct LimitedReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			impl ::subxt::Call for LimitedReserveTransferAssets {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "limited_reserve_transfer_assets";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct LimitedTeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			impl ::subxt::Call for LimitedTeleportAssets {
				const PALLET: &'static str = "RelayerXcm";
				const FUNCTION: &'static str = "limited_teleport_assets";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Send, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Send>()? ==
						[
							39u8, 55u8, 61u8, 33u8, 51u8, 200u8, 141u8, 158u8, 162u8, 162u8, 224u8,
							60u8, 42u8, 230u8, 36u8, 41u8, 133u8, 2u8, 100u8, 15u8, 253u8, 202u8,
							35u8, 247u8, 91u8, 138u8, 10u8, 220u8, 201u8, 80u8, 234u8, 166u8,
						] {
						let call = Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector and"]
				#[doc = "fee-weight is calculated locally and thus remote weights are assumed to be equal to"]
				#[doc = "local weights."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `dest_weight`: Equal to the total weight on `dest` of the XCM message"]
				#[doc = "  `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`."]
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						TeleportAssets,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<TeleportAssets>()? ==
						[
							55u8, 192u8, 217u8, 186u8, 230u8, 234u8, 26u8, 194u8, 243u8, 199u8,
							16u8, 227u8, 225u8, 88u8, 130u8, 219u8, 228u8, 110u8, 20u8, 255u8,
							233u8, 147u8, 121u8, 173u8, 126u8, 248u8, 192u8, 243u8, 211u8, 91u8,
							115u8, 148u8,
						] {
						let call = TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination chain and forward"]
				#[doc = "a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector and"]
				#[doc = "fee-weight is calculated locally and thus remote weights are assumed to be equal to"]
				#[doc = "local weights."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
				#[doc = "  `dest` side."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ReserveTransferAssets,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ReserveTransferAssets>()? ==
						[
							134u8, 229u8, 104u8, 209u8, 160u8, 7u8, 99u8, 175u8, 128u8, 110u8,
							189u8, 225u8, 141u8, 1u8, 10u8, 17u8, 247u8, 233u8, 146u8, 19u8, 31u8,
							145u8, 217u8, 144u8, 85u8, 223u8, 197u8, 249u8, 1u8, 222u8, 98u8, 13u8,
						] {
						let call = ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Execute an XCM message from a local, signed, origin."]
				#[doc = ""]
				#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
				#[doc = "partially."]
				#[doc = ""]
				#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
				#[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
				#[doc = "attempt will be made."]
				#[doc = ""]
				#[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
				#[doc = "to completion; only that *some* of it was executed."]
				pub fn execute(
					&self,
					message: runtime_types::xcm::VersionedXcm,
					max_weight: ::core::primitive::u64,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Execute,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Execute>()? ==
						[
							229u8, 158u8, 151u8, 136u8, 102u8, 70u8, 144u8, 119u8, 86u8, 10u8,
							203u8, 106u8, 159u8, 96u8, 33u8, 157u8, 218u8, 133u8, 45u8, 254u8,
							198u8, 217u8, 103u8, 175u8, 56u8, 173u8, 77u8, 39u8, 253u8, 125u8,
							31u8, 152u8,
						] {
						let call = Execute { message: ::std::boxed::Box::new(message), max_weight };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Extoll that a particular destination can be communicated with through a particular"]
				#[doc = "version of XCM."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The destination that is being described."]
				#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
				pub fn force_xcm_version(
					&self,
					location: runtime_types::xcm::v1::multilocation::MultiLocation,
					xcm_version: ::core::primitive::u32,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceXcmVersion,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceXcmVersion>()? ==
						[
							32u8, 219u8, 213u8, 152u8, 203u8, 73u8, 121u8, 64u8, 78u8, 53u8, 110u8,
							23u8, 87u8, 93u8, 34u8, 166u8, 205u8, 189u8, 25u8, 160u8, 172u8, 178u8,
							125u8, 182u8, 37u8, 254u8, 220u8, 179u8, 70u8, 252u8, 63u8, 94u8,
						] {
						let call = ForceXcmVersion {
							location: ::std::boxed::Box::new(location),
							xcm_version,
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
				#[doc = "version a destination can accept is unknown)."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceDefaultXcmVersion,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceDefaultXcmVersion>()? ==
						[
							44u8, 161u8, 28u8, 189u8, 162u8, 221u8, 14u8, 31u8, 8u8, 211u8, 181u8,
							51u8, 197u8, 14u8, 87u8, 198u8, 3u8, 240u8, 90u8, 78u8, 141u8, 131u8,
							205u8, 250u8, 211u8, 150u8, 237u8, 160u8, 239u8, 226u8, 233u8, 29u8,
						] {
						let call = ForceDefaultXcmVersion { maybe_xcm_version };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceSubscribeVersionNotify,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceSubscribeVersionNotify>()? ==
						[
							41u8, 248u8, 187u8, 195u8, 146u8, 143u8, 0u8, 246u8, 248u8, 38u8,
							128u8, 200u8, 143u8, 149u8, 127u8, 73u8, 3u8, 247u8, 106u8, 6u8, 56u8,
							50u8, 207u8, 234u8, 137u8, 201u8, 16u8, 21u8, 226u8, 148u8, 181u8,
							44u8,
						] {
						let call = ForceSubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
				#[doc = "version changes."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
				#[doc = "  notifications which we no longer desire."]
				pub fn force_unsubscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceUnsubscribeVersionNotify,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceUnsubscribeVersionNotify>()? ==
						[
							150u8, 202u8, 148u8, 13u8, 187u8, 169u8, 5u8, 60u8, 25u8, 144u8, 43u8,
							196u8, 35u8, 215u8, 184u8, 72u8, 143u8, 220u8, 176u8, 27u8, 100u8,
							245u8, 31u8, 243u8, 0u8, 83u8, 165u8, 7u8, 102u8, 172u8, 218u8, 133u8,
						] {
						let call = ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination chain and forward"]
				#[doc = "a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
				#[doc = "  `dest` side."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_reserve_transfer_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						LimitedReserveTransferAssets,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<LimitedReserveTransferAssets>()? ==
						[
							242u8, 206u8, 126u8, 164u8, 44u8, 116u8, 181u8, 90u8, 121u8, 124u8,
							120u8, 240u8, 129u8, 217u8, 131u8, 100u8, 248u8, 149u8, 56u8, 154u8,
							35u8, 91u8, 210u8, 118u8, 207u8, 110u8, 42u8, 249u8, 160u8, 155u8,
							251u8, 68u8,
						] {
						let call = LimitedReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `dest_weight`: Equal to the total weight on `dest` of the XCM message"]
				#[doc = "  `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						LimitedTeleportAssets,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<LimitedTeleportAssets>()? ==
						[
							189u8, 233u8, 43u8, 16u8, 158u8, 114u8, 154u8, 233u8, 179u8, 144u8,
							81u8, 179u8, 169u8, 38u8, 4u8, 130u8, 95u8, 237u8, 172u8, 167u8, 2u8,
							169u8, 53u8, 252u8, 159u8, 42u8, 143u8, 216u8, 112u8, 155u8, 48u8,
							129u8,
						] {
						let call = LimitedTeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Execution of an XCM message was attempted."]
			#[doc = ""]
			#[doc = "\\[ outcome \\]"]
			pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
			impl ::subxt::Event for Attempted {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A XCM message was sent."]
			#[doc = ""]
			#[doc = "\\[ origin, destination, message \\]"]
			pub struct Sent(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v2::Xcm,
			);
			impl ::subxt::Event for Sent {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "Sent";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Query response received which does not match a registered query. This may be because a"]
			#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
			#[doc = "because the query timed out."]
			#[doc = ""]
			#[doc = "\\[ origin location, id \\]"]
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for UnexpectedResponse {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
			#[doc = "no registered notification call."]
			#[doc = ""]
			#[doc = "\\[ id, response \\]"]
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::Response,
			);
			impl ::subxt::Event for ResponseReady {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Query response has been received and query is removed. The registered notification has"]
			#[doc = "been dispatched and executed successfully."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct Notified(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::Event for Notified {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "Notified";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Query response has been received and query is removed. The registered notification could"]
			#[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
			#[doc = "originally budgeted by this runtime for the query result."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
			pub struct NotifyOverweight(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for NotifyOverweight {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Query response has been received and query is removed. There was a general error with"]
			#[doc = "dispatching the notification call."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct NotifyDispatchError(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::Event for NotifyDispatchError {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
			#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
			#[doc = "is not `(origin, QueryId, Response)`."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct NotifyDecodeFailed(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::Event for NotifyDecodeFailed {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Expected query response has been received but the origin location of the response does"]
			#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
			#[doc = "be received and acted upon."]
			#[doc = ""]
			#[doc = "\\[ origin location, id, expected location \\]"]
			pub struct InvalidResponder(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
			);
			impl ::subxt::Event for InvalidResponder {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Expected query response has been received but the expected origin location placed in"]
			#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
			#[doc = ""]
			#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
			#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
			#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
			#[doc = "needed."]
			#[doc = ""]
			#[doc = "\\[ origin location, id \\]"]
			pub struct InvalidResponderVersion(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for InvalidResponderVersion {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			#[doc = "Received query response has been read and removed."]
			#[doc = ""]
			#[doc = "\\[ id \\]"]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::Event for ResponseTaken {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some assets have been placed in an asset trap."]
			#[doc = ""]
			#[doc = "\\[ hash, origin, assets \\]"]
			pub struct AssetsTrapped(
				pub ::subxt::sp_core::H256,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::Event for AssetsTrapped {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An XCM version change notification message has been attempted to be sent."]
			#[doc = ""]
			#[doc = "\\[ destination, result \\]"]
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for VersionChangeNotified {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The supported version of a location has been changed. This might be through an"]
			#[doc = "automatic notification or a manual intervention."]
			#[doc = ""]
			#[doc = "\\[ location, XCM version \\]"]
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::Event for SupportedVersionChanged {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "sending the notification to it."]
			#[doc = ""]
			#[doc = "\\[ location, query ID, error \\]"]
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::traits::Error,
			);
			impl ::subxt::Event for NotifyTargetSendFail {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "migrating the location to our new XCM format."]
			#[doc = ""]
			#[doc = "\\[ location, query ID \\]"]
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for NotifyTargetMigrationFail {
				const PALLET: &'static str = "RelayerXcm";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
		}
	}
	pub mod cumulus_xcm {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message is invalid XCM."]
			#[doc = "\\[ id \\]"]
			pub struct InvalidFormat(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::Event for InvalidFormat {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message is unsupported version of XCM."]
			#[doc = "\\[ id \\]"]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 8usize]);
			impl ::subxt::Event for UnsupportedVersion {
				const PALLET: &'static str = "CumulusXcm";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message executed with the given outcome."]
			#[doc = "\\[ id, outcome \\]"]
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
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: ::core::primitive::u64,
			}
			impl ::subxt::Call for ServiceOverweight {
				const PALLET: &'static str = "DmpQueue";
				const FUNCTION: &'static str = "service_overweight";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Service a single overweight message."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
				#[doc = "- `index`: The index of the overweight message to service."]
				#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
				#[doc = ""]
				#[doc = "Errors:"]
				#[doc = "- `Unknown`: Message of `index` is unknown."]
				#[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
				#[doc = ""]
				#[doc = "Events:"]
				#[doc = "- `OverweightServiced`: On success."]
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: ::core::primitive::u64,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ServiceOverweight,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ServiceOverweight>()? ==
						[
							229u8, 167u8, 106u8, 63u8, 141u8, 80u8, 8u8, 201u8, 156u8, 34u8, 47u8,
							104u8, 116u8, 57u8, 35u8, 216u8, 132u8, 3u8, 201u8, 169u8, 38u8, 107u8,
							149u8, 120u8, 42u8, 130u8, 100u8, 133u8, 214u8, 48u8, 99u8, 146u8,
						] {
						let call = ServiceOverweight { index, weight_limit };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message is invalid XCM."]
			#[doc = "\\[ id \\]"]
			pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::Event for InvalidFormat {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message is unsupported version of XCM."]
			#[doc = "\\[ id \\]"]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::Event for UnsupportedVersion {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message executed with the given outcome."]
			#[doc = "\\[ id, outcome \\]"]
			pub struct ExecutedDownward(
				pub [::core::primitive::u8; 32usize],
				pub runtime_types::xcm::v2::traits::Outcome,
			);
			impl ::subxt::Event for ExecutedDownward {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "ExecutedDownward";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "The weight limit for handling downward messages was reached."]
			#[doc = "\\[ id, remaining, required \\]"]
			pub struct WeightExhausted(
				pub [::core::primitive::u8; 32usize],
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for WeightExhausted {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "WeightExhausted";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message is overweight and was placed in the overweight queue."]
			#[doc = "\\[ id, index, required \\]"]
			pub struct OverweightEnqueued(
				pub [::core::primitive::u8; 32usize],
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::Event for OverweightEnqueued {
				const PALLET: &'static str = "DmpQueue";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Downward message from the overweight queue was executed."]
			#[doc = "\\[ index, used \\]"]
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
			pub struct Pages<'a>(pub &'a ::core::primitive::u32);
			impl ::subxt::StorageEntry for Pages<'_> {
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
			pub struct Overweight<'a>(pub &'a ::core::primitive::u64);
			impl ::subxt::StorageEntry for Overweight<'_> {
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
				#[doc = " The configuration."]
				pub async fn configuration(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_dmp_queue::ConfigData,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Configuration>()? ==
						[
							56u8, 109u8, 219u8, 202u8, 168u8, 10u8, 171u8, 185u8, 37u8, 140u8,
							78u8, 29u8, 152u8, 136u8, 67u8, 225u8, 10u8, 0u8, 185u8, 176u8, 133u8,
							142u8, 214u8, 253u8, 240u8, 148u8, 241u8, 66u8, 54u8, 19u8, 100u8,
							180u8,
						] {
						let entry = Configuration;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The page index."]
				pub async fn page_index(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<PageIndex>()? ==
						[
							122u8, 54u8, 173u8, 79u8, 103u8, 237u8, 132u8, 20u8, 204u8, 109u8,
							127u8, 98u8, 134u8, 55u8, 214u8, 32u8, 225u8, 138u8, 243u8, 105u8,
							104u8, 60u8, 19u8, 172u8, 228u8, 160u8, 226u8, 233u8, 41u8, 176u8,
							116u8, 162u8,
						] {
						let entry = PageIndex;
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The queue pages."]
				pub async fn pages(
					&self,
					_0: &::core::primitive::u32,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::std::vec::Vec<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Pages>()? ==
						[
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
						] {
						let entry = Pages(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The queue pages."]
				pub async fn pages_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Pages<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Pages>()? ==
						[
							228u8, 86u8, 33u8, 107u8, 248u8, 4u8, 223u8, 175u8, 222u8, 25u8, 204u8,
							42u8, 235u8, 21u8, 215u8, 91u8, 167u8, 14u8, 133u8, 151u8, 190u8, 57u8,
							138u8, 208u8, 79u8, 244u8, 132u8, 14u8, 48u8, 247u8, 171u8, 108u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The overweight messages."]
				pub async fn overweight(
					&self,
					_0: &::core::primitive::u64,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<(
						::core::primitive::u32,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Overweight>()? ==
						[
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
						] {
						let entry = Overweight(_0);
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The overweight messages."]
				pub async fn overweight_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Overweight<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Overweight>()? ==
						[
							222u8, 85u8, 143u8, 49u8, 42u8, 248u8, 138u8, 163u8, 46u8, 199u8,
							188u8, 61u8, 137u8, 135u8, 127u8, 146u8, 210u8, 254u8, 121u8, 42u8,
							112u8, 114u8, 22u8, 228u8, 207u8, 207u8, 245u8, 175u8, 152u8, 140u8,
							225u8, 237u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod liquid_crowdloan {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct MakeClaimable;
			impl ::subxt::Call for MakeClaimable {
				const PALLET: &'static str = "LiquidCrowdloan";
				const FUNCTION: &'static str = "make_claimable";
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct Claim {
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::Call for Claim {
				const PALLET: &'static str = "LiquidCrowdloan";
				const FUNCTION: &'static str = "claim";
			}
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				pub fn make_claimable(
					&self,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						MakeClaimable,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<MakeClaimable>()? ==
						[
							245u8, 254u8, 25u8, 62u8, 213u8, 193u8, 21u8, 202u8, 160u8, 136u8,
							117u8, 181u8, 195u8, 97u8, 190u8, 222u8, 199u8, 92u8, 219u8, 125u8,
							209u8, 10u8, 94u8, 0u8, 121u8, 22u8, 108u8, 236u8, 173u8, 85u8, 117u8,
							231u8,
						] {
						let call = MakeClaimable {};
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Attempts to claim some crowdloan bonus from the crowdloan pot."]
				#[doc = "No-op if amount is zero."]
				pub fn claim(
					&self,
					amount: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<'a, T, X, Claim, DispatchError, root_mod::Event>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Claim>()? ==
						[
							0u8, 24u8, 23u8, 240u8, 120u8, 23u8, 201u8, 32u8, 78u8, 173u8, 122u8,
							34u8, 94u8, 100u8, 138u8, 249u8, 164u8, 52u8, 38u8, 164u8, 50u8, 17u8,
							86u8, 121u8, 228u8, 215u8, 42u8, 254u8, 202u8, 71u8, 243u8, 132u8,
						] {
						let call = Claim { amount };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::pallet_crowdloan_bonus::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Initiated(pub runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::Event for Initiated {
				const PALLET: &'static str = "LiquidCrowdloan";
				const EVENT: &'static str = "Initiated";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::core::option::Option<::core::primitive::bool>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<IsClaimable>()? ==
						[
							12u8, 13u8, 144u8, 42u8, 73u8, 124u8, 143u8, 97u8, 133u8, 168u8, 52u8,
							92u8, 210u8, 242u8, 87u8, 186u8, 159u8, 45u8, 72u8, 95u8, 60u8, 28u8,
							63u8, 187u8, 101u8, 131u8, 23u8, 83u8, 125u8, 64u8, 159u8, 224u8,
						] {
						let entry = IsClaimable;
						self.client.storage().fetch(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod tokens {
		use super::{root_mod, runtime_types};
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			pub struct TransactionApi<'a, T: ::subxt::Config, X> {
				client: &'a ::subxt::Client<T>,
				marker: ::core::marker::PhantomData<X>,
			}
			impl<'a, T, X> TransactionApi<'a, T, X>
			where
				T: ::subxt::Config,
				X: ::subxt::extrinsic::ExtrinsicParams<T>,
			{
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client, marker: ::core::marker::PhantomData }
				}
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
				#[doc = "It will decrease the total issuance of the system by the"]
				#[doc = "`TransferFee`. If the sender's account is below the existential"]
				#[doc = "deposit as a result of the transfer, the account will be reaped."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the"]
				#[doc = "transactor."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `amount`: free balance amount to tranfer."]
				pub fn transfer(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						Transfer,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<Transfer>()? ==
						[
							253u8, 229u8, 247u8, 0u8, 236u8, 248u8, 183u8, 214u8, 30u8, 174u8,
							223u8, 182u8, 70u8, 203u8, 251u8, 232u8, 7u8, 79u8, 133u8, 44u8, 10u8,
							74u8, 251u8, 48u8, 114u8, 143u8, 142u8, 64u8, 10u8, 90u8, 149u8, 90u8,
						] {
						let call = Transfer { dest, currency_id, amount };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Transfer all remaining balance to the given account."]
				#[doc = ""]
				#[doc = "NOTE: This function only attempts to transfer _transferable_"]
				#[doc = "balances. This means that any locked, reserved, or existential"]
				#[doc = "deposits (when `keep_alive` is `true`), will not be transferred by"]
				#[doc = "this function. To ensure that this function results in a killed"]
				#[doc = "account, you might need to prepare the account by removing any"]
				#[doc = "reference counters, storage deposits, etc..."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the"]
				#[doc = "transactor."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all`"]
				#[doc = "  operation should send all of the funds the account has, causing"]
				#[doc = "  the sender account to be killed (false), or transfer everything"]
				#[doc = "  except at least the existential deposit, which will guarantee to"]
				#[doc = "  keep the sender account alive (true)."]
				pub fn transfer_all(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					keep_alive: ::core::primitive::bool,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						TransferAll,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<TransferAll>()? ==
						[
							5u8, 215u8, 36u8, 5u8, 187u8, 245u8, 152u8, 214u8, 81u8, 200u8, 17u8,
							114u8, 134u8, 231u8, 77u8, 123u8, 30u8, 251u8, 50u8, 158u8, 187u8,
							159u8, 141u8, 29u8, 62u8, 159u8, 202u8, 60u8, 214u8, 68u8, 6u8, 141u8,
						] {
						let call = TransferAll { dest, currency_id, keep_alive };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Same as the [`transfer`] call, but with a check that the transfer"]
				#[doc = "will not kill the origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer`] instead."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be `Signed` by the"]
				#[doc = "transactor."]
				#[doc = ""]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `amount`: free balance amount to tranfer."]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					amount: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						TransferKeepAlive,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<TransferKeepAlive>()? ==
						[
							47u8, 99u8, 198u8, 98u8, 112u8, 113u8, 220u8, 88u8, 95u8, 98u8, 57u8,
							230u8, 34u8, 198u8, 170u8, 144u8, 89u8, 19u8, 248u8, 231u8, 254u8,
							193u8, 216u8, 39u8, 162u8, 104u8, 248u8, 43u8, 149u8, 222u8, 13u8,
							158u8,
						] {
						let call = TransferKeepAlive { dest, currency_id, amount };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Exactly as `transfer`, except the origin must be root and the source"]
				#[doc = "account may be specified."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Root_."]
				#[doc = ""]
				#[doc = "- `source`: The sender of the transfer."]
				#[doc = "- `dest`: The recipient of the transfer."]
				#[doc = "- `currency_id`: currency type."]
				#[doc = "- `amount`: free balance amount to tranfer."]
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
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						ForceTransfer,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<ForceTransfer>()? ==
						[
							69u8, 121u8, 194u8, 112u8, 41u8, 101u8, 99u8, 114u8, 129u8, 95u8,
							152u8, 97u8, 126u8, 91u8, 186u8, 152u8, 15u8, 209u8, 106u8, 208u8,
							95u8, 215u8, 94u8, 171u8, 225u8, 249u8, 27u8, 108u8, 90u8, 107u8, 89u8,
							66u8,
						] {
						let call = ForceTransfer { source, dest, currency_id, amount };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = "Set the balances of a given account."]
				#[doc = ""]
				#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it"]
				#[doc = "will also decrease the total issuance of the system"]
				#[doc = "(`TotalIssuance`). If the new free or reserved balance is below the"]
				#[doc = "existential deposit, it will reap the `AccountInfo`."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn set_balance(
					&self,
					who: ::subxt::sp_runtime::MultiAddress<
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u32,
					>,
					currency_id: runtime_types::primitives::currency::CurrencyId,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> Result<
					::subxt::SubmittableExtrinsic<
						'a,
						T,
						X,
						SetBalance,
						DispatchError,
						root_mod::Event,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().call_hash::<SetBalance>()? ==
						[
							184u8, 87u8, 202u8, 34u8, 234u8, 64u8, 225u8, 32u8, 35u8, 73u8, 143u8,
							49u8, 135u8, 230u8, 216u8, 24u8, 158u8, 137u8, 131u8, 136u8, 206u8,
							45u8, 89u8, 201u8, 113u8, 45u8, 219u8, 188u8, 25u8, 154u8, 215u8, 49u8,
						] {
						let call = SetBalance { who, currency_id, new_free, new_reserved };
						Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub type Event = runtime_types::orml_tokens::module::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An account was created with some free balance. \\[currency_id,"]
			#[doc = "account, free_balance\\]"]
			pub struct Endowed(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Endowed {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "An account was removed whose balance was non-zero but below"]
			#[doc = "ExistentialDeposit, resulting in an outright loss. \\[currency_id,"]
			#[doc = "account, balance\\]"]
			pub struct DustLost(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for DustLost {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Transfer succeeded. \\[currency_id, from, to, value\\]"]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			#[doc = "\\[currency_id, who, value\\]"]
			pub struct Reserved(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Reserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			#[doc = "\\[currency_id, who, value\\]"]
			pub struct Unreserved(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
			);
			impl ::subxt::Event for Unreserved {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "Some reserved balance was repatriated (moved from reserved to"]
			#[doc = "another account)."]
			#[doc = "\\[currency_id, from, to, amount_actually_moved, status\\]"]
			pub struct RepatriatedReserve(
				pub runtime_types::primitives::currency::CurrencyId,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::subxt::sp_core::crypto::AccountId32,
				pub ::core::primitive::u128,
				pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			);
			impl ::subxt::Event for RepatriatedReserve {
				const PALLET: &'static str = "Tokens";
				const EVENT: &'static str = "RepatriatedReserve";
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			#[doc = "A balance was set by root. \\[who, free, reserved\\]"]
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
			pub struct TotalIssuance<'a>(pub &'a runtime_types::primitives::currency::CurrencyId);
			impl ::subxt::StorageEntry for TotalIssuance<'_> {
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
			pub struct Locks<'a>(
				pub &'a ::subxt::sp_core::crypto::AccountId32,
				pub &'a runtime_types::primitives::currency::CurrencyId,
			);
			impl ::subxt::StorageEntry for Locks<'_> {
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
			pub struct Accounts<'a>(
				pub &'a ::subxt::sp_core::crypto::AccountId32,
				pub &'a runtime_types::primitives::currency::CurrencyId,
			);
			impl ::subxt::StorageEntry for Accounts<'_> {
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
				#[doc = " The total issuance of a token type."]
				pub async fn total_issuance(
					&self,
					_0: &runtime_types::primitives::currency::CurrencyId,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError> {
					if self.client.metadata().storage_hash::<TotalIssuance>()? ==
						[
							241u8, 129u8, 5u8, 167u8, 216u8, 197u8, 164u8, 93u8, 33u8, 196u8, 21u8,
							6u8, 165u8, 64u8, 194u8, 9u8, 168u8, 124u8, 131u8, 45u8, 93u8, 214u8,
							69u8, 181u8, 154u8, 80u8, 199u8, 207u8, 45u8, 237u8, 17u8, 105u8,
						] {
						let entry = TotalIssuance(_0);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The total issuance of a token type."]
				pub async fn total_issuance_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, TotalIssuance<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<TotalIssuance>()? ==
						[
							241u8, 129u8, 5u8, 167u8, 216u8, 197u8, 164u8, 93u8, 33u8, 196u8, 21u8,
							6u8, 165u8, 64u8, 194u8, 9u8, 168u8, 124u8, 131u8, 45u8, 93u8, 214u8,
							69u8, 181u8, 154u8, 80u8, 199u8, 207u8, 45u8, 237u8, 17u8, 105u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Any liquidity locks of a token type under an account."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub async fn locks(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					_1: &runtime_types::primitives::currency::CurrencyId,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::frame_support::storage::bounded_vec::BoundedVec<
						runtime_types::orml_tokens::BalanceLock<::core::primitive::u128>,
					>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Locks>()? ==
						[
							106u8, 95u8, 21u8, 107u8, 207u8, 248u8, 136u8, 57u8, 249u8, 58u8,
							181u8, 195u8, 161u8, 8u8, 239u8, 232u8, 69u8, 203u8, 119u8, 82u8,
							248u8, 105u8, 115u8, 126u8, 54u8, 177u8, 115u8, 41u8, 164u8, 144u8,
							14u8, 202u8,
						] {
						let entry = Locks(_0, _1);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " Any liquidity locks of a token type under an account."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub async fn locks_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
				{
					if self.client.metadata().storage_hash::<Locks>()? ==
						[
							106u8, 95u8, 21u8, 107u8, 207u8, 248u8, 136u8, 57u8, 249u8, 58u8,
							181u8, 195u8, 161u8, 8u8, 239u8, 232u8, 69u8, 203u8, 119u8, 82u8,
							248u8, 105u8, 115u8, 126u8, 54u8, 177u8, 115u8, 41u8, 164u8, 144u8,
							14u8, 202u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The balance of a token type under an account."]
				#[doc = ""]
				#[doc = " NOTE: If the total is ever zero, decrease account ref account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this module is used to store"]
				#[doc = " balances."]
				pub async fn accounts(
					&self,
					_0: &::subxt::sp_core::crypto::AccountId32,
					_1: &runtime_types::primitives::currency::CurrencyId,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					runtime_types::orml_tokens::AccountData<::core::primitive::u128>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Accounts>()? ==
						[
							94u8, 147u8, 137u8, 73u8, 118u8, 3u8, 43u8, 29u8, 26u8, 78u8, 23u8,
							2u8, 162u8, 111u8, 102u8, 126u8, 166u8, 97u8, 85u8, 107u8, 56u8, 34u8,
							3u8, 107u8, 93u8, 134u8, 20u8, 219u8, 146u8, 202u8, 24u8, 89u8,
						] {
						let entry = Accounts(_0, _1);
						self.client.storage().fetch_or_default(&entry, block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
				#[doc = " The balance of a token type under an account."]
				#[doc = ""]
				#[doc = " NOTE: If the total is ever zero, decrease account ref account."]
				#[doc = ""]
				#[doc = " NOTE: This is only used in the case that this module is used to store"]
				#[doc = " balances."]
				pub async fn accounts_iter(
					&self,
					block_hash: ::core::option::Option<T::Hash>,
				) -> ::core::result::Result<
					::subxt::KeyIter<'a, T, Accounts<'a>>,
					::subxt::BasicError,
				> {
					if self.client.metadata().storage_hash::<Accounts>()? ==
						[
							94u8, 147u8, 137u8, 73u8, 118u8, 3u8, 43u8, 29u8, 26u8, 78u8, 23u8,
							2u8, 162u8, 111u8, 102u8, 126u8, 166u8, 97u8, 85u8, 107u8, 56u8, 34u8,
							3u8, 107u8, 93u8, 134u8, 20u8, 219u8, 146u8, 202u8, 24u8, 89u8,
						] {
						self.client.storage().iter(block_hash).await
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi<'a, T: ::subxt::Config> {
				client: &'a ::subxt::Client<T>,
			}
			impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
				pub fn new(client: &'a ::subxt::Client<T>) -> Self {
					Self { client }
				}
				pub fn max_locks(
					&self,
				) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError> {
					if self.client.metadata().constant_hash("Tokens", "MaxLocks")? ==
						[
							250u8, 58u8, 19u8, 15u8, 35u8, 113u8, 227u8, 89u8, 39u8, 75u8, 21u8,
							108u8, 202u8, 32u8, 163u8, 167u8, 207u8, 233u8, 69u8, 151u8, 53u8,
							164u8, 230u8, 16u8, 14u8, 22u8, 172u8, 46u8, 36u8, 216u8, 29u8, 1u8,
						] {
						let pallet = self.client.metadata().pallet("Tokens")?;
						let constant = pallet.constant("MaxLocks")?;
						let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
						Ok(value)
					} else {
						Err(::subxt::MetadataError::IncompatibleMetadata.into())
					}
				}
			}
		}
	}
	pub mod runtime_types {
		use super::runtime_types;
		pub mod composable_runtime {
			use super::runtime_types;
			pub mod opaque {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct SessionKeys {
					pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				RelayerXcm(runtime_types::pallet_xcm::pallet::Call),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Call),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
				#[codec(index = 50)]
				LiquidCrowdloan(runtime_types::pallet_crowdloan_bonus::pallet::Call),
				#[codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Call),
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				RelayerXcm(runtime_types::pallet_xcm::pallet::Event),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
				#[codec(index = 43)]
				DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
				#[codec(index = 50)]
				LiquidCrowdloan(runtime_types::pallet_crowdloan_bonus::pallet::Event),
				#[codec(index = 52)]
				Tokens(runtime_types::orml_tokens::module::Event),
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				RelayerXcm(runtime_types::pallet_xcm::pallet::Origin),
				#[codec(index = 42)]
				CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
				#[codec(index = 4)]
				Void(runtime_types::sp_core::Void),
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct Runtime;
		}
		pub mod cumulus_pallet_dmp_queue {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Service a single overweight message."]
					#[doc = ""]
					#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
					#[doc = "- `index`: The index of the overweight message to service."]
					#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
					#[doc = ""]
					#[doc = "Errors:"]
					#[doc = "- `Unknown`: Message of `index` is unknown."]
					#[doc = "- `OverLimit`: Message execution may use greater than `weight_limit`."]
					#[doc = ""]
					#[doc = "Events:"]
					#[doc = "- `OverweightServiced`: On success."]
					service_overweight {
						index: ::core::primitive::u64,
						weight_limit: ::core::primitive::u64,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The message index given is unknown."]
					Unknown,
					#[codec(index = 1)]
					#[doc = "The amount of weight given is possibly not enough for executing the message."]
					OverLimit,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					#[doc = "\\[ id \\]"]
					InvalidFormat([::core::primitive::u8; 32usize]),
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					#[doc = "\\[ id \\]"]
					UnsupportedVersion([::core::primitive::u8; 32usize]),
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					#[doc = "\\[ id, outcome \\]"]
					ExecutedDownward(
						[::core::primitive::u8; 32usize],
						runtime_types::xcm::v2::traits::Outcome,
					),
					#[codec(index = 3)]
					#[doc = "The weight limit for handling downward messages was reached."]
					#[doc = "\\[ id, remaining, required \\]"]
					WeightExhausted(
						[::core::primitive::u8; 32usize],
						::core::primitive::u64,
						::core::primitive::u64,
					),
					#[codec(index = 4)]
					#[doc = "Downward message is overweight and was placed in the overweight queue."]
					#[doc = "\\[ id, index, required \\]"]
					OverweightEnqueued(
						[::core::primitive::u8; 32usize],
						::core::primitive::u64,
						::core::primitive::u64,
					),
					#[codec(index = 5)]
					#[doc = "Downward message from the overweight queue was executed."]
					#[doc = "\\[ index, used \\]"]
					OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
				}
			}
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct ConfigData {
				pub max_individual: ::core::primitive::u64,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					# [codec (index = 0)] # [doc = "Set the current validation data."] # [doc = ""] # [doc = "This should be invoked exactly once per block. It will panic at the finalization"] # [doc = "phase if the call was not invoked."] # [doc = ""] # [doc = "The dispatch origin for this call must be `Inherent`"] # [doc = ""] # [doc = "As a side effect, this function upgrades the current validation function"] # [doc = "if the appropriate time has come."] set_validation_data { data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData , } , # [codec (index = 1)] sudo_send_upward_message { message : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 2)] authorize_upgrade { code_hash : :: subxt :: sp_core :: H256 , } , # [codec (index = 3)] enact_authorized_upgrade { code : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , }
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to upgrade validation function while existing upgrade pending"]
					OverlappingUpgrades,
					#[codec(index = 1)]
					#[doc = "Polkadot currently prohibits this parachain from upgrading its validation function"]
					ProhibitedByPolkadot,
					#[codec(index = 2)]
					#[doc = "The supplied validation function has compiled into a blob larger than Polkadot is"]
					#[doc = "willing to run"]
					TooBig,
					#[codec(index = 3)]
					#[doc = "The inherent which supplies the validation data did not run this block"]
					ValidationDataNotAvailable,
					#[codec(index = 4)]
					#[doc = "The inherent which supplies the host configuration did not run this block"]
					HostConfigurationNotAvailable,
					#[codec(index = 5)]
					#[doc = "No validation function upgrade is currently scheduled."]
					NotScheduled,
					#[codec(index = 6)]
					#[doc = "No code upgrade has been authorized."]
					NothingAuthorized,
					#[codec(index = 7)]
					#[doc = "The given code upgrade has not been authorized."]
					Unauthorized,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The validation function has been scheduled to apply."]
					ValidationFunctionStored,
					#[codec(index = 1)]
					#[doc = "The validation function was applied as of the contained relay chain block number."]
					ValidationFunctionApplied(::core::primitive::u32),
					#[codec(index = 2)]
					#[doc = "The relay-chain aborted the upgrade process."]
					ValidationFunctionDiscarded,
					#[codec(index = 3)]
					#[doc = "An upgrade has been authorized."]
					UpgradeAuthorized(::subxt::sp_core::H256),
					#[codec(index = 4)]
					#[doc = "Some downward messages have been received and will be processed."]
					#[doc = "\\[ count \\]"]
					DownwardMessagesReceived(::core::primitive::u32),
					#[codec(index = 5)]
					#[doc = "Downward messages were processed using the given weight."]
					#[doc = "\\[ weight_used, result_mqc_head \\]"]
					DownwardMessagesProcessed(::core::primitive::u64, ::subxt::sp_core::H256),
				}
			}
			pub mod relay_state_snapshot {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct MessageQueueChain(pub ::subxt::sp_core::H256);
		}
		pub mod cumulus_pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Downward message is invalid XCM."]
					#[doc = "\\[ id \\]"]
					InvalidFormat([::core::primitive::u8; 8usize]),
					#[codec(index = 1)]
					#[doc = "Downward message is unsupported version of XCM."]
					#[doc = "\\[ id \\]"]
					UnsupportedVersion([::core::primitive::u8; 8usize]),
					#[codec(index = 2)]
					#[doc = "Downward message executed with the given outcome."]
					#[doc = "\\[ id, outcome \\]"]
					ExecutedDownward(
						[::core::primitive::u8; 8usize],
						runtime_types::xcm::v2::traits::Outcome,
					),
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Failed to send XCM message."]
					FailedToSend,
					#[codec(index = 1)]
					#[doc = "Bad XCM origin."]
					BadXcmOrigin,
					#[codec(index = 2)]
					#[doc = "Bad XCM data."]
					BadXcm,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Some XCM was executed ok."]
					Success(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 1)]
					#[doc = "Some XCM failed."]
					Fail(
						::core::option::Option<::subxt::sp_core::H256>,
						runtime_types::xcm::v2::traits::Error,
					),
					#[codec(index = 2)]
					#[doc = "Bad XCM version used."]
					BadVersion(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 3)]
					#[doc = "Bad XCM format used."]
					BadFormat(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 4)]
					#[doc = "An upward message was sent to the relay chain."]
					UpwardMessageSent(::core::option::Option<::subxt::sp_core::H256>),
					#[codec(index = 5)]
					#[doc = "An HRMP message was sent to a sibling parachain."]
					XcmpMessageSent(::core::option::Option<::subxt::sp_core::H256>),
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum InboundStatus {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum OutboundStatus {
				#[codec(index = 0)]
				Ok,
				#[codec(index = 1)]
				Suspended,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct QueueConfigData {
				pub suspend_threshold: ::core::primitive::u32,
				pub drop_threshold: ::core::primitive::u32,
				pub resume_threshold: ::core::primitive::u32,
				pub threshold_weight: ::core::primitive::u64,
				pub weight_restrict_decay: ::core::primitive::u64,
			}
		}
		pub mod cumulus_primitives_parachain_inherent {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				pub horizontal_messages: ::subxt::KeyedVec<
					runtime_types::polkadot_parachain::primitives::Id,
					::std::vec::Vec<
						runtime_types::polkadot_core_primitives::InboundHrmpMessage<
							::core::primitive::u32,
						>,
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
				pub mod weak_bounded_vec {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod tokens {
					use super::runtime_types;
					pub mod misc {
						use super::runtime_types;
						#[derive(
							:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug,
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct DispatchInfo {
					pub weight: ::core::primitive::u64,
					pub class: runtime_types::frame_support::weights::DispatchClass,
					pub pays_fee: runtime_types::frame_support::weights::Pays,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct RuntimeDbWeight {
					pub read: ::core::primitive::u64,
					pub write: ::core::primitive::u64,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct WeightToFeeCoefficient<_0> {
					pub coeff_integer: _0,
					pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
					pub negative: ::core::primitive::bool,
					pub degree: ::core::primitive::u8,
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::weights::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct BlockWeights {
					pub base_block: ::core::primitive::u64,
					pub max_block: ::core::primitive::u64,
					pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct WeightsPerClass {
					pub base_extrinsic: ::core::primitive::u64,
					pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
					pub max_total: ::core::option::Option<::core::primitive::u64>,
					pub reserved: ::core::option::Option<::core::primitive::u64>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "A dispatch that will fill the block weight up to the given ratio."]
					fill_block { ratio: runtime_types::sp_arithmetic::per_things::Perbill },
					#[codec(index = 1)]
					#[doc = "Make some on-chain remark."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`"]
					#[doc = "# </weight>"]
					remark { remark: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 2)]
					#[doc = "Set the number of pages in the WebAssembly environment's heap."]
					set_heap_pages { pages: ::core::primitive::u64 },
					#[codec(index = 3)]
					#[doc = "Set the new runtime code."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
					#[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
					#[doc = "  expensive)."]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
					#[doc = "expensive. We will treat this as a full block."]
					#[doc = "# </weight>"]
					set_code { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					#[doc = "Set the new runtime code without doing any checks of the given `code`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(C)` where `C` length of `code`"]
					#[doc = "- 1 storage write (codec `O(C)`)."]
					#[doc = "- 1 digest item."]
					#[doc = "- 1 event."]
					#[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
					#[doc = "block. # </weight>"]
					set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 5)]
					#[doc = "Set some items of storage."]
					set_storage {
						items: ::std::vec::Vec<(
							::std::vec::Vec<::core::primitive::u8>,
							::std::vec::Vec<::core::primitive::u8>,
						)>,
					},
					#[codec(index = 6)]
					#[doc = "Kill some items from storage."]
					kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
					#[codec(index = 7)]
					#[doc = "Kill all storage items with a key that starts with the given prefix."]
					#[doc = ""]
					#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
					#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
					kill_prefix {
						prefix: ::std::vec::Vec<::core::primitive::u8>,
						subkeys: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "Make some on-chain remark and emit event."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(b)` where b is the length of the remark."]
					#[doc = "- 1 event."]
					#[doc = "# </weight>"]
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The name of specification does not match between the current runtime"]
					#[doc = "and the new runtime."]
					InvalidSpecName,
					#[codec(index = 1)]
					#[doc = "The specification version is not allowed to decrease between the current runtime"]
					#[doc = "and the new runtime."]
					SpecVersionNeedsToIncrease,
					#[codec(index = 2)]
					#[doc = "Failed to extract the runtime version from the new runtime."]
					#[doc = ""]
					#[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
					FailedToExtractRuntimeVersion,
					#[codec(index = 3)]
					#[doc = "Suicide called when the account has non-default composite data."]
					NonDefaultComposite,
					#[codec(index = 4)]
					#[doc = "There is a non-zero reference count preventing the account from being purged."]
					NonZeroRefCount,
					#[codec(index = 5)]
					#[doc = "The origin filter prevent the call to be dispatched."]
					CallFiltered,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully. \\[info\\]"]
					ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
					#[codec(index = 1)]
					#[doc = "An extrinsic failed. \\[error, info\\]"]
					ExtrinsicFailed(
						runtime_types::sp_runtime::DispatchError,
						runtime_types::frame_support::weights::DispatchInfo,
					),
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new \\[account\\] was created."]
					NewAccount(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 4)]
					#[doc = "An \\[account\\] was reaped."]
					KilledAccount(::subxt::sp_core::crypto::AccountId32),
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened. \\[origin, remark_hash\\]"]
					Remarked(::subxt::sp_core::crypto::AccountId32, ::subxt::sp_core::H256),
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "It will decrease the total issuance of the system by the"]
					#[doc = "`TransferFee`. If the sender's account is below the existential"]
					#[doc = "deposit as a result of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the"]
					#[doc = "transactor."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `amount`: free balance amount to tranfer."]
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
					#[doc = "Transfer all remaining balance to the given account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_"]
					#[doc = "balances. This means that any locked, reserved, or existential"]
					#[doc = "deposits (when `keep_alive` is `true`), will not be transferred by"]
					#[doc = "this function. To ensure that this function results in a killed"]
					#[doc = "account, you might need to prepare the account by removing any"]
					#[doc = "reference counters, storage deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the"]
					#[doc = "transactor."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all`"]
					#[doc = "  operation should send all of the funds the account has, causing"]
					#[doc = "  the sender account to be killed (false), or transfer everything"]
					#[doc = "  except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true)."]
					transfer_all {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						currency_id: runtime_types::primitives::currency::CurrencyId,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 2)]
					#[doc = "Same as the [`transfer`] call, but with a check that the transfer"]
					#[doc = "will not kill the origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer`] instead."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the"]
					#[doc = "transactor."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `amount`: free balance amount to tranfer."]
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
					#[doc = "Exactly as `transfer`, except the origin must be root and the source"]
					#[doc = "account may be specified."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `source`: The sender of the transfer."]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `currency_id`: currency type."]
					#[doc = "- `amount`: free balance amount to tranfer."]
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
					#[doc = "Set the balances of a given account."]
					#[doc = ""]
					#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it"]
					#[doc = "will also decrease the total issuance of the system"]
					#[doc = "(`TotalIssuance`). If the new free or reserved balance is below the"]
					#[doc = "existential deposit, it will reap the `AccountInfo`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The balance is too low"]
					BalanceTooLow,
					#[codec(index = 1)]
					#[doc = "Cannot convert Amount into Balance type"]
					AmountIntoBalanceFailed,
					#[codec(index = 2)]
					#[doc = "Failed because liquidity restrictions due to locking"]
					LiquidityRestrictions,
					#[codec(index = 3)]
					#[doc = "Failed because the maximum locks was exceeded"]
					MaxLocksExceeded,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account"]
					KeepAlive,
					#[codec(index = 5)]
					#[doc = "Value too low to create account due to existential deposit"]
					ExistentialDeposit,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist"]
					DeadAccount,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance. \\[currency_id,"]
					#[doc = "account, free_balance\\]"]
					Endowed(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below"]
					#[doc = "ExistentialDeposit, resulting in an outright loss. \\[currency_id,"]
					#[doc = "account, balance\\]"]
					DustLost(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 2)]
					#[doc = "Transfer succeeded. \\[currency_id, from, to, value\\]"]
					Transfer(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 3)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					#[doc = "\\[currency_id, who, value\\]"]
					Reserved(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 4)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					#[doc = "\\[currency_id, who, value\\]"]
					Unreserved(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					),
					#[codec(index = 5)]
					#[doc = "Some reserved balance was repatriated (moved from reserved to"]
					#[doc = "another account)."]
					#[doc = "\\[currency_id, from, to, amount_actually_moved, status\\]"]
					RepatriatedReserve(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					),
					#[codec(index = 6)]
					#[doc = "A balance was set by root. \\[who, free, reserved\\]"]
					BalanceSet(
						runtime_types::primitives::currency::CurrencyId,
						::subxt::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::u128,
					),
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub frozen: _0,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
			}
		}
		pub mod pallet_authorship {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Provide a set of uncles."]
					set_uncles {
						new_uncles: ::std::vec::Vec<
							runtime_types::sp_runtime::generic::header::Header<
								::core::primitive::u32,
								runtime_types::sp_runtime::traits::BlakeTwo256,
							>,
						>,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The uncle parent not in the chain."]
					InvalidUncleParent,
					#[codec(index = 1)]
					#[doc = "Uncles already set in the block."]
					UnclesAlreadySet,
					#[codec(index = 2)]
					#[doc = "Too many uncles."]
					TooManyUncles,
					#[codec(index = 3)]
					#[doc = "The uncle is genesis."]
					GenesisUncle,
					#[codec(index = 4)]
					#[doc = "The uncle is too high in chain."]
					TooHighUncle,
					#[codec(index = 5)]
					#[doc = "The uncle is already included."]
					UncleAlreadyIncluded,
					#[codec(index = 6)]
					#[doc = "The uncle isn't recent enough to be included."]
					OldUncle,
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
					#[doc = "It will decrease the total issuance of the system by the `TransferFee`."]
					#[doc = "If the sender's account is below the existential deposit as a result"]
					#[doc = "of the transfer, the account will be reaped."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
					#[doc = "  types. See related functions below."]
					#[doc = "- It contains a limited number of reads and writes internally and no complex"]
					#[doc = "  computation."]
					#[doc = ""]
					#[doc = "Related functions:"]
					#[doc = ""]
					#[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
					#[doc = "  - Transferring balances to accounts that did not exist before will cause"]
					#[doc = "    `T::OnNewAccount::on_new_account` to be called."]
					#[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
					#[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
					#[doc = "    that the transfer will not kill the origin account."]
					#[doc = "---------------------------------"]
					#[doc = "- Origin account is already in memory, so no DB operations for them."]
					#[doc = "# </weight>"]
					transfer {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Set the balances of a given account."]
					#[doc = ""]
					#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
					#[doc = "also decrease the total issuance of the system (`TotalIssuance`)."]
					#[doc = "If the new free or reserved balance is below the existential deposit,"]
					#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
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
					#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
					#[doc = "specified."]
					#[doc = "# <weight>"]
					#[doc = "- Same as transfer, but additional read and write because the source account is not"]
					#[doc = "  assumed to be in the overlay."]
					#[doc = "# </weight>"]
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
					#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
					#[doc = "origin account."]
					#[doc = ""]
					#[doc = "99% of the time you want [`transfer`] instead."]
					#[doc = ""]
					#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
					transfer_keep_alive {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Transfer the entire transferable balance from the caller account."]
					#[doc = ""]
					#[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
					#[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
					#[doc = "transferred by this function. To ensure that this function results in a killed account,"]
					#[doc = "you might need to prepare the account by removing any reference counters, storage"]
					#[doc = "deposits, etc..."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be Signed."]
					#[doc = ""]
					#[doc = "- `dest`: The recipient of the transfer."]
					#[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
					#[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
					#[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
					#[doc = "  keep the sender account alive (true). # <weight>"]
					#[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
					#[doc = "  #</weight>"]
					transfer_all {
						dest: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						amount: ::core::primitive::u128,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Vesting balance too high to send value"]
					VestingBalance,
					#[codec(index = 1)]
					#[doc = "Account liquidity restrictions prevent withdrawal"]
					LiquidityRestrictions,
					#[codec(index = 2)]
					#[doc = "Balance too low to send value"]
					InsufficientBalance,
					#[codec(index = 3)]
					#[doc = "Value too low to create account due to existential deposit"]
					ExistentialDeposit,
					#[codec(index = 4)]
					#[doc = "Transfer/payment would kill account"]
					KeepAlive,
					#[codec(index = 5)]
					#[doc = "A vesting schedule already exists for this account"]
					ExistingVestingSchedule,
					#[codec(index = 6)]
					#[doc = "Beneficiary account must pre-exist"]
					DeadAccount,
					#[codec(index = 7)]
					#[doc = "Number of named reserves exceed MaxReserves"]
					TooManyReserves,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::sp_core::crypto::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::sp_core::crypto::AccountId32,
						to: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						who: ::subxt::sp_core::crypto::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved {
						who: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved {
						who: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::sp_core::crypto::AccountId32,
						to: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit {
						who: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw {
						who: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed {
						who: ::subxt::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
				pub reasons: runtime_types::pallet_balances::Reasons,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum Reasons {
				#[codec(index = 0)]
				Fee,
				#[codec(index = 1)]
				Misc,
				#[codec(index = 2)]
				All,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum Releases {
				#[codec(index = 0)]
				V1_0_0,
				#[codec(index = 1)]
				V2_0_0,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod pallet_collator_selection {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct CandidateInfo<_0, _1> {
					pub who: _0,
					pub deposit: _1,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many candidates"]
					TooManyCandidates,
					#[codec(index = 1)]
					#[doc = "Too few candidates"]
					TooFewCandidates,
					#[codec(index = 2)]
					#[doc = "Unknown error"]
					Unknown,
					#[codec(index = 3)]
					#[doc = "Permission issue"]
					Permission,
					#[codec(index = 4)]
					#[doc = "User is already a candidate"]
					AlreadyCandidate,
					#[codec(index = 5)]
					#[doc = "User is not a candidate"]
					NotCandidate,
					#[codec(index = 6)]
					#[doc = "User is already an Invulnerable"]
					AlreadyInvulnerable,
					#[codec(index = 7)]
					#[doc = "Account has no associated validator ID"]
					NoAssociatedValidatorId,
					#[codec(index = 8)]
					#[doc = "Validator ID is not yet registered"]
					ValidatorNotRegistered,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the collective's membership."]
					#[doc = ""]
					#[doc = "- `new_members`: The new member list. Be nice to the chain and provide it sorted."]
					#[doc = "- `prime`: The prime member whose vote sets the default."]
					#[doc = "- `old_count`: The upper bound for the previous number of members in storage. Used for"]
					#[doc = "  weight estimation."]
					#[doc = ""]
					#[doc = "Requires root origin."]
					#[doc = ""]
					#[doc = "NOTE: Does not enforce the expected `MaxMembers` limit on the amount of members, but"]
					#[doc = "      the weight estimations rely on it to estimate dispatchable weight."]
					#[doc = ""]
					#[doc = "# WARNING:"]
					#[doc = ""]
					#[doc = "The `pallet-collective` can also be managed by logic outside of the pallet through the"]
					#[doc = "implementation of the trait [`ChangeMembers`]."]
					#[doc = "Any call to `set_members` must be careful that the member set doesn't get out of sync"]
					#[doc = "with other logic managing the member set."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(MP + N)` where:"]
					#[doc = "  - `M` old-members-count (code- and governance-bounded)"]
					#[doc = "  - `N` new-members-count (code- and governance-bounded)"]
					#[doc = "  - `P` proposals-count (code-bounded)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage mutation (codec `O(M)` read, `O(N)` write) for reading and writing the"]
					#[doc = "    members"]
					#[doc = "  - 1 storage read (codec `O(P)`) for reading the proposals"]
					#[doc = "  - `P` storage mutations (codec `O(M)`) for updating the votes for each proposal"]
					#[doc = "  - 1 storage write (codec `O(1)`) for deleting the old `prime` and setting the new one"]
					#[doc = "# </weight>"]
					set_members {
						new_members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
						old_count: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "Dispatch a proposal from a member using the `Member` origin."]
					#[doc = ""]
					#[doc = "Origin must be a member of the collective."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(M + P)` where `M` members-count (code-bounded) and `P` complexity of dispatching"]
					#[doc = "  `proposal`"]
					#[doc = "- DB: 1 read (codec `O(M)`) + DB access of `proposal`"]
					#[doc = "- 1 event"]
					#[doc = "# </weight>"]
					execute {
						proposal: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Add a new proposal to either be voted on or executed directly."]
					#[doc = ""]
					#[doc = "Requires the sender to be member."]
					#[doc = ""]
					#[doc = "`threshold` determines whether `proposal` is executed directly (`threshold < 2`)"]
					#[doc = "or put up for voting."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(B + M + P1)` or `O(B + M + P2)` where:"]
					#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
					#[doc = "  - `M` is members-count (code- and governance-bounded)"]
					#[doc = "  - branching is influenced by `threshold` where:"]
					#[doc = "    - `P1` is proposal execution complexity (`threshold < 2`)"]
					#[doc = "    - `P2` is proposals-count (code-bounded) (`threshold >= 2`)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage read `is_member` (codec `O(M)`)"]
					#[doc = "  - 1 storage read `ProposalOf::contains_key` (codec `O(1)`)"]
					#[doc = "  - DB accesses influenced by `threshold`:"]
					#[doc = "    - EITHER storage accesses done by `proposal` (`threshold < 2`)"]
					#[doc = "    - OR proposal insertion (`threshold <= 2`)"]
					#[doc = "      - 1 storage mutation `Proposals` (codec `O(P2)`)"]
					#[doc = "      - 1 storage mutation `ProposalCount` (codec `O(1)`)"]
					#[doc = "      - 1 storage write `ProposalOf` (codec `O(B)`)"]
					#[doc = "      - 1 storage write `Voting` (codec `O(M)`)"]
					#[doc = "  - 1 event"]
					#[doc = "# </weight>"]
					propose {
						#[codec(compact)]
						threshold: ::core::primitive::u32,
						proposal: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
						#[codec(compact)]
						length_bound: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Add an aye or nay vote for the sender to the given proposal."]
					#[doc = ""]
					#[doc = "Requires the sender to be a member."]
					#[doc = ""]
					#[doc = "Transaction fees will be waived if the member is voting on any particular proposal"]
					#[doc = "for the first time and the call is successful. Subsequent vote changes will charge a"]
					#[doc = "fee."]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(M)` where `M` is members-count (code- and governance-bounded)"]
					#[doc = "- DB:"]
					#[doc = "  - 1 storage read `Members` (codec `O(M)`)"]
					#[doc = "  - 1 storage mutation `Voting` (codec `O(M)`)"]
					#[doc = "- 1 event"]
					#[doc = "# </weight>"]
					vote {
						proposal: ::subxt::sp_core::H256,
						#[codec(compact)]
						index: ::core::primitive::u32,
						approve: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "Close a vote that is either approved, disapproved or whose voting period has ended."]
					#[doc = ""]
					#[doc = "May be called by any signed account in order to finish voting and close the proposal."]
					#[doc = ""]
					#[doc = "If called before the end of the voting period it will only close the vote if it is"]
					#[doc = "has enough votes to be approved or disapproved."]
					#[doc = ""]
					#[doc = "If called after the end of the voting period abstentions are counted as rejections"]
					#[doc = "unless there is a prime member set and the prime member cast an approval."]
					#[doc = ""]
					#[doc = "If the close operation completes successfully with disapproval, the transaction fee will"]
					#[doc = "be waived. Otherwise execution of the approved operation will be charged to the caller."]
					#[doc = ""]
					#[doc = "+ `proposal_weight_bound`: The maximum amount of weight consumed by executing the closed"]
					#[doc = "proposal."]
					#[doc = "+ `length_bound`: The upper bound for the length of the proposal in storage. Checked via"]
					#[doc = "`storage::read` so it is `size_of::<u32>() == 4` larger than the pure length."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "## Weight"]
					#[doc = "- `O(B + M + P1 + P2)` where:"]
					#[doc = "  - `B` is `proposal` size in bytes (length-fee-bounded)"]
					#[doc = "  - `M` is members-count (code- and governance-bounded)"]
					#[doc = "  - `P1` is the complexity of `proposal` preimage."]
					#[doc = "  - `P2` is proposal-count (code-bounded)"]
					#[doc = "- DB:"]
					#[doc = " - 2 storage reads (`Members`: codec `O(M)`, `Prime`: codec `O(1)`)"]
					#[doc = " - 3 mutations (`Voting`: codec `O(M)`, `ProposalOf`: codec `O(B)`, `Proposals`: codec"]
					#[doc = "   `O(P2)`)"]
					#[doc = " - any mutations done while executing `proposal` (`P1`)"]
					#[doc = "- up to 3 events"]
					#[doc = "# </weight>"]
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
					#[doc = "Disapprove a proposal, close, and remove it from the system, regardless of its current"]
					#[doc = "state."]
					#[doc = ""]
					#[doc = "Must be called by the Root origin."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "* `proposal_hash`: The hash of the proposal that should be disapproved."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Complexity: O(P) where P is the number of max proposals"]
					#[doc = "DB Weight:"]
					#[doc = "* Reads: Proposals"]
					#[doc = "* Writes: Voting, Proposals, ProposalOf"]
					#[doc = "# </weight>"]
					disapprove_proposal { proposal_hash: ::subxt::sp_core::H256 },
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Account is not a member"]
					NotMember,
					#[codec(index = 1)]
					#[doc = "Duplicate proposals not allowed"]
					DuplicateProposal,
					#[codec(index = 2)]
					#[doc = "Proposal must exist"]
					ProposalMissing,
					#[codec(index = 3)]
					#[doc = "Mismatched index"]
					WrongIndex,
					#[codec(index = 4)]
					#[doc = "Duplicate vote ignored"]
					DuplicateVote,
					#[codec(index = 5)]
					#[doc = "Members are already initialized!"]
					AlreadyInitialized,
					#[codec(index = 6)]
					#[doc = "The close call was made too early, before the end of the voting."]
					TooEarly,
					#[codec(index = 7)]
					#[doc = "There can only be a maximum of `MaxProposals` active proposals."]
					TooManyProposals,
					#[codec(index = 8)]
					#[doc = "The given weight bound for the proposal was too low."]
					WrongProposalWeight,
					#[codec(index = 9)]
					#[doc = "The given length bound for the proposal was too low."]
					WrongProposalLength,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
					#[doc = "`MemberCount`)."]
					Proposed {
						account: ::subxt::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: ::subxt::sp_core::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A motion (given hash) has been voted on by given account, leaving"]
					#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
					Voted {
						account: ::subxt::sp_core::crypto::AccountId32,
						proposal_hash: ::subxt::sp_core::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "A motion was approved by the required threshold."]
					Approved { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 3)]
					#[doc = "A motion was not approved by the required threshold."]
					Disapproved { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 4)]
					#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
					Executed {
						proposal_hash: ::subxt::sp_core::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
					MemberExecuted {
						proposal_hash: ::subxt::sp_core::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
					Closed {
						proposal_hash: ::subxt::sp_core::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					make_claimable,
					#[codec(index = 1)]
					#[doc = "Attempts to claim some crowdloan bonus from the crowdloan pot."]
					#[doc = "No-op if amount is zero."]
					claim { amount: ::core::primitive::u128 },
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Pallet has already been initiated."]
					AlreadyInitiated,
					#[codec(index = 1)]
					#[doc = "Claiming has not yet been enabled."]
					NotClaimable,
					#[codec(index = 2)]
					#[doc = "Crowdloan Bonus pot is empty."]
					EmptyPot,
					#[codec(index = 3)]
					#[doc = "User has insufficent tokens to claim crowdloan bonus."]
					InsufficientTokens,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					Initiated(runtime_types::primitives::currency::CurrencyId),
					#[codec(index = 1)]
					Claimed(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
				}
			}
		}
		pub mod pallet_democracy {
			use super::runtime_types;
			pub mod conviction {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Propose a sensitive action to be taken."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the sender must"]
					#[doc = "have funds to cover the deposit."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The hash of the proposal preimage."]
					#[doc = "- `value`: The amount of deposit (must be at least `MinimumDeposit`)."]
					#[doc = ""]
					#[doc = "Emits `Proposed`."]
					#[doc = ""]
					#[doc = "Weight: `O(p)`"]
					propose {
						proposal_hash: ::subxt::sp_core::H256,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Signals agreement with a particular proposal."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the sender"]
					#[doc = "must have funds to cover the deposit, equal to the original deposit."]
					#[doc = ""]
					#[doc = "- `proposal`: The index of the proposal to second."]
					#[doc = "- `seconds_upper_bound`: an upper bound on the current number of seconds on this"]
					#[doc = "  proposal. Extrinsic is weighted according to this value with no refund."]
					#[doc = ""]
					#[doc = "Weight: `O(S)` where S is the number of seconds a proposal already has."]
					second {
						#[codec(compact)]
						proposal: ::core::primitive::u32,
						#[codec(compact)]
						seconds_upper_bound: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
					#[doc = "otherwise it is a vote to keep the status quo."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `ref_index`: The index of the referendum to vote for."]
					#[doc = "- `vote`: The vote configuration."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter has voted on."]
					vote {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
						vote: runtime_types::pallet_democracy::vote::AccountVote<
							::core::primitive::u128,
						>,
					},
					#[codec(index = 3)]
					#[doc = "Schedule an emergency cancellation of a referendum. Cannot happen twice to the same"]
					#[doc = "referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `CancellationOrigin`."]
					#[doc = ""]
					#[doc = "-`ref_index`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`."]
					emergency_cancel { ref_index: ::core::primitive::u32 },
					#[codec(index = 4)]
					#[doc = "Schedule a referendum to be tabled once it is legal to schedule an external"]
					#[doc = "referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `ExternalOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Weight: `O(V)` with V number of vetoers in the blacklist of proposal."]
					#[doc = "  Decoding vec of length V. Charged as maximum"]
					external_propose { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 5)]
					#[doc = "Schedule a majority-carries referendum to be tabled next once it is legal to schedule"]
					#[doc = "an external referendum."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `ExternalMajorityOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
					#[doc = "pre-scheduled `external_propose` call."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					external_propose_majority { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 6)]
					#[doc = "Schedule a negative-turnout-bias referendum to be tabled next once it is legal to"]
					#[doc = "schedule an external referendum."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `ExternalDefaultOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal."]
					#[doc = ""]
					#[doc = "Unlike `external_propose`, blacklisting has no effect on this and it may replace a"]
					#[doc = "pre-scheduled `external_propose` call."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					external_propose_default { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 7)]
					#[doc = "Schedule the currently externally-proposed majority-carries referendum to be tabled"]
					#[doc = "immediately. If there is no externally-proposed referendum currently, or if there is one"]
					#[doc = "but it is not a majority-carries referendum then it fails."]
					#[doc = ""]
					#[doc = "The dispatch of this call must be `FastTrackOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The hash of the current external proposal."]
					#[doc = "- `voting_period`: The period that is allowed for voting on this proposal. Increased to"]
					#[doc = "  `FastTrackVotingPeriod` if too low."]
					#[doc = "- `delay`: The number of block after voting has ended in approval and this should be"]
					#[doc = "  enacted. This doesn't have a minimum amount."]
					#[doc = ""]
					#[doc = "Emits `Started`."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`"]
					fast_track {
						proposal_hash: ::subxt::sp_core::H256,
						voting_period: ::core::primitive::u32,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 8)]
					#[doc = "Veto and blacklist the external proposal hash."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `VetoOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of the proposal to veto and blacklist."]
					#[doc = ""]
					#[doc = "Emits `Vetoed`."]
					#[doc = ""]
					#[doc = "Weight: `O(V + log(V))` where V is number of `existing vetoers`"]
					veto_external { proposal_hash: ::subxt::sp_core::H256 },
					#[codec(index = 9)]
					#[doc = "Remove a referendum."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `ref_index`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "# Weight: `O(1)`."]
					cancel_referendum {
						#[codec(compact)]
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 10)]
					#[doc = "Cancel a proposal queued for enactment."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `which`: The index of the referendum to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(D)` where `D` is the items in the dispatch queue. Weighted as `D = 10`."]
					cancel_queued { which: ::core::primitive::u32 },
					#[codec(index = 11)]
					#[doc = "Delegate the voting power (with some given conviction) of the sending account."]
					#[doc = ""]
					#[doc = "The balance delegated is locked for as long as it's delegated, and thereafter for the"]
					#[doc = "time appropriate for the conviction's lock period."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_, and the signing account must either:"]
					#[doc = "  - be delegating already; or"]
					#[doc = "  - have no voting activity (if there is, then it will need to be removed/consolidated"]
					#[doc = "    through `reap_vote` or `unvote`)."]
					#[doc = ""]
					#[doc = "- `to`: The account whose voting the `target` account's voting power will follow."]
					#[doc = "- `conviction`: The conviction that will be attached to the delegated votes. When the"]
					#[doc = "  account is undelegated, the funds will be locked for the corresponding period."]
					#[doc = "- `balance`: The amount of the account's balance to be used in delegating. This must not"]
					#[doc = "  be more than the account's current balance."]
					#[doc = ""]
					#[doc = "Emits `Delegated`."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
					#[doc = "  voted on. Weight is charged as if maximum votes."]
					delegate {
						to: ::subxt::sp_core::crypto::AccountId32,
						conviction: runtime_types::pallet_democracy::conviction::Conviction,
						balance: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "Undelegate the voting power of the sending account."]
					#[doc = ""]
					#[doc = "Tokens may be unlocked following once an amount of time consistent with the lock period"]
					#[doc = "of the conviction with which the delegation was issued."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_ and the signing account must be"]
					#[doc = "currently delegating."]
					#[doc = ""]
					#[doc = "Emits `Undelegated`."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` where R is the number of referendums the voter delegating to has"]
					#[doc = "  voted on. Weight is charged as if maximum votes."]
					undelegate,
					#[codec(index = 13)]
					#[doc = "Clears all public proposals."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Root_."]
					#[doc = ""]
					#[doc = "Weight: `O(1)`."]
					clear_public_proposals,
					#[codec(index = 14)]
					#[doc = "Register the preimage for an upcoming proposal. This doesn't require the proposal to be"]
					#[doc = "in the dispatch queue but does require a deposit, returned once enacted."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `encoded_proposal`: The preimage of a proposal."]
					#[doc = ""]
					#[doc = "Emits `PreimageNoted`."]
					#[doc = ""]
					#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
					note_preimage { encoded_proposal: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 15)]
					#[doc = "Same as `note_preimage` but origin is `OperationalPreimageOrigin`."]
					note_preimage_operational {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 16)]
					#[doc = "Register the preimage for an upcoming proposal. This requires the proposal to be"]
					#[doc = "in the dispatch queue. No deposit is needed. When this call is successful, i.e."]
					#[doc = "the preimage has not been uploaded before and matches some imminent proposal,"]
					#[doc = "no fee is paid."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `encoded_proposal`: The preimage of a proposal."]
					#[doc = ""]
					#[doc = "Emits `PreimageNoted`."]
					#[doc = ""]
					#[doc = "Weight: `O(E)` with E size of `encoded_proposal` (protected by a required deposit)."]
					note_imminent_preimage {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 17)]
					#[doc = "Same as `note_imminent_preimage` but origin is `OperationalPreimageOrigin`."]
					note_imminent_preimage_operational {
						encoded_proposal: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 18)]
					#[doc = "Remove an expired proposal preimage and collect the deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The preimage hash of a proposal."]
					#[doc = "- `proposal_length_upper_bound`: an upper bound on length of the proposal. Extrinsic is"]
					#[doc = "  weighted according to this value with no refund."]
					#[doc = ""]
					#[doc = "This will only work after `VotingPeriod` blocks from the time that the preimage was"]
					#[doc = "noted, if it's the same account doing it. If it's a different account, then it'll only"]
					#[doc = "work an additional `EnactmentPeriod` later."]
					#[doc = ""]
					#[doc = "Emits `PreimageReaped`."]
					#[doc = ""]
					#[doc = "Weight: `O(D)` where D is length of proposal."]
					reap_preimage {
						proposal_hash: ::subxt::sp_core::H256,
						#[codec(compact)]
						proposal_len_upper_bound: ::core::primitive::u32,
					},
					#[codec(index = 19)]
					#[doc = "Unlock tokens that have an expired lock."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `target`: The account to remove the lock on."]
					#[doc = ""]
					#[doc = "Weight: `O(R)` with R number of vote of target."]
					unlock { target: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 20)]
					#[doc = "Remove a vote for a referendum."]
					#[doc = ""]
					#[doc = "If:"]
					#[doc = "- the referendum was cancelled, or"]
					#[doc = "- the referendum is ongoing, or"]
					#[doc = "- the referendum has ended such that"]
					#[doc = "  - the vote of the account was in opposition to the result; or"]
					#[doc = "  - there was no conviction to the account's vote; or"]
					#[doc = "  - the account made a split vote"]
					#[doc = "...then the vote is removed cleanly and a following call to `unlock` may result in more"]
					#[doc = "funds being available."]
					#[doc = ""]
					#[doc = "If, however, the referendum has ended and:"]
					#[doc = "- it finished corresponding to the vote of the account, and"]
					#[doc = "- the account made a standard vote with conviction, and"]
					#[doc = "- the lock period of the conviction is not over"]
					#[doc = "...then the lock will be aggregated into the overall account's lock, which may involve"]
					#[doc = "*overlocking* (where the two locks are combined into a single lock that is the maximum"]
					#[doc = "of both the amount locked and the time is it locked for)."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_, and the signer must have a vote"]
					#[doc = "registered for referendum `index`."]
					#[doc = ""]
					#[doc = "- `index`: The index of referendum of the vote to be removed."]
					#[doc = ""]
					#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
					#[doc = "  Weight is calculated for the maximum number of vote."]
					remove_vote { index: ::core::primitive::u32 },
					#[codec(index = 21)]
					#[doc = "Remove a vote for a referendum."]
					#[doc = ""]
					#[doc = "If the `target` is equal to the signer, then this function is exactly equivalent to"]
					#[doc = "`remove_vote`. If not equal to the signer, then the vote must have expired,"]
					#[doc = "either because the referendum was cancelled, because the voter lost the referendum or"]
					#[doc = "because the conviction period is over."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `target`: The account of the vote to be removed; this account must have voted for"]
					#[doc = "  referendum `index`."]
					#[doc = "- `index`: The index of referendum of the vote to be removed."]
					#[doc = ""]
					#[doc = "Weight: `O(R + log R)` where R is the number of referenda that `target` has voted on."]
					#[doc = "  Weight is calculated for the maximum number of vote."]
					remove_other_vote {
						target: ::subxt::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 22)]
					#[doc = "Enact a proposal from a referendum. For now we just make the weight be the maximum."]
					enact_proposal {
						proposal_hash: ::subxt::sp_core::H256,
						index: ::core::primitive::u32,
					},
					#[codec(index = 23)]
					#[doc = "Permanently place a proposal into the blacklist. This prevents it from ever being"]
					#[doc = "proposed again."]
					#[doc = ""]
					#[doc = "If called on a queued public or external proposal, then this will result in it being"]
					#[doc = "removed. If the `ref_index` supplied is an active referendum with the proposal hash,"]
					#[doc = "then it will be cancelled."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `BlacklistOrigin`."]
					#[doc = ""]
					#[doc = "- `proposal_hash`: The proposal hash to blacklist permanently."]
					#[doc = "- `ref_index`: An ongoing referendum whose hash is `proposal_hash`, which will be"]
					#[doc = "cancelled."]
					#[doc = ""]
					#[doc = "Weight: `O(p)` (though as this is an high-privilege dispatch, we assume it has a"]
					#[doc = "  reasonable value)."]
					blacklist {
						proposal_hash: ::subxt::sp_core::H256,
						maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 24)]
					#[doc = "Remove a proposal."]
					#[doc = ""]
					#[doc = "The dispatch origin of this call must be `CancelProposalOrigin`."]
					#[doc = ""]
					#[doc = "- `prop_index`: The index of the proposal to cancel."]
					#[doc = ""]
					#[doc = "Weight: `O(p)` where `p = PublicProps::<T>::decode_len()`"]
					cancel_proposal {
						#[codec(compact)]
						prop_index: ::core::primitive::u32,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Value too low"]
					ValueLow,
					#[codec(index = 1)]
					#[doc = "Proposal does not exist"]
					ProposalMissing,
					#[codec(index = 2)]
					#[doc = "Cannot cancel the same proposal twice"]
					AlreadyCanceled,
					#[codec(index = 3)]
					#[doc = "Proposal already made"]
					DuplicateProposal,
					#[codec(index = 4)]
					#[doc = "Proposal still blacklisted"]
					ProposalBlacklisted,
					#[codec(index = 5)]
					#[doc = "Next external proposal not simple majority"]
					NotSimpleMajority,
					#[codec(index = 6)]
					#[doc = "Invalid hash"]
					InvalidHash,
					#[codec(index = 7)]
					#[doc = "No external proposal"]
					NoProposal,
					#[codec(index = 8)]
					#[doc = "Identity may not veto a proposal twice"]
					AlreadyVetoed,
					#[codec(index = 9)]
					#[doc = "Preimage already noted"]
					DuplicatePreimage,
					#[codec(index = 10)]
					#[doc = "Not imminent"]
					NotImminent,
					#[codec(index = 11)]
					#[doc = "Too early"]
					TooEarly,
					#[codec(index = 12)]
					#[doc = "Imminent"]
					Imminent,
					#[codec(index = 13)]
					#[doc = "Preimage not found"]
					PreimageMissing,
					#[codec(index = 14)]
					#[doc = "Vote given for invalid referendum"]
					ReferendumInvalid,
					#[codec(index = 15)]
					#[doc = "Invalid preimage"]
					PreimageInvalid,
					#[codec(index = 16)]
					#[doc = "No proposals waiting"]
					NoneWaiting,
					#[codec(index = 17)]
					#[doc = "The given account did not vote on the referendum."]
					NotVoter,
					#[codec(index = 18)]
					#[doc = "The actor has no permission to conduct the action."]
					NoPermission,
					#[codec(index = 19)]
					#[doc = "The account is already delegating."]
					AlreadyDelegating,
					#[codec(index = 20)]
					#[doc = "Too high a balance was provided that the account cannot afford."]
					InsufficientFunds,
					#[codec(index = 21)]
					#[doc = "The account is not currently delegating."]
					NotDelegating,
					#[codec(index = 22)]
					#[doc = "The account currently has votes attached to it and the operation cannot succeed until"]
					#[doc = "these are removed, either through `unvote` or `reap_vote`."]
					VotesExist,
					#[codec(index = 23)]
					#[doc = "The instant referendum origin is currently disallowed."]
					InstantNotAllowed,
					#[codec(index = 24)]
					#[doc = "Delegation to oneself makes no sense."]
					Nonsense,
					#[codec(index = 25)]
					#[doc = "Invalid upper bound."]
					WrongUpperBound,
					#[codec(index = 26)]
					#[doc = "Maximum number of votes reached."]
					MaxVotesReached,
					#[codec(index = 27)]
					#[doc = "Maximum number of proposals reached."]
					TooManyProposals,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion has been proposed by a public account."]
					Proposed {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "A public proposal has been tabled for referendum vote."]
					Tabled {
						proposal_index: ::core::primitive::u32,
						deposit: ::core::primitive::u128,
						depositors: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 2)]
					#[doc = "An external proposal has been tabled."]
					ExternalTabled,
					#[codec(index = 3)]
					#[doc = "A referendum has begun."]
					Started {
						ref_index: ::core::primitive::u32,
						threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					},
					#[codec(index = 4)]
					#[doc = "A proposal has been approved by referendum."]
					Passed { ref_index: ::core::primitive::u32 },
					#[codec(index = 5)]
					#[doc = "A proposal has been rejected by referendum."]
					NotPassed { ref_index: ::core::primitive::u32 },
					#[codec(index = 6)]
					#[doc = "A referendum has been cancelled."]
					Cancelled { ref_index: ::core::primitive::u32 },
					#[codec(index = 7)]
					#[doc = "A proposal has been enacted."]
					Executed {
						ref_index: ::core::primitive::u32,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 8)]
					#[doc = "An account has delegated their vote to another account."]
					Delegated {
						who: ::subxt::sp_core::crypto::AccountId32,
						target: ::subxt::sp_core::crypto::AccountId32,
					},
					#[codec(index = 9)]
					#[doc = "An account has cancelled a previous delegation operation."]
					Undelegated { account: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 10)]
					#[doc = "An external proposal has been vetoed."]
					Vetoed {
						who: ::subxt::sp_core::crypto::AccountId32,
						proposal_hash: ::subxt::sp_core::H256,
						until: ::core::primitive::u32,
					},
					#[codec(index = 11)]
					#[doc = "A proposal's preimage was noted, and the deposit taken."]
					PreimageNoted {
						proposal_hash: ::subxt::sp_core::H256,
						who: ::subxt::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 12)]
					#[doc = "A proposal preimage was removed and used (the deposit was returned)."]
					PreimageUsed {
						proposal_hash: ::subxt::sp_core::H256,
						provider: ::subxt::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
					},
					#[codec(index = 13)]
					#[doc = "A proposal could not be executed because its preimage was invalid."]
					PreimageInvalid {
						proposal_hash: ::subxt::sp_core::H256,
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 14)]
					#[doc = "A proposal could not be executed because its preimage was missing."]
					PreimageMissing {
						proposal_hash: ::subxt::sp_core::H256,
						ref_index: ::core::primitive::u32,
					},
					#[codec(index = 15)]
					#[doc = "A registered preimage was removed and the deposit collected by the reaper."]
					PreimageReaped {
						proposal_hash: ::subxt::sp_core::H256,
						provider: ::subxt::sp_core::crypto::AccountId32,
						deposit: ::core::primitive::u128,
						reaper: ::subxt::sp_core::crypto::AccountId32,
					},
					#[codec(index = 16)]
					#[doc = "A proposal_hash has been blacklisted permanently."]
					Blacklisted { proposal_hash: ::subxt::sp_core::H256 },
				}
			}
			pub mod types {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct Delegations<_0> {
					pub votes: _0,
					pub capital: _0,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum ReferendumInfo<_0, _1, _2> {
					#[codec(index = 0)]
					Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
					#[codec(index = 1)]
					Finished { approved: ::core::primitive::bool, end: _0 },
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct ReferendumStatus<_0, _1, _2> {
					pub end: _0,
					pub proposal_hash: _1,
					pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
					pub delay: _0,
					pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct Tally<_0> {
					pub ayes: _0,
					pub nays: _0,
					pub turnout: _0,
				}
			}
			pub mod vote {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum AccountVote<_0> {
					#[codec(index = 0)]
					Standard { vote: runtime_types::pallet_democracy::vote::Vote, balance: _0 },
					#[codec(index = 1)]
					Split { aye: _0, nay: _0 },
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct PriorLock<_0, _1>(pub _0, pub _1);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Decode,
					:: subxt :: codec :: Encode,
					Debug,
				)]
				pub struct Vote(pub ::core::primitive::u8);
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum VoteThreshold {
					#[codec(index = 0)]
					SuperMajorityApprove,
					#[codec(index = 1)]
					SuperMajorityAgainst,
					#[codec(index = 2)]
					SimpleMajority,
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum Releases {
				#[codec(index = 0)]
				V1,
			}
		}
		pub mod pallet_indices {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Assign an previously unassigned index."]
					#[doc = ""]
					#[doc = "Payment: `Deposit` is reserved from the sender account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be claimed. This must not be in use."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					claim { index: ::core::primitive::u32 },
					#[codec(index = 1)]
					#[doc = "Assign an index already owned by the sender to another account. The balance reservation"]
					#[doc = "is effectively transferred to the new account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be re-assigned. This must be owned by the sender."]
					#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One transfer operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "   - Reads: Indices Accounts, System Account (recipient)"]
					#[doc = "   - Writes: Indices Accounts, System Account (recipient)"]
					#[doc = "# </weight>"]
					transfer {
						new: ::subxt::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Free up an index owned by the sender."]
					#[doc = ""]
					#[doc = "Payment: Any previous deposit placed for the index is unreserved in the sender account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the sender must own the index."]
					#[doc = ""]
					#[doc = "- `index`: the index to be freed. This must be owned by the sender."]
					#[doc = ""]
					#[doc = "Emits `IndexFreed` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- One reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					free { index: ::core::primitive::u32 },
					#[codec(index = 3)]
					#[doc = "Force an index to an account. This doesn't require a deposit. If the index is already"]
					#[doc = "held, then any deposit is reimbursed to its current owner."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "- `index`: the index to be (re-)assigned."]
					#[doc = "- `new`: the new owner of the index. This function is a no-op if it is equal to sender."]
					#[doc = "- `freeze`: if set to `true`, will freeze the index so it cannot be transferred."]
					#[doc = ""]
					#[doc = "Emits `IndexAssigned` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- Up to one reserve operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "   - Reads: Indices Accounts, System Account (original owner)"]
					#[doc = "   - Writes: Indices Accounts, System Account (original owner)"]
					#[doc = "# </weight>"]
					force_transfer {
						new: ::subxt::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
						freeze: ::core::primitive::bool,
					},
					#[codec(index = 4)]
					#[doc = "Freeze an index so it will always point to the sender account. This consumes the"]
					#[doc = "deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and the signing account must have a"]
					#[doc = "non-frozen account `index`."]
					#[doc = ""]
					#[doc = "- `index`: the index to be frozen in place."]
					#[doc = ""]
					#[doc = "Emits `IndexFrozen` if successful."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)`."]
					#[doc = "- One storage mutation (codec `O(1)`)."]
					#[doc = "- Up to one slash operation."]
					#[doc = "- One event."]
					#[doc = "-------------------"]
					#[doc = "- DB Weight: 1 Read/Write (Accounts)"]
					#[doc = "# </weight>"]
					freeze { index: ::core::primitive::u32 },
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The index was not already assigned."]
					NotAssigned,
					#[codec(index = 1)]
					#[doc = "The index is assigned to another account."]
					NotOwner,
					#[codec(index = 2)]
					#[doc = "The index was not available."]
					InUse,
					#[codec(index = 3)]
					#[doc = "The source and destination accounts are identical."]
					NotTransfer,
					#[codec(index = 4)]
					#[doc = "The index is permanent and may not be freed/changed."]
					Permanent,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A account index was assigned."]
					IndexAssigned {
						who: ::subxt::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A account index has been freed up (unassigned)."]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "A account index has been frozen to its current account ID."]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Add a member `who` to the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::AddOrigin`."]
					add_member { who: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 1)]
					#[doc = "Remove a member `who` from the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::RemoveOrigin`."]
					remove_member { who: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 2)]
					#[doc = "Swap out one member `remove` for another `add`."]
					#[doc = ""]
					#[doc = "May only be called from `T::SwapOrigin`."]
					#[doc = ""]
					#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
					swap_member {
						remove: ::subxt::sp_core::crypto::AccountId32,
						add: ::subxt::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
					#[doc = "pass `members` pre-sorted."]
					#[doc = ""]
					#[doc = "May only be called from `T::ResetOrigin`."]
					reset_members {
						members: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 4)]
					#[doc = "Swap out the sending member for some other key `new`."]
					#[doc = ""]
					#[doc = "May only be called from `Signed` origin of a current member."]
					#[doc = ""]
					#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
					change_key { new: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 5)]
					#[doc = "Set the prime member. Must be a current member."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					set_prime { who: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 6)]
					#[doc = "Remove the prime member if it exists."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					clear_prime,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Already a member."]
					AlreadyMember,
					#[codec(index = 1)]
					#[doc = "Not a member."]
					NotMember,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "The given member was added; see the transaction for who."]
					MemberAdded,
					#[codec(index = 1)]
					#[doc = "The given member was removed; see the transaction for who."]
					MemberRemoved,
					#[codec(index = 2)]
					#[doc = "Two members were swapped; see the transaction for who."]
					MembersSwapped,
					#[codec(index = 3)]
					#[doc = "The membership was reset; see the transaction for who the new set is."]
					MembersReset,
					#[codec(index = 4)]
					#[doc = "One of the members' keys changed."]
					KeyChanged,
					#[codec(index = 5)]
					#[doc = "Phantom member, never used."]
					Dummy,
				}
			}
		}
		pub mod pallet_scheduler {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Anonymously schedule a task."]
					schedule {
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
					},
					#[codec(index = 1)]
					#[doc = "Cancel an anonymously scheduled task."]
					cancel { when: ::core::primitive::u32, index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "Schedule a named task."]
					schedule_named {
						id: ::std::vec::Vec<::core::primitive::u8>,
						when: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
					},
					#[codec(index = 3)]
					#[doc = "Cancel a named scheduled task."]
					cancel_named { id: ::std::vec::Vec<::core::primitive::u8> },
					#[codec(index = 4)]
					#[doc = "Anonymously schedule a task after a delay."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Same as [`schedule`]."]
					#[doc = "# </weight>"]
					schedule_after {
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
					},
					#[codec(index = 5)]
					#[doc = "Schedule a named task after a delay."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Same as [`schedule_named`](Self::schedule_named)."]
					#[doc = "# </weight>"]
					schedule_named_after {
						id: ::std::vec::Vec<::core::primitive::u8>,
						after: ::core::primitive::u32,
						maybe_periodic: ::core::option::Option<(
							::core::primitive::u32,
							::core::primitive::u32,
						)>,
						priority: ::core::primitive::u8,
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Failed to schedule a call"]
					FailedToSchedule,
					#[codec(index = 1)]
					#[doc = "Cannot find the scheduled call."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Given target block number is in the past."]
					TargetBlockNumberInPast,
					#[codec(index = 3)]
					#[doc = "Reschedule failed because it does not change scheduled time."]
					RescheduleNoChange,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Scheduled some task. \\[when, index\\]"]
					Scheduled(::core::primitive::u32, ::core::primitive::u32),
					#[codec(index = 1)]
					#[doc = "Canceled some task. \\[when, index\\]"]
					Canceled(::core::primitive::u32, ::core::primitive::u32),
					#[codec(index = 2)]
					#[doc = "Dispatched some task. \\[task, id, result\\]"]
					Dispatched(
						(::core::primitive::u32, ::core::primitive::u32),
						::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum Releases {
				#[codec(index = 0)]
				V1,
				#[codec(index = 1)]
				V2,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Sets the session key(s) of the function caller to `keys`."]
					#[doc = "Allows an account to set its session key prior to becoming a validator."]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be signed."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)`. Actual cost depends on the number of length of"]
					#[doc = "  `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `origin account`, `T::ValidatorIdOf`, `NextKeys`"]
					#[doc = "- DbWrites: `origin account`, `NextKeys`"]
					#[doc = "- DbReads per key id: `KeyOwner`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
					set_keys {
						keys: runtime_types::composable_runtime::opaque::SessionKeys,
						proof: ::std::vec::Vec<::core::primitive::u8>,
					},
					#[codec(index = 1)]
					#[doc = "Removes any session key(s) of the function caller."]
					#[doc = ""]
					#[doc = "This doesn't take effect until the next session."]
					#[doc = ""]
					#[doc = "The dispatch origin of this function must be Signed and the account must be either be"]
					#[doc = "convertible to a validator ID using the chain's typical addressing system (this usually"]
					#[doc = "means being a controller account) or directly convertible into a validator ID (which"]
					#[doc = "usually means being a stash account)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(1)` in number of key types. Actual cost depends on the number of length"]
					#[doc = "  of `T::Keys::key_ids()` which is fixed."]
					#[doc = "- DbReads: `T::ValidatorIdOf`, `NextKeys`, `origin account`"]
					#[doc = "- DbWrites: `NextKeys`, `origin account`"]
					#[doc = "- DbWrites per key id: `KeyOwner`"]
					#[doc = "# </weight>"]
					purge_keys,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Invalid ownership proof."]
					InvalidProof,
					#[codec(index = 1)]
					#[doc = "No associated validator ID for account."]
					NoAssociatedValidatorId,
					#[codec(index = 2)]
					#[doc = "Registered duplicate key."]
					DuplicatedKey,
					#[codec(index = 3)]
					#[doc = "No keys are associated with this account."]
					NoKeys,
					#[codec(index = 4)]
					#[doc = "Key setting account is not live, so it's impossible to associate keys."]
					NoAccount,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New session has happened. Note that the argument is the session index, not the"]
					#[doc = "block number as the type might suggest."]
					NewSession { session_index: ::core::primitive::u32 },
				}
			}
		}
		pub mod pallet_sudo {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo { call: ::std::boxed::Box<runtime_types::composable_runtime::Call> },
					#[codec(index = 1)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
					#[doc = "This function does not check the weight of the call, and instead allows the"]
					#[doc = "Sudo user to specify the weight of the call."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- The weight of this call is defined by the caller."]
					#[doc = "# </weight>"]
					sudo_unchecked_weight {
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
						weight: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					#[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
					#[doc = "key."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB change."]
					#[doc = "# </weight>"]
					set_key {
						new: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 3)]
					#[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
					#[doc = "a given account."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + 10,000."]
					#[doc = "# </weight>"]
					sudo_as {
						who: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account"]
					RequireSudo,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo just took place. \\[result\\]"]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied."]
					KeyChanged { new_sudoer: ::subxt::sp_core::crypto::AccountId32 },
					#[codec(index = 2)]
					#[doc = "A sudo just took place. \\[result\\]"]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Set the current time."]
					#[doc = ""]
					#[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
					#[doc = "phase, if this call hasn't been invoked by that time."]
					#[doc = ""]
					#[doc = "The timestamp should be greater than the previous one by the amount specified by"]
					#[doc = "`MinimumPeriod`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be `Inherent`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
					#[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
					#[doc = "  `on_finalize`)"]
					#[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
					#[doc = "# </weight>"]
					set {
						#[codec(compact)]
						now: ::core::primitive::u64,
					},
				}
			}
		}
		pub mod pallet_transaction_payment {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Put forward a suggestion for spending. A deposit proportional to the value"]
					#[doc = "is reserved and slashed if the proposal is rejected. It is returned once the"]
					#[doc = "proposal is awarded."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)"]
					#[doc = "- DbReads: `ProposalCount`, `origin account`"]
					#[doc = "- DbWrites: `ProposalCount`, `Proposals`, `origin account`"]
					#[doc = "# </weight>"]
					propose_spend {
						#[codec(compact)]
						value: ::core::primitive::u128,
						beneficiary: ::subxt::sp_runtime::MultiAddress<
							::subxt::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					},
					#[codec(index = 1)]
					#[doc = "Reject a proposed spend. The original deposit will be slashed."]
					#[doc = ""]
					#[doc = "May only be called from `T::RejectOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)"]
					#[doc = "- DbReads: `Proposals`, `rejected proposer account`"]
					#[doc = "- DbWrites: `Proposals`, `rejected proposer account`"]
					#[doc = "# </weight>"]
					reject_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
					#[doc = "and the original deposit will be returned."]
					#[doc = ""]
					#[doc = "May only be called from `T::ApproveOrigin`."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(1)."]
					#[doc = "- DbReads: `Proposals`, `Approvals`"]
					#[doc = "- DbWrite: `Approvals`"]
					#[doc = "# </weight>"]
					approve_proposal {
						#[codec(compact)]
						proposal_id: ::core::primitive::u32,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Proposer's balance is too low."]
					InsufficientProposersBalance,
					#[codec(index = 1)]
					#[doc = "No proposal or bounty at that index."]
					InvalidIndex,
					#[codec(index = 2)]
					#[doc = "Too many approvals in the queue."]
					TooManyApprovals,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New proposal. \\[proposal_index\\]"]
					Proposed(::core::primitive::u32),
					#[codec(index = 1)]
					#[doc = "We have ended a spend period and will now allocate funds. \\[budget_remaining\\]"]
					Spending(::core::primitive::u128),
					#[codec(index = 2)]
					#[doc = "Some funds have been allocated. \\[proposal_index, award, beneficiary\\]"]
					Awarded(
						::core::primitive::u32,
						::core::primitive::u128,
						::subxt::sp_core::crypto::AccountId32,
					),
					#[codec(index = 3)]
					#[doc = "A proposal was rejected; funds were slashed. \\[proposal_index, slashed\\]"]
					Rejected(::core::primitive::u32, ::core::primitive::u128),
					#[codec(index = 4)]
					#[doc = "Some of our funds have been burnt. \\[burn\\]"]
					Burnt(::core::primitive::u128),
					#[codec(index = 5)]
					#[doc = "Spending has finished; this is the amount that rolls over until next spend."]
					#[doc = "\\[budget_remaining\\]"]
					Rollover(::core::primitive::u128),
					#[codec(index = 6)]
					#[doc = "Some funds have been deposited. \\[deposit\\]"]
					Deposit(::core::primitive::u128),
				}
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Send a batch of dispatch calls."]
					#[doc = ""]
					#[doc = "May be called from any origin."]
					#[doc = ""]
					#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
					#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
					#[doc = ""]
					#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
					#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
					#[doc = "# </weight>"]
					#[doc = ""]
					#[doc = "This will return `Ok` in all circumstances. To determine the success of the batch, an"]
					#[doc = "event is deposited. If a call failed and the batch was interrupted, then the"]
					#[doc = "`BatchInterrupted` event is deposited, along with the number of successful calls made"]
					#[doc = "and the error of the failed call. If all were successful, then the `BatchCompleted`"]
					#[doc = "event is deposited."]
					batch { calls: ::std::vec::Vec<runtime_types::composable_runtime::Call> },
					#[codec(index = 1)]
					#[doc = "Send a call through an indexed pseudonym of the sender."]
					#[doc = ""]
					#[doc = "Filter from origin are passed along. The call will be dispatched with an origin which"]
					#[doc = "use the same filter as the origin of this call."]
					#[doc = ""]
					#[doc = "NOTE: If you need to ensure that any account-based filtering is not honored (i.e."]
					#[doc = "because you expect `proxy` to have been used prior in the call stack and you do not want"]
					#[doc = "the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`"]
					#[doc = "in the Multisig pallet instead."]
					#[doc = ""]
					#[doc = "NOTE: Prior to version *12, this was called `as_limited_sub`."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					as_derivative {
						index: ::core::primitive::u16,
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
					},
					#[codec(index = 2)]
					#[doc = "Send a batch of dispatch calls and atomically execute them."]
					#[doc = "The whole transaction will rollback and fail if any of the calls failed."]
					#[doc = ""]
					#[doc = "May be called from any origin."]
					#[doc = ""]
					#[doc = "- `calls`: The calls to be dispatched from the same origin. The number of call must not"]
					#[doc = "  exceed the constant: `batched_calls_limit` (available in constant metadata)."]
					#[doc = ""]
					#[doc = "If origin is root then call are dispatch without checking origin filter. (This includes"]
					#[doc = "bypassing `frame_system::Config::BaseCallFilter`)."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: O(C) where C is the number of calls to be batched."]
					#[doc = "# </weight>"]
					batch_all { calls: ::std::vec::Vec<runtime_types::composable_runtime::Call> },
					#[codec(index = 3)]
					#[doc = "Dispatches a function call with a provided origin."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Root_."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- O(1)."]
					#[doc = "- Limited storage reads."]
					#[doc = "- One DB write (event)."]
					#[doc = "- Weight of derivative `call` execution + T::WeightInfo::dispatch_as()."]
					#[doc = "# </weight>"]
					dispatch_as {
						as_origin:
							::std::boxed::Box<runtime_types::composable_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::composable_runtime::Call>,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many calls batched."]
					TooManyCalls,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
					#[doc = "well as the error."]
					BatchInterrupted {
						index: ::core::primitive::u32,
						error: runtime_types::sp_runtime::DispatchError,
					},
					#[codec(index = 1)]
					#[doc = "Batch of dispatches completed fully with no error."]
					BatchCompleted,
					#[codec(index = 2)]
					#[doc = "A single item within a Batch of dispatches has completed with no error."]
					ItemCompleted,
					#[codec(index = 3)]
					#[doc = "A call was dispatched. \\[result\\]"]
					DispatchedAs(
						::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					),
				}
			}
		}
		pub mod pallet_xcm {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector and"]
					#[doc = "fee-weight is calculated locally and thus remote weights are assumed to be equal to"]
					#[doc = "local weights."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `dest_weight`: Equal to the total weight on `dest` of the XCM message"]
					#[doc = "  `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`."]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination chain and forward"]
					#[doc = "a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector and"]
					#[doc = "fee-weight is calculated locally and thus remote weights are assumed to be equal to"]
					#[doc = "local weights."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
					#[doc = "  `dest` side."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Execute an XCM message from a local, signed, origin."]
					#[doc = ""]
					#[doc = "An event is deposited indicating whether `msg` could be executed completely or only"]
					#[doc = "partially."]
					#[doc = ""]
					#[doc = "No more than `max_weight` will be used in its attempted execution. If this is less than the"]
					#[doc = "maximum amount of weight that the message could take to be executed, then no execution"]
					#[doc = "attempt will be made."]
					#[doc = ""]
					#[doc = "NOTE: A successful return to this does *not* imply that the `msg` was executed successfully"]
					#[doc = "to completion; only that *some* of it was executed."]
					execute {
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 4)]
					#[doc = "Extoll that a particular destination can be communicated with through a particular"]
					#[doc = "version of XCM."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The destination that is being described."]
					#[doc = "- `xcm_version`: The latest version of XCM that `location` supports."]
					force_xcm_version {
						location:
							::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
						xcm_version: ::core::primitive::u32,
					},
					#[codec(index = 5)]
					#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
					#[doc = "version a destination can accept is unknown)."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
					force_default_xcm_version {
						maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
					},
					#[codec(index = 6)]
					#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
					force_subscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 7)]
					#[doc = "Require that a particular destination should no longer notify us regarding any XCM"]
					#[doc = "version changes."]
					#[doc = ""]
					#[doc = "- `origin`: Must be Root."]
					#[doc = "- `location`: The location to which we are currently subscribed for XCM version"]
					#[doc = "  notifications which we no longer desire."]
					force_unsubscribe_version_notify {
						location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
					},
					#[codec(index = 8)]
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination chain and forward"]
					#[doc = "a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. This should include the assets used to pay the fee on the"]
					#[doc = "  `dest` side."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_reserve_transfer_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
					#[codec(index = 9)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the first asset listed in the `assets` vector."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `dest_weight`: Equal to the total weight on `dest` of the XCM message"]
					#[doc = "  `Teleport { assets, effects: [ BuyExecution{..}, DepositAsset{..} ] }`."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
					#[doc = "to it."]
					Unreachable,
					#[codec(index = 1)]
					#[doc = "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps"]
					#[doc = "a lack of space for buffering the message."]
					SendFailure,
					#[codec(index = 2)]
					#[doc = "The message execution fails the filter."]
					Filtered,
					#[codec(index = 3)]
					#[doc = "The message's weight could not be determined."]
					UnweighableMessage,
					#[codec(index = 4)]
					#[doc = "The destination `MultiLocation` provided cannot be inverted."]
					DestinationNotInvertible,
					#[codec(index = 5)]
					#[doc = "The assets to be sent are empty."]
					Empty,
					#[codec(index = 6)]
					#[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
					CannotReanchor,
					#[codec(index = 7)]
					#[doc = "Too many assets have been attempted for transfer."]
					TooManyAssets,
					#[codec(index = 8)]
					#[doc = "Origin is invalid for sending."]
					InvalidOrigin,
					#[codec(index = 9)]
					#[doc = "The version of the `Versioned` value used is not able to be interpreted."]
					BadVersion,
					#[codec(index = 10)]
					#[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
					#[doc = "desired version of XCM)."]
					BadLocation,
					#[codec(index = 11)]
					#[doc = "The referenced subscription could not be found."]
					NoSubscription,
					#[codec(index = 12)]
					#[doc = "The location is invalid since it already has a subscription from us."]
					AlreadySubscribed,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "Execution of an XCM message was attempted."]
					#[doc = ""]
					#[doc = "\\[ outcome \\]"]
					Attempted(runtime_types::xcm::v2::traits::Outcome),
					#[codec(index = 1)]
					#[doc = "A XCM message was sent."]
					#[doc = ""]
					#[doc = "\\[ origin, destination, message \\]"]
					Sent(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::v2::Xcm,
					),
					#[codec(index = 2)]
					#[doc = "Query response received which does not match a registered query. This may be because a"]
					#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
					#[doc = "because the query timed out."]
					#[doc = ""]
					#[doc = "\\[ origin location, id \\]"]
					UnexpectedResponse(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 3)]
					#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
					#[doc = "no registered notification call."]
					#[doc = ""]
					#[doc = "\\[ id, response \\]"]
					ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
					#[codec(index = 4)]
					#[doc = "Query response has been received and query is removed. The registered notification has"]
					#[doc = "been dispatched and executed successfully."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					Notified(::core::primitive::u64, ::core::primitive::u8, ::core::primitive::u8),
					#[codec(index = 5)]
					#[doc = "Query response has been received and query is removed. The registered notification could"]
					#[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
					#[doc = "originally budgeted by this runtime for the query result."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index, actual weight, max budgeted weight \\]"]
					NotifyOverweight(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
						::core::primitive::u64,
						::core::primitive::u64,
					),
					#[codec(index = 6)]
					#[doc = "Query response has been received and query is removed. There was a general error with"]
					#[doc = "dispatching the notification call."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					NotifyDispatchError(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 7)]
					#[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
					#[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
					#[doc = "is not `(origin, QueryId, Response)`."]
					#[doc = ""]
					#[doc = "\\[ id, pallet index, call index \\]"]
					NotifyDecodeFailed(
						::core::primitive::u64,
						::core::primitive::u8,
						::core::primitive::u8,
					),
					#[codec(index = 8)]
					#[doc = "Expected query response has been received but the origin location of the response does"]
					#[doc = "not match that expected. The query remains registered for a later, valid, response to"]
					#[doc = "be received and acted upon."]
					#[doc = ""]
					#[doc = "\\[ origin location, id, expected location \\]"]
					InvalidResponder(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						::core::option::Option<
							runtime_types::xcm::v1::multilocation::MultiLocation,
						>,
					),
					#[codec(index = 9)]
					#[doc = "Expected query response has been received but the expected origin location placed in"]
					#[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
					#[doc = ""]
					#[doc = "This is unexpected (since a location placed in storage in a previously executing"]
					#[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
					#[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
					#[doc = "needed."]
					#[doc = ""]
					#[doc = "\\[ origin location, id \\]"]
					InvalidResponderVersion(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
					),
					#[codec(index = 10)]
					#[doc = "Received query response has been read and removed."]
					#[doc = ""]
					#[doc = "\\[ id \\]"]
					ResponseTaken(::core::primitive::u64),
					#[codec(index = 11)]
					#[doc = "Some assets have been placed in an asset trap."]
					#[doc = ""]
					#[doc = "\\[ hash, origin, assets \\]"]
					AssetsTrapped(
						::subxt::sp_core::H256,
						runtime_types::xcm::v1::multilocation::MultiLocation,
						runtime_types::xcm::VersionedMultiAssets,
					),
					#[codec(index = 12)]
					#[doc = "An XCM version change notification message has been attempted to be sent."]
					#[doc = ""]
					#[doc = "\\[ destination, result \\]"]
					VersionChangeNotified(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 13)]
					#[doc = "The supported version of a location has been changed. This might be through an"]
					#[doc = "automatic notification or a manual intervention."]
					#[doc = ""]
					#[doc = "\\[ location, XCM version \\]"]
					SupportedVersionChanged(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u32,
					),
					#[codec(index = 14)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "sending the notification to it."]
					#[doc = ""]
					#[doc = "\\[ location, query ID, error \\]"]
					NotifyTargetSendFail(
						runtime_types::xcm::v1::multilocation::MultiLocation,
						::core::primitive::u64,
						runtime_types::xcm::v2::traits::Error,
					),
					#[codec(index = 15)]
					#[doc = "A given location which had a version change subscription was dropped owing to an error"]
					#[doc = "migrating the location to our new XCM format."]
					#[doc = ""]
					#[doc = "\\[ location, query ID \\]"]
					NotifyTargetMigrationFail(
						runtime_types::xcm::VersionedMultiLocation,
						::core::primitive::u64,
					),
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Origin {
					#[codec(index = 0)]
					Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
					#[codec(index = 1)]
					Response(runtime_types::xcm::v1::multilocation::MultiLocation),
				}
			}
		}
		pub mod polkadot_core_primitives {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Decode,
					:: subxt :: codec :: Encode,
					Debug,
				)]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct AbridgedHrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::sp_core::H256>,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct PersistedValidationData<_0, _1> {
					pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub relay_parent_number: _1,
					pub relay_parent_storage_root: _0,
					pub max_pov_size: _1,
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub struct H256(pub [::core::primitive::u8; 32usize]);
		}
		pub mod primitives {
			use super::runtime_types;
			pub mod currency {
				use super::runtime_types;
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Decode,
					:: subxt :: codec :: Encode,
					Debug,
				)]
				pub struct CurrencyId(pub ::core::primitive::u128);
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Decode,
					:: subxt :: codec :: Encode,
					Debug,
				)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Decode,
					:: subxt :: codec :: Encode,
					Debug,
				)]
				pub struct Perbill(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: codec :: CompactAs,
					:: subxt :: codec :: Decode,
					:: subxt :: codec :: Encode,
					Debug,
				)]
				pub struct Permill(pub ::core::primitive::u32);
			}
		}
		pub mod sp_consensus_aura {
			use super::runtime_types;
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: codec :: CompactAs,
				:: subxt :: codec :: Decode,
				:: subxt :: codec :: Encode,
				Debug,
			)]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod sr25519 {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum Void {}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						pub ::std::vec::Vec<::core::primitive::u8>,
						#[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
					);
				}
			}
			pub mod multiaddress {
				use super::runtime_types;
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct BlakeTwo256;
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct StorageProof {
					pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v0 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
					}
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v1::multiasset::AssetId,
						pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
					}
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
					}
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
					);
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v1::multilocation::Junctions,
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					Version(::core::primitive::u32),
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
					#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(::core::primitive::u64),
						#[codec(index = 1)]
						Incomplete(::core::primitive::u64, runtime_types::xcm::v2::traits::Error),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v2::traits::Error),
					}
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum VersionedMultiAssets {
				#[codec(index = 0)]
				V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multiasset::MultiAssets),
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
			pub enum VersionedMultiLocation {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::multi_location::MultiLocation),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multilocation::MultiLocation),
			}
			#[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
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
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
		fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
			if let &Self::Module { index, error } = self {
				Some(::subxt::ModuleErrorData { pallet_index: index, error: [error, 0, 0, 0] })
			} else {
				None
			}
		}
	}
	pub struct RuntimeApi<T: ::subxt::Config, X> {
		pub client: ::subxt::Client<T>,
		marker: ::core::marker::PhantomData<X>,
	}
	impl<T, X> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X>
	where
		T: ::subxt::Config,
		X: ::subxt::extrinsic::ExtrinsicParams<T>,
	{
		fn from(client: ::subxt::Client<T>) -> Self {
			Self { client, marker: ::core::marker::PhantomData }
		}
	}
	impl<'a, T, X> RuntimeApi<T, X>
	where
		T: ::subxt::Config,
		X: ::subxt::extrinsic::ExtrinsicParams<T>,
	{
		pub fn validate_metadata(&'a self) -> Result<(), ::subxt::MetadataError> {
			if self.client.metadata().metadata_hash(&PALLETS) !=
				[
					50u8, 172u8, 221u8, 106u8, 123u8, 83u8, 229u8, 17u8, 96u8, 48u8, 158u8, 63u8,
					252u8, 40u8, 95u8, 102u8, 85u8, 82u8, 100u8, 216u8, 172u8, 107u8, 210u8, 5u8,
					161u8, 60u8, 131u8, 31u8, 244u8, 190u8, 123u8, 62u8,
				] {
				Err(::subxt::MetadataError::IncompatibleMetadata)
			} else {
				Ok(())
			}
		}
		pub fn constants(&'a self) -> ConstantsApi<'a, T> {
			ConstantsApi { client: &self.client }
		}
		pub fn storage(&'a self) -> StorageApi<'a, T> {
			StorageApi { client: &self.client }
		}
		pub fn tx(&'a self) -> TransactionApi<'a, T, X> {
			TransactionApi { client: &self.client, marker: ::core::marker::PhantomData }
		}
		pub fn events(&'a self) -> EventsApi<'a, T> {
			EventsApi { client: &self.client }
		}
	}
	pub struct EventsApi<'a, T: ::subxt::Config> {
		client: &'a ::subxt::Client<T>,
	}
	impl<'a, T: ::subxt::Config> EventsApi<'a, T> {
		pub async fn at(
			&self,
			block_hash: T::Hash,
		) -> Result<::subxt::events::Events<'a, T, Event>, ::subxt::BasicError> {
			::subxt::events::at::<T, Event>(self.client, block_hash).await
		}
		pub async fn subscribe(
			&self,
		) -> Result<
			::subxt::events::EventSubscription<'a, ::subxt::events::EventSub<T::Header>, T, Event>,
			::subxt::BasicError,
		> {
			::subxt::events::subscribe::<T, Event>(self.client).await
		}
		pub async fn subscribe_finalized(
			&self,
		) -> Result<
			::subxt::events::EventSubscription<
				'a,
				::subxt::events::FinalizedEventSub<'a, T::Header>,
				T,
				Event,
			>,
			::subxt::BasicError,
		> {
			::subxt::events::subscribe_finalized::<T, Event>(self.client).await
		}
	}
	pub struct ConstantsApi<'a, T: ::subxt::Config> {
		client: &'a ::subxt::Client<T>,
	}
	impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
		pub fn system(&self) -> system::constants::ConstantsApi<'a, T> {
			system::constants::ConstantsApi::new(self.client)
		}
		pub fn timestamp(&self) -> timestamp::constants::ConstantsApi<'a, T> {
			timestamp::constants::ConstantsApi::new(self.client)
		}
		pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi<'a, T> {
			transaction_payment::constants::ConstantsApi::new(self.client)
		}
		pub fn indices(&self) -> indices::constants::ConstantsApi<'a, T> {
			indices::constants::ConstantsApi::new(self.client)
		}
		pub fn balances(&self) -> balances::constants::ConstantsApi<'a, T> {
			balances::constants::ConstantsApi::new(self.client)
		}
		pub fn authorship(&self) -> authorship::constants::ConstantsApi<'a, T> {
			authorship::constants::ConstantsApi::new(self.client)
		}
		pub fn treasury(&self) -> treasury::constants::ConstantsApi<'a, T> {
			treasury::constants::ConstantsApi::new(self.client)
		}
		pub fn democracy(&self) -> democracy::constants::ConstantsApi<'a, T> {
			democracy::constants::ConstantsApi::new(self.client)
		}
		pub fn scheduler(&self) -> scheduler::constants::ConstantsApi<'a, T> {
			scheduler::constants::ConstantsApi::new(self.client)
		}
		pub fn utility(&self) -> utility::constants::ConstantsApi<'a, T> {
			utility::constants::ConstantsApi::new(self.client)
		}
		pub fn tokens(&self) -> tokens::constants::ConstantsApi<'a, T> {
			tokens::constants::ConstantsApi::new(self.client)
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
		pub fn liquid_crowdloan(&self) -> liquid_crowdloan::storage::StorageApi<'a, T> {
			liquid_crowdloan::storage::StorageApi::new(self.client)
		}
		pub fn tokens(&self) -> tokens::storage::StorageApi<'a, T> {
			tokens::storage::StorageApi::new(self.client)
		}
	}
	pub struct TransactionApi<'a, T: ::subxt::Config, X> {
		client: &'a ::subxt::Client<T>,
		marker: ::core::marker::PhantomData<X>,
	}
	impl<'a, T, X> TransactionApi<'a, T, X>
	where
		T: ::subxt::Config,
		X: ::subxt::extrinsic::ExtrinsicParams<T>,
	{
		pub fn system(&self) -> system::calls::TransactionApi<'a, T, X> {
			system::calls::TransactionApi::new(self.client)
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X> {
			timestamp::calls::TransactionApi::new(self.client)
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X> {
			sudo::calls::TransactionApi::new(self.client)
		}
		pub fn indices(&self) -> indices::calls::TransactionApi<'a, T, X> {
			indices::calls::TransactionApi::new(self.client)
		}
		pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X> {
			balances::calls::TransactionApi::new(self.client)
		}
		pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi<'a, T, X> {
			parachain_system::calls::TransactionApi::new(self.client)
		}
		pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T, X> {
			authorship::calls::TransactionApi::new(self.client)
		}
		pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi<'a, T, X> {
			collator_selection::calls::TransactionApi::new(self.client)
		}
		pub fn session(&self) -> session::calls::TransactionApi<'a, T, X> {
			session::calls::TransactionApi::new(self.client)
		}
		pub fn council(&self) -> council::calls::TransactionApi<'a, T, X> {
			council::calls::TransactionApi::new(self.client)
		}
		pub fn council_membership(&self) -> council_membership::calls::TransactionApi<'a, T, X> {
			council_membership::calls::TransactionApi::new(self.client)
		}
		pub fn treasury(&self) -> treasury::calls::TransactionApi<'a, T, X> {
			treasury::calls::TransactionApi::new(self.client)
		}
		pub fn democracy(&self) -> democracy::calls::TransactionApi<'a, T, X> {
			democracy::calls::TransactionApi::new(self.client)
		}
		pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T, X> {
			scheduler::calls::TransactionApi::new(self.client)
		}
		pub fn utility(&self) -> utility::calls::TransactionApi<'a, T, X> {
			utility::calls::TransactionApi::new(self.client)
		}
		pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi<'a, T, X> {
			xcmp_queue::calls::TransactionApi::new(self.client)
		}
		pub fn relayer_xcm(&self) -> relayer_xcm::calls::TransactionApi<'a, T, X> {
			relayer_xcm::calls::TransactionApi::new(self.client)
		}
		pub fn cumulus_xcm(&self) -> cumulus_xcm::calls::TransactionApi<'a, T, X> {
			cumulus_xcm::calls::TransactionApi::new(self.client)
		}
		pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi<'a, T, X> {
			dmp_queue::calls::TransactionApi::new(self.client)
		}
		pub fn liquid_crowdloan(&self) -> liquid_crowdloan::calls::TransactionApi<'a, T, X> {
			liquid_crowdloan::calls::TransactionApi::new(self.client)
		}
		pub fn tokens(&self) -> tokens::calls::TransactionApi<'a, T, X> {
			tokens::calls::TransactionApi::new(self.client)
		}
	}
}
