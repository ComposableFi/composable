#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
	use super::api as root_mod;
	pub static PALLETS: [&str; 43usize] = [
		"System",
		"Babe",
		"Timestamp",
		"Indices",
		"Balances",
		"TransactionPayment",
		"Authorship",
		"Offences",
		"Historical",
		"Session",
		"Grandpa",
		"ImOnline",
		"AuthorityDiscovery",
		"ParachainsOrigin",
		"Configuration",
		"ParasShared",
		"ParaInclusion",
		"ParaInherent",
		"ParaScheduler",
		"Paras",
		"Initializer",
		"Dmp",
		"Ump",
		"Hrmp",
		"ParaSessionInfo",
		"ParasDisputes",
		"Registrar",
		"Auctions",
		"Crowdloan",
		"Slots",
		"ParasSudoWrapper",
		"AssignedSlots",
		"Sudo",
		"Mmr",
		"Beefy",
		"MmrLeaf",
		"ValidatorManager",
		"Collective",
		"Membership",
		"Utility",
		"Proxy",
		"Multisig",
		"XcmPallet",
	];
	#[derive(:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug)]
	pub enum Event {
		#[codec(index = 0)]
		System(system::Event),
		#[codec(index = 3)]
		Indices(indices::Event),
		#[codec(index = 4)]
		Balances(balances::Event),
		#[codec(index = 5)]
		TransactionPayment(transaction_payment::Event),
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
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct FillBlock {
				pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Remark {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHeapPages {
				pub pages: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetCode {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetCodeWithoutChecks {
				pub code: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetStorage {
				pub items: ::std::vec::Vec<(
					::std::vec::Vec<::core::primitive::u8>,
					::std::vec::Vec<::core::primitive::u8>,
				)>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillStorage {
				pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillPrefix {
				pub prefix: ::std::vec::Vec<::core::primitive::u8>,
				pub subkeys: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemarkWithEvent {
				pub remark: ::std::vec::Vec<::core::primitive::u8>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "A dispatch that will fill the block weight up to the given ratio."]
				pub fn fill_block(
					&self,
					ratio: runtime_types::sp_arithmetic::per_things::Perbill,
				) -> ::subxt::tx::StaticTxPayload<FillBlock> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"fill_block",
						FillBlock { ratio },
						[
							48u8, 18u8, 205u8, 90u8, 222u8, 4u8, 20u8, 251u8, 173u8, 76u8, 167u8,
							4u8, 83u8, 203u8, 160u8, 89u8, 132u8, 218u8, 191u8, 145u8, 130u8,
							245u8, 177u8, 201u8, 169u8, 129u8, 173u8, 105u8, 88u8, 45u8, 136u8,
							191u8,
						],
					)
				}
				#[doc = "Make some on-chain remark."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(1)`"]
				#[doc = "# </weight>"]
				pub fn remark(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<Remark> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"remark",
						Remark { remark },
						[
							101u8, 80u8, 195u8, 226u8, 224u8, 247u8, 60u8, 128u8, 3u8, 101u8, 51u8,
							147u8, 96u8, 126u8, 76u8, 230u8, 194u8, 227u8, 191u8, 73u8, 160u8,
							146u8, 87u8, 147u8, 243u8, 28u8, 228u8, 116u8, 224u8, 181u8, 129u8,
							160u8,
						],
					)
				}
				#[doc = "Set the number of pages in the WebAssembly environment's heap."]
				pub fn set_heap_pages(
					&self,
					pages: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SetHeapPages> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_heap_pages",
						SetHeapPages { pages },
						[
							43u8, 103u8, 128u8, 49u8, 156u8, 136u8, 11u8, 204u8, 80u8, 6u8, 244u8,
							86u8, 171u8, 44u8, 140u8, 225u8, 142u8, 198u8, 43u8, 87u8, 26u8, 45u8,
							125u8, 222u8, 165u8, 254u8, 172u8, 158u8, 39u8, 178u8, 86u8, 87u8,
						],
					)
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
				) -> ::subxt::tx::StaticTxPayload<SetCode> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_code",
						SetCode { code },
						[
							27u8, 104u8, 244u8, 205u8, 188u8, 254u8, 121u8, 13u8, 106u8, 120u8,
							244u8, 108u8, 97u8, 84u8, 100u8, 68u8, 26u8, 69u8, 93u8, 128u8, 107u8,
							4u8, 3u8, 142u8, 13u8, 134u8, 196u8, 62u8, 113u8, 181u8, 14u8, 40u8,
						],
					)
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
				) -> ::subxt::tx::StaticTxPayload<SetCodeWithoutChecks> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_code_without_checks",
						SetCodeWithoutChecks { code },
						[
							102u8, 160u8, 125u8, 235u8, 30u8, 23u8, 45u8, 239u8, 112u8, 148u8,
							159u8, 158u8, 42u8, 93u8, 206u8, 94u8, 80u8, 250u8, 66u8, 195u8, 60u8,
							40u8, 142u8, 169u8, 183u8, 80u8, 80u8, 96u8, 3u8, 231u8, 99u8, 216u8,
						],
					)
				}
				#[doc = "Set some items of storage."]
				pub fn set_storage(
					&self,
					items: ::std::vec::Vec<(
						::std::vec::Vec<::core::primitive::u8>,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
				) -> ::subxt::tx::StaticTxPayload<SetStorage> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"set_storage",
						SetStorage { items },
						[
							74u8, 43u8, 106u8, 255u8, 50u8, 151u8, 192u8, 155u8, 14u8, 90u8, 19u8,
							45u8, 165u8, 16u8, 235u8, 242u8, 21u8, 131u8, 33u8, 172u8, 119u8, 78u8,
							140u8, 10u8, 107u8, 202u8, 122u8, 235u8, 181u8, 191u8, 22u8, 116u8,
						],
					)
				}
				#[doc = "Kill some items from storage."]
				pub fn kill_storage(
					&self,
					keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
				) -> ::subxt::tx::StaticTxPayload<KillStorage> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"kill_storage",
						KillStorage { keys },
						[
							174u8, 174u8, 13u8, 174u8, 75u8, 138u8, 128u8, 235u8, 222u8, 216u8,
							85u8, 18u8, 198u8, 1u8, 138u8, 70u8, 19u8, 108u8, 209u8, 41u8, 228u8,
							67u8, 130u8, 230u8, 160u8, 207u8, 11u8, 180u8, 139u8, 242u8, 41u8,
							15u8,
						],
					)
				}
				#[doc = "Kill all storage items with a key that starts with the given prefix."]
				#[doc = ""]
				#[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
				#[doc = "the prefix we are removing to accurately calculate the weight of this function."]
				pub fn kill_prefix(
					&self,
					prefix: ::std::vec::Vec<::core::primitive::u8>,
					subkeys: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<KillPrefix> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"kill_prefix",
						KillPrefix { prefix, subkeys },
						[
							203u8, 116u8, 217u8, 42u8, 154u8, 215u8, 77u8, 217u8, 13u8, 22u8,
							193u8, 2u8, 128u8, 115u8, 179u8, 115u8, 187u8, 218u8, 129u8, 34u8,
							80u8, 4u8, 173u8, 120u8, 92u8, 35u8, 237u8, 112u8, 201u8, 207u8, 200u8,
							48u8,
						],
					)
				}
				#[doc = "Make some on-chain remark and emit event."]
				pub fn remark_with_event(
					&self,
					remark: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<RemarkWithEvent> {
					::subxt::tx::StaticTxPayload::new(
						"System",
						"remark_with_event",
						RemarkWithEvent { remark },
						[
							123u8, 225u8, 180u8, 179u8, 144u8, 74u8, 27u8, 85u8, 101u8, 75u8,
							134u8, 44u8, 181u8, 25u8, 183u8, 158u8, 14u8, 213u8, 56u8, 225u8,
							136u8, 88u8, 26u8, 114u8, 178u8, 43u8, 176u8, 43u8, 240u8, 84u8, 116u8,
							46u8,
						],
					)
				}
			}
		}
		#[doc = "Event for the System pallet."]
		pub type Event = runtime_types::frame_system::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An extrinsic completed successfully."]
			pub struct ExtrinsicSuccess {
				pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicSuccess";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An extrinsic failed."]
			pub struct ExtrinsicFailed {
				pub dispatch_error: runtime_types::sp_runtime::DispatchError,
				pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
			}
			impl ::subxt::events::StaticEvent for ExtrinsicFailed {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "ExtrinsicFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "`:code` was updated."]
			pub struct CodeUpdated;
			impl ::subxt::events::StaticEvent for CodeUpdated {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "CodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new account was created."]
			pub struct NewAccount {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for NewAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "NewAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was reaped."]
			pub struct KilledAccount {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for KilledAccount {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "KilledAccount";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "On on-chain remark happened."]
			pub struct Remarked {
				pub sender: ::subxt::ext::sp_core::crypto::AccountId32,
				pub hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Remarked {
				const PALLET: &'static str = "System";
				const EVENT: &'static str = "Remarked";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The full account information for a particular account ID."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
						],
					)
				}
				#[doc = " The full account information for a particular account ID."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::AccountInfo<
							::core::primitive::u32,
							runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Account",
						Vec::new(),
						[
							176u8, 187u8, 21u8, 220u8, 159u8, 204u8, 127u8, 14u8, 21u8, 69u8, 77u8,
							114u8, 230u8, 141u8, 107u8, 79u8, 23u8, 16u8, 174u8, 243u8, 252u8,
							42u8, 65u8, 120u8, 229u8, 38u8, 210u8, 255u8, 22u8, 40u8, 109u8, 223u8,
						],
					)
				}
				#[doc = " Total extrinsics count for the current block."]
				pub fn extrinsic_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicCount",
						vec![],
						[
							223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
							222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
							41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
						],
					)
				}
				#[doc = " The current weight for the block."]
				pub fn block_weight(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::weights::PerDispatchClass<
							::core::primitive::u64,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockWeight",
						vec![],
						[
							91u8, 211u8, 177u8, 36u8, 147u8, 249u8, 55u8, 164u8, 48u8, 49u8, 55u8,
							11u8, 121u8, 193u8, 103u8, 69u8, 38u8, 142u8, 148u8, 36u8, 137u8, 41u8,
							115u8, 195u8, 31u8, 174u8, 163u8, 125u8, 69u8, 5u8, 94u8, 79u8,
						],
					)
				}
				#[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
				pub fn all_extrinsics_len(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"AllExtrinsicsLen",
						vec![],
						[
							202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
							254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
							219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
							134u8, 60u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}
				#[doc = " Map of block numbers to block hashes."]
				pub fn block_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"BlockHash",
						Vec::new(),
						[
							50u8, 112u8, 176u8, 239u8, 175u8, 18u8, 205u8, 20u8, 241u8, 195u8,
							21u8, 228u8, 186u8, 57u8, 200u8, 25u8, 38u8, 44u8, 106u8, 20u8, 168u8,
							80u8, 76u8, 235u8, 12u8, 51u8, 137u8, 149u8, 200u8, 4u8, 220u8, 237u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}
				#[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
				pub fn extrinsic_data_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExtrinsicData",
						Vec::new(),
						[
							210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
							254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
							59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
						],
					)
				}
				#[doc = " The current block number being processed. Set by `execute_block`."]
				pub fn number(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Number",
						vec![],
						[
							228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
							246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
							36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
							164u8, 191u8,
						],
					)
				}
				#[doc = " Hash of the previous block."]
				pub fn parent_hash(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ParentHash",
						vec![],
						[
							232u8, 206u8, 177u8, 119u8, 38u8, 57u8, 233u8, 50u8, 225u8, 49u8,
							169u8, 176u8, 210u8, 51u8, 231u8, 176u8, 234u8, 186u8, 188u8, 112u8,
							15u8, 152u8, 195u8, 232u8, 201u8, 97u8, 208u8, 249u8, 9u8, 163u8, 69u8,
							36u8,
						],
					)
				}
				#[doc = " Digest of the current block, also part of the block header."]
				pub fn digest(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::generic::digest::Digest,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Digest",
						vec![],
						[
							83u8, 141u8, 200u8, 132u8, 182u8, 55u8, 197u8, 122u8, 13u8, 159u8,
							31u8, 42u8, 60u8, 191u8, 89u8, 221u8, 242u8, 47u8, 199u8, 213u8, 48u8,
							216u8, 131u8, 168u8, 245u8, 82u8, 56u8, 190u8, 62u8, 69u8, 96u8, 37u8,
						],
					)
				}
				#[doc = " Events deposited for the current block."]
				#[doc = ""]
				#[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
				#[doc = " It could otherwise inflate the PoV size of a block."]
				#[doc = ""]
				#[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
				#[doc = " just in case someone still reads them from within the runtime."]
				pub fn events(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::frame_system::EventRecord<
								runtime_types::rococo_runtime::Event,
								::subxt::ext::sp_core::H256,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"Events",
						vec![],
						[
							218u8, 183u8, 92u8, 188u8, 82u8, 44u8, 55u8, 174u8, 72u8, 112u8, 231u8,
							77u8, 250u8, 228u8, 39u8, 66u8, 44u8, 61u8, 171u8, 196u8, 231u8, 39u8,
							167u8, 216u8, 202u8, 197u8, 84u8, 75u8, 50u8, 177u8, 134u8, 178u8,
						],
					)
				}
				#[doc = " The number of events in the `Events<T>` list."]
				pub fn event_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventCount",
						vec![],
						[
							236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
							203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
							161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
							112u8,
						],
					)
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
				pub fn event_topics(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventTopics",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
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
				pub fn event_topics_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"EventTopics",
						Vec::new(),
						[
							205u8, 90u8, 142u8, 190u8, 176u8, 37u8, 94u8, 82u8, 98u8, 1u8, 129u8,
							63u8, 246u8, 101u8, 130u8, 58u8, 216u8, 16u8, 139u8, 196u8, 154u8,
							111u8, 110u8, 178u8, 24u8, 44u8, 183u8, 176u8, 232u8, 82u8, 223u8,
							38u8,
						],
					)
				}
				#[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
				pub fn last_runtime_upgrade(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::LastRuntimeUpgradeInfo,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"LastRuntimeUpgrade",
						vec![],
						[
							52u8, 37u8, 117u8, 111u8, 57u8, 130u8, 196u8, 14u8, 99u8, 77u8, 91u8,
							126u8, 178u8, 249u8, 78u8, 34u8, 9u8, 194u8, 92u8, 105u8, 113u8, 81u8,
							185u8, 127u8, 245u8, 184u8, 60u8, 29u8, 234u8, 182u8, 96u8, 196u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
				pub fn upgraded_to_u32_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"UpgradedToU32RefCount",
						vec![],
						[
							171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
							178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
							83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
						],
					)
				}
				#[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
				#[doc = " (default) if not."]
				pub fn upgraded_to_triple_ref_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"UpgradedToTripleRefCount",
						vec![],
						[
							90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
							210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
							155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
							185u8,
						],
					)
				}
				#[doc = " The execution phase of the block."]
				pub fn execution_phase(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_system::Phase>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"System",
						"ExecutionPhase",
						vec![],
						[
							230u8, 183u8, 221u8, 135u8, 226u8, 223u8, 55u8, 104u8, 138u8, 224u8,
							103u8, 156u8, 222u8, 99u8, 203u8, 199u8, 164u8, 168u8, 193u8, 133u8,
							201u8, 155u8, 63u8, 95u8, 17u8, 206u8, 165u8, 123u8, 161u8, 33u8,
							172u8, 93u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Block & extrinsics weights: base values and limits."]
				pub fn block_weights(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockWeights,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"BlockWeights",
						[
							153u8, 164u8, 86u8, 79u8, 97u8, 114u8, 248u8, 181u8, 179u8, 186u8,
							214u8, 124u8, 215u8, 96u8, 116u8, 109u8, 215u8, 182u8, 61u8, 10u8,
							77u8, 74u8, 29u8, 125u8, 131u8, 111u8, 249u8, 208u8, 233u8, 170u8,
							11u8, 14u8,
						],
					)
				}
				#[doc = " The maximum length of a block (in bytes)."]
				pub fn block_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_system::limits::BlockLength,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"BlockLength",
						[
							116u8, 184u8, 225u8, 228u8, 207u8, 203u8, 4u8, 220u8, 234u8, 198u8,
							150u8, 108u8, 205u8, 87u8, 194u8, 131u8, 229u8, 51u8, 140u8, 4u8, 47u8,
							12u8, 200u8, 144u8, 153u8, 62u8, 51u8, 39u8, 138u8, 205u8, 203u8,
							236u8,
						],
					)
				}
				#[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
				pub fn block_hash_count(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"BlockHashCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The weight of runtime database operations the runtime can invoke."]
				pub fn db_weight(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::weights::RuntimeDbWeight,
					>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"DbWeight",
						[
							124u8, 162u8, 190u8, 149u8, 49u8, 177u8, 162u8, 231u8, 62u8, 167u8,
							199u8, 181u8, 43u8, 232u8, 185u8, 116u8, 195u8, 51u8, 233u8, 223u8,
							20u8, 129u8, 246u8, 13u8, 65u8, 180u8, 64u8, 9u8, 157u8, 59u8, 245u8,
							118u8,
						],
					)
				}
				#[doc = " Get the chain's current version."]
				pub fn version(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_version::RuntimeVersion>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"Version",
						[
							93u8, 98u8, 57u8, 243u8, 229u8, 8u8, 234u8, 231u8, 72u8, 230u8, 139u8,
							47u8, 63u8, 181u8, 17u8, 2u8, 220u8, 231u8, 104u8, 237u8, 185u8, 143u8,
							165u8, 253u8, 188u8, 76u8, 147u8, 12u8, 170u8, 26u8, 74u8, 200u8,
						],
					)
				}
				#[doc = " The designated SS85 prefix of this chain."]
				#[doc = ""]
				#[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
				#[doc = " that the runtime should know about the prefix in order to make use of it as"]
				#[doc = " an identifier of the chain."]
				pub fn ss58_prefix(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"System",
						"SS58Prefix",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod babe {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReportEquivocation {
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReportEquivocationUnsigned {
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PlanConfigChange {
				pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Report authority equivocation/misbehavior. This method will verify"]
				#[doc = "the equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence will"]
				#[doc = "be reported."]
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
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocation> {
					::subxt::tx::StaticTxPayload::new(
						"Babe",
						"report_equivocation",
						ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							177u8, 237u8, 107u8, 138u8, 237u8, 233u8, 30u8, 195u8, 112u8, 176u8,
							185u8, 113u8, 157u8, 221u8, 134u8, 151u8, 62u8, 151u8, 64u8, 164u8,
							254u8, 112u8, 2u8, 94u8, 175u8, 79u8, 160u8, 3u8, 72u8, 145u8, 244u8,
							137u8,
						],
					)
				}
				#[doc = "Report authority equivocation/misbehavior. This method will verify"]
				#[doc = "the equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence will"]
				#[doc = "be reported."]
				#[doc = "This extrinsic must be called unsigned and it is expected that only"]
				#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
				#[doc = "if the block author is defined it will be defined as the equivocation"]
				#[doc = "reporter."]
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
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned> {
					::subxt::tx::StaticTxPayload::new(
						"Babe",
						"report_equivocation_unsigned",
						ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							56u8, 103u8, 238u8, 118u8, 61u8, 192u8, 222u8, 87u8, 254u8, 24u8,
							138u8, 219u8, 210u8, 85u8, 201u8, 147u8, 128u8, 49u8, 199u8, 144u8,
							46u8, 158u8, 163u8, 31u8, 101u8, 224u8, 72u8, 98u8, 68u8, 120u8, 215u8,
							19u8,
						],
					)
				}
				#[doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"]
				#[doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."]
				#[doc = "Multiple calls to this method will replace any existing planned config change that had"]
				#[doc = "not been enacted yet."]
				pub fn plan_config_change(
					&self,
					config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
				) -> ::subxt::tx::StaticTxPayload<PlanConfigChange> {
					::subxt::tx::StaticTxPayload::new(
						"Babe",
						"plan_config_change",
						PlanConfigChange { config },
						[
							229u8, 157u8, 41u8, 58u8, 56u8, 4u8, 52u8, 107u8, 104u8, 20u8, 42u8,
							110u8, 1u8, 17u8, 45u8, 196u8, 30u8, 135u8, 63u8, 46u8, 40u8, 137u8,
							209u8, 37u8, 24u8, 108u8, 251u8, 189u8, 77u8, 208u8, 74u8, 32u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Current epoch index."]
				pub fn epoch_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"EpochIndex",
						vec![],
						[
							51u8, 27u8, 91u8, 156u8, 118u8, 99u8, 46u8, 219u8, 190u8, 147u8, 205u8,
							23u8, 106u8, 169u8, 121u8, 218u8, 208u8, 235u8, 135u8, 127u8, 243u8,
							41u8, 55u8, 243u8, 235u8, 122u8, 57u8, 86u8, 37u8, 90u8, 208u8, 71u8,
						],
					)
				}
				#[doc = " Current epoch authorities."]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<(
							runtime_types::sp_consensus_babe::app::Public,
							::core::primitive::u64,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Authorities",
						vec![],
						[
							61u8, 8u8, 133u8, 111u8, 169u8, 120u8, 0u8, 213u8, 31u8, 159u8, 204u8,
							212u8, 18u8, 205u8, 93u8, 84u8, 140u8, 108u8, 136u8, 209u8, 234u8,
							107u8, 145u8, 9u8, 204u8, 224u8, 105u8, 9u8, 238u8, 241u8, 65u8, 30u8,
						],
					)
				}
				#[doc = " The slot at which the first epoch actually started. This is 0"]
				#[doc = " until the first block of the chain."]
				pub fn genesis_slot(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"GenesisSlot",
						vec![],
						[
							234u8, 127u8, 243u8, 100u8, 124u8, 160u8, 66u8, 248u8, 48u8, 218u8,
							61u8, 52u8, 54u8, 142u8, 158u8, 77u8, 32u8, 63u8, 156u8, 39u8, 94u8,
							255u8, 192u8, 238u8, 170u8, 118u8, 58u8, 42u8, 199u8, 61u8, 199u8,
							77u8,
						],
					)
				}
				#[doc = " Current slot number."]
				pub fn current_slot(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::sp_consensus_slots::Slot>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"CurrentSlot",
						vec![],
						[
							139u8, 237u8, 185u8, 137u8, 251u8, 179u8, 69u8, 167u8, 133u8, 168u8,
							204u8, 64u8, 178u8, 123u8, 92u8, 250u8, 119u8, 190u8, 208u8, 178u8,
							208u8, 176u8, 124u8, 187u8, 74u8, 165u8, 33u8, 78u8, 161u8, 206u8, 8u8,
							108u8,
						],
					)
				}
				#[doc = " The epoch randomness for the *current* epoch."]
				#[doc = ""]
				#[doc = " # Security"]
				#[doc = ""]
				#[doc = " This MUST NOT be used for gambling, as it can be influenced by a"]
				#[doc = " malicious validator in the short term. It MAY be used in many"]
				#[doc = " cryptographic protocols, however, so long as one remembers that this"]
				#[doc = " (like everything else on-chain) it is public. For example, it can be"]
				#[doc = " used where a number is needed that cannot have been chosen by an"]
				#[doc = " adversary, for purposes such as public-coin zero-knowledge proofs."]
				pub fn randomness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 32usize]>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Randomness",
						vec![],
						[
							191u8, 197u8, 25u8, 164u8, 104u8, 248u8, 247u8, 193u8, 244u8, 60u8,
							181u8, 195u8, 248u8, 90u8, 41u8, 199u8, 82u8, 123u8, 72u8, 126u8, 18u8,
							17u8, 128u8, 215u8, 34u8, 251u8, 227u8, 70u8, 166u8, 10u8, 104u8,
							140u8,
						],
					)
				}
				#[doc = " Pending epoch configuration change that will be applied when the next epoch is enacted."]
				pub fn pending_epoch_config_change(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"PendingEpochConfigChange",
						vec![],
						[
							4u8, 201u8, 0u8, 204u8, 47u8, 246u8, 4u8, 185u8, 163u8, 242u8, 242u8,
							152u8, 29u8, 222u8, 71u8, 127u8, 49u8, 203u8, 206u8, 180u8, 244u8,
							50u8, 80u8, 49u8, 199u8, 97u8, 3u8, 170u8, 156u8, 139u8, 106u8, 113u8,
						],
					)
				}
				#[doc = " Next epoch randomness."]
				pub fn next_randomness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<[::core::primitive::u8; 32usize]>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"NextRandomness",
						vec![],
						[
							185u8, 98u8, 45u8, 109u8, 253u8, 38u8, 238u8, 221u8, 240u8, 29u8, 38u8,
							107u8, 118u8, 117u8, 131u8, 115u8, 21u8, 255u8, 203u8, 81u8, 243u8,
							251u8, 91u8, 60u8, 163u8, 202u8, 125u8, 193u8, 173u8, 234u8, 166u8,
							92u8,
						],
					)
				}
				#[doc = " Next epoch authorities."]
				pub fn next_authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<(
							runtime_types::sp_consensus_babe::app::Public,
							::core::primitive::u64,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"NextAuthorities",
						vec![],
						[
							201u8, 193u8, 164u8, 18u8, 155u8, 253u8, 124u8, 163u8, 143u8, 73u8,
							212u8, 20u8, 241u8, 108u8, 110u8, 5u8, 171u8, 66u8, 224u8, 208u8, 10u8,
							65u8, 148u8, 164u8, 1u8, 12u8, 216u8, 83u8, 20u8, 226u8, 254u8, 183u8,
						],
					)
				}
				#[doc = " Randomness under construction."]
				#[doc = ""]
				#[doc = " We make a trade-off between storage accesses and list length."]
				#[doc = " We store the under-construction randomness in segments of up to"]
				#[doc = " `UNDER_CONSTRUCTION_SEGMENT_LENGTH`."]
				#[doc = ""]
				#[doc = " Once a segment reaches this length, we begin the next one."]
				#[doc = " We reset all segments and return to `0` at the beginning of every"]
				#[doc = " epoch."]
				pub fn segment_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"SegmentIndex",
						vec![],
						[
							128u8, 45u8, 87u8, 58u8, 174u8, 152u8, 241u8, 156u8, 56u8, 192u8, 19u8,
							45u8, 75u8, 160u8, 35u8, 253u8, 145u8, 11u8, 178u8, 81u8, 114u8, 117u8,
							112u8, 107u8, 163u8, 208u8, 240u8, 151u8, 102u8, 176u8, 246u8, 5u8,
						],
					)
				}
				#[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
				pub fn under_construction(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							[::core::primitive::u8; 32usize],
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"UnderConstruction",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
							182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
							89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
						],
					)
				}
				#[doc = " TWOX-NOTE: `SegmentIndex` is an increasing integer, so this is okay."]
				pub fn under_construction_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							[::core::primitive::u8; 32usize],
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"UnderConstruction",
						Vec::new(),
						[
							180u8, 4u8, 149u8, 245u8, 231u8, 92u8, 99u8, 170u8, 254u8, 172u8,
							182u8, 3u8, 152u8, 156u8, 132u8, 196u8, 140u8, 97u8, 7u8, 84u8, 220u8,
							89u8, 195u8, 177u8, 235u8, 51u8, 98u8, 144u8, 73u8, 238u8, 59u8, 164u8,
						],
					)
				}
				#[doc = " Temporary value (cleared at block finalization) which is `Some`"]
				#[doc = " if per-block initialization has already been called for current block."]
				pub fn initialized(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<
							runtime_types::sp_consensus_babe::digests::PreDigest,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Initialized",
						vec![],
						[
							142u8, 101u8, 250u8, 113u8, 93u8, 201u8, 157u8, 18u8, 166u8, 153u8,
							59u8, 197u8, 107u8, 247u8, 124u8, 110u8, 202u8, 67u8, 62u8, 57u8,
							186u8, 134u8, 49u8, 182u8, 149u8, 44u8, 255u8, 85u8, 87u8, 177u8,
							149u8, 121u8,
						],
					)
				}
				#[doc = " This field should always be populated during block processing unless"]
				#[doc = " secondary plain slots are enabled (which don't contain a VRF output)."]
				#[doc = ""]
				#[doc = " It is set in `on_finalize`, before it will contain the value from the last block."]
				pub fn author_vrf_randomness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<[::core::primitive::u8; 32usize]>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"AuthorVrfRandomness",
						vec![],
						[
							66u8, 235u8, 74u8, 252u8, 222u8, 135u8, 19u8, 28u8, 74u8, 191u8, 170u8,
							197u8, 207u8, 127u8, 77u8, 121u8, 138u8, 138u8, 110u8, 187u8, 34u8,
							14u8, 230u8, 43u8, 241u8, 241u8, 63u8, 163u8, 53u8, 179u8, 250u8,
							247u8,
						],
					)
				}
				#[doc = " The block numbers when the last and current epoch have started, respectively `N-1` and"]
				#[doc = " `N`."]
				#[doc = " NOTE: We track this is in order to annotate the block number when a given pool of"]
				#[doc = " entropy was fixed (i.e. it was known to chain observers). Since epochs are defined in"]
				#[doc = " slots, which may be skipped, the block numbers may not line up with the slot numbers."]
				pub fn epoch_start(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"EpochStart",
						vec![],
						[
							196u8, 39u8, 241u8, 20u8, 150u8, 180u8, 136u8, 4u8, 195u8, 205u8,
							218u8, 10u8, 130u8, 131u8, 168u8, 243u8, 207u8, 249u8, 58u8, 195u8,
							177u8, 119u8, 110u8, 243u8, 241u8, 3u8, 245u8, 56u8, 157u8, 5u8, 68u8,
							60u8,
						],
					)
				}
				#[doc = " How late the current block is compared to its parent."]
				#[doc = ""]
				#[doc = " This entry is populated as part of block execution and is cleaned up"]
				#[doc = " on block finalization. Querying this storage entry outside of block"]
				#[doc = " execution context should always yield zero."]
				pub fn lateness(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"Lateness",
						vec![],
						[
							229u8, 230u8, 224u8, 89u8, 49u8, 213u8, 198u8, 236u8, 144u8, 56u8,
							193u8, 234u8, 62u8, 242u8, 191u8, 199u8, 105u8, 131u8, 74u8, 63u8,
							75u8, 1u8, 210u8, 49u8, 3u8, 128u8, 18u8, 77u8, 219u8, 146u8, 60u8,
							88u8,
						],
					)
				}
				#[doc = " The configuration for the current epoch. Should never be `None` as it is initialized in"]
				#[doc = " genesis."]
				pub fn epoch_config(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"EpochConfig",
						vec![],
						[
							41u8, 118u8, 141u8, 244u8, 72u8, 17u8, 125u8, 203u8, 43u8, 153u8,
							203u8, 119u8, 117u8, 223u8, 123u8, 133u8, 73u8, 235u8, 130u8, 21u8,
							160u8, 167u8, 16u8, 173u8, 177u8, 35u8, 117u8, 97u8, 149u8, 49u8,
							220u8, 24u8,
						],
					)
				}
				#[doc = " The configuration for the next epoch, `None` if the config will not change"]
				#[doc = " (you can fallback to `EpochConfig` instead in that case)."]
				pub fn next_epoch_config(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_consensus_babe::BabeEpochConfiguration,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Babe",
						"NextEpochConfig",
						vec![],
						[
							111u8, 182u8, 144u8, 180u8, 92u8, 146u8, 102u8, 249u8, 196u8, 229u8,
							226u8, 30u8, 25u8, 198u8, 133u8, 9u8, 136u8, 95u8, 11u8, 151u8, 139u8,
							156u8, 105u8, 228u8, 181u8, 12u8, 175u8, 148u8, 174u8, 33u8, 233u8,
							228u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The amount of time, in slots, that each epoch should last."]
				#[doc = " NOTE: Currently it is not possible to change the epoch duration after"]
				#[doc = " the chain has started. Attempting to do so will brick block production."]
				pub fn epoch_duration(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Babe",
						"EpochDuration",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " The expected average block time at which BABE should be creating"]
				#[doc = " blocks. Since BABE is probabilistic it is not trivial to figure out"]
				#[doc = " what the expected average block time should be based on the slot"]
				#[doc = " duration and the security parameter `c` (where `1 - c` represents"]
				#[doc = " the probability of a slot being empty)."]
				pub fn expected_block_time(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Babe",
						"ExpectedBlockTime",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
				#[doc = " Max number of authorities allowed"]
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Babe",
						"MaxAuthorities",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod timestamp {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Set {
				#[codec(compact)]
				pub now: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
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
				) -> ::subxt::tx::StaticTxPayload<Set> {
					::subxt::tx::StaticTxPayload::new(
						"Timestamp",
						"set",
						Set { now },
						[
							6u8, 97u8, 172u8, 236u8, 118u8, 238u8, 228u8, 114u8, 15u8, 115u8,
							102u8, 85u8, 66u8, 151u8, 16u8, 33u8, 187u8, 17u8, 166u8, 88u8, 127u8,
							214u8, 182u8, 51u8, 168u8, 88u8, 43u8, 101u8, 185u8, 8u8, 1u8, 28u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Current time for the current block."]
				pub fn now(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Timestamp",
						"Now",
						vec![],
						[
							148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
							221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
							185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
						],
					)
				}
				#[doc = " Did the timestamp get updated in this block?"]
				pub fn did_update(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Timestamp",
						"DidUpdate",
						vec![],
						[
							70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
							13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
							27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
				#[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
				#[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
				#[doc = " double this period on default settings."]
				pub fn minimum_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Timestamp",
						"MinimumPeriod",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod indices {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Claim {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub new: ::subxt::ext::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Free {
				pub index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				pub new: ::subxt::ext::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
				pub freeze: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Freeze {
				pub index: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
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
				) -> ::subxt::tx::StaticTxPayload<Claim> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"claim",
						Claim { index },
						[
							5u8, 24u8, 11u8, 173u8, 226u8, 170u8, 0u8, 30u8, 193u8, 102u8, 214u8,
							59u8, 252u8, 32u8, 221u8, 88u8, 196u8, 189u8, 244u8, 18u8, 233u8, 37u8,
							228u8, 248u8, 76u8, 175u8, 212u8, 233u8, 238u8, 203u8, 162u8, 68u8,
						],
					)
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
					new: ::subxt::ext::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"transfer",
						Transfer { new, index },
						[
							229u8, 48u8, 45u8, 2u8, 206u8, 24u8, 60u8, 43u8, 202u8, 99u8, 80u8,
							172u8, 62u8, 134u8, 224u8, 128u8, 107u8, 219u8, 57u8, 87u8, 144u8,
							220u8, 207u8, 79u8, 7u8, 89u8, 208u8, 75u8, 158u8, 75u8, 10u8, 113u8,
						],
					)
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
				) -> ::subxt::tx::StaticTxPayload<Free> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"free",
						Free { index },
						[
							133u8, 202u8, 225u8, 127u8, 69u8, 145u8, 43u8, 13u8, 160u8, 248u8,
							215u8, 243u8, 232u8, 166u8, 74u8, 203u8, 235u8, 138u8, 255u8, 27u8,
							163u8, 71u8, 254u8, 217u8, 6u8, 208u8, 202u8, 204u8, 238u8, 70u8,
							126u8, 252u8,
						],
					)
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
					new: ::subxt::ext::sp_core::crypto::AccountId32,
					index: ::core::primitive::u32,
					freeze: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"force_transfer",
						ForceTransfer { new, index, freeze },
						[
							2u8, 134u8, 200u8, 233u8, 224u8, 80u8, 237u8, 130u8, 28u8, 159u8,
							130u8, 223u8, 124u8, 205u8, 248u8, 70u8, 246u8, 77u8, 73u8, 193u8,
							78u8, 85u8, 58u8, 29u8, 191u8, 217u8, 252u8, 178u8, 113u8, 255u8,
							151u8, 49u8,
						],
					)
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
				) -> ::subxt::tx::StaticTxPayload<Freeze> {
					::subxt::tx::StaticTxPayload::new(
						"Indices",
						"freeze",
						Freeze { index },
						[
							121u8, 45u8, 118u8, 2u8, 72u8, 48u8, 38u8, 7u8, 234u8, 204u8, 68u8,
							20u8, 76u8, 251u8, 205u8, 246u8, 149u8, 31u8, 168u8, 186u8, 208u8,
							90u8, 40u8, 47u8, 100u8, 228u8, 188u8, 33u8, 79u8, 220u8, 105u8, 209u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_indices::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A account index was assigned."]
			pub struct IndexAssigned {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexAssigned {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexAssigned";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A account index has been freed up (unassigned)."]
			pub struct IndexFreed {
				pub index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for IndexFreed {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFreed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A account index has been frozen to its current account ID."]
			pub struct IndexFrozen {
				pub index: ::core::primitive::u32,
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for IndexFrozen {
				const PALLET: &'static str = "Indices";
				const EVENT: &'static str = "IndexFrozen";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The lookup from index to account."]
				pub fn accounts(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Indices",
						"Accounts",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
						],
					)
				}
				#[doc = " The lookup from index to account."]
				pub fn accounts_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u128,
						::core::primitive::bool,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Indices",
						"Accounts",
						Vec::new(),
						[
							211u8, 169u8, 54u8, 254u8, 88u8, 57u8, 22u8, 223u8, 108u8, 27u8, 38u8,
							9u8, 202u8, 209u8, 111u8, 209u8, 144u8, 13u8, 211u8, 114u8, 239u8,
							127u8, 75u8, 166u8, 234u8, 222u8, 225u8, 35u8, 160u8, 163u8, 112u8,
							242u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The deposit needed for reserving an index."]
				pub fn deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Indices",
						"Deposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod balances {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Transfer {
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetBalance {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				#[codec(compact)]
				pub new_free: ::core::primitive::u128,
				#[codec(compact)]
				pub new_reserved: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceTransfer {
				pub source: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferKeepAlive {
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TransferAll {
				pub dest: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				pub keep_alive: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceUnreserve {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				pub amount: ::core::primitive::u128,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Transfer some liquid free balance to another account."]
				#[doc = ""]
				#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
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
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Transfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer",
						Transfer { dest, value },
						[
							111u8, 222u8, 32u8, 56u8, 171u8, 77u8, 252u8, 29u8, 194u8, 155u8,
							200u8, 192u8, 198u8, 81u8, 23u8, 115u8, 236u8, 91u8, 218u8, 114u8,
							107u8, 141u8, 138u8, 100u8, 237u8, 21u8, 58u8, 172u8, 3u8, 20u8, 216u8,
							38u8,
						],
					)
				}
				#[doc = "Set the balances of a given account."]
				#[doc = ""]
				#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
				#[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
				#[doc = "If the new free or reserved balance is below the existential deposit,"]
				#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call is `root`."]
				pub fn set_balance(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					new_free: ::core::primitive::u128,
					new_reserved: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetBalance> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"set_balance",
						SetBalance { who, new_free, new_reserved },
						[
							234u8, 215u8, 97u8, 98u8, 243u8, 199u8, 57u8, 76u8, 59u8, 161u8, 118u8,
							207u8, 34u8, 197u8, 198u8, 61u8, 231u8, 210u8, 169u8, 235u8, 150u8,
							137u8, 173u8, 49u8, 28u8, 77u8, 84u8, 149u8, 143u8, 210u8, 139u8,
							193u8,
						],
					)
				}
				#[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
				#[doc = "specified."]
				#[doc = "# <weight>"]
				#[doc = "- Same as transfer, but additional read and write because the source account is not"]
				#[doc = "  assumed to be in the overlay."]
				#[doc = "# </weight>"]
				pub fn force_transfer(
					&self,
					source: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceTransfer> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_transfer",
						ForceTransfer { source, dest, value },
						[
							79u8, 174u8, 212u8, 108u8, 184u8, 33u8, 170u8, 29u8, 232u8, 254u8,
							195u8, 218u8, 221u8, 134u8, 57u8, 99u8, 6u8, 70u8, 181u8, 227u8, 56u8,
							239u8, 243u8, 158u8, 157u8, 245u8, 36u8, 162u8, 11u8, 237u8, 147u8,
							15u8,
						],
					)
				}
				#[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
				#[doc = "origin account."]
				#[doc = ""]
				#[doc = "99% of the time you want [`transfer`] instead."]
				#[doc = ""]
				#[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
				pub fn transfer_keep_alive(
					&self,
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					value: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<TransferKeepAlive> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_keep_alive",
						TransferKeepAlive { dest, value },
						[
							112u8, 179u8, 75u8, 168u8, 193u8, 221u8, 9u8, 82u8, 190u8, 113u8,
							253u8, 13u8, 130u8, 134u8, 170u8, 216u8, 136u8, 111u8, 242u8, 220u8,
							202u8, 112u8, 47u8, 79u8, 73u8, 244u8, 226u8, 59u8, 240u8, 188u8,
							210u8, 208u8,
						],
					)
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
					dest: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					keep_alive: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<TransferAll> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"transfer_all",
						TransferAll { dest, keep_alive },
						[
							46u8, 129u8, 29u8, 177u8, 221u8, 107u8, 245u8, 69u8, 238u8, 126u8,
							145u8, 26u8, 219u8, 208u8, 14u8, 80u8, 149u8, 1u8, 214u8, 63u8, 67u8,
							201u8, 144u8, 45u8, 129u8, 145u8, 174u8, 71u8, 238u8, 113u8, 208u8,
							34u8,
						],
					)
				}
				#[doc = "Unreserve some balance from a user by force."]
				#[doc = ""]
				#[doc = "Can only be called by ROOT."]
				pub fn force_unreserve(
					&self,
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<ForceUnreserve> {
					::subxt::tx::StaticTxPayload::new(
						"Balances",
						"force_unreserve",
						ForceUnreserve { who, amount },
						[
							160u8, 146u8, 137u8, 76u8, 157u8, 187u8, 66u8, 148u8, 207u8, 76u8,
							32u8, 254u8, 82u8, 215u8, 35u8, 161u8, 213u8, 52u8, 32u8, 98u8, 102u8,
							106u8, 234u8, 123u8, 6u8, 175u8, 184u8, 188u8, 174u8, 106u8, 176u8,
							78u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_balances::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was created with some free balance."]
			pub struct Endowed {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub free_balance: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Endowed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Endowed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
			#[doc = "resulting in an outright loss."]
			pub struct DustLost {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for DustLost {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "DustLost";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Transfer succeeded."]
			pub struct Transfer {
				pub from: ::subxt::ext::sp_core::crypto::AccountId32,
				pub to: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Transfer {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Transfer";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A balance was set by root."]
			pub struct BalanceSet {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub free: ::core::primitive::u128,
				pub reserved: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for BalanceSet {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "BalanceSet";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was reserved (moved from free to reserved)."]
			pub struct Reserved {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was unreserved (moved from reserved to free)."]
			pub struct Unreserved {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some balance was moved from the reserve of the first account to the second account."]
			#[doc = "Final argument indicates the destination balance type."]
			pub struct ReserveRepatriated {
				pub from: ::subxt::ext::sp_core::crypto::AccountId32,
				pub to: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
				pub destination_status:
					runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
			}
			impl ::subxt::events::StaticEvent for ReserveRepatriated {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "ReserveRepatriated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was deposited (e.g. for transaction fees)."]
			pub struct Deposit {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Deposit {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Deposit";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
			pub struct Withdraw {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdraw {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Withdraw";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
			pub struct Slashed {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Slashed {
				const PALLET: &'static str = "Balances";
				const EVENT: &'static str = "Slashed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The total units issued in the system."]
				pub fn total_issuance(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"TotalIssuance",
						vec![],
						[
							1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
							156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
							20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}
				#[doc = " The Balances pallet example of storing the balance of an account."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " You can also store the balance of an account in the `System` pallet."]
				#[doc = ""]
				#[doc = " # Example"]
				#[doc = ""]
				#[doc = " ```nocompile"]
				#[doc = "  impl pallet_balances::Config for Runtime {"]
				#[doc = "   type AccountStore = System"]
				#[doc = "  }"]
				#[doc = " ```"]
				#[doc = ""]
				#[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
				#[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
				#[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
				#[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
				pub fn account_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Account",
						Vec::new(),
						[
							246u8, 154u8, 253u8, 71u8, 192u8, 192u8, 192u8, 236u8, 128u8, 80u8,
							40u8, 252u8, 201u8, 43u8, 3u8, 131u8, 19u8, 49u8, 141u8, 240u8, 172u8,
							217u8, 215u8, 109u8, 87u8, 135u8, 248u8, 57u8, 98u8, 185u8, 22u8, 4u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}
				#[doc = " Any liquidity locks on some account balances."]
				#[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
				pub fn locks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Locks",
						Vec::new(),
						[
							216u8, 253u8, 87u8, 73u8, 24u8, 218u8, 35u8, 0u8, 244u8, 134u8, 195u8,
							58u8, 255u8, 64u8, 153u8, 212u8, 210u8, 232u8, 4u8, 122u8, 90u8, 212u8,
							136u8, 14u8, 127u8, 232u8, 8u8, 192u8, 40u8, 233u8, 18u8, 250u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}
				#[doc = " Named reserves on some account balances."]
				pub fn reserves_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_balances::ReserveData<
								[::core::primitive::u8; 8usize],
								::core::primitive::u128,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"Reserves",
						Vec::new(),
						[
							17u8, 32u8, 191u8, 46u8, 76u8, 220u8, 101u8, 100u8, 42u8, 250u8, 128u8,
							167u8, 117u8, 44u8, 85u8, 96u8, 105u8, 216u8, 16u8, 147u8, 74u8, 55u8,
							183u8, 94u8, 160u8, 177u8, 26u8, 187u8, 71u8, 197u8, 187u8, 163u8,
						],
					)
				}
				#[doc = " Storage version of the pallet."]
				#[doc = ""]
				#[doc = " This is set to v2.0.0 for new networks."]
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::pallet_balances::Releases>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Balances",
						"StorageVersion",
						vec![],
						[
							135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
							235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
							174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
							173u8, 96u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The minimum amount required to keep an account open."]
				pub fn existential_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"ExistentialDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum number of locks that should exist on an account."]
				#[doc = " Not strictly enforced, but used for weight estimation."]
				pub fn max_locks(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"MaxLocks",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum number of named reserves that can exist on an account."]
				pub fn max_reserves(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Balances",
						"MaxReserves",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod transaction_payment {
		use super::{root_mod, runtime_types};
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
			#[doc = "has been paid by `who`."]
			pub struct TransactionFeePaid {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub actual_fee: ::core::primitive::u128,
				pub tip: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for TransactionFeePaid {
				const PALLET: &'static str = "TransactionPayment";
				const EVENT: &'static str = "TransactionFeePaid";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				pub fn next_fee_multiplier(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_arithmetic::fixed_point::FixedU128,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"NextFeeMultiplier",
						vec![],
						[
							210u8, 0u8, 206u8, 165u8, 183u8, 10u8, 206u8, 52u8, 14u8, 90u8, 218u8,
							197u8, 189u8, 125u8, 113u8, 216u8, 52u8, 161u8, 45u8, 24u8, 245u8,
							237u8, 121u8, 41u8, 106u8, 29u8, 45u8, 129u8, 250u8, 203u8, 206u8,
							180u8,
						],
					)
				}
				pub fn storage_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_transaction_payment::Releases,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"TransactionPayment",
						"StorageVersion",
						vec![],
						[
							219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
							200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
							58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
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
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u8>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"TransactionPayment",
						"OperationalFeeMultiplier",
						[
							141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
							28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
							114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
							165u8,
						],
					)
				}
			}
		}
	}
	pub mod authorship {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetUncles {
				pub new_uncles: ::std::vec::Vec<
					runtime_types::sp_runtime::generic::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Provide a set of uncles."]
				pub fn set_uncles(
					&self,
					new_uncles: ::std::vec::Vec<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				) -> ::subxt::tx::StaticTxPayload<SetUncles> {
					::subxt::tx::StaticTxPayload::new(
						"Authorship",
						"set_uncles",
						SetUncles { new_uncles },
						[
							181u8, 70u8, 222u8, 83u8, 154u8, 215u8, 200u8, 64u8, 154u8, 228u8,
							115u8, 247u8, 117u8, 89u8, 229u8, 102u8, 128u8, 189u8, 90u8, 60u8,
							223u8, 19u8, 111u8, 172u8, 5u8, 223u8, 132u8, 37u8, 235u8, 119u8, 42u8,
							64u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Uncles"]
				pub fn uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_authorship::UncleEntryItem<
								::core::primitive::u32,
								::subxt::ext::sp_core::H256,
								::subxt::ext::sp_core::crypto::AccountId32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Uncles",
						vec![],
						[
							193u8, 226u8, 196u8, 151u8, 233u8, 82u8, 60u8, 164u8, 27u8, 156u8,
							231u8, 51u8, 79u8, 134u8, 170u8, 166u8, 71u8, 120u8, 250u8, 255u8,
							52u8, 168u8, 74u8, 199u8, 122u8, 253u8, 248u8, 178u8, 39u8, 233u8,
							132u8, 67u8,
						],
					)
				}
				#[doc = " Author of current block."]
				pub fn author(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"Author",
						vec![],
						[
							149u8, 42u8, 33u8, 147u8, 190u8, 207u8, 174u8, 227u8, 190u8, 110u8,
							25u8, 131u8, 5u8, 167u8, 237u8, 188u8, 188u8, 33u8, 177u8, 126u8,
							181u8, 49u8, 126u8, 118u8, 46u8, 128u8, 154u8, 95u8, 15u8, 91u8, 103u8,
							113u8,
						],
					)
				}
				#[doc = " Whether uncles were already set in this block."]
				pub fn did_set_uncles(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Authorship",
						"DidSetUncles",
						vec![],
						[
							64u8, 3u8, 208u8, 187u8, 50u8, 45u8, 37u8, 88u8, 163u8, 226u8, 37u8,
							126u8, 232u8, 107u8, 156u8, 187u8, 29u8, 15u8, 53u8, 46u8, 28u8, 73u8,
							83u8, 123u8, 14u8, 244u8, 243u8, 43u8, 245u8, 143u8, 15u8, 115u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The number of blocks back we should accept uncles."]
				#[doc = " This means that we will deal with uncle-parents that are"]
				#[doc = " `UncleGenerations + 1` before `now`."]
				pub fn uncle_generations(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Authorship",
						"UncleGenerations",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod offences {
		use super::{root_mod, runtime_types};
		#[doc = "Events type."]
		pub type Event = runtime_types::pallet_offences::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
			#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
			#[doc = "\\[kind, timeslot\\]."]
			pub struct Offence {
				pub kind: [::core::primitive::u8; 16usize],
				pub timeslot: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for Offence {
				const PALLET: &'static str = "Offences";
				const EVENT: &'static str = "Offence";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_staking::offence::OffenceDetails<
							::subxt::ext::sp_core::crypto::AccountId32,
							(::subxt::ext::sp_core::crypto::AccountId32, ()),
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"Reports",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							168u8, 5u8, 232u8, 75u8, 28u8, 231u8, 107u8, 52u8, 186u8, 140u8, 79u8,
							242u8, 15u8, 201u8, 83u8, 78u8, 146u8, 109u8, 192u8, 106u8, 253u8,
							106u8, 91u8, 67u8, 224u8, 69u8, 176u8, 189u8, 243u8, 46u8, 12u8, 211u8,
						],
					)
				}
				#[doc = " The primary structure that holds all offence records keyed by report identifiers."]
				pub fn reports_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_staking::offence::OffenceDetails<
							::subxt::ext::sp_core::crypto::AccountId32,
							(::subxt::ext::sp_core::crypto::AccountId32, ()),
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"Reports",
						Vec::new(),
						[
							168u8, 5u8, 232u8, 75u8, 28u8, 231u8, 107u8, 52u8, 186u8, 140u8, 79u8,
							242u8, 15u8, 201u8, 83u8, 78u8, 146u8, 109u8, 192u8, 106u8, 253u8,
							106u8, 91u8, 67u8, 224u8, 69u8, 176u8, 189u8, 243u8, 46u8, 12u8, 211u8,
						],
					)
				}
				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 16usize]>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::H256>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ConcurrentReportsIndex",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							106u8, 21u8, 104u8, 5u8, 4u8, 66u8, 28u8, 70u8, 161u8, 195u8, 238u8,
							28u8, 69u8, 241u8, 221u8, 113u8, 140u8, 103u8, 181u8, 143u8, 60u8,
							177u8, 13u8, 129u8, 224u8, 149u8, 77u8, 32u8, 75u8, 74u8, 101u8, 65u8,
						],
					)
				}
				#[doc = " A vector of reports of the same kind that happened at the same time slot."]
				pub fn concurrent_reports_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::H256>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ConcurrentReportsIndex",
						Vec::new(),
						[
							106u8, 21u8, 104u8, 5u8, 4u8, 66u8, 28u8, 70u8, 161u8, 195u8, 238u8,
							28u8, 69u8, 241u8, 221u8, 113u8, 140u8, 103u8, 181u8, 143u8, 60u8,
							177u8, 13u8, 129u8, 224u8, 149u8, 77u8, 32u8, 75u8, 74u8, 101u8, 65u8,
						],
					)
				}
				#[doc = " Enumerates all reports of a kind along with the time they happened."]
				#[doc = ""]
				#[doc = " All reports are sorted by the time of offence."]
				#[doc = ""]
				#[doc = " Note that the actual type of this mapping is `Vec<u8>`, this is because values of"]
				#[doc = " different types are not supported at the moment so we are doing the manual serialization."]
				pub fn reports_by_kind_index(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 16usize]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ReportsByKindIndex",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							162u8, 66u8, 131u8, 48u8, 250u8, 237u8, 179u8, 214u8, 36u8, 137u8,
							226u8, 136u8, 120u8, 61u8, 215u8, 43u8, 164u8, 50u8, 91u8, 164u8, 20u8,
							96u8, 189u8, 100u8, 242u8, 106u8, 21u8, 136u8, 98u8, 215u8, 180u8,
							145u8,
						],
					)
				}
				#[doc = " Enumerates all reports of a kind along with the time they happened."]
				#[doc = ""]
				#[doc = " All reports are sorted by the time of offence."]
				#[doc = ""]
				#[doc = " Note that the actual type of this mapping is `Vec<u8>`, this is because values of"]
				#[doc = " different types are not supported at the moment so we are doing the manual serialization."]
				pub fn reports_by_kind_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u8>>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Offences",
						"ReportsByKindIndex",
						Vec::new(),
						[
							162u8, 66u8, 131u8, 48u8, 250u8, 237u8, 179u8, 214u8, 36u8, 137u8,
							226u8, 136u8, 120u8, 61u8, 215u8, 43u8, 164u8, 50u8, 91u8, 164u8, 20u8,
							96u8, 189u8, 100u8, 242u8, 106u8, 21u8, 136u8, 98u8, 215u8, 180u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod historical {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Historical",
						"HistoricalSessions",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							94u8, 72u8, 245u8, 151u8, 214u8, 10u8, 12u8, 113u8, 13u8, 141u8, 176u8,
							178u8, 115u8, 238u8, 224u8, 181u8, 18u8, 5u8, 71u8, 65u8, 189u8, 148u8,
							161u8, 106u8, 24u8, 211u8, 72u8, 66u8, 221u8, 244u8, 117u8, 184u8,
						],
					)
				}
				#[doc = " Mapping from historical session indices to session-data root hash and validator count."]
				pub fn historical_sessions_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Historical",
						"HistoricalSessions",
						Vec::new(),
						[
							94u8, 72u8, 245u8, 151u8, 214u8, 10u8, 12u8, 113u8, 13u8, 141u8, 176u8,
							178u8, 115u8, 238u8, 224u8, 181u8, 18u8, 5u8, 71u8, 65u8, 189u8, 148u8,
							161u8, 106u8, 24u8, 211u8, 72u8, 66u8, 221u8, 244u8, 117u8, 184u8,
						],
					)
				}
				#[doc = " The range of historical sessions we store. [first, last)"]
				pub fn stored_range(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Historical",
						"StoredRange",
						vec![],
						[
							89u8, 239u8, 197u8, 93u8, 135u8, 62u8, 142u8, 237u8, 64u8, 200u8,
							164u8, 4u8, 130u8, 233u8, 16u8, 238u8, 166u8, 206u8, 71u8, 42u8, 171u8,
							84u8, 8u8, 245u8, 183u8, 216u8, 212u8, 16u8, 190u8, 3u8, 167u8, 189u8,
						],
					)
				}
			}
		}
	}
	pub mod session {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetKeys {
				pub keys: runtime_types::rococo_runtime::SessionKeys,
				pub proof: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PurgeKeys;
			pub struct TransactionApi;
			impl TransactionApi {
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
					keys: runtime_types::rococo_runtime::SessionKeys,
					proof: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<SetKeys> {
					::subxt::tx::StaticTxPayload::new(
						"Session",
						"set_keys",
						SetKeys { keys, proof },
						[
							82u8, 210u8, 196u8, 56u8, 158u8, 100u8, 232u8, 40u8, 165u8, 92u8, 61u8,
							41u8, 8u8, 132u8, 75u8, 74u8, 60u8, 2u8, 2u8, 118u8, 120u8, 226u8,
							246u8, 94u8, 224u8, 113u8, 205u8, 206u8, 194u8, 141u8, 81u8, 107u8,
						],
					)
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
				pub fn purge_keys(&self) -> ::subxt::tx::StaticTxPayload<PurgeKeys> {
					::subxt::tx::StaticTxPayload::new(
						"Session",
						"purge_keys",
						PurgeKeys {},
						[
							200u8, 255u8, 4u8, 213u8, 188u8, 92u8, 99u8, 116u8, 163u8, 152u8, 29u8,
							35u8, 133u8, 119u8, 246u8, 44u8, 91u8, 31u8, 145u8, 23u8, 213u8, 64u8,
							71u8, 242u8, 207u8, 239u8, 231u8, 37u8, 61u8, 63u8, 190u8, 35u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_session::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "New session has happened. Note that the argument is the session index, not the"]
			#[doc = "block number as the type might suggest."]
			pub struct NewSession {
				pub session_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewSession {
				const PALLET: &'static str = "Session";
				const EVENT: &'static str = "NewSession";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current set of validators."]
				pub fn validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"Validators",
						vec![],
						[
							144u8, 235u8, 200u8, 43u8, 151u8, 57u8, 147u8, 172u8, 201u8, 202u8,
							242u8, 96u8, 57u8, 76u8, 124u8, 77u8, 42u8, 113u8, 218u8, 220u8, 230u8,
							32u8, 151u8, 152u8, 172u8, 106u8, 60u8, 227u8, 122u8, 118u8, 137u8,
							68u8,
						],
					)
				}
				#[doc = " Current index of the session."]
				pub fn current_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"CurrentIndex",
						vec![],
						[
							148u8, 179u8, 159u8, 15u8, 197u8, 95u8, 214u8, 30u8, 209u8, 251u8,
							183u8, 231u8, 91u8, 25u8, 181u8, 191u8, 143u8, 252u8, 227u8, 80u8,
							159u8, 66u8, 194u8, 67u8, 113u8, 74u8, 111u8, 91u8, 218u8, 187u8,
							130u8, 40u8,
						],
					)
				}
				#[doc = " True if the underlying economic identities or weighting behind the validators"]
				#[doc = " has changed in the queued validator set."]
				pub fn queued_changed(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedChanged",
						vec![],
						[
							105u8, 140u8, 235u8, 218u8, 96u8, 100u8, 252u8, 10u8, 58u8, 221u8,
							244u8, 251u8, 67u8, 91u8, 80u8, 202u8, 152u8, 42u8, 50u8, 113u8, 200u8,
							247u8, 59u8, 213u8, 77u8, 195u8, 1u8, 150u8, 220u8, 18u8, 245u8, 46u8,
						],
					)
				}
				#[doc = " The queued keys for the next session. When the next session begins, these keys"]
				#[doc = " will be used to determine the validator's session keys."]
				pub fn queued_keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::subxt::ext::sp_core::crypto::AccountId32,
							runtime_types::rococo_runtime::SessionKeys,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"QueuedKeys",
						vec![],
						[
							213u8, 10u8, 195u8, 77u8, 112u8, 45u8, 17u8, 136u8, 7u8, 229u8, 215u8,
							96u8, 119u8, 171u8, 255u8, 104u8, 136u8, 61u8, 63u8, 2u8, 208u8, 203u8,
							34u8, 50u8, 159u8, 176u8, 229u8, 2u8, 147u8, 232u8, 211u8, 198u8,
						],
					)
				}
				#[doc = " Indices of disabled validators."]
				#[doc = ""]
				#[doc = " The vec is always kept sorted so that we can find whether a given validator is"]
				#[doc = " disabled using binary search. It gets cleared when `on_session_ending` returns"]
				#[doc = " a new set of identities."]
				pub fn disabled_validators(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"DisabledValidators",
						vec![],
						[
							135u8, 22u8, 22u8, 97u8, 82u8, 217u8, 144u8, 141u8, 121u8, 240u8,
							189u8, 16u8, 176u8, 88u8, 177u8, 31u8, 20u8, 242u8, 73u8, 104u8, 11u8,
							110u8, 214u8, 34u8, 52u8, 217u8, 106u8, 33u8, 174u8, 174u8, 198u8,
							84u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::rococo_runtime::SessionKeys>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							126u8, 73u8, 196u8, 89u8, 165u8, 118u8, 39u8, 112u8, 201u8, 255u8,
							82u8, 77u8, 139u8, 172u8, 158u8, 193u8, 53u8, 153u8, 133u8, 238u8,
							255u8, 209u8, 222u8, 33u8, 194u8, 201u8, 94u8, 23u8, 197u8, 38u8,
							145u8, 217u8,
						],
					)
				}
				#[doc = " The next session keys for a validator."]
				pub fn next_keys_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::rococo_runtime::SessionKeys>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"NextKeys",
						Vec::new(),
						[
							126u8, 73u8, 196u8, 89u8, 165u8, 118u8, 39u8, 112u8, 201u8, 255u8,
							82u8, 77u8, 139u8, 172u8, 158u8, 193u8, 53u8, 153u8, 133u8, 238u8,
							255u8, 209u8, 222u8, 33u8, 194u8, 201u8, 94u8, 23u8, 197u8, 38u8,
							145u8, 217u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::sp_core::crypto::KeyTypeId>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
				#[doc = " The owner of a key. The key is the `KeyTypeId` + the encoded key."]
				pub fn key_owner_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Session",
						"KeyOwner",
						Vec::new(),
						[
							4u8, 91u8, 25u8, 84u8, 250u8, 201u8, 174u8, 129u8, 201u8, 58u8, 197u8,
							199u8, 137u8, 240u8, 118u8, 33u8, 99u8, 2u8, 195u8, 57u8, 53u8, 172u8,
							0u8, 148u8, 203u8, 144u8, 149u8, 64u8, 135u8, 254u8, 242u8, 215u8,
						],
					)
				}
			}
		}
	}
	pub mod grandpa {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReportEquivocation {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_finality_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReportEquivocationUnsigned {
				pub equivocation_proof: ::std::boxed::Box<
					runtime_types::sp_finality_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
				>,
				pub key_owner_proof: runtime_types::sp_session::MembershipProof,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct NoteStalled {
				pub delay: ::core::primitive::u32,
				pub best_finalized_block_number: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				pub fn report_equivocation(
					&self,
					equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocation> {
					::subxt::tx::StaticTxPayload::new(
						"Grandpa",
						"report_equivocation",
						ReportEquivocation {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							156u8, 162u8, 189u8, 89u8, 60u8, 156u8, 129u8, 176u8, 62u8, 35u8,
							214u8, 7u8, 68u8, 245u8, 130u8, 117u8, 30u8, 3u8, 73u8, 218u8, 142u8,
							82u8, 13u8, 141u8, 124u8, 19u8, 53u8, 138u8, 70u8, 4u8, 40u8, 32u8,
						],
					)
				}
				#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
				#[doc = "equivocation proof and validate the given key ownership proof"]
				#[doc = "against the extracted offender. If both are valid, the offence"]
				#[doc = "will be reported."]
				#[doc = ""]
				#[doc = "This extrinsic must be called unsigned and it is expected that only"]
				#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
				#[doc = "if the block author is defined it will be defined as the equivocation"]
				#[doc = "reporter."]
				pub fn report_equivocation_unsigned(
					&self,
					equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
						::subxt::ext::sp_core::H256,
						::core::primitive::u32,
					>,
					key_owner_proof: runtime_types::sp_session::MembershipProof,
				) -> ::subxt::tx::StaticTxPayload<ReportEquivocationUnsigned> {
					::subxt::tx::StaticTxPayload::new(
						"Grandpa",
						"report_equivocation_unsigned",
						ReportEquivocationUnsigned {
							equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
							key_owner_proof,
						},
						[
							166u8, 26u8, 217u8, 185u8, 215u8, 37u8, 174u8, 170u8, 137u8, 160u8,
							151u8, 43u8, 246u8, 86u8, 58u8, 18u8, 248u8, 73u8, 99u8, 161u8, 158u8,
							93u8, 212u8, 186u8, 224u8, 253u8, 234u8, 18u8, 151u8, 111u8, 227u8,
							249u8,
						],
					)
				}
				#[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
				#[doc = ""]
				#[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
				#[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
				#[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
				#[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
				#[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
				#[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
				#[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
				#[doc = "block of all validators of the new authority set."]
				#[doc = ""]
				#[doc = "Only callable by root."]
				pub fn note_stalled(
					&self,
					delay: ::core::primitive::u32,
					best_finalized_block_number: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<NoteStalled> {
					::subxt::tx::StaticTxPayload::new(
						"Grandpa",
						"note_stalled",
						NoteStalled { delay, best_finalized_block_number },
						[
							197u8, 236u8, 137u8, 32u8, 46u8, 200u8, 144u8, 13u8, 89u8, 181u8,
							235u8, 73u8, 167u8, 131u8, 174u8, 93u8, 42u8, 136u8, 238u8, 59u8,
							129u8, 60u8, 83u8, 100u8, 5u8, 182u8, 99u8, 250u8, 145u8, 180u8, 1u8,
							199u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_grandpa::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "New authority set has been applied."]
			pub struct NewAuthorities {
				pub authority_set: ::std::vec::Vec<(
					runtime_types::sp_finality_grandpa::app::Public,
					::core::primitive::u64,
				)>,
			}
			impl ::subxt::events::StaticEvent for NewAuthorities {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "NewAuthorities";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Current authority set has been paused."]
			pub struct Paused;
			impl ::subxt::events::StaticEvent for Paused {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Paused";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Current authority set has been resumed."]
			pub struct Resumed;
			impl ::subxt::events::StaticEvent for Resumed {
				const PALLET: &'static str = "Grandpa";
				const EVENT: &'static str = "Resumed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " State of the current authority set."]
				pub fn state(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"State",
						vec![],
						[
							211u8, 149u8, 114u8, 217u8, 206u8, 194u8, 115u8, 67u8, 12u8, 218u8,
							246u8, 213u8, 208u8, 29u8, 216u8, 104u8, 2u8, 39u8, 123u8, 172u8,
							252u8, 210u8, 52u8, 129u8, 147u8, 237u8, 244u8, 68u8, 252u8, 169u8,
							97u8, 148u8,
						],
					)
				}
				#[doc = " Pending change: (signaled at, scheduled change)."]
				pub fn pending_change(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"PendingChange",
						vec![],
						[
							178u8, 24u8, 140u8, 7u8, 8u8, 196u8, 18u8, 58u8, 3u8, 226u8, 181u8,
							47u8, 155u8, 160u8, 70u8, 12u8, 75u8, 189u8, 38u8, 255u8, 104u8, 141u8,
							64u8, 34u8, 134u8, 201u8, 102u8, 21u8, 75u8, 81u8, 218u8, 60u8,
						],
					)
				}
				#[doc = " next block number where we can force a change."]
				pub fn next_forced(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"NextForced",
						vec![],
						[
							99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8, 67u8,
							6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8, 19u8, 169u8,
							196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8, 156u8,
						],
					)
				}
				#[doc = " `true` if we are currently stalled."]
				pub fn stalled(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"Stalled",
						vec![],
						[
							219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8, 186u8,
							189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8, 231u8,
							109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8, 24u8,
							185u8,
						],
					)
				}
				#[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
				#[doc = " in the \"set\" of Grandpa validators from genesis."]
				pub fn current_set_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"CurrentSetId",
						vec![],
						[
							129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8, 20u8,
							178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8, 105u8,
							52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8, 7u8,
							108u8,
						],
					)
				}
				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"SetIdSession",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
							166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
							108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
							134u8,
						],
					)
				}
				#[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
				#[doc = " members were responsible."]
				#[doc = ""]
				#[doc = " TWOX-NOTE: `SetId` is not under user control."]
				pub fn set_id_session_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Grandpa",
						"SetIdSession",
						Vec::new(),
						[
							91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
							166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
							108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
							134u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " Max Authorities in use"]
				pub fn max_authorities(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Grandpa",
						"MaxAuthorities",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod im_online {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Heartbeat {
				pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
				pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "# <weight>"]
				#[doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"]
				#[doc = "  length of `heartbeat.network_state.external_address`"]
				#[doc = "  - `O(K)`: decoding of length `K`"]
				#[doc = "  - `O(E)`: decoding/encoding of length `E`"]
				#[doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"]
				#[doc = "  `ReceivedHeartbeats`"]
				#[doc = "- DbWrites: `ReceivedHeartbeats`"]
				#[doc = "# </weight>"]
				pub fn heartbeat(
					&self,
					heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
					signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
				) -> ::subxt::tx::StaticTxPayload<Heartbeat> {
					::subxt::tx::StaticTxPayload::new(
						"ImOnline",
						"heartbeat",
						Heartbeat { heartbeat, signature },
						[
							212u8, 23u8, 174u8, 246u8, 60u8, 220u8, 178u8, 137u8, 53u8, 146u8,
							165u8, 225u8, 179u8, 209u8, 233u8, 152u8, 129u8, 210u8, 126u8, 32u8,
							216u8, 22u8, 76u8, 196u8, 255u8, 128u8, 246u8, 161u8, 30u8, 186u8,
							249u8, 34u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_im_online::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new heartbeat was received from `AuthorityId`."]
			pub struct HeartbeatReceived {
				pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
			}
			impl ::subxt::events::StaticEvent for HeartbeatReceived {
				const PALLET: &'static str = "ImOnline";
				const EVENT: &'static str = "HeartbeatReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "At the end of the session, no offence was committed."]
			pub struct AllGood;
			impl ::subxt::events::StaticEvent for AllGood {
				const PALLET: &'static str = "ImOnline";
				const EVENT: &'static str = "AllGood";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "At the end of the session, at least one validator was found to be offline."]
			pub struct SomeOffline {
				pub offline: ::std::vec::Vec<(::subxt::ext::sp_core::crypto::AccountId32, ())>,
			}
			impl ::subxt::events::StaticEvent for SomeOffline {
				const PALLET: &'static str = "ImOnline";
				const EVENT: &'static str = "SomeOffline";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The block number after which it's ok to send heartbeats in the current"]
				#[doc = " session."]
				#[doc = ""]
				#[doc = " At the beginning of each session we set this to a value that should fall"]
				#[doc = " roughly in the middle of the session duration. The idea is to first wait for"]
				#[doc = " the validators to produce a block in the current session, so that the"]
				#[doc = " heartbeat later on will not be necessary."]
				#[doc = ""]
				#[doc = " This value will only be used as a fallback if we fail to get a proper session"]
				#[doc = " progress estimate from `NextSessionRotation`, as those estimates should be"]
				#[doc = " more accurate then the value we calculate for `HeartbeatAfter`."]
				pub fn heartbeat_after(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"HeartbeatAfter",
						vec![],
						[
							108u8, 100u8, 85u8, 198u8, 226u8, 122u8, 94u8, 225u8, 97u8, 154u8,
							135u8, 95u8, 106u8, 28u8, 185u8, 78u8, 192u8, 196u8, 35u8, 191u8, 12u8,
							19u8, 163u8, 46u8, 232u8, 235u8, 193u8, 81u8, 126u8, 204u8, 25u8,
							228u8,
						],
					)
				}
				#[doc = " The current set of keys that may issue a heartbeat."]
				pub fn keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"Keys",
						vec![],
						[
							6u8, 198u8, 221u8, 58u8, 14u8, 166u8, 245u8, 103u8, 191u8, 20u8, 69u8,
							233u8, 147u8, 245u8, 24u8, 64u8, 207u8, 180u8, 39u8, 208u8, 252u8,
							236u8, 247u8, 112u8, 187u8, 97u8, 70u8, 11u8, 248u8, 148u8, 208u8,
							106u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
				#[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
				pub fn received_heartbeats(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::traits::misc::WrapperOpaque<
							runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"ReceivedHeartbeats",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							233u8, 128u8, 140u8, 233u8, 55u8, 146u8, 172u8, 54u8, 54u8, 57u8,
							141u8, 106u8, 168u8, 59u8, 147u8, 253u8, 119u8, 48u8, 50u8, 251u8,
							242u8, 109u8, 251u8, 2u8, 136u8, 80u8, 146u8, 121u8, 180u8, 219u8,
							245u8, 37u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `SessionIndex` and `AuthIndex` to"]
				#[doc = " `WrapperOpaque<BoundedOpaqueNetworkState>`."]
				pub fn received_heartbeats_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::frame_support::traits::misc::WrapperOpaque<
							runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"ReceivedHeartbeats",
						Vec::new(),
						[
							233u8, 128u8, 140u8, 233u8, 55u8, 146u8, 172u8, 54u8, 54u8, 57u8,
							141u8, 106u8, 168u8, 59u8, 147u8, 253u8, 119u8, 48u8, 50u8, 251u8,
							242u8, 109u8, 251u8, 2u8, 136u8, 80u8, 146u8, 121u8, 180u8, 219u8,
							245u8, 37u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
				#[doc = " number of blocks authored by the given authority."]
				pub fn authored_blocks(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"AuthoredBlocks",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
						],
						[
							50u8, 4u8, 242u8, 240u8, 247u8, 184u8, 114u8, 245u8, 233u8, 170u8,
							24u8, 197u8, 18u8, 245u8, 8u8, 28u8, 33u8, 115u8, 166u8, 245u8, 221u8,
							223u8, 56u8, 144u8, 33u8, 139u8, 10u8, 227u8, 228u8, 223u8, 103u8,
							151u8,
						],
					)
				}
				#[doc = " For each session index, we keep a mapping of `ValidatorId<T>` to the"]
				#[doc = " number of blocks authored by the given authority."]
				pub fn authored_blocks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ImOnline",
						"AuthoredBlocks",
						Vec::new(),
						[
							50u8, 4u8, 242u8, 240u8, 247u8, 184u8, 114u8, 245u8, 233u8, 170u8,
							24u8, 197u8, 18u8, 245u8, 8u8, 28u8, 33u8, 115u8, 166u8, 245u8, 221u8,
							223u8, 56u8, 144u8, 33u8, 139u8, 10u8, 227u8, 228u8, 223u8, 103u8,
							151u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " A configuration for base priority of unsigned transactions."]
				#[doc = ""]
				#[doc = " This is exposed so that it can be tuned for particular runtime, when"]
				#[doc = " multiple pallets send unsigned transactions."]
				pub fn unsigned_priority(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"ImOnline",
						"UnsignedPriority",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod authority_discovery {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Keys of the current authority set."]
				pub fn keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::sp_authority_discovery::app::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AuthorityDiscovery",
						"Keys",
						vec![],
						[
							6u8, 198u8, 221u8, 58u8, 14u8, 166u8, 245u8, 103u8, 191u8, 20u8, 69u8,
							233u8, 147u8, 245u8, 24u8, 64u8, 207u8, 180u8, 39u8, 208u8, 252u8,
							236u8, 247u8, 112u8, 187u8, 97u8, 70u8, 11u8, 248u8, 148u8, 208u8,
							106u8,
						],
					)
				}
				#[doc = " Keys of the next authority set."]
				pub fn next_keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
							runtime_types::sp_authority_discovery::app::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AuthorityDiscovery",
						"NextKeys",
						vec![],
						[
							213u8, 94u8, 49u8, 159u8, 135u8, 1u8, 13u8, 150u8, 28u8, 15u8, 105u8,
							130u8, 90u8, 15u8, 130u8, 138u8, 186u8, 118u8, 10u8, 238u8, 173u8,
							229u8, 8u8, 144u8, 206u8, 121u8, 90u8, 203u8, 125u8, 106u8, 145u8,
							144u8,
						],
					)
				}
			}
		}
	}
	pub mod parachains_origin {
		use super::{root_mod, runtime_types};
	}
	pub mod configuration {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetValidationUpgradeCooldown {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetValidationUpgradeDelay {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetCodeRetentionPeriod {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxCodeSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxPovSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxHeadDataSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetParathreadCores {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetParathreadRetries {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetGroupRotationFrequency {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetChainAvailabilityPeriod {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetThreadAvailabilityPeriod {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetSchedulingLookahead {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMaxValidatorsPerCore {
				pub new: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMaxValidators {
				pub new: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetDisputePeriod {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetDisputePostConclusionAcceptancePeriod {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetDisputeMaxSpamSlots {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetDisputeConclusionByTimeOutPeriod {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetNoShowSlots {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetNDelayTranches {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetZerothDelayTrancheWidth {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetNeededApprovals {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetRelayVrfModuloSamples {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxUpwardQueueCount {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxUpwardQueueSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxDownwardMessageSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetUmpServiceTotalWeight {
				pub new: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxUpwardMessageSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMaxUpwardMessageNumPerCandidate {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpOpenRequestTtl {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpSenderDeposit {
				pub new: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpRecipientDeposit {
				pub new: ::core::primitive::u128,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpChannelMaxCapacity {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpChannelMaxTotalSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpMaxParachainInboundChannels {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpMaxParathreadInboundChannels {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpChannelMaxMessageSize {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpMaxParachainOutboundChannels {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpMaxParathreadOutboundChannels {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetHrmpMaxMessageNumPerCandidate {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetUmpMaxIndividualWeight {
				pub new: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetPvfCheckingEnabled {
				pub new: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetPvfVotingTtl {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct SetMinimumValidationUpgradeDelay {
				pub new: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetBypassConsistencyCheck {
				pub new: ::core::primitive::bool,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the validation upgrade cooldown."]
				pub fn set_validation_upgrade_cooldown(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetValidationUpgradeCooldown> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_validation_upgrade_cooldown",
						SetValidationUpgradeCooldown { new },
						[
							109u8, 185u8, 0u8, 59u8, 177u8, 198u8, 76u8, 90u8, 108u8, 190u8, 56u8,
							126u8, 147u8, 110u8, 76u8, 111u8, 38u8, 200u8, 230u8, 144u8, 42u8,
							167u8, 175u8, 220u8, 102u8, 37u8, 60u8, 10u8, 118u8, 79u8, 146u8,
							203u8,
						],
					)
				}
				#[doc = "Set the validation upgrade delay."]
				pub fn set_validation_upgrade_delay(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetValidationUpgradeDelay> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_validation_upgrade_delay",
						SetValidationUpgradeDelay { new },
						[
							18u8, 130u8, 158u8, 253u8, 160u8, 194u8, 220u8, 120u8, 9u8, 68u8,
							232u8, 176u8, 34u8, 81u8, 200u8, 236u8, 141u8, 139u8, 62u8, 110u8,
							76u8, 9u8, 218u8, 69u8, 55u8, 2u8, 233u8, 109u8, 83u8, 117u8, 141u8,
							253u8,
						],
					)
				}
				#[doc = "Set the acceptance period for an included candidate."]
				pub fn set_code_retention_period(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetCodeRetentionPeriod> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_code_retention_period",
						SetCodeRetentionPeriod { new },
						[
							221u8, 140u8, 253u8, 111u8, 64u8, 236u8, 93u8, 52u8, 214u8, 245u8,
							178u8, 30u8, 77u8, 166u8, 242u8, 252u8, 203u8, 106u8, 12u8, 195u8,
							27u8, 159u8, 96u8, 197u8, 145u8, 69u8, 241u8, 59u8, 74u8, 220u8, 62u8,
							205u8,
						],
					)
				}
				#[doc = "Set the max validation code size for incoming upgrades."]
				pub fn set_max_code_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxCodeSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_code_size",
						SetMaxCodeSize { new },
						[
							232u8, 106u8, 45u8, 195u8, 27u8, 162u8, 188u8, 213u8, 137u8, 13u8,
							123u8, 89u8, 215u8, 141u8, 231u8, 82u8, 205u8, 215u8, 73u8, 142u8,
							115u8, 109u8, 132u8, 118u8, 194u8, 211u8, 82u8, 20u8, 75u8, 55u8,
							218u8, 46u8,
						],
					)
				}
				#[doc = "Set the max POV block size for incoming upgrades."]
				pub fn set_max_pov_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxPovSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_pov_size",
						SetMaxPovSize { new },
						[
							15u8, 176u8, 13u8, 19u8, 177u8, 160u8, 211u8, 238u8, 29u8, 194u8,
							187u8, 235u8, 244u8, 65u8, 158u8, 47u8, 102u8, 221u8, 95u8, 10u8, 21u8,
							33u8, 219u8, 234u8, 82u8, 122u8, 75u8, 53u8, 14u8, 126u8, 218u8, 23u8,
						],
					)
				}
				#[doc = "Set the max head data size for paras."]
				pub fn set_max_head_data_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxHeadDataSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_head_data_size",
						SetMaxHeadDataSize { new },
						[
							219u8, 128u8, 213u8, 65u8, 190u8, 224u8, 87u8, 80u8, 172u8, 112u8,
							160u8, 229u8, 52u8, 1u8, 189u8, 125u8, 177u8, 139u8, 103u8, 39u8, 21u8,
							125u8, 62u8, 177u8, 74u8, 25u8, 41u8, 11u8, 200u8, 79u8, 139u8, 171u8,
						],
					)
				}
				#[doc = "Set the number of parathread execution cores."]
				pub fn set_parathread_cores(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetParathreadCores> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_parathread_cores",
						SetParathreadCores { new },
						[
							155u8, 102u8, 168u8, 202u8, 236u8, 87u8, 16u8, 128u8, 141u8, 99u8,
							154u8, 162u8, 216u8, 198u8, 236u8, 233u8, 104u8, 230u8, 137u8, 132u8,
							41u8, 106u8, 167u8, 81u8, 195u8, 172u8, 107u8, 28u8, 138u8, 254u8,
							180u8, 61u8,
						],
					)
				}
				#[doc = "Set the number of retries for a particular parathread."]
				pub fn set_parathread_retries(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetParathreadRetries> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_parathread_retries",
						SetParathreadRetries { new },
						[
							192u8, 81u8, 152u8, 41u8, 40u8, 3u8, 251u8, 205u8, 244u8, 133u8, 42u8,
							197u8, 21u8, 221u8, 80u8, 196u8, 222u8, 69u8, 153u8, 39u8, 161u8, 90u8,
							4u8, 38u8, 167u8, 131u8, 237u8, 42u8, 135u8, 37u8, 156u8, 108u8,
						],
					)
				}
				#[doc = "Set the parachain validator-group rotation frequency"]
				pub fn set_group_rotation_frequency(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetGroupRotationFrequency> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_group_rotation_frequency",
						SetGroupRotationFrequency { new },
						[
							205u8, 222u8, 129u8, 36u8, 136u8, 186u8, 114u8, 70u8, 214u8, 22u8,
							112u8, 65u8, 56u8, 42u8, 103u8, 93u8, 108u8, 242u8, 188u8, 229u8,
							150u8, 19u8, 12u8, 222u8, 25u8, 254u8, 48u8, 218u8, 200u8, 208u8,
							132u8, 251u8,
						],
					)
				}
				#[doc = "Set the availability period for parachains."]
				pub fn set_chain_availability_period(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetChainAvailabilityPeriod> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_chain_availability_period",
						SetChainAvailabilityPeriod { new },
						[
							171u8, 21u8, 54u8, 241u8, 19u8, 100u8, 54u8, 143u8, 97u8, 191u8, 193u8,
							96u8, 7u8, 86u8, 255u8, 109u8, 255u8, 93u8, 113u8, 28u8, 182u8, 75u8,
							120u8, 208u8, 91u8, 125u8, 156u8, 38u8, 56u8, 230u8, 24u8, 139u8,
						],
					)
				}
				#[doc = "Set the availability period for parathreads."]
				pub fn set_thread_availability_period(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetThreadAvailabilityPeriod> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_thread_availability_period",
						SetThreadAvailabilityPeriod { new },
						[
							208u8, 27u8, 246u8, 33u8, 90u8, 200u8, 75u8, 177u8, 19u8, 107u8, 236u8,
							43u8, 159u8, 156u8, 184u8, 10u8, 146u8, 71u8, 212u8, 129u8, 44u8, 19u8,
							162u8, 172u8, 162u8, 46u8, 166u8, 10u8, 67u8, 112u8, 206u8, 50u8,
						],
					)
				}
				#[doc = "Set the scheduling lookahead, in expected number of blocks at peak throughput."]
				pub fn set_scheduling_lookahead(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetSchedulingLookahead> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_scheduling_lookahead",
						SetSchedulingLookahead { new },
						[
							220u8, 74u8, 0u8, 150u8, 45u8, 29u8, 56u8, 210u8, 66u8, 12u8, 119u8,
							176u8, 103u8, 24u8, 216u8, 55u8, 211u8, 120u8, 233u8, 204u8, 167u8,
							100u8, 199u8, 157u8, 186u8, 174u8, 40u8, 218u8, 19u8, 230u8, 253u8,
							7u8,
						],
					)
				}
				#[doc = "Set the maximum number of validators to assign to any core."]
				pub fn set_max_validators_per_core(
					&self,
					new: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<SetMaxValidatorsPerCore> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_validators_per_core",
						SetMaxValidatorsPerCore { new },
						[
							227u8, 113u8, 192u8, 116u8, 114u8, 171u8, 27u8, 22u8, 84u8, 117u8,
							146u8, 152u8, 94u8, 101u8, 14u8, 52u8, 228u8, 170u8, 163u8, 82u8,
							248u8, 130u8, 32u8, 103u8, 225u8, 151u8, 145u8, 36u8, 98u8, 158u8, 6u8,
							245u8,
						],
					)
				}
				#[doc = "Set the maximum number of validators to use in parachain consensus."]
				pub fn set_max_validators(
					&self,
					new: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<SetMaxValidators> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_validators",
						SetMaxValidators { new },
						[
							143u8, 212u8, 59u8, 147u8, 4u8, 55u8, 142u8, 209u8, 237u8, 76u8, 7u8,
							178u8, 41u8, 81u8, 4u8, 203u8, 184u8, 149u8, 32u8, 1u8, 106u8, 180u8,
							121u8, 20u8, 137u8, 169u8, 144u8, 77u8, 38u8, 53u8, 243u8, 127u8,
						],
					)
				}
				#[doc = "Set the dispute period, in number of sessions to keep for disputes."]
				pub fn set_dispute_period(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetDisputePeriod> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_dispute_period",
						SetDisputePeriod { new },
						[
							36u8, 191u8, 142u8, 240u8, 48u8, 101u8, 10u8, 197u8, 117u8, 125u8,
							156u8, 189u8, 130u8, 77u8, 242u8, 130u8, 205u8, 154u8, 152u8, 47u8,
							75u8, 56u8, 63u8, 61u8, 33u8, 163u8, 151u8, 97u8, 105u8, 99u8, 55u8,
							180u8,
						],
					)
				}
				#[doc = "Set the dispute post conclusion acceptance period."]
				pub fn set_dispute_post_conclusion_acceptance_period(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetDisputePostConclusionAcceptancePeriod> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_dispute_post_conclusion_acceptance_period",
						SetDisputePostConclusionAcceptancePeriod { new },
						[
							66u8, 56u8, 45u8, 87u8, 51u8, 49u8, 91u8, 95u8, 255u8, 185u8, 54u8,
							165u8, 85u8, 142u8, 238u8, 251u8, 174u8, 81u8, 3u8, 61u8, 92u8, 97u8,
							203u8, 20u8, 107u8, 50u8, 208u8, 250u8, 208u8, 159u8, 225u8, 175u8,
						],
					)
				}
				#[doc = "Set the maximum number of dispute spam slots."]
				pub fn set_dispute_max_spam_slots(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetDisputeMaxSpamSlots> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_dispute_max_spam_slots",
						SetDisputeMaxSpamSlots { new },
						[
							177u8, 58u8, 3u8, 205u8, 145u8, 85u8, 160u8, 162u8, 13u8, 171u8, 124u8,
							54u8, 58u8, 209u8, 88u8, 131u8, 230u8, 248u8, 142u8, 18u8, 121u8,
							129u8, 196u8, 121u8, 25u8, 15u8, 252u8, 229u8, 89u8, 230u8, 14u8, 68u8,
						],
					)
				}
				#[doc = "Set the dispute conclusion by time out period."]
				pub fn set_dispute_conclusion_by_time_out_period(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetDisputeConclusionByTimeOutPeriod> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_dispute_conclusion_by_time_out_period",
						SetDisputeConclusionByTimeOutPeriod { new },
						[
							238u8, 102u8, 27u8, 169u8, 68u8, 116u8, 198u8, 64u8, 190u8, 33u8, 36u8,
							98u8, 176u8, 157u8, 123u8, 148u8, 126u8, 85u8, 32u8, 19u8, 49u8, 40u8,
							172u8, 41u8, 195u8, 182u8, 44u8, 255u8, 136u8, 204u8, 250u8, 6u8,
						],
					)
				}
				#[doc = "Set the no show slots, in number of number of consensus slots."]
				#[doc = "Must be at least 1."]
				pub fn set_no_show_slots(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetNoShowSlots> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_no_show_slots",
						SetNoShowSlots { new },
						[
							94u8, 230u8, 89u8, 131u8, 188u8, 246u8, 251u8, 34u8, 249u8, 16u8,
							134u8, 63u8, 238u8, 115u8, 19u8, 97u8, 97u8, 218u8, 238u8, 115u8,
							126u8, 140u8, 236u8, 17u8, 177u8, 192u8, 210u8, 239u8, 126u8, 107u8,
							117u8, 207u8,
						],
					)
				}
				#[doc = "Set the total number of delay tranches."]
				pub fn set_n_delay_tranches(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetNDelayTranches> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_n_delay_tranches",
						SetNDelayTranches { new },
						[
							195u8, 168u8, 178u8, 51u8, 20u8, 107u8, 227u8, 236u8, 57u8, 30u8,
							130u8, 93u8, 149u8, 2u8, 161u8, 66u8, 48u8, 37u8, 71u8, 108u8, 195u8,
							65u8, 153u8, 30u8, 181u8, 181u8, 158u8, 252u8, 120u8, 119u8, 36u8,
							146u8,
						],
					)
				}
				#[doc = "Set the zeroth delay tranche width."]
				pub fn set_zeroth_delay_tranche_width(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetZerothDelayTrancheWidth> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_zeroth_delay_tranche_width",
						SetZerothDelayTrancheWidth { new },
						[
							69u8, 56u8, 125u8, 24u8, 181u8, 62u8, 99u8, 92u8, 166u8, 107u8, 91u8,
							134u8, 230u8, 128u8, 214u8, 135u8, 245u8, 64u8, 62u8, 78u8, 96u8,
							231u8, 195u8, 29u8, 158u8, 113u8, 46u8, 96u8, 29u8, 0u8, 154u8, 80u8,
						],
					)
				}
				#[doc = "Set the number of validators needed to approve a block."]
				pub fn set_needed_approvals(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetNeededApprovals> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_needed_approvals",
						SetNeededApprovals { new },
						[
							238u8, 55u8, 134u8, 30u8, 67u8, 153u8, 150u8, 5u8, 226u8, 227u8, 185u8,
							188u8, 66u8, 60u8, 147u8, 118u8, 46u8, 174u8, 104u8, 100u8, 26u8,
							162u8, 65u8, 58u8, 162u8, 52u8, 211u8, 66u8, 242u8, 177u8, 230u8, 98u8,
						],
					)
				}
				#[doc = "Set the number of samples to do of the `RelayVRFModulo` approval assignment criterion."]
				pub fn set_relay_vrf_modulo_samples(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetRelayVrfModuloSamples> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_relay_vrf_modulo_samples",
						SetRelayVrfModuloSamples { new },
						[
							76u8, 101u8, 207u8, 184u8, 211u8, 8u8, 43u8, 4u8, 165u8, 147u8, 166u8,
							3u8, 189u8, 42u8, 125u8, 130u8, 21u8, 43u8, 189u8, 120u8, 239u8, 131u8,
							235u8, 35u8, 151u8, 15u8, 30u8, 81u8, 0u8, 2u8, 64u8, 21u8,
						],
					)
				}
				#[doc = "Sets the maximum items that can present in a upward dispatch queue at once."]
				pub fn set_max_upward_queue_count(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxUpwardQueueCount> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_upward_queue_count",
						SetMaxUpwardQueueCount { new },
						[
							116u8, 186u8, 216u8, 17u8, 150u8, 187u8, 86u8, 154u8, 92u8, 122u8,
							178u8, 167u8, 215u8, 165u8, 55u8, 86u8, 229u8, 114u8, 10u8, 149u8,
							50u8, 183u8, 165u8, 32u8, 233u8, 105u8, 82u8, 177u8, 120u8, 25u8, 44u8,
							130u8,
						],
					)
				}
				#[doc = "Sets the maximum total size of items that can present in a upward dispatch queue at once."]
				pub fn set_max_upward_queue_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxUpwardQueueSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_upward_queue_size",
						SetMaxUpwardQueueSize { new },
						[
							18u8, 60u8, 141u8, 57u8, 134u8, 96u8, 140u8, 85u8, 137u8, 9u8, 209u8,
							123u8, 10u8, 165u8, 33u8, 184u8, 34u8, 82u8, 59u8, 60u8, 30u8, 47u8,
							22u8, 163u8, 119u8, 200u8, 197u8, 192u8, 112u8, 243u8, 156u8, 12u8,
						],
					)
				}
				#[doc = "Set the critical downward message size."]
				pub fn set_max_downward_message_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxDownwardMessageSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_downward_message_size",
						SetMaxDownwardMessageSize { new },
						[
							104u8, 25u8, 229u8, 184u8, 53u8, 246u8, 206u8, 180u8, 13u8, 156u8,
							14u8, 224u8, 215u8, 115u8, 104u8, 127u8, 167u8, 189u8, 239u8, 183u8,
							68u8, 124u8, 55u8, 211u8, 186u8, 115u8, 70u8, 195u8, 61u8, 151u8, 32u8,
							218u8,
						],
					)
				}
				#[doc = "Sets the soft limit for the phase of dispatching dispatchable upward messages."]
				pub fn set_ump_service_total_weight(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SetUmpServiceTotalWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_ump_service_total_weight",
						SetUmpServiceTotalWeight { new },
						[
							253u8, 228u8, 226u8, 127u8, 202u8, 30u8, 148u8, 254u8, 133u8, 38u8,
							2u8, 83u8, 173u8, 147u8, 113u8, 224u8, 16u8, 160u8, 13u8, 238u8, 196u8,
							174u8, 104u8, 147u8, 57u8, 14u8, 213u8, 32u8, 220u8, 162u8, 89u8,
							244u8,
						],
					)
				}
				#[doc = "Sets the maximum size of an upward message that can be sent by a candidate."]
				pub fn set_max_upward_message_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxUpwardMessageSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_upward_message_size",
						SetMaxUpwardMessageSize { new },
						[
							213u8, 120u8, 21u8, 247u8, 101u8, 21u8, 164u8, 228u8, 33u8, 115u8,
							20u8, 138u8, 28u8, 174u8, 247u8, 39u8, 194u8, 113u8, 34u8, 73u8, 142u8,
							94u8, 116u8, 151u8, 113u8, 92u8, 151u8, 227u8, 116u8, 250u8, 101u8,
							179u8,
						],
					)
				}
				#[doc = "Sets the maximum number of messages that a candidate can contain."]
				pub fn set_max_upward_message_num_per_candidate(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMaxUpwardMessageNumPerCandidate> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_max_upward_message_num_per_candidate",
						SetMaxUpwardMessageNumPerCandidate { new },
						[
							54u8, 133u8, 226u8, 138u8, 184u8, 27u8, 130u8, 153u8, 130u8, 196u8,
							54u8, 79u8, 124u8, 10u8, 37u8, 139u8, 59u8, 190u8, 169u8, 87u8, 255u8,
							211u8, 38u8, 142u8, 37u8, 74u8, 144u8, 204u8, 75u8, 94u8, 154u8, 149u8,
						],
					)
				}
				#[doc = "Sets the number of sessions after which an HRMP open channel request expires."]
				pub fn set_hrmp_open_request_ttl(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpOpenRequestTtl> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_open_request_ttl",
						SetHrmpOpenRequestTtl { new },
						[
							192u8, 113u8, 113u8, 133u8, 197u8, 75u8, 88u8, 67u8, 130u8, 207u8,
							37u8, 192u8, 157u8, 159u8, 114u8, 75u8, 83u8, 180u8, 194u8, 180u8,
							96u8, 129u8, 7u8, 138u8, 110u8, 14u8, 229u8, 98u8, 71u8, 22u8, 229u8,
							247u8,
						],
					)
				}
				#[doc = "Sets the amount of funds that the sender should provide for opening an HRMP channel."]
				pub fn set_hrmp_sender_deposit(
					&self,
					new: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpSenderDeposit> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_sender_deposit",
						SetHrmpSenderDeposit { new },
						[
							49u8, 38u8, 173u8, 114u8, 66u8, 140u8, 15u8, 151u8, 193u8, 54u8, 128u8,
							108u8, 72u8, 71u8, 28u8, 65u8, 129u8, 199u8, 105u8, 61u8, 96u8, 119u8,
							16u8, 53u8, 115u8, 120u8, 152u8, 122u8, 182u8, 171u8, 233u8, 48u8,
						],
					)
				}
				#[doc = "Sets the amount of funds that the recipient should provide for accepting opening an HRMP"]
				#[doc = "channel."]
				pub fn set_hrmp_recipient_deposit(
					&self,
					new: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpRecipientDeposit> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_recipient_deposit",
						SetHrmpRecipientDeposit { new },
						[
							209u8, 212u8, 164u8, 56u8, 71u8, 215u8, 98u8, 250u8, 202u8, 150u8,
							228u8, 6u8, 166u8, 94u8, 171u8, 142u8, 10u8, 253u8, 89u8, 43u8, 6u8,
							173u8, 8u8, 235u8, 52u8, 18u8, 78u8, 129u8, 227u8, 61u8, 74u8, 83u8,
						],
					)
				}
				#[doc = "Sets the maximum number of messages allowed in an HRMP channel at once."]
				pub fn set_hrmp_channel_max_capacity(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpChannelMaxCapacity> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_channel_max_capacity",
						SetHrmpChannelMaxCapacity { new },
						[
							148u8, 109u8, 67u8, 220u8, 1u8, 115u8, 70u8, 93u8, 138u8, 190u8, 60u8,
							220u8, 80u8, 137u8, 246u8, 230u8, 115u8, 162u8, 30u8, 197u8, 11u8,
							33u8, 211u8, 224u8, 49u8, 165u8, 149u8, 155u8, 197u8, 44u8, 6u8, 167u8,
						],
					)
				}
				#[doc = "Sets the maximum total size of messages in bytes allowed in an HRMP channel at once."]
				pub fn set_hrmp_channel_max_total_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpChannelMaxTotalSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_channel_max_total_size",
						SetHrmpChannelMaxTotalSize { new },
						[
							79u8, 40u8, 207u8, 173u8, 168u8, 143u8, 130u8, 240u8, 205u8, 34u8,
							61u8, 217u8, 215u8, 106u8, 61u8, 181u8, 8u8, 21u8, 105u8, 64u8, 183u8,
							235u8, 39u8, 133u8, 70u8, 77u8, 233u8, 201u8, 222u8, 8u8, 43u8, 159u8,
						],
					)
				}
				#[doc = "Sets the maximum number of inbound HRMP channels a parachain is allowed to accept."]
				pub fn set_hrmp_max_parachain_inbound_channels(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpMaxParachainInboundChannels> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_max_parachain_inbound_channels",
						SetHrmpMaxParachainInboundChannels { new },
						[
							91u8, 215u8, 212u8, 131u8, 140u8, 185u8, 119u8, 184u8, 61u8, 121u8,
							120u8, 73u8, 202u8, 98u8, 124u8, 187u8, 171u8, 84u8, 136u8, 77u8,
							103u8, 169u8, 185u8, 8u8, 214u8, 214u8, 23u8, 195u8, 100u8, 72u8, 45u8,
							12u8,
						],
					)
				}
				#[doc = "Sets the maximum number of inbound HRMP channels a parathread is allowed to accept."]
				pub fn set_hrmp_max_parathread_inbound_channels(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpMaxParathreadInboundChannels> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_max_parathread_inbound_channels",
						SetHrmpMaxParathreadInboundChannels { new },
						[
							209u8, 66u8, 180u8, 20u8, 87u8, 242u8, 219u8, 71u8, 22u8, 145u8, 220u8,
							48u8, 44u8, 42u8, 77u8, 69u8, 255u8, 82u8, 27u8, 125u8, 231u8, 111u8,
							23u8, 32u8, 239u8, 28u8, 200u8, 255u8, 91u8, 207u8, 99u8, 107u8,
						],
					)
				}
				#[doc = "Sets the maximum size of a message that could ever be put into an HRMP channel."]
				pub fn set_hrmp_channel_max_message_size(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpChannelMaxMessageSize> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_channel_max_message_size",
						SetHrmpChannelMaxMessageSize { new },
						[
							17u8, 224u8, 230u8, 9u8, 114u8, 221u8, 138u8, 46u8, 234u8, 151u8, 27u8,
							34u8, 179u8, 67u8, 113u8, 228u8, 128u8, 212u8, 209u8, 125u8, 122u8,
							1u8, 79u8, 28u8, 10u8, 14u8, 83u8, 65u8, 253u8, 173u8, 116u8, 209u8,
						],
					)
				}
				#[doc = "Sets the maximum number of outbound HRMP channels a parachain is allowed to open."]
				pub fn set_hrmp_max_parachain_outbound_channels(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpMaxParachainOutboundChannels> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_max_parachain_outbound_channels",
						SetHrmpMaxParachainOutboundChannels { new },
						[
							26u8, 146u8, 150u8, 88u8, 236u8, 8u8, 63u8, 103u8, 71u8, 11u8, 20u8,
							210u8, 205u8, 106u8, 101u8, 112u8, 116u8, 73u8, 116u8, 136u8, 149u8,
							181u8, 207u8, 95u8, 151u8, 7u8, 98u8, 17u8, 224u8, 157u8, 117u8, 88u8,
						],
					)
				}
				#[doc = "Sets the maximum number of outbound HRMP channels a parathread is allowed to open."]
				pub fn set_hrmp_max_parathread_outbound_channels(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpMaxParathreadOutboundChannels> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_max_parathread_outbound_channels",
						SetHrmpMaxParathreadOutboundChannels { new },
						[
							31u8, 72u8, 93u8, 21u8, 180u8, 156u8, 101u8, 24u8, 145u8, 220u8, 194u8,
							93u8, 176u8, 164u8, 53u8, 123u8, 36u8, 113u8, 152u8, 13u8, 222u8, 54u8,
							175u8, 170u8, 235u8, 68u8, 236u8, 130u8, 178u8, 56u8, 140u8, 31u8,
						],
					)
				}
				#[doc = "Sets the maximum number of outbound HRMP messages can be sent by a candidate."]
				pub fn set_hrmp_max_message_num_per_candidate(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetHrmpMaxMessageNumPerCandidate> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_hrmp_max_message_num_per_candidate",
						SetHrmpMaxMessageNumPerCandidate { new },
						[
							244u8, 94u8, 225u8, 194u8, 133u8, 116u8, 202u8, 238u8, 8u8, 57u8,
							122u8, 125u8, 6u8, 131u8, 84u8, 102u8, 180u8, 67u8, 250u8, 136u8, 30u8,
							29u8, 110u8, 105u8, 219u8, 166u8, 91u8, 140u8, 44u8, 192u8, 37u8,
							185u8,
						],
					)
				}
				#[doc = "Sets the maximum amount of weight any individual upward message may consume."]
				pub fn set_ump_max_individual_weight(
					&self,
					new: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SetUmpMaxIndividualWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_ump_max_individual_weight",
						SetUmpMaxIndividualWeight { new },
						[
							122u8, 12u8, 77u8, 188u8, 26u8, 100u8, 16u8, 182u8, 66u8, 159u8, 127u8,
							111u8, 193u8, 204u8, 119u8, 102u8, 186u8, 12u8, 25u8, 193u8, 178u8,
							253u8, 85u8, 171u8, 199u8, 161u8, 167u8, 242u8, 104u8, 242u8, 149u8,
							161u8,
						],
					)
				}
				#[doc = "Enable or disable PVF pre-checking. Consult the field documentation prior executing."]
				pub fn set_pvf_checking_enabled(
					&self,
					new: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<SetPvfCheckingEnabled> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_pvf_checking_enabled",
						SetPvfCheckingEnabled { new },
						[
							123u8, 76u8, 1u8, 112u8, 174u8, 245u8, 18u8, 67u8, 13u8, 29u8, 219u8,
							197u8, 201u8, 112u8, 230u8, 191u8, 37u8, 148u8, 73u8, 125u8, 54u8,
							236u8, 3u8, 80u8, 114u8, 155u8, 244u8, 132u8, 57u8, 63u8, 158u8, 248u8,
						],
					)
				}
				#[doc = "Set the number of session changes after which a PVF pre-checking voting is rejected."]
				pub fn set_pvf_voting_ttl(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetPvfVotingTtl> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_pvf_voting_ttl",
						SetPvfVotingTtl { new },
						[
							17u8, 11u8, 98u8, 217u8, 208u8, 102u8, 238u8, 83u8, 118u8, 123u8, 20u8,
							18u8, 46u8, 212u8, 21u8, 164u8, 61u8, 104u8, 208u8, 204u8, 91u8, 210u8,
							40u8, 6u8, 201u8, 147u8, 46u8, 166u8, 219u8, 227u8, 121u8, 187u8,
						],
					)
				}
				#[doc = "Sets the minimum delay between announcing the upgrade block for a parachain until the"]
				#[doc = "upgrade taking place."]
				#[doc = ""]
				#[doc = "See the field documentation for information and constraints for the new value."]
				pub fn set_minimum_validation_upgrade_delay(
					&self,
					new: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMinimumValidationUpgradeDelay> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_minimum_validation_upgrade_delay",
						SetMinimumValidationUpgradeDelay { new },
						[
							205u8, 188u8, 75u8, 136u8, 228u8, 26u8, 112u8, 27u8, 119u8, 37u8,
							252u8, 109u8, 23u8, 145u8, 21u8, 212u8, 7u8, 28u8, 242u8, 210u8, 182u8,
							111u8, 121u8, 109u8, 50u8, 130u8, 46u8, 127u8, 122u8, 40u8, 141u8,
							242u8,
						],
					)
				}
				#[doc = "Setting this to true will disable consistency checks for the configuration setters."]
				#[doc = "Use with caution."]
				pub fn set_bypass_consistency_check(
					&self,
					new: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<SetBypassConsistencyCheck> {
					::subxt::tx::StaticTxPayload::new(
						"Configuration",
						"set_bypass_consistency_check",
						SetBypassConsistencyCheck { new },
						[
							80u8, 66u8, 200u8, 98u8, 54u8, 207u8, 64u8, 99u8, 162u8, 121u8, 26u8,
							173u8, 113u8, 224u8, 240u8, 106u8, 69u8, 191u8, 177u8, 107u8, 34u8,
							74u8, 103u8, 128u8, 252u8, 160u8, 169u8, 246u8, 125u8, 127u8, 153u8,
							129u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The active configuration for the current session."]				pub fn active_config (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
					::subxt::storage::address::StaticStorageAddress::new(
						"Configuration",
						"ActiveConfig",
						vec![],
						[
							159u8, 121u8, 140u8, 88u8, 122u8, 8u8, 91u8, 46u8, 13u8, 126u8, 128u8,
							7u8, 29u8, 95u8, 160u8, 50u8, 194u8, 59u8, 249u8, 41u8, 224u8, 158u8,
							251u8, 44u8, 146u8, 17u8, 34u8, 244u8, 18u8, 0u8, 156u8, 17u8,
						],
					)
				}
				#[doc = " Pending configuration changes."]
				#[doc = ""]
				#[doc = " This is a list of configuration changes, each with a session index at which it should"]
				#[doc = " be applied."]
				#[doc = ""]
				#[doc = " The list is sorted ascending by session index. Also, this list can only contain at most"]
				#[doc = " 2 items: for the next session and for the `scheduled_session`."]				pub fn pending_configs (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < (:: core :: primitive :: u32 , runtime_types :: polkadot_runtime_parachains :: configuration :: HostConfiguration < :: core :: primitive :: u32 > ,) > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
					::subxt::storage::address::StaticStorageAddress::new(
						"Configuration",
						"PendingConfigs",
						vec![],
						[
							143u8, 101u8, 164u8, 41u8, 30u8, 112u8, 74u8, 127u8, 88u8, 27u8, 144u8,
							27u8, 134u8, 253u8, 172u8, 17u8, 247u8, 247u8, 75u8, 186u8, 137u8,
							195u8, 91u8, 37u8, 148u8, 77u8, 29u8, 45u8, 131u8, 28u8, 208u8, 241u8,
						],
					)
				}
				#[doc = " If this is set, then the configuration setters will bypass the consistency checks. This"]
				#[doc = " is meant to be used only as the last resort."]
				pub fn bypass_consistency_check(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::bool>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Configuration",
						"BypassConsistencyCheck",
						vec![],
						[
							42u8, 191u8, 122u8, 163u8, 112u8, 2u8, 148u8, 59u8, 79u8, 219u8, 184u8,
							172u8, 246u8, 136u8, 185u8, 251u8, 189u8, 226u8, 83u8, 129u8, 162u8,
							109u8, 148u8, 75u8, 120u8, 216u8, 44u8, 28u8, 221u8, 78u8, 177u8, 94u8,
						],
					)
				}
			}
		}
	}
	pub mod paras_shared {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current session index."]
				pub fn current_session_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasShared",
						"CurrentSessionIndex",
						vec![],
						[
							83u8, 15u8, 20u8, 55u8, 103u8, 65u8, 76u8, 202u8, 69u8, 14u8, 221u8,
							93u8, 38u8, 163u8, 167u8, 83u8, 18u8, 245u8, 33u8, 175u8, 7u8, 97u8,
							67u8, 186u8, 96u8, 57u8, 147u8, 120u8, 107u8, 91u8, 147u8, 64u8,
						],
					)
				}
				#[doc = " All the validators actively participating in parachain consensus."]
				#[doc = " Indices are into the broader validator set."]
				pub fn active_validator_indices(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_primitives::v2::ValidatorIndex>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasShared",
						"ActiveValidatorIndices",
						vec![],
						[
							123u8, 26u8, 202u8, 53u8, 219u8, 42u8, 54u8, 92u8, 144u8, 74u8, 228u8,
							234u8, 129u8, 216u8, 161u8, 98u8, 199u8, 12u8, 13u8, 231u8, 23u8,
							166u8, 185u8, 209u8, 191u8, 33u8, 231u8, 252u8, 232u8, 44u8, 213u8,
							221u8,
						],
					)
				}
				#[doc = " The parachain attestation keys of the validators actively participating in parachain consensus."]
				#[doc = " This should be the same length as `ActiveValidatorIndices`."]
				pub fn active_validator_keys(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_primitives::v2::validator_app::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasShared",
						"ActiveValidatorKeys",
						vec![],
						[
							33u8, 14u8, 54u8, 86u8, 184u8, 171u8, 194u8, 35u8, 187u8, 252u8, 181u8,
							79u8, 229u8, 134u8, 50u8, 235u8, 162u8, 216u8, 108u8, 160u8, 175u8,
							172u8, 239u8, 114u8, 57u8, 238u8, 9u8, 54u8, 57u8, 196u8, 105u8, 15u8,
						],
					)
				}
			}
		}
	}
	pub mod para_inclusion {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A candidate was backed. `[candidate, head_data]`"]
			pub struct CandidateBacked(
				pub  runtime_types::polkadot_primitives::v2::CandidateReceipt<
					::subxt::ext::sp_core::H256,
				>,
				pub runtime_types::polkadot_parachain::primitives::HeadData,
				pub runtime_types::polkadot_primitives::v2::CoreIndex,
				pub runtime_types::polkadot_primitives::v2::GroupIndex,
			);
			impl ::subxt::events::StaticEvent for CandidateBacked {
				const PALLET: &'static str = "ParaInclusion";
				const EVENT: &'static str = "CandidateBacked";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A candidate was included. `[candidate, head_data]`"]
			pub struct CandidateIncluded(
				pub  runtime_types::polkadot_primitives::v2::CandidateReceipt<
					::subxt::ext::sp_core::H256,
				>,
				pub runtime_types::polkadot_parachain::primitives::HeadData,
				pub runtime_types::polkadot_primitives::v2::CoreIndex,
				pub runtime_types::polkadot_primitives::v2::GroupIndex,
			);
			impl ::subxt::events::StaticEvent for CandidateIncluded {
				const PALLET: &'static str = "ParaInclusion";
				const EVENT: &'static str = "CandidateIncluded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A candidate timed out. `[candidate, head_data]`"]
			pub struct CandidateTimedOut(
				pub  runtime_types::polkadot_primitives::v2::CandidateReceipt<
					::subxt::ext::sp_core::H256,
				>,
				pub runtime_types::polkadot_parachain::primitives::HeadData,
				pub runtime_types::polkadot_primitives::v2::CoreIndex,
			);
			impl ::subxt::events::StaticEvent for CandidateTimedOut {
				const PALLET: &'static str = "ParaInclusion";
				const EVENT: &'static str = "CandidateTimedOut";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The latest bitfield for each validator, referred to by their index in the validator set."]				pub fn availability_bitfields (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: polkadot_primitives :: v2 :: ValidatorIndex > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInclusion",
						"AvailabilityBitfields",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							149u8, 215u8, 123u8, 226u8, 73u8, 240u8, 102u8, 39u8, 243u8, 232u8,
							226u8, 116u8, 65u8, 180u8, 110u8, 4u8, 194u8, 50u8, 60u8, 193u8, 142u8,
							62u8, 20u8, 148u8, 106u8, 162u8, 96u8, 114u8, 215u8, 250u8, 111u8,
							225u8,
						],
					)
				}
				#[doc = " The latest bitfield for each validator, referred to by their index in the validator set."]				pub fn availability_bitfields_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: polkadot_runtime_parachains :: inclusion :: AvailabilityBitfieldRecord < :: core :: primitive :: u32 > > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInclusion",
						"AvailabilityBitfields",
						Vec::new(),
						[
							149u8, 215u8, 123u8, 226u8, 73u8, 240u8, 102u8, 39u8, 243u8, 232u8,
							226u8, 116u8, 65u8, 180u8, 110u8, 4u8, 194u8, 50u8, 60u8, 193u8, 142u8,
							62u8, 20u8, 148u8, 106u8, 162u8, 96u8, 114u8, 215u8, 250u8, 111u8,
							225u8,
						],
					)
				}
				#[doc = " Candidates pending availability by `ParaId`."]				pub fn pending_availability (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: polkadot_parachain :: primitives :: Id > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInclusion",
						"PendingAvailability",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							54u8, 166u8, 18u8, 56u8, 51u8, 241u8, 31u8, 165u8, 220u8, 138u8, 67u8,
							171u8, 23u8, 101u8, 109u8, 26u8, 211u8, 237u8, 81u8, 143u8, 192u8,
							214u8, 49u8, 42u8, 69u8, 30u8, 168u8, 113u8, 72u8, 12u8, 140u8, 242u8,
						],
					)
				}
				#[doc = " Candidates pending availability by `ParaId`."]				pub fn pending_availability_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: polkadot_runtime_parachains :: inclusion :: CandidatePendingAvailability < :: subxt :: ext :: sp_core :: H256 , :: core :: primitive :: u32 > > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInclusion",
						"PendingAvailability",
						Vec::new(),
						[
							54u8, 166u8, 18u8, 56u8, 51u8, 241u8, 31u8, 165u8, 220u8, 138u8, 67u8,
							171u8, 23u8, 101u8, 109u8, 26u8, 211u8, 237u8, 81u8, 143u8, 192u8,
							214u8, 49u8, 42u8, 69u8, 30u8, 168u8, 113u8, 72u8, 12u8, 140u8, 242u8,
						],
					)
				}
				#[doc = " The commitments of candidates pending availability, by `ParaId`."]
				pub fn pending_availability_commitments(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::CandidateCommitments<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInclusion",
						"PendingAvailabilityCommitments",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							146u8, 206u8, 148u8, 102u8, 55u8, 101u8, 144u8, 33u8, 197u8, 232u8,
							64u8, 205u8, 216u8, 21u8, 247u8, 170u8, 237u8, 115u8, 144u8, 43u8,
							106u8, 87u8, 82u8, 39u8, 11u8, 87u8, 149u8, 195u8, 56u8, 59u8, 54u8,
							8u8,
						],
					)
				}
				#[doc = " The commitments of candidates pending availability, by `ParaId`."]
				pub fn pending_availability_commitments_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::CandidateCommitments<
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInclusion",
						"PendingAvailabilityCommitments",
						Vec::new(),
						[
							146u8, 206u8, 148u8, 102u8, 55u8, 101u8, 144u8, 33u8, 197u8, 232u8,
							64u8, 205u8, 216u8, 21u8, 247u8, 170u8, 237u8, 115u8, 144u8, 43u8,
							106u8, 87u8, 82u8, 39u8, 11u8, 87u8, 149u8, 195u8, 56u8, 59u8, 54u8,
							8u8,
						],
					)
				}
			}
		}
	}
	pub mod para_inherent {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Enter {
				pub data: runtime_types::polkadot_primitives::v2::InherentData<
					runtime_types::sp_runtime::generic::header::Header<
						::core::primitive::u32,
						runtime_types::sp_runtime::traits::BlakeTwo256,
					>,
				>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Enter the paras inherent. This will process bitfields and backed candidates."]
				pub fn enter(
					&self,
					data: runtime_types::polkadot_primitives::v2::InherentData<
						runtime_types::sp_runtime::generic::header::Header<
							::core::primitive::u32,
							runtime_types::sp_runtime::traits::BlakeTwo256,
						>,
					>,
				) -> ::subxt::tx::StaticTxPayload<Enter> {
					::subxt::tx::StaticTxPayload::new(
						"ParaInherent",
						"enter",
						Enter { data },
						[
							92u8, 247u8, 59u8, 6u8, 2u8, 102u8, 76u8, 147u8, 46u8, 232u8, 38u8,
							191u8, 145u8, 155u8, 23u8, 39u8, 228u8, 95u8, 57u8, 249u8, 247u8, 20u8,
							9u8, 189u8, 156u8, 187u8, 207u8, 107u8, 0u8, 13u8, 228u8, 6u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Whether the paras inherent was included within this block."]
				#[doc = ""]
				#[doc = " The `Option<()>` is effectively a `bool`, but it never hits storage in the `None` variant"]
				#[doc = " due to the guarantees of FRAME's storage APIs."]
				#[doc = ""]
				#[doc = " If this is `None` at the end of the block, we panic and render the block invalid."]
				pub fn included(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<()>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInherent",
						"Included",
						vec![],
						[
							208u8, 213u8, 76u8, 64u8, 90u8, 141u8, 144u8, 52u8, 220u8, 35u8, 143u8,
							171u8, 45u8, 59u8, 9u8, 218u8, 29u8, 186u8, 139u8, 203u8, 205u8, 12u8,
							10u8, 2u8, 27u8, 167u8, 182u8, 244u8, 167u8, 220u8, 44u8, 16u8,
						],
					)
				}
				#[doc = " Scraped on chain data for extracting resolved disputes as well as backing votes."]
				pub fn on_chain_votes(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::ScrapedOnChainVotes<
							::subxt::ext::sp_core::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaInherent",
						"OnChainVotes",
						vec![],
						[
							187u8, 34u8, 219u8, 197u8, 202u8, 214u8, 140u8, 152u8, 253u8, 65u8,
							206u8, 217u8, 36u8, 40u8, 107u8, 215u8, 135u8, 115u8, 35u8, 61u8,
							180u8, 131u8, 0u8, 184u8, 193u8, 76u8, 165u8, 63u8, 106u8, 222u8,
							126u8, 113u8,
						],
					)
				}
			}
		}
	}
	pub mod para_scheduler {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " All the validator groups. One for each core. Indices are into `ActiveValidators` - not the"]
				#[doc = " broader set of Polkadot validators, but instead just the subset used for parachains during"]
				#[doc = " this session."]
				#[doc = ""]
				#[doc = " Bound: The number of cores is the sum of the numbers of parachains and parathread multiplexers."]
				#[doc = " Reasonably, 100-1000. The dominant factor is the number of validators: safe upper bound at 10k."]
				pub fn validator_groups(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							::std::vec::Vec<runtime_types::polkadot_primitives::v2::ValidatorIndex>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaScheduler",
						"ValidatorGroups",
						vec![],
						[
							175u8, 187u8, 69u8, 76u8, 211u8, 36u8, 162u8, 147u8, 83u8, 65u8, 83u8,
							44u8, 241u8, 112u8, 246u8, 14u8, 237u8, 255u8, 248u8, 58u8, 44u8,
							207u8, 159u8, 112u8, 31u8, 90u8, 15u8, 85u8, 4u8, 212u8, 215u8, 211u8,
						],
					)
				}
				#[doc = " A queue of upcoming claims and which core they should be mapped onto."]
				#[doc = ""]
				#[doc = " The number of queued claims is bounded at the `scheduling_lookahead`"]
				#[doc = " multiplied by the number of parathread multiplexer cores. Reasonably, 10 * 50 = 500."]
				pub fn parathread_queue(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::scheduler::ParathreadClaimQueue,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaScheduler",
						"ParathreadQueue",
						vec![],
						[
							79u8, 144u8, 191u8, 114u8, 235u8, 55u8, 133u8, 208u8, 73u8, 97u8, 73u8,
							148u8, 96u8, 185u8, 110u8, 95u8, 132u8, 54u8, 244u8, 86u8, 50u8, 218u8,
							121u8, 226u8, 153u8, 58u8, 232u8, 202u8, 132u8, 147u8, 168u8, 48u8,
						],
					)
				}
				#[doc = " One entry for each availability core. Entries are `None` if the core is not currently occupied. Can be"]
				#[doc = " temporarily `Some` if scheduled but not occupied."]
				#[doc = " The i'th parachain belongs to the i'th core, with the remaining cores all being"]
				#[doc = " parathread-multiplexers."]
				#[doc = ""]
				#[doc = " Bounded by the maximum of either of these two values:"]
				#[doc = "   * The number of parachains and parathread multiplexers"]
				#[doc = "   * The number of validators divided by `configuration.max_validators_per_core`."]
				pub fn availability_cores(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							::core::option::Option<
								runtime_types::polkadot_primitives::v2::CoreOccupied,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaScheduler",
						"AvailabilityCores",
						vec![],
						[
							103u8, 94u8, 52u8, 17u8, 118u8, 25u8, 254u8, 190u8, 74u8, 91u8, 64u8,
							205u8, 243u8, 113u8, 143u8, 166u8, 193u8, 110u8, 214u8, 151u8, 24u8,
							112u8, 69u8, 131u8, 235u8, 78u8, 240u8, 120u8, 240u8, 68u8, 56u8,
							215u8,
						],
					)
				}
				#[doc = " An index used to ensure that only one claim on a parathread exists in the queue or is"]
				#[doc = " currently being handled by an occupied core."]
				#[doc = ""]
				#[doc = " Bounded by the number of parathread cores and scheduling lookahead. Reasonably, 10 * 50 = 500."]
				pub fn parathread_claim_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaScheduler",
						"ParathreadClaimIndex",
						vec![],
						[
							64u8, 17u8, 173u8, 35u8, 14u8, 16u8, 149u8, 200u8, 118u8, 211u8, 130u8,
							15u8, 124u8, 112u8, 44u8, 220u8, 156u8, 132u8, 119u8, 148u8, 24u8,
							120u8, 252u8, 246u8, 204u8, 119u8, 206u8, 85u8, 44u8, 210u8, 135u8,
							83u8,
						],
					)
				}
				#[doc = " The block number where the session start occurred. Used to track how many group rotations have occurred."]
				#[doc = ""]
				#[doc = " Note that in the context of parachains modules the session change is signaled during"]
				#[doc = " the block and enacted at the end of the block (at the finalization stage, to be exact)."]
				#[doc = " Thus for all intents and purposes the effect of the session change is observed at the"]
				#[doc = " block following the session change, block number of which we save in this storage value."]
				pub fn session_start_block(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaScheduler",
						"SessionStartBlock",
						vec![],
						[
							122u8, 37u8, 150u8, 1u8, 185u8, 201u8, 168u8, 67u8, 55u8, 17u8, 101u8,
							18u8, 133u8, 212u8, 6u8, 73u8, 191u8, 204u8, 229u8, 22u8, 185u8, 120u8,
							24u8, 245u8, 121u8, 215u8, 124u8, 210u8, 49u8, 28u8, 26u8, 80u8,
						],
					)
				}
				#[doc = " Currently scheduled cores - free but up to be occupied."]
				#[doc = ""]
				#[doc = " Bounded by the number of cores: one for each parachain and parathread multiplexer."]
				#[doc = ""]
				#[doc = " The value contained here will not be valid after the end of a block. Runtime APIs should be used to determine scheduled cores/"]
				#[doc = " for the upcoming block."]
				pub fn scheduled(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_runtime_parachains::scheduler::CoreAssignment,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaScheduler",
						"Scheduled",
						vec![],
						[
							246u8, 105u8, 102u8, 107u8, 143u8, 92u8, 220u8, 69u8, 71u8, 102u8,
							212u8, 157u8, 56u8, 112u8, 42u8, 179u8, 183u8, 139u8, 128u8, 81u8,
							239u8, 84u8, 103u8, 126u8, 82u8, 247u8, 39u8, 39u8, 231u8, 218u8,
							131u8, 53u8,
						],
					)
				}
			}
		}
	}
	pub mod paras {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceSetCurrentCode {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceSetCurrentHead {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceScheduleCodeUpgrade {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				pub relay_parent_number: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceNoteNewHead {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub new_head: runtime_types::polkadot_parachain::primitives::HeadData,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceQueueAction {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddTrustedValidationCode {
				pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PokeUnusedValidationCode {
				pub validation_code_hash:
					runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct IncludePvfCheckStatement {
				pub stmt: runtime_types::polkadot_primitives::v2::PvfCheckStatement,
				pub signature: runtime_types::polkadot_primitives::v2::validator_app::Signature,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Set the storage for the parachain validation code immediately."]
				pub fn force_set_current_code(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				) -> ::subxt::tx::StaticTxPayload<ForceSetCurrentCode> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"force_set_current_code",
						ForceSetCurrentCode { para, new_code },
						[
							56u8, 59u8, 48u8, 185u8, 106u8, 99u8, 250u8, 32u8, 207u8, 2u8, 4u8,
							110u8, 165u8, 131u8, 22u8, 33u8, 248u8, 175u8, 186u8, 6u8, 118u8, 51u8,
							74u8, 239u8, 68u8, 122u8, 148u8, 242u8, 193u8, 131u8, 6u8, 135u8,
						],
					)
				}
				#[doc = "Set the storage for the current parachain head data immediately."]
				pub fn force_set_current_head(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_head: runtime_types::polkadot_parachain::primitives::HeadData,
				) -> ::subxt::tx::StaticTxPayload<ForceSetCurrentHead> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"force_set_current_head",
						ForceSetCurrentHead { para, new_head },
						[
							203u8, 70u8, 33u8, 168u8, 133u8, 64u8, 146u8, 137u8, 156u8, 104u8,
							183u8, 26u8, 74u8, 227u8, 154u8, 224u8, 75u8, 85u8, 143u8, 51u8, 60u8,
							194u8, 59u8, 94u8, 100u8, 84u8, 194u8, 100u8, 153u8, 9u8, 222u8, 63u8,
						],
					)
				}
				#[doc = "Schedule an upgrade as if it was scheduled in the given relay parent block."]
				pub fn force_schedule_code_upgrade(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
					relay_parent_number: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceScheduleCodeUpgrade> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"force_schedule_code_upgrade",
						ForceScheduleCodeUpgrade { para, new_code, relay_parent_number },
						[
							30u8, 210u8, 178u8, 31u8, 48u8, 144u8, 167u8, 117u8, 220u8, 36u8,
							175u8, 220u8, 145u8, 193u8, 20u8, 98u8, 149u8, 130u8, 66u8, 54u8, 20u8,
							204u8, 231u8, 116u8, 203u8, 179u8, 253u8, 106u8, 55u8, 58u8, 116u8,
							109u8,
						],
					)
				}
				#[doc = "Note a new block head for para within the context of the current block."]
				pub fn force_note_new_head(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					new_head: runtime_types::polkadot_parachain::primitives::HeadData,
				) -> ::subxt::tx::StaticTxPayload<ForceNoteNewHead> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"force_note_new_head",
						ForceNoteNewHead { para, new_head },
						[
							83u8, 93u8, 166u8, 142u8, 213u8, 1u8, 243u8, 73u8, 192u8, 164u8, 104u8,
							206u8, 99u8, 250u8, 31u8, 222u8, 231u8, 54u8, 12u8, 45u8, 92u8, 74u8,
							248u8, 50u8, 180u8, 86u8, 251u8, 172u8, 227u8, 88u8, 45u8, 127u8,
						],
					)
				}
				#[doc = "Put a parachain directly into the next session's action queue."]
				#[doc = "We can't queue it any sooner than this without going into the"]
				#[doc = "initializer..."]
				pub fn force_queue_action(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<ForceQueueAction> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"force_queue_action",
						ForceQueueAction { para },
						[
							195u8, 243u8, 79u8, 34u8, 111u8, 246u8, 109u8, 90u8, 251u8, 137u8,
							48u8, 23u8, 117u8, 29u8, 26u8, 200u8, 37u8, 64u8, 36u8, 254u8, 224u8,
							99u8, 165u8, 246u8, 8u8, 76u8, 250u8, 36u8, 141u8, 67u8, 185u8, 17u8,
						],
					)
				}
				#[doc = "Adds the validation code to the storage."]
				#[doc = ""]
				#[doc = "The code will not be added if it is already present. Additionally, if PVF pre-checking"]
				#[doc = "is running for that code, it will be instantly accepted."]
				#[doc = ""]
				#[doc = "Otherwise, the code will be added into the storage. Note that the code will be added"]
				#[doc = "into storage with reference count 0. This is to account the fact that there are no users"]
				#[doc = "for this code yet. The caller will have to make sure that this code eventually gets"]
				#[doc = "used by some parachain or removed from the storage to avoid storage leaks. For the latter"]
				#[doc = "prefer to use the `poke_unused_validation_code` dispatchable to raw storage manipulation."]
				#[doc = ""]
				#[doc = "This function is mainly meant to be used for upgrading parachains that do not follow"]
				#[doc = "the go-ahead signal while the PVF pre-checking feature is enabled."]
				pub fn add_trusted_validation_code(
					&self,
					validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				) -> ::subxt::tx::StaticTxPayload<AddTrustedValidationCode> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"add_trusted_validation_code",
						AddTrustedValidationCode { validation_code },
						[
							160u8, 199u8, 245u8, 178u8, 58u8, 65u8, 79u8, 199u8, 53u8, 60u8, 84u8,
							225u8, 2u8, 145u8, 154u8, 204u8, 165u8, 171u8, 173u8, 223u8, 59u8,
							196u8, 37u8, 12u8, 243u8, 158u8, 77u8, 184u8, 58u8, 64u8, 133u8, 71u8,
						],
					)
				}
				#[doc = "Remove the validation code from the storage iff the reference count is 0."]
				#[doc = ""]
				#[doc = "This is better than removing the storage directly, because it will not remove the code"]
				#[doc = "that was suddenly got used by some parachain while this dispatchable was pending"]
				#[doc = "dispatching."]
				pub fn poke_unused_validation_code(
					&self,
					validation_code_hash : runtime_types :: polkadot_parachain :: primitives :: ValidationCodeHash,
				) -> ::subxt::tx::StaticTxPayload<PokeUnusedValidationCode> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"poke_unused_validation_code",
						PokeUnusedValidationCode { validation_code_hash },
						[
							98u8, 9u8, 24u8, 180u8, 8u8, 144u8, 36u8, 28u8, 111u8, 83u8, 162u8,
							160u8, 66u8, 119u8, 177u8, 117u8, 143u8, 233u8, 241u8, 128u8, 189u8,
							118u8, 241u8, 30u8, 74u8, 171u8, 193u8, 177u8, 233u8, 12u8, 254u8,
							146u8,
						],
					)
				}
				#[doc = "Includes a statement for a PVF pre-checking vote. Potentially, finalizes the vote and"]
				#[doc = "enacts the results if that was the last vote before achieving the supermajority."]
				pub fn include_pvf_check_statement(
					&self,
					stmt: runtime_types::polkadot_primitives::v2::PvfCheckStatement,
					signature: runtime_types::polkadot_primitives::v2::validator_app::Signature,
				) -> ::subxt::tx::StaticTxPayload<IncludePvfCheckStatement> {
					::subxt::tx::StaticTxPayload::new(
						"Paras",
						"include_pvf_check_statement",
						IncludePvfCheckStatement { stmt, signature },
						[
							22u8, 136u8, 241u8, 59u8, 36u8, 249u8, 239u8, 255u8, 169u8, 117u8,
							19u8, 58u8, 214u8, 16u8, 135u8, 65u8, 13u8, 250u8, 5u8, 41u8, 144u8,
							29u8, 207u8, 73u8, 215u8, 221u8, 1u8, 253u8, 123u8, 110u8, 6u8, 196u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_parachains::paras::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Current code has been updated for a Para. `para_id`"]
			pub struct CurrentCodeUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for CurrentCodeUpdated {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "CurrentCodeUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Current head has been updated for a Para. `para_id`"]
			pub struct CurrentHeadUpdated(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for CurrentHeadUpdated {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "CurrentHeadUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A code upgrade has been scheduled for a Para. `para_id`"]
			pub struct CodeUpgradeScheduled(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for CodeUpgradeScheduled {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "CodeUpgradeScheduled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new head has been noted for a Para. `para_id`"]
			pub struct NewHeadNoted(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for NewHeadNoted {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "NewHeadNoted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A para has been queued to execute pending actions. `para_id`"]
			pub struct ActionQueued(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for ActionQueued {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "ActionQueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given para either initiated or subscribed to a PVF check for the given validation"]
			#[doc = "code. `code_hash` `para_id`"]
			pub struct PvfCheckStarted(
				pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				pub runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::events::StaticEvent for PvfCheckStarted {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "PvfCheckStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given validation code was accepted by the PVF pre-checking vote."]
			#[doc = "`code_hash` `para_id`"]
			pub struct PvfCheckAccepted(
				pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				pub runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::events::StaticEvent for PvfCheckAccepted {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "PvfCheckAccepted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given validation code was rejected by the PVF pre-checking vote."]
			#[doc = "`code_hash` `para_id`"]
			pub struct PvfCheckRejected(
				pub runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				pub runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::events::StaticEvent for PvfCheckRejected {
				const PALLET: &'static str = "Paras";
				const EVENT: &'static str = "PvfCheckRejected";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " All currently active PVF pre-checking votes."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - There are no PVF pre-checking votes that exists in list but not in the set and vice versa."]
				pub fn pvf_active_vote_map(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PvfActiveVoteMap",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							84u8, 214u8, 221u8, 221u8, 244u8, 56u8, 135u8, 87u8, 252u8, 39u8,
							188u8, 13u8, 196u8, 25u8, 214u8, 186u8, 152u8, 181u8, 190u8, 39u8,
							235u8, 211u8, 236u8, 114u8, 67u8, 85u8, 138u8, 43u8, 248u8, 134u8,
							124u8, 73u8,
						],
					)
				}
				#[doc = " All currently active PVF pre-checking votes."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - There are no PVF pre-checking votes that exists in list but not in the set and vice versa."]
				pub fn pvf_active_vote_map_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PvfActiveVoteMap",
						Vec::new(),
						[
							84u8, 214u8, 221u8, 221u8, 244u8, 56u8, 135u8, 87u8, 252u8, 39u8,
							188u8, 13u8, 196u8, 25u8, 214u8, 186u8, 152u8, 181u8, 190u8, 39u8,
							235u8, 211u8, 236u8, 114u8, 67u8, 85u8, 138u8, 43u8, 248u8, 134u8,
							124u8, 73u8,
						],
					)
				}
				#[doc = " The list of all currently active PVF votes. Auxiliary to `PvfActiveVoteMap`."]
				pub fn pvf_active_vote_list(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PvfActiveVoteList",
						vec![],
						[
							196u8, 23u8, 108u8, 162u8, 29u8, 33u8, 49u8, 219u8, 127u8, 26u8, 241u8,
							58u8, 102u8, 43u8, 156u8, 3u8, 87u8, 153u8, 195u8, 96u8, 68u8, 132u8,
							170u8, 162u8, 18u8, 156u8, 121u8, 63u8, 53u8, 91u8, 68u8, 69u8,
						],
					)
				}
				#[doc = " All parachains. Ordered ascending by `ParaId`. Parathreads are not included."]
				#[doc = ""]
				#[doc = " Consider using the [`ParachainsCache`] type of modifying."]
				pub fn parachains(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"Parachains",
						vec![],
						[
							85u8, 234u8, 218u8, 69u8, 20u8, 169u8, 235u8, 6u8, 69u8, 126u8, 28u8,
							18u8, 57u8, 93u8, 238u8, 7u8, 167u8, 221u8, 75u8, 35u8, 36u8, 4u8,
							46u8, 55u8, 234u8, 123u8, 122u8, 173u8, 13u8, 205u8, 58u8, 226u8,
						],
					)
				}
				#[doc = " The current lifecycle of a all known Para IDs."]
				pub fn para_lifecycles(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"ParaLifecycles",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							221u8, 103u8, 112u8, 222u8, 86u8, 2u8, 172u8, 187u8, 174u8, 106u8, 4u8,
							253u8, 35u8, 73u8, 18u8, 78u8, 25u8, 31u8, 124u8, 110u8, 81u8, 62u8,
							215u8, 228u8, 183u8, 132u8, 138u8, 213u8, 186u8, 209u8, 191u8, 186u8,
						],
					)
				}
				#[doc = " The current lifecycle of a all known Para IDs."]
				pub fn para_lifecycles_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"ParaLifecycles",
						Vec::new(),
						[
							221u8, 103u8, 112u8, 222u8, 86u8, 2u8, 172u8, 187u8, 174u8, 106u8, 4u8,
							253u8, 35u8, 73u8, 18u8, 78u8, 25u8, 31u8, 124u8, 110u8, 81u8, 62u8,
							215u8, 228u8, 183u8, 132u8, 138u8, 213u8, 186u8, 209u8, 191u8, 186u8,
						],
					)
				}
				#[doc = " The head-data of every registered para."]
				pub fn heads(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::HeadData,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"Heads",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							122u8, 38u8, 181u8, 121u8, 245u8, 100u8, 136u8, 233u8, 237u8, 248u8,
							127u8, 2u8, 147u8, 41u8, 202u8, 242u8, 238u8, 70u8, 55u8, 200u8, 15u8,
							106u8, 138u8, 108u8, 192u8, 61u8, 158u8, 134u8, 131u8, 142u8, 70u8,
							3u8,
						],
					)
				}
				#[doc = " The head-data of every registered para."]
				pub fn heads_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::HeadData,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"Heads",
						Vec::new(),
						[
							122u8, 38u8, 181u8, 121u8, 245u8, 100u8, 136u8, 233u8, 237u8, 248u8,
							127u8, 2u8, 147u8, 41u8, 202u8, 242u8, 238u8, 70u8, 55u8, 200u8, 15u8,
							106u8, 138u8, 108u8, 192u8, 61u8, 158u8, 134u8, 131u8, 142u8, 70u8,
							3u8,
						],
					)
				}
				#[doc = " The validation code hash of every live para."]
				#[doc = ""]
				#[doc = " Corresponding code can be retrieved with [`CodeByHash`]."]
				pub fn current_code_hash(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"CurrentCodeHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							179u8, 145u8, 45u8, 44u8, 130u8, 240u8, 50u8, 128u8, 190u8, 133u8,
							66u8, 85u8, 47u8, 141u8, 56u8, 87u8, 131u8, 99u8, 170u8, 203u8, 8u8,
							51u8, 123u8, 73u8, 206u8, 30u8, 173u8, 35u8, 157u8, 195u8, 104u8,
							236u8,
						],
					)
				}
				#[doc = " The validation code hash of every live para."]
				#[doc = ""]
				#[doc = " Corresponding code can be retrieved with [`CodeByHash`]."]
				pub fn current_code_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"CurrentCodeHash",
						Vec::new(),
						[
							179u8, 145u8, 45u8, 44u8, 130u8, 240u8, 50u8, 128u8, 190u8, 133u8,
							66u8, 85u8, 47u8, 141u8, 56u8, 87u8, 131u8, 99u8, 170u8, 203u8, 8u8,
							51u8, 123u8, 73u8, 206u8, 30u8, 173u8, 35u8, 157u8, 195u8, 104u8,
							236u8,
						],
					)
				}
				#[doc = " Actual past code hash, indicated by the para id as well as the block number at which it"]
				#[doc = " became outdated."]
				#[doc = ""]
				#[doc = " Corresponding code can be retrieved with [`CodeByHash`]."]
				pub fn past_code_hash(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
					_1: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PastCodeHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							241u8, 112u8, 128u8, 226u8, 163u8, 193u8, 77u8, 78u8, 177u8, 146u8,
							31u8, 143u8, 44u8, 140u8, 159u8, 134u8, 221u8, 129u8, 36u8, 224u8,
							46u8, 119u8, 245u8, 253u8, 55u8, 22u8, 137u8, 187u8, 71u8, 94u8, 88u8,
							124u8,
						],
					)
				}
				#[doc = " Actual past code hash, indicated by the para id as well as the block number at which it"]
				#[doc = " became outdated."]
				#[doc = ""]
				#[doc = " Corresponding code can be retrieved with [`CodeByHash`]."]
				pub fn past_code_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PastCodeHash",
						Vec::new(),
						[
							241u8, 112u8, 128u8, 226u8, 163u8, 193u8, 77u8, 78u8, 177u8, 146u8,
							31u8, 143u8, 44u8, 140u8, 159u8, 134u8, 221u8, 129u8, 36u8, 224u8,
							46u8, 119u8, 245u8, 253u8, 55u8, 22u8, 137u8, 187u8, 71u8, 94u8, 88u8,
							124u8,
						],
					)
				}
				#[doc = " Past code of parachains. The parachains themselves may not be registered anymore,"]
				#[doc = " but we also keep their code on-chain for the same amount of time as outdated code"]
				#[doc = " to keep it available for secondary checkers."]
				pub fn past_code_meta(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PastCodeMeta",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							251u8, 52u8, 126u8, 12u8, 21u8, 178u8, 151u8, 195u8, 153u8, 17u8,
							215u8, 242u8, 118u8, 192u8, 86u8, 72u8, 36u8, 97u8, 245u8, 134u8,
							155u8, 117u8, 85u8, 93u8, 225u8, 209u8, 63u8, 164u8, 168u8, 72u8,
							171u8, 228u8,
						],
					)
				}
				#[doc = " Past code of parachains. The parachains themselves may not be registered anymore,"]
				#[doc = " but we also keep their code on-chain for the same amount of time as outdated code"]
				#[doc = " to keep it available for secondary checkers."]
				pub fn past_code_meta_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
							::core::primitive::u32,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PastCodeMeta",
						Vec::new(),
						[
							251u8, 52u8, 126u8, 12u8, 21u8, 178u8, 151u8, 195u8, 153u8, 17u8,
							215u8, 242u8, 118u8, 192u8, 86u8, 72u8, 36u8, 97u8, 245u8, 134u8,
							155u8, 117u8, 85u8, 93u8, 225u8, 209u8, 63u8, 164u8, 168u8, 72u8,
							171u8, 228u8,
						],
					)
				}
				#[doc = " Which paras have past code that needs pruning and the relay-chain block at which the code was replaced."]
				#[doc = " Note that this is the actual height of the included block, not the expected height at which the"]
				#[doc = " code upgrade would be applied, although they may be equal."]
				#[doc = " This is to ensure the entire acceptance period is covered, not an offset acceptance period starting"]
				#[doc = " from the time at which the parachain perceives a code upgrade as having occurred."]
				#[doc = " Multiple entries for a single para are permitted. Ordered ascending by block number."]
				pub fn past_code_pruning(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"PastCodePruning",
						vec![],
						[
							59u8, 26u8, 175u8, 129u8, 174u8, 27u8, 239u8, 77u8, 38u8, 130u8, 37u8,
							134u8, 62u8, 28u8, 218u8, 176u8, 16u8, 137u8, 175u8, 90u8, 248u8, 44u8,
							248u8, 172u8, 231u8, 6u8, 36u8, 190u8, 109u8, 237u8, 228u8, 135u8,
						],
					)
				}
				#[doc = " The block number at which the planned code change is expected for a para."]
				#[doc = " The change will be applied after the first parablock for this ID included which executes"]
				#[doc = " in the context of a relay chain block with a number >= `expected_at`."]
				pub fn future_code_upgrades(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"FutureCodeUpgrades",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							40u8, 134u8, 185u8, 28u8, 48u8, 105u8, 152u8, 229u8, 166u8, 235u8,
							32u8, 173u8, 64u8, 63u8, 151u8, 157u8, 190u8, 214u8, 7u8, 8u8, 6u8,
							128u8, 21u8, 104u8, 175u8, 71u8, 130u8, 207u8, 158u8, 115u8, 172u8,
							149u8,
						],
					)
				}
				#[doc = " The block number at which the planned code change is expected for a para."]
				#[doc = " The change will be applied after the first parablock for this ID included which executes"]
				#[doc = " in the context of a relay chain block with a number >= `expected_at`."]
				pub fn future_code_upgrades_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"FutureCodeUpgrades",
						Vec::new(),
						[
							40u8, 134u8, 185u8, 28u8, 48u8, 105u8, 152u8, 229u8, 166u8, 235u8,
							32u8, 173u8, 64u8, 63u8, 151u8, 157u8, 190u8, 214u8, 7u8, 8u8, 6u8,
							128u8, 21u8, 104u8, 175u8, 71u8, 130u8, 207u8, 158u8, 115u8, 172u8,
							149u8,
						],
					)
				}
				#[doc = " The actual future code hash of a para."]
				#[doc = ""]
				#[doc = " Corresponding code can be retrieved with [`CodeByHash`]."]
				pub fn future_code_hash(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"FutureCodeHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							252u8, 24u8, 95u8, 200u8, 207u8, 91u8, 66u8, 103u8, 54u8, 171u8, 191u8,
							187u8, 73u8, 170u8, 132u8, 59u8, 205u8, 125u8, 68u8, 194u8, 122u8,
							124u8, 190u8, 53u8, 241u8, 225u8, 131u8, 53u8, 44u8, 145u8, 244u8,
							216u8,
						],
					)
				}
				#[doc = " The actual future code hash of a para."]
				#[doc = ""]
				#[doc = " Corresponding code can be retrieved with [`CodeByHash`]."]
				pub fn future_code_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"FutureCodeHash",
						Vec::new(),
						[
							252u8, 24u8, 95u8, 200u8, 207u8, 91u8, 66u8, 103u8, 54u8, 171u8, 191u8,
							187u8, 73u8, 170u8, 132u8, 59u8, 205u8, 125u8, 68u8, 194u8, 122u8,
							124u8, 190u8, 53u8, 241u8, 225u8, 131u8, 53u8, 44u8, 145u8, 244u8,
							216u8,
						],
					)
				}
				#[doc = " This is used by the relay-chain to communicate to a parachain a go-ahead with in the upgrade procedure."]
				#[doc = ""]
				#[doc = " This value is absent when there are no upgrades scheduled or during the time the relay chain"]
				#[doc = " performs the checks. It is set at the first relay-chain block when the corresponding parachain"]
				#[doc = " can switch its upgrade function. As soon as the parachain's block is included, the value"]
				#[doc = " gets reset to `None`."]
				#[doc = ""]
				#[doc = " NOTE that this field is used by parachains via merkle storage proofs, therefore changing"]
				#[doc = " the format will require migration of parachains."]
				pub fn upgrade_go_ahead_signal(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::UpgradeGoAhead,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpgradeGoAheadSignal",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							159u8, 47u8, 247u8, 154u8, 54u8, 20u8, 235u8, 49u8, 74u8, 41u8, 65u8,
							51u8, 52u8, 187u8, 242u8, 6u8, 84u8, 144u8, 200u8, 176u8, 222u8, 232u8,
							70u8, 50u8, 70u8, 97u8, 61u8, 249u8, 245u8, 120u8, 98u8, 183u8,
						],
					)
				}
				#[doc = " This is used by the relay-chain to communicate to a parachain a go-ahead with in the upgrade procedure."]
				#[doc = ""]
				#[doc = " This value is absent when there are no upgrades scheduled or during the time the relay chain"]
				#[doc = " performs the checks. It is set at the first relay-chain block when the corresponding parachain"]
				#[doc = " can switch its upgrade function. As soon as the parachain's block is included, the value"]
				#[doc = " gets reset to `None`."]
				#[doc = ""]
				#[doc = " NOTE that this field is used by parachains via merkle storage proofs, therefore changing"]
				#[doc = " the format will require migration of parachains."]
				pub fn upgrade_go_ahead_signal_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::UpgradeGoAhead,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpgradeGoAheadSignal",
						Vec::new(),
						[
							159u8, 47u8, 247u8, 154u8, 54u8, 20u8, 235u8, 49u8, 74u8, 41u8, 65u8,
							51u8, 52u8, 187u8, 242u8, 6u8, 84u8, 144u8, 200u8, 176u8, 222u8, 232u8,
							70u8, 50u8, 70u8, 97u8, 61u8, 249u8, 245u8, 120u8, 98u8, 183u8,
						],
					)
				}
				#[doc = " This is used by the relay-chain to communicate that there are restrictions for performing"]
				#[doc = " an upgrade for this parachain."]
				#[doc = ""]
				#[doc = " This may be a because the parachain waits for the upgrade cooldown to expire. Another"]
				#[doc = " potential use case is when we want to perform some maintenance (such as storage migration)"]
				#[doc = " we could restrict upgrades to make the process simpler."]
				#[doc = ""]
				#[doc = " NOTE that this field is used by parachains via merkle storage proofs, therefore changing"]
				#[doc = " the format will require migration of parachains."]
				pub fn upgrade_restriction_signal(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::UpgradeRestriction,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpgradeRestrictionSignal",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							86u8, 190u8, 41u8, 79u8, 66u8, 68u8, 46u8, 87u8, 212u8, 176u8, 255u8,
							134u8, 104u8, 121u8, 44u8, 143u8, 248u8, 100u8, 35u8, 157u8, 91u8,
							165u8, 118u8, 38u8, 49u8, 171u8, 158u8, 163u8, 45u8, 92u8, 44u8, 11u8,
						],
					)
				}
				#[doc = " This is used by the relay-chain to communicate that there are restrictions for performing"]
				#[doc = " an upgrade for this parachain."]
				#[doc = ""]
				#[doc = " This may be a because the parachain waits for the upgrade cooldown to expire. Another"]
				#[doc = " potential use case is when we want to perform some maintenance (such as storage migration)"]
				#[doc = " we could restrict upgrades to make the process simpler."]
				#[doc = ""]
				#[doc = " NOTE that this field is used by parachains via merkle storage proofs, therefore changing"]
				#[doc = " the format will require migration of parachains."]
				pub fn upgrade_restriction_signal_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::UpgradeRestriction,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpgradeRestrictionSignal",
						Vec::new(),
						[
							86u8, 190u8, 41u8, 79u8, 66u8, 68u8, 46u8, 87u8, 212u8, 176u8, 255u8,
							134u8, 104u8, 121u8, 44u8, 143u8, 248u8, 100u8, 35u8, 157u8, 91u8,
							165u8, 118u8, 38u8, 49u8, 171u8, 158u8, 163u8, 45u8, 92u8, 44u8, 11u8,
						],
					)
				}
				#[doc = " The list of parachains that are awaiting for their upgrade restriction to cooldown."]
				#[doc = ""]
				#[doc = " Ordered ascending by block number."]
				pub fn upgrade_cooldowns(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpgradeCooldowns",
						vec![],
						[
							205u8, 236u8, 140u8, 145u8, 241u8, 245u8, 60u8, 68u8, 23u8, 175u8,
							226u8, 174u8, 154u8, 107u8, 243u8, 197u8, 61u8, 218u8, 7u8, 24u8,
							109u8, 221u8, 178u8, 80u8, 242u8, 123u8, 33u8, 119u8, 5u8, 241u8,
							179u8, 13u8,
						],
					)
				}
				#[doc = " The list of upcoming code upgrades. Each item is a pair of which para performs a code"]
				#[doc = " upgrade and at which relay-chain block it is expected at."]
				#[doc = ""]
				#[doc = " Ordered ascending by block number."]
				pub fn upcoming_upgrades(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpcomingUpgrades",
						vec![],
						[
							165u8, 112u8, 215u8, 149u8, 125u8, 175u8, 206u8, 195u8, 214u8, 173u8,
							0u8, 144u8, 46u8, 197u8, 55u8, 204u8, 144u8, 68u8, 158u8, 156u8, 145u8,
							230u8, 173u8, 101u8, 255u8, 108u8, 52u8, 199u8, 142u8, 37u8, 55u8,
							32u8,
						],
					)
				}
				#[doc = " The actions to perform during the start of a specific session index."]
				pub fn actions_queue(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"ActionsQueue",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							209u8, 106u8, 198u8, 228u8, 148u8, 75u8, 246u8, 248u8, 12u8, 143u8,
							175u8, 56u8, 168u8, 243u8, 67u8, 166u8, 59u8, 92u8, 219u8, 184u8, 1u8,
							34u8, 241u8, 66u8, 245u8, 48u8, 174u8, 41u8, 123u8, 16u8, 178u8, 161u8,
						],
					)
				}
				#[doc = " The actions to perform during the start of a specific session index."]
				pub fn actions_queue_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"ActionsQueue",
						Vec::new(),
						[
							209u8, 106u8, 198u8, 228u8, 148u8, 75u8, 246u8, 248u8, 12u8, 143u8,
							175u8, 56u8, 168u8, 243u8, 67u8, 166u8, 59u8, 92u8, 219u8, 184u8, 1u8,
							34u8, 241u8, 66u8, 245u8, 48u8, 174u8, 41u8, 123u8, 16u8, 178u8, 161u8,
						],
					)
				}
				#[doc = " Upcoming paras instantiation arguments."]
				#[doc = ""]
				#[doc = " NOTE that after PVF pre-checking is enabled the para genesis arg will have it's code set"]
				#[doc = " to empty. Instead, the code will be saved into the storage right away via `CodeByHash`."]
				pub fn upcoming_paras_genesis(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpcomingParasGenesis",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							84u8, 41u8, 210u8, 81u8, 237u8, 249u8, 162u8, 208u8, 247u8, 223u8,
							208u8, 201u8, 54u8, 43u8, 222u8, 187u8, 8u8, 116u8, 184u8, 221u8,
							107u8, 243u8, 48u8, 168u8, 108u8, 47u8, 133u8, 236u8, 184u8, 174u8,
							130u8, 145u8,
						],
					)
				}
				#[doc = " Upcoming paras instantiation arguments."]
				#[doc = ""]
				#[doc = " NOTE that after PVF pre-checking is enabled the para genesis arg will have it's code set"]
				#[doc = " to empty. Instead, the code will be saved into the storage right away via `CodeByHash`."]
				pub fn upcoming_paras_genesis_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"UpcomingParasGenesis",
						Vec::new(),
						[
							84u8, 41u8, 210u8, 81u8, 237u8, 249u8, 162u8, 208u8, 247u8, 223u8,
							208u8, 201u8, 54u8, 43u8, 222u8, 187u8, 8u8, 116u8, 184u8, 221u8,
							107u8, 243u8, 48u8, 168u8, 108u8, 47u8, 133u8, 236u8, 184u8, 174u8,
							130u8, 145u8,
						],
					)
				}
				#[doc = " The number of reference on the validation code in [`CodeByHash`] storage."]
				pub fn code_by_hash_refs(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"CodeByHashRefs",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							24u8, 6u8, 23u8, 50u8, 105u8, 203u8, 126u8, 161u8, 0u8, 5u8, 121u8,
							165u8, 204u8, 106u8, 68u8, 91u8, 84u8, 126u8, 29u8, 239u8, 236u8,
							138u8, 32u8, 237u8, 241u8, 226u8, 190u8, 233u8, 160u8, 143u8, 88u8,
							168u8,
						],
					)
				}
				#[doc = " The number of reference on the validation code in [`CodeByHash`] storage."]
				pub fn code_by_hash_refs_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"CodeByHashRefs",
						Vec::new(),
						[
							24u8, 6u8, 23u8, 50u8, 105u8, 203u8, 126u8, 161u8, 0u8, 5u8, 121u8,
							165u8, 204u8, 106u8, 68u8, 91u8, 84u8, 126u8, 29u8, 239u8, 236u8,
							138u8, 32u8, 237u8, 241u8, 226u8, 190u8, 233u8, 160u8, 143u8, 88u8,
							168u8,
						],
					)
				}
				#[doc = " Validation code stored by its hash."]
				#[doc = ""]
				#[doc = " This storage is consistent with [`FutureCodeHash`], [`CurrentCodeHash`] and"]
				#[doc = " [`PastCodeHash`]."]
				pub fn code_by_hash(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCode,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"CodeByHash",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							58u8, 104u8, 36u8, 34u8, 226u8, 210u8, 253u8, 90u8, 23u8, 3u8, 6u8,
							202u8, 9u8, 44u8, 107u8, 108u8, 41u8, 149u8, 255u8, 173u8, 119u8,
							206u8, 201u8, 180u8, 32u8, 193u8, 44u8, 232u8, 73u8, 15u8, 210u8,
							226u8,
						],
					)
				}
				#[doc = " Validation code stored by its hash."]
				#[doc = ""]
				#[doc = " This storage is consistent with [`FutureCodeHash`], [`CurrentCodeHash`] and"]
				#[doc = " [`PastCodeHash`]."]
				pub fn code_by_hash_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::ValidationCode,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Paras",
						"CodeByHash",
						Vec::new(),
						[
							58u8, 104u8, 36u8, 34u8, 226u8, 210u8, 253u8, 90u8, 23u8, 3u8, 6u8,
							202u8, 9u8, 44u8, 107u8, 108u8, 41u8, 149u8, 255u8, 173u8, 119u8,
							206u8, 201u8, 180u8, 32u8, 193u8, 44u8, 232u8, 73u8, 15u8, 210u8,
							226u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				pub fn unsigned_priority(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Paras",
						"UnsignedPriority",
						[
							128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
							59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
							103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
							246u8,
						],
					)
				}
			}
		}
	}
	pub mod initializer {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct ForceApprove {
				pub up_to: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Issue a signal to the consensus engine to forcibly act as though all parachain"]
				#[doc = "blocks in all relay chain blocks up to and including the given number in the current"]
				#[doc = "chain are valid and should be finalized."]
				pub fn force_approve(
					&self,
					up_to: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceApprove> {
					::subxt::tx::StaticTxPayload::new(
						"Initializer",
						"force_approve",
						ForceApprove { up_to },
						[
							28u8, 117u8, 86u8, 182u8, 19u8, 127u8, 43u8, 17u8, 153u8, 80u8, 193u8,
							53u8, 120u8, 41u8, 205u8, 23u8, 252u8, 148u8, 77u8, 227u8, 138u8, 35u8,
							182u8, 122u8, 112u8, 232u8, 246u8, 69u8, 173u8, 97u8, 42u8, 103u8,
						],
					)
				}
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Whether the parachains modules have been initialized within this block."]
				#[doc = ""]
				#[doc = " Semantically a `bool`, but this guarantees it should never hit the trie,"]
				#[doc = " as this is cleared in `on_finalize` and Frame optimizes `None` values to be empty values."]
				#[doc = ""]
				#[doc = " As a `bool`, `set(false)` and `remove()` both lead to the next `get()` being false, but one of"]
				#[doc = " them writes to the trie and one does not. This confusion makes `Option<()>` more suitable for"]
				#[doc = " the semantics of this variable."]
				pub fn has_initialized(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<()>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Initializer",
						"HasInitialized",
						vec![],
						[
							251u8, 135u8, 247u8, 61u8, 139u8, 102u8, 12u8, 122u8, 227u8, 123u8,
							11u8, 232u8, 120u8, 80u8, 81u8, 48u8, 216u8, 115u8, 159u8, 131u8,
							133u8, 105u8, 200u8, 122u8, 114u8, 6u8, 109u8, 4u8, 164u8, 204u8,
							214u8, 111u8,
						],
					)
				}
				#[doc = " Buffered session changes along with the block number at which they should be applied."]
				#[doc = ""]
				#[doc = " Typically this will be empty or one element long. Apart from that this item never hits"]
				#[doc = " the storage."]
				#[doc = ""]
				#[doc = " However this is a `Vec` regardless to handle various edge cases that may occur at runtime"]
				#[doc = " upgrade boundaries or if governance intervenes."]				pub fn buffered_session_changes (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < :: std :: vec :: Vec < runtime_types :: polkadot_runtime_parachains :: initializer :: BufferedSessionChange > > , :: subxt :: storage :: address :: Yes , :: subxt :: storage :: address :: Yes , () >{
					::subxt::storage::address::StaticStorageAddress::new(
						"Initializer",
						"BufferedSessionChanges",
						vec![],
						[
							176u8, 60u8, 165u8, 138u8, 99u8, 140u8, 22u8, 169u8, 121u8, 65u8,
							203u8, 85u8, 39u8, 198u8, 91u8, 167u8, 118u8, 49u8, 129u8, 128u8,
							171u8, 232u8, 249u8, 39u8, 6u8, 101u8, 57u8, 193u8, 85u8, 143u8, 105u8,
							29u8,
						],
					)
				}
			}
		}
	}
	pub mod dmp {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			pub struct TransactionApi;
			impl TransactionApi {}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The downward messages addressed for a certain para."]
				pub fn downward_message_queues(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_core_primitives::InboundDownwardMessage<
								::core::primitive::u32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Dmp",
						"DownwardMessageQueues",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							57u8, 115u8, 112u8, 195u8, 25u8, 43u8, 104u8, 199u8, 107u8, 238u8,
							93u8, 129u8, 141u8, 167u8, 167u8, 9u8, 85u8, 34u8, 53u8, 117u8, 148u8,
							246u8, 196u8, 46u8, 96u8, 114u8, 15u8, 88u8, 94u8, 40u8, 18u8, 31u8,
						],
					)
				}
				#[doc = " The downward messages addressed for a certain para."]
				pub fn downward_message_queues_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_core_primitives::InboundDownwardMessage<
								::core::primitive::u32,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Dmp",
						"DownwardMessageQueues",
						Vec::new(),
						[
							57u8, 115u8, 112u8, 195u8, 25u8, 43u8, 104u8, 199u8, 107u8, 238u8,
							93u8, 129u8, 141u8, 167u8, 167u8, 9u8, 85u8, 34u8, 53u8, 117u8, 148u8,
							246u8, 196u8, 46u8, 96u8, 114u8, 15u8, 88u8, 94u8, 40u8, 18u8, 31u8,
						],
					)
				}
				#[doc = " A mapping that stores the downward message queue MQC head for each para."]
				#[doc = ""]
				#[doc = " Each link in this chain has a form:"]
				#[doc = " `(prev_head, B, H(M))`, where"]
				#[doc = " - `prev_head`: is the previous head hash or zero if none."]
				#[doc = " - `B`: is the relay-chain block number in which a message was appended."]
				#[doc = " - `H(M)`: is the hash of the message being appended."]
				pub fn downward_message_queue_heads(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Dmp",
						"DownwardMessageQueueHeads",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							137u8, 70u8, 108u8, 184u8, 177u8, 204u8, 17u8, 187u8, 250u8, 134u8,
							85u8, 18u8, 239u8, 185u8, 167u8, 224u8, 70u8, 18u8, 38u8, 245u8, 176u8,
							122u8, 254u8, 251u8, 204u8, 126u8, 34u8, 207u8, 163u8, 104u8, 103u8,
							38u8,
						],
					)
				}
				#[doc = " A mapping that stores the downward message queue MQC head for each para."]
				#[doc = ""]
				#[doc = " Each link in this chain has a form:"]
				#[doc = " `(prev_head, B, H(M))`, where"]
				#[doc = " - `prev_head`: is the previous head hash or zero if none."]
				#[doc = " - `B`: is the relay-chain block number in which a message was appended."]
				#[doc = " - `H(M)`: is the hash of the message being appended."]
				pub fn downward_message_queue_heads_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Dmp",
						"DownwardMessageQueueHeads",
						Vec::new(),
						[
							137u8, 70u8, 108u8, 184u8, 177u8, 204u8, 17u8, 187u8, 250u8, 134u8,
							85u8, 18u8, 239u8, 185u8, 167u8, 224u8, 70u8, 18u8, 38u8, 245u8, 176u8,
							122u8, 254u8, 251u8, 204u8, 126u8, 34u8, 207u8, 163u8, 104u8, 103u8,
							38u8,
						],
					)
				}
			}
		}
	}
	pub mod ump {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ServiceOverweight {
				pub index: ::core::primitive::u64,
				pub weight_limit: ::core::primitive::u64,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Service a single overweight upward message."]
				#[doc = ""]
				#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
				#[doc = "- `index`: The index of the overweight message to service."]
				#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
				#[doc = ""]
				#[doc = "Errors:"]
				#[doc = "- `UnknownMessageIndex`: Message of `index` is unknown."]
				#[doc = "- `WeightOverLimit`: Message execution may use greater than `weight_limit`."]
				#[doc = ""]
				#[doc = "Events:"]
				#[doc = "- `OverweightServiced`: On success."]
				pub fn service_overweight(
					&self,
					index: ::core::primitive::u64,
					weight_limit: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<ServiceOverweight> {
					::subxt::tx::StaticTxPayload::new(
						"Ump",
						"service_overweight",
						ServiceOverweight { index, weight_limit },
						[
							225u8, 41u8, 132u8, 91u8, 28u8, 116u8, 89u8, 197u8, 194u8, 131u8, 28u8,
							217u8, 102u8, 241u8, 122u8, 230u8, 242u8, 75u8, 83u8, 67u8, 104u8,
							55u8, 133u8, 129u8, 91u8, 25u8, 185u8, 131u8, 22u8, 253u8, 84u8, 221u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_parachains::ump::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Upward message is invalid XCM."]
			#[doc = "\\[ id \\]"]
			pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for InvalidFormat {
				const PALLET: &'static str = "Ump";
				const EVENT: &'static str = "InvalidFormat";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Upward message is unsupported version of XCM."]
			#[doc = "\\[ id \\]"]
			pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
			impl ::subxt::events::StaticEvent for UnsupportedVersion {
				const PALLET: &'static str = "Ump";
				const EVENT: &'static str = "UnsupportedVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Upward message executed with the given outcome."]
			#[doc = "\\[ id, outcome \\]"]
			pub struct ExecutedUpward(
				pub [::core::primitive::u8; 32usize],
				pub runtime_types::xcm::v2::traits::Outcome,
			);
			impl ::subxt::events::StaticEvent for ExecutedUpward {
				const PALLET: &'static str = "Ump";
				const EVENT: &'static str = "ExecutedUpward";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The weight limit for handling upward messages was reached."]
			#[doc = "\\[ id, remaining, required \\]"]
			pub struct WeightExhausted(
				pub [::core::primitive::u8; 32usize],
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for WeightExhausted {
				const PALLET: &'static str = "Ump";
				const EVENT: &'static str = "WeightExhausted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some upward messages have been received and will be processed."]
			#[doc = "\\[ para, count, size \\]"]
			pub struct UpwardMessagesReceived(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for UpwardMessagesReceived {
				const PALLET: &'static str = "Ump";
				const EVENT: &'static str = "UpwardMessagesReceived";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The weight budget was exceeded for an individual upward message."]
			#[doc = ""]
			#[doc = "This message can be later dispatched manually using `service_overweight` dispatchable"]
			#[doc = "using the assigned `overweight_index`."]
			#[doc = ""]
			#[doc = "\\[ para, id, overweight_index, required \\]"]
			pub struct OverweightEnqueued(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub [::core::primitive::u8; 32usize],
				pub ::core::primitive::u64,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for OverweightEnqueued {
				const PALLET: &'static str = "Ump";
				const EVENT: &'static str = "OverweightEnqueued";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Upward message from the overweight queue was executed with the given actual weight"]
			#[doc = "used."]
			#[doc = ""]
			#[doc = "\\[ overweight_index, used \\]"]
			pub struct OverweightServiced(pub ::core::primitive::u64, pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for OverweightServiced {
				const PALLET: &'static str = "Ump";
				const EVENT: &'static str = "OverweightServiced";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The messages waiting to be handled by the relay-chain originating from a certain parachain."]
				#[doc = ""]
				#[doc = " Note that some upward messages might have been already processed by the inclusion logic. E.g."]
				#[doc = " channel management messages."]
				#[doc = ""]
				#[doc = " The messages are processed in FIFO order."]
				pub fn relay_dispatch_queues(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"RelayDispatchQueues",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							237u8, 72u8, 167u8, 6u8, 67u8, 106u8, 186u8, 191u8, 160u8, 9u8, 62u8,
							102u8, 229u8, 164u8, 100u8, 24u8, 202u8, 109u8, 90u8, 24u8, 192u8,
							233u8, 26u8, 239u8, 23u8, 127u8, 77u8, 191u8, 144u8, 14u8, 3u8, 141u8,
						],
					)
				}
				#[doc = " The messages waiting to be handled by the relay-chain originating from a certain parachain."]
				#[doc = ""]
				#[doc = " Note that some upward messages might have been already processed by the inclusion logic. E.g."]
				#[doc = " channel management messages."]
				#[doc = ""]
				#[doc = " The messages are processed in FIFO order."]
				pub fn relay_dispatch_queues_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"RelayDispatchQueues",
						Vec::new(),
						[
							237u8, 72u8, 167u8, 6u8, 67u8, 106u8, 186u8, 191u8, 160u8, 9u8, 62u8,
							102u8, 229u8, 164u8, 100u8, 24u8, 202u8, 109u8, 90u8, 24u8, 192u8,
							233u8, 26u8, 239u8, 23u8, 127u8, 77u8, 191u8, 144u8, 14u8, 3u8, 141u8,
						],
					)
				}
				#[doc = " Size of the dispatch queues. Caches sizes of the queues in `RelayDispatchQueue`."]
				#[doc = ""]
				#[doc = " First item in the tuple is the count of messages and second"]
				#[doc = " is the total length (in bytes) of the message payloads."]
				#[doc = ""]
				#[doc = " Note that this is an auxiliary mapping: it's possible to tell the byte size and the number of"]
				#[doc = " messages only looking at `RelayDispatchQueues`. This mapping is separate to avoid the cost of"]
				#[doc = " loading the whole message queue if only the total size and count are required."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - The set of keys should exactly match the set of keys of `RelayDispatchQueues`."]
				pub fn relay_dispatch_queue_size(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"RelayDispatchQueueSize",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							243u8, 120u8, 70u8, 2u8, 208u8, 105u8, 180u8, 25u8, 86u8, 219u8, 151u8,
							227u8, 233u8, 53u8, 151u8, 29u8, 231u8, 40u8, 84u8, 163u8, 71u8, 254u8,
							19u8, 47u8, 174u8, 63u8, 200u8, 173u8, 86u8, 199u8, 207u8, 138u8,
						],
					)
				}
				#[doc = " Size of the dispatch queues. Caches sizes of the queues in `RelayDispatchQueue`."]
				#[doc = ""]
				#[doc = " First item in the tuple is the count of messages and second"]
				#[doc = " is the total length (in bytes) of the message payloads."]
				#[doc = ""]
				#[doc = " Note that this is an auxiliary mapping: it's possible to tell the byte size and the number of"]
				#[doc = " messages only looking at `RelayDispatchQueues`. This mapping is separate to avoid the cost of"]
				#[doc = " loading the whole message queue if only the total size and count are required."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - The set of keys should exactly match the set of keys of `RelayDispatchQueues`."]
				pub fn relay_dispatch_queue_size_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"RelayDispatchQueueSize",
						Vec::new(),
						[
							243u8, 120u8, 70u8, 2u8, 208u8, 105u8, 180u8, 25u8, 86u8, 219u8, 151u8,
							227u8, 233u8, 53u8, 151u8, 29u8, 231u8, 40u8, 84u8, 163u8, 71u8, 254u8,
							19u8, 47u8, 174u8, 63u8, 200u8, 173u8, 86u8, 199u8, 207u8, 138u8,
						],
					)
				}
				#[doc = " The ordered list of `ParaId`s that have a `RelayDispatchQueue` entry."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - The set of items from this vector should be exactly the set of the keys in"]
				#[doc = "   `RelayDispatchQueues` and `RelayDispatchQueueSize`."]
				pub fn needs_dispatch(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"NeedsDispatch",
						vec![],
						[
							176u8, 94u8, 152u8, 112u8, 46u8, 124u8, 89u8, 29u8, 92u8, 104u8, 192u8,
							58u8, 59u8, 186u8, 81u8, 150u8, 157u8, 61u8, 235u8, 182u8, 222u8,
							127u8, 109u8, 11u8, 104u8, 175u8, 141u8, 219u8, 15u8, 152u8, 255u8,
							40u8,
						],
					)
				}
				#[doc = " This is the para that gets will get dispatched first during the next upward dispatchable queue"]
				#[doc = " execution round."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - If `Some(para)`, then `para` must be present in `NeedsDispatch`."]
				pub fn next_dispatch_round_start_with(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"NextDispatchRoundStartWith",
						vec![],
						[
							157u8, 221u8, 6u8, 175u8, 61u8, 99u8, 250u8, 30u8, 177u8, 53u8, 37u8,
							191u8, 138u8, 65u8, 251u8, 216u8, 37u8, 84u8, 246u8, 76u8, 8u8, 29u8,
							18u8, 253u8, 143u8, 75u8, 129u8, 141u8, 48u8, 178u8, 135u8, 197u8,
						],
					)
				}
				#[doc = " The messages that exceeded max individual message weight budget."]
				#[doc = ""]
				#[doc = " These messages stay there until manually dispatched."]
				pub fn overweight(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::polkadot_parachain::primitives::Id,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"Overweight",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							49u8, 4u8, 221u8, 218u8, 249u8, 183u8, 49u8, 198u8, 48u8, 42u8, 110u8,
							67u8, 47u8, 50u8, 181u8, 141u8, 184u8, 47u8, 114u8, 3u8, 232u8, 132u8,
							32u8, 201u8, 13u8, 213u8, 175u8, 236u8, 111u8, 87u8, 146u8, 212u8,
						],
					)
				}
				#[doc = " The messages that exceeded max individual message weight budget."]
				#[doc = ""]
				#[doc = " These messages stay there until manually dispatched."]
				pub fn overweight_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::polkadot_parachain::primitives::Id,
						::std::vec::Vec<::core::primitive::u8>,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"Overweight",
						Vec::new(),
						[
							49u8, 4u8, 221u8, 218u8, 249u8, 183u8, 49u8, 198u8, 48u8, 42u8, 110u8,
							67u8, 47u8, 50u8, 181u8, 141u8, 184u8, 47u8, 114u8, 3u8, 232u8, 132u8,
							32u8, 201u8, 13u8, 213u8, 175u8, 236u8, 111u8, 87u8, 146u8, 212u8,
						],
					)
				}
				#[doc = " The number of overweight messages ever recorded in `Overweight` (and thus the lowest free"]
				#[doc = " index)."]
				pub fn overweight_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Ump",
						"OverweightCount",
						vec![],
						[
							102u8, 180u8, 196u8, 148u8, 115u8, 62u8, 46u8, 238u8, 97u8, 116u8,
							117u8, 42u8, 14u8, 5u8, 72u8, 237u8, 230u8, 46u8, 150u8, 126u8, 89u8,
							64u8, 233u8, 166u8, 180u8, 137u8, 52u8, 233u8, 252u8, 255u8, 36u8,
							20u8,
						],
					)
				}
			}
		}
	}
	pub mod hrmp {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct HrmpInitOpenChannel {
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub proposed_max_capacity: ::core::primitive::u32,
				pub proposed_max_message_size: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct HrmpAcceptOpenChannel {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct HrmpCloseChannel {
				pub channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceCleanHrmp {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub inbound: ::core::primitive::u32,
				pub outbound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct ForceProcessHrmpOpen {
				pub channels: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct ForceProcessHrmpClose {
				pub channels: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct HrmpCancelOpenRequest {
				pub channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
				pub open_requests: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Initiate opening a channel from a parachain to a given recipient with given channel"]
				#[doc = "parameters."]
				#[doc = ""]
				#[doc = "- `proposed_max_capacity` - specifies how many messages can be in the channel at once."]
				#[doc = "- `proposed_max_message_size` - specifies the maximum size of the messages."]
				#[doc = ""]
				#[doc = "These numbers are a subject to the relay-chain configuration limits."]
				#[doc = ""]
				#[doc = "The channel can be opened only after the recipient confirms it and only on a session"]
				#[doc = "change."]
				pub fn hrmp_init_open_channel(
					&self,
					recipient: runtime_types::polkadot_parachain::primitives::Id,
					proposed_max_capacity: ::core::primitive::u32,
					proposed_max_message_size: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<HrmpInitOpenChannel> {
					::subxt::tx::StaticTxPayload::new(
						"Hrmp",
						"hrmp_init_open_channel",
						HrmpInitOpenChannel {
							recipient,
							proposed_max_capacity,
							proposed_max_message_size,
						},
						[
							170u8, 72u8, 58u8, 42u8, 38u8, 11u8, 110u8, 229u8, 239u8, 136u8, 99u8,
							230u8, 223u8, 225u8, 126u8, 61u8, 234u8, 185u8, 101u8, 156u8, 40u8,
							102u8, 253u8, 123u8, 77u8, 204u8, 217u8, 86u8, 162u8, 66u8, 25u8,
							214u8,
						],
					)
				}
				#[doc = "Accept a pending open channel request from the given sender."]
				#[doc = ""]
				#[doc = "The channel will be opened only on the next session boundary."]
				pub fn hrmp_accept_open_channel(
					&self,
					sender: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<HrmpAcceptOpenChannel> {
					::subxt::tx::StaticTxPayload::new(
						"Hrmp",
						"hrmp_accept_open_channel",
						HrmpAcceptOpenChannel { sender },
						[
							75u8, 111u8, 52u8, 164u8, 204u8, 100u8, 204u8, 111u8, 127u8, 84u8,
							60u8, 136u8, 95u8, 255u8, 48u8, 157u8, 189u8, 76u8, 33u8, 177u8, 223u8,
							27u8, 74u8, 221u8, 139u8, 1u8, 12u8, 128u8, 242u8, 7u8, 3u8, 53u8,
						],
					)
				}
				#[doc = "Initiate unilateral closing of a channel. The origin must be either the sender or the"]
				#[doc = "recipient in the channel being closed."]
				#[doc = ""]
				#[doc = "The closure can only happen on a session change."]
				pub fn hrmp_close_channel(
					&self,
					channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
				) -> ::subxt::tx::StaticTxPayload<HrmpCloseChannel> {
					::subxt::tx::StaticTxPayload::new(
						"Hrmp",
						"hrmp_close_channel",
						HrmpCloseChannel { channel_id },
						[
							11u8, 202u8, 76u8, 107u8, 213u8, 21u8, 191u8, 190u8, 40u8, 229u8, 60u8,
							115u8, 232u8, 136u8, 41u8, 114u8, 21u8, 19u8, 238u8, 236u8, 202u8,
							56u8, 212u8, 232u8, 34u8, 84u8, 27u8, 70u8, 176u8, 252u8, 218u8, 52u8,
						],
					)
				}
				#[doc = "This extrinsic triggers the cleanup of all the HRMP storage items that"]
				#[doc = "a para may have. Normally this happens once per session, but this allows"]
				#[doc = "you to trigger the cleanup immediately for a specific parachain."]
				#[doc = ""]
				#[doc = "Origin must be Root."]
				#[doc = ""]
				#[doc = "Number of inbound and outbound channels for `para` must be provided as witness data of weighing."]
				pub fn force_clean_hrmp(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					inbound: ::core::primitive::u32,
					outbound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceCleanHrmp> {
					::subxt::tx::StaticTxPayload::new(
						"Hrmp",
						"force_clean_hrmp",
						ForceCleanHrmp { para, inbound, outbound },
						[
							171u8, 109u8, 147u8, 49u8, 163u8, 107u8, 36u8, 169u8, 117u8, 193u8,
							231u8, 114u8, 207u8, 38u8, 240u8, 195u8, 155u8, 222u8, 244u8, 142u8,
							93u8, 79u8, 111u8, 43u8, 5u8, 33u8, 190u8, 30u8, 200u8, 225u8, 173u8,
							64u8,
						],
					)
				}
				#[doc = "Force process HRMP open channel requests."]
				#[doc = ""]
				#[doc = "If there are pending HRMP open channel requests, you can use this"]
				#[doc = "function process all of those requests immediately."]
				#[doc = ""]
				#[doc = "Total number of opening channels must be provided as witness data of weighing."]
				pub fn force_process_hrmp_open(
					&self,
					channels: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceProcessHrmpOpen> {
					::subxt::tx::StaticTxPayload::new(
						"Hrmp",
						"force_process_hrmp_open",
						ForceProcessHrmpOpen { channels },
						[
							231u8, 80u8, 233u8, 15u8, 131u8, 167u8, 223u8, 28u8, 182u8, 185u8,
							213u8, 24u8, 154u8, 160u8, 68u8, 94u8, 160u8, 59u8, 78u8, 85u8, 85u8,
							149u8, 130u8, 131u8, 9u8, 162u8, 169u8, 119u8, 165u8, 44u8, 150u8,
							50u8,
						],
					)
				}
				#[doc = "Force process HRMP close channel requests."]
				#[doc = ""]
				#[doc = "If there are pending HRMP close channel requests, you can use this"]
				#[doc = "function process all of those requests immediately."]
				#[doc = ""]
				#[doc = "Total number of closing channels must be provided as witness data of weighing."]
				pub fn force_process_hrmp_close(
					&self,
					channels: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceProcessHrmpClose> {
					::subxt::tx::StaticTxPayload::new(
						"Hrmp",
						"force_process_hrmp_close",
						ForceProcessHrmpClose { channels },
						[
							248u8, 138u8, 30u8, 151u8, 53u8, 16u8, 44u8, 116u8, 51u8, 194u8, 173u8,
							252u8, 143u8, 53u8, 184u8, 129u8, 80u8, 80u8, 25u8, 118u8, 47u8, 183u8,
							249u8, 130u8, 8u8, 221u8, 56u8, 106u8, 182u8, 114u8, 186u8, 161u8,
						],
					)
				}
				#[doc = "This cancels a pending open channel request. It can be canceled by either of the sender"]
				#[doc = "or the recipient for that request. The origin must be either of those."]
				#[doc = ""]
				#[doc = "The cancellation happens immediately. It is not possible to cancel the request if it is"]
				#[doc = "already accepted."]
				#[doc = ""]
				#[doc = "Total number of open requests (i.e. `HrmpOpenChannelRequestsList`) must be provided as"]
				#[doc = "witness data."]
				pub fn hrmp_cancel_open_request(
					&self,
					channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
					open_requests: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<HrmpCancelOpenRequest> {
					::subxt::tx::StaticTxPayload::new(
						"Hrmp",
						"hrmp_cancel_open_request",
						HrmpCancelOpenRequest { channel_id, open_requests },
						[
							136u8, 217u8, 56u8, 138u8, 227u8, 37u8, 120u8, 83u8, 116u8, 228u8,
							42u8, 111u8, 206u8, 210u8, 177u8, 235u8, 225u8, 98u8, 172u8, 184u8,
							87u8, 65u8, 77u8, 129u8, 7u8, 0u8, 232u8, 139u8, 134u8, 1u8, 59u8,
							19u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Open HRMP channel requested."]
			#[doc = "`[sender, recipient, proposed_max_capacity, proposed_max_message_size]`"]
			pub struct OpenChannelRequested(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub ::core::primitive::u32,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for OpenChannelRequested {
				const PALLET: &'static str = "Hrmp";
				const EVENT: &'static str = "OpenChannelRequested";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An HRMP channel request sent by the receiver was canceled by either party."]
			#[doc = "`[by_parachain, channel_id]`"]
			pub struct OpenChannelCanceled(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
			);
			impl ::subxt::events::StaticEvent for OpenChannelCanceled {
				const PALLET: &'static str = "Hrmp";
				const EVENT: &'static str = "OpenChannelCanceled";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Open HRMP channel accepted. `[sender, recipient]`"]
			pub struct OpenChannelAccepted(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub runtime_types::polkadot_parachain::primitives::Id,
			);
			impl ::subxt::events::StaticEvent for OpenChannelAccepted {
				const PALLET: &'static str = "Hrmp";
				const EVENT: &'static str = "OpenChannelAccepted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "HRMP channel closed. `[by_parachain, channel_id]`"]
			pub struct ChannelClosed(
				pub runtime_types::polkadot_parachain::primitives::Id,
				pub runtime_types::polkadot_parachain::primitives::HrmpChannelId,
			);
			impl ::subxt::events::StaticEvent for ChannelClosed {
				const PALLET: &'static str = "Hrmp";
				const EVENT: &'static str = "ChannelClosed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The set of pending HRMP open channel requests."]
				#[doc = ""]
				#[doc = " The set is accompanied by a list for iteration."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - There are no channels that exists in list but not in the set and vice versa."]
				pub fn hrmp_open_channel_requests(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::HrmpChannelId,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpOpenChannelRequests",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							226u8, 115u8, 207u8, 13u8, 5u8, 81u8, 64u8, 161u8, 246u8, 4u8, 17u8,
							207u8, 210u8, 109u8, 91u8, 54u8, 28u8, 53u8, 35u8, 74u8, 62u8, 91u8,
							196u8, 236u8, 18u8, 70u8, 13u8, 86u8, 235u8, 74u8, 181u8, 169u8,
						],
					)
				}
				#[doc = " The set of pending HRMP open channel requests."]
				#[doc = ""]
				#[doc = " The set is accompanied by a list for iteration."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - There are no channels that exists in list but not in the set and vice versa."]
				pub fn hrmp_open_channel_requests_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpOpenChannelRequests",
						Vec::new(),
						[
							226u8, 115u8, 207u8, 13u8, 5u8, 81u8, 64u8, 161u8, 246u8, 4u8, 17u8,
							207u8, 210u8, 109u8, 91u8, 54u8, 28u8, 53u8, 35u8, 74u8, 62u8, 91u8,
							196u8, 236u8, 18u8, 70u8, 13u8, 86u8, 235u8, 74u8, 181u8, 169u8,
						],
					)
				}
				pub fn hrmp_open_channel_requests_list(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpOpenChannelRequestsList",
						vec![],
						[
							187u8, 157u8, 7u8, 183u8, 88u8, 215u8, 128u8, 174u8, 244u8, 130u8,
							137u8, 13u8, 110u8, 126u8, 181u8, 165u8, 142u8, 69u8, 75u8, 37u8, 37u8,
							149u8, 46u8, 100u8, 140u8, 69u8, 234u8, 171u8, 103u8, 136u8, 223u8,
							193u8,
						],
					)
				}
				#[doc = " This mapping tracks how many open channel requests are initiated by a given sender para."]
				#[doc = " Invariant: `HrmpOpenChannelRequests` should contain the same number of items that has"]
				#[doc = " `(X, _)` as the number of `HrmpOpenChannelRequestCount` for `X`."]
				pub fn hrmp_open_channel_request_count(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpOpenChannelRequestCount",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							156u8, 87u8, 232u8, 34u8, 30u8, 237u8, 159u8, 78u8, 212u8, 138u8,
							140u8, 206u8, 191u8, 117u8, 209u8, 218u8, 251u8, 146u8, 217u8, 56u8,
							93u8, 15u8, 131u8, 64u8, 126u8, 253u8, 126u8, 1u8, 12u8, 242u8, 176u8,
							217u8,
						],
					)
				}
				#[doc = " This mapping tracks how many open channel requests are initiated by a given sender para."]
				#[doc = " Invariant: `HrmpOpenChannelRequests` should contain the same number of items that has"]
				#[doc = " `(X, _)` as the number of `HrmpOpenChannelRequestCount` for `X`."]
				pub fn hrmp_open_channel_request_count_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpOpenChannelRequestCount",
						Vec::new(),
						[
							156u8, 87u8, 232u8, 34u8, 30u8, 237u8, 159u8, 78u8, 212u8, 138u8,
							140u8, 206u8, 191u8, 117u8, 209u8, 218u8, 251u8, 146u8, 217u8, 56u8,
							93u8, 15u8, 131u8, 64u8, 126u8, 253u8, 126u8, 1u8, 12u8, 242u8, 176u8,
							217u8,
						],
					)
				}
				#[doc = " This mapping tracks how many open channel requests were accepted by a given recipient para."]
				#[doc = " Invariant: `HrmpOpenChannelRequests` should contain the same number of items `(_, X)` with"]
				#[doc = " `confirmed` set to true, as the number of `HrmpAcceptedChannelRequestCount` for `X`."]
				pub fn hrmp_accepted_channel_request_count(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpAcceptedChannelRequestCount",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							93u8, 183u8, 17u8, 253u8, 119u8, 213u8, 106u8, 205u8, 17u8, 10u8,
							230u8, 242u8, 5u8, 223u8, 49u8, 235u8, 41u8, 221u8, 80u8, 51u8, 153u8,
							62u8, 191u8, 3u8, 120u8, 224u8, 46u8, 164u8, 161u8, 6u8, 15u8, 15u8,
						],
					)
				}
				#[doc = " This mapping tracks how many open channel requests were accepted by a given recipient para."]
				#[doc = " Invariant: `HrmpOpenChannelRequests` should contain the same number of items `(_, X)` with"]
				#[doc = " `confirmed` set to true, as the number of `HrmpAcceptedChannelRequestCount` for `X`."]
				pub fn hrmp_accepted_channel_request_count_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpAcceptedChannelRequestCount",
						Vec::new(),
						[
							93u8, 183u8, 17u8, 253u8, 119u8, 213u8, 106u8, 205u8, 17u8, 10u8,
							230u8, 242u8, 5u8, 223u8, 49u8, 235u8, 41u8, 221u8, 80u8, 51u8, 153u8,
							62u8, 191u8, 3u8, 120u8, 224u8, 46u8, 164u8, 161u8, 6u8, 15u8, 15u8,
						],
					)
				}
				#[doc = " A set of pending HRMP close channel requests that are going to be closed during the session"]
				#[doc = " change. Used for checking if a given channel is registered for closure."]
				#[doc = ""]
				#[doc = " The set is accompanied by a list for iteration."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - There are no channels that exists in list but not in the set and vice versa."]
				pub fn hrmp_close_channel_requests(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::HrmpChannelId,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<()>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpCloseChannelRequests",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							125u8, 131u8, 1u8, 231u8, 19u8, 207u8, 229u8, 72u8, 150u8, 100u8,
							165u8, 215u8, 241u8, 165u8, 91u8, 35u8, 230u8, 148u8, 127u8, 249u8,
							128u8, 221u8, 167u8, 172u8, 67u8, 30u8, 177u8, 176u8, 134u8, 223u8,
							39u8, 87u8,
						],
					)
				}
				#[doc = " A set of pending HRMP close channel requests that are going to be closed during the session"]
				#[doc = " change. Used for checking if a given channel is registered for closure."]
				#[doc = ""]
				#[doc = " The set is accompanied by a list for iteration."]
				#[doc = ""]
				#[doc = " Invariant:"]
				#[doc = " - There are no channels that exists in list but not in the set and vice versa."]
				pub fn hrmp_close_channel_requests_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<()>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpCloseChannelRequests",
						Vec::new(),
						[
							125u8, 131u8, 1u8, 231u8, 19u8, 207u8, 229u8, 72u8, 150u8, 100u8,
							165u8, 215u8, 241u8, 165u8, 91u8, 35u8, 230u8, 148u8, 127u8, 249u8,
							128u8, 221u8, 167u8, 172u8, 67u8, 30u8, 177u8, 176u8, 134u8, 223u8,
							39u8, 87u8,
						],
					)
				}
				pub fn hrmp_close_channel_requests_list(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpCloseChannelRequestsList",
						vec![],
						[
							192u8, 165u8, 71u8, 70u8, 211u8, 233u8, 155u8, 146u8, 160u8, 58u8,
							103u8, 64u8, 123u8, 232u8, 157u8, 71u8, 199u8, 223u8, 158u8, 5u8,
							164u8, 93u8, 231u8, 153u8, 1u8, 63u8, 155u8, 4u8, 138u8, 89u8, 178u8,
							116u8,
						],
					)
				}
				#[doc = " The HRMP watermark associated with each para."]
				#[doc = " Invariant:"]
				#[doc = " - each para `P` used here as a key should satisfy `Paras::is_valid_para(P)` within a session."]
				pub fn hrmp_watermarks(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpWatermarks",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							231u8, 195u8, 117u8, 35u8, 235u8, 18u8, 80u8, 28u8, 212u8, 37u8, 253u8,
							204u8, 71u8, 217u8, 12u8, 35u8, 219u8, 250u8, 45u8, 83u8, 102u8, 236u8,
							186u8, 149u8, 54u8, 31u8, 83u8, 19u8, 129u8, 51u8, 103u8, 155u8,
						],
					)
				}
				#[doc = " The HRMP watermark associated with each para."]
				#[doc = " Invariant:"]
				#[doc = " - each para `P` used here as a key should satisfy `Paras::is_valid_para(P)` within a session."]
				pub fn hrmp_watermarks_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpWatermarks",
						Vec::new(),
						[
							231u8, 195u8, 117u8, 35u8, 235u8, 18u8, 80u8, 28u8, 212u8, 37u8, 253u8,
							204u8, 71u8, 217u8, 12u8, 35u8, 219u8, 250u8, 45u8, 83u8, 102u8, 236u8,
							186u8, 149u8, 54u8, 31u8, 83u8, 19u8, 129u8, 51u8, 103u8, 155u8,
						],
					)
				}
				#[doc = " HRMP channel data associated with each para."]
				#[doc = " Invariant:"]
				#[doc = " - each participant in the channel should satisfy `Paras::is_valid_para(P)` within a session."]
				pub fn hrmp_channels(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::HrmpChannelId,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpChannels",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							224u8, 252u8, 187u8, 122u8, 179u8, 193u8, 227u8, 250u8, 255u8, 216u8,
							107u8, 26u8, 224u8, 16u8, 178u8, 111u8, 77u8, 237u8, 177u8, 148u8,
							22u8, 17u8, 213u8, 99u8, 194u8, 140u8, 217u8, 211u8, 151u8, 51u8, 66u8,
							169u8,
						],
					)
				}
				#[doc = " HRMP channel data associated with each para."]
				#[doc = " Invariant:"]
				#[doc = " - each participant in the channel should satisfy `Paras::is_valid_para(P)` within a session."]
				pub fn hrmp_channels_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpChannels",
						Vec::new(),
						[
							224u8, 252u8, 187u8, 122u8, 179u8, 193u8, 227u8, 250u8, 255u8, 216u8,
							107u8, 26u8, 224u8, 16u8, 178u8, 111u8, 77u8, 237u8, 177u8, 148u8,
							22u8, 17u8, 213u8, 99u8, 194u8, 140u8, 217u8, 211u8, 151u8, 51u8, 66u8,
							169u8,
						],
					)
				}
				#[doc = " Ingress/egress indexes allow to find all the senders and receivers given the opposite side."]
				#[doc = " I.e."]
				#[doc = ""]
				#[doc = " (a) ingress index allows to find all the senders for a given recipient."]
				#[doc = " (b) egress index allows to find all the recipients for a given sender."]
				#[doc = ""]
				#[doc = " Invariants:"]
				#[doc = " - for each ingress index entry for `P` each item `I` in the index should present in"]
				#[doc = "   `HrmpChannels` as `(I, P)`."]
				#[doc = " - for each egress index entry for `P` each item `E` in the index should present in"]
				#[doc = "   `HrmpChannels` as `(P, E)`."]
				#[doc = " - there should be no other dangling channels in `HrmpChannels`."]
				#[doc = " - the vectors are sorted."]
				pub fn hrmp_ingress_channels_index(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpIngressChannelsIndex",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							58u8, 193u8, 212u8, 225u8, 48u8, 195u8, 119u8, 15u8, 61u8, 166u8,
							249u8, 1u8, 118u8, 67u8, 253u8, 40u8, 58u8, 220u8, 124u8, 152u8, 4u8,
							16u8, 155u8, 151u8, 195u8, 15u8, 205u8, 175u8, 234u8, 0u8, 101u8, 99u8,
						],
					)
				}
				#[doc = " Ingress/egress indexes allow to find all the senders and receivers given the opposite side."]
				#[doc = " I.e."]
				#[doc = ""]
				#[doc = " (a) ingress index allows to find all the senders for a given recipient."]
				#[doc = " (b) egress index allows to find all the recipients for a given sender."]
				#[doc = ""]
				#[doc = " Invariants:"]
				#[doc = " - for each ingress index entry for `P` each item `I` in the index should present in"]
				#[doc = "   `HrmpChannels` as `(I, P)`."]
				#[doc = " - for each egress index entry for `P` each item `E` in the index should present in"]
				#[doc = "   `HrmpChannels` as `(P, E)`."]
				#[doc = " - there should be no other dangling channels in `HrmpChannels`."]
				#[doc = " - the vectors are sorted."]
				pub fn hrmp_ingress_channels_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpIngressChannelsIndex",
						Vec::new(),
						[
							58u8, 193u8, 212u8, 225u8, 48u8, 195u8, 119u8, 15u8, 61u8, 166u8,
							249u8, 1u8, 118u8, 67u8, 253u8, 40u8, 58u8, 220u8, 124u8, 152u8, 4u8,
							16u8, 155u8, 151u8, 195u8, 15u8, 205u8, 175u8, 234u8, 0u8, 101u8, 99u8,
						],
					)
				}
				pub fn hrmp_egress_channels_index(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpEgressChannelsIndex",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							9u8, 242u8, 41u8, 234u8, 85u8, 193u8, 232u8, 245u8, 254u8, 26u8, 240u8,
							113u8, 184u8, 151u8, 150u8, 44u8, 43u8, 98u8, 84u8, 209u8, 145u8,
							175u8, 128u8, 68u8, 183u8, 112u8, 171u8, 236u8, 211u8, 32u8, 177u8,
							88u8,
						],
					)
				}
				pub fn hrmp_egress_channels_index_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpEgressChannelsIndex",
						Vec::new(),
						[
							9u8, 242u8, 41u8, 234u8, 85u8, 193u8, 232u8, 245u8, 254u8, 26u8, 240u8,
							113u8, 184u8, 151u8, 150u8, 44u8, 43u8, 98u8, 84u8, 209u8, 145u8,
							175u8, 128u8, 68u8, 183u8, 112u8, 171u8, 236u8, 211u8, 32u8, 177u8,
							88u8,
						],
					)
				}
				#[doc = " Storage for the messages for each channel."]
				#[doc = " Invariant: cannot be non-empty if the corresponding channel in `HrmpChannels` is `None`."]
				pub fn hrmp_channel_contents(
					&self,
					_0: impl ::std::borrow::Borrow<
						runtime_types::polkadot_parachain::primitives::HrmpChannelId,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_core_primitives::InboundHrmpMessage<
								::core::primitive::u32,
							>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpChannelContents",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							114u8, 86u8, 172u8, 88u8, 118u8, 243u8, 133u8, 147u8, 108u8, 60u8,
							128u8, 235u8, 45u8, 80u8, 225u8, 130u8, 89u8, 50u8, 40u8, 118u8, 63u8,
							3u8, 83u8, 222u8, 75u8, 167u8, 148u8, 150u8, 193u8, 90u8, 196u8, 225u8,
						],
					)
				}
				#[doc = " Storage for the messages for each channel."]
				#[doc = " Invariant: cannot be non-empty if the corresponding channel in `HrmpChannels` is `None`."]
				pub fn hrmp_channel_contents_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_core_primitives::InboundHrmpMessage<
								::core::primitive::u32,
							>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpChannelContents",
						Vec::new(),
						[
							114u8, 86u8, 172u8, 88u8, 118u8, 243u8, 133u8, 147u8, 108u8, 60u8,
							128u8, 235u8, 45u8, 80u8, 225u8, 130u8, 89u8, 50u8, 40u8, 118u8, 63u8,
							3u8, 83u8, 222u8, 75u8, 167u8, 148u8, 150u8, 193u8, 90u8, 196u8, 225u8,
						],
					)
				}
				#[doc = " Maintains a mapping that can be used to answer the question: What paras sent a message at"]
				#[doc = " the given block number for a given receiver. Invariants:"]
				#[doc = " - The inner `Vec<ParaId>` is never empty."]
				#[doc = " - The inner `Vec<ParaId>` cannot store two same `ParaId`."]
				#[doc = " - The outer vector is sorted ascending by block number and cannot store two items with the"]
				#[doc = "   same block number."]
				pub fn hrmp_channel_digests(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::core::primitive::u32,
							::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpChannelDigests",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							205u8, 18u8, 60u8, 54u8, 123u8, 40u8, 160u8, 149u8, 174u8, 45u8, 135u8,
							213u8, 83u8, 44u8, 97u8, 243u8, 47u8, 200u8, 156u8, 131u8, 15u8, 63u8,
							170u8, 206u8, 101u8, 17u8, 244u8, 132u8, 73u8, 133u8, 79u8, 104u8,
						],
					)
				}
				#[doc = " Maintains a mapping that can be used to answer the question: What paras sent a message at"]
				#[doc = " the given block number for a given receiver. Invariants:"]
				#[doc = " - The inner `Vec<ParaId>` is never empty."]
				#[doc = " - The inner `Vec<ParaId>` cannot store two same `ParaId`."]
				#[doc = " - The outer vector is sorted ascending by block number and cannot store two items with the"]
				#[doc = "   same block number."]
				pub fn hrmp_channel_digests_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<(
							::core::primitive::u32,
							::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
						)>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Hrmp",
						"HrmpChannelDigests",
						Vec::new(),
						[
							205u8, 18u8, 60u8, 54u8, 123u8, 40u8, 160u8, 149u8, 174u8, 45u8, 135u8,
							213u8, 83u8, 44u8, 97u8, 243u8, 47u8, 200u8, 156u8, 131u8, 15u8, 63u8,
							170u8, 206u8, 101u8, 17u8, 244u8, 132u8, 73u8, 133u8, 79u8, 104u8,
						],
					)
				}
			}
		}
	}
	pub mod para_session_info {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Assignment keys for the current session."]
				#[doc = " Note that this API is private due to it being prone to 'off-by-one' at session boundaries."]
				#[doc = " When in doubt, use `Sessions` API instead."]
				pub fn assignment_keys_unsafe(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							runtime_types::polkadot_primitives::v2::assignment_app::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaSessionInfo",
						"AssignmentKeysUnsafe",
						vec![],
						[
							80u8, 24u8, 61u8, 132u8, 118u8, 225u8, 207u8, 75u8, 35u8, 240u8, 209u8,
							255u8, 19u8, 240u8, 114u8, 174u8, 86u8, 65u8, 65u8, 52u8, 135u8, 232u8,
							59u8, 208u8, 3u8, 107u8, 114u8, 241u8, 14u8, 98u8, 40u8, 226u8,
						],
					)
				}
				#[doc = " The earliest session for which previous session info is stored."]
				pub fn earliest_stored_session(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaSessionInfo",
						"EarliestStoredSession",
						vec![],
						[
							25u8, 143u8, 246u8, 184u8, 35u8, 166u8, 140u8, 147u8, 171u8, 5u8,
							164u8, 159u8, 228u8, 21u8, 248u8, 236u8, 48u8, 210u8, 133u8, 140u8,
							171u8, 3u8, 85u8, 250u8, 160u8, 102u8, 95u8, 46u8, 33u8, 81u8, 102u8,
							241u8,
						],
					)
				}
				#[doc = " Session information in a rolling window."]
				#[doc = " Should have an entry in range `EarliestStoredSession..=CurrentSessionIndex`."]
				#[doc = " Does not have any entries before the session index in the first session change notification."]
				pub fn sessions(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::SessionInfo,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaSessionInfo",
						"Sessions",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							33u8, 46u8, 71u8, 15u8, 195u8, 14u8, 107u8, 223u8, 112u8, 69u8, 249u8,
							233u8, 86u8, 249u8, 79u8, 164u8, 20u8, 71u8, 191u8, 32u8, 67u8, 195u8,
							128u8, 61u8, 67u8, 84u8, 79u8, 137u8, 248u8, 85u8, 253u8, 21u8,
						],
					)
				}
				#[doc = " Session information in a rolling window."]
				#[doc = " Should have an entry in range `EarliestStoredSession..=CurrentSessionIndex`."]
				#[doc = " Does not have any entries before the session index in the first session change notification."]
				pub fn sessions_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::SessionInfo,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaSessionInfo",
						"Sessions",
						Vec::new(),
						[
							33u8, 46u8, 71u8, 15u8, 195u8, 14u8, 107u8, 223u8, 112u8, 69u8, 249u8,
							233u8, 86u8, 249u8, 79u8, 164u8, 20u8, 71u8, 191u8, 32u8, 67u8, 195u8,
							128u8, 61u8, 67u8, 84u8, 79u8, 137u8, 248u8, 85u8, 253u8, 21u8,
						],
					)
				}
				#[doc = " The validator account keys of the validators actively participating in parachain consensus."]
				pub fn account_keys(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaSessionInfo",
						"AccountKeys",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							48u8, 179u8, 139u8, 15u8, 144u8, 71u8, 92u8, 160u8, 254u8, 237u8, 98u8,
							60u8, 254u8, 208u8, 201u8, 32u8, 79u8, 55u8, 3u8, 33u8, 188u8, 134u8,
							18u8, 151u8, 132u8, 40u8, 192u8, 215u8, 94u8, 124u8, 148u8, 142u8,
						],
					)
				}
				#[doc = " The validator account keys of the validators actively participating in parachain consensus."]
				pub fn account_keys_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParaSessionInfo",
						"AccountKeys",
						Vec::new(),
						[
							48u8, 179u8, 139u8, 15u8, 144u8, 71u8, 92u8, 160u8, 254u8, 237u8, 98u8,
							60u8, 254u8, 208u8, 201u8, 32u8, 79u8, 55u8, 3u8, 33u8, 188u8, 134u8,
							18u8, 151u8, 132u8, 40u8, 192u8, 215u8, 94u8, 124u8, 148u8, 142u8,
						],
					)
				}
			}
		}
	}
	pub mod paras_disputes {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceUnfreeze;
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn force_unfreeze(&self) -> ::subxt::tx::StaticTxPayload<ForceUnfreeze> {
					::subxt::tx::StaticTxPayload::new(
						"ParasDisputes",
						"force_unfreeze",
						ForceUnfreeze {},
						[
							212u8, 211u8, 58u8, 159u8, 23u8, 220u8, 64u8, 175u8, 65u8, 50u8, 192u8,
							122u8, 113u8, 189u8, 74u8, 191u8, 48u8, 93u8, 251u8, 50u8, 237u8,
							240u8, 91u8, 139u8, 193u8, 114u8, 131u8, 125u8, 124u8, 236u8, 191u8,
							190u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_parachains::disputes::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A dispute has been initiated. \\[candidate hash, dispute location\\]"]
			pub struct DisputeInitiated(
				pub runtime_types::polkadot_core_primitives::CandidateHash,
				pub runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation,
			);
			impl ::subxt::events::StaticEvent for DisputeInitiated {
				const PALLET: &'static str = "ParasDisputes";
				const EVENT: &'static str = "DisputeInitiated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A dispute has concluded for or against a candidate."]
			#[doc = "`\\[para id, candidate hash, dispute result\\]`"]
			pub struct DisputeConcluded(
				pub runtime_types::polkadot_core_primitives::CandidateHash,
				pub runtime_types::polkadot_runtime_parachains::disputes::DisputeResult,
			);
			impl ::subxt::events::StaticEvent for DisputeConcluded {
				const PALLET: &'static str = "ParasDisputes";
				const EVENT: &'static str = "DisputeConcluded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A dispute has timed out due to insufficient participation."]
			#[doc = "`\\[para id, candidate hash\\]`"]
			pub struct DisputeTimedOut(pub runtime_types::polkadot_core_primitives::CandidateHash);
			impl ::subxt::events::StaticEvent for DisputeTimedOut {
				const PALLET: &'static str = "ParasDisputes";
				const EVENT: &'static str = "DisputeTimedOut";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A dispute has concluded with supermajority against a candidate."]
			#[doc = "Block authors should no longer build on top of this head and should"]
			#[doc = "instead revert the block at the given height. This should be the"]
			#[doc = "number of the child of the last known valid block in the chain."]
			pub struct Revert(pub ::core::primitive::u32);
			impl ::subxt::events::StaticEvent for Revert {
				const PALLET: &'static str = "ParasDisputes";
				const EVENT: &'static str = "Revert";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The last pruned session, if any. All data stored by this module"]
				#[doc = " references sessions."]
				pub fn last_pruned_session(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"LastPrunedSession",
						vec![],
						[
							125u8, 138u8, 99u8, 242u8, 9u8, 246u8, 215u8, 246u8, 141u8, 6u8, 129u8,
							87u8, 27u8, 58u8, 53u8, 121u8, 61u8, 119u8, 35u8, 104u8, 33u8, 43u8,
							179u8, 82u8, 244u8, 121u8, 174u8, 135u8, 87u8, 119u8, 236u8, 105u8,
						],
					)
				}
				#[doc = " All ongoing or concluded disputes for the last several sessions."]
				pub fn disputes(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<
						runtime_types::polkadot_core_primitives::CandidateHash,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::DisputeState<
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"Disputes",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							192u8, 238u8, 255u8, 67u8, 169u8, 86u8, 99u8, 243u8, 228u8, 88u8,
							142u8, 138u8, 183u8, 117u8, 82u8, 22u8, 163u8, 30u8, 175u8, 247u8,
							50u8, 204u8, 12u8, 171u8, 57u8, 189u8, 151u8, 191u8, 196u8, 89u8, 94u8,
							165u8,
						],
					)
				}
				#[doc = " All ongoing or concluded disputes for the last several sessions."]
				pub fn disputes_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_primitives::v2::DisputeState<
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"Disputes",
						Vec::new(),
						[
							192u8, 238u8, 255u8, 67u8, 169u8, 86u8, 99u8, 243u8, 228u8, 88u8,
							142u8, 138u8, 183u8, 117u8, 82u8, 22u8, 163u8, 30u8, 175u8, 247u8,
							50u8, 204u8, 12u8, 171u8, 57u8, 189u8, 151u8, 191u8, 196u8, 89u8, 94u8,
							165u8,
						],
					)
				}
				#[doc = " All included blocks on the chain, as well as the block number in this chain that"]
				#[doc = " should be reverted back to if the candidate is disputed and determined to be invalid."]
				pub fn included(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<
						runtime_types::polkadot_core_primitives::CandidateHash,
					>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"Included",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							129u8, 50u8, 76u8, 60u8, 82u8, 106u8, 248u8, 164u8, 152u8, 80u8, 58u8,
							185u8, 211u8, 225u8, 122u8, 100u8, 234u8, 241u8, 123u8, 205u8, 4u8,
							8u8, 193u8, 116u8, 167u8, 158u8, 252u8, 223u8, 204u8, 226u8, 74u8,
							195u8,
						],
					)
				}
				#[doc = " All included blocks on the chain, as well as the block number in this chain that"]
				#[doc = " should be reverted back to if the candidate is disputed and determined to be invalid."]
				pub fn included_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"Included",
						Vec::new(),
						[
							129u8, 50u8, 76u8, 60u8, 82u8, 106u8, 248u8, 164u8, 152u8, 80u8, 58u8,
							185u8, 211u8, 225u8, 122u8, 100u8, 234u8, 241u8, 123u8, 205u8, 4u8,
							8u8, 193u8, 116u8, 167u8, 158u8, 252u8, 223u8, 204u8, 226u8, 74u8,
							195u8,
						],
					)
				}
				#[doc = " Maps session indices to a vector indicating the number of potentially-spam disputes"]
				#[doc = " each validator is participating in. Potentially-spam disputes are remote disputes which have"]
				#[doc = " fewer than `byzantine_threshold + 1` validators."]
				#[doc = ""]
				#[doc = " The i'th entry of the vector corresponds to the i'th validator in the session."]
				pub fn spam_slots(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"SpamSlots",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							172u8, 23u8, 120u8, 188u8, 71u8, 248u8, 252u8, 41u8, 132u8, 221u8,
							98u8, 215u8, 33u8, 242u8, 168u8, 196u8, 90u8, 123u8, 190u8, 27u8,
							147u8, 6u8, 196u8, 175u8, 198u8, 216u8, 50u8, 74u8, 138u8, 122u8,
							251u8, 238u8,
						],
					)
				}
				#[doc = " Maps session indices to a vector indicating the number of potentially-spam disputes"]
				#[doc = " each validator is participating in. Potentially-spam disputes are remote disputes which have"]
				#[doc = " fewer than `byzantine_threshold + 1` validators."]
				#[doc = ""]
				#[doc = " The i'th entry of the vector corresponds to the i'th validator in the session."]
				pub fn spam_slots_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::std::vec::Vec<::core::primitive::u32>>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"SpamSlots",
						Vec::new(),
						[
							172u8, 23u8, 120u8, 188u8, 71u8, 248u8, 252u8, 41u8, 132u8, 221u8,
							98u8, 215u8, 33u8, 242u8, 168u8, 196u8, 90u8, 123u8, 190u8, 27u8,
							147u8, 6u8, 196u8, 175u8, 198u8, 216u8, 50u8, 74u8, 138u8, 122u8,
							251u8, 238u8,
						],
					)
				}
				#[doc = " Whether the chain is frozen. Starts as `None`. When this is `Some`,"]
				#[doc = " the chain will not accept any new parachain blocks for backing or inclusion,"]
				#[doc = " and its value indicates the last valid block number in the chain."]
				#[doc = " It can only be set back to `None` by governance intervention."]
				pub fn frozen(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::core::option::Option<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ParasDisputes",
						"Frozen",
						vec![],
						[
							133u8, 100u8, 86u8, 220u8, 180u8, 189u8, 65u8, 131u8, 64u8, 56u8,
							219u8, 47u8, 130u8, 167u8, 210u8, 125u8, 49u8, 7u8, 153u8, 254u8, 20u8,
							53u8, 218u8, 177u8, 122u8, 148u8, 16u8, 198u8, 251u8, 50u8, 194u8,
							128u8,
						],
					)
				}
			}
		}
	}
	pub mod registrar {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Register {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
				pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
				pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceRegister {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub deposit: ::core::primitive::u128,
				pub id: runtime_types::polkadot_parachain::primitives::Id,
				pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
				pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Deregister {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Swap {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
				pub other: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceRemoveLock {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Reserve;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Register head data and validation code for a reserved Para Id."]
				#[doc = ""]
				#[doc = "## Arguments"]
				#[doc = "- `origin`: Must be called by a `Signed` origin."]
				#[doc = "- `id`: The para ID. Must be owned/managed by the `origin` signing account."]
				#[doc = "- `genesis_head`: The genesis head data of the parachain/thread."]
				#[doc = "- `validation_code`: The initial validation code of the parachain/thread."]
				#[doc = ""]
				#[doc = "## Deposits/Fees"]
				#[doc = "The origin signed account must reserve a corresponding deposit for the registration. Anything already"]
				#[doc = "reserved previously for this para ID is accounted for."]
				#[doc = ""]
				#[doc = "## Events"]
				#[doc = "The `Registered` event is emitted in case of success."]
				pub fn register(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
					genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
					validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				) -> ::subxt::tx::StaticTxPayload<Register> {
					::subxt::tx::StaticTxPayload::new(
						"Registrar",
						"register",
						Register { id, genesis_head, validation_code },
						[
							154u8, 84u8, 201u8, 125u8, 72u8, 69u8, 188u8, 42u8, 225u8, 14u8, 136u8,
							48u8, 78u8, 86u8, 99u8, 238u8, 252u8, 255u8, 226u8, 219u8, 214u8, 17u8,
							19u8, 9u8, 12u8, 13u8, 174u8, 243u8, 37u8, 134u8, 76u8, 23u8,
						],
					)
				}
				#[doc = "Force the registration of a Para Id on the relay chain."]
				#[doc = ""]
				#[doc = "This function must be called by a Root origin."]
				#[doc = ""]
				#[doc = "The deposit taken can be specified for this registration. Any `ParaId`"]
				#[doc = "can be registered, including sub-1000 IDs which are System Parachains."]
				pub fn force_register(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
					deposit: ::core::primitive::u128,
					id: runtime_types::polkadot_parachain::primitives::Id,
					genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
					validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
				) -> ::subxt::tx::StaticTxPayload<ForceRegister> {
					::subxt::tx::StaticTxPayload::new(
						"Registrar",
						"force_register",
						ForceRegister { who, deposit, id, genesis_head, validation_code },
						[
							59u8, 24u8, 236u8, 163u8, 53u8, 49u8, 92u8, 199u8, 38u8, 76u8, 101u8,
							73u8, 166u8, 105u8, 145u8, 55u8, 89u8, 30u8, 30u8, 137u8, 151u8, 219u8,
							116u8, 226u8, 168u8, 220u8, 222u8, 6u8, 105u8, 91u8, 254u8, 216u8,
						],
					)
				}
				#[doc = "Deregister a Para Id, freeing all data and returning any deposit."]
				#[doc = ""]
				#[doc = "The caller must be Root, the `para` owner, or the `para` itself. The para must be a parathread."]
				pub fn deregister(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<Deregister> {
					::subxt::tx::StaticTxPayload::new(
						"Registrar",
						"deregister",
						Deregister { id },
						[
							137u8, 9u8, 146u8, 11u8, 126u8, 125u8, 186u8, 222u8, 246u8, 199u8,
							94u8, 229u8, 147u8, 245u8, 213u8, 51u8, 203u8, 181u8, 78u8, 87u8, 18u8,
							255u8, 79u8, 107u8, 234u8, 2u8, 21u8, 212u8, 1u8, 73u8, 173u8, 253u8,
						],
					)
				}
				#[doc = "Swap a parachain with another parachain or parathread."]
				#[doc = ""]
				#[doc = "The origin must be Root, the `para` owner, or the `para` itself."]
				#[doc = ""]
				#[doc = "The swap will happen only if there is already an opposite swap pending. If there is not,"]
				#[doc = "the swap will be stored in the pending swaps map, ready for a later confirmatory swap."]
				#[doc = ""]
				#[doc = "The `ParaId`s remain mapped to the same head data and code so external code can rely on"]
				#[doc = "`ParaId` to be a long-term identifier of a notional \"parachain\". However, their"]
				#[doc = "scheduling info (i.e. whether they're a parathread or parachain), auction information"]
				#[doc = "and the auction deposit are switched."]
				pub fn swap(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
					other: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<Swap> {
					::subxt::tx::StaticTxPayload::new(
						"Registrar",
						"swap",
						Swap { id, other },
						[
							238u8, 154u8, 249u8, 250u8, 57u8, 242u8, 47u8, 17u8, 50u8, 70u8, 124u8,
							189u8, 193u8, 137u8, 107u8, 138u8, 216u8, 137u8, 160u8, 103u8, 192u8,
							133u8, 7u8, 130u8, 41u8, 39u8, 47u8, 139u8, 202u8, 7u8, 84u8, 214u8,
						],
					)
				}
				#[doc = "Remove a manager lock from a para. This will allow the manager of a"]
				#[doc = "previously locked para to deregister or swap a para without using governance."]
				#[doc = ""]
				#[doc = "Can only be called by the Root origin."]
				pub fn force_remove_lock(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<ForceRemoveLock> {
					::subxt::tx::StaticTxPayload::new(
						"Registrar",
						"force_remove_lock",
						ForceRemoveLock { para },
						[
							161u8, 77u8, 236u8, 143u8, 243u8, 159u8, 88u8, 61u8, 217u8, 140u8,
							161u8, 61u8, 20u8, 76u8, 130u8, 59u8, 85u8, 219u8, 105u8, 234u8, 146u8,
							142u8, 121u8, 154u8, 170u8, 210u8, 204u8, 175u8, 160u8, 86u8, 249u8,
							150u8,
						],
					)
				}
				#[doc = "Reserve a Para Id on the relay chain."]
				#[doc = ""]
				#[doc = "This function will reserve a new Para Id to be owned/managed by the origin account."]
				#[doc = "The origin account is able to register head data and validation code using `register` to create"]
				#[doc = "a parathread. Using the Slots pallet, a parathread can then be upgraded to get a parachain slot."]
				#[doc = ""]
				#[doc = "## Arguments"]
				#[doc = "- `origin`: Must be called by a `Signed` origin. Becomes the manager/owner of the new para ID."]
				#[doc = ""]
				#[doc = "## Deposits/Fees"]
				#[doc = "The origin must reserve a deposit of `ParaDeposit` for the registration."]
				#[doc = ""]
				#[doc = "## Events"]
				#[doc = "The `Reserved` event is emitted in case of success, which provides the ID reserved for use."]
				pub fn reserve(&self) -> ::subxt::tx::StaticTxPayload<Reserve> {
					::subxt::tx::StaticTxPayload::new(
						"Registrar",
						"reserve",
						Reserve {},
						[
							22u8, 210u8, 13u8, 54u8, 253u8, 13u8, 89u8, 174u8, 232u8, 119u8, 148u8,
							206u8, 130u8, 133u8, 199u8, 127u8, 201u8, 205u8, 8u8, 213u8, 108u8,
							93u8, 135u8, 88u8, 238u8, 171u8, 31u8, 193u8, 23u8, 113u8, 106u8,
							135u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Registered {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
				pub manager: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Registered {
				const PALLET: &'static str = "Registrar";
				const EVENT: &'static str = "Registered";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Deregistered {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::events::StaticEvent for Deregistered {
				const PALLET: &'static str = "Registrar";
				const EVENT: &'static str = "Deregistered";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Reserved {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Registrar";
				const EVENT: &'static str = "Reserved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Pending swap operations."]
				pub fn pending_swap(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Registrar",
						"PendingSwap",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							121u8, 124u8, 4u8, 120u8, 173u8, 48u8, 227u8, 135u8, 72u8, 74u8, 238u8,
							230u8, 1u8, 175u8, 33u8, 241u8, 138u8, 82u8, 217u8, 129u8, 138u8,
							107u8, 59u8, 8u8, 205u8, 244u8, 192u8, 159u8, 171u8, 123u8, 149u8,
							174u8,
						],
					)
				}
				#[doc = " Pending swap operations."]
				pub fn pending_swap_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Registrar",
						"PendingSwap",
						Vec::new(),
						[
							121u8, 124u8, 4u8, 120u8, 173u8, 48u8, 227u8, 135u8, 72u8, 74u8, 238u8,
							230u8, 1u8, 175u8, 33u8, 241u8, 138u8, 82u8, 217u8, 129u8, 138u8,
							107u8, 59u8, 8u8, 205u8, 244u8, 192u8, 159u8, 171u8, 123u8, 149u8,
							174u8,
						],
					)
				}
				#[doc = " Amount held on deposit for each para and the original depositor."]
				#[doc = ""]
				#[doc = " The given account ID is responsible for registering the code and initial head data, but may only do"]
				#[doc = " so if it isn't yet registered. (After that, it's up to governance to do so.)"]
				pub fn paras(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Registrar",
						"Paras",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							149u8, 3u8, 25u8, 145u8, 60u8, 126u8, 219u8, 71u8, 88u8, 241u8, 122u8,
							99u8, 134u8, 191u8, 60u8, 172u8, 230u8, 230u8, 110u8, 31u8, 43u8, 6u8,
							146u8, 161u8, 51u8, 21u8, 169u8, 220u8, 240u8, 218u8, 124u8, 56u8,
						],
					)
				}
				#[doc = " Amount held on deposit for each para and the original depositor."]
				#[doc = ""]
				#[doc = " The given account ID is responsible for registering the code and initial head data, but may only do"]
				#[doc = " so if it isn't yet registered. (After that, it's up to governance to do so.)"]
				pub fn paras_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Registrar",
						"Paras",
						Vec::new(),
						[
							149u8, 3u8, 25u8, 145u8, 60u8, 126u8, 219u8, 71u8, 88u8, 241u8, 122u8,
							99u8, 134u8, 191u8, 60u8, 172u8, 230u8, 230u8, 110u8, 31u8, 43u8, 6u8,
							146u8, 161u8, 51u8, 21u8, 169u8, 220u8, 240u8, 218u8, 124u8, 56u8,
						],
					)
				}
				#[doc = " The next free `ParaId`."]
				pub fn next_free_para_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_parachain::primitives::Id,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Registrar",
						"NextFreeParaId",
						vec![],
						[
							139u8, 76u8, 36u8, 150u8, 237u8, 36u8, 143u8, 242u8, 252u8, 29u8,
							236u8, 168u8, 97u8, 50u8, 175u8, 120u8, 83u8, 118u8, 205u8, 64u8, 95u8,
							65u8, 7u8, 230u8, 171u8, 86u8, 189u8, 205u8, 231u8, 211u8, 97u8, 29u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The deposit to be paid to run a parathread."]
				#[doc = " This should include the cost for storing the genesis head and validation code."]
				pub fn para_deposit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Registrar",
						"ParaDeposit",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The deposit to be paid per byte stored on chain."]
				pub fn data_deposit_per_byte(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Registrar",
						"DataDepositPerByte",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod auctions {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct NewAuction {
				#[codec(compact)]
				pub duration: ::core::primitive::u32,
				#[codec(compact)]
				pub lease_period_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Bid {
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CancelAuction;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Create a new auction."]
				#[doc = ""]
				#[doc = "This can only happen when there isn't already an auction in progress and may only be"]
				#[doc = "called by the root origin. Accepts the `duration` of this auction and the"]
				#[doc = "`lease_period_index` of the initial lease period of the four that are to be auctioned."]
				pub fn new_auction(
					&self,
					duration: ::core::primitive::u32,
					lease_period_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<NewAuction> {
					::subxt::tx::StaticTxPayload::new(
						"Auctions",
						"new_auction",
						NewAuction { duration, lease_period_index },
						[
							171u8, 40u8, 200u8, 164u8, 213u8, 10u8, 145u8, 164u8, 212u8, 14u8,
							117u8, 215u8, 248u8, 59u8, 34u8, 79u8, 50u8, 176u8, 164u8, 143u8, 92u8,
							82u8, 207u8, 37u8, 103u8, 252u8, 255u8, 142u8, 239u8, 134u8, 114u8,
							151u8,
						],
					)
				}
				#[doc = "Make a new bid from an account (including a parachain account) for deploying a new"]
				#[doc = "parachain."]
				#[doc = ""]
				#[doc = "Multiple simultaneous bids from the same bidder are allowed only as long as all active"]
				#[doc = "bids overlap each other (i.e. are mutually exclusive). Bids cannot be redacted."]
				#[doc = ""]
				#[doc = "- `sub` is the sub-bidder ID, allowing for multiple competing bids to be made by (and"]
				#[doc = "funded by) the same account."]
				#[doc = "- `auction_index` is the index of the auction to bid on. Should just be the present"]
				#[doc = "value of `AuctionCounter`."]
				#[doc = "- `first_slot` is the first lease period index of the range to bid on. This is the"]
				#[doc = "absolute lease period index value, not an auction-specific offset."]
				#[doc = "- `last_slot` is the last lease period index of the range to bid on. This is the"]
				#[doc = "absolute lease period index value, not an auction-specific offset."]
				#[doc = "- `amount` is the amount to bid to be held as deposit for the parachain should the"]
				#[doc = "bid win. This amount is held throughout the range."]
				pub fn bid(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					auction_index: ::core::primitive::u32,
					first_slot: ::core::primitive::u32,
					last_slot: ::core::primitive::u32,
					amount: ::core::primitive::u128,
				) -> ::subxt::tx::StaticTxPayload<Bid> {
					::subxt::tx::StaticTxPayload::new(
						"Auctions",
						"bid",
						Bid { para, auction_index, first_slot, last_slot, amount },
						[
							243u8, 233u8, 248u8, 221u8, 239u8, 59u8, 65u8, 63u8, 125u8, 129u8,
							202u8, 165u8, 30u8, 228u8, 32u8, 73u8, 225u8, 38u8, 128u8, 98u8, 102u8,
							46u8, 203u8, 32u8, 70u8, 74u8, 136u8, 163u8, 83u8, 211u8, 227u8, 139u8,
						],
					)
				}
				#[doc = "Cancel an in-progress auction."]
				#[doc = ""]
				#[doc = "Can only be called by Root origin."]
				pub fn cancel_auction(&self) -> ::subxt::tx::StaticTxPayload<CancelAuction> {
					::subxt::tx::StaticTxPayload::new(
						"Auctions",
						"cancel_auction",
						CancelAuction {},
						[
							182u8, 223u8, 178u8, 136u8, 1u8, 115u8, 229u8, 78u8, 166u8, 128u8,
							28u8, 106u8, 6u8, 248u8, 46u8, 55u8, 110u8, 120u8, 213u8, 11u8, 90u8,
							217u8, 42u8, 120u8, 47u8, 83u8, 126u8, 216u8, 236u8, 251u8, 255u8,
							50u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_common::auctions::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An auction started. Provides its index and the block number where it will begin to"]
			#[doc = "close and the first lease period of the quadruplet that is auctioned."]
			pub struct AuctionStarted {
				pub auction_index: ::core::primitive::u32,
				pub lease_period: ::core::primitive::u32,
				pub ending: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for AuctionStarted {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "AuctionStarted";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "An auction ended. All funds become unreserved."]
			pub struct AuctionClosed {
				pub auction_index: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for AuctionClosed {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "AuctionClosed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Funds were reserved for a winning bid. First balance is the extra amount reserved."]
			#[doc = "Second is the total."]
			pub struct Reserved {
				pub bidder: ::subxt::ext::sp_core::crypto::AccountId32,
				pub extra_reserved: ::core::primitive::u128,
				pub total_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Reserved {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "Reserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Funds were unreserved since bidder is no longer active. `[bidder, amount]`"]
			pub struct Unreserved {
				pub bidder: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Unreserved {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "Unreserved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Someone attempted to lease the same slot twice for a parachain. The amount is held in reserve"]
			#[doc = "but no parachain slot has been leased."]
			pub struct ReserveConfiscated {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
				pub leaser: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for ReserveConfiscated {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "ReserveConfiscated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new bid has been accepted as the current winner."]
			pub struct BidAccepted {
				pub bidder: ::subxt::ext::sp_core::crypto::AccountId32,
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
				pub amount: ::core::primitive::u128,
				pub first_slot: ::core::primitive::u32,
				pub last_slot: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for BidAccepted {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "BidAccepted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The winning offset was chosen for an auction. This will map into the `Winning` storage map."]
			pub struct WinningOffset {
				pub auction_index: ::core::primitive::u32,
				pub block_number: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for WinningOffset {
				const PALLET: &'static str = "Auctions";
				const EVENT: &'static str = "WinningOffset";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Number of auctions started so far."]
				pub fn auction_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Auctions",
						"AuctionCounter",
						vec![],
						[
							67u8, 247u8, 96u8, 152u8, 0u8, 224u8, 230u8, 98u8, 194u8, 107u8, 3u8,
							203u8, 51u8, 201u8, 149u8, 22u8, 184u8, 80u8, 251u8, 239u8, 253u8,
							19u8, 58u8, 192u8, 65u8, 96u8, 189u8, 54u8, 175u8, 130u8, 143u8, 181u8,
						],
					)
				}
				#[doc = " Information relating to the current auction, if there is one."]
				#[doc = ""]
				#[doc = " The first item in the tuple is the lease period index that the first of the four"]
				#[doc = " contiguous lease periods on auction is for. The second is the block number when the"]
				#[doc = " auction will \"begin to end\", i.e. the first block of the Ending Period of the auction."]
				pub fn auction_info(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Auctions",
						"AuctionInfo",
						vec![],
						[
							73u8, 216u8, 173u8, 230u8, 132u8, 78u8, 83u8, 62u8, 200u8, 69u8, 17u8,
							73u8, 57u8, 107u8, 160u8, 90u8, 147u8, 84u8, 29u8, 110u8, 144u8, 215u8,
							169u8, 110u8, 217u8, 77u8, 109u8, 204u8, 1u8, 164u8, 95u8, 83u8,
						],
					)
				}
				#[doc = " Amounts currently reserved in the accounts of the bidders currently winning"]
				#[doc = " (sub-)ranges."]
				pub fn reserved_amounts(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Auctions",
						"ReservedAmounts",
						vec![::subxt::storage::address::StorageMapKey::new(
							&(_0.borrow(), _1.borrow()),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							120u8, 85u8, 180u8, 244u8, 154u8, 135u8, 87u8, 79u8, 75u8, 169u8,
							220u8, 117u8, 227u8, 85u8, 198u8, 214u8, 28u8, 126u8, 66u8, 188u8,
							137u8, 111u8, 110u8, 152u8, 18u8, 233u8, 76u8, 166u8, 55u8, 233u8,
							93u8, 62u8,
						],
					)
				}
				#[doc = " Amounts currently reserved in the accounts of the bidders currently winning"]
				#[doc = " (sub-)ranges."]
				pub fn reserved_amounts_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Auctions",
						"ReservedAmounts",
						Vec::new(),
						[
							120u8, 85u8, 180u8, 244u8, 154u8, 135u8, 87u8, 79u8, 75u8, 169u8,
							220u8, 117u8, 227u8, 85u8, 198u8, 214u8, 28u8, 126u8, 66u8, 188u8,
							137u8, 111u8, 110u8, 152u8, 18u8, 233u8, 76u8, 166u8, 55u8, 233u8,
							93u8, 62u8,
						],
					)
				}
				#[doc = " The winning bids for each of the 10 ranges at each sample in the final Ending Period of"]
				#[doc = " the current auction. The map's key is the 0-based index into the Sample Size. The"]
				#[doc = " first sample of the ending period is 0; the last is `Sample Size - 1`."]
				pub fn winning(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						[::core::option::Option<(
							::subxt::ext::sp_core::crypto::AccountId32,
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u128,
						)>; 36usize],
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Auctions",
						"Winning",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							63u8, 56u8, 143u8, 200u8, 12u8, 71u8, 187u8, 73u8, 215u8, 93u8, 222u8,
							102u8, 5u8, 113u8, 6u8, 170u8, 95u8, 228u8, 28u8, 58u8, 109u8, 62u8,
							3u8, 125u8, 211u8, 139u8, 194u8, 30u8, 151u8, 147u8, 47u8, 205u8,
						],
					)
				}
				#[doc = " The winning bids for each of the 10 ranges at each sample in the final Ending Period of"]
				#[doc = " the current auction. The map's key is the 0-based index into the Sample Size. The"]
				#[doc = " first sample of the ending period is 0; the last is `Sample Size - 1`."]
				pub fn winning_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						[::core::option::Option<(
							::subxt::ext::sp_core::crypto::AccountId32,
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u128,
						)>; 36usize],
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Auctions",
						"Winning",
						Vec::new(),
						[
							63u8, 56u8, 143u8, 200u8, 12u8, 71u8, 187u8, 73u8, 215u8, 93u8, 222u8,
							102u8, 5u8, 113u8, 6u8, 170u8, 95u8, 228u8, 28u8, 58u8, 109u8, 62u8,
							3u8, 125u8, 211u8, 139u8, 194u8, 30u8, 151u8, 147u8, 47u8, 205u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The number of blocks over which an auction may be retroactively ended."]
				pub fn ending_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Auctions",
						"EndingPeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The length of each sample to take during the ending period."]
				#[doc = ""]
				#[doc = " `EndingPeriod` / `SampleLength` = Total # of Samples"]
				pub fn sample_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Auctions",
						"SampleLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn slot_range_count(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Auctions",
						"SlotRangeCount",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				pub fn lease_periods_per_slot(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Auctions",
						"LeasePeriodsPerSlot",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod crowdloan {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Create {
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Contribute {
				#[codec(compact)]
				pub index: runtime_types::polkadot_parachain::primitives::Id,
				#[codec(compact)]
				pub value: ::core::primitive::u128,
				pub signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Withdraw {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				#[codec(compact)]
				pub index: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Refund {
				#[codec(compact)]
				pub index: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Dissolve {
				#[codec(compact)]
				pub index: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Edit {
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddMemo {
				pub index: runtime_types::polkadot_parachain::primitives::Id,
				pub memo: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Poke {
				pub index: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ContributeAll {
				#[codec(compact)]
				pub index: runtime_types::polkadot_parachain::primitives::Id,
				pub signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Create a new crowdloaning campaign for a parachain slot with the given lease period range."]
				#[doc = ""]
				#[doc = "This applies a lock to your parachain configuration, ensuring that it cannot be changed"]
				#[doc = "by the parachain manager."]
				pub fn create(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
					cap: ::core::primitive::u128,
					first_period: ::core::primitive::u32,
					last_period: ::core::primitive::u32,
					end: ::core::primitive::u32,
					verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
				) -> ::subxt::tx::StaticTxPayload<Create> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"create",
						Create { index, cap, first_period, last_period, end, verifier },
						[
							78u8, 52u8, 156u8, 23u8, 104u8, 251u8, 20u8, 233u8, 42u8, 231u8, 16u8,
							192u8, 164u8, 68u8, 98u8, 129u8, 88u8, 126u8, 123u8, 4u8, 210u8, 161u8,
							190u8, 90u8, 67u8, 235u8, 74u8, 184u8, 180u8, 197u8, 248u8, 238u8,
						],
					)
				}
				#[doc = "Contribute to a crowd sale. This will transfer some balance over to fund a parachain"]
				#[doc = "slot. It will be withdrawable when the crowdloan has ended and the funds are unused."]
				pub fn contribute(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
					value: ::core::primitive::u128,
					signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
				) -> ::subxt::tx::StaticTxPayload<Contribute> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"contribute",
						Contribute { index, value, signature },
						[
							159u8, 180u8, 248u8, 203u8, 128u8, 231u8, 28u8, 84u8, 14u8, 214u8,
							69u8, 217u8, 62u8, 201u8, 169u8, 160u8, 45u8, 160u8, 125u8, 255u8,
							95u8, 140u8, 58u8, 3u8, 224u8, 157u8, 199u8, 229u8, 72u8, 40u8, 218u8,
							55u8,
						],
					)
				}
				#[doc = "Withdraw full balance of a specific contributor."]
				#[doc = ""]
				#[doc = "Origin must be signed, but can come from anyone."]
				#[doc = ""]
				#[doc = "The fund must be either in, or ready for, retirement. For a fund to be *in* retirement, then the retirement"]
				#[doc = "flag must be set. For a fund to be ready for retirement, then:"]
				#[doc = "- it must not already be in retirement;"]
				#[doc = "- the amount of raised funds must be bigger than the _free_ balance of the account;"]
				#[doc = "- and either:"]
				#[doc = "  - the block number must be at least `end`; or"]
				#[doc = "  - the current lease period must be greater than the fund's `last_period`."]
				#[doc = ""]
				#[doc = "In this case, the fund's retirement flag is set and its `end` is reset to the current block"]
				#[doc = "number."]
				#[doc = ""]
				#[doc = "- `who`: The account whose contribution should be withdrawn."]
				#[doc = "- `index`: The parachain to whose crowdloan the contribution was made."]
				pub fn withdraw(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
					index: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<Withdraw> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"withdraw",
						Withdraw { who, index },
						[
							147u8, 177u8, 116u8, 152u8, 9u8, 102u8, 4u8, 201u8, 204u8, 145u8,
							104u8, 226u8, 86u8, 211u8, 66u8, 109u8, 109u8, 139u8, 229u8, 97u8,
							215u8, 101u8, 255u8, 181u8, 121u8, 139u8, 165u8, 169u8, 112u8, 173u8,
							213u8, 121u8,
						],
					)
				}
				#[doc = "Automatically refund contributors of an ended crowdloan."]
				#[doc = "Due to weight restrictions, this function may need to be called multiple"]
				#[doc = "times to fully refund all users. We will refund `RemoveKeysLimit` users at a time."]
				#[doc = ""]
				#[doc = "Origin must be signed, but can come from anyone."]
				pub fn refund(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<Refund> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"refund",
						Refund { index },
						[
							223u8, 64u8, 5u8, 135u8, 15u8, 234u8, 60u8, 114u8, 199u8, 216u8, 73u8,
							165u8, 198u8, 34u8, 140u8, 142u8, 214u8, 254u8, 203u8, 163u8, 224u8,
							120u8, 104u8, 54u8, 12u8, 126u8, 72u8, 147u8, 20u8, 180u8, 251u8,
							208u8,
						],
					)
				}
				#[doc = "Remove a fund after the retirement period has ended and all funds have been returned."]
				pub fn dissolve(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<Dissolve> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"dissolve",
						Dissolve { index },
						[
							100u8, 67u8, 105u8, 3u8, 213u8, 149u8, 201u8, 146u8, 241u8, 62u8, 31u8,
							108u8, 58u8, 30u8, 241u8, 141u8, 134u8, 115u8, 56u8, 131u8, 60u8, 75u8,
							143u8, 227u8, 11u8, 32u8, 31u8, 230u8, 165u8, 227u8, 170u8, 126u8,
						],
					)
				}
				#[doc = "Edit the configuration for an in-progress crowdloan."]
				#[doc = ""]
				#[doc = "Can only be called by Root origin."]
				pub fn edit(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
					cap: ::core::primitive::u128,
					first_period: ::core::primitive::u32,
					last_period: ::core::primitive::u32,
					end: ::core::primitive::u32,
					verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
				) -> ::subxt::tx::StaticTxPayload<Edit> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"edit",
						Edit { index, cap, first_period, last_period, end, verifier },
						[
							222u8, 124u8, 94u8, 221u8, 36u8, 183u8, 67u8, 114u8, 198u8, 107u8,
							154u8, 174u8, 142u8, 47u8, 3u8, 181u8, 72u8, 29u8, 2u8, 83u8, 81u8,
							47u8, 168u8, 142u8, 139u8, 63u8, 136u8, 191u8, 41u8, 252u8, 221u8,
							56u8,
						],
					)
				}
				#[doc = "Add an optional memo to an existing crowdloan contribution."]
				#[doc = ""]
				#[doc = "Origin must be Signed, and the user must have contributed to the crowdloan."]
				pub fn add_memo(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
					memo: ::std::vec::Vec<::core::primitive::u8>,
				) -> ::subxt::tx::StaticTxPayload<AddMemo> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"add_memo",
						AddMemo { index, memo },
						[
							104u8, 199u8, 143u8, 251u8, 28u8, 49u8, 144u8, 186u8, 83u8, 108u8,
							26u8, 127u8, 22u8, 141u8, 48u8, 62u8, 194u8, 193u8, 97u8, 10u8, 84u8,
							89u8, 236u8, 191u8, 40u8, 8u8, 1u8, 250u8, 112u8, 165u8, 221u8, 112u8,
						],
					)
				}
				#[doc = "Poke the fund into `NewRaise`"]
				#[doc = ""]
				#[doc = "Origin must be Signed, and the fund has non-zero raise."]
				pub fn poke(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<Poke> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"poke",
						Poke { index },
						[
							118u8, 60u8, 131u8, 17u8, 27u8, 153u8, 57u8, 24u8, 191u8, 211u8, 101u8,
							123u8, 34u8, 145u8, 193u8, 113u8, 244u8, 162u8, 148u8, 143u8, 81u8,
							86u8, 136u8, 23u8, 48u8, 185u8, 52u8, 60u8, 216u8, 243u8, 63u8, 102u8,
						],
					)
				}
				#[doc = "Contribute your entire balance to a crowd sale. This will transfer the entire balance of a user over to fund a parachain"]
				#[doc = "slot. It will be withdrawable when the crowdloan has ended and the funds are unused."]
				pub fn contribute_all(
					&self,
					index: runtime_types::polkadot_parachain::primitives::Id,
					signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
				) -> ::subxt::tx::StaticTxPayload<ContributeAll> {
					::subxt::tx::StaticTxPayload::new(
						"Crowdloan",
						"contribute_all",
						ContributeAll { index, signature },
						[
							94u8, 61u8, 105u8, 107u8, 204u8, 18u8, 223u8, 242u8, 19u8, 162u8,
							205u8, 130u8, 203u8, 73u8, 42u8, 85u8, 208u8, 157u8, 115u8, 112u8,
							168u8, 10u8, 163u8, 80u8, 222u8, 71u8, 23u8, 194u8, 142u8, 4u8, 82u8,
							253u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_common::crowdloan::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Create a new crowdloaning campaign."]
			pub struct Created {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::events::StaticEvent for Created {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "Created";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Contributed to a crowd sale."]
			pub struct Contributed {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub fund_index: runtime_types::polkadot_parachain::primitives::Id,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Contributed {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "Contributed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Withdrew full balance of a contributor."]
			pub struct Withdrew {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub fund_index: runtime_types::polkadot_parachain::primitives::Id,
				pub amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Withdrew {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "Withdrew";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The loans in a fund have been partially dissolved, i.e. there are some left"]
			#[doc = "over child keys that still need to be killed."]
			pub struct PartiallyRefunded {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::events::StaticEvent for PartiallyRefunded {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "PartiallyRefunded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "All loans in a fund have been refunded."]
			pub struct AllRefunded {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::events::StaticEvent for AllRefunded {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "AllRefunded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Fund is dissolved."]
			pub struct Dissolved {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::events::StaticEvent for Dissolved {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "Dissolved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The result of trying to submit a new bid to the Slots pallet."]
			pub struct HandleBidResult {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for HandleBidResult {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "HandleBidResult";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The configuration to a crowdloan has been edited."]
			pub struct Edited {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::events::StaticEvent for Edited {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "Edited";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A memo has been updated."]
			pub struct MemoUpdated {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
				pub memo: ::std::vec::Vec<::core::primitive::u8>,
			}
			impl ::subxt::events::StaticEvent for MemoUpdated {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "MemoUpdated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A parachain has been moved to `NewRaise`"]
			pub struct AddedToNewRaise {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
			}
			impl ::subxt::events::StaticEvent for AddedToNewRaise {
				const PALLET: &'static str = "Crowdloan";
				const EVENT: &'static str = "AddedToNewRaise";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Info on all of the funds."]
				pub fn funds(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Crowdloan",
						"Funds",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							231u8, 126u8, 89u8, 84u8, 167u8, 23u8, 211u8, 70u8, 203u8, 124u8, 20u8,
							162u8, 112u8, 38u8, 201u8, 207u8, 82u8, 202u8, 80u8, 228u8, 4u8, 41u8,
							95u8, 190u8, 193u8, 185u8, 178u8, 85u8, 179u8, 102u8, 53u8, 63u8,
						],
					)
				}
				#[doc = " Info on all of the funds."]
				pub fn funds_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u128,
							::core::primitive::u32,
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Crowdloan",
						"Funds",
						Vec::new(),
						[
							231u8, 126u8, 89u8, 84u8, 167u8, 23u8, 211u8, 70u8, 203u8, 124u8, 20u8,
							162u8, 112u8, 38u8, 201u8, 207u8, 82u8, 202u8, 80u8, 228u8, 4u8, 41u8,
							95u8, 190u8, 193u8, 185u8, 178u8, 85u8, 179u8, 102u8, 53u8, 63u8,
						],
					)
				}
				#[doc = " The funds that have had additional contributions during the last block. This is used"]
				#[doc = " in order to determine which funds should submit new or updated bids."]
				pub fn new_raise(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Crowdloan",
						"NewRaise",
						vec![],
						[
							8u8, 180u8, 9u8, 197u8, 254u8, 198u8, 89u8, 112u8, 29u8, 153u8, 243u8,
							196u8, 92u8, 204u8, 135u8, 232u8, 93u8, 239u8, 147u8, 103u8, 130u8,
							28u8, 128u8, 124u8, 4u8, 236u8, 29u8, 248u8, 27u8, 165u8, 111u8, 147u8,
						],
					)
				}
				#[doc = " The number of auctions that have entered into their ending period so far."]
				pub fn endings_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Crowdloan",
						"EndingsCount",
						vec![],
						[
							12u8, 159u8, 166u8, 75u8, 192u8, 33u8, 21u8, 244u8, 149u8, 200u8, 49u8,
							54u8, 191u8, 174u8, 202u8, 86u8, 76u8, 115u8, 189u8, 35u8, 192u8,
							175u8, 156u8, 188u8, 41u8, 23u8, 92u8, 36u8, 141u8, 235u8, 248u8,
							143u8,
						],
					)
				}
				#[doc = " Tracker for the next available fund index"]
				pub fn next_fund_index(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Crowdloan",
						"NextFundIndex",
						vec![],
						[
							1u8, 215u8, 164u8, 194u8, 231u8, 34u8, 207u8, 19u8, 149u8, 187u8, 3u8,
							176u8, 194u8, 240u8, 180u8, 169u8, 214u8, 194u8, 202u8, 240u8, 209u8,
							6u8, 244u8, 46u8, 54u8, 142u8, 61u8, 220u8, 240u8, 96u8, 10u8, 168u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " `PalletId` for the crowdloan pallet. An appropriate value could be `PalletId(*b\"py/cfund\")`"]
				pub fn pallet_id(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::frame_support::PalletId>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Crowdloan",
						"PalletId",
						[
							139u8, 109u8, 228u8, 151u8, 252u8, 32u8, 130u8, 69u8, 112u8, 154u8,
							174u8, 45u8, 83u8, 245u8, 51u8, 132u8, 173u8, 5u8, 186u8, 24u8, 243u8,
							9u8, 12u8, 214u8, 80u8, 74u8, 69u8, 189u8, 30u8, 94u8, 22u8, 39u8,
						],
					)
				}
				#[doc = " The minimum amount that may be contributed into a crowdloan. Should almost certainly be at"]
				#[doc = " least `ExistentialDeposit`."]
				pub fn min_contribution(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Crowdloan",
						"MinContribution",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " Max number of storage keys to remove per extrinsic call."]
				pub fn remove_keys_limit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Crowdloan",
						"RemoveKeysLimit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod slots {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceLease {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
				pub leaser: ::subxt::ext::sp_core::crypto::AccountId32,
				pub amount: ::core::primitive::u128,
				pub period_begin: ::core::primitive::u32,
				pub period_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ClearAllLeases {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TriggerOnboard {
				pub para: runtime_types::polkadot_parachain::primitives::Id,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Just a connect into the `lease_out` call, in case Root wants to force some lease to happen"]
				#[doc = "independently of any other on-chain mechanism to use it."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
				pub fn force_lease(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
					leaser: ::subxt::ext::sp_core::crypto::AccountId32,
					amount: ::core::primitive::u128,
					period_begin: ::core::primitive::u32,
					period_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<ForceLease> {
					::subxt::tx::StaticTxPayload::new(
						"Slots",
						"force_lease",
						ForceLease { para, leaser, amount, period_begin, period_count },
						[
							196u8, 2u8, 63u8, 229u8, 18u8, 134u8, 48u8, 4u8, 165u8, 46u8, 173u8,
							0u8, 189u8, 35u8, 99u8, 84u8, 103u8, 124u8, 233u8, 246u8, 60u8, 172u8,
							181u8, 205u8, 154u8, 164u8, 36u8, 178u8, 60u8, 164u8, 166u8, 21u8,
						],
					)
				}
				#[doc = "Clear all leases for a Para Id, refunding any deposits back to the original owners."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
				pub fn clear_all_leases(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<ClearAllLeases> {
					::subxt::tx::StaticTxPayload::new(
						"Slots",
						"clear_all_leases",
						ClearAllLeases { para },
						[
							16u8, 14u8, 185u8, 45u8, 149u8, 70u8, 177u8, 133u8, 130u8, 173u8,
							196u8, 244u8, 77u8, 63u8, 218u8, 64u8, 108u8, 83u8, 84u8, 184u8, 175u8,
							122u8, 36u8, 115u8, 146u8, 117u8, 132u8, 82u8, 2u8, 144u8, 62u8, 179u8,
						],
					)
				}
				#[doc = "Try to onboard a parachain that has a lease for the current lease period."]
				#[doc = ""]
				#[doc = "This function can be useful if there was some state issue with a para that should"]
				#[doc = "have onboarded, but was unable to. As long as they have a lease period, we can"]
				#[doc = "let them onboard from here."]
				#[doc = ""]
				#[doc = "Origin must be signed, but can be called by anyone."]
				pub fn trigger_onboard(
					&self,
					para: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<TriggerOnboard> {
					::subxt::tx::StaticTxPayload::new(
						"Slots",
						"trigger_onboard",
						TriggerOnboard { para },
						[
							74u8, 158u8, 122u8, 37u8, 34u8, 62u8, 61u8, 218u8, 94u8, 222u8, 1u8,
							153u8, 131u8, 215u8, 157u8, 180u8, 98u8, 130u8, 151u8, 179u8, 22u8,
							120u8, 32u8, 207u8, 208u8, 46u8, 248u8, 43u8, 154u8, 118u8, 106u8, 2u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_common::slots::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "A new `[lease_period]` is beginning."]
			pub struct NewLeasePeriod {
				pub lease_period: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for NewLeasePeriod {
				const PALLET: &'static str = "Slots";
				const EVENT: &'static str = "NewLeasePeriod";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A para has won the right to a continuous set of lease periods as a parachain."]
			#[doc = "First balance is any extra amount reserved on top of the para's existing deposit."]
			#[doc = "Second balance is the total amount reserved."]
			pub struct Leased {
				pub para_id: runtime_types::polkadot_parachain::primitives::Id,
				pub leaser: ::subxt::ext::sp_core::crypto::AccountId32,
				pub period_begin: ::core::primitive::u32,
				pub period_count: ::core::primitive::u32,
				pub extra_reserved: ::core::primitive::u128,
				pub total_amount: ::core::primitive::u128,
			}
			impl ::subxt::events::StaticEvent for Leased {
				const PALLET: &'static str = "Slots";
				const EVENT: &'static str = "Leased";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Amounts held on deposit for each (possibly future) leased parachain."]
				#[doc = ""]
				#[doc = " The actual amount locked on its behalf by any account at any time is the maximum of the second values"]
				#[doc = " of the items in this list whose first value is the account."]
				#[doc = ""]
				#[doc = " The first item in the list is the amount locked for the current Lease Period. Following"]
				#[doc = " items are for the subsequent lease periods."]
				#[doc = ""]
				#[doc = " The default value (an empty list) implies that the parachain no longer exists (or never"]
				#[doc = " existed) as far as this pallet is concerned."]
				#[doc = ""]
				#[doc = " If a parachain doesn't exist *yet* but is scheduled to exist in the future, then it"]
				#[doc = " will be left-padded with one or more `None`s to denote the fact that nothing is held on"]
				#[doc = " deposit for the non-existent chain currently, but is held at some point in the future."]
				#[doc = ""]
				#[doc = " It is illegal for a `None` value to trail in the list."]
				pub fn leases(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							::core::option::Option<(
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u128,
							)>,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Slots",
						"Leases",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							7u8, 104u8, 17u8, 66u8, 157u8, 89u8, 238u8, 38u8, 233u8, 241u8, 110u8,
							67u8, 132u8, 101u8, 243u8, 62u8, 73u8, 7u8, 9u8, 172u8, 22u8, 51u8,
							118u8, 87u8, 3u8, 224u8, 120u8, 88u8, 139u8, 11u8, 96u8, 147u8,
						],
					)
				}
				#[doc = " Amounts held on deposit for each (possibly future) leased parachain."]
				#[doc = ""]
				#[doc = " The actual amount locked on its behalf by any account at any time is the maximum of the second values"]
				#[doc = " of the items in this list whose first value is the account."]
				#[doc = ""]
				#[doc = " The first item in the list is the amount locked for the current Lease Period. Following"]
				#[doc = " items are for the subsequent lease periods."]
				#[doc = ""]
				#[doc = " The default value (an empty list) implies that the parachain no longer exists (or never"]
				#[doc = " existed) as far as this pallet is concerned."]
				#[doc = ""]
				#[doc = " If a parachain doesn't exist *yet* but is scheduled to exist in the future, then it"]
				#[doc = " will be left-padded with one or more `None`s to denote the fact that nothing is held on"]
				#[doc = " deposit for the non-existent chain currently, but is held at some point in the future."]
				#[doc = ""]
				#[doc = " It is illegal for a `None` value to trail in the list."]
				pub fn leases_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<
							::core::option::Option<(
								::subxt::ext::sp_core::crypto::AccountId32,
								::core::primitive::u128,
							)>,
						>,
					>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Slots",
						"Leases",
						Vec::new(),
						[
							7u8, 104u8, 17u8, 66u8, 157u8, 89u8, 238u8, 38u8, 233u8, 241u8, 110u8,
							67u8, 132u8, 101u8, 243u8, 62u8, 73u8, 7u8, 9u8, 172u8, 22u8, 51u8,
							118u8, 87u8, 3u8, 224u8, 120u8, 88u8, 139u8, 11u8, 96u8, 147u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The number of blocks over which a single period lasts."]
				pub fn lease_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Slots",
						"LeasePeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The number of blocks to offset each lease period by."]
				pub fn lease_offset(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Slots",
						"LeaseOffset",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod paras_sudo_wrapper {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoScheduleParaInitialize {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
				pub genesis: runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoScheduleParaCleanup {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoScheduleParathreadUpgrade {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoScheduleParachainDowngrade {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoQueueDownwardXcm {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
				pub xcm: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoEstablishHrmpChannel {
				pub sender: runtime_types::polkadot_parachain::primitives::Id,
				pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				pub max_capacity: ::core::primitive::u32,
				pub max_message_size: ::core::primitive::u32,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Schedule a para to be initialized at the start of the next session."]
				pub fn sudo_schedule_para_initialize(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
					genesis: runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
				) -> ::subxt::tx::StaticTxPayload<SudoScheduleParaInitialize> {
					::subxt::tx::StaticTxPayload::new(
						"ParasSudoWrapper",
						"sudo_schedule_para_initialize",
						SudoScheduleParaInitialize { id, genesis },
						[
							82u8, 79u8, 183u8, 27u8, 43u8, 32u8, 178u8, 196u8, 23u8, 144u8, 25u8,
							231u8, 172u8, 118u8, 60u8, 146u8, 209u8, 98u8, 206u8, 241u8, 67u8,
							210u8, 149u8, 75u8, 191u8, 164u8, 80u8, 186u8, 249u8, 1u8, 155u8,
							235u8,
						],
					)
				}
				#[doc = "Schedule a para to be cleaned up at the start of the next session."]
				pub fn sudo_schedule_para_cleanup(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<SudoScheduleParaCleanup> {
					::subxt::tx::StaticTxPayload::new(
						"ParasSudoWrapper",
						"sudo_schedule_para_cleanup",
						SudoScheduleParaCleanup { id },
						[
							243u8, 249u8, 108u8, 82u8, 119u8, 200u8, 221u8, 147u8, 17u8, 191u8,
							73u8, 108u8, 149u8, 254u8, 32u8, 114u8, 151u8, 89u8, 127u8, 39u8,
							179u8, 69u8, 39u8, 211u8, 109u8, 237u8, 61u8, 87u8, 251u8, 89u8, 238u8,
							226u8,
						],
					)
				}
				#[doc = "Upgrade a parathread to a parachain"]
				pub fn sudo_schedule_parathread_upgrade(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<SudoScheduleParathreadUpgrade> {
					::subxt::tx::StaticTxPayload::new(
						"ParasSudoWrapper",
						"sudo_schedule_parathread_upgrade",
						SudoScheduleParathreadUpgrade { id },
						[
							168u8, 153u8, 55u8, 74u8, 132u8, 51u8, 111u8, 167u8, 245u8, 23u8, 94u8,
							223u8, 122u8, 210u8, 93u8, 53u8, 186u8, 145u8, 72u8, 9u8, 5u8, 193u8,
							4u8, 146u8, 175u8, 200u8, 98u8, 111u8, 110u8, 161u8, 122u8, 229u8,
						],
					)
				}
				#[doc = "Downgrade a parachain to a parathread"]
				pub fn sudo_schedule_parachain_downgrade(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<SudoScheduleParachainDowngrade> {
					::subxt::tx::StaticTxPayload::new(
						"ParasSudoWrapper",
						"sudo_schedule_parachain_downgrade",
						SudoScheduleParachainDowngrade { id },
						[
							239u8, 188u8, 61u8, 93u8, 81u8, 236u8, 141u8, 47u8, 250u8, 150u8, 33u8,
							165u8, 84u8, 41u8, 221u8, 228u8, 222u8, 81u8, 159u8, 131u8, 41u8,
							244u8, 160u8, 31u8, 49u8, 179u8, 200u8, 97u8, 67u8, 100u8, 110u8,
							196u8,
						],
					)
				}
				#[doc = "Send a downward XCM to the given para."]
				#[doc = ""]
				#[doc = "The given parachain should exist and the payload should not exceed the preconfigured size"]
				#[doc = "`config.max_downward_message_size`."]
				pub fn sudo_queue_downward_xcm(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
					xcm: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::StaticTxPayload<SudoQueueDownwardXcm> {
					::subxt::tx::StaticTxPayload::new(
						"ParasSudoWrapper",
						"sudo_queue_downward_xcm",
						SudoQueueDownwardXcm { id, xcm: ::std::boxed::Box::new(xcm) },
						[
							35u8, 35u8, 202u8, 106u8, 19u8, 88u8, 79u8, 146u8, 193u8, 245u8, 175u8,
							244u8, 93u8, 192u8, 146u8, 140u8, 182u8, 182u8, 43u8, 129u8, 92u8,
							230u8, 240u8, 31u8, 97u8, 34u8, 41u8, 5u8, 89u8, 104u8, 40u8, 49u8,
						],
					)
				}
				#[doc = "Forcefully establish a channel from the sender to the recipient."]
				#[doc = ""]
				#[doc = "This is equivalent to sending an `Hrmp::hrmp_init_open_channel` extrinsic followed by"]
				#[doc = "`Hrmp::hrmp_accept_open_channel`."]
				pub fn sudo_establish_hrmp_channel(
					&self,
					sender: runtime_types::polkadot_parachain::primitives::Id,
					recipient: runtime_types::polkadot_parachain::primitives::Id,
					max_capacity: ::core::primitive::u32,
					max_message_size: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SudoEstablishHrmpChannel> {
					::subxt::tx::StaticTxPayload::new(
						"ParasSudoWrapper",
						"sudo_establish_hrmp_channel",
						SudoEstablishHrmpChannel {
							sender,
							recipient,
							max_capacity,
							max_message_size,
						},
						[
							46u8, 69u8, 133u8, 33u8, 98u8, 44u8, 27u8, 190u8, 248u8, 70u8, 254u8,
							61u8, 67u8, 234u8, 122u8, 6u8, 192u8, 147u8, 170u8, 221u8, 89u8, 154u8,
							90u8, 53u8, 48u8, 137u8, 118u8, 207u8, 213u8, 240u8, 197u8, 87u8,
						],
					)
				}
			}
		}
	}
	pub mod assigned_slots {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AssignPermParachainSlot {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AssignTempParachainSlot {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
				pub lease_period_start:
					runtime_types::polkadot_runtime_common::assigned_slots::SlotLeasePeriodStart,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct UnassignParachainSlot {
				pub id: runtime_types::polkadot_parachain::primitives::Id,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Assign a permanent parachain slot and immediately create a lease for it."]
				pub fn assign_perm_parachain_slot(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<AssignPermParachainSlot> {
					::subxt::tx::StaticTxPayload::new(
						"AssignedSlots",
						"assign_perm_parachain_slot",
						AssignPermParachainSlot { id },
						[
							118u8, 248u8, 251u8, 88u8, 24u8, 122u8, 7u8, 247u8, 54u8, 81u8, 137u8,
							86u8, 153u8, 151u8, 188u8, 9u8, 186u8, 83u8, 253u8, 45u8, 135u8, 149u8,
							51u8, 60u8, 81u8, 147u8, 63u8, 218u8, 140u8, 207u8, 244u8, 165u8,
						],
					)
				}
				#[doc = "Assign a temporary parachain slot. The function tries to create a lease for it"]
				#[doc = "immediately if `SlotLeasePeriodStart::Current` is specified, and if the number"]
				#[doc = "of currently active temporary slots is below `MaxTemporarySlotPerLeasePeriod`."]
				pub fn assign_temp_parachain_slot(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
					lease_period_start : runtime_types :: polkadot_runtime_common :: assigned_slots :: SlotLeasePeriodStart,
				) -> ::subxt::tx::StaticTxPayload<AssignTempParachainSlot> {
					::subxt::tx::StaticTxPayload::new(
						"AssignedSlots",
						"assign_temp_parachain_slot",
						AssignTempParachainSlot { id, lease_period_start },
						[
							10u8, 235u8, 166u8, 173u8, 8u8, 149u8, 199u8, 31u8, 9u8, 134u8, 134u8,
							13u8, 252u8, 177u8, 47u8, 9u8, 189u8, 178u8, 6u8, 141u8, 202u8, 100u8,
							232u8, 94u8, 75u8, 26u8, 26u8, 202u8, 84u8, 143u8, 220u8, 13u8,
						],
					)
				}
				#[doc = "Unassign a permanent or temporary parachain slot"]
				pub fn unassign_parachain_slot(
					&self,
					id: runtime_types::polkadot_parachain::primitives::Id,
				) -> ::subxt::tx::StaticTxPayload<UnassignParachainSlot> {
					::subxt::tx::StaticTxPayload::new(
						"AssignedSlots",
						"unassign_parachain_slot",
						UnassignParachainSlot { id },
						[
							121u8, 135u8, 19u8, 137u8, 54u8, 151u8, 163u8, 155u8, 172u8, 37u8,
							188u8, 214u8, 106u8, 12u8, 14u8, 230u8, 2u8, 114u8, 141u8, 217u8, 24u8,
							145u8, 253u8, 179u8, 41u8, 9u8, 95u8, 128u8, 141u8, 190u8, 151u8, 29u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::polkadot_runtime_common::assigned_slots::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A para was assigned a permanent parachain slot"]
			pub struct PermanentSlotAssigned(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for PermanentSlotAssigned {
				const PALLET: &'static str = "AssignedSlots";
				const EVENT: &'static str = "PermanentSlotAssigned";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A para was assigned a temporary parachain slot"]
			pub struct TemporarySlotAssigned(pub runtime_types::polkadot_parachain::primitives::Id);
			impl ::subxt::events::StaticEvent for TemporarySlotAssigned {
				const PALLET: &'static str = "AssignedSlots";
				const EVENT: &'static str = "TemporarySlotAssigned";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Assigned permanent slots, with their start lease period, and duration."]
				pub fn permanent_slots(
					&self,
					_0: impl ::std::borrow::Borrow<runtime_types::polkadot_parachain::primitives::Id>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssignedSlots",
						"PermanentSlots",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							123u8, 228u8, 93u8, 2u8, 196u8, 127u8, 156u8, 106u8, 36u8, 133u8,
							137u8, 229u8, 137u8, 116u8, 107u8, 143u8, 47u8, 88u8, 65u8, 26u8,
							104u8, 15u8, 36u8, 128u8, 189u8, 108u8, 243u8, 37u8, 11u8, 171u8,
							147u8, 9u8,
						],
					)
				}
				#[doc = " Assigned permanent slots, with their start lease period, and duration."]
				pub fn permanent_slots_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u32,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssignedSlots",
						"PermanentSlots",
						Vec::new(),
						[
							123u8, 228u8, 93u8, 2u8, 196u8, 127u8, 156u8, 106u8, 36u8, 133u8,
							137u8, 229u8, 137u8, 116u8, 107u8, 143u8, 47u8, 88u8, 65u8, 26u8,
							104u8, 15u8, 36u8, 128u8, 189u8, 108u8, 243u8, 37u8, 11u8, 171u8,
							147u8, 9u8,
						],
					)
				}
				#[doc = " Number of assigned (and active) permanent slots."]
				pub fn permanent_slot_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssignedSlots",
						"PermanentSlotCount",
						vec![],
						[
							186u8, 224u8, 144u8, 167u8, 64u8, 193u8, 68u8, 25u8, 146u8, 86u8,
							109u8, 81u8, 100u8, 197u8, 25u8, 4u8, 27u8, 131u8, 162u8, 7u8, 148u8,
							198u8, 162u8, 100u8, 197u8, 86u8, 37u8, 43u8, 240u8, 25u8, 18u8, 66u8,
						],
					)
				}
				#[doc = " Assigned temporary slots."]				pub fn temporary_slots (& self , _0 : impl :: std :: borrow :: Borrow < runtime_types :: polkadot_parachain :: primitives :: Id > ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: polkadot_runtime_common :: assigned_slots :: ParachainTemporarySlot < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , :: subxt :: storage :: address :: Yes , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"AssignedSlots",
						"TemporarySlots",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							239u8, 224u8, 55u8, 181u8, 165u8, 130u8, 114u8, 123u8, 255u8, 11u8,
							248u8, 69u8, 243u8, 159u8, 72u8, 174u8, 138u8, 154u8, 11u8, 247u8,
							80u8, 216u8, 175u8, 190u8, 49u8, 138u8, 246u8, 66u8, 221u8, 61u8, 18u8,
							101u8,
						],
					)
				}
				#[doc = " Assigned temporary slots."]				pub fn temporary_slots_root (& self ,) -> :: subxt :: storage :: address :: StaticStorageAddress :: < :: subxt :: metadata :: DecodeStaticType < runtime_types :: polkadot_runtime_common :: assigned_slots :: ParachainTemporarySlot < :: subxt :: ext :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u32 > > , () , () , :: subxt :: storage :: address :: Yes >{
					::subxt::storage::address::StaticStorageAddress::new(
						"AssignedSlots",
						"TemporarySlots",
						Vec::new(),
						[
							239u8, 224u8, 55u8, 181u8, 165u8, 130u8, 114u8, 123u8, 255u8, 11u8,
							248u8, 69u8, 243u8, 159u8, 72u8, 174u8, 138u8, 154u8, 11u8, 247u8,
							80u8, 216u8, 175u8, 190u8, 49u8, 138u8, 246u8, 66u8, 221u8, 61u8, 18u8,
							101u8,
						],
					)
				}
				#[doc = " Number of assigned temporary slots."]
				pub fn temporary_slot_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssignedSlots",
						"TemporarySlotCount",
						vec![],
						[
							19u8, 243u8, 53u8, 131u8, 195u8, 143u8, 31u8, 224u8, 182u8, 69u8,
							209u8, 123u8, 82u8, 155u8, 96u8, 242u8, 109u8, 6u8, 27u8, 193u8, 251u8,
							45u8, 204u8, 10u8, 43u8, 185u8, 152u8, 181u8, 35u8, 183u8, 235u8,
							204u8,
						],
					)
				}
				#[doc = " Number of active temporary slots in current slot lease period."]
				pub fn active_temporary_slot_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"AssignedSlots",
						"ActiveTemporarySlotCount",
						vec![],
						[
							72u8, 42u8, 13u8, 42u8, 195u8, 143u8, 174u8, 137u8, 110u8, 144u8,
							190u8, 117u8, 102u8, 91u8, 66u8, 131u8, 69u8, 139u8, 156u8, 149u8,
							99u8, 177u8, 118u8, 72u8, 168u8, 191u8, 198u8, 135u8, 72u8, 192u8,
							130u8, 139u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The number of lease periods a permanent parachain slot lasts."]
				pub fn permanent_slot_lease_period_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"AssignedSlots",
						"PermanentSlotLeasePeriodLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The number of lease periods a temporary parachain slot lasts."]
				pub fn temporary_slot_lease_period_length(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"AssignedSlots",
						"TemporarySlotLeasePeriodLength",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The max number of permanent slots that can be assigned."]
				pub fn max_permanent_slots(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"AssignedSlots",
						"MaxPermanentSlots",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The max number of temporary slots that can be assigned."]
				pub fn max_temporary_slots(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"AssignedSlots",
						"MaxTemporarySlots",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The max number of temporary slots to be scheduled per lease periods."]
				pub fn max_temporary_slot_per_lease_period(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"AssignedSlots",
						"MaxTemporarySlotPerLeasePeriod",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod sudo {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Sudo {
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoUncheckedWeight {
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
				pub weight: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetKey {
				pub new: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SudoAs {
				pub who: ::subxt::ext::sp_runtime::MultiAddress<
					::subxt::ext::sp_core::crypto::AccountId32,
					(),
				>,
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
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
					call: runtime_types::rococo_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<Sudo> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo",
						Sudo { call: ::std::boxed::Box::new(call) },
						[
							81u8, 115u8, 18u8, 99u8, 243u8, 111u8, 210u8, 37u8, 183u8, 240u8, 51u8,
							75u8, 216u8, 84u8, 249u8, 119u8, 216u8, 21u8, 148u8, 57u8, 77u8, 190u8,
							54u8, 194u8, 146u8, 116u8, 15u8, 76u8, 62u8, 248u8, 72u8, 206u8,
						],
					)
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
					call: runtime_types::rococo_runtime::Call,
					weight: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<SudoUncheckedWeight> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_unchecked_weight",
						SudoUncheckedWeight { call: ::std::boxed::Box::new(call), weight },
						[
							237u8, 32u8, 87u8, 79u8, 89u8, 212u8, 128u8, 70u8, 244u8, 137u8, 73u8,
							135u8, 201u8, 86u8, 45u8, 104u8, 106u8, 78u8, 196u8, 43u8, 129u8,
							177u8, 139u8, 120u8, 224u8, 123u8, 23u8, 224u8, 166u8, 246u8, 68u8,
							66u8,
						],
					)
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
					new: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
				) -> ::subxt::tx::StaticTxPayload<SetKey> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"set_key",
						SetKey { new },
						[
							23u8, 224u8, 218u8, 169u8, 8u8, 28u8, 111u8, 199u8, 26u8, 88u8, 225u8,
							105u8, 17u8, 19u8, 87u8, 156u8, 97u8, 67u8, 89u8, 173u8, 70u8, 0u8,
							5u8, 246u8, 198u8, 135u8, 182u8, 180u8, 44u8, 9u8, 212u8, 95u8,
						],
					)
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
					who: ::subxt::ext::sp_runtime::MultiAddress<
						::subxt::ext::sp_core::crypto::AccountId32,
						(),
					>,
					call: runtime_types::rococo_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<SudoAs> {
					::subxt::tx::StaticTxPayload::new(
						"Sudo",
						"sudo_as",
						SudoAs { who, call: ::std::boxed::Box::new(call) },
						[
							142u8, 87u8, 169u8, 202u8, 210u8, 186u8, 231u8, 195u8, 48u8, 192u8,
							19u8, 18u8, 131u8, 187u8, 104u8, 212u8, 3u8, 203u8, 10u8, 145u8, 215u8,
							124u8, 135u8, 154u8, 120u8, 0u8, 174u8, 178u8, 141u8, 150u8, 61u8,
							46u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_sudo::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct Sudid {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Sudid {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "Sudid";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
			pub struct KeyChanged {
				pub old_sudoer: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A sudo just took place. \\[result\\]"]
			pub struct SudoAsDone {
				pub sudo_result:
					::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for SudoAsDone {
				const PALLET: &'static str = "Sudo";
				const EVENT: &'static str = "SudoAsDone";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The `AccountId` of the sudo key."]
				pub fn key(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Sudo",
						"Key",
						vec![],
						[
							244u8, 73u8, 188u8, 136u8, 218u8, 163u8, 68u8, 179u8, 122u8, 173u8,
							34u8, 108u8, 137u8, 28u8, 182u8, 16u8, 196u8, 92u8, 138u8, 34u8, 102u8,
							80u8, 199u8, 88u8, 107u8, 207u8, 36u8, 22u8, 168u8, 167u8, 20u8, 142u8,
						],
					)
				}
			}
		}
	}
	pub mod mmr {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Latest MMR Root hash."]
				pub fn root_hash(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"RootHash",
						vec![],
						[
							182u8, 163u8, 37u8, 44u8, 2u8, 163u8, 57u8, 184u8, 97u8, 55u8, 1u8,
							116u8, 55u8, 169u8, 23u8, 221u8, 182u8, 5u8, 174u8, 217u8, 111u8, 55u8,
							180u8, 161u8, 69u8, 120u8, 212u8, 73u8, 2u8, 1u8, 39u8, 224u8,
						],
					)
				}
				#[doc = " Current size of the MMR (number of leaves)."]
				pub fn number_of_leaves(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"NumberOfLeaves",
						vec![],
						[
							138u8, 124u8, 23u8, 186u8, 255u8, 231u8, 187u8, 122u8, 213u8, 160u8,
							29u8, 24u8, 88u8, 98u8, 171u8, 36u8, 195u8, 216u8, 27u8, 190u8, 192u8,
							152u8, 8u8, 13u8, 210u8, 232u8, 45u8, 184u8, 240u8, 255u8, 156u8,
							204u8,
						],
					)
				}
				#[doc = " Hashes of the nodes in the MMR."]
				#[doc = ""]
				#[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
				#[doc = " are pruned and only stored in the Offchain DB."]
				pub fn nodes(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"Nodes",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							188u8, 148u8, 126u8, 226u8, 142u8, 91u8, 61u8, 52u8, 213u8, 36u8,
							120u8, 232u8, 20u8, 11u8, 61u8, 1u8, 130u8, 155u8, 81u8, 34u8, 153u8,
							149u8, 210u8, 232u8, 113u8, 242u8, 249u8, 8u8, 61u8, 51u8, 148u8, 98u8,
						],
					)
				}
				#[doc = " Hashes of the nodes in the MMR."]
				#[doc = ""]
				#[doc = " Note this collection only contains MMR peaks, the inner nodes (and leaves)"]
				#[doc = " are pruned and only stored in the Offchain DB."]
				pub fn nodes_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::H256>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Mmr",
						"Nodes",
						Vec::new(),
						[
							188u8, 148u8, 126u8, 226u8, 142u8, 91u8, 61u8, 52u8, 213u8, 36u8,
							120u8, 232u8, 20u8, 11u8, 61u8, 1u8, 130u8, 155u8, 81u8, 34u8, 153u8,
							149u8, 210u8, 232u8, 113u8, 242u8, 249u8, 8u8, 61u8, 51u8, 148u8, 98u8,
						],
					)
				}
			}
		}
	}
	pub mod beefy {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current authorities set"]
				pub fn authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::beefy_primitives::crypto::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Beefy",
						"Authorities",
						vec![],
						[
							180u8, 103u8, 249u8, 204u8, 109u8, 0u8, 211u8, 102u8, 59u8, 184u8,
							31u8, 52u8, 140u8, 248u8, 10u8, 127u8, 19u8, 50u8, 254u8, 116u8, 124u8,
							5u8, 94u8, 42u8, 9u8, 138u8, 159u8, 94u8, 26u8, 136u8, 236u8, 141u8,
						],
					)
				}
				#[doc = " The current validator set id"]
				pub fn validator_set_id(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Beefy",
						"ValidatorSetId",
						vec![],
						[
							132u8, 47u8, 139u8, 239u8, 214u8, 179u8, 24u8, 63u8, 55u8, 154u8,
							248u8, 206u8, 73u8, 7u8, 52u8, 135u8, 54u8, 111u8, 250u8, 106u8, 71u8,
							78u8, 44u8, 44u8, 235u8, 177u8, 36u8, 112u8, 17u8, 122u8, 252u8, 80u8,
						],
					)
				}
				#[doc = " Authorities set scheduled to be used with the next session"]
				pub fn next_authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::beefy_primitives::crypto::Public,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Beefy",
						"NextAuthorities",
						vec![],
						[
							64u8, 174u8, 216u8, 177u8, 95u8, 133u8, 175u8, 16u8, 171u8, 110u8, 7u8,
							244u8, 111u8, 89u8, 57u8, 46u8, 52u8, 28u8, 150u8, 117u8, 156u8, 47u8,
							91u8, 135u8, 196u8, 102u8, 46u8, 4u8, 224u8, 155u8, 83u8, 36u8,
						],
					)
				}
			}
		}
	}
	pub mod mmr_leaf {
		use super::{root_mod, runtime_types};
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Details of current BEEFY authority set."]
				pub fn beefy_authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::beefy_primitives::mmr::BeefyAuthoritySet<
							::subxt::ext::sp_core::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"MmrLeaf",
						"BeefyAuthorities",
						vec![],
						[
							238u8, 154u8, 245u8, 133u8, 41u8, 170u8, 91u8, 75u8, 59u8, 169u8,
							160u8, 202u8, 204u8, 13u8, 89u8, 0u8, 153u8, 166u8, 54u8, 255u8, 64u8,
							63u8, 164u8, 33u8, 4u8, 193u8, 79u8, 231u8, 10u8, 95u8, 40u8, 86u8,
						],
					)
				}
				#[doc = " Details of next BEEFY authority set."]
				#[doc = ""]
				#[doc = " This storage entry is used as cache for calls to `update_beefy_next_authority_set`."]
				pub fn beefy_next_authorities(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::beefy_primitives::mmr::BeefyAuthoritySet<
							::subxt::ext::sp_core::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"MmrLeaf",
						"BeefyNextAuthorities",
						vec![],
						[
							39u8, 40u8, 15u8, 157u8, 20u8, 100u8, 124u8, 98u8, 13u8, 243u8, 221u8,
							133u8, 245u8, 210u8, 99u8, 159u8, 240u8, 158u8, 33u8, 140u8, 142u8,
							216u8, 86u8, 227u8, 42u8, 224u8, 148u8, 200u8, 70u8, 105u8, 87u8,
							155u8,
						],
					)
				}
			}
		}
	}
	pub mod validator_manager {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RegisterValidators {
				pub validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct DeregisterValidators {
				pub validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Add new validators to the set."]
				#[doc = ""]
				#[doc = "The new validators will be active from current session + 2."]
				pub fn register_validators(
					&self,
					validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<RegisterValidators> {
					::subxt::tx::StaticTxPayload::new(
						"ValidatorManager",
						"register_validators",
						RegisterValidators { validators },
						[
							17u8, 237u8, 46u8, 131u8, 162u8, 213u8, 36u8, 137u8, 92u8, 108u8,
							181u8, 49u8, 13u8, 232u8, 79u8, 39u8, 80u8, 200u8, 88u8, 168u8, 16u8,
							239u8, 53u8, 255u8, 155u8, 176u8, 130u8, 69u8, 19u8, 39u8, 48u8, 214u8,
						],
					)
				}
				#[doc = "Remove validators from the set."]
				#[doc = ""]
				#[doc = "The removed validators will be deactivated from current session + 2."]
				pub fn deregister_validators(
					&self,
					validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<DeregisterValidators> {
					::subxt::tx::StaticTxPayload::new(
						"ValidatorManager",
						"deregister_validators",
						DeregisterValidators { validators },
						[
							174u8, 68u8, 36u8, 38u8, 204u8, 164u8, 127u8, 114u8, 51u8, 193u8, 35u8,
							231u8, 161u8, 11u8, 206u8, 181u8, 117u8, 72u8, 226u8, 175u8, 166u8,
							33u8, 135u8, 192u8, 180u8, 192u8, 98u8, 208u8, 135u8, 249u8, 122u8,
							159u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::rococo_runtime::validator_manager::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "New validators were added to the set."]
			pub struct ValidatorsRegistered(
				pub ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			);
			impl ::subxt::events::StaticEvent for ValidatorsRegistered {
				const PALLET: &'static str = "ValidatorManager";
				const EVENT: &'static str = "ValidatorsRegistered";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Validators were removed from the set."]
			pub struct ValidatorsDeregistered(
				pub ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			);
			impl ::subxt::events::StaticEvent for ValidatorsDeregistered {
				const PALLET: &'static str = "ValidatorManager";
				const EVENT: &'static str = "ValidatorsDeregistered";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " Validators that should be retired, because their Parachain was deregistered."]
				pub fn validators_to_retire(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ValidatorManager",
						"ValidatorsToRetire",
						vec![],
						[
							174u8, 83u8, 236u8, 203u8, 146u8, 196u8, 48u8, 111u8, 29u8, 182u8,
							114u8, 60u8, 7u8, 134u8, 2u8, 255u8, 1u8, 42u8, 186u8, 222u8, 93u8,
							153u8, 108u8, 35u8, 1u8, 91u8, 197u8, 144u8, 31u8, 81u8, 67u8, 136u8,
						],
					)
				}
				#[doc = " Validators that should be added."]
				pub fn validators_to_add(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"ValidatorManager",
						"ValidatorsToAdd",
						vec![],
						[
							244u8, 237u8, 251u8, 6u8, 157u8, 59u8, 227u8, 61u8, 240u8, 204u8, 12u8,
							87u8, 118u8, 12u8, 61u8, 103u8, 194u8, 128u8, 7u8, 67u8, 218u8, 129u8,
							106u8, 33u8, 135u8, 95u8, 45u8, 208u8, 42u8, 99u8, 83u8, 69u8,
						],
					)
				}
			}
		}
	}
	pub mod collective {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetMembers {
				pub new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
				pub old_count: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Execute {
				pub proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Propose {
				#[codec(compact)]
				pub threshold: ::core::primitive::u32,
				pub proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Vote {
				pub proposal: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				pub approve: ::core::primitive::bool,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Close {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				#[codec(compact)]
				pub index: ::core::primitive::u32,
				#[codec(compact)]
				pub proposal_weight_bound: ::core::primitive::u64,
				#[codec(compact)]
				pub length_bound: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct DisapproveProposal {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			pub struct TransactionApi;
			impl TransactionApi {
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
					new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					old_count: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<SetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"Collective",
						"set_members",
						SetMembers { new_members, prime, old_count },
						[
							196u8, 103u8, 123u8, 125u8, 226u8, 177u8, 126u8, 37u8, 160u8, 114u8,
							34u8, 136u8, 219u8, 84u8, 199u8, 94u8, 242u8, 20u8, 126u8, 126u8,
							166u8, 190u8, 198u8, 33u8, 162u8, 113u8, 237u8, 222u8, 90u8, 1u8, 2u8,
							234u8,
						],
					)
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
					proposal: runtime_types::rococo_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"Collective",
						"execute",
						Execute { proposal: ::std::boxed::Box::new(proposal), length_bound },
						[
							197u8, 37u8, 92u8, 133u8, 36u8, 138u8, 26u8, 241u8, 66u8, 144u8, 148u8,
							150u8, 191u8, 241u8, 25u8, 137u8, 234u8, 47u8, 249u8, 82u8, 199u8,
							123u8, 19u8, 164u8, 116u8, 162u8, 61u8, 169u8, 221u8, 44u8, 81u8,
							251u8,
						],
					)
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
					proposal: runtime_types::rococo_runtime::Call,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Propose> {
					::subxt::tx::StaticTxPayload::new(
						"Collective",
						"propose",
						Propose {
							threshold,
							proposal: ::std::boxed::Box::new(proposal),
							length_bound,
						},
						[
							122u8, 27u8, 77u8, 39u8, 193u8, 56u8, 42u8, 219u8, 26u8, 179u8, 133u8,
							125u8, 26u8, 34u8, 195u8, 84u8, 146u8, 255u8, 10u8, 165u8, 67u8, 198u8,
							216u8, 5u8, 148u8, 90u8, 138u8, 80u8, 135u8, 94u8, 48u8, 113u8,
						],
					)
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
					proposal: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
					approve: ::core::primitive::bool,
				) -> ::subxt::tx::StaticTxPayload<Vote> {
					::subxt::tx::StaticTxPayload::new(
						"Collective",
						"vote",
						Vote { proposal, index, approve },
						[
							108u8, 46u8, 180u8, 148u8, 145u8, 24u8, 173u8, 56u8, 36u8, 100u8,
							216u8, 43u8, 178u8, 202u8, 26u8, 136u8, 93u8, 84u8, 80u8, 134u8, 14u8,
							42u8, 248u8, 205u8, 68u8, 92u8, 79u8, 11u8, 113u8, 115u8, 157u8, 100u8,
						],
					)
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
					proposal_hash: ::subxt::ext::sp_core::H256,
					index: ::core::primitive::u32,
					proposal_weight_bound: ::core::primitive::u64,
					length_bound: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<Close> {
					::subxt::tx::StaticTxPayload::new(
						"Collective",
						"close",
						Close { proposal_hash, index, proposal_weight_bound, length_bound },
						[
							88u8, 8u8, 33u8, 184u8, 4u8, 97u8, 120u8, 237u8, 43u8, 183u8, 130u8,
							139u8, 65u8, 74u8, 166u8, 119u8, 246u8, 65u8, 132u8, 219u8, 118u8,
							69u8, 182u8, 195u8, 111u8, 204u8, 107u8, 78u8, 152u8, 218u8, 181u8,
							208u8,
						],
					)
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
					proposal_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<DisapproveProposal> {
					::subxt::tx::StaticTxPayload::new(
						"Collective",
						"disapprove_proposal",
						DisapproveProposal { proposal_hash },
						[
							25u8, 123u8, 1u8, 8u8, 74u8, 37u8, 3u8, 40u8, 97u8, 37u8, 175u8, 224u8,
							72u8, 155u8, 123u8, 109u8, 104u8, 43u8, 91u8, 125u8, 199u8, 51u8, 17u8,
							225u8, 133u8, 38u8, 120u8, 76u8, 164u8, 5u8, 194u8, 201u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_collective::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
			#[doc = "`MemberCount`)."]
			pub struct Proposed {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proposal_index: ::core::primitive::u32,
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub threshold: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Proposed {
				const PALLET: &'static str = "Collective";
				const EVENT: &'static str = "Proposed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion (given hash) has been voted on by given account, leaving"]
			#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
			pub struct Voted {
				pub account: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub voted: ::core::primitive::bool,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Voted {
				const PALLET: &'static str = "Collective";
				const EVENT: &'static str = "Voted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was approved by the required threshold."]
			pub struct Approved {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Approved {
				const PALLET: &'static str = "Collective";
				const EVENT: &'static str = "Approved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was not approved by the required threshold."]
			pub struct Disapproved {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Disapproved {
				const PALLET: &'static str = "Collective";
				const EVENT: &'static str = "Disapproved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
			pub struct Executed {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for Executed {
				const PALLET: &'static str = "Collective";
				const EVENT: &'static str = "Executed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
			pub struct MemberExecuted {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MemberExecuted {
				const PALLET: &'static str = "Collective";
				const EVENT: &'static str = "MemberExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
			pub struct Closed {
				pub proposal_hash: ::subxt::ext::sp_core::H256,
				pub yes: ::core::primitive::u32,
				pub no: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for Closed {
				const PALLET: &'static str = "Collective";
				const EVENT: &'static str = "Closed";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The hashes of the active proposals."]
				pub fn proposals(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							::subxt::ext::sp_core::H256,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"Proposals",
						vec![],
						[
							10u8, 133u8, 82u8, 54u8, 193u8, 41u8, 253u8, 159u8, 56u8, 96u8, 249u8,
							148u8, 43u8, 57u8, 116u8, 43u8, 222u8, 243u8, 237u8, 231u8, 238u8,
							60u8, 26u8, 225u8, 19u8, 203u8, 213u8, 220u8, 114u8, 217u8, 100u8,
							27u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::rococo_runtime::Call>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"ProposalOf",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							67u8, 105u8, 133u8, 144u8, 202u8, 92u8, 56u8, 137u8, 9u8, 72u8, 215u8,
							81u8, 240u8, 23u8, 179u8, 39u8, 1u8, 121u8, 209u8, 72u8, 198u8, 138u8,
							248u8, 244u8, 135u8, 19u8, 104u8, 14u8, 19u8, 145u8, 142u8, 170u8,
						],
					)
				}
				#[doc = " Actual proposal for a given hash, if it's current."]
				pub fn proposal_of_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<runtime_types::rococo_runtime::Call>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"ProposalOf",
						Vec::new(),
						[
							67u8, 105u8, 133u8, 144u8, 202u8, 92u8, 56u8, 137u8, 9u8, 72u8, 215u8,
							81u8, 240u8, 23u8, 179u8, 39u8, 1u8, 121u8, 209u8, 72u8, 198u8, 138u8,
							248u8, 244u8, 135u8, 19u8, 104u8, 14u8, 19u8, 145u8, 142u8, 170u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"Voting",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Votes on a given proposal, if it is ongoing."]
				pub fn voting_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_collective::Votes<
							::subxt::ext::sp_core::crypto::AccountId32,
							::core::primitive::u32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"Voting",
						Vec::new(),
						[
							89u8, 108u8, 65u8, 58u8, 60u8, 116u8, 54u8, 68u8, 179u8, 73u8, 161u8,
							168u8, 78u8, 213u8, 208u8, 54u8, 244u8, 58u8, 70u8, 209u8, 170u8,
							136u8, 215u8, 3u8, 2u8, 105u8, 229u8, 217u8, 240u8, 230u8, 107u8,
							221u8,
						],
					)
				}
				#[doc = " Proposals so far."]
				pub fn proposal_count(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"ProposalCount",
						vec![],
						[
							132u8, 145u8, 78u8, 218u8, 51u8, 189u8, 55u8, 172u8, 143u8, 33u8,
							140u8, 99u8, 124u8, 208u8, 57u8, 232u8, 154u8, 110u8, 32u8, 142u8,
							24u8, 149u8, 109u8, 105u8, 30u8, 83u8, 39u8, 177u8, 127u8, 160u8, 34u8,
							70u8,
						],
					)
				}
				#[doc = " The current members of the collective. This is stored sorted (just by value)."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"Members",
						vec![],
						[
							162u8, 72u8, 174u8, 204u8, 140u8, 105u8, 205u8, 176u8, 197u8, 117u8,
							206u8, 134u8, 157u8, 110u8, 139u8, 54u8, 43u8, 233u8, 25u8, 51u8, 36u8,
							238u8, 94u8, 124u8, 221u8, 52u8, 237u8, 71u8, 125u8, 56u8, 129u8,
							222u8,
						],
					)
				}
				#[doc = " The prime member that helps determine the default vote behavior in case of absentations."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Collective",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod membership {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddMember {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveMember {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SwapMember {
				pub remove: ::subxt::ext::sp_core::crypto::AccountId32,
				pub add: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ResetMembers {
				pub members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ChangeKey {
				pub new: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SetPrime {
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ClearPrime;
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Add a member `who` to the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::AddOrigin`."]
				pub fn add_member(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<AddMember> {
					::subxt::tx::StaticTxPayload::new(
						"Membership",
						"add_member",
						AddMember { who },
						[
							106u8, 33u8, 171u8, 114u8, 223u8, 105u8, 71u8, 15u8, 77u8, 253u8, 40u8,
							204u8, 244u8, 142u8, 103u8, 177u8, 200u8, 243u8, 114u8, 241u8, 36u8,
							135u8, 175u8, 255u8, 124u8, 193u8, 30u8, 46u8, 186u8, 172u8, 176u8,
							98u8,
						],
					)
				}
				#[doc = "Remove a member `who` from the set."]
				#[doc = ""]
				#[doc = "May only be called from `T::RemoveOrigin`."]
				pub fn remove_member(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<RemoveMember> {
					::subxt::tx::StaticTxPayload::new(
						"Membership",
						"remove_member",
						RemoveMember { who },
						[
							100u8, 17u8, 75u8, 92u8, 58u8, 100u8, 34u8, 187u8, 41u8, 160u8, 137u8,
							58u8, 78u8, 166u8, 161u8, 116u8, 1u8, 67u8, 201u8, 144u8, 103u8, 84u8,
							55u8, 246u8, 133u8, 180u8, 148u8, 86u8, 175u8, 175u8, 70u8, 73u8,
						],
					)
				}
				#[doc = "Swap out one member `remove` for another `add`."]
				#[doc = ""]
				#[doc = "May only be called from `T::SwapOrigin`."]
				#[doc = ""]
				#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
				pub fn swap_member(
					&self,
					remove: ::subxt::ext::sp_core::crypto::AccountId32,
					add: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<SwapMember> {
					::subxt::tx::StaticTxPayload::new(
						"Membership",
						"swap_member",
						SwapMember { remove, add },
						[
							66u8, 84u8, 183u8, 29u8, 104u8, 163u8, 220u8, 217u8, 103u8, 234u8,
							233u8, 138u8, 191u8, 147u8, 51u8, 98u8, 46u8, 51u8, 179u8, 200u8, 23u8,
							59u8, 112u8, 53u8, 8u8, 75u8, 135u8, 232u8, 116u8, 201u8, 60u8, 249u8,
						],
					)
				}
				#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
				#[doc = "pass `members` pre-sorted."]
				#[doc = ""]
				#[doc = "May only be called from `T::ResetOrigin`."]
				pub fn reset_members(
					&self,
					members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::tx::StaticTxPayload<ResetMembers> {
					::subxt::tx::StaticTxPayload::new(
						"Membership",
						"reset_members",
						ResetMembers { members },
						[
							9u8, 35u8, 28u8, 59u8, 158u8, 232u8, 89u8, 78u8, 101u8, 53u8, 240u8,
							98u8, 13u8, 104u8, 235u8, 161u8, 201u8, 150u8, 117u8, 32u8, 75u8,
							209u8, 166u8, 252u8, 57u8, 131u8, 96u8, 215u8, 51u8, 81u8, 42u8, 123u8,
						],
					)
				}
				#[doc = "Swap out the sending member for some other key `new`."]
				#[doc = ""]
				#[doc = "May only be called from `Signed` origin of a current member."]
				#[doc = ""]
				#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
				pub fn change_key(
					&self,
					new: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<ChangeKey> {
					::subxt::tx::StaticTxPayload::new(
						"Membership",
						"change_key",
						ChangeKey { new },
						[
							53u8, 60u8, 54u8, 231u8, 151u8, 0u8, 27u8, 175u8, 250u8, 80u8, 74u8,
							184u8, 184u8, 63u8, 90u8, 216u8, 186u8, 136u8, 74u8, 214u8, 111u8,
							186u8, 137u8, 140u8, 108u8, 194u8, 128u8, 97u8, 168u8, 184u8, 112u8,
							60u8,
						],
					)
				}
				#[doc = "Set the prime member. Must be a current member."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn set_prime(
					&self,
					who: ::subxt::ext::sp_core::crypto::AccountId32,
				) -> ::subxt::tx::StaticTxPayload<SetPrime> {
					::subxt::tx::StaticTxPayload::new(
						"Membership",
						"set_prime",
						SetPrime { who },
						[
							123u8, 95u8, 75u8, 129u8, 19u8, 34u8, 192u8, 65u8, 169u8, 47u8, 184u8,
							246u8, 55u8, 250u8, 31u8, 158u8, 57u8, 197u8, 22u8, 112u8, 167u8,
							198u8, 136u8, 17u8, 15u8, 203u8, 101u8, 149u8, 15u8, 39u8, 16u8, 232u8,
						],
					)
				}
				#[doc = "Remove the prime member if it exists."]
				#[doc = ""]
				#[doc = "May only be called from `T::PrimeOrigin`."]
				pub fn clear_prime(&self) -> ::subxt::tx::StaticTxPayload<ClearPrime> {
					::subxt::tx::StaticTxPayload::new(
						"Membership",
						"clear_prime",
						ClearPrime {},
						[
							186u8, 182u8, 225u8, 90u8, 71u8, 124u8, 69u8, 100u8, 234u8, 25u8, 53u8,
							23u8, 182u8, 32u8, 176u8, 81u8, 54u8, 140u8, 235u8, 126u8, 247u8, 7u8,
							155u8, 62u8, 35u8, 135u8, 48u8, 61u8, 88u8, 160u8, 183u8, 72u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_membership::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was added; see the transaction for who."]
			pub struct MemberAdded;
			impl ::subxt::events::StaticEvent for MemberAdded {
				const PALLET: &'static str = "Membership";
				const EVENT: &'static str = "MemberAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The given member was removed; see the transaction for who."]
			pub struct MemberRemoved;
			impl ::subxt::events::StaticEvent for MemberRemoved {
				const PALLET: &'static str = "Membership";
				const EVENT: &'static str = "MemberRemoved";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Two members were swapped; see the transaction for who."]
			pub struct MembersSwapped;
			impl ::subxt::events::StaticEvent for MembersSwapped {
				const PALLET: &'static str = "Membership";
				const EVENT: &'static str = "MembersSwapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The membership was reset; see the transaction for who the new set is."]
			pub struct MembersReset;
			impl ::subxt::events::StaticEvent for MembersReset {
				const PALLET: &'static str = "Membership";
				const EVENT: &'static str = "MembersReset";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "One of the members' keys changed."]
			pub struct KeyChanged;
			impl ::subxt::events::StaticEvent for KeyChanged {
				const PALLET: &'static str = "Membership";
				const EVENT: &'static str = "KeyChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Phantom member, never used."]
			pub struct Dummy;
			impl ::subxt::events::StaticEvent for Dummy {
				const PALLET: &'static str = "Membership";
				const EVENT: &'static str = "Dummy";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The current membership, stored as an ordered Vec."]
				pub fn members(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Membership",
						"Members",
						vec![],
						[
							56u8, 56u8, 29u8, 90u8, 26u8, 115u8, 252u8, 185u8, 37u8, 108u8, 16u8,
							46u8, 136u8, 139u8, 30u8, 19u8, 235u8, 78u8, 176u8, 129u8, 180u8, 57u8,
							178u8, 239u8, 211u8, 6u8, 64u8, 129u8, 195u8, 46u8, 178u8, 157u8,
						],
					)
				}
				#[doc = " The current prime member, if one exists."]
				pub fn prime(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::subxt::ext::sp_core::crypto::AccountId32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Membership",
						"Prime",
						vec![],
						[
							108u8, 118u8, 54u8, 193u8, 207u8, 227u8, 119u8, 97u8, 23u8, 239u8,
							157u8, 69u8, 56u8, 142u8, 106u8, 17u8, 215u8, 159u8, 48u8, 42u8, 185u8,
							209u8, 49u8, 159u8, 32u8, 168u8, 111u8, 158u8, 159u8, 217u8, 244u8,
							158u8,
						],
					)
				}
			}
		}
	}
	pub mod utility {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Batch {
				pub calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AsDerivative {
				pub index: ::core::primitive::u16,
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BatchAll {
				pub calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct DispatchAs {
				pub as_origin: ::std::boxed::Box<runtime_types::rococo_runtime::OriginCaller>,
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceBatch {
				pub calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
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
					calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
				) -> ::subxt::tx::StaticTxPayload<Batch> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"batch",
						Batch { calls },
						[
							112u8, 89u8, 27u8, 92u8, 33u8, 3u8, 207u8, 127u8, 201u8, 216u8, 153u8,
							74u8, 169u8, 57u8, 7u8, 183u8, 81u8, 156u8, 237u8, 103u8, 239u8, 186u8,
							251u8, 85u8, 179u8, 191u8, 143u8, 102u8, 226u8, 113u8, 77u8, 235u8,
						],
					)
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
					call: runtime_types::rococo_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<AsDerivative> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"as_derivative",
						AsDerivative { index, call: ::std::boxed::Box::new(call) },
						[
							94u8, 113u8, 78u8, 129u8, 53u8, 171u8, 199u8, 187u8, 151u8, 131u8,
							140u8, 146u8, 142u8, 240u8, 206u8, 227u8, 158u8, 159u8, 248u8, 161u8,
							120u8, 176u8, 39u8, 136u8, 68u8, 87u8, 22u8, 53u8, 32u8, 104u8, 43u8,
							101u8,
						],
					)
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
					calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
				) -> ::subxt::tx::StaticTxPayload<BatchAll> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"batch_all",
						BatchAll { calls },
						[
							163u8, 163u8, 96u8, 232u8, 203u8, 75u8, 99u8, 50u8, 148u8, 134u8, 37u8,
							175u8, 248u8, 187u8, 17u8, 173u8, 88u8, 158u8, 185u8, 97u8, 149u8,
							47u8, 60u8, 151u8, 204u8, 246u8, 31u8, 156u8, 16u8, 102u8, 250u8,
							106u8,
						],
					)
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
					as_origin: runtime_types::rococo_runtime::OriginCaller,
					call: runtime_types::rococo_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<DispatchAs> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"dispatch_as",
						DispatchAs {
							as_origin: ::std::boxed::Box::new(as_origin),
							call: ::std::boxed::Box::new(call),
						},
						[
							158u8, 83u8, 219u8, 137u8, 19u8, 162u8, 13u8, 143u8, 154u8, 94u8, 24u8,
							145u8, 237u8, 212u8, 129u8, 250u8, 216u8, 93u8, 142u8, 118u8, 81u8,
							135u8, 167u8, 107u8, 199u8, 46u8, 165u8, 84u8, 71u8, 182u8, 0u8, 135u8,
						],
					)
				}
				#[doc = "Send a batch of dispatch calls."]
				#[doc = "Unlike `batch`, it allows errors and won't interrupt."]
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
				pub fn force_batch(
					&self,
					calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call>,
				) -> ::subxt::tx::StaticTxPayload<ForceBatch> {
					::subxt::tx::StaticTxPayload::new(
						"Utility",
						"force_batch",
						ForceBatch { calls },
						[
							29u8, 120u8, 38u8, 122u8, 32u8, 33u8, 124u8, 168u8, 167u8, 229u8,
							130u8, 15u8, 153u8, 202u8, 60u8, 122u8, 8u8, 100u8, 237u8, 228u8,
							197u8, 145u8, 77u8, 180u8, 182u8, 236u8, 171u8, 62u8, 187u8, 89u8,
							107u8, 209u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_utility::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
			#[doc = "well as the error."]
			pub struct BatchInterrupted {
				pub index: ::core::primitive::u32,
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for BatchInterrupted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchInterrupted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Batch of dispatches completed fully with no error."]
			pub struct BatchCompleted;
			impl ::subxt::events::StaticEvent for BatchCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Batch of dispatches completed but has errors."]
			pub struct BatchCompletedWithErrors;
			impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "BatchCompletedWithErrors";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single item within a Batch of dispatches has completed with no error."]
			pub struct ItemCompleted;
			impl ::subxt::events::StaticEvent for ItemCompleted {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemCompleted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A single item within a Batch of dispatches has completed with error."]
			pub struct ItemFailed {
				pub error: runtime_types::sp_runtime::DispatchError,
			}
			impl ::subxt::events::StaticEvent for ItemFailed {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "ItemFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A call was dispatched."]
			pub struct DispatchedAs {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for DispatchedAs {
				const PALLET: &'static str = "Utility";
				const EVENT: &'static str = "DispatchedAs";
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The limit on the number of batched calls."]
				pub fn batched_calls_limit(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Utility",
						"batched_calls_limit",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
			}
		}
	}
	pub mod proxy {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Proxy {
				pub real: ::subxt::ext::sp_core::crypto::AccountId32,
				pub force_proxy_type:
					::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AddProxy {
				pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::rococo_runtime::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveProxy {
				pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::rococo_runtime::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveProxies;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Anonymous {
				pub proxy_type: runtime_types::rococo_runtime::ProxyType,
				pub delay: ::core::primitive::u32,
				pub index: ::core::primitive::u16,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct KillAnonymous {
				pub spawner: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::rococo_runtime::ProxyType,
				pub index: ::core::primitive::u16,
				#[codec(compact)]
				pub height: ::core::primitive::u32,
				#[codec(compact)]
				pub ext_index: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Announce {
				pub real: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RemoveAnnouncement {
				pub real: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct RejectAnnouncement {
				pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: ::subxt::ext::sp_core::H256,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ProxyAnnounced {
				pub delegate: ::subxt::ext::sp_core::crypto::AccountId32,
				pub real: ::subxt::ext::sp_core::crypto::AccountId32,
				pub force_proxy_type:
					::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Dispatch the given `call` from an account that the sender is authorised for through"]
				#[doc = "`add_proxy`."]
				#[doc = ""]
				#[doc = "Removes any corresponding announcement(s)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
				#[doc = "- `call`: The call to be made by the `real` account."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of the number of proxies the user has (P)."]
				#[doc = "# </weight>"]
				pub fn proxy(
					&self,
					real: ::subxt::ext::sp_core::crypto::AccountId32,
					force_proxy_type: ::core::option::Option<
						runtime_types::rococo_runtime::ProxyType,
					>,
					call: runtime_types::rococo_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<Proxy> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"proxy",
						Proxy { real, force_proxy_type, call: ::std::boxed::Box::new(call) },
						[
							189u8, 11u8, 194u8, 96u8, 23u8, 233u8, 60u8, 201u8, 0u8, 33u8, 171u8,
							64u8, 30u8, 165u8, 0u8, 29u8, 88u8, 203u8, 221u8, 228u8, 169u8, 18u8,
							124u8, 232u8, 239u8, 62u8, 243u8, 89u8, 30u8, 9u8, 122u8, 44u8,
						],
					)
				}
				#[doc = "Register a proxy account for the sender that is able to make calls on its behalf."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `proxy`: The account that the `caller` would like to make a proxy."]
				#[doc = "- `proxy_type`: The permissions allowed for this proxy account."]
				#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
				#[doc = "zero."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of the number of proxies the user has (P)."]
				#[doc = "# </weight>"]
				pub fn add_proxy(
					&self,
					delegate: ::subxt::ext::sp_core::crypto::AccountId32,
					proxy_type: runtime_types::rococo_runtime::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<AddProxy> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"add_proxy",
						AddProxy { delegate, proxy_type, delay },
						[
							56u8, 8u8, 89u8, 33u8, 85u8, 24u8, 130u8, 7u8, 46u8, 148u8, 124u8,
							72u8, 173u8, 185u8, 37u8, 91u8, 107u8, 42u8, 48u8, 18u8, 26u8, 139u8,
							131u8, 136u8, 160u8, 219u8, 110u8, 189u8, 230u8, 245u8, 56u8, 94u8,
						],
					)
				}
				#[doc = "Unregister a proxy account for the sender."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `proxy`: The account that the `caller` would like to remove as a proxy."]
				#[doc = "- `proxy_type`: The permissions currently enabled for the removed proxy account."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of the number of proxies the user has (P)."]
				#[doc = "# </weight>"]
				pub fn remove_proxy(
					&self,
					delegate: ::subxt::ext::sp_core::crypto::AccountId32,
					proxy_type: runtime_types::rococo_runtime::ProxyType,
					delay: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<RemoveProxy> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"remove_proxy",
						RemoveProxy { delegate, proxy_type, delay },
						[
							148u8, 225u8, 195u8, 66u8, 65u8, 83u8, 127u8, 60u8, 43u8, 165u8, 240u8,
							223u8, 190u8, 40u8, 89u8, 105u8, 103u8, 69u8, 5u8, 200u8, 68u8, 49u8,
							50u8, 188u8, 172u8, 187u8, 102u8, 205u8, 132u8, 110u8, 145u8, 162u8,
						],
					)
				}
				#[doc = "Unregister all proxy accounts for the sender."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "WARNING: This may be called on accounts created by `anonymous`, however if done, then"]
				#[doc = "the unreserved fees will be inaccessible. **All access to this account will be lost.**"]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of the number of proxies the user has (P)."]
				#[doc = "# </weight>"]
				pub fn remove_proxies(&self) -> ::subxt::tx::StaticTxPayload<RemoveProxies> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"remove_proxies",
						RemoveProxies {},
						[
							15u8, 237u8, 27u8, 166u8, 254u8, 218u8, 92u8, 5u8, 213u8, 239u8, 99u8,
							59u8, 1u8, 26u8, 73u8, 252u8, 81u8, 94u8, 214u8, 227u8, 169u8, 58u8,
							40u8, 253u8, 187u8, 225u8, 192u8, 26u8, 19u8, 23u8, 121u8, 129u8,
						],
					)
				}
				#[doc = "Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and"]
				#[doc = "initialize it with a proxy of `proxy_type` for `origin` sender."]
				#[doc = ""]
				#[doc = "Requires a `Signed` origin."]
				#[doc = ""]
				#[doc = "- `proxy_type`: The type of the proxy that the sender will be registered as over the"]
				#[doc = "new account. This will almost always be the most permissive `ProxyType` possible to"]
				#[doc = "allow for maximum flexibility."]
				#[doc = "- `index`: A disambiguation index, in case this is called multiple times in the same"]
				#[doc = "transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just"]
				#[doc = "want to use `0`."]
				#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
				#[doc = "zero."]
				#[doc = ""]
				#[doc = "Fails with `Duplicate` if this has already been called in this transaction, from the"]
				#[doc = "same sender, with the same parameters."]
				#[doc = ""]
				#[doc = "Fails if there are insufficient funds to pay for deposit."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of the number of proxies the user has (P)."]
				#[doc = "# </weight>"]
				#[doc = "TODO: Might be over counting 1 read"]
				pub fn anonymous(
					&self,
					proxy_type: runtime_types::rococo_runtime::ProxyType,
					delay: ::core::primitive::u32,
					index: ::core::primitive::u16,
				) -> ::subxt::tx::StaticTxPayload<Anonymous> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"anonymous",
						Anonymous { proxy_type, delay, index },
						[
							35u8, 107u8, 118u8, 113u8, 201u8, 172u8, 81u8, 50u8, 44u8, 137u8, 43u8,
							213u8, 218u8, 128u8, 59u8, 30u8, 255u8, 101u8, 181u8, 86u8, 98u8,
							242u8, 65u8, 57u8, 225u8, 233u8, 23u8, 144u8, 13u8, 241u8, 228u8,
							205u8,
						],
					)
				}
				#[doc = "Removes a previously spawned anonymous proxy."]
				#[doc = ""]
				#[doc = "WARNING: **All access to this account will be lost.** Any funds held in it will be"]
				#[doc = "inaccessible."]
				#[doc = ""]
				#[doc = "Requires a `Signed` origin, and the sender account must have been created by a call to"]
				#[doc = "`anonymous` with corresponding parameters."]
				#[doc = ""]
				#[doc = "- `spawner`: The account that originally called `anonymous` to create this account."]
				#[doc = "- `index`: The disambiguation index originally passed to `anonymous`. Probably `0`."]
				#[doc = "- `proxy_type`: The proxy type originally passed to `anonymous`."]
				#[doc = "- `height`: The height of the chain when the call to `anonymous` was processed."]
				#[doc = "- `ext_index`: The extrinsic index in which the call to `anonymous` was processed."]
				#[doc = ""]
				#[doc = "Fails with `NoPermission` in case the caller is not a previously created anonymous"]
				#[doc = "account whose `anonymous` call has corresponding parameters."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of the number of proxies the user has (P)."]
				#[doc = "# </weight>"]
				pub fn kill_anonymous(
					&self,
					spawner: ::subxt::ext::sp_core::crypto::AccountId32,
					proxy_type: runtime_types::rococo_runtime::ProxyType,
					index: ::core::primitive::u16,
					height: ::core::primitive::u32,
					ext_index: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<KillAnonymous> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"kill_anonymous",
						KillAnonymous { spawner, proxy_type, index, height, ext_index },
						[
							14u8, 125u8, 191u8, 178u8, 131u8, 27u8, 7u8, 8u8, 159u8, 199u8, 20u8,
							244u8, 70u8, 10u8, 107u8, 113u8, 50u8, 113u8, 116u8, 64u8, 43u8, 16u8,
							7u8, 234u8, 26u8, 82u8, 32u8, 103u8, 149u8, 191u8, 161u8, 140u8,
						],
					)
				}
				#[doc = "Publish the hash of a proxy-call that will be made in the future."]
				#[doc = ""]
				#[doc = "This must be called some number of blocks before the corresponding `proxy` is attempted"]
				#[doc = "if the delay associated with the proxy relationship is greater than zero."]
				#[doc = ""]
				#[doc = "No more than `MaxPending` announcements may be made at any one time."]
				#[doc = ""]
				#[doc = "This will take a deposit of `AnnouncementDepositFactor` as well as"]
				#[doc = "`AnnouncementDepositBase` if there are no other pending announcements."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_ and a proxy of `real`."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of:"]
				#[doc = "- A: the number of announcements made."]
				#[doc = "- P: the number of proxies the user has."]
				#[doc = "# </weight>"]
				pub fn announce(
					&self,
					real: ::subxt::ext::sp_core::crypto::AccountId32,
					call_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<Announce> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"announce",
						Announce { real, call_hash },
						[
							99u8, 237u8, 158u8, 131u8, 185u8, 119u8, 88u8, 167u8, 253u8, 29u8,
							82u8, 216u8, 225u8, 33u8, 181u8, 244u8, 85u8, 176u8, 106u8, 66u8,
							166u8, 174u8, 218u8, 98u8, 119u8, 86u8, 218u8, 89u8, 150u8, 255u8,
							86u8, 40u8,
						],
					)
				}
				#[doc = "Remove a given announcement."]
				#[doc = ""]
				#[doc = "May be called by a proxy account to remove a call they previously announced and return"]
				#[doc = "the deposit."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of:"]
				#[doc = "- A: the number of announcements made."]
				#[doc = "- P: the number of proxies the user has."]
				#[doc = "# </weight>"]
				pub fn remove_announcement(
					&self,
					real: ::subxt::ext::sp_core::crypto::AccountId32,
					call_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<RemoveAnnouncement> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"remove_announcement",
						RemoveAnnouncement { real, call_hash },
						[
							197u8, 54u8, 240u8, 51u8, 65u8, 218u8, 154u8, 165u8, 24u8, 54u8, 157u8,
							30u8, 144u8, 22u8, 247u8, 177u8, 105u8, 38u8, 9u8, 25u8, 127u8, 36u8,
							97u8, 84u8, 18u8, 3u8, 246u8, 238u8, 60u8, 17u8, 236u8, 69u8,
						],
					)
				}
				#[doc = "Remove the given announcement of a delegate."]
				#[doc = ""]
				#[doc = "May be called by a target (proxied) account to remove a call that one of their delegates"]
				#[doc = "(`delegate`) has announced they want to execute. The deposit is returned."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `delegate`: The account that previously announced the call."]
				#[doc = "- `call_hash`: The hash of the call to be made."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of:"]
				#[doc = "- A: the number of announcements made."]
				#[doc = "- P: the number of proxies the user has."]
				#[doc = "# </weight>"]
				pub fn reject_announcement(
					&self,
					delegate: ::subxt::ext::sp_core::crypto::AccountId32,
					call_hash: ::subxt::ext::sp_core::H256,
				) -> ::subxt::tx::StaticTxPayload<RejectAnnouncement> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"reject_announcement",
						RejectAnnouncement { delegate, call_hash },
						[
							205u8, 123u8, 102u8, 30u8, 196u8, 250u8, 247u8, 50u8, 243u8, 55u8,
							67u8, 66u8, 160u8, 147u8, 92u8, 204u8, 75u8, 69u8, 68u8, 140u8, 40u8,
							250u8, 53u8, 203u8, 228u8, 239u8, 62u8, 66u8, 254u8, 30u8, 126u8,
							206u8,
						],
					)
				}
				#[doc = "Dispatch the given `call` from an account that the sender is authorized for through"]
				#[doc = "`add_proxy`."]
				#[doc = ""]
				#[doc = "Removes any corresponding announcement(s)."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "Parameters:"]
				#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
				#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
				#[doc = "- `call`: The call to be made by the `real` account."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "Weight is a function of:"]
				#[doc = "- A: the number of announcements made."]
				#[doc = "- P: the number of proxies the user has."]
				#[doc = "# </weight>"]
				pub fn proxy_announced(
					&self,
					delegate: ::subxt::ext::sp_core::crypto::AccountId32,
					real: ::subxt::ext::sp_core::crypto::AccountId32,
					force_proxy_type: ::core::option::Option<
						runtime_types::rococo_runtime::ProxyType,
					>,
					call: runtime_types::rococo_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<ProxyAnnounced> {
					::subxt::tx::StaticTxPayload::new(
						"Proxy",
						"proxy_announced",
						ProxyAnnounced {
							delegate,
							real,
							force_proxy_type,
							call: ::std::boxed::Box::new(call),
						},
						[
							17u8, 70u8, 107u8, 78u8, 130u8, 225u8, 200u8, 150u8, 134u8, 202u8,
							157u8, 104u8, 184u8, 157u8, 207u8, 152u8, 202u8, 83u8, 138u8, 23u8,
							214u8, 190u8, 47u8, 146u8, 204u8, 114u8, 57u8, 29u8, 167u8, 140u8,
							150u8, 108u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_proxy::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proxy was executed correctly, with the given."]
			pub struct ProxyExecuted {
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for ProxyExecuted {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Anonymous account has been created by new proxy with given"]
			#[doc = "disambiguation index and proxy type."]
			pub struct AnonymousCreated {
				pub anonymous: ::subxt::ext::sp_core::crypto::AccountId32,
				pub who: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::rococo_runtime::ProxyType,
				pub disambiguation_index: ::core::primitive::u16,
			}
			impl ::subxt::events::StaticEvent for AnonymousCreated {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "AnonymousCreated";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An announcement was placed to make a call in the future."]
			pub struct Announced {
				pub real: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proxy: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: ::subxt::ext::sp_core::H256,
			}
			impl ::subxt::events::StaticEvent for Announced {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "Announced";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proxy was added."]
			pub struct ProxyAdded {
				pub delegator: ::subxt::ext::sp_core::crypto::AccountId32,
				pub delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::rococo_runtime::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProxyAdded {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyAdded";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A proxy was removed."]
			pub struct ProxyRemoved {
				pub delegator: ::subxt::ext::sp_core::crypto::AccountId32,
				pub delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
				pub proxy_type: runtime_types::rococo_runtime::ProxyType,
				pub delay: ::core::primitive::u32,
			}
			impl ::subxt::events::StaticEvent for ProxyRemoved {
				const PALLET: &'static str = "Proxy";
				const EVENT: &'static str = "ProxyRemoved";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The set of account proxies. Maps the account which has delegated to the accounts"]
				#[doc = " which are being delegated to, together with the amount held on deposit."]
				pub fn proxies(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::ProxyDefinition<
								::subxt::ext::sp_core::crypto::AccountId32,
								runtime_types::rococo_runtime::ProxyType,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Proxies",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							228u8, 194u8, 249u8, 210u8, 229u8, 183u8, 225u8, 183u8, 231u8, 220u8,
							45u8, 17u8, 169u8, 168u8, 229u8, 78u8, 18u8, 89u8, 17u8, 68u8, 112u8,
							47u8, 176u8, 255u8, 125u8, 185u8, 40u8, 216u8, 11u8, 21u8, 99u8, 225u8,
						],
					)
				}
				#[doc = " The set of account proxies. Maps the account which has delegated to the accounts"]
				#[doc = " which are being delegated to, together with the amount held on deposit."]
				pub fn proxies_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::ProxyDefinition<
								::subxt::ext::sp_core::crypto::AccountId32,
								runtime_types::rococo_runtime::ProxyType,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Proxies",
						Vec::new(),
						[
							228u8, 194u8, 249u8, 210u8, 229u8, 183u8, 225u8, 183u8, 231u8, 220u8,
							45u8, 17u8, 169u8, 168u8, 229u8, 78u8, 18u8, 89u8, 17u8, 68u8, 112u8,
							47u8, 176u8, 255u8, 125u8, 185u8, 40u8, 216u8, 11u8, 21u8, 99u8, 225u8,
						],
					)
				}
				#[doc = " The announcements made by the proxy (key)."]
				pub fn announcements(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::Announcement<
								::subxt::ext::sp_core::crypto::AccountId32,
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Announcements",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Twox64Concat,
						)],
						[
							233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
							37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
							160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
						],
					)
				}
				#[doc = " The announcements made by the proxy (key)."]
				pub fn announcements_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
							runtime_types::pallet_proxy::Announcement<
								::subxt::ext::sp_core::crypto::AccountId32,
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						::core::primitive::u128,
					)>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Proxy",
						"Announcements",
						Vec::new(),
						[
							233u8, 38u8, 249u8, 89u8, 103u8, 87u8, 64u8, 52u8, 140u8, 228u8, 110u8,
							37u8, 8u8, 92u8, 48u8, 7u8, 46u8, 99u8, 179u8, 83u8, 232u8, 171u8,
							160u8, 45u8, 37u8, 23u8, 151u8, 198u8, 237u8, 103u8, 217u8, 53u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The base amount of currency needed to reserve for creating a proxy."]
				#[doc = ""]
				#[doc = " This is held for an additional storage item whose value size is"]
				#[doc = " `sizeof(Balance)` bytes and whose key size is `sizeof(AccountId)` bytes."]
				pub fn proxy_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"ProxyDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of currency needed per proxy added."]
				#[doc = ""]
				#[doc = " This is held for adding 32 bytes plus an instance of `ProxyType` more into a"]
				#[doc = " pre-existing storage value. Thus, when configuring `ProxyDepositFactor` one should take"]
				#[doc = " into account `32 + proxy_type.encode().len()` bytes of data."]
				pub fn proxy_deposit_factor(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"ProxyDepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum amount of proxies allowed for a single account."]
				pub fn max_proxies(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"MaxProxies",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The maximum amount of time-delayed announcements that are allowed to be pending."]
				pub fn max_pending(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"MaxPending",
						[
							98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
							125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
							178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
							145u8,
						],
					)
				}
				#[doc = " The base amount of currency needed to reserve for creating an announcement."]
				#[doc = ""]
				#[doc = " This is held when a new storage item holding a `Balance` is created (typically 16"]
				#[doc = " bytes)."]
				pub fn announcement_deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"AnnouncementDepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of currency needed per announcement made."]
				#[doc = ""]
				#[doc = " This is held for adding an `AccountId`, `Hash` and `BlockNumber` (typically 68 bytes)"]
				#[doc = " into a pre-existing storage value."]
				pub fn announcement_deposit_factor(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Proxy",
						"AnnouncementDepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
			}
		}
	}
	pub mod multisig {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AsMultiThreshold1 {
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub maybe_timepoint: ::core::option::Option<
					runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				>,
				pub call: ::subxt::utils::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
				pub store_call: ::core::primitive::bool,
				pub max_weight: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ApproveAsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub maybe_timepoint: ::core::option::Option<
					runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				>,
				pub call_hash: [::core::primitive::u8; 32usize],
				pub max_weight: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CancelAsMulti {
				pub threshold: ::core::primitive::u16,
				pub other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			pub struct TransactionApi;
			impl TransactionApi {
				#[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
				#[doc = "multi-signature, but do not participate in the approval process."]
				#[doc = "- `call`: The call to be executed."]
				#[doc = ""]
				#[doc = "Result is equivalent to the dispatched result."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
				#[doc = "-------------------------------"]
				#[doc = "- DB Weight: None"]
				#[doc = "- Plus Call Weight"]
				#[doc = "# </weight>"]
				pub fn as_multi_threshold_1(
					&self,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					call: runtime_types::rococo_runtime::Call,
				) -> ::subxt::tx::StaticTxPayload<AsMultiThreshold1> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"as_multi_threshold_1",
						AsMultiThreshold1 { other_signatories, call: ::std::boxed::Box::new(call) },
						[
							147u8, 210u8, 241u8, 159u8, 19u8, 8u8, 72u8, 236u8, 142u8, 34u8, 130u8,
							224u8, 44u8, 30u8, 43u8, 22u8, 239u8, 75u8, 5u8, 43u8, 142u8, 247u8,
							162u8, 83u8, 9u8, 1u8, 243u8, 189u8, 43u8, 164u8, 46u8, 229u8,
						],
					)
				}
				#[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
				#[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
				#[doc = ""]
				#[doc = "If there are enough, then dispatch the call."]
				#[doc = ""]
				#[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
				#[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
				#[doc = "is cancelled."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
				#[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
				#[doc = "dispatch. May not be empty."]
				#[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
				#[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
				#[doc = "transaction index) of the first approval transaction."]
				#[doc = "- `call`: The call to be executed."]
				#[doc = ""]
				#[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
				#[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
				#[doc = ""]
				#[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
				#[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
				#[doc = "may be found in the deposited `MultisigExecuted` event."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(S + Z + Call)`."]
				#[doc = "- Up to one balance-reserve or unreserve operation."]
				#[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
				#[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
				#[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
				#[doc = "- One encode & hash, both of complexity `O(S)`."]
				#[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
				#[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
				#[doc = "- One event."]
				#[doc = "- The weight of the `call`."]
				#[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
				#[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
				#[doc = "-------------------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "    - Reads: Multisig Storage, [Caller Account], Calls (if `store_call`)"]
				#[doc = "    - Writes: Multisig Storage, [Caller Account], Calls (if `store_call`)"]
				#[doc = "- Plus Call Weight"]
				#[doc = "# </weight>"]
				pub fn as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call: ::subxt::utils::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
					store_call: ::core::primitive::bool,
					max_weight: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<AsMulti> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"as_multi",
						AsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call,
							store_call,
							max_weight,
						},
						[
							248u8, 140u8, 74u8, 0u8, 32u8, 140u8, 178u8, 41u8, 206u8, 105u8, 51u8,
							54u8, 108u8, 127u8, 229u8, 25u8, 123u8, 31u8, 222u8, 176u8, 255u8,
							132u8, 56u8, 142u8, 191u8, 204u8, 152u8, 49u8, 233u8, 193u8, 237u8,
							7u8,
						],
					)
				}
				#[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
				#[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
				#[doc = ""]
				#[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
				#[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
				#[doc = "is cancelled."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
				#[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
				#[doc = "dispatch. May not be empty."]
				#[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
				#[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
				#[doc = "transaction index) of the first approval transaction."]
				#[doc = "- `call_hash`: The hash of the call to be executed."]
				#[doc = ""]
				#[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(S)`."]
				#[doc = "- Up to one balance-reserve or unreserve operation."]
				#[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
				#[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
				#[doc = "- One encode & hash, both of complexity `O(S)`."]
				#[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
				#[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
				#[doc = "- One event."]
				#[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
				#[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
				#[doc = "----------------------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "    - Read: Multisig Storage, [Caller Account]"]
				#[doc = "    - Write: Multisig Storage, [Caller Account]"]
				#[doc = "# </weight>"]
				pub fn approve_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					maybe_timepoint: ::core::option::Option<
						runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					>,
					call_hash: [::core::primitive::u8; 32usize],
					max_weight: ::core::primitive::u64,
				) -> ::subxt::tx::StaticTxPayload<ApproveAsMulti> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"approve_as_multi",
						ApproveAsMulti {
							threshold,
							other_signatories,
							maybe_timepoint,
							call_hash,
							max_weight,
						},
						[
							55u8, 94u8, 230u8, 217u8, 37u8, 143u8, 44u8, 108u8, 123u8, 250u8, 26u8,
							44u8, 236u8, 69u8, 63u8, 90u8, 126u8, 15u8, 233u8, 142u8, 213u8, 11u8,
							141u8, 147u8, 151u8, 24u8, 167u8, 62u8, 96u8, 227u8, 181u8, 140u8,
						],
					)
				}
				#[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
				#[doc = "for this operation will be unreserved on success."]
				#[doc = ""]
				#[doc = "The dispatch origin for this call must be _Signed_."]
				#[doc = ""]
				#[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
				#[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
				#[doc = "dispatch. May not be empty."]
				#[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
				#[doc = "transaction for this dispatch."]
				#[doc = "- `call_hash`: The hash of the call to be executed."]
				#[doc = ""]
				#[doc = "# <weight>"]
				#[doc = "- `O(S)`."]
				#[doc = "- Up to one balance-reserve or unreserve operation."]
				#[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
				#[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
				#[doc = "- One encode & hash, both of complexity `O(S)`."]
				#[doc = "- One event."]
				#[doc = "- I/O: 1 read `O(S)`, one remove."]
				#[doc = "- Storage: removes one item."]
				#[doc = "----------------------------------"]
				#[doc = "- DB Weight:"]
				#[doc = "    - Read: Multisig Storage, [Caller Account], Refund Account, Calls"]
				#[doc = "    - Write: Multisig Storage, [Caller Account], Refund Account, Calls"]
				#[doc = "# </weight>"]
				pub fn cancel_as_multi(
					&self,
					threshold: ::core::primitive::u16,
					other_signatories: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
					call_hash: [::core::primitive::u8; 32usize],
				) -> ::subxt::tx::StaticTxPayload<CancelAsMulti> {
					::subxt::tx::StaticTxPayload::new(
						"Multisig",
						"cancel_as_multi",
						CancelAsMulti { threshold, other_signatories, timepoint, call_hash },
						[
							30u8, 25u8, 186u8, 142u8, 168u8, 81u8, 235u8, 164u8, 82u8, 209u8, 66u8,
							129u8, 209u8, 78u8, 172u8, 9u8, 163u8, 222u8, 125u8, 57u8, 2u8, 43u8,
							169u8, 174u8, 159u8, 167u8, 25u8, 226u8, 254u8, 110u8, 80u8, 216u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_multisig::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A new multisig operation has begun."]
			pub struct NewMultisig {
				pub approving: ::subxt::ext::sp_core::crypto::AccountId32,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for NewMultisig {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "NewMultisig";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A multisig operation has been approved by someone."]
			pub struct MultisigApproval {
				pub approving: ::subxt::ext::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigApproval {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigApproval";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A multisig operation has been executed."]
			pub struct MultisigExecuted {
				pub approving: ::subxt::ext::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
				pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
			}
			impl ::subxt::events::StaticEvent for MultisigExecuted {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigExecuted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A multisig operation has been cancelled."]
			pub struct MultisigCancelled {
				pub cancelling: ::subxt::ext::sp_core::crypto::AccountId32,
				pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
				pub multisig: ::subxt::ext::sp_core::crypto::AccountId32,
				pub call_hash: [::core::primitive::u8; 32usize],
			}
			impl ::subxt::events::StaticEvent for MultisigCancelled {
				const PALLET: &'static str = "Multisig";
				const EVENT: &'static str = "MultisigCancelled";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The set of open multisig operations."]
				pub fn multisigs(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::crypto::AccountId32>,
					_1: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_multisig::Multisig<
							::core::primitive::u32,
							::core::primitive::u128,
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Multisig",
						"Multisigs",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							145u8, 78u8, 57u8, 171u8, 199u8, 158u8, 226u8, 250u8, 224u8, 133u8,
							45u8, 251u8, 202u8, 22u8, 171u8, 132u8, 229u8, 110u8, 248u8, 233u8,
							38u8, 2u8, 247u8, 140u8, 150u8, 103u8, 211u8, 209u8, 160u8, 158u8,
							23u8, 215u8,
						],
					)
				}
				#[doc = " The set of open multisig operations."]
				pub fn multisigs_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_multisig::Multisig<
							::core::primitive::u32,
							::core::primitive::u128,
							::subxt::ext::sp_core::crypto::AccountId32,
						>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Multisig",
						"Multisigs",
						Vec::new(),
						[
							145u8, 78u8, 57u8, 171u8, 199u8, 158u8, 226u8, 250u8, 224u8, 133u8,
							45u8, 251u8, 202u8, 22u8, 171u8, 132u8, 229u8, 110u8, 248u8, 233u8,
							38u8, 2u8, 247u8, 140u8, 150u8, 103u8, 211u8, 209u8, 160u8, 158u8,
							23u8, 215u8,
						],
					)
				}
				pub fn calls(
					&self,
					_0: impl ::std::borrow::Borrow<[::core::primitive::u8; 32usize]>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::utils::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Multisig",
						"Calls",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							127u8, 34u8, 209u8, 141u8, 134u8, 131u8, 19u8, 248u8, 89u8, 89u8, 60u8,
							132u8, 221u8, 221u8, 149u8, 102u8, 15u8, 42u8, 62u8, 38u8, 116u8,
							183u8, 44u8, 216u8, 59u8, 118u8, 197u8, 97u8, 237u8, 39u8, 142u8, 0u8,
						],
					)
				}
				pub fn calls_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::subxt::utils::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
						::subxt::ext::sp_core::crypto::AccountId32,
						::core::primitive::u128,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"Multisig",
						"Calls",
						Vec::new(),
						[
							127u8, 34u8, 209u8, 141u8, 134u8, 131u8, 19u8, 248u8, 89u8, 89u8, 60u8,
							132u8, 221u8, 221u8, 149u8, 102u8, 15u8, 42u8, 62u8, 38u8, 116u8,
							183u8, 44u8, 216u8, 59u8, 118u8, 197u8, 97u8, 237u8, 39u8, 142u8, 0u8,
						],
					)
				}
			}
		}
		pub mod constants {
			use super::runtime_types;
			pub struct ConstantsApi;
			impl ConstantsApi {
				#[doc = " The base amount of currency needed to reserve for creating a multisig execution or to"]
				#[doc = " store a dispatch call for later."]
				#[doc = ""]
				#[doc = " This is held for an additional storage item whose value size is"]
				#[doc = " `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is"]
				#[doc = " `32 + sizeof(AccountId)` bytes."]
				pub fn deposit_base(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Multisig",
						"DepositBase",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The amount of currency needed per unit threshold when creating a multisig execution."]
				#[doc = ""]
				#[doc = " This is held for adding 32 bytes more into a pre-existing storage value."]
				pub fn deposit_factor(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u128>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Multisig",
						"DepositFactor",
						[
							84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
							27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
							136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
						],
					)
				}
				#[doc = " The maximum amount of signatories allowed in the multisig."]
				pub fn max_signatories(
					&self,
				) -> ::subxt::constants::StaticConstantAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u16>,
				> {
					::subxt::constants::StaticConstantAddress::new(
						"Multisig",
						"MaxSignatories",
						[
							116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
							41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
							90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
						],
					)
				}
			}
		}
	}
	pub mod xcm_pallet {
		use super::{root_mod, runtime_types};
		#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
		pub mod calls {
			use super::{root_mod, runtime_types};
			type DispatchError = runtime_types::sp_runtime::DispatchError;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Send {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct TeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Execute {
				pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
				pub max_weight: ::core::primitive::u64,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceXcmVersion {
				pub location:
					::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
				pub xcm_version: ::core::primitive::u32,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceDefaultXcmVersion {
				pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceSubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ForceUnsubscribeVersionNotify {
				pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LimitedReserveTransferAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LimitedTeleportAssets {
				pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
				pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
				pub fee_asset_item: ::core::primitive::u32,
				pub weight_limit: runtime_types::xcm::v2::WeightLimit,
			}
			pub struct TransactionApi;
			impl TransactionApi {
				pub fn send(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					message: runtime_types::xcm::VersionedXcm,
				) -> ::subxt::tx::StaticTxPayload<Send> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"send",
						Send {
							dest: ::std::boxed::Box::new(dest),
							message: ::std::boxed::Box::new(message),
						},
						[
							190u8, 88u8, 197u8, 248u8, 111u8, 198u8, 199u8, 206u8, 39u8, 121u8,
							23u8, 121u8, 93u8, 82u8, 22u8, 61u8, 96u8, 210u8, 142u8, 249u8, 195u8,
							78u8, 44u8, 8u8, 118u8, 120u8, 113u8, 168u8, 99u8, 94u8, 232u8, 4u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				pub fn teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
				) -> ::subxt::tx::StaticTxPayload<TeleportAssets> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"teleport_assets",
						TeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							255u8, 5u8, 68u8, 38u8, 44u8, 181u8, 75u8, 221u8, 239u8, 103u8, 88u8,
							47u8, 136u8, 90u8, 253u8, 55u8, 0u8, 122u8, 217u8, 126u8, 13u8, 77u8,
							209u8, 41u8, 7u8, 35u8, 235u8, 171u8, 150u8, 235u8, 202u8, 240u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
				#[doc = "chain and forward a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
				#[doc = "with all fees taken as needed from the asset."]
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
				) -> ::subxt::tx::StaticTxPayload<ReserveTransferAssets> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"reserve_transfer_assets",
						ReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
						},
						[
							177u8, 160u8, 188u8, 106u8, 153u8, 135u8, 121u8, 12u8, 83u8, 233u8,
							43u8, 161u8, 133u8, 26u8, 104u8, 79u8, 113u8, 8u8, 33u8, 128u8, 82u8,
							62u8, 30u8, 46u8, 203u8, 199u8, 175u8, 193u8, 55u8, 130u8, 206u8, 28u8,
						],
					)
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
				) -> ::subxt::tx::StaticTxPayload<Execute> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"execute",
						Execute { message: ::std::boxed::Box::new(message), max_weight },
						[
							191u8, 177u8, 39u8, 21u8, 1u8, 110u8, 39u8, 58u8, 94u8, 27u8, 44u8,
							18u8, 253u8, 135u8, 100u8, 205u8, 0u8, 231u8, 68u8, 247u8, 5u8, 140u8,
							131u8, 184u8, 251u8, 197u8, 100u8, 113u8, 253u8, 255u8, 120u8, 206u8,
						],
					)
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
				) -> ::subxt::tx::StaticTxPayload<ForceXcmVersion> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"force_xcm_version",
						ForceXcmVersion { location: ::std::boxed::Box::new(location), xcm_version },
						[
							231u8, 106u8, 60u8, 226u8, 31u8, 25u8, 20u8, 115u8, 107u8, 246u8,
							248u8, 11u8, 71u8, 183u8, 93u8, 3u8, 219u8, 21u8, 97u8, 188u8, 119u8,
							121u8, 239u8, 72u8, 200u8, 81u8, 6u8, 177u8, 111u8, 188u8, 168u8, 86u8,
						],
					)
				}
				#[doc = "Set a safe XCM version (the version that XCM should be encoded with if the most recent"]
				#[doc = "version a destination can accept is unknown)."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `maybe_xcm_version`: The default XCM encoding version, or `None` to disable."]
				pub fn force_default_xcm_version(
					&self,
					maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
				) -> ::subxt::tx::StaticTxPayload<ForceDefaultXcmVersion> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"force_default_xcm_version",
						ForceDefaultXcmVersion { maybe_xcm_version },
						[
							38u8, 36u8, 59u8, 231u8, 18u8, 79u8, 76u8, 9u8, 200u8, 125u8, 214u8,
							166u8, 37u8, 99u8, 111u8, 161u8, 135u8, 2u8, 133u8, 157u8, 165u8, 18u8,
							152u8, 81u8, 209u8, 255u8, 137u8, 237u8, 28u8, 126u8, 224u8, 141u8,
						],
					)
				}
				#[doc = "Ask a location to notify us regarding their XCM version and any changes to it."]
				#[doc = ""]
				#[doc = "- `origin`: Must be Root."]
				#[doc = "- `location`: The location to which we should subscribe for XCM version notifications."]
				pub fn force_subscribe_version_notify(
					&self,
					location: runtime_types::xcm::VersionedMultiLocation,
				) -> ::subxt::tx::StaticTxPayload<ForceSubscribeVersionNotify> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"force_subscribe_version_notify",
						ForceSubscribeVersionNotify { location: ::std::boxed::Box::new(location) },
						[
							136u8, 216u8, 207u8, 51u8, 42u8, 153u8, 92u8, 70u8, 140u8, 169u8,
							172u8, 89u8, 69u8, 28u8, 200u8, 100u8, 209u8, 226u8, 194u8, 240u8,
							71u8, 38u8, 18u8, 6u8, 6u8, 83u8, 103u8, 254u8, 248u8, 241u8, 62u8,
							189u8,
						],
					)
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
				) -> ::subxt::tx::StaticTxPayload<ForceUnsubscribeVersionNotify> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"force_unsubscribe_version_notify",
						ForceUnsubscribeVersionNotify {
							location: ::std::boxed::Box::new(location),
						},
						[
							51u8, 72u8, 5u8, 227u8, 251u8, 243u8, 199u8, 9u8, 8u8, 213u8, 191u8,
							52u8, 21u8, 215u8, 170u8, 6u8, 53u8, 242u8, 225u8, 89u8, 150u8, 142u8,
							104u8, 249u8, 225u8, 209u8, 142u8, 234u8, 161u8, 100u8, 153u8, 120u8,
						],
					)
				}
				#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
				#[doc = "chain and forward a notification XCM."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
				#[doc = "at risk."]
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
				) -> ::subxt::tx::StaticTxPayload<LimitedReserveTransferAssets> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"limited_reserve_transfer_assets",
						LimitedReserveTransferAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							191u8, 81u8, 68u8, 116u8, 196u8, 125u8, 226u8, 154u8, 144u8, 126u8,
							159u8, 149u8, 17u8, 124u8, 205u8, 60u8, 249u8, 106u8, 38u8, 251u8,
							136u8, 128u8, 81u8, 201u8, 164u8, 242u8, 216u8, 80u8, 21u8, 234u8,
							20u8, 70u8,
						],
					)
				}
				#[doc = "Teleport some assets from the local chain to some destination chain."]
				#[doc = ""]
				#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
				#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
				#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
				#[doc = "at risk."]
				#[doc = ""]
				#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
				#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
				#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
				#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
				#[doc = "  an `AccountId32` value."]
				#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
				#[doc = "  `dest` side. May not be empty."]
				#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
				#[doc = "  fees."]
				#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
				pub fn limited_teleport_assets(
					&self,
					dest: runtime_types::xcm::VersionedMultiLocation,
					beneficiary: runtime_types::xcm::VersionedMultiLocation,
					assets: runtime_types::xcm::VersionedMultiAssets,
					fee_asset_item: ::core::primitive::u32,
					weight_limit: runtime_types::xcm::v2::WeightLimit,
				) -> ::subxt::tx::StaticTxPayload<LimitedTeleportAssets> {
					::subxt::tx::StaticTxPayload::new(
						"XcmPallet",
						"limited_teleport_assets",
						LimitedTeleportAssets {
							dest: ::std::boxed::Box::new(dest),
							beneficiary: ::std::boxed::Box::new(beneficiary),
							assets: ::std::boxed::Box::new(assets),
							fee_asset_item,
							weight_limit,
						},
						[
							29u8, 31u8, 229u8, 83u8, 40u8, 60u8, 36u8, 185u8, 169u8, 74u8, 30u8,
							47u8, 118u8, 118u8, 22u8, 15u8, 246u8, 220u8, 169u8, 135u8, 72u8,
							154u8, 109u8, 192u8, 195u8, 58u8, 121u8, 240u8, 166u8, 243u8, 29u8,
							29u8,
						],
					)
				}
			}
		}
		#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
		pub type Event = runtime_types::pallet_xcm::pallet::Event;
		pub mod events {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Execution of an XCM message was attempted."]
			#[doc = ""]
			#[doc = "\\[ outcome \\]"]
			pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
			impl ::subxt::events::StaticEvent for Attempted {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "Attempted";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A XCM message was sent."]
			#[doc = ""]
			#[doc = "\\[ origin, destination, message \\]"]
			pub struct Sent(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::v2::Xcm,
			);
			impl ::subxt::events::StaticEvent for Sent {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "Sent";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response received which does not match a registered query. This may be because a"]
			#[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
			#[doc = "because the query timed out."]
			#[doc = ""]
			#[doc = "\\[ origin location, id \\]"]
			pub struct UnexpectedResponse(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for UnexpectedResponse {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "UnexpectedResponse";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
			#[doc = "no registered notification call."]
			#[doc = ""]
			#[doc = "\\[ id, response \\]"]
			pub struct ResponseReady(
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::Response,
			);
			impl ::subxt::events::StaticEvent for ResponseReady {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "ResponseReady";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. The registered notification has"]
			#[doc = "been dispatched and executed successfully."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct Notified(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for Notified {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "Notified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
			impl ::subxt::events::StaticEvent for NotifyOverweight {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "NotifyOverweight";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Query response has been received and query is removed. There was a general error with"]
			#[doc = "dispatching the notification call."]
			#[doc = ""]
			#[doc = "\\[ id, pallet index, call index \\]"]
			pub struct NotifyDispatchError(
				pub ::core::primitive::u64,
				pub ::core::primitive::u8,
				pub ::core::primitive::u8,
			);
			impl ::subxt::events::StaticEvent for NotifyDispatchError {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "NotifyDispatchError";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
			impl ::subxt::events::StaticEvent for NotifyDecodeFailed {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "NotifyDecodeFailed";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
			impl ::subxt::events::StaticEvent for InvalidResponder {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "InvalidResponder";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
			impl ::subxt::events::StaticEvent for InvalidResponderVersion {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "InvalidResponderVersion";
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			#[doc = "Received query response has been read and removed."]
			#[doc = ""]
			#[doc = "\\[ id \\]"]
			pub struct ResponseTaken(pub ::core::primitive::u64);
			impl ::subxt::events::StaticEvent for ResponseTaken {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "ResponseTaken";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "Some assets have been placed in an asset trap."]
			#[doc = ""]
			#[doc = "\\[ hash, origin, assets \\]"]
			pub struct AssetsTrapped(
				pub ::subxt::ext::sp_core::H256,
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub runtime_types::xcm::VersionedMultiAssets,
			);
			impl ::subxt::events::StaticEvent for AssetsTrapped {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "AssetsTrapped";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "An XCM version change notification message has been attempted to be sent."]
			#[doc = ""]
			#[doc = "\\[ destination, result \\]"]
			pub struct VersionChangeNotified(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for VersionChangeNotified {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "VersionChangeNotified";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "The supported version of a location has been changed. This might be through an"]
			#[doc = "automatic notification or a manual intervention."]
			#[doc = ""]
			#[doc = "\\[ location, XCM version \\]"]
			pub struct SupportedVersionChanged(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u32,
			);
			impl ::subxt::events::StaticEvent for SupportedVersionChanged {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "SupportedVersionChanged";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "sending the notification to it."]
			#[doc = ""]
			#[doc = "\\[ location, query ID, error \\]"]
			pub struct NotifyTargetSendFail(
				pub runtime_types::xcm::v1::multilocation::MultiLocation,
				pub ::core::primitive::u64,
				pub runtime_types::xcm::v2::traits::Error,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetSendFail {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "NotifyTargetSendFail";
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			#[doc = "A given location which had a version change subscription was dropped owing to an error"]
			#[doc = "migrating the location to our new XCM format."]
			#[doc = ""]
			#[doc = "\\[ location, query ID \\]"]
			pub struct NotifyTargetMigrationFail(
				pub runtime_types::xcm::VersionedMultiLocation,
				pub ::core::primitive::u64,
			);
			impl ::subxt::events::StaticEvent for NotifyTargetMigrationFail {
				const PALLET: &'static str = "XcmPallet";
				const EVENT: &'static str = "NotifyTargetMigrationFail";
			}
		}
		pub mod storage {
			use super::runtime_types;
			pub struct StorageApi;
			impl StorageApi {
				#[doc = " The latest available query index."]
				pub fn query_counter(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"QueryCounter",
						vec![],
						[
							137u8, 58u8, 184u8, 88u8, 247u8, 22u8, 151u8, 64u8, 50u8, 77u8, 49u8,
							10u8, 234u8, 84u8, 213u8, 156u8, 26u8, 200u8, 214u8, 225u8, 125u8,
							231u8, 42u8, 93u8, 159u8, 168u8, 86u8, 201u8, 116u8, 153u8, 41u8,
							127u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u64>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"Queries",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Blake2_128Concat,
						)],
						[
							251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
							124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
							148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
							48u8,
						],
					)
				}
				#[doc = " The ongoing queries."]
				pub fn queries_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
					>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"Queries",
						Vec::new(),
						[
							251u8, 97u8, 131u8, 135u8, 93u8, 68u8, 156u8, 25u8, 181u8, 231u8,
							124u8, 93u8, 170u8, 114u8, 250u8, 177u8, 172u8, 51u8, 59u8, 44u8,
							148u8, 189u8, 199u8, 62u8, 118u8, 89u8, 75u8, 29u8, 71u8, 49u8, 248u8,
							48u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps(
					&self,
					_0: impl ::std::borrow::Borrow<::subxt::ext::sp_core::H256>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"AssetTraps",
						vec![::subxt::storage::address::StorageMapKey::new(
							_0.borrow(),
							::subxt::storage::address::StorageHasher::Identity,
						)],
						[
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
						],
					)
				}
				#[doc = " The existing asset traps."]
				#[doc = ""]
				#[doc = " Key is the blake2 256 hash of (origin, versioned `MultiAssets`) pair. Value is the number of"]
				#[doc = " times this pair has been trapped (usually just 1 if it exists at all)."]
				pub fn asset_traps_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"AssetTraps",
						Vec::new(),
						[
							4u8, 185u8, 92u8, 4u8, 7u8, 71u8, 214u8, 1u8, 141u8, 59u8, 87u8, 55u8,
							149u8, 26u8, 125u8, 8u8, 88u8, 31u8, 240u8, 138u8, 133u8, 28u8, 37u8,
							131u8, 107u8, 218u8, 86u8, 152u8, 147u8, 44u8, 19u8, 239u8,
						],
					)
				}
				#[doc = " Default version to encode XCM when latest version of destination is unknown. If `None`,"]
				#[doc = " then the destinations whose XCM version is unknown are considered unreachable."]
				pub fn safe_xcm_version(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"SafeXcmVersion",
						vec![],
						[
							1u8, 223u8, 218u8, 204u8, 222u8, 129u8, 137u8, 237u8, 197u8, 142u8,
							233u8, 66u8, 229u8, 153u8, 138u8, 222u8, 113u8, 164u8, 135u8, 213u8,
							233u8, 34u8, 24u8, 23u8, 215u8, 59u8, 40u8, 188u8, 45u8, 244u8, 205u8,
							199u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"SupportedVersion",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
							14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
							219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
						],
					)
				}
				#[doc = " The Latest versions that we know various locations support."]
				pub fn supported_version_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u32>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"SupportedVersion",
						Vec::new(),
						[
							112u8, 34u8, 251u8, 179u8, 217u8, 54u8, 125u8, 242u8, 190u8, 8u8, 44u8,
							14u8, 138u8, 76u8, 241u8, 95u8, 233u8, 96u8, 141u8, 26u8, 151u8, 196u8,
							219u8, 137u8, 165u8, 27u8, 87u8, 128u8, 19u8, 35u8, 222u8, 202u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"VersionNotifiers",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
							104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
							190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
							10u8,
						],
					)
				}
				#[doc = " All locations that we have requested version notifications from."]
				pub fn version_notifiers_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<::core::primitive::u64>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"VersionNotifiers",
						Vec::new(),
						[
							233u8, 217u8, 119u8, 102u8, 41u8, 77u8, 198u8, 24u8, 161u8, 22u8,
							104u8, 149u8, 204u8, 128u8, 123u8, 166u8, 17u8, 36u8, 202u8, 92u8,
							190u8, 44u8, 73u8, 239u8, 88u8, 17u8, 92u8, 41u8, 236u8, 80u8, 154u8,
							10u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets(
					&self,
					_0: impl ::std::borrow::Borrow<::core::primitive::u32>,
					_1: impl ::std::borrow::Borrow<runtime_types::xcm::VersionedMultiLocation>,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u64,
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					::subxt::storage::address::Yes,
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"VersionNotifyTargets",
						vec![
							::subxt::storage::address::StorageMapKey::new(
								_0.borrow(),
								::subxt::storage::address::StorageHasher::Twox64Concat,
							),
							::subxt::storage::address::StorageMapKey::new(
								_1.borrow(),
								::subxt::storage::address::StorageHasher::Blake2_128Concat,
							),
						],
						[
							108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
							136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
							15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
						],
					)
				}
				#[doc = " The target locations that are subscribed to our version changes, as well as the most recent"]
				#[doc = " of our versions we informed them of."]
				pub fn version_notify_targets_root(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<(
						::core::primitive::u64,
						::core::primitive::u64,
						::core::primitive::u32,
					)>,
					(),
					(),
					::subxt::storage::address::Yes,
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"VersionNotifyTargets",
						Vec::new(),
						[
							108u8, 104u8, 137u8, 191u8, 2u8, 2u8, 240u8, 174u8, 32u8, 174u8, 150u8,
							136u8, 33u8, 84u8, 30u8, 74u8, 95u8, 94u8, 20u8, 112u8, 101u8, 204u8,
							15u8, 47u8, 136u8, 56u8, 40u8, 66u8, 1u8, 42u8, 16u8, 247u8,
						],
					)
				}
				#[doc = " Destinations whose latest XCM version we would like to know. Duplicates not allowed, and"]
				#[doc = " the `u32` counter is the number of times that a send to the destination has been attempted,"]
				#[doc = " which is used as a prioritization."]
				pub fn version_discovery_queue(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<(
							runtime_types::xcm::VersionedMultiLocation,
							::core::primitive::u32,
						)>,
					>,
					::subxt::storage::address::Yes,
					::subxt::storage::address::Yes,
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"VersionDiscoveryQueue",
						vec![],
						[
							30u8, 163u8, 210u8, 133u8, 30u8, 63u8, 36u8, 9u8, 162u8, 133u8, 99u8,
							170u8, 34u8, 205u8, 27u8, 41u8, 226u8, 141u8, 165u8, 151u8, 46u8,
							140u8, 150u8, 242u8, 178u8, 88u8, 164u8, 12u8, 129u8, 118u8, 25u8,
							79u8,
						],
					)
				}
				#[doc = " The current migration's stage, if any."]
				pub fn current_migration(
					&self,
				) -> ::subxt::storage::address::StaticStorageAddress<
					::subxt::metadata::DecodeStaticType<
						runtime_types::pallet_xcm::pallet::VersionMigrationStage,
					>,
					::subxt::storage::address::Yes,
					(),
					(),
				> {
					::subxt::storage::address::StaticStorageAddress::new(
						"XcmPallet",
						"CurrentMigration",
						vec![],
						[
							137u8, 144u8, 168u8, 185u8, 158u8, 90u8, 127u8, 243u8, 227u8, 134u8,
							150u8, 73u8, 15u8, 99u8, 23u8, 47u8, 68u8, 18u8, 39u8, 16u8, 24u8,
							43u8, 161u8, 56u8, 66u8, 111u8, 16u8, 7u8, 252u8, 125u8, 100u8, 225u8,
						],
					)
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
			}
			pub mod mmr {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BeefyAuthoritySet<_0> {
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Lsb0;
			}
		}
		pub mod finality_grandpa {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Equivocation<_0, _1, _2> {
				pub round_number: ::core::primitive::u64,
				pub identity: _0,
				pub first: (_1, _2),
				pub second: (_1, _2),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Precommit<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Prevote<_0, _1> {
				pub target_hash: _0,
				pub target_number: _1,
			}
		}
		pub mod frame_support {
			use super::runtime_types;
			pub mod dispatch {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum RawOrigin<_0> {
					#[codec(index = 0)]
					Root,
					#[codec(index = 1)]
					Signed(_0),
					#[codec(index = 2)]
					None,
				}
			}
			pub mod traits {
				use super::runtime_types;
				pub mod misc {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct WrapperKeepOpaque<_0>(
						#[codec(compact)] pub ::core::primitive::u32,
						pub _0,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
							:: subxt :: ext :: codec :: Decode,
							:: subxt :: ext :: codec :: Encode,
							Debug,
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum DispatchClass {
					#[codec(index = 0)]
					Normal,
					#[codec(index = 1)]
					Operational,
					#[codec(index = 2)]
					Mandatory,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DispatchInfo {
					pub weight: ::core::primitive::u64,
					pub class: runtime_types::frame_support::weights::DispatchClass,
					pub pays_fee: runtime_types::frame_support::weights::Pays,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Pays {
					#[codec(index = 0)]
					Yes,
					#[codec(index = 1)]
					No,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PerDispatchClass<_0> {
					pub normal: _0,
					pub operational: _0,
					pub mandatory: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct RuntimeDbWeight {
					pub read: ::core::primitive::u64,
					pub write: ::core::primitive::u64,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct PalletId(pub [::core::primitive::u8; 8usize]);
		}
		pub mod frame_system {
			use super::runtime_types;
			pub mod extensions {
				use super::runtime_types;
				pub mod check_genesis {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckGenesis;
				}
				pub mod check_mortality {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
				}
				pub mod check_non_zero_sender {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckNonZeroSender;
				}
				pub mod check_nonce {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
				}
				pub mod check_spec_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckSpecVersion;
				}
				pub mod check_tx_version {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckTxVersion;
				}
				pub mod check_weight {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct CheckWeight;
				}
			}
			pub mod limits {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlockLength {
					pub max: runtime_types::frame_support::weights::PerDispatchClass<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlockWeights {
					pub base_block: ::core::primitive::u64,
					pub max_block: ::core::primitive::u64,
					pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
						runtime_types::frame_system::limits::WeightsPerClass,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct WeightsPerClass {
					pub base_extrinsic: ::core::primitive::u64,
					pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
					pub max_total: ::core::option::Option<::core::primitive::u64>,
					pub reserved: ::core::option::Option<::core::primitive::u64>,
				}
			}
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
					remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the System pallet"]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Event for the System pallet."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An extrinsic completed successfully."]
					ExtrinsicSuccess {
						dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
					},
					#[codec(index = 1)]
					#[doc = "An extrinsic failed."]
					ExtrinsicFailed {
						dispatch_error: runtime_types::sp_runtime::DispatchError,
						dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
					},
					#[codec(index = 2)]
					#[doc = "`:code` was updated."]
					CodeUpdated,
					#[codec(index = 3)]
					#[doc = "A new account was created."]
					NewAccount { account: ::subxt::ext::sp_core::crypto::AccountId32 },
					#[codec(index = 4)]
					#[doc = "An account was reaped."]
					KilledAccount { account: ::subxt::ext::sp_core::crypto::AccountId32 },
					#[codec(index = 5)]
					#[doc = "On on-chain remark happened."]
					Remarked {
						sender: ::subxt::ext::sp_core::crypto::AccountId32,
						hash: ::subxt::ext::sp_core::H256,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AccountInfo<_0, _1> {
				pub nonce: _0,
				pub consumers: _0,
				pub providers: _0,
				pub sufficients: _0,
				pub data: _1,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct EventRecord<_0, _1> {
				pub phase: runtime_types::frame_system::Phase,
				pub event: _0,
				pub topics: ::std::vec::Vec<_1>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct LastRuntimeUpgradeInfo {
				#[codec(compact)]
				pub spec_version: ::core::primitive::u32,
				pub spec_name: ::std::string::String,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Phase {
				#[codec(index = 0)]
				ApplyExtrinsic(::core::primitive::u32),
				#[codec(index = 1)]
				Finalization,
				#[codec(index = 2)]
				Initialization,
			}
		}
		pub mod pallet_authorship {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Report authority equivocation/misbehavior. This method will verify"]
					#[doc = "the equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence will"]
					#[doc = "be reported."]
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
					#[doc = "Report authority equivocation/misbehavior. This method will verify"]
					#[doc = "the equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence will"]
					#[doc = "be reported."]
					#[doc = "This extrinsic must be called unsigned and it is expected that only"]
					#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
					#[doc = "if the block author is defined it will be defined as the equivocation"]
					#[doc = "reporter."]
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
					#[doc = "Plan an epoch config change. The epoch config change is recorded and will be enacted on"]
					#[doc = "the next call to `enact_epoch_change`. The config will be activated one epoch after."]
					#[doc = "Multiple calls to this method will replace any existing planned config change that had"]
					#[doc = "not been enacted yet."]
					plan_config_change {
						config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
					InvalidEquivocationProof,
					#[codec(index = 1)]
					#[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
					InvalidKeyOwnershipProof,
					#[codec(index = 2)]
					#[doc = "A given equivocation report is valid but already previously reported."]
					DuplicateOffenceReport,
					#[codec(index = 3)]
					#[doc = "Submitted configuration is invalid."]
					InvalidConfiguration,
				}
			}
		}
		pub mod pallet_balances {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Transfer some liquid free balance to another account."]
					#[doc = ""]
					#[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
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
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
						>,
						#[codec(compact)]
						value: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "Set the balances of a given account."]
					#[doc = ""]
					#[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
					#[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
					#[doc = "If the new free or reserved balance is below the existential deposit,"]
					#[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call is `root`."]
					set_balance {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
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
						source: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
						>,
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
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
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
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
						dest: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
						>,
						keep_alive: ::core::primitive::bool,
					},
					#[codec(index = 5)]
					#[doc = "Unreserve some balance from a user by force."]
					#[doc = ""]
					#[doc = "Can only be called by ROOT."]
					force_unreserve {
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
						>,
						amount: ::core::primitive::u128,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "An account was created with some free balance."]
					Endowed {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						free_balance: ::core::primitive::u128,
					},
					#[codec(index = 1)]
					#[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
					#[doc = "resulting in an outright loss."]
					DustLost {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 2)]
					#[doc = "Transfer succeeded."]
					Transfer {
						from: ::subxt::ext::sp_core::crypto::AccountId32,
						to: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 3)]
					#[doc = "A balance was set by root."]
					BalanceSet {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						free: ::core::primitive::u128,
						reserved: ::core::primitive::u128,
					},
					#[codec(index = 4)]
					#[doc = "Some balance was reserved (moved from free to reserved)."]
					Reserved {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 5)]
					#[doc = "Some balance was unreserved (moved from reserved to free)."]
					Unreserved {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 6)]
					#[doc = "Some balance was moved from the reserve of the first account to the second account."]
					#[doc = "Final argument indicates the destination balance type."]
					ReserveRepatriated {
						from: ::subxt::ext::sp_core::crypto::AccountId32,
						to: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
						destination_status:
							runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
					},
					#[codec(index = 7)]
					#[doc = "Some amount was deposited (e.g. for transaction fees)."]
					Deposit {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 8)]
					#[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
					Withdraw {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
					#[codec(index = 9)]
					#[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
					Slashed {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						amount: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct AccountData<_0> {
				pub free: _0,
				pub reserved: _0,
				pub misc_frozen: _0,
				pub fee_frozen: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BalanceLock<_0> {
				pub id: [::core::primitive::u8; 8usize],
				pub amount: _0,
				pub reasons: runtime_types::pallet_balances::Reasons,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Reasons {
				#[codec(index = 0)]
				Fee,
				#[codec(index = 1)]
				Misc,
				#[codec(index = 2)]
				All,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Releases {
				#[codec(index = 0)]
				V1_0_0,
				#[codec(index = 1)]
				V2_0_0,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ReserveData<_0, _1> {
				pub id: _0,
				pub amount: _1,
			}
		}
		pub mod pallet_collective {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
						new_members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						prime: ::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
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
						proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
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
						proposal: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
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
						proposal: ::subxt::ext::sp_core::H256,
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
						proposal_hash: ::subxt::ext::sp_core::H256,
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
					disapprove_proposal { proposal_hash: ::subxt::ext::sp_core::H256 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
					#[doc = "`MemberCount`)."]
					Proposed {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						proposal_index: ::core::primitive::u32,
						proposal_hash: ::subxt::ext::sp_core::H256,
						threshold: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A motion (given hash) has been voted on by given account, leaving"]
					#[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
					Voted {
						account: ::subxt::ext::sp_core::crypto::AccountId32,
						proposal_hash: ::subxt::ext::sp_core::H256,
						voted: ::core::primitive::bool,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "A motion was approved by the required threshold."]
					Approved { proposal_hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 3)]
					#[doc = "A motion was not approved by the required threshold."]
					Disapproved { proposal_hash: ::subxt::ext::sp_core::H256 },
					#[codec(index = 4)]
					#[doc = "A motion was executed; result will be `Ok` if it returned without error."]
					Executed {
						proposal_hash: ::subxt::ext::sp_core::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 5)]
					#[doc = "A single member did some action; result will be `Ok` if it returned without error."]
					MemberExecuted {
						proposal_hash: ::subxt::ext::sp_core::H256,
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 6)]
					#[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
					Closed {
						proposal_hash: ::subxt::ext::sp_core::H256,
						yes: ::core::primitive::u32,
						no: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum RawOrigin<_0> {
				#[codec(index = 0)]
				Members(::core::primitive::u32, ::core::primitive::u32),
				#[codec(index = 1)]
				Member(_0),
				#[codec(index = 2)]
				_Phantom,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
					#[doc = "equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence"]
					#[doc = "will be reported."]
					report_equivocation {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_finality_grandpa::EquivocationProof<
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 1)]
					#[doc = "Report voter equivocation/misbehavior. This method will verify the"]
					#[doc = "equivocation proof and validate the given key ownership proof"]
					#[doc = "against the extracted offender. If both are valid, the offence"]
					#[doc = "will be reported."]
					#[doc = ""]
					#[doc = "This extrinsic must be called unsigned and it is expected that only"]
					#[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
					#[doc = "if the block author is defined it will be defined as the equivocation"]
					#[doc = "reporter."]
					report_equivocation_unsigned {
						equivocation_proof: ::std::boxed::Box<
							runtime_types::sp_finality_grandpa::EquivocationProof<
								::subxt::ext::sp_core::H256,
								::core::primitive::u32,
							>,
						>,
						key_owner_proof: runtime_types::sp_session::MembershipProof,
					},
					#[codec(index = 2)]
					#[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
					#[doc = ""]
					#[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
					#[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
					#[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
					#[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
					#[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
					#[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
					#[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
					#[doc = "block of all validators of the new authority set."]
					#[doc = ""]
					#[doc = "Only callable by root."]
					note_stalled {
						delay: ::core::primitive::u32,
						best_finalized_block_number: ::core::primitive::u32,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
					#[doc = "(either paused or already pending pause)."]
					PauseFailed,
					#[codec(index = 1)]
					#[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
					#[doc = "(either live or already pending resume)."]
					ResumeFailed,
					#[codec(index = 2)]
					#[doc = "Attempt to signal GRANDPA change with one already pending."]
					ChangePending,
					#[codec(index = 3)]
					#[doc = "Cannot signal forced change so soon after last."]
					TooSoon,
					#[codec(index = 4)]
					#[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
					InvalidKeyOwnershipProof,
					#[codec(index = 5)]
					#[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
					InvalidEquivocationProof,
					#[codec(index = 6)]
					#[doc = "A given equivocation report is valid but already previously reported."]
					DuplicateOffenceReport,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "New authority set has been applied."]
					NewAuthorities {
						authority_set: ::std::vec::Vec<(
							runtime_types::sp_finality_grandpa::app::Public,
							::core::primitive::u64,
						)>,
					},
					#[codec(index = 1)]
					#[doc = "Current authority set has been paused."]
					Paused,
					#[codec(index = 2)]
					#[doc = "Current authority set has been resumed."]
					Resumed,
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct StoredPendingChange<_0> {
				pub scheduled_at: _0,
				pub delay: _0,
				pub next_authorities:
					runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<(
						runtime_types::sp_finality_grandpa::app::Public,
						::core::primitive::u64,
					)>,
				pub forced: ::core::option::Option<_0>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "# <weight>"]
					#[doc = "- Complexity: `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is"]
					#[doc = "  length of `heartbeat.network_state.external_address`"]
					#[doc = "  - `O(K)`: decoding of length `K`"]
					#[doc = "  - `O(E)`: decoding/encoding of length `E`"]
					#[doc = "- DbReads: pallet_session `Validators`, pallet_session `CurrentIndex`, `Keys`,"]
					#[doc = "  `ReceivedHeartbeats`"]
					#[doc = "- DbWrites: `ReceivedHeartbeats`"]
					#[doc = "# </weight>"]
					heartbeat {
						heartbeat:
							runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
						signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Non existent public key."]
					InvalidKey,
					#[codec(index = 1)]
					#[doc = "Duplicated heartbeat."]
					DuplicatedHeartbeat,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new heartbeat was received from `AuthorityId`."]
					HeartbeatReceived {
						authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
					},
					#[codec(index = 1)]
					#[doc = "At the end of the session, no offence was committed."]
					AllGood,
					#[codec(index = 2)]
					#[doc = "At the end of the session, at least one validator was found to be offline."]
					SomeOffline {
						offline: ::std::vec::Vec<(::subxt::ext::sp_core::crypto::AccountId32, ())>,
					},
				}
			}
			pub mod sr25519 {
				use super::runtime_types;
				pub mod app_sr25519 {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BoundedOpaqueNetworkState {
				pub peer_id: runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
					::core::primitive::u8,
				>,
				pub external_addresses:
					runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
						runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
							::core::primitive::u8,
						>,
					>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
						new: ::subxt::ext::sp_core::crypto::AccountId32,
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
						new: ::subxt::ext::sp_core::crypto::AccountId32,
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A account index was assigned."]
					IndexAssigned {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						index: ::core::primitive::u32,
					},
					#[codec(index = 1)]
					#[doc = "A account index has been freed up (unassigned)."]
					IndexFreed { index: ::core::primitive::u32 },
					#[codec(index = 2)]
					#[doc = "A account index has been frozen to its current account ID."]
					IndexFrozen {
						index: ::core::primitive::u32,
						who: ::subxt::ext::sp_core::crypto::AccountId32,
					},
				}
			}
		}
		pub mod pallet_membership {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Add a member `who` to the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::AddOrigin`."]
					add_member { who: ::subxt::ext::sp_core::crypto::AccountId32 },
					#[codec(index = 1)]
					#[doc = "Remove a member `who` from the set."]
					#[doc = ""]
					#[doc = "May only be called from `T::RemoveOrigin`."]
					remove_member { who: ::subxt::ext::sp_core::crypto::AccountId32 },
					#[codec(index = 2)]
					#[doc = "Swap out one member `remove` for another `add`."]
					#[doc = ""]
					#[doc = "May only be called from `T::SwapOrigin`."]
					#[doc = ""]
					#[doc = "Prime membership is *not* passed from `remove` to `add`, if extant."]
					swap_member {
						remove: ::subxt::ext::sp_core::crypto::AccountId32,
						add: ::subxt::ext::sp_core::crypto::AccountId32,
					},
					#[codec(index = 3)]
					#[doc = "Change the membership to a new set, disregarding the existing membership. Be nice and"]
					#[doc = "pass `members` pre-sorted."]
					#[doc = ""]
					#[doc = "May only be called from `T::ResetOrigin`."]
					reset_members {
						members: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
					},
					#[codec(index = 4)]
					#[doc = "Swap out the sending member for some other key `new`."]
					#[doc = ""]
					#[doc = "May only be called from `Signed` origin of a current member."]
					#[doc = ""]
					#[doc = "Prime membership is passed from the origin account to `new`, if extant."]
					change_key { new: ::subxt::ext::sp_core::crypto::AccountId32 },
					#[codec(index = 5)]
					#[doc = "Set the prime member. Must be a current member."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					set_prime { who: ::subxt::ext::sp_core::crypto::AccountId32 },
					#[codec(index = 6)]
					#[doc = "Remove the prime member if it exists."]
					#[doc = ""]
					#[doc = "May only be called from `T::PrimeOrigin`."]
					clear_prime,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Already a member."]
					AlreadyMember,
					#[codec(index = 1)]
					#[doc = "Not a member."]
					NotMember,
					#[codec(index = 2)]
					#[doc = "Too many members."]
					TooManyMembers,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
		pub mod pallet_multisig {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Immediately dispatch a multi-signature call using a single approval from the caller."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `other_signatories`: The accounts (other than the sender) who are part of the"]
					#[doc = "multi-signature, but do not participate in the approval process."]
					#[doc = "- `call`: The call to be executed."]
					#[doc = ""]
					#[doc = "Result is equivalent to the dispatched result."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "O(Z + C) where Z is the length of the call and C its execution weight."]
					#[doc = "-------------------------------"]
					#[doc = "- DB Weight: None"]
					#[doc = "- Plus Call Weight"]
					#[doc = "# </weight>"]
					as_multi_threshold_1 {
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
					},
					#[codec(index = 1)]
					#[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
					#[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
					#[doc = ""]
					#[doc = "If there are enough, then dispatch the call."]
					#[doc = ""]
					#[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
					#[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
					#[doc = "is cancelled."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
					#[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
					#[doc = "dispatch. May not be empty."]
					#[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
					#[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
					#[doc = "transaction index) of the first approval transaction."]
					#[doc = "- `call`: The call to be executed."]
					#[doc = ""]
					#[doc = "NOTE: Unless this is the final approval, you will generally want to use"]
					#[doc = "`approve_as_multi` instead, since it only requires a hash of the call."]
					#[doc = ""]
					#[doc = "Result is equivalent to the dispatched result if `threshold` is exactly `1`. Otherwise"]
					#[doc = "on success, result is `Ok` and the result from the interior call, if it was executed,"]
					#[doc = "may be found in the deposited `MultisigExecuted` event."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(S + Z + Call)`."]
					#[doc = "- Up to one balance-reserve or unreserve operation."]
					#[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
					#[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
					#[doc = "- One call encode & hash, both of complexity `O(Z)` where `Z` is tx-len."]
					#[doc = "- One encode & hash, both of complexity `O(S)`."]
					#[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
					#[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
					#[doc = "- One event."]
					#[doc = "- The weight of the `call`."]
					#[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
					#[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
					#[doc = "-------------------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "    - Reads: Multisig Storage, [Caller Account], Calls (if `store_call`)"]
					#[doc = "    - Writes: Multisig Storage, [Caller Account], Calls (if `store_call`)"]
					#[doc = "- Plus Call Weight"]
					#[doc = "# </weight>"]
					as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call:
							::subxt::utils::WrapperKeepOpaque<runtime_types::rococo_runtime::Call>,
						store_call: ::core::primitive::bool,
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 2)]
					#[doc = "Register approval for a dispatch to be made from a deterministic composite account if"]
					#[doc = "approved by a total of `threshold - 1` of `other_signatories`."]
					#[doc = ""]
					#[doc = "Payment: `DepositBase` will be reserved if this is the first approval, plus"]
					#[doc = "`threshold` times `DepositFactor`. It is returned once this dispatch happens or"]
					#[doc = "is cancelled."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
					#[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
					#[doc = "dispatch. May not be empty."]
					#[doc = "- `maybe_timepoint`: If this is the first approval, then this must be `None`. If it is"]
					#[doc = "not the first approval, then it must be `Some`, with the timepoint (block number and"]
					#[doc = "transaction index) of the first approval transaction."]
					#[doc = "- `call_hash`: The hash of the call to be executed."]
					#[doc = ""]
					#[doc = "NOTE: If this is the final approval, you will want to use `as_multi` instead."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(S)`."]
					#[doc = "- Up to one balance-reserve or unreserve operation."]
					#[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
					#[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
					#[doc = "- One encode & hash, both of complexity `O(S)`."]
					#[doc = "- Up to one binary search and insert (`O(logS + S)`)."]
					#[doc = "- I/O: 1 read `O(S)`, up to 1 mutate `O(S)`. Up to one remove."]
					#[doc = "- One event."]
					#[doc = "- Storage: inserts one item, value size bounded by `MaxSignatories`, with a deposit"]
					#[doc = "  taken for its lifetime of `DepositBase + threshold * DepositFactor`."]
					#[doc = "----------------------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "    - Read: Multisig Storage, [Caller Account]"]
					#[doc = "    - Write: Multisig Storage, [Caller Account]"]
					#[doc = "# </weight>"]
					approve_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						maybe_timepoint: ::core::option::Option<
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						>,
						call_hash: [::core::primitive::u8; 32usize],
						max_weight: ::core::primitive::u64,
					},
					#[codec(index = 3)]
					#[doc = "Cancel a pre-existing, on-going multisig transaction. Any deposit reserved previously"]
					#[doc = "for this operation will be unreserved on success."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "- `threshold`: The total number of approvals for this dispatch before it is executed."]
					#[doc = "- `other_signatories`: The accounts (other than the sender) who can approve this"]
					#[doc = "dispatch. May not be empty."]
					#[doc = "- `timepoint`: The timepoint (block number and transaction index) of the first approval"]
					#[doc = "transaction for this dispatch."]
					#[doc = "- `call_hash`: The hash of the call to be executed."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "- `O(S)`."]
					#[doc = "- Up to one balance-reserve or unreserve operation."]
					#[doc = "- One passthrough operation, one insert, both `O(S)` where `S` is the number of"]
					#[doc = "  signatories. `S` is capped by `MaxSignatories`, with weight being proportional."]
					#[doc = "- One encode & hash, both of complexity `O(S)`."]
					#[doc = "- One event."]
					#[doc = "- I/O: 1 read `O(S)`, one remove."]
					#[doc = "- Storage: removes one item."]
					#[doc = "----------------------------------"]
					#[doc = "- DB Weight:"]
					#[doc = "    - Read: Multisig Storage, [Caller Account], Refund Account, Calls"]
					#[doc = "    - Write: Multisig Storage, [Caller Account], Refund Account, Calls"]
					#[doc = "# </weight>"]
					cancel_as_multi {
						threshold: ::core::primitive::u16,
						other_signatories:
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Threshold must be 2 or greater."]
					MinimumThreshold,
					#[codec(index = 1)]
					#[doc = "Call is already approved by this signatory."]
					AlreadyApproved,
					#[codec(index = 2)]
					#[doc = "Call doesn't need any (more) approvals."]
					NoApprovalsNeeded,
					#[codec(index = 3)]
					#[doc = "There are too few signatories in the list."]
					TooFewSignatories,
					#[codec(index = 4)]
					#[doc = "There are too many signatories in the list."]
					TooManySignatories,
					#[codec(index = 5)]
					#[doc = "The signatories were provided out of order; they should be ordered."]
					SignatoriesOutOfOrder,
					#[codec(index = 6)]
					#[doc = "The sender was contained in the other signatories; it shouldn't be."]
					SenderInSignatories,
					#[codec(index = 7)]
					#[doc = "Multisig operation not found when attempting to cancel."]
					NotFound,
					#[codec(index = 8)]
					#[doc = "Only the account that originally created the multisig is able to cancel it."]
					NotOwner,
					#[codec(index = 9)]
					#[doc = "No timepoint was given, yet the multisig operation is already underway."]
					NoTimepoint,
					#[codec(index = 10)]
					#[doc = "A different timepoint was given to the multisig operation that is underway."]
					WrongTimepoint,
					#[codec(index = 11)]
					#[doc = "A timepoint was given, yet no multisig operation is underway."]
					UnexpectedTimepoint,
					#[codec(index = 12)]
					#[doc = "The maximum weight information provided was too low."]
					MaxWeightTooLow,
					#[codec(index = 13)]
					#[doc = "The data to be stored is already stored."]
					AlreadyStored,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A new multisig operation has begun."]
					NewMultisig {
						approving: ::subxt::ext::sp_core::crypto::AccountId32,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 1)]
					#[doc = "A multisig operation has been approved by someone."]
					MultisigApproval {
						approving: ::subxt::ext::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
					#[codec(index = 2)]
					#[doc = "A multisig operation has been executed."]
					MultisigExecuted {
						approving: ::subxt::ext::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 3)]
					#[doc = "A multisig operation has been cancelled."]
					MultisigCancelled {
						cancelling: ::subxt::ext::sp_core::crypto::AccountId32,
						timepoint:
							runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
						multisig: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: [::core::primitive::u8; 32usize],
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Multisig<_0, _1, _2> {
				pub when: runtime_types::pallet_multisig::Timepoint<_0>,
				pub deposit: _1,
				pub depositor: _2,
				pub approvals: ::std::vec::Vec<_2>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Timepoint<_0> {
				pub height: _0,
				pub index: _0,
			}
		}
		pub mod pallet_offences {
			use super::runtime_types;
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Events type."]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
					#[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
					#[doc = "\\[kind, timeslot\\]."]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					#[doc = "Dispatch the given `call` from an account that the sender is authorised for through"]
					#[doc = "`add_proxy`."]
					#[doc = ""]
					#[doc = "Removes any corresponding announcement(s)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
					#[doc = "- `call`: The call to be made by the `real` account."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of the number of proxies the user has (P)."]
					#[doc = "# </weight>"]
					proxy {
						real: ::subxt::ext::sp_core::crypto::AccountId32,
						force_proxy_type:
							::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
						call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
					},
					#[codec(index = 1)]
					#[doc = "Register a proxy account for the sender that is able to make calls on its behalf."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `proxy`: The account that the `caller` would like to make a proxy."]
					#[doc = "- `proxy_type`: The permissions allowed for this proxy account."]
					#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
					#[doc = "zero."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of the number of proxies the user has (P)."]
					#[doc = "# </weight>"]
					add_proxy {
						delegate: ::subxt::ext::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Unregister a proxy account for the sender."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `proxy`: The account that the `caller` would like to remove as a proxy."]
					#[doc = "- `proxy_type`: The permissions currently enabled for the removed proxy account."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of the number of proxies the user has (P)."]
					#[doc = "# </weight>"]
					remove_proxy {
						delegate: ::subxt::ext::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 3)]
					#[doc = "Unregister all proxy accounts for the sender."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "WARNING: This may be called on accounts created by `anonymous`, however if done, then"]
					#[doc = "the unreserved fees will be inaccessible. **All access to this account will be lost.**"]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of the number of proxies the user has (P)."]
					#[doc = "# </weight>"]
					remove_proxies,
					#[codec(index = 4)]
					#[doc = "Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and"]
					#[doc = "initialize it with a proxy of `proxy_type` for `origin` sender."]
					#[doc = ""]
					#[doc = "Requires a `Signed` origin."]
					#[doc = ""]
					#[doc = "- `proxy_type`: The type of the proxy that the sender will be registered as over the"]
					#[doc = "new account. This will almost always be the most permissive `ProxyType` possible to"]
					#[doc = "allow for maximum flexibility."]
					#[doc = "- `index`: A disambiguation index, in case this is called multiple times in the same"]
					#[doc = "transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just"]
					#[doc = "want to use `0`."]
					#[doc = "- `delay`: The announcement period required of the initial proxy. Will generally be"]
					#[doc = "zero."]
					#[doc = ""]
					#[doc = "Fails with `Duplicate` if this has already been called in this transaction, from the"]
					#[doc = "same sender, with the same parameters."]
					#[doc = ""]
					#[doc = "Fails if there are insufficient funds to pay for deposit."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of the number of proxies the user has (P)."]
					#[doc = "# </weight>"]
					#[doc = "TODO: Might be over counting 1 read"]
					anonymous {
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						delay: ::core::primitive::u32,
						index: ::core::primitive::u16,
					},
					#[codec(index = 5)]
					#[doc = "Removes a previously spawned anonymous proxy."]
					#[doc = ""]
					#[doc = "WARNING: **All access to this account will be lost.** Any funds held in it will be"]
					#[doc = "inaccessible."]
					#[doc = ""]
					#[doc = "Requires a `Signed` origin, and the sender account must have been created by a call to"]
					#[doc = "`anonymous` with corresponding parameters."]
					#[doc = ""]
					#[doc = "- `spawner`: The account that originally called `anonymous` to create this account."]
					#[doc = "- `index`: The disambiguation index originally passed to `anonymous`. Probably `0`."]
					#[doc = "- `proxy_type`: The proxy type originally passed to `anonymous`."]
					#[doc = "- `height`: The height of the chain when the call to `anonymous` was processed."]
					#[doc = "- `ext_index`: The extrinsic index in which the call to `anonymous` was processed."]
					#[doc = ""]
					#[doc = "Fails with `NoPermission` in case the caller is not a previously created anonymous"]
					#[doc = "account whose `anonymous` call has corresponding parameters."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of the number of proxies the user has (P)."]
					#[doc = "# </weight>"]
					kill_anonymous {
						spawner: ::subxt::ext::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						index: ::core::primitive::u16,
						#[codec(compact)]
						height: ::core::primitive::u32,
						#[codec(compact)]
						ext_index: ::core::primitive::u32,
					},
					#[codec(index = 6)]
					#[doc = "Publish the hash of a proxy-call that will be made in the future."]
					#[doc = ""]
					#[doc = "This must be called some number of blocks before the corresponding `proxy` is attempted"]
					#[doc = "if the delay associated with the proxy relationship is greater than zero."]
					#[doc = ""]
					#[doc = "No more than `MaxPending` announcements may be made at any one time."]
					#[doc = ""]
					#[doc = "This will take a deposit of `AnnouncementDepositFactor` as well as"]
					#[doc = "`AnnouncementDepositBase` if there are no other pending announcements."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_ and a proxy of `real`."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of:"]
					#[doc = "- A: the number of announcements made."]
					#[doc = "- P: the number of proxies the user has."]
					#[doc = "# </weight>"]
					announce {
						real: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 7)]
					#[doc = "Remove a given announcement."]
					#[doc = ""]
					#[doc = "May be called by a proxy account to remove a call they previously announced and return"]
					#[doc = "the deposit."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `call_hash`: The hash of the call to be made by the `real` account."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of:"]
					#[doc = "- A: the number of announcements made."]
					#[doc = "- P: the number of proxies the user has."]
					#[doc = "# </weight>"]
					remove_announcement {
						real: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 8)]
					#[doc = "Remove the given announcement of a delegate."]
					#[doc = ""]
					#[doc = "May be called by a target (proxied) account to remove a call that one of their delegates"]
					#[doc = "(`delegate`) has announced they want to execute. The deposit is returned."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `delegate`: The account that previously announced the call."]
					#[doc = "- `call_hash`: The hash of the call to be made."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of:"]
					#[doc = "- A: the number of announcements made."]
					#[doc = "- P: the number of proxies the user has."]
					#[doc = "# </weight>"]
					reject_announcement {
						delegate: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 9)]
					#[doc = "Dispatch the given `call` from an account that the sender is authorized for through"]
					#[doc = "`add_proxy`."]
					#[doc = ""]
					#[doc = "Removes any corresponding announcement(s)."]
					#[doc = ""]
					#[doc = "The dispatch origin for this call must be _Signed_."]
					#[doc = ""]
					#[doc = "Parameters:"]
					#[doc = "- `real`: The account that the proxy will make a call on behalf of."]
					#[doc = "- `force_proxy_type`: Specify the exact proxy type to be used and checked for this call."]
					#[doc = "- `call`: The call to be made by the `real` account."]
					#[doc = ""]
					#[doc = "# <weight>"]
					#[doc = "Weight is a function of:"]
					#[doc = "- A: the number of announcements made."]
					#[doc = "- P: the number of proxies the user has."]
					#[doc = "# </weight>"]
					proxy_announced {
						delegate: ::subxt::ext::sp_core::crypto::AccountId32,
						real: ::subxt::ext::sp_core::crypto::AccountId32,
						force_proxy_type:
							::core::option::Option<runtime_types::rococo_runtime::ProxyType>,
						call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "There are too many proxies registered or too many announcements pending."]
					TooMany,
					#[codec(index = 1)]
					#[doc = "Proxy registration not found."]
					NotFound,
					#[codec(index = 2)]
					#[doc = "Sender is not a proxy of the account to be proxied."]
					NotProxy,
					#[codec(index = 3)]
					#[doc = "A call which is incompatible with the proxy type's filter was attempted."]
					Unproxyable,
					#[codec(index = 4)]
					#[doc = "Account is already a proxy."]
					Duplicate,
					#[codec(index = 5)]
					#[doc = "Call may not be made by proxy because it may escalate its privileges."]
					NoPermission,
					#[codec(index = 6)]
					#[doc = "Announcement, if made at all, was made too recently."]
					Unannounced,
					#[codec(index = 7)]
					#[doc = "Cannot add self as proxy."]
					NoSelfProxy,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A proxy was executed correctly, with the given."]
					ProxyExecuted {
						result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "Anonymous account has been created by new proxy with given"]
					#[doc = "disambiguation index and proxy type."]
					AnonymousCreated {
						anonymous: ::subxt::ext::sp_core::crypto::AccountId32,
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						disambiguation_index: ::core::primitive::u16,
					},
					#[codec(index = 2)]
					#[doc = "An announcement was placed to make a call in the future."]
					Announced {
						real: ::subxt::ext::sp_core::crypto::AccountId32,
						proxy: ::subxt::ext::sp_core::crypto::AccountId32,
						call_hash: ::subxt::ext::sp_core::H256,
					},
					#[codec(index = 3)]
					#[doc = "A proxy was added."]
					ProxyAdded {
						delegator: ::subxt::ext::sp_core::crypto::AccountId32,
						delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
					#[codec(index = 4)]
					#[doc = "A proxy was removed."]
					ProxyRemoved {
						delegator: ::subxt::ext::sp_core::crypto::AccountId32,
						delegatee: ::subxt::ext::sp_core::crypto::AccountId32,
						proxy_type: runtime_types::rococo_runtime::ProxyType,
						delay: ::core::primitive::u32,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Announcement<_0, _1, _2> {
				pub real: _0,
				pub call_hash: _1,
				pub height: _2,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
						keys: runtime_types::rococo_runtime::SessionKeys,
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the session pallet."]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
					sudo { call: ::std::boxed::Box<runtime_types::rococo_runtime::Call> },
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
						call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
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
						new: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
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
						who: ::subxt::ext::sp_runtime::MultiAddress<
							::subxt::ext::sp_core::crypto::AccountId32,
							(),
						>,
						call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Error for the Sudo pallet"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Sender must be the Sudo account"]
					RequireSudo,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A sudo just took place. \\[result\\]"]
					Sudid {
						sudo_result:
							::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
					},
					#[codec(index = 1)]
					#[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
					KeyChanged {
						old_sudoer:
							::core::option::Option<::subxt::ext::sp_core::crypto::AccountId32>,
					},
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
			pub mod pallet {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
				pub enum Event {
					#[codec(index = 0)]
					#[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
					#[doc = "has been paid by `who`."]
					TransactionFeePaid {
						who: ::subxt::ext::sp_core::crypto::AccountId32,
						actual_fee: ::core::primitive::u128,
						tip: ::core::primitive::u128,
					},
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
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
					batch { calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call> },
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
						call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
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
					batch_all { calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call> },
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
						as_origin: ::std::boxed::Box<runtime_types::rococo_runtime::OriginCaller>,
						call: ::std::boxed::Box<runtime_types::rococo_runtime::Call>,
					},
					#[codec(index = 4)]
					#[doc = "Send a batch of dispatch calls."]
					#[doc = "Unlike `batch`, it allows errors and won't interrupt."]
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
					force_batch { calls: ::std::vec::Vec<runtime_types::rococo_runtime::Call> },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
				pub enum Error {
					#[codec(index = 0)]
					#[doc = "Too many calls batched."]
					TooManyCalls,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
					#[doc = "Batch of dispatches completed but has errors."]
					BatchCompletedWithErrors,
					#[codec(index = 3)]
					#[doc = "A single item within a Batch of dispatches has completed with no error."]
					ItemCompleted,
					#[codec(index = 4)]
					#[doc = "A single item within a Batch of dispatches has completed with error."]
					ItemFailed { error: runtime_types::sp_runtime::DispatchError },
					#[codec(index = 5)]
					#[doc = "A call was dispatched."]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
				pub enum Call {
					#[codec(index = 0)]
					send {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
					},
					#[codec(index = 1)]
					#[doc = "Teleport some assets from the local chain to some destination chain."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
					},
					#[codec(index = 2)]
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
					#[doc = "chain and forward a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`. The weight limit for fees is not provided and thus is unlimited,"]
					#[doc = "with all fees taken as needed from the asset."]
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
					#[doc = "Transfer some assets from the local chain to the sovereign account of a destination"]
					#[doc = "chain and forward a notification XCM."]
					#[doc = ""]
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
					#[doc = "at risk."]
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
					#[doc = "Fee payment on the destination side is made from the asset in the `assets` vector of"]
					#[doc = "index `fee_asset_item`, up to enough to pay for `weight_limit` of weight. If more weight"]
					#[doc = "is needed than `weight_limit`, then the operation will fail and the assets send may be"]
					#[doc = "at risk."]
					#[doc = ""]
					#[doc = "- `origin`: Must be capable of withdrawing the `assets` and executing XCM."]
					#[doc = "- `dest`: Destination context for the assets. Will typically be `X2(Parent, Parachain(..))` to send"]
					#[doc = "  from parachain to parachain, or `X1(Parachain(..))` to send from relay to parachain."]
					#[doc = "- `beneficiary`: A beneficiary location for the assets in the context of `dest`. Will generally be"]
					#[doc = "  an `AccountId32` value."]
					#[doc = "- `assets`: The assets to be withdrawn. The first item should be the currency used to to pay the fee on the"]
					#[doc = "  `dest` side. May not be empty."]
					#[doc = "- `fee_asset_item`: The index into `assets` of the item which should be used to pay"]
					#[doc = "  fees."]
					#[doc = "- `weight_limit`: The remote-side weight limit, if any, for the XCM fee purchase."]
					limited_teleport_assets {
						dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
						assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
						fee_asset_item: ::core::primitive::u32,
						weight_limit: runtime_types::xcm::v2::WeightLimit,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
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
						::subxt::ext::sp_core::H256,
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Origin {
					#[codec(index = 0)]
					Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
					#[codec(index = 1)]
					Response(runtime_types::xcm::v1::multilocation::MultiLocation),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					Ready { response: runtime_types::xcm::VersionedResponse, at: _0 },
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct CandidateHash(pub ::subxt::ext::sp_core::H256);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundDownwardMessage<_0> {
				pub sent_at: _0,
				pub msg: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct InboundHrmpMessage<_0> {
				pub sent_at: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct OutboundHrmpMessage<_0> {
				pub recipient: _0,
				pub data: ::std::vec::Vec<::core::primitive::u8>,
			}
		}
		pub mod polkadot_parachain {
			use super::runtime_types;
			pub mod primitives {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct HrmpChannelId {
					pub sender: runtime_types::polkadot_parachain::primitives::Id,
					pub recipient: runtime_types::polkadot_parachain::primitives::Id,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Id(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ValidationCode(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ValidationCodeHash(pub ::subxt::ext::sp_core::H256);
			}
		}
		pub mod polkadot_primitives {
			use super::runtime_types;
			pub mod v2 {
				use super::runtime_types;
				pub mod assignment_app {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
				}
				pub mod collator_app {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
				pub mod signed {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct UncheckedSigned<_0, _1> {
						pub payload: _0,
						pub validator_index: runtime_types::polkadot_primitives::v2::ValidatorIndex,
						pub signature:
							runtime_types::polkadot_primitives::v2::validator_app::Signature,
						#[codec(skip)]
						pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
					}
				}
				pub mod validator_app {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Public(pub runtime_types::sp_core::sr25519::Public);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AvailabilityBitfield(
					pub  ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BackedCandidate<_0> {
					pub candidate:
						runtime_types::polkadot_primitives::v2::CommittedCandidateReceipt<_0>,
					pub validity_votes: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::ValidityAttestation,
					>,
					pub validator_indices: ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CandidateDescriptor<_0> {
					pub para_id: runtime_types::polkadot_parachain::primitives::Id,
					pub relay_parent: _0,
					pub collator: runtime_types::polkadot_primitives::v2::collator_app::Public,
					pub persisted_validation_data_hash: _0,
					pub pov_hash: _0,
					pub erasure_root: _0,
					pub signature: runtime_types::polkadot_primitives::v2::collator_app::Signature,
					pub para_head: _0,
					pub validation_code_hash:
						runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CandidateReceipt<_0> {
					pub descriptor: runtime_types::polkadot_primitives::v2::CandidateDescriptor<_0>,
					pub commitments_hash: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CommittedCandidateReceipt<_0> {
					pub descriptor: runtime_types::polkadot_primitives::v2::CandidateDescriptor<_0>,
					pub commitments: runtime_types::polkadot_primitives::v2::CandidateCommitments<
						::core::primitive::u32,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct CoreIndex(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum CoreOccupied {
					#[codec(index = 0)]
					Parathread(runtime_types::polkadot_primitives::v2::ParathreadEntry),
					#[codec(index = 1)]
					Parachain,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DisputeState<_0> {
					pub validators_for: ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
					pub validators_against: ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
					pub start: _0,
					pub concluded_at: ::core::option::Option<_0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum DisputeStatement {
					#[codec(index = 0)]
					Valid(runtime_types::polkadot_primitives::v2::ValidDisputeStatementKind),
					#[codec(index = 1)]
					Invalid(runtime_types::polkadot_primitives::v2::InvalidDisputeStatementKind),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DisputeStatementSet {
					pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
					pub session: ::core::primitive::u32,
					pub statements: ::std::vec::Vec<(
						runtime_types::polkadot_primitives::v2::DisputeStatement,
						runtime_types::polkadot_primitives::v2::ValidatorIndex,
						runtime_types::polkadot_primitives::v2::validator_app::Signature,
					)>,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct GroupIndex(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct InherentData<_0> {
					pub bitfields: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::signed::UncheckedSigned<
							runtime_types::polkadot_primitives::v2::AvailabilityBitfield,
							runtime_types::polkadot_primitives::v2::AvailabilityBitfield,
						>,
					>,
					pub backed_candidates: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::BackedCandidate<
							::subxt::ext::sp_core::H256,
						>,
					>,
					pub disputes: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::DisputeStatementSet,
					>,
					pub parent_header: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum InvalidDisputeStatementKind {
					#[codec(index = 0)]
					Explicit,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ParathreadClaim(
					pub runtime_types::polkadot_parachain::primitives::Id,
					pub runtime_types::polkadot_primitives::v2::collator_app::Public,
				);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ParathreadEntry {
					pub claim: runtime_types::polkadot_primitives::v2::ParathreadClaim,
					pub retries: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PvfCheckStatement {
					pub accept: ::core::primitive::bool,
					pub subject: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
					pub session_index: ::core::primitive::u32,
					pub validator_index: runtime_types::polkadot_primitives::v2::ValidatorIndex,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ScrapedOnChainVotes<_0> {
					pub session: ::core::primitive::u32,
					pub backing_validators_per_candidate: ::std::vec::Vec<(
						runtime_types::polkadot_primitives::v2::CandidateReceipt<_0>,
						::std::vec::Vec<(
							runtime_types::polkadot_primitives::v2::ValidatorIndex,
							runtime_types::polkadot_primitives::v2::ValidityAttestation,
						)>,
					)>,
					pub disputes: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::DisputeStatementSet,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct SessionInfo {
					pub active_validator_indices:
						::std::vec::Vec<runtime_types::polkadot_primitives::v2::ValidatorIndex>,
					pub random_seed: [::core::primitive::u8; 32usize],
					pub dispute_period: ::core::primitive::u32,
					pub validators: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::validator_app::Public,
					>,
					pub discovery_keys:
						::std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
					pub assignment_keys: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::assignment_app::Public,
					>,
					pub validator_groups: ::std::vec::Vec<
						::std::vec::Vec<runtime_types::polkadot_primitives::v2::ValidatorIndex>,
					>,
					pub n_cores: ::core::primitive::u32,
					pub zeroth_delay_tranche_width: ::core::primitive::u32,
					pub relay_vrf_modulo_samples: ::core::primitive::u32,
					pub n_delay_tranches: ::core::primitive::u32,
					pub no_show_slots: ::core::primitive::u32,
					pub needed_approvals: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum UpgradeGoAhead {
					#[codec(index = 0)]
					Abort,
					#[codec(index = 1)]
					GoAhead,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum UpgradeRestriction {
					#[codec(index = 0)]
					Present,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum ValidDisputeStatementKind {
					#[codec(index = 0)]
					Explicit,
					#[codec(index = 1)]
					BackingSeconded(::subxt::ext::sp_core::H256),
					#[codec(index = 2)]
					BackingValid(::subxt::ext::sp_core::H256),
					#[codec(index = 3)]
					ApprovalChecking,
				}
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct ValidatorIndex(pub ::core::primitive::u32);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum ValidityAttestation {
					#[codec(index = 1)]
					Implicit(runtime_types::polkadot_primitives::v2::validator_app::Signature),
					#[codec(index = 2)]
					Explicit(runtime_types::polkadot_primitives::v2::validator_app::Signature),
				}
			}
		}
		pub mod polkadot_runtime_common {
			use super::runtime_types;
			pub mod assigned_slots {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						# [codec (index = 0)] # [doc = "Assign a permanent parachain slot and immediately create a lease for it."] assign_perm_parachain_slot { id : runtime_types :: polkadot_parachain :: primitives :: Id , } , # [codec (index = 1)] # [doc = "Assign a temporary parachain slot. The function tries to create a lease for it"] # [doc = "immediately if `SlotLeasePeriodStart::Current` is specified, and if the number"] # [doc = "of currently active temporary slots is below `MaxTemporarySlotPerLeasePeriod`."] assign_temp_parachain_slot { id : runtime_types :: polkadot_parachain :: primitives :: Id , lease_period_start : runtime_types :: polkadot_runtime_common :: assigned_slots :: SlotLeasePeriodStart , } , # [codec (index = 2)] # [doc = "Unassign a permanent or temporary parachain slot"] unassign_parachain_slot { id : runtime_types :: polkadot_parachain :: primitives :: Id , } , }
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The specified parachain or parathread is not registered."]
						ParaDoesntExist,
						#[codec(index = 1)]
						#[doc = "Not a parathread."]
						NotParathread,
						#[codec(index = 2)]
						#[doc = "Cannot upgrade parathread."]
						CannotUpgrade,
						#[codec(index = 3)]
						#[doc = "Cannot downgrade parachain."]
						CannotDowngrade,
						#[codec(index = 4)]
						#[doc = "Permanent or Temporary slot already assigned."]
						SlotAlreadyAssigned,
						#[codec(index = 5)]
						#[doc = "Permanent or Temporary slot has not been assigned."]
						SlotNotAssigned,
						#[codec(index = 6)]
						#[doc = "An ongoing lease already exists."]
						OngoingLeaseExists,
						#[codec(index = 7)]
						MaxPermanentSlotsExceeded,
						#[codec(index = 8)]
						MaxTemporarySlotsExceeded,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "A para was assigned a permanent parachain slot"]
						PermanentSlotAssigned(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 1)]
						#[doc = "A para was assigned a temporary parachain slot"]
						TemporarySlotAssigned(runtime_types::polkadot_parachain::primitives::Id),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ParachainTemporarySlot<_0, _1> {
					pub manager: _0,
					pub period_begin: _1,
					pub period_count: _1,
					pub last_lease: ::core::option::Option<_1>,
					pub lease_count: _1,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Create a new auction."]
						#[doc = ""]
						#[doc = "This can only happen when there isn't already an auction in progress and may only be"]
						#[doc = "called by the root origin. Accepts the `duration` of this auction and the"]
						#[doc = "`lease_period_index` of the initial lease period of the four that are to be auctioned."]
						new_auction {
							#[codec(compact)]
							duration: ::core::primitive::u32,
							#[codec(compact)]
							lease_period_index: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						#[doc = "Make a new bid from an account (including a parachain account) for deploying a new"]
						#[doc = "parachain."]
						#[doc = ""]
						#[doc = "Multiple simultaneous bids from the same bidder are allowed only as long as all active"]
						#[doc = "bids overlap each other (i.e. are mutually exclusive). Bids cannot be redacted."]
						#[doc = ""]
						#[doc = "- `sub` is the sub-bidder ID, allowing for multiple competing bids to be made by (and"]
						#[doc = "funded by) the same account."]
						#[doc = "- `auction_index` is the index of the auction to bid on. Should just be the present"]
						#[doc = "value of `AuctionCounter`."]
						#[doc = "- `first_slot` is the first lease period index of the range to bid on. This is the"]
						#[doc = "absolute lease period index value, not an auction-specific offset."]
						#[doc = "- `last_slot` is the last lease period index of the range to bid on. This is the"]
						#[doc = "absolute lease period index value, not an auction-specific offset."]
						#[doc = "- `amount` is the amount to bid to be held as deposit for the parachain should the"]
						#[doc = "bid win. This amount is held throughout the range."]
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
						#[doc = "Cancel an in-progress auction."]
						#[doc = ""]
						#[doc = "Can only be called by Root origin."]
						cancel_auction,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "This auction is already in progress."]
						AuctionInProgress,
						#[codec(index = 1)]
						#[doc = "The lease period is in the past."]
						LeasePeriodInPast,
						#[codec(index = 2)]
						#[doc = "Para is not registered"]
						ParaNotRegistered,
						#[codec(index = 3)]
						#[doc = "Not a current auction."]
						NotCurrentAuction,
						#[codec(index = 4)]
						#[doc = "Not an auction."]
						NotAuction,
						#[codec(index = 5)]
						#[doc = "Auction has already ended."]
						AuctionEnded,
						#[codec(index = 6)]
						#[doc = "The para is already leased out for part of this range."]
						AlreadyLeasedOut,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "An auction started. Provides its index and the block number where it will begin to"]
						#[doc = "close and the first lease period of the quadruplet that is auctioned."]
						AuctionStarted {
							auction_index: ::core::primitive::u32,
							lease_period: ::core::primitive::u32,
							ending: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						#[doc = "An auction ended. All funds become unreserved."]
						AuctionClosed { auction_index: ::core::primitive::u32 },
						#[codec(index = 2)]
						#[doc = "Funds were reserved for a winning bid. First balance is the extra amount reserved."]
						#[doc = "Second is the total."]
						Reserved {
							bidder: ::subxt::ext::sp_core::crypto::AccountId32,
							extra_reserved: ::core::primitive::u128,
							total_amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						#[doc = "Funds were unreserved since bidder is no longer active. `[bidder, amount]`"]
						Unreserved {
							bidder: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 4)]
						#[doc = "Someone attempted to lease the same slot twice for a parachain. The amount is held in reserve"]
						#[doc = "but no parachain slot has been leased."]
						ReserveConfiscated {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							leaser: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 5)]
						#[doc = "A new bid has been accepted as the current winner."]
						BidAccepted {
							bidder: ::subxt::ext::sp_core::crypto::AccountId32,
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							amount: ::core::primitive::u128,
							first_slot: ::core::primitive::u32,
							last_slot: ::core::primitive::u32,
						},
						#[codec(index = 6)]
						#[doc = "The winning offset was chosen for an auction. This will map into the `Winning` storage map."]
						WinningOffset {
							auction_index: ::core::primitive::u32,
							block_number: ::core::primitive::u32,
						},
					}
				}
			}
			pub mod crowdloan {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Create a new crowdloaning campaign for a parachain slot with the given lease period range."]
						#[doc = ""]
						#[doc = "This applies a lock to your parachain configuration, ensuring that it cannot be changed"]
						#[doc = "by the parachain manager."]
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
						#[doc = "Contribute to a crowd sale. This will transfer some balance over to fund a parachain"]
						#[doc = "slot. It will be withdrawable when the crowdloan has ended and the funds are unused."]
						contribute {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
							#[codec(compact)]
							value: ::core::primitive::u128,
							signature:
								::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
						},
						#[codec(index = 2)]
						#[doc = "Withdraw full balance of a specific contributor."]
						#[doc = ""]
						#[doc = "Origin must be signed, but can come from anyone."]
						#[doc = ""]
						#[doc = "The fund must be either in, or ready for, retirement. For a fund to be *in* retirement, then the retirement"]
						#[doc = "flag must be set. For a fund to be ready for retirement, then:"]
						#[doc = "- it must not already be in retirement;"]
						#[doc = "- the amount of raised funds must be bigger than the _free_ balance of the account;"]
						#[doc = "- and either:"]
						#[doc = "  - the block number must be at least `end`; or"]
						#[doc = "  - the current lease period must be greater than the fund's `last_period`."]
						#[doc = ""]
						#[doc = "In this case, the fund's retirement flag is set and its `end` is reset to the current block"]
						#[doc = "number."]
						#[doc = ""]
						#[doc = "- `who`: The account whose contribution should be withdrawn."]
						#[doc = "- `index`: The parachain to whose crowdloan the contribution was made."]
						withdraw {
							who: ::subxt::ext::sp_core::crypto::AccountId32,
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 3)]
						#[doc = "Automatically refund contributors of an ended crowdloan."]
						#[doc = "Due to weight restrictions, this function may need to be called multiple"]
						#[doc = "times to fully refund all users. We will refund `RemoveKeysLimit` users at a time."]
						#[doc = ""]
						#[doc = "Origin must be signed, but can come from anyone."]
						refund {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 4)]
						#[doc = "Remove a fund after the retirement period has ended and all funds have been returned."]
						dissolve {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 5)]
						#[doc = "Edit the configuration for an in-progress crowdloan."]
						#[doc = ""]
						#[doc = "Can only be called by Root origin."]
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
						#[doc = "Add an optional memo to an existing crowdloan contribution."]
						#[doc = ""]
						#[doc = "Origin must be Signed, and the user must have contributed to the crowdloan."]
						add_memo {
							index: runtime_types::polkadot_parachain::primitives::Id,
							memo: ::std::vec::Vec<::core::primitive::u8>,
						},
						#[codec(index = 7)]
						#[doc = "Poke the fund into `NewRaise`"]
						#[doc = ""]
						#[doc = "Origin must be Signed, and the fund has non-zero raise."]
						poke { index: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 8)]
						#[doc = "Contribute your entire balance to a crowd sale. This will transfer the entire balance of a user over to fund a parachain"]
						#[doc = "slot. It will be withdrawable when the crowdloan has ended and the funds are unused."]
						contribute_all {
							#[codec(compact)]
							index: runtime_types::polkadot_parachain::primitives::Id,
							signature:
								::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The current lease period is more than the first lease period."]
						FirstPeriodInPast,
						#[codec(index = 1)]
						#[doc = "The first lease period needs to at least be less than 3 `max_value`."]
						FirstPeriodTooFarInFuture,
						#[codec(index = 2)]
						#[doc = "Last lease period must be greater than first lease period."]
						LastPeriodBeforeFirstPeriod,
						#[codec(index = 3)]
						#[doc = "The last lease period cannot be more than 3 periods after the first period."]
						LastPeriodTooFarInFuture,
						#[codec(index = 4)]
						#[doc = "The campaign ends before the current block number. The end must be in the future."]
						CannotEndInPast,
						#[codec(index = 5)]
						#[doc = "The end date for this crowdloan is not sensible."]
						EndTooFarInFuture,
						#[codec(index = 6)]
						#[doc = "There was an overflow."]
						Overflow,
						#[codec(index = 7)]
						#[doc = "The contribution was below the minimum, `MinContribution`."]
						ContributionTooSmall,
						#[codec(index = 8)]
						#[doc = "Invalid fund index."]
						InvalidParaId,
						#[codec(index = 9)]
						#[doc = "Contributions exceed maximum amount."]
						CapExceeded,
						#[codec(index = 10)]
						#[doc = "The contribution period has already ended."]
						ContributionPeriodOver,
						#[codec(index = 11)]
						#[doc = "The origin of this call is invalid."]
						InvalidOrigin,
						#[codec(index = 12)]
						#[doc = "This crowdloan does not correspond to a parachain."]
						NotParachain,
						#[codec(index = 13)]
						#[doc = "This parachain lease is still active and retirement cannot yet begin."]
						LeaseActive,
						#[codec(index = 14)]
						#[doc = "This parachain's bid or lease is still active and withdraw cannot yet begin."]
						BidOrLeaseActive,
						#[codec(index = 15)]
						#[doc = "The crowdloan has not yet ended."]
						FundNotEnded,
						#[codec(index = 16)]
						#[doc = "There are no contributions stored in this crowdloan."]
						NoContributions,
						#[codec(index = 17)]
						#[doc = "The crowdloan is not ready to dissolve. Potentially still has a slot or in retirement period."]
						NotReadyToDissolve,
						#[codec(index = 18)]
						#[doc = "Invalid signature."]
						InvalidSignature,
						#[codec(index = 19)]
						#[doc = "The provided memo is too large."]
						MemoTooLarge,
						#[codec(index = 20)]
						#[doc = "The fund is already in `NewRaise`"]
						AlreadyInNewRaise,
						#[codec(index = 21)]
						#[doc = "No contributions allowed during the VRF delay"]
						VrfDelayInProgress,
						#[codec(index = 22)]
						#[doc = "A lease period has not started yet, due to an offset in the starting block."]
						NoLeasePeriod,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "Create a new crowdloaning campaign."]
						Created { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 1)]
						#[doc = "Contributed to a crowd sale."]
						Contributed {
							who: ::subxt::ext::sp_core::crypto::AccountId32,
							fund_index: runtime_types::polkadot_parachain::primitives::Id,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 2)]
						#[doc = "Withdrew full balance of a contributor."]
						Withdrew {
							who: ::subxt::ext::sp_core::crypto::AccountId32,
							fund_index: runtime_types::polkadot_parachain::primitives::Id,
							amount: ::core::primitive::u128,
						},
						#[codec(index = 3)]
						#[doc = "The loans in a fund have been partially dissolved, i.e. there are some left"]
						#[doc = "over child keys that still need to be killed."]
						PartiallyRefunded {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 4)]
						#[doc = "All loans in a fund have been refunded."]
						AllRefunded { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 5)]
						#[doc = "Fund is dissolved."]
						Dissolved { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 6)]
						#[doc = "The result of trying to submit a new bid to the Slots pallet."]
						HandleBidResult {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							result: ::core::result::Result<
								(),
								runtime_types::sp_runtime::DispatchError,
							>,
						},
						#[codec(index = 7)]
						#[doc = "The configuration to a crowdloan has been edited."]
						Edited { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 8)]
						#[doc = "A memo has been updated."]
						MemoUpdated {
							who: ::subxt::ext::sp_core::crypto::AccountId32,
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							memo: ::std::vec::Vec<::core::primitive::u8>,
						},
						#[codec(index = 9)]
						#[doc = "A parachain has been moved to `NewRaise`"]
						AddedToNewRaise {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					pub fund_index: _2,
					#[codec(skip)]
					pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Register head data and validation code for a reserved Para Id."]
						#[doc = ""]
						#[doc = "## Arguments"]
						#[doc = "- `origin`: Must be called by a `Signed` origin."]
						#[doc = "- `id`: The para ID. Must be owned/managed by the `origin` signing account."]
						#[doc = "- `genesis_head`: The genesis head data of the parachain/thread."]
						#[doc = "- `validation_code`: The initial validation code of the parachain/thread."]
						#[doc = ""]
						#[doc = "## Deposits/Fees"]
						#[doc = "The origin signed account must reserve a corresponding deposit for the registration. Anything already"]
						#[doc = "reserved previously for this para ID is accounted for."]
						#[doc = ""]
						#[doc = "## Events"]
						#[doc = "The `Registered` event is emitted in case of success."]
						register {
							id: runtime_types::polkadot_parachain::primitives::Id,
							genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
							validation_code:
								runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 1)]
						#[doc = "Force the registration of a Para Id on the relay chain."]
						#[doc = ""]
						#[doc = "This function must be called by a Root origin."]
						#[doc = ""]
						#[doc = "The deposit taken can be specified for this registration. Any `ParaId`"]
						#[doc = "can be registered, including sub-1000 IDs which are System Parachains."]
						force_register {
							who: ::subxt::ext::sp_core::crypto::AccountId32,
							deposit: ::core::primitive::u128,
							id: runtime_types::polkadot_parachain::primitives::Id,
							genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
							validation_code:
								runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 2)]
						#[doc = "Deregister a Para Id, freeing all data and returning any deposit."]
						#[doc = ""]
						#[doc = "The caller must be Root, the `para` owner, or the `para` itself. The para must be a parathread."]
						deregister { id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 3)]
						#[doc = "Swap a parachain with another parachain or parathread."]
						#[doc = ""]
						#[doc = "The origin must be Root, the `para` owner, or the `para` itself."]
						#[doc = ""]
						#[doc = "The swap will happen only if there is already an opposite swap pending. If there is not,"]
						#[doc = "the swap will be stored in the pending swaps map, ready for a later confirmatory swap."]
						#[doc = ""]
						#[doc = "The `ParaId`s remain mapped to the same head data and code so external code can rely on"]
						#[doc = "`ParaId` to be a long-term identifier of a notional \"parachain\". However, their"]
						#[doc = "scheduling info (i.e. whether they're a parathread or parachain), auction information"]
						#[doc = "and the auction deposit are switched."]
						swap {
							id: runtime_types::polkadot_parachain::primitives::Id,
							other: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 4)]
						#[doc = "Remove a manager lock from a para. This will allow the manager of a"]
						#[doc = "previously locked para to deregister or swap a para without using governance."]
						#[doc = ""]
						#[doc = "Can only be called by the Root origin."]
						force_remove_lock {
							para: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 5)]
						#[doc = "Reserve a Para Id on the relay chain."]
						#[doc = ""]
						#[doc = "This function will reserve a new Para Id to be owned/managed by the origin account."]
						#[doc = "The origin account is able to register head data and validation code using `register` to create"]
						#[doc = "a parathread. Using the Slots pallet, a parathread can then be upgraded to get a parachain slot."]
						#[doc = ""]
						#[doc = "## Arguments"]
						#[doc = "- `origin`: Must be called by a `Signed` origin. Becomes the manager/owner of the new para ID."]
						#[doc = ""]
						#[doc = "## Deposits/Fees"]
						#[doc = "The origin must reserve a deposit of `ParaDeposit` for the registration."]
						#[doc = ""]
						#[doc = "## Events"]
						#[doc = "The `Reserved` event is emitted in case of success, which provides the ID reserved for use."]
						reserve,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The ID is not registered."]
						NotRegistered,
						#[codec(index = 1)]
						#[doc = "The ID is already registered."]
						AlreadyRegistered,
						#[codec(index = 2)]
						#[doc = "The caller is not the owner of this Id."]
						NotOwner,
						#[codec(index = 3)]
						#[doc = "Invalid para code size."]
						CodeTooLarge,
						#[codec(index = 4)]
						#[doc = "Invalid para head data size."]
						HeadDataTooLarge,
						#[codec(index = 5)]
						#[doc = "Para is not a Parachain."]
						NotParachain,
						#[codec(index = 6)]
						#[doc = "Para is not a Parathread."]
						NotParathread,
						#[codec(index = 7)]
						#[doc = "Cannot deregister para"]
						CannotDeregister,
						#[codec(index = 8)]
						#[doc = "Cannot schedule downgrade of parachain to parathread"]
						CannotDowngrade,
						#[codec(index = 9)]
						#[doc = "Cannot schedule upgrade of parathread to parachain"]
						CannotUpgrade,
						#[codec(index = 10)]
						#[doc = "Para is locked from manipulation by the manager. Must use parachain or relay chain governance."]
						ParaLocked,
						#[codec(index = 11)]
						#[doc = "The ID given for registration has not been reserved."]
						NotReserved,
						#[codec(index = 12)]
						#[doc = "Registering parachain with empty code is not allowed."]
						EmptyCode,
						#[codec(index = 13)]
						#[doc = "Cannot perform a parachain slot / lifecycle swap. Check that the state of both paras are"]
						#[doc = "correct for the swap to work."]
						CannotSwap,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						Registered {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							manager: ::subxt::ext::sp_core::crypto::AccountId32,
						},
						#[codec(index = 1)]
						Deregistered { para_id: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 2)]
						Reserved {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							who: ::subxt::ext::sp_core::crypto::AccountId32,
						},
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Schedule a para to be initialized at the start of the next session."]
						sudo_schedule_para_initialize {
							id: runtime_types::polkadot_parachain::primitives::Id,
							genesis:
								runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
						},
						#[codec(index = 1)]
						#[doc = "Schedule a para to be cleaned up at the start of the next session."]
						sudo_schedule_para_cleanup {
							id: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 2)]
						#[doc = "Upgrade a parathread to a parachain"]
						sudo_schedule_parathread_upgrade {
							id: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 3)]
						#[doc = "Downgrade a parachain to a parathread"]
						sudo_schedule_parachain_downgrade {
							id: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 4)]
						#[doc = "Send a downward XCM to the given para."]
						#[doc = ""]
						#[doc = "The given parachain should exist and the payload should not exceed the preconfigured size"]
						#[doc = "`config.max_downward_message_size`."]
						sudo_queue_downward_xcm {
							id: runtime_types::polkadot_parachain::primitives::Id,
							xcm: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
						},
						#[codec(index = 5)]
						#[doc = "Forcefully establish a channel from the sender to the recipient."]
						#[doc = ""]
						#[doc = "This is equivalent to sending an `Hrmp::hrmp_init_open_channel` extrinsic followed by"]
						#[doc = "`Hrmp::hrmp_accept_open_channel`."]
						sudo_establish_hrmp_channel {
							sender: runtime_types::polkadot_parachain::primitives::Id,
							recipient: runtime_types::polkadot_parachain::primitives::Id,
							max_capacity: ::core::primitive::u32,
							max_message_size: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The specified parachain or parathread is not registered."]
						ParaDoesntExist,
						#[codec(index = 1)]
						#[doc = "The specified parachain or parathread is already registered."]
						ParaAlreadyExists,
						#[codec(index = 2)]
						#[doc = "A DMP message couldn't be sent because it exceeds the maximum size allowed for a downward"]
						#[doc = "message."]
						ExceedsMaxMessageSize,
						#[codec(index = 3)]
						#[doc = "Could not schedule para cleanup."]
						CouldntCleanup,
						#[codec(index = 4)]
						#[doc = "Not a parathread."]
						NotParathread,
						#[codec(index = 5)]
						#[doc = "Not a parachain."]
						NotParachain,
						#[codec(index = 6)]
						#[doc = "Cannot upgrade parathread."]
						CannotUpgrade,
						#[codec(index = 7)]
						#[doc = "Cannot downgrade parachain."]
						CannotDowngrade,
					}
				}
			}
			pub mod slots {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Just a connect into the `lease_out` call, in case Root wants to force some lease to happen"]
						#[doc = "independently of any other on-chain mechanism to use it."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
						force_lease {
							para: runtime_types::polkadot_parachain::primitives::Id,
							leaser: ::subxt::ext::sp_core::crypto::AccountId32,
							amount: ::core::primitive::u128,
							period_begin: ::core::primitive::u32,
							period_count: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						#[doc = "Clear all leases for a Para Id, refunding any deposits back to the original owners."]
						#[doc = ""]
						#[doc = "The dispatch origin for this call must match `T::ForceOrigin`."]
						clear_all_leases { para: runtime_types::polkadot_parachain::primitives::Id },
						#[codec(index = 2)]
						#[doc = "Try to onboard a parachain that has a lease for the current lease period."]
						#[doc = ""]
						#[doc = "This function can be useful if there was some state issue with a para that should"]
						#[doc = "have onboarded, but was unable to. As long as they have a lease period, we can"]
						#[doc = "let them onboard from here."]
						#[doc = ""]
						#[doc = "Origin must be signed, but can be called by anyone."]
						trigger_onboard { para: runtime_types::polkadot_parachain::primitives::Id },
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The parachain ID is not onboarding."]
						ParaNotOnboarding,
						#[codec(index = 1)]
						#[doc = "There was an error with the lease."]
						LeaseError,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "A new `[lease_period]` is beginning."]
						NewLeasePeriod { lease_period: ::core::primitive::u32 },
						#[codec(index = 1)]
						#[doc = "A para has won the right to a continuous set of lease periods as a parachain."]
						#[doc = "First balance is any extra amount reserved on top of the para's existing deposit."]
						#[doc = "Second balance is the total amount reserved."]
						Leased {
							para_id: runtime_types::polkadot_parachain::primitives::Id,
							leaser: ::subxt::ext::sp_core::crypto::AccountId32,
							period_begin: ::core::primitive::u32,
							period_count: ::core::primitive::u32,
							extra_reserved: ::core::primitive::u128,
							total_amount: ::core::primitive::u128,
						},
					}
				}
			}
		}
		pub mod polkadot_runtime_parachains {
			use super::runtime_types;
			pub mod configuration {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Set the validation upgrade cooldown."]
						set_validation_upgrade_cooldown { new: ::core::primitive::u32 },
						#[codec(index = 1)]
						#[doc = "Set the validation upgrade delay."]
						set_validation_upgrade_delay { new: ::core::primitive::u32 },
						#[codec(index = 2)]
						#[doc = "Set the acceptance period for an included candidate."]
						set_code_retention_period { new: ::core::primitive::u32 },
						#[codec(index = 3)]
						#[doc = "Set the max validation code size for incoming upgrades."]
						set_max_code_size { new: ::core::primitive::u32 },
						#[codec(index = 4)]
						#[doc = "Set the max POV block size for incoming upgrades."]
						set_max_pov_size { new: ::core::primitive::u32 },
						#[codec(index = 5)]
						#[doc = "Set the max head data size for paras."]
						set_max_head_data_size { new: ::core::primitive::u32 },
						#[codec(index = 6)]
						#[doc = "Set the number of parathread execution cores."]
						set_parathread_cores { new: ::core::primitive::u32 },
						#[codec(index = 7)]
						#[doc = "Set the number of retries for a particular parathread."]
						set_parathread_retries { new: ::core::primitive::u32 },
						#[codec(index = 8)]
						#[doc = "Set the parachain validator-group rotation frequency"]
						set_group_rotation_frequency { new: ::core::primitive::u32 },
						#[codec(index = 9)]
						#[doc = "Set the availability period for parachains."]
						set_chain_availability_period { new: ::core::primitive::u32 },
						#[codec(index = 10)]
						#[doc = "Set the availability period for parathreads."]
						set_thread_availability_period { new: ::core::primitive::u32 },
						#[codec(index = 11)]
						#[doc = "Set the scheduling lookahead, in expected number of blocks at peak throughput."]
						set_scheduling_lookahead { new: ::core::primitive::u32 },
						#[codec(index = 12)]
						#[doc = "Set the maximum number of validators to assign to any core."]
						set_max_validators_per_core {
							new: ::core::option::Option<::core::primitive::u32>,
						},
						#[codec(index = 13)]
						#[doc = "Set the maximum number of validators to use in parachain consensus."]
						set_max_validators { new: ::core::option::Option<::core::primitive::u32> },
						#[codec(index = 14)]
						#[doc = "Set the dispute period, in number of sessions to keep for disputes."]
						set_dispute_period { new: ::core::primitive::u32 },
						#[codec(index = 15)]
						#[doc = "Set the dispute post conclusion acceptance period."]
						set_dispute_post_conclusion_acceptance_period {
							new: ::core::primitive::u32,
						},
						#[codec(index = 16)]
						#[doc = "Set the maximum number of dispute spam slots."]
						set_dispute_max_spam_slots { new: ::core::primitive::u32 },
						#[codec(index = 17)]
						#[doc = "Set the dispute conclusion by time out period."]
						set_dispute_conclusion_by_time_out_period { new: ::core::primitive::u32 },
						#[codec(index = 18)]
						#[doc = "Set the no show slots, in number of number of consensus slots."]
						#[doc = "Must be at least 1."]
						set_no_show_slots { new: ::core::primitive::u32 },
						#[codec(index = 19)]
						#[doc = "Set the total number of delay tranches."]
						set_n_delay_tranches { new: ::core::primitive::u32 },
						#[codec(index = 20)]
						#[doc = "Set the zeroth delay tranche width."]
						set_zeroth_delay_tranche_width { new: ::core::primitive::u32 },
						#[codec(index = 21)]
						#[doc = "Set the number of validators needed to approve a block."]
						set_needed_approvals { new: ::core::primitive::u32 },
						#[codec(index = 22)]
						#[doc = "Set the number of samples to do of the `RelayVRFModulo` approval assignment criterion."]
						set_relay_vrf_modulo_samples { new: ::core::primitive::u32 },
						#[codec(index = 23)]
						#[doc = "Sets the maximum items that can present in a upward dispatch queue at once."]
						set_max_upward_queue_count { new: ::core::primitive::u32 },
						#[codec(index = 24)]
						#[doc = "Sets the maximum total size of items that can present in a upward dispatch queue at once."]
						set_max_upward_queue_size { new: ::core::primitive::u32 },
						#[codec(index = 25)]
						#[doc = "Set the critical downward message size."]
						set_max_downward_message_size { new: ::core::primitive::u32 },
						#[codec(index = 26)]
						#[doc = "Sets the soft limit for the phase of dispatching dispatchable upward messages."]
						set_ump_service_total_weight { new: ::core::primitive::u64 },
						#[codec(index = 27)]
						#[doc = "Sets the maximum size of an upward message that can be sent by a candidate."]
						set_max_upward_message_size { new: ::core::primitive::u32 },
						#[codec(index = 28)]
						#[doc = "Sets the maximum number of messages that a candidate can contain."]
						set_max_upward_message_num_per_candidate { new: ::core::primitive::u32 },
						#[codec(index = 29)]
						#[doc = "Sets the number of sessions after which an HRMP open channel request expires."]
						set_hrmp_open_request_ttl { new: ::core::primitive::u32 },
						#[codec(index = 30)]
						#[doc = "Sets the amount of funds that the sender should provide for opening an HRMP channel."]
						set_hrmp_sender_deposit { new: ::core::primitive::u128 },
						#[codec(index = 31)]
						#[doc = "Sets the amount of funds that the recipient should provide for accepting opening an HRMP"]
						#[doc = "channel."]
						set_hrmp_recipient_deposit { new: ::core::primitive::u128 },
						#[codec(index = 32)]
						#[doc = "Sets the maximum number of messages allowed in an HRMP channel at once."]
						set_hrmp_channel_max_capacity { new: ::core::primitive::u32 },
						#[codec(index = 33)]
						#[doc = "Sets the maximum total size of messages in bytes allowed in an HRMP channel at once."]
						set_hrmp_channel_max_total_size { new: ::core::primitive::u32 },
						#[codec(index = 34)]
						#[doc = "Sets the maximum number of inbound HRMP channels a parachain is allowed to accept."]
						set_hrmp_max_parachain_inbound_channels { new: ::core::primitive::u32 },
						#[codec(index = 35)]
						#[doc = "Sets the maximum number of inbound HRMP channels a parathread is allowed to accept."]
						set_hrmp_max_parathread_inbound_channels { new: ::core::primitive::u32 },
						#[codec(index = 36)]
						#[doc = "Sets the maximum size of a message that could ever be put into an HRMP channel."]
						set_hrmp_channel_max_message_size { new: ::core::primitive::u32 },
						#[codec(index = 37)]
						#[doc = "Sets the maximum number of outbound HRMP channels a parachain is allowed to open."]
						set_hrmp_max_parachain_outbound_channels { new: ::core::primitive::u32 },
						#[codec(index = 38)]
						#[doc = "Sets the maximum number of outbound HRMP channels a parathread is allowed to open."]
						set_hrmp_max_parathread_outbound_channels { new: ::core::primitive::u32 },
						#[codec(index = 39)]
						#[doc = "Sets the maximum number of outbound HRMP messages can be sent by a candidate."]
						set_hrmp_max_message_num_per_candidate { new: ::core::primitive::u32 },
						#[codec(index = 40)]
						#[doc = "Sets the maximum amount of weight any individual upward message may consume."]
						set_ump_max_individual_weight { new: ::core::primitive::u64 },
						#[codec(index = 41)]
						#[doc = "Enable or disable PVF pre-checking. Consult the field documentation prior executing."]
						set_pvf_checking_enabled { new: ::core::primitive::bool },
						#[codec(index = 42)]
						#[doc = "Set the number of session changes after which a PVF pre-checking voting is rejected."]
						set_pvf_voting_ttl { new: ::core::primitive::u32 },
						#[codec(index = 43)]
						#[doc = "Sets the minimum delay between announcing the upgrade block for a parachain until the"]
						#[doc = "upgrade taking place."]
						#[doc = ""]
						#[doc = "See the field documentation for information and constraints for the new value."]
						set_minimum_validation_upgrade_delay { new: ::core::primitive::u32 },
						#[codec(index = 44)]
						#[doc = "Setting this to true will disable consistency checks for the configuration setters."]
						#[doc = "Use with caution."]
						set_bypass_consistency_check { new: ::core::primitive::bool },
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The new value for a configuration parameter is invalid."]
						InvalidNewValue,
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						force_unfreeze,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "Duplicate dispute statement sets provided."]
						DuplicateDisputeStatementSets,
						#[codec(index = 1)]
						#[doc = "Ancient dispute statement provided."]
						AncientDisputeStatement,
						#[codec(index = 2)]
						#[doc = "Validator index on statement is out of bounds for session."]
						ValidatorIndexOutOfBounds,
						#[codec(index = 3)]
						#[doc = "Invalid signature on statement."]
						InvalidSignature,
						#[codec(index = 4)]
						#[doc = "Validator vote submitted more than once to dispute."]
						DuplicateStatement,
						#[codec(index = 5)]
						#[doc = "Too many spam slots used by some specific validator."]
						PotentialSpam,
						#[codec(index = 6)]
						#[doc = "A dispute where there are only votes on one side."]
						SingleSidedDispute,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "A dispute has been initiated. \\[candidate hash, dispute location\\]"]
						DisputeInitiated(
							runtime_types::polkadot_core_primitives::CandidateHash,
							runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation,
						),
						#[codec(index = 1)]
						#[doc = "A dispute has concluded for or against a candidate."]
						#[doc = "`\\[para id, candidate hash, dispute result\\]`"]
						DisputeConcluded(
							runtime_types::polkadot_core_primitives::CandidateHash,
							runtime_types::polkadot_runtime_parachains::disputes::DisputeResult,
						),
						#[codec(index = 2)]
						#[doc = "A dispute has timed out due to insufficient participation."]
						#[doc = "`\\[para id, candidate hash\\]`"]
						DisputeTimedOut(runtime_types::polkadot_core_primitives::CandidateHash),
						#[codec(index = 3)]
						#[doc = "A dispute has concluded with supermajority against a candidate."]
						#[doc = "Block authors should no longer build on top of this head and should"]
						#[doc = "instead revert the block at the given height. This should be the"]
						#[doc = "number of the child of the last known valid block in the chain."]
						Revert(::core::primitive::u32),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum DisputeLocation {
					#[codec(index = 0)]
					Local,
					#[codec(index = 1)]
					Remote,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {}
				}
			}
			pub mod hrmp {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Initiate opening a channel from a parachain to a given recipient with given channel"]
						#[doc = "parameters."]
						#[doc = ""]
						#[doc = "- `proposed_max_capacity` - specifies how many messages can be in the channel at once."]
						#[doc = "- `proposed_max_message_size` - specifies the maximum size of the messages."]
						#[doc = ""]
						#[doc = "These numbers are a subject to the relay-chain configuration limits."]
						#[doc = ""]
						#[doc = "The channel can be opened only after the recipient confirms it and only on a session"]
						#[doc = "change."]
						hrmp_init_open_channel {
							recipient: runtime_types::polkadot_parachain::primitives::Id,
							proposed_max_capacity: ::core::primitive::u32,
							proposed_max_message_size: ::core::primitive::u32,
						},
						#[codec(index = 1)]
						#[doc = "Accept a pending open channel request from the given sender."]
						#[doc = ""]
						#[doc = "The channel will be opened only on the next session boundary."]
						hrmp_accept_open_channel {
							sender: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 2)]
						#[doc = "Initiate unilateral closing of a channel. The origin must be either the sender or the"]
						#[doc = "recipient in the channel being closed."]
						#[doc = ""]
						#[doc = "The closure can only happen on a session change."]
						hrmp_close_channel {
							channel_id:
								runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						},
						#[codec(index = 3)]
						#[doc = "This extrinsic triggers the cleanup of all the HRMP storage items that"]
						#[doc = "a para may have. Normally this happens once per session, but this allows"]
						#[doc = "you to trigger the cleanup immediately for a specific parachain."]
						#[doc = ""]
						#[doc = "Origin must be Root."]
						#[doc = ""]
						#[doc = "Number of inbound and outbound channels for `para` must be provided as witness data of weighing."]
						force_clean_hrmp {
							para: runtime_types::polkadot_parachain::primitives::Id,
							inbound: ::core::primitive::u32,
							outbound: ::core::primitive::u32,
						},
						#[codec(index = 4)]
						#[doc = "Force process HRMP open channel requests."]
						#[doc = ""]
						#[doc = "If there are pending HRMP open channel requests, you can use this"]
						#[doc = "function process all of those requests immediately."]
						#[doc = ""]
						#[doc = "Total number of opening channels must be provided as witness data of weighing."]
						force_process_hrmp_open { channels: ::core::primitive::u32 },
						#[codec(index = 5)]
						#[doc = "Force process HRMP close channel requests."]
						#[doc = ""]
						#[doc = "If there are pending HRMP close channel requests, you can use this"]
						#[doc = "function process all of those requests immediately."]
						#[doc = ""]
						#[doc = "Total number of closing channels must be provided as witness data of weighing."]
						force_process_hrmp_close { channels: ::core::primitive::u32 },
						#[codec(index = 6)]
						#[doc = "This cancels a pending open channel request. It can be canceled by either of the sender"]
						#[doc = "or the recipient for that request. The origin must be either of those."]
						#[doc = ""]
						#[doc = "The cancellation happens immediately. It is not possible to cancel the request if it is"]
						#[doc = "already accepted."]
						#[doc = ""]
						#[doc = "Total number of open requests (i.e. `HrmpOpenChannelRequestsList`) must be provided as"]
						#[doc = "witness data."]
						hrmp_cancel_open_request {
							channel_id:
								runtime_types::polkadot_parachain::primitives::HrmpChannelId,
							open_requests: ::core::primitive::u32,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The sender tried to open a channel to themselves."]
						OpenHrmpChannelToSelf,
						#[codec(index = 1)]
						#[doc = "The recipient is not a valid para."]
						OpenHrmpChannelInvalidRecipient,
						#[codec(index = 2)]
						#[doc = "The requested capacity is zero."]
						OpenHrmpChannelZeroCapacity,
						#[codec(index = 3)]
						#[doc = "The requested capacity exceeds the global limit."]
						OpenHrmpChannelCapacityExceedsLimit,
						#[codec(index = 4)]
						#[doc = "The requested maximum message size is 0."]
						OpenHrmpChannelZeroMessageSize,
						#[codec(index = 5)]
						#[doc = "The open request requested the message size that exceeds the global limit."]
						OpenHrmpChannelMessageSizeExceedsLimit,
						#[codec(index = 6)]
						#[doc = "The channel already exists"]
						OpenHrmpChannelAlreadyExists,
						#[codec(index = 7)]
						#[doc = "There is already a request to open the same channel."]
						OpenHrmpChannelAlreadyRequested,
						#[codec(index = 8)]
						#[doc = "The sender already has the maximum number of allowed outbound channels."]
						OpenHrmpChannelLimitExceeded,
						#[codec(index = 9)]
						#[doc = "The channel from the sender to the origin doesn't exist."]
						AcceptHrmpChannelDoesntExist,
						#[codec(index = 10)]
						#[doc = "The channel is already confirmed."]
						AcceptHrmpChannelAlreadyConfirmed,
						#[codec(index = 11)]
						#[doc = "The recipient already has the maximum number of allowed inbound channels."]
						AcceptHrmpChannelLimitExceeded,
						#[codec(index = 12)]
						#[doc = "The origin tries to close a channel where it is neither the sender nor the recipient."]
						CloseHrmpChannelUnauthorized,
						#[codec(index = 13)]
						#[doc = "The channel to be closed doesn't exist."]
						CloseHrmpChannelDoesntExist,
						#[codec(index = 14)]
						#[doc = "The channel close request is already requested."]
						CloseHrmpChannelAlreadyUnderway,
						#[codec(index = 15)]
						#[doc = "Canceling is requested by neither the sender nor recipient of the open channel request."]
						CancelHrmpOpenChannelUnauthorized,
						#[codec(index = 16)]
						#[doc = "The open request doesn't exist."]
						OpenHrmpChannelDoesntExist,
						#[codec(index = 17)]
						#[doc = "Cannot cancel an HRMP open channel request because it is already confirmed."]
						OpenHrmpChannelAlreadyConfirmed,
						#[codec(index = 18)]
						#[doc = "The provided witness data is wrong."]
						WrongWitness,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "Open HRMP channel requested."]
						#[doc = "`[sender, recipient, proposed_max_capacity, proposed_max_message_size]`"]
						OpenChannelRequested(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
							::core::primitive::u32,
						),
						#[codec(index = 1)]
						#[doc = "An HRMP channel request sent by the receiver was canceled by either party."]
						#[doc = "`[by_parachain, channel_id]`"]
						OpenChannelCanceled(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						),
						#[codec(index = 2)]
						#[doc = "Open HRMP channel accepted. `[sender, recipient]`"]
						OpenChannelAccepted(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::Id,
						),
						#[codec(index = 3)]
						#[doc = "HRMP channel closed. `[by_parachain, channel_id]`"]
						ChannelClosed(
							runtime_types::polkadot_parachain::primitives::Id,
							runtime_types::polkadot_parachain::primitives::HrmpChannelId,
						),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct HrmpChannel {
					pub max_capacity: ::core::primitive::u32,
					pub max_total_size: ::core::primitive::u32,
					pub max_message_size: ::core::primitive::u32,
					pub msg_count: ::core::primitive::u32,
					pub total_size: ::core::primitive::u32,
					pub mqc_head: ::core::option::Option<::subxt::ext::sp_core::H256>,
					pub sender_deposit: ::core::primitive::u128,
					pub recipient_deposit: ::core::primitive::u128,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "Validator indices are out of order or contains duplicates."]
						UnsortedOrDuplicateValidatorIndices,
						#[codec(index = 1)]
						#[doc = "Dispute statement sets are out of order or contain duplicates."]
						UnsortedOrDuplicateDisputeStatementSet,
						#[codec(index = 2)]
						#[doc = "Backed candidates are out of order (core index) or contain duplicates."]
						UnsortedOrDuplicateBackedCandidates,
						#[codec(index = 3)]
						#[doc = "A different relay parent was provided compared to the on-chain stored one."]
						UnexpectedRelayParent,
						#[codec(index = 4)]
						#[doc = "Availability bitfield has unexpected size."]
						WrongBitfieldSize,
						#[codec(index = 5)]
						#[doc = "Bitfield consists of zeros only."]
						BitfieldAllZeros,
						#[codec(index = 6)]
						#[doc = "Multiple bitfields submitted by same validator or validators out of order by index."]
						BitfieldDuplicateOrUnordered,
						#[codec(index = 7)]
						#[doc = "Validator index out of bounds."]
						ValidatorIndexOutOfBounds,
						#[codec(index = 8)]
						#[doc = "Invalid signature"]
						InvalidBitfieldSignature,
						#[codec(index = 9)]
						#[doc = "Candidate submitted but para not scheduled."]
						UnscheduledCandidate,
						#[codec(index = 10)]
						#[doc = "Candidate scheduled despite pending candidate already existing for the para."]
						CandidateScheduledBeforeParaFree,
						#[codec(index = 11)]
						#[doc = "Candidate included with the wrong collator."]
						WrongCollator,
						#[codec(index = 12)]
						#[doc = "Scheduled cores out of order."]
						ScheduledOutOfOrder,
						#[codec(index = 13)]
						#[doc = "Head data exceeds the configured maximum."]
						HeadDataTooLarge,
						#[codec(index = 14)]
						#[doc = "Code upgrade prematurely."]
						PrematureCodeUpgrade,
						#[codec(index = 15)]
						#[doc = "Output code is too large"]
						NewCodeTooLarge,
						#[codec(index = 16)]
						#[doc = "Candidate not in parent context."]
						CandidateNotInParentContext,
						#[codec(index = 17)]
						#[doc = "Invalid group index in core assignment."]
						InvalidGroupIndex,
						#[codec(index = 18)]
						#[doc = "Insufficient (non-majority) backing."]
						InsufficientBacking,
						#[codec(index = 19)]
						#[doc = "Invalid (bad signature, unknown validator, etc.) backing."]
						InvalidBacking,
						#[codec(index = 20)]
						#[doc = "Collator did not sign PoV."]
						NotCollatorSigned,
						#[codec(index = 21)]
						#[doc = "The validation data hash does not match expected."]
						ValidationDataHashMismatch,
						#[codec(index = 22)]
						#[doc = "The downward message queue is not processed correctly."]
						IncorrectDownwardMessageHandling,
						#[codec(index = 23)]
						#[doc = "At least one upward message sent does not pass the acceptance criteria."]
						InvalidUpwardMessages,
						#[codec(index = 24)]
						#[doc = "The candidate didn't follow the rules of HRMP watermark advancement."]
						HrmpWatermarkMishandling,
						#[codec(index = 25)]
						#[doc = "The HRMP messages sent by the candidate is not valid."]
						InvalidOutboundHrmp,
						#[codec(index = 26)]
						#[doc = "The validation code hash of the candidate is not valid."]
						InvalidValidationCodeHash,
						#[codec(index = 27)]
						#[doc = "The `para_head` hash in the candidate descriptor doesn't match the hash of the actual para head in the"]
						#[doc = "commitments."]
						ParaHeadMismatch,
						#[codec(index = 28)]
						#[doc = "A bitfield that references a freed core,"]
						#[doc = "either intentionally or as part of a concluded"]
						#[doc = "invalid dispute."]
						BitfieldReferencesFreedCore,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "A candidate was backed. `[candidate, head_data]`"]
						CandidateBacked(
							runtime_types::polkadot_primitives::v2::CandidateReceipt<
								::subxt::ext::sp_core::H256,
							>,
							runtime_types::polkadot_parachain::primitives::HeadData,
							runtime_types::polkadot_primitives::v2::CoreIndex,
							runtime_types::polkadot_primitives::v2::GroupIndex,
						),
						#[codec(index = 1)]
						#[doc = "A candidate was included. `[candidate, head_data]`"]
						CandidateIncluded(
							runtime_types::polkadot_primitives::v2::CandidateReceipt<
								::subxt::ext::sp_core::H256,
							>,
							runtime_types::polkadot_parachain::primitives::HeadData,
							runtime_types::polkadot_primitives::v2::CoreIndex,
							runtime_types::polkadot_primitives::v2::GroupIndex,
						),
						#[codec(index = 2)]
						#[doc = "A candidate timed out. `[candidate, head_data]`"]
						CandidateTimedOut(
							runtime_types::polkadot_primitives::v2::CandidateReceipt<
								::subxt::ext::sp_core::H256,
							>,
							runtime_types::polkadot_parachain::primitives::HeadData,
							runtime_types::polkadot_primitives::v2::CoreIndex,
						),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AvailabilityBitfieldRecord<_0> {
					pub bitfield: runtime_types::polkadot_primitives::v2::AvailabilityBitfield,
					pub submitted_at: _0,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CandidatePendingAvailability<_0, _1> {
					pub core: runtime_types::polkadot_primitives::v2::CoreIndex,
					pub hash: runtime_types::polkadot_core_primitives::CandidateHash,
					pub descriptor: runtime_types::polkadot_primitives::v2::CandidateDescriptor<_0>,
					pub availability_votes: ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
					pub backers: ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
					pub relay_parent_number: _1,
					pub backed_in_number: _1,
					pub backing_group: runtime_types::polkadot_primitives::v2::GroupIndex,
				}
			}
			pub mod initializer {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Issue a signal to the consensus engine to forcibly act as though all parachain"]
						#[doc = "blocks in all relay chain blocks up to and including the given number in the current"]
						#[doc = "chain are valid and should be finalized."]
						force_approve { up_to: ::core::primitive::u32 },
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BufferedSessionChange {
					pub validators: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::validator_app::Public,
					>,
					pub queued: ::std::vec::Vec<
						runtime_types::polkadot_primitives::v2::validator_app::Public,
					>,
					pub session_index: ::core::primitive::u32,
				}
			}
			pub mod origin {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Set the storage for the parachain validation code immediately."]
						force_set_current_code {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 1)]
						#[doc = "Set the storage for the current parachain head data immediately."]
						force_set_current_head {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_head: runtime_types::polkadot_parachain::primitives::HeadData,
						},
						#[codec(index = 2)]
						#[doc = "Schedule an upgrade as if it was scheduled in the given relay parent block."]
						force_schedule_code_upgrade {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
							relay_parent_number: ::core::primitive::u32,
						},
						#[codec(index = 3)]
						#[doc = "Note a new block head for para within the context of the current block."]
						force_note_new_head {
							para: runtime_types::polkadot_parachain::primitives::Id,
							new_head: runtime_types::polkadot_parachain::primitives::HeadData,
						},
						#[codec(index = 4)]
						#[doc = "Put a parachain directly into the next session's action queue."]
						#[doc = "We can't queue it any sooner than this without going into the"]
						#[doc = "initializer..."]
						force_queue_action {
							para: runtime_types::polkadot_parachain::primitives::Id,
						},
						#[codec(index = 5)]
						#[doc = "Adds the validation code to the storage."]
						#[doc = ""]
						#[doc = "The code will not be added if it is already present. Additionally, if PVF pre-checking"]
						#[doc = "is running for that code, it will be instantly accepted."]
						#[doc = ""]
						#[doc = "Otherwise, the code will be added into the storage. Note that the code will be added"]
						#[doc = "into storage with reference count 0. This is to account the fact that there are no users"]
						#[doc = "for this code yet. The caller will have to make sure that this code eventually gets"]
						#[doc = "used by some parachain or removed from the storage to avoid storage leaks. For the latter"]
						#[doc = "prefer to use the `poke_unused_validation_code` dispatchable to raw storage manipulation."]
						#[doc = ""]
						#[doc = "This function is mainly meant to be used for upgrading parachains that do not follow"]
						#[doc = "the go-ahead signal while the PVF pre-checking feature is enabled."]
						add_trusted_validation_code {
							validation_code:
								runtime_types::polkadot_parachain::primitives::ValidationCode,
						},
						#[codec(index = 6)]
						#[doc = "Remove the validation code from the storage iff the reference count is 0."]
						#[doc = ""]
						#[doc = "This is better than removing the storage directly, because it will not remove the code"]
						#[doc = "that was suddenly got used by some parachain while this dispatchable was pending"]
						#[doc = "dispatching."]
						poke_unused_validation_code {
							validation_code_hash:
								runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
						},
						#[codec(index = 7)]
						#[doc = "Includes a statement for a PVF pre-checking vote. Potentially, finalizes the vote and"]
						#[doc = "enacts the results if that was the last vote before achieving the supermajority."]
						include_pvf_check_statement {
							stmt: runtime_types::polkadot_primitives::v2::PvfCheckStatement,
							signature:
								runtime_types::polkadot_primitives::v2::validator_app::Signature,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "Para is not registered in our system."]
						NotRegistered,
						#[codec(index = 1)]
						#[doc = "Para cannot be onboarded because it is already tracked by our system."]
						CannotOnboard,
						#[codec(index = 2)]
						#[doc = "Para cannot be offboarded at this time."]
						CannotOffboard,
						#[codec(index = 3)]
						#[doc = "Para cannot be upgraded to a parachain."]
						CannotUpgrade,
						#[codec(index = 4)]
						#[doc = "Para cannot be downgraded to a parathread."]
						CannotDowngrade,
						#[codec(index = 5)]
						#[doc = "The statement for PVF pre-checking is stale."]
						PvfCheckStatementStale,
						#[codec(index = 6)]
						#[doc = "The statement for PVF pre-checking is for a future session."]
						PvfCheckStatementFuture,
						#[codec(index = 7)]
						#[doc = "Claimed validator index is out of bounds."]
						PvfCheckValidatorIndexOutOfBounds,
						#[codec(index = 8)]
						#[doc = "The signature for the PVF pre-checking is invalid."]
						PvfCheckInvalidSignature,
						#[codec(index = 9)]
						#[doc = "The given validator already has cast a vote."]
						PvfCheckDoubleVote,
						#[codec(index = 10)]
						#[doc = "The given PVF does not exist at the moment of process a vote."]
						PvfCheckSubjectInvalid,
						#[codec(index = 11)]
						#[doc = "The PVF pre-checking statement cannot be included since the PVF pre-checking mechanism"]
						#[doc = "is disabled."]
						PvfCheckDisabled,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "Current code has been updated for a Para. `para_id`"]
						CurrentCodeUpdated(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 1)]
						#[doc = "Current head has been updated for a Para. `para_id`"]
						CurrentHeadUpdated(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 2)]
						#[doc = "A code upgrade has been scheduled for a Para. `para_id`"]
						CodeUpgradeScheduled(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 3)]
						#[doc = "A new head has been noted for a Para. `para_id`"]
						NewHeadNoted(runtime_types::polkadot_parachain::primitives::Id),
						#[codec(index = 4)]
						#[doc = "A para has been queued to execute pending actions. `para_id`"]
						ActionQueued(
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
						),
						#[codec(index = 5)]
						#[doc = "The given para either initiated or subscribed to a PVF check for the given validation"]
						#[doc = "code. `code_hash` `para_id`"]
						PvfCheckStarted(
							runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
							runtime_types::polkadot_parachain::primitives::Id,
						),
						#[codec(index = 6)]
						#[doc = "The given validation code was accepted by the PVF pre-checking vote."]
						#[doc = "`code_hash` `para_id`"]
						PvfCheckAccepted(
							runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
							runtime_types::polkadot_parachain::primitives::Id,
						),
						#[codec(index = 7)]
						#[doc = "The given validation code was rejected by the PVF pre-checking vote."]
						#[doc = "`code_hash` `para_id`"]
						PvfCheckRejected(
							runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
							runtime_types::polkadot_parachain::primitives::Id,
						),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ParaGenesisArgs {
					pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
					pub validation_code:
						runtime_types::polkadot_parachain::primitives::ValidationCode,
					pub parachain: ::core::primitive::bool,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ParaPastCodeMeta<_0> {
					pub upgrade_times: ::std::vec::Vec<
						runtime_types::polkadot_runtime_parachains::paras::ReplacementTimes<_0>,
					>,
					pub last_pruned: ::core::option::Option<_0>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PvfCheckActiveVoteState<_0> {
					pub votes_accept: ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
					pub votes_reject: ::subxt::ext::bitvec::vec::BitVec<
						::core::primitive::u8,
						::subxt::ext::bitvec::order::Lsb0,
					>,
					pub age: _0,
					pub created_at: _0,
					pub causes: ::std::vec::Vec<
						runtime_types::polkadot_runtime_parachains::paras::PvfCheckCause<_0>,
					>,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum PvfCheckCause<_0> {
					#[codec(index = 0)]
					Onboarding(runtime_types::polkadot_parachain::primitives::Id),
					#[codec(index = 1)]
					Upgrade {
						id: runtime_types::polkadot_parachain::primitives::Id,
						relay_parent_number: _0,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ReplacementTimes<_0> {
					pub expected_at: _0,
					pub activated_at: _0,
				}
			}
			pub mod paras_inherent {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Enter the paras inherent. This will process bitfields and backed candidates."]
						enter {
							data: runtime_types::polkadot_primitives::v2::InherentData<
								runtime_types::sp_runtime::generic::header::Header<
									::core::primitive::u32,
									runtime_types::sp_runtime::traits::BlakeTwo256,
								>,
							>,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "Inclusion inherent called more than once per block."]
						TooManyInclusionInherents,
						#[codec(index = 1)]
						#[doc = "The hash of the submitted parent header doesn't correspond to the saved block hash of"]
						#[doc = "the parent."]
						InvalidParentHeader,
						#[codec(index = 2)]
						#[doc = "Disputed candidate that was concluded invalid."]
						CandidateConcludedInvalid,
						#[codec(index = 3)]
						#[doc = "The data given to the inherent will result in an overweight block."]
						InherentOverweight,
						#[codec(index = 4)]
						#[doc = "The ordering of dispute statements was invalid."]
						DisputeStatementsUnsortedOrDuplicates,
						#[codec(index = 5)]
						#[doc = "A dispute statement was invalid."]
						DisputeInvalid,
					}
				}
			}
			pub mod scheduler {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum AssignmentKind {
					#[codec(index = 0)]
					Parachain,
					#[codec(index = 1)]
					Parathread(
						runtime_types::polkadot_primitives::v2::collator_app::Public,
						::core::primitive::u32,
					),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct CoreAssignment {
					pub core: runtime_types::polkadot_primitives::v2::CoreIndex,
					pub para_id: runtime_types::polkadot_parachain::primitives::Id,
					pub kind: runtime_types::polkadot_runtime_parachains::scheduler::AssignmentKind,
					pub group_idx: runtime_types::polkadot_primitives::v2::GroupIndex,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct ParathreadClaimQueue {
					pub queue: ::std::vec::Vec<
						runtime_types::polkadot_runtime_parachains::scheduler::QueuedParathread,
					>,
					pub next_core_offset: ::core::primitive::u32,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct QueuedParathread {
					pub claim: runtime_types::polkadot_primitives::v2::ParathreadEntry,
					pub core_offset: ::core::primitive::u32,
				}
			}
			pub mod shared {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {}
				}
			}
			pub mod ump {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Service a single overweight upward message."]
						#[doc = ""]
						#[doc = "- `origin`: Must pass `ExecuteOverweightOrigin`."]
						#[doc = "- `index`: The index of the overweight message to service."]
						#[doc = "- `weight_limit`: The amount of weight that message execution may take."]
						#[doc = ""]
						#[doc = "Errors:"]
						#[doc = "- `UnknownMessageIndex`: Message of `index` is unknown."]
						#[doc = "- `WeightOverLimit`: Message execution may use greater than `weight_limit`."]
						#[doc = ""]
						#[doc = "Events:"]
						#[doc = "- `OverweightServiced`: On success."]
						service_overweight {
							index: ::core::primitive::u64,
							weight_limit: ::core::primitive::u64,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/v3/runtime/events-and-errors)\n\t\t\tof this pallet.\n\t\t\t"]
					pub enum Error {
						#[codec(index = 0)]
						#[doc = "The message index given is unknown."]
						UnknownMessageIndex,
						#[codec(index = 1)]
						#[doc = "The amount of weight given is possibly not enough for executing the message."]
						WeightOverLimit,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "Upward message is invalid XCM."]
						#[doc = "\\[ id \\]"]
						InvalidFormat([::core::primitive::u8; 32usize]),
						#[codec(index = 1)]
						#[doc = "Upward message is unsupported version of XCM."]
						#[doc = "\\[ id \\]"]
						UnsupportedVersion([::core::primitive::u8; 32usize]),
						#[codec(index = 2)]
						#[doc = "Upward message executed with the given outcome."]
						#[doc = "\\[ id, outcome \\]"]
						ExecutedUpward(
							[::core::primitive::u8; 32usize],
							runtime_types::xcm::v2::traits::Outcome,
						),
						#[codec(index = 3)]
						#[doc = "The weight limit for handling upward messages was reached."]
						#[doc = "\\[ id, remaining, required \\]"]
						WeightExhausted(
							[::core::primitive::u8; 32usize],
							::core::primitive::u64,
							::core::primitive::u64,
						),
						#[codec(index = 4)]
						#[doc = "Some upward messages have been received and will be processed."]
						#[doc = "\\[ para, count, size \\]"]
						UpwardMessagesReceived(
							runtime_types::polkadot_parachain::primitives::Id,
							::core::primitive::u32,
							::core::primitive::u32,
						),
						#[codec(index = 5)]
						#[doc = "The weight budget was exceeded for an individual upward message."]
						#[doc = ""]
						#[doc = "This message can be later dispatched manually using `service_overweight` dispatchable"]
						#[doc = "using the assigned `overweight_index`."]
						#[doc = ""]
						#[doc = "\\[ para, id, overweight_index, required \\]"]
						OverweightEnqueued(
							runtime_types::polkadot_parachain::primitives::Id,
							[::core::primitive::u8; 32usize],
							::core::primitive::u64,
							::core::primitive::u64,
						),
						#[codec(index = 6)]
						#[doc = "Upward message from the overweight queue was executed with the given actual weight"]
						#[doc = "used."]
						#[doc = ""]
						#[doc = "\\[ overweight_index, used \\]"]
						OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
					}
				}
			}
		}
		pub mod primitive_types {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct H256(pub [::core::primitive::u8; 32usize]);
		}
		pub mod rococo_runtime {
			use super::runtime_types;
			pub mod validator_manager {
				use super::runtime_types;
				pub mod pallet {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "Contains one variant per dispatchable that can be called by an extrinsic."]
					pub enum Call {
						#[codec(index = 0)]
						#[doc = "Add new validators to the set."]
						#[doc = ""]
						#[doc = "The new validators will be active from current session + 2."]
						register_validators {
							validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						},
						#[codec(index = 1)]
						#[doc = "Remove validators from the set."]
						#[doc = ""]
						#[doc = "The removed validators will be deactivated from current session + 2."]
						deregister_validators {
							validators: ::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					#[doc = "\n\t\t\tThe [event](https://docs.substrate.io/v3/runtime/events-and-errors) emitted\n\t\t\tby this pallet.\n\t\t\t"]
					pub enum Event {
						#[codec(index = 0)]
						#[doc = "New validators were added to the set."]
						ValidatorsRegistered(
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						),
						#[codec(index = 1)]
						#[doc = "Validators were removed from the set."]
						ValidatorsDeregistered(
							::std::vec::Vec<::subxt::ext::sp_core::crypto::AccountId32>,
						),
					}
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[codec(index = 36)]
				ValidatorManager(runtime_types::rococo_runtime::validator_manager::pallet::Call),
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Event {
				#[codec(index = 0)]
				System(runtime_types::frame_system::pallet::Event),
				#[codec(index = 3)]
				Indices(runtime_types::pallet_indices::pallet::Event),
				#[codec(index = 4)]
				Balances(runtime_types::pallet_balances::pallet::Event),
				#[codec(index = 5)]
				TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum OriginCaller {
				#[codec(index = 0)]
				system(
					runtime_types::frame_support::dispatch::RawOrigin<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 13)]
				ParachainsOrigin(
					runtime_types::polkadot_runtime_parachains::origin::pallet::Origin,
				),
				#[codec(index = 80)]
				Collective(
					runtime_types::pallet_collective::RawOrigin<
						::subxt::ext::sp_core::crypto::AccountId32,
					>,
				),
				#[codec(index = 99)]
				XcmPallet(runtime_types::pallet_xcm::pallet::Origin),
				#[codec(index = 4)]
				Void(runtime_types::sp_core::Void),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum ProxyType {
				#[codec(index = 0)]
				Any,
				#[codec(index = 1)]
				CancelProxy,
				#[codec(index = 2)]
				Auction,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct Runtime;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct SessionKeys {
				pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
				pub babe: runtime_types::sp_consensus_babe::app::Public,
				pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
				pub para_validator: runtime_types::polkadot_primitives::v2::validator_app::Public,
				pub para_assignment: runtime_types::polkadot_primitives::v2::assignment_app::Public,
				pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
				pub beefy: runtime_types::beefy_primitives::crypto::Public,
			}
		}
		pub mod sp_arithmetic {
			use super::runtime_types;
			pub mod fixed_point {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct FixedU128(pub ::core::primitive::u128);
			}
			pub mod per_things {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: CompactAs,
					:: subxt :: ext :: codec :: Decode,
					:: subxt :: ext :: codec :: Encode,
					Debug,
				)]
				pub struct Perbill(pub ::core::primitive::u32);
			}
		}
		pub mod sp_authority_discovery {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
		}
		pub mod sp_consensus_babe {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub runtime_types::sp_core::sr25519::Public);
			}
			pub mod digests {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum NextConfigDescriptor {
					#[codec(index = 1)]
					V1 {
						c: (::core::primitive::u64, ::core::primitive::u64),
						allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
					},
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum PreDigest {
					#[codec(index = 1)]
					Primary(runtime_types::sp_consensus_babe::digests::PrimaryPreDigest),
					#[codec(index = 2)]
					SecondaryPlain(
						runtime_types::sp_consensus_babe::digests::SecondaryPlainPreDigest,
					),
					#[codec(index = 3)]
					SecondaryVRF(runtime_types::sp_consensus_babe::digests::SecondaryVRFPreDigest),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct PrimaryPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_output: [::core::primitive::u8; 32usize],
					pub vrf_proof: [::core::primitive::u8; 64usize],
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct SecondaryPlainPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct SecondaryVRFPreDigest {
					pub authority_index: ::core::primitive::u32,
					pub slot: runtime_types::sp_consensus_slots::Slot,
					pub vrf_output: [::core::primitive::u8; 32usize],
					pub vrf_proof: [::core::primitive::u8; 64usize],
				}
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum AllowedSlots {
				#[codec(index = 0)]
				PrimarySlots,
				#[codec(index = 1)]
				PrimaryAndSecondaryPlainSlots,
				#[codec(index = 2)]
				PrimaryAndSecondaryVRFSlots,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct BabeEpochConfiguration {
				pub c: (::core::primitive::u64, ::core::primitive::u64),
				pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
			}
		}
		pub mod sp_consensus_slots {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct EquivocationProof<_0, _1> {
				pub offender: _1,
				pub slot: runtime_types::sp_consensus_slots::Slot,
				pub first_header: _0,
				pub second_header: _0,
			}
			#[derive(
				:: subxt :: ext :: codec :: CompactAs,
				:: subxt :: ext :: codec :: Decode,
				:: subxt :: ext :: codec :: Encode,
				Debug,
			)]
			pub struct Slot(pub ::core::primitive::u64);
		}
		pub mod sp_core {
			use super::runtime_types;
			pub mod crypto {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
			}
			pub mod ecdsa {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub [::core::primitive::u8; 33usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 65usize]);
			}
			pub mod ed25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			pub mod offchain {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct OpaqueNetworkState {
					pub peer_id: runtime_types::sp_core::OpaquePeerId,
					pub external_addresses:
						::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
				}
			}
			pub mod sr25519 {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub [::core::primitive::u8; 32usize]);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub [::core::primitive::u8; 64usize]);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum Void {}
		}
		pub mod sp_finality_grandpa {
			use super::runtime_types;
			pub mod app {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Public(pub runtime_types::sp_core::ed25519::Public);
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct EquivocationProof<_0, _1> {
				pub set_id: ::core::primitive::u64,
				pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
			}
		}
		pub mod sp_runtime {
			use super::runtime_types;
			pub mod bounded {
				use super::runtime_types;
				pub mod bounded_vec {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
				pub mod weak_bounded_vec {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
				}
			}
			pub mod generic {
				use super::runtime_types;
				pub mod digest {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Digest {
						pub logs:
							::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct Header<_0, _1> {
						pub parent_hash: ::subxt::ext::sp_core::H256,
						#[codec(compact)]
						pub number: _0,
						pub state_root: ::subxt::ext::sp_core::H256,
						pub extrinsics_root: ::subxt::ext::sp_core::H256,
						pub digest: runtime_types::sp_runtime::generic::digest::Digest,
						#[codec(skip)]
						pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
					}
				}
				pub mod unchecked_extrinsic {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
						pub ::std::vec::Vec<::core::primitive::u8>,
						#[codec(skip)] pub ::core::marker::PhantomData<(_0, _2, _1, _3)>,
					);
				}
			}
			pub mod multiaddress {
				use super::runtime_types;
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct BlakeTwo256;
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum ArithmeticError {
				#[codec(index = 0)]
				Underflow,
				#[codec(index = 1)]
				Overflow,
				#[codec(index = 2)]
				DivisionByZero,
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum DispatchError {
				#[codec(index = 0)]
				Other,
				#[codec(index = 1)]
				CannotLookup,
				#[codec(index = 2)]
				BadOrigin,
				#[codec(index = 3)]
				Module(runtime_types::sp_runtime::ModuleError),
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
				#[codec(index = 9)]
				Transactional(runtime_types::sp_runtime::TransactionalError),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub struct ModuleError {
				pub index: ::core::primitive::u8,
				pub error: [::core::primitive::u8; 4usize],
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum MultiSignature {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Signature),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Signature),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Signature),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum MultiSigner {
				#[codec(index = 0)]
				Ed25519(runtime_types::sp_core::ed25519::Public),
				#[codec(index = 1)]
				Sr25519(runtime_types::sp_core::sr25519::Public),
				#[codec(index = 2)]
				Ecdsa(runtime_types::sp_core::ecdsa::Public),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum TransactionalError {
				#[codec(index = 0)]
				LimitReached,
				#[codec(index = 1)]
				NoLayer,
			}
		}
		pub mod sp_session {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct OffenceDetails<_0, _1> {
					pub offender: _1,
					pub reporters: ::std::vec::Vec<_0>,
				}
			}
		}
		pub mod sp_version {
			use super::runtime_types;
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct DoubleEncoded {
					pub encoded: ::std::vec::Vec<::core::primitive::u8>,
				}
			}
			pub mod v0 {
				use super::runtime_types;
				pub mod junction {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum BodyId {
						#[codec(index = 0)]
						Unit,
						#[codec(index = 1)]
						Named(
							runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
						GeneralKey(
							runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 8)]
						OnlyChild,
						#[codec(index = 9)]
						Plurality {
							id: runtime_types::xcm::v0::junction::BodyId,
							part: runtime_types::xcm::v0::junction::BodyPart,
						},
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum NetworkId {
						#[codec(index = 0)]
						Any,
						#[codec(index = 1)]
						Named(
							runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
						#[codec(index = 2)]
						Polkadot,
						#[codec(index = 3)]
						Kusama,
					}
				}
				pub mod multi_asset {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
						GeneralKey(
							runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
								::core::primitive::u8,
							>,
						),
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum AssetId {
						#[codec(index = 0)]
						Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
						#[codec(index = 1)]
						Abstract(::std::vec::Vec<::core::primitive::u8>),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Fungibility {
						#[codec(index = 0)]
						Fungible(#[codec(compact)] ::core::primitive::u128),
						#[codec(index = 1)]
						NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiAsset {
						pub id: runtime_types::xcm::v1::multiasset::AssetId,
						pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum MultiAssetFilter {
						#[codec(index = 0)]
						Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
						#[codec(index = 1)]
						Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiAssets(
						pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
					);
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum WildFungibility {
						#[codec(index = 0)]
						Fungible,
						#[codec(index = 1)]
						NonFungible,
					}
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub struct MultiLocation {
						pub parents: ::core::primitive::u8,
						pub interior: runtime_types::xcm::v1::multilocation::Junctions,
					}
				}
				pub mod order {
					use super::runtime_types;
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum Response {
					#[codec(index = 0)]
					Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
					#[codec(index = 1)]
					Version(::core::primitive::u32),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
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
					#[derive(
						:: subxt :: ext :: codec :: Decode,
						:: subxt :: ext :: codec :: Encode,
						Debug,
					)]
					pub enum Outcome {
						#[codec(index = 0)]
						Complete(::core::primitive::u64),
						#[codec(index = 1)]
						Incomplete(::core::primitive::u64, runtime_types::xcm::v2::traits::Error),
						#[codec(index = 2)]
						Error(runtime_types::xcm::v2::traits::Error),
					}
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
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
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub enum WeightLimit {
					#[codec(index = 0)]
					Unlimited,
					#[codec(index = 1)]
					Limited(#[codec(compact)] ::core::primitive::u64),
				}
				#[derive(
					:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
				)]
				pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedMultiAssets {
				#[codec(index = 0)]
				V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multiasset::MultiAssets),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedMultiLocation {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::multi_location::MultiLocation),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::multilocation::MultiLocation),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
			pub enum VersionedResponse {
				#[codec(index = 0)]
				V0(runtime_types::xcm::v0::Response),
				#[codec(index = 1)]
				V1(runtime_types::xcm::v1::Response),
				#[codec(index = 2)]
				V2(runtime_types::xcm::v2::Response),
			}
			#[derive(
				:: subxt :: ext :: codec :: Decode, :: subxt :: ext :: codec :: Encode, Debug,
			)]
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
	#[doc = r" The default error type returned when there is a runtime issue,"]
	#[doc = r" exposed here for ease of use."]
	pub type DispatchError = runtime_types::sp_runtime::DispatchError;
	pub fn constants() -> ConstantsApi {
		ConstantsApi
	}
	pub fn storage() -> StorageApi {
		StorageApi
	}
	pub fn tx() -> TransactionApi {
		TransactionApi
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
	pub struct StorageApi;
	impl StorageApi {
		pub fn system(&self) -> system::storage::StorageApi {
			system::storage::StorageApi
		}
		pub fn babe(&self) -> babe::storage::StorageApi {
			babe::storage::StorageApi
		}
		pub fn timestamp(&self) -> timestamp::storage::StorageApi {
			timestamp::storage::StorageApi
		}
		pub fn indices(&self) -> indices::storage::StorageApi {
			indices::storage::StorageApi
		}
		pub fn balances(&self) -> balances::storage::StorageApi {
			balances::storage::StorageApi
		}
		pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
			transaction_payment::storage::StorageApi
		}
		pub fn authorship(&self) -> authorship::storage::StorageApi {
			authorship::storage::StorageApi
		}
		pub fn offences(&self) -> offences::storage::StorageApi {
			offences::storage::StorageApi
		}
		pub fn historical(&self) -> historical::storage::StorageApi {
			historical::storage::StorageApi
		}
		pub fn session(&self) -> session::storage::StorageApi {
			session::storage::StorageApi
		}
		pub fn grandpa(&self) -> grandpa::storage::StorageApi {
			grandpa::storage::StorageApi
		}
		pub fn im_online(&self) -> im_online::storage::StorageApi {
			im_online::storage::StorageApi
		}
		pub fn authority_discovery(&self) -> authority_discovery::storage::StorageApi {
			authority_discovery::storage::StorageApi
		}
		pub fn configuration(&self) -> configuration::storage::StorageApi {
			configuration::storage::StorageApi
		}
		pub fn paras_shared(&self) -> paras_shared::storage::StorageApi {
			paras_shared::storage::StorageApi
		}
		pub fn para_inclusion(&self) -> para_inclusion::storage::StorageApi {
			para_inclusion::storage::StorageApi
		}
		pub fn para_inherent(&self) -> para_inherent::storage::StorageApi {
			para_inherent::storage::StorageApi
		}
		pub fn para_scheduler(&self) -> para_scheduler::storage::StorageApi {
			para_scheduler::storage::StorageApi
		}
		pub fn paras(&self) -> paras::storage::StorageApi {
			paras::storage::StorageApi
		}
		pub fn initializer(&self) -> initializer::storage::StorageApi {
			initializer::storage::StorageApi
		}
		pub fn dmp(&self) -> dmp::storage::StorageApi {
			dmp::storage::StorageApi
		}
		pub fn ump(&self) -> ump::storage::StorageApi {
			ump::storage::StorageApi
		}
		pub fn hrmp(&self) -> hrmp::storage::StorageApi {
			hrmp::storage::StorageApi
		}
		pub fn para_session_info(&self) -> para_session_info::storage::StorageApi {
			para_session_info::storage::StorageApi
		}
		pub fn paras_disputes(&self) -> paras_disputes::storage::StorageApi {
			paras_disputes::storage::StorageApi
		}
		pub fn registrar(&self) -> registrar::storage::StorageApi {
			registrar::storage::StorageApi
		}
		pub fn auctions(&self) -> auctions::storage::StorageApi {
			auctions::storage::StorageApi
		}
		pub fn crowdloan(&self) -> crowdloan::storage::StorageApi {
			crowdloan::storage::StorageApi
		}
		pub fn slots(&self) -> slots::storage::StorageApi {
			slots::storage::StorageApi
		}
		pub fn assigned_slots(&self) -> assigned_slots::storage::StorageApi {
			assigned_slots::storage::StorageApi
		}
		pub fn sudo(&self) -> sudo::storage::StorageApi {
			sudo::storage::StorageApi
		}
		pub fn mmr(&self) -> mmr::storage::StorageApi {
			mmr::storage::StorageApi
		}
		pub fn beefy(&self) -> beefy::storage::StorageApi {
			beefy::storage::StorageApi
		}
		pub fn mmr_leaf(&self) -> mmr_leaf::storage::StorageApi {
			mmr_leaf::storage::StorageApi
		}
		pub fn validator_manager(&self) -> validator_manager::storage::StorageApi {
			validator_manager::storage::StorageApi
		}
		pub fn collective(&self) -> collective::storage::StorageApi {
			collective::storage::StorageApi
		}
		pub fn membership(&self) -> membership::storage::StorageApi {
			membership::storage::StorageApi
		}
		pub fn proxy(&self) -> proxy::storage::StorageApi {
			proxy::storage::StorageApi
		}
		pub fn multisig(&self) -> multisig::storage::StorageApi {
			multisig::storage::StorageApi
		}
		pub fn xcm_pallet(&self) -> xcm_pallet::storage::StorageApi {
			xcm_pallet::storage::StorageApi
		}
	}
	pub struct TransactionApi;
	impl TransactionApi {
		pub fn system(&self) -> system::calls::TransactionApi {
			system::calls::TransactionApi
		}
		pub fn babe(&self) -> babe::calls::TransactionApi {
			babe::calls::TransactionApi
		}
		pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
			timestamp::calls::TransactionApi
		}
		pub fn indices(&self) -> indices::calls::TransactionApi {
			indices::calls::TransactionApi
		}
		pub fn balances(&self) -> balances::calls::TransactionApi {
			balances::calls::TransactionApi
		}
		pub fn authorship(&self) -> authorship::calls::TransactionApi {
			authorship::calls::TransactionApi
		}
		pub fn session(&self) -> session::calls::TransactionApi {
			session::calls::TransactionApi
		}
		pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
			grandpa::calls::TransactionApi
		}
		pub fn im_online(&self) -> im_online::calls::TransactionApi {
			im_online::calls::TransactionApi
		}
		pub fn configuration(&self) -> configuration::calls::TransactionApi {
			configuration::calls::TransactionApi
		}
		pub fn paras_shared(&self) -> paras_shared::calls::TransactionApi {
			paras_shared::calls::TransactionApi
		}
		pub fn para_inclusion(&self) -> para_inclusion::calls::TransactionApi {
			para_inclusion::calls::TransactionApi
		}
		pub fn para_inherent(&self) -> para_inherent::calls::TransactionApi {
			para_inherent::calls::TransactionApi
		}
		pub fn paras(&self) -> paras::calls::TransactionApi {
			paras::calls::TransactionApi
		}
		pub fn initializer(&self) -> initializer::calls::TransactionApi {
			initializer::calls::TransactionApi
		}
		pub fn dmp(&self) -> dmp::calls::TransactionApi {
			dmp::calls::TransactionApi
		}
		pub fn ump(&self) -> ump::calls::TransactionApi {
			ump::calls::TransactionApi
		}
		pub fn hrmp(&self) -> hrmp::calls::TransactionApi {
			hrmp::calls::TransactionApi
		}
		pub fn paras_disputes(&self) -> paras_disputes::calls::TransactionApi {
			paras_disputes::calls::TransactionApi
		}
		pub fn registrar(&self) -> registrar::calls::TransactionApi {
			registrar::calls::TransactionApi
		}
		pub fn auctions(&self) -> auctions::calls::TransactionApi {
			auctions::calls::TransactionApi
		}
		pub fn crowdloan(&self) -> crowdloan::calls::TransactionApi {
			crowdloan::calls::TransactionApi
		}
		pub fn slots(&self) -> slots::calls::TransactionApi {
			slots::calls::TransactionApi
		}
		pub fn paras_sudo_wrapper(&self) -> paras_sudo_wrapper::calls::TransactionApi {
			paras_sudo_wrapper::calls::TransactionApi
		}
		pub fn assigned_slots(&self) -> assigned_slots::calls::TransactionApi {
			assigned_slots::calls::TransactionApi
		}
		pub fn sudo(&self) -> sudo::calls::TransactionApi {
			sudo::calls::TransactionApi
		}
		pub fn validator_manager(&self) -> validator_manager::calls::TransactionApi {
			validator_manager::calls::TransactionApi
		}
		pub fn collective(&self) -> collective::calls::TransactionApi {
			collective::calls::TransactionApi
		}
		pub fn membership(&self) -> membership::calls::TransactionApi {
			membership::calls::TransactionApi
		}
		pub fn utility(&self) -> utility::calls::TransactionApi {
			utility::calls::TransactionApi
		}
		pub fn proxy(&self) -> proxy::calls::TransactionApi {
			proxy::calls::TransactionApi
		}
		pub fn multisig(&self) -> multisig::calls::TransactionApi {
			multisig::calls::TransactionApi
		}
		pub fn xcm_pallet(&self) -> xcm_pallet::calls::TransactionApi {
			xcm_pallet::calls::TransactionApi
		}
	}
	#[doc = r" check whether the Client you are using is aligned with the statically generated codegen."]
	pub fn validate_codegen<T: ::subxt::Config, C: ::subxt::client::OfflineClientT<T>>(
		client: &C,
	) -> Result<(), ::subxt::error::MetadataError> {
		let runtime_metadata_hash = client.metadata().metadata_hash(&PALLETS);
		if runtime_metadata_hash !=
			[
				228u8, 153u8, 116u8, 143u8, 213u8, 196u8, 203u8, 41u8, 253u8, 88u8, 167u8, 190u8,
				195u8, 132u8, 69u8, 25u8, 92u8, 235u8, 171u8, 252u8, 210u8, 133u8, 204u8, 162u8,
				251u8, 244u8, 170u8, 144u8, 184u8, 156u8, 207u8, 26u8,
			] {
			Err(::subxt::error::MetadataError::IncompatibleMetadata)
		} else {
			Ok(())
		}
	}
}
