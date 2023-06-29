// pub use pallet::{Error, *};

// type AccoindIdOf<T> = <T as frame_system::Config>::AccountId;
// use frame_support::BoundedVec;

// #[derive(
// 	Copy,
// 	Clone,
// 	PartialEq,
// 	Eq,
// 	Hash,
// 	codec::Encode,
// 	codec::Decode,
// 	scale_info::TypeInfo,
// 	Ord,
// 	PartialOrd,
// )]
// pub struct ChainInfo {
// 	pub chain_id: u128,
// 	pub channel_id: u64,        //for packet or memo
// 	pub timestamp: Option<u64>, //for packet
// 	pub height: Option<u64>,    //for memo packet message forwarding
// 	pub retries: Option<u64>,   //for memo packet message forwarding
// 	pub timeout: Option<u64>,   //for memo packet message forwarding
// }

// #[derive(Serialize, Debug)]
// struct MemoForward {
// 	receiver: String,
// 	port: String,
// 	channel: String,
// 	timeout: String,
// 	retries: u64,
// 	next: Option<Box<MemoForward>>,
// }

// #[derive(Serialize, Debug)]
// struct MemoData {
// 	forward: MemoForward,
// }

// impl MemoData {
// 	/// Support only addresses from cosmos ecosystem based on bech32.
// 	fn new<T: Config>(
// 		mut vec: Vec<(ChainInfo, BoundedVec<u8, T::ChainNameVecLimit>, [u8; 32])>,
// 	) -> core::result::Result<Option<Self>, Error<T>> {
// 		vec.reverse();
// 		let mut memo_data: Option<MemoData> = None;
// 		for (i, name, address) in vec {
// 			let result: core::result::Result<Vec<bech32::u5>, bech32::Error> =
// 				address.into_iter().map(bech32::u5::try_from_u8).collect();
// 			let data =
// 				result.map_err(|_| Error::<T>::IncorrectAddress { chain_id: i.chain_id as u8 })?;

// 			let name = String::from_utf8(name.into())
// 				.map_err(|_| Error::<T>::IncorrectChainName { chain_id: i.chain_id as u8 })?;
// 			let result_address = bech32::encode(&name, data.clone()).map_err(|_| {
// 				Error::<T>::FailedToEncodeBech32Address { chain_id: i.chain_id as u8 }
// 			})?;

// 			let new_memo = MemoData {
// 				forward: MemoForward {
// 					receiver: result_address,
// 					port: String::from("transfer"),
// 					channel: String::from(format!("channel-{}", i.channel_id)),
// 					timeout: String::from(i.timeout.unwrap_or_default().to_string()),
// 					retries: i.retries.unwrap_or_default(),
// 					next: memo_data.map(|x| Box::new(x.forward)), // memo_data is boxed here
// 				},
// 			};
// 			memo_data = Some(new_memo);
// 		}
// 		Ok(memo_data)
// 	}
// }

// #[frame_support::pallet]
// pub mod pallet {
// 	use super::*;
// 	use frame_support::{pallet_prelude::*, BoundedBTreeSet};
// 	use ibc_primitives::Timeout as IbcTimeout;
// 	use pallet_ibc::{MultiAddress, TransferParams};
// 	use std::str::FromStr;

// 	/// ## Configuration
// 	/// The pallet's configuration trait.
// 	#[pallet::config]
// 	pub trait Config: frame_system::Config + pallet_ibc::Config {
// 		#[allow(missing_docs)]
// 		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

// 		#[pallet::constant]
// 		type PalletInstanceId: Get<u8>;

// 		#[pallet::constant]
// 		type MaxMultihopCount: Get<u32>;

// 		#[pallet::constant]
// 		type ChainNameVecLimit: Get<u32>;
// 	}

// 	// The pallet's events
// 	#[pallet::event]
// 	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
// 	pub enum Event<T: Config> {
// 		SuccessXcmToIbc {
// 			origin_address: T::AccountId,
// 			to: [u8; 32],
// 			amount: u128,
// 			asset_id: T::AssetId,
// 			memo: Option<T::MemoMessage>,
// 		},
// 		FailedXcmToIbc {
// 			origin_address: T::AccountId,
// 			to: [u8; 32],
// 			amount: u128,
// 			asset_id: T::AssetId,
// 			memo: Option<T::MemoMessage>,
// 		},
// 	}

// 	#[pallet::error]
// 	pub enum Error<T> {
// 		IncorrectAddress { chain_id: u8 },
// 		IncorrectChainName { chain_id: u8 },
// 		FailedToEncodeBech32Address { chain_id: u8 },
// 		IncorrectMultiLocation,
// 		XcmDepositFailed,
// 		MultiHopRouteDoesNotExist,
// 		DoesNotSupportNonFungible,
// 		IncorrectCountOfAddresses,
// 		FailedToConstructMemo,
// 	}

