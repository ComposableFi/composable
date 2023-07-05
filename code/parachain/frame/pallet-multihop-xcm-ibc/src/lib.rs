// #![cfg_attr(
// 	not(test),
// 	deny(
// 		clippy::disallowed_methods,
// 		clippy::disallowed_types,
// 		clippy::indexing_slicing,
// 		clippy::todo,
// 		clippy::unwrap_used,
// 		clippy::panic
// 	)
// )] // allow in tests
// #![deny(clippy::unseparated_literal_suffix, unused_imports, dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
pub use pallet::*;

mod prelude;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::{pallet_prelude::*, BoundedBTreeSet};
	use frame_system::RawOrigin;
	use ibc_primitives::Timeout as IbcTimeout;
	use pallet_ibc::{MultiAddress, TransferParams};
	use xcm::latest::prelude::*;
	// use prelude::{MultiCurrencyCallback, MemoData};
	use composable_traits::{
		prelude::{String, Vec},
		xcm::assets::MultiCurrencyCallback, currency,
	};
	use core::str::FromStr;
	use frame_system::ensure_root;

	use composable_traits::{
		prelude::ToString,
		xcm::memo::{ChainInfo, Forward, MemoData},
	};
	use sp_std::boxed::Box;

	use frame_support::BoundedVec;

	type AccoindIdOf<T> = <T as frame_system::Config>::AccountId;
	use frame_system::pallet_prelude::OriginFor;

	/// ## Configuration
	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_ibc::Config + orml_xtokens::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type PalletInstanceId: Get<u8>;

		#[pallet::constant]
		type MaxMultihopCount: Get<u32>;

		#[pallet::constant]
		type ChainNameVecLimit: Get<u32>;
	}

	// The pallet's events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SuccessXcmToIbc {
			origin_address: T::AccountId,
			to: [u8; 32],
			amount: u128,
			asset_id: T::AssetId,
			memo: Option<T::MemoMessage>,
			bytes: Vec<u8>,
			memo_size: u128,
		},
		FailedXcmToIbc {
			origin_address: T::AccountId,
			to: [u8; 32],
			amount: u128,
			asset_id: T::AssetId,
			memo: Option<T::MemoMessage>,
		},
		FailedCallback {
			origin_address: [u8; 32],
			route_id: u128,
			reason: u8,
		},
		MultihopMemo {
			reason: u8,
			memo_none: bool,
		},
		FailedMatchLocation {},
	}

	#[pallet::error]
	pub enum Error<T> {
		IncorrectAddress { chain_id: u8 },
		IncorrectChainName { chain_id: u8 },
		FailedToEncodeBech32Address { chain_id: u8 },
		IncorrectMultiLocation,
		XcmDepositFailed,
		MultiHopRouteDoesNotExist,
		DoesNotSupportNonFungible,
		IncorrectCountOfAddresses,
		FailedToConstructMemo,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn route_id_to_miltihop_path)]
	pub type ChainIdToMiltihopRoutePath<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u128, //chain id
		BoundedVec<(ChainInfo, BoundedVec<u8, T::ChainNameVecLimit>), T::MaxMultihopCount>, /* route to forward */
		ValueQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// The pallet's dispatchable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(1000)]
		pub fn add_route(
			origin: OriginFor<T>,
			route_id: u128,
			route: BoundedVec<
				(ChainInfo, BoundedVec<u8, T::ChainNameVecLimit>),
				T::MaxMultihopCount,
			>,
		) -> DispatchResult {
			ensure_root(origin)?;
			ChainIdToMiltihopRoutePath::<T>::insert(route_id, route);
			Ok(())
		}
	}

	impl<T: Config> pallet_ibc::ics20::SubstrateMultihopXcmHandler for Pallet<T>
	where u128 : Into<<T as orml_xtokens::Config>::Balance>,
		  u128 : Into<<T as orml_xtokens::Config>::CurrencyId>,
	 {
		type AccountId = T::AccountId;
		//todo use para id to xcm into some parachain
		// fn transfer_xcm(from: T::AccountId, to: T::AccountId, para_id: Option<u32>, amount: u128, currency: u128) -> Option<()> where <T as orml_xtokens::Config>::CurrencyId: From<u128>, <T as orml_xtokens::Config>::Balance: From<u128>{
			fn transfer_xcm(from: T::AccountId, to: T::AccountId, para_id: Option<u32>, amount: u128, currency: u128) -> Option<()> {
			let signed_account_id = RawOrigin::Signed(from.clone());
			let acc_bytes = T::AccountId::encode(&to);
			let id = acc_bytes.try_into().unwrap();
			let _result = orml_xtokens::Pallet::<T>::transfer(
				signed_account_id.into(),
				currency.into(),
				amount.into(),
				Box::new(
					xcm::latest::MultiLocation::new(
						0,
						xcm::latest::Junctions::X1(xcm::latest::Junction::AccountId32 { id: id, network: None })
					)
					.into()
				),
				WeightLimit::Unlimited,
			);
			None
		}
	}
	impl<T: Config> Pallet<T> {

		// //todo use para id to xcm into some parachain
		// pub fn transfer_xcm(from: T::AccountId, to: T::AccountId, para_id: Option<u128>, amount: u128, currency: u128) where <T as orml_xtokens::Config>::CurrencyId: From<u128>, <T as orml_xtokens::Config>::Balance: From<u128>{
		// 	let signed_account_id = RawOrigin::Signed(from.clone());
		// 	let acc_bytes = T::AccountId::encode(&to);
		// 	let id = acc_bytes.try_into().unwrap();
		// 	let _result = orml_xtokens::Pallet::<T>::transfer(
		// 		signed_account_id.into(),
		// 		currency.into(),
		// 		amount.into(),
		// 		Box::new(
		// 			xcm::latest::MultiLocation::new(
		// 				0,
		// 				xcm::latest::Junctions::X1(xcm::latest::Junction::AccountId32 { id: id, network: None })
		// 			)
		// 			.into()
		// 		),
		// 		WeightLimit::Unlimited,
		// 	);
			
		// }

		/// Support only addresses from cosmos ecosystem based on bech32.
		pub fn create_memo(
			mut vec: Vec<(ChainInfo, Vec<u8>, [u8; 32])>,
		) -> Result<Option<MemoData>, DispatchError> {
			vec.reverse(); // reverse to create memo from the end

			//osmosis(ibc) <- huahua(ibc) <- centauri(ibc)  =  ibc transfer from composable to
			//picasso with memo substrate(xcm) hop can be only the last one
			//moonriver(xcm) = ibc transfer from composable to picasso
			//polkadot(xcm) = ibc transfer from picasso to composable

			let mut last_memo_data: Option<MemoData> = None;

			for (i, name, address) in vec {
				let mut forward = if i.is_substrate_xcm {
					// let str = "0x" + hex::encode(&bytes);
					let memo_receiver = scale_info::prelude::format!("0x{}", hex::encode(&address));
					Forward::new_xcm_memo(memo_receiver, i.para_id)
				} else {
					let memo_receiver = if i.is_substrate_ibc{
						scale_info::prelude::format!("0x{}", hex::encode(&address))
					}
					else{
						let result: core::result::Result<Vec<bech32_no_std::u5>, bech32_no_std::Error> =
						address.into_iter().map(bech32_no_std::u5::try_from_u8).collect();
						let data =
							// result.map_err(|_| Error::<T>::IncorrectAddress { chain_id: i.chain_id as u8 })?;
							result.map_err(|_| DispatchError::Other("()"))?;

						let name = String::from_utf8(name.into())
							// .map_err(|_| Error::<T>::IncorrectChainName { chain_id: i.chain_id as u8
							// })?;
							.map_err(|_| DispatchError::Other("()"))?;
							bech32_no_std::encode(&name, data.clone()).map_err(|_| {
								// Error::<T>::FailedToEncodeBech32Address { chain_id: i.chain_id as u8
								// }
								DispatchError::Other("()")
							})?
					};

					Forward::new_ibc_memo(
						memo_receiver,
						String::from("transfer"),
						String::from(scale_info::prelude::format!("channel-{}", i.channel_id)),
						i.timeout.unwrap_or_default().to_string(),
						i.retries.unwrap_or_default(),
					)
				};
				if let Some(memo_memo) = last_memo_data {
					forward.next = Some(Box::new(memo_memo));
				};
				let new_memo = MemoData::new(forward);
				last_memo_data = Some(new_memo);
			}
			<Pallet<T>>::deposit_event(crate::Event::<T>::MultihopMemo {
				reason: 255,
				memo_none: last_memo_data.is_none(),
			});
			Ok(last_memo_data)
		}
	}

	impl<T: Config> MultiCurrencyCallback for Pallet<T>
	where
		T: Send + Sync,
		u32: From<<T as frame_system::Config>::BlockNumber>,
		sp_runtime::AccountId32: From<<T as frame_system::Config>::AccountId>,
	{
		type AssetId = T::AssetId;

		fn deposit_asset(
			asset: &xcm::latest::MultiAsset,
			location: &xcm::latest::MultiLocation,
			context: &xcm::latest::XcmContext,
			deposit_result: xcm::latest::Result,
			asset_id: Option<Self::AssetId>,
		) -> Option<()> {
			let location_info = match location {
				MultiLocation {
					parents: 0,
					interior:
						X4(
							PalletInstance(pallet_id),
							GeneralIndex(route_id),
							AccountId32 { id: current_network_address, network: _ },
							AccountId32 { id: ibc1, network: _ },
						),
				} => {
					let mut vec = sp_std::vec::Vec::new();
					vec.push(ibc1.clone());
					(pallet_id, *current_network_address, *route_id, vec)
				},
				MultiLocation {
					parents: 0,
					interior:
						X5(
							PalletInstance(pallet_id),
							GeneralIndex(route_id),
							AccountId32 { id: current_network_address, network: _ },
							AccountId32 { id: ibc1, network: _ },
							AccountId32 { id: ibc2, network: _ },
						),
				} => {
					let mut vec = sp_std::vec::Vec::new();
					vec.push(ibc1.clone());
					vec.push(ibc2.clone());
					(pallet_id, *current_network_address, *route_id, vec)
				},
				MultiLocation {
					parents: 0,
					interior:
						X6(
							PalletInstance(pallet_id),
							GeneralIndex(route_id),
							AccountId32 { id: current_network_address, network: _ },
							AccountId32 { id: ibc1, network: _ },
							AccountId32 { id: ibc2, network: _ },
							AccountId32 { id: ibc3, network: _ },
						),
				} => {
					let mut vec = sp_std::vec::Vec::new();
					vec.push(ibc1.clone());
					vec.push(ibc2.clone());
					vec.push(ibc3.clone());
					(pallet_id, *current_network_address, *route_id, vec)
				},
				MultiLocation {
					parents: 0,
					interior:
						X7(
							PalletInstance(pallet_id),
							GeneralIndex(route_id),
							AccountId32 { id: current_network_address, network: _ },
							AccountId32 { id: ibc1, network: _ },
							AccountId32 { id: ibc2, network: _ },
							AccountId32 { id: ibc3, network: _ },
							AccountId32 { id: ibc4, network: _ },
						),
				} => {
					let mut vec = sp_std::vec::Vec::new();
					vec.push(ibc1.clone());
					vec.push(ibc2.clone());
					vec.push(ibc3.clone());
					vec.push(ibc4.clone());
					(pallet_id, *current_network_address, *route_id, vec)
				},
				MultiLocation {
					parents: 0,
					interior:
						X8(
							PalletInstance(pallet_id),
							GeneralIndex(route_id),
							AccountId32 { id: current_network_address, network: _ },
							AccountId32 { id: ibc1, network: _ },
							AccountId32 { id: ibc2, network: _ },
							AccountId32 { id: ibc3, network: _ },
							AccountId32 { id: ibc4, network: _ },
							AccountId32 { id: ibc5, network: _ },
						),
				} => {
					let mut vec = sp_std::vec::Vec::new();
					vec.push(ibc1.clone());
					vec.push(ibc2.clone());
					vec.push(ibc3.clone());
					vec.push(ibc4.clone());
					vec.push(ibc5.clone());
					(pallet_id, *current_network_address, *route_id, vec)
				},
				_ => {
					//emit event
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedMatchLocation {});
					return None
				},
			};

			let (pallet_id, address_from, route_id, mut addresses) = location_info;

			if *pallet_id != T::PalletInstanceId::get() {
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 1,
				});
				return None
			}

			// return None;

			//deposit does not executed propertly. nothing todo. assets will stay in the account id
			// address
			// deposit_result.map_err(|_| Error::<T>::XcmDepositFailed)?;
			deposit_result.ok()?;

			//route does not exist
			// let route = ChainIdToMiltihopRoutePath::<T>::try_get(chain_id)
			// 	.map_err(|_| Error::<T>::MultiHopRouteDoesNotExist)?;
			let Ok(mut route) = ChainIdToMiltihopRoutePath::<T>::try_get(route_id) else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 2,
				});
				return None;
			};

			let route_len = route.len();
			//order route by chain_id 
			route.sort_by(|a, b| a.0.order.cmp(&b.0.order));
			let mut chain_info_iter = route.into_iter();

			//route does not exist
			// let (next_chain_info, _) =
			// 	chain_info_iter.next().ok_or(Error::<T>::MultiHopRouteDoesNotExist)?;
			let Some((next_chain_info, _)) = chain_info_iter.next() else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 3,
				});
				return None;
			};

			if addresses.len() != route_len {
				//wrong XCM MultiLocation. route len does not match addresses list in XCM call.
				// return Err(Error::<T>::IncorrectCountOfAddresses)
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 4,
				});
				return None
			}

			let raw_address_to = addresses.remove(0); //remove first element and put into transfer_params.
			let mut account_id = MultiAddress::<AccoindIdOf<T>>::Raw(raw_address_to.to_vec());
			//account_id 32 bytes to MultiAddress does not work. need convert with bech32 into IBC
			// string address and then convert into MultiAddress raw address
			if !next_chain_info.is_substrate_ibc {
				//does not support not substrate ibc
				//to support IBC chain need to convert address to IBC address using bech32(the same
				// as in memo::new function)
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 5,
				});
				return None
			} else {
				let account_from = sp_runtime::AccountId32::new(raw_address_to);
				let mut account_from_32: &[u8] = sp_runtime::AccountId32::as_ref(&account_from);
				let Ok(account_id_from) = T::AccountId::decode(&mut account_from_32) else{
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
						origin_address: address_from,
						route_id,
						reason: 6,
					});
					return None;
				};
				account_id = MultiAddress::<AccoindIdOf<T>>::Id(account_id_from);
			}
			// let accunt_id = MultiAddress::<AccoindIdOf<T>>::Id();
			let transfer_params = TransferParams::<AccoindIdOf<T>> {
				to: account_id,
				source_channel: next_chain_info.channel_id,
				timeout: IbcTimeout::Offset {
					timestamp: next_chain_info.timestamp,
					height: next_chain_info.height,
				},
			};

			let account_from = sp_runtime::AccountId32::new(address_from);
			let mut account_from_32: &[u8] = sp_runtime::AccountId32::as_ref(&account_from);
			let Ok(account_id_from) = T::AccountId::decode(&mut account_from_32) else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 7,
				});
				return None;
			};
			let signed_account_id = RawOrigin::Signed(account_id_from.clone());

			//do not support non fungible.
			let Fungibility::Fungible(ref amount) = asset.fun else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 8,
				});
				return None;
				// return Err(Error::<T>::DoesNotSupportNonFungible);
			};

			let mut memo: Option<<T as pallet_ibc::Config>::MemoMessage> = None;

			// chain_info_iter does not contains the first IBC chain in the route, addresses does
			// not contain first ibc address as well.

			let vec: sp_std::vec::Vec<_> = chain_info_iter
				.zip(addresses.into_iter())
				.map(|(i, address)| (i.0, i.1.into_inner(), address.clone()))
				.collect();

			//not able to derive address. and construct memo for multihop.
			//TODO: uncomment when memo will be supported.
			let memo_data = Pallet::<T>::create_memo(vec);
			let Ok(memo_data) = memo_data else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: 9,
				});
				return None;
			};
			match Some(memo_data) {
				Some(memo_data) => {
					let memo_str = serde_json::to_string(&memo_data).unwrap();
					<Pallet<T>>::deposit_event(crate::Event::<T>::MultihopMemo {
						reason: 10,
						memo_none: memo_str.len() < 1,
					});
					let memo_result = <T as pallet_ibc::Config>::MemoMessage::from_str(&memo_str);

					let Ok(memo_result) = memo_result else{
						<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
							origin_address: address_from,
							route_id,
							reason: 10,
						});
						return None;
					};
					memo = Some(memo_result)
					// memo = Some(memo_result.map_err(|_| Error::<T>::FailedToConstructMemo)?);
				},
				_ => {},
			}

			<Pallet<T>>::deposit_event(crate::Event::<T>::MultihopMemo {
				reason: 11,
				memo_none: memo.is_none()
			});

			let result = pallet_ibc::Pallet::<T>::transfer(
				signed_account_id.into(),
				transfer_params,
				asset_id.unwrap(), //TODO remove unwrap
				(*amount).into(),
				memo.clone(),
			);

			let memo_bytes = memo.clone().map(|m| m.to_string().as_bytes().to_vec()).unwrap_or(Vec::new());
			match result {
				Ok(_) => {
					<Pallet<T>>::deposit_event(crate::Event::<T>::SuccessXcmToIbc {
						origin_address: account_id_from,
						to: raw_address_to.clone(),
						amount: *amount,
						asset_id: asset_id.unwrap(),
						memo,
						bytes: memo_bytes.clone(),
						memo_size: memo_bytes.len() as u128,
					});
				},
				Err(e) => {
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedXcmToIbc {
						origin_address: account_id_from,
						to: raw_address_to.clone(),
						amount: *amount,
						asset_id: asset_id.unwrap(),
						memo,
					});
					return None
				},
			}
			Some(())
		}
	}
}