// 	#[pallet::pallet]
// 	#[pallet::generate_store(pub (super) trait Store)]
// 	#[pallet::without_storage_info]
// 	pub struct Pallet<T>(_);

// 	#[pallet::storage]
// 	#[allow(clippy::disallowed_types)]
// 	pub type ChainIdToMiltihopRoutePath<T: Config> = StorageMap<
// 		_,
// 		Blake2_128Concat,
// 		u128, //chain id
// 		BoundedBTreeSet<(ChainInfo, BoundedVec<u8, T::ChainNameVecLimit>), T::MaxMultihopCount>, /* route to forward */
// 		ValueQuery,
// 	>;

// 	#[pallet::hooks]
// 	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

// 	// The pallet's dispatchable functions.
// 	#[pallet::call]
// 	impl<T: Config> Pallet<T> {}

// 	use frame_system::RawOrigin;

// 	use xcm::latest::prelude::*;
// 	impl<T: Config> MultiCurrencyCallback<T> for Pallet<T>
// 	where
// 		T: Send + Sync,
// 		u32: From<<T as frame_system::Config>::BlockNumber>,
// 		sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>,
// 	{
// 		fn deposit_asset(
// 			asset: &MultiAsset,
// 			location: &MultiLocation,
// 			_context: &XcmContext,
// 			deposit_result: Result,
// 			asset_id: Option<<T as pallet_ibc::Config>::AssetId>,
// 		) -> core::result::Result<(), Error<T>> {
// 			let location_info = match location {
// 				MultiLocation {
// 					parents: 0,
// 					interior:
// 						X4(
// 							PalletInstance(pallet_id),
// 							GeneralIndex(chain_id),
// 							AccountId32 { id: current_network_address, network: None },
// 							AccountId32 { id: ibc1, network: None },
// 						),
// 				} if *pallet_id == T::PalletInstanceId::get() =>
// 					Some((*current_network_address, *chain_id, vec![ibc1])),
// 				MultiLocation {
// 					parents: 0,
// 					interior:
// 						X5(
// 							PalletInstance(pallet_id),
// 							GeneralIndex(chain_id),
// 							AccountId32 { id: current_network_address, network: None },
// 							AccountId32 { id: ibc1, network: None },
// 							AccountId32 { id: ibc2, network: None },
// 						),
// 				} if *pallet_id == T::PalletInstanceId::get() =>
// 					Some((*current_network_address, *chain_id, vec![ibc1, ibc2])),
// 				MultiLocation {
// 					parents: 0,
// 					interior:
// 						X6(
// 							PalletInstance(pallet_id),
// 							GeneralIndex(chain_id),
// 							AccountId32 { id: current_network_address, network: None },
// 							AccountId32 { id: ibc1, network: None },
// 							AccountId32 { id: ibc2, network: None },
// 							AccountId32 { id: ibc3, network: None },
// 						),
// 				} if *pallet_id == T::PalletInstanceId::get() =>
// 					Some((*current_network_address, *chain_id, vec![ibc1, ibc2, ibc3])),
// 				MultiLocation {
// 					parents: 0,
// 					interior:
// 						X7(
// 							PalletInstance(pallet_id),
// 							GeneralIndex(chain_id),
// 							AccountId32 { id: current_network_address, network: None },
// 							AccountId32 { id: ibc1, network: None },
// 							AccountId32 { id: ibc2, network: None },
// 							AccountId32 { id: ibc3, network: None },
// 							AccountId32 { id: ibc4, network: None },
// 						),
// 				} if *pallet_id == T::PalletInstanceId::get() =>
// 					Some((*current_network_address, *chain_id, vec![ibc1, ibc2, ibc3, ibc4])),
// 				MultiLocation {
// 					parents: 0,
// 					interior:
// 						X8(
// 							PalletInstance(pallet_id),
// 							GeneralIndex(chain_id),
// 							AccountId32 { id: current_network_address, network: None },
// 							AccountId32 { id: ibc1, network: None },
// 							AccountId32 { id: ibc2, network: None },
// 							AccountId32 { id: ibc3, network: None },
// 							AccountId32 { id: ibc4, network: None },
// 							AccountId32 { id: ibc5, network: None },
// 						),
// 				} if *pallet_id == T::PalletInstanceId::get() =>
// 					Some((*current_network_address, *chain_id, vec![ibc1, ibc2, ibc3, ibc4, ibc5])),
// 				_ => None,
// 			};

// 			let (address_from, chain_id, mut addresses) =
// 				location_info.ok_or_else(|| Error::<T>::IncorrectMultiLocation)?;

// 			//deposit does not executed propertly. nothing todo. assets will stay in the account id
// 			// address
// 			deposit_result.map_err(|_| Error::<T>::XcmDepositFailed)?;

// 			//route does not exist
// 			let route = ChainIdToMiltihopRoutePath::<T>::try_get(chain_id)
// 				.map_err(|_| Error::<T>::MultiHopRouteDoesNotExist)?;

// 			let route_len = route.len();
// 			let mut chain_info_iter = route.into_iter();

// 			//route does not exist
// 			let (next_chain_info, _) =
// 				chain_info_iter.next().ok_or(Error::<T>::MultiHopRouteDoesNotExist)?;

// 			if addresses.len() != route_len - 1 {
// 				//wrong XCM MultiLocation. route len does not match addresses list in XCM call.
// 				return Err(Error::<T>::IncorrectCountOfAddresses)
// 			}

// 			let raw_address_to = addresses.remove(0); //remove first element and put into transfer_params.
// 			let account_id = MultiAddress::<AccoindIdOf<T>>::Raw(raw_address_to.to_vec());
// 			let transfer_params = TransferParams::<AccoindIdOf<T>> {
// 				to: account_id,
// 				source_channel: next_chain_info.channel_id,
// 				timeout: IbcTimeout::Offset {
// 					timestamp: next_chain_info.timestamp,
// 					height: next_chain_info.height,
// 				},
// 			};

// 			let account_from = sp_runtime::AccountId32::new(address_from);
// 			let mut account_from_32: &[u8] = sp_runtime::AccountId32::as_ref(&account_from);
// 			//TODO replace unwrap.
// 			let account_id_from = T::AccountId::decode(&mut account_from_32).unwrap();
// 			let signed_account_id = RawOrigin::Signed(account_id_from.clone());

// 			//do not support non fungible.
// 			let Fungibility::Fungible(ref amount) = asset.fun else{
// 				return Err(Error::<T>::DoesNotSupportNonFungible);
// 			};

// 			let mut memo: Option<<T as pallet_ibc::Config>::MemoMessage> = None;

// 			// chain_info_iter does not contains the first IBC chain in the route, addresses does
// 			// not contain first ibc address as well.
// 			let vec: Vec<_> = chain_info_iter
// 				.zip(addresses.into_iter())
// 				.map(|(i, address)| (i.0, i.1, address.clone()))
// 				.collect();

// 			//not able to derive address. and construct memo for multihop.
// 			let memo_data =
// 				MemoData::new::<T>(vec).map_err(|_| Error::<T>::FailedToConstructMemo)?;
// 			match memo_data {
// 				Some(memo_data) => {
// 					let memo_str = format!("{:?}", memo_data); //create a string memo

// 					let memo_result = <T as pallet_ibc::Config>::MemoMessage::from_str(&memo_str);

// 					memo = Some(memo_result.map_err(|_| Error::<T>::FailedToConstructMemo)?);
// 				},
// 				_ => {},
// 			}

// 			let result = pallet_ibc::Pallet::<T>::transfer(
// 				signed_account_id.into(),
// 				transfer_params,
// 				asset_id.unwrap(),
// 				(*amount).into(),
// 				memo.clone(),
// 			);
// 			match result {
// 				Ok(_) => {
// 					<Pallet<T>>::deposit_event(crate::Event::<T>::SuccessXcmToIbc {
// 						origin_address: account_id_from,
// 						to: raw_address_to.clone(),
// 						amount: *amount,
// 						asset_id: asset_id.unwrap(),
// 						memo,
// 					});
// 				},
// 				Err(_) => {
// 					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedXcmToIbc {
// 						origin_address: account_id_from,
// 						to: raw_address_to.clone(),
// 						amount: *amount,
// 						asset_id: asset_id.unwrap(),
// 						memo,
// 					});
// 				},
// 			}
// 			core::result::Result::Ok(())
// 		}
// 	}
// }

// use serde::Serialize;
// use xcm::v3::*;
// pub trait MultiCurrencyCallback<T: Config> {
// 	fn deposit_asset(
// 		asset: &MultiAsset,
// 		location: &MultiLocation,
// 		context: &XcmContext,
// 		deposit_result: Result,
// 		asset_id: Option<<T as pallet_ibc::Config>::AssetId>,
// 	) -> core::result::Result<(), Error<T>>;
// 	//check result, unwrap memo if exists and execute ibc packet
// }
