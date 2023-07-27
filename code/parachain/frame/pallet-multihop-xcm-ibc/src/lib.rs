#![cfg_attr(
	not(test),
	deny(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![warn(bad_style, trivial_numeric_casts)]
#![deny(
	bare_trait_objects,
	improper_ctypes,
	no_mangle_generic_items,
	non_shorthand_field_patterns,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	trivial_casts,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_extern_crates,
	unused_imports,
	unused_parens,
	while_true
)]
#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

mod prelude;

#[frame_support::pallet]
pub mod pallet {

	use super::{prelude::*, *};
	use bech32_no_std::u5;
	use composable_traits::{
		centauri::Map,
		prelude::{String, Vec},
		xcm::{assets::MultiCurrencyCallback, memo::ChainHop},
	};
	use frame_support::pallet_prelude::*;
	use frame_system::{ensure_root, RawOrigin};
	use ibc_primitives::Timeout as IbcTimeout;
	use ibc_rs_scale::core::ics24_host::identifier::{ChannelId, PortId};
	use pallet_ibc::{MultiAddress, TransferParams};
	use xc_core::ibc::ics20::{
		pfm::{Forward, IbcSubstrate},
		MemoData,
	};
	use xcm::latest::prelude::*;

	use composable_traits::{prelude::ToString, xcm::memo::ChainInfo};
	use sp_std::boxed::Box;

	use frame_support::BoundedVec;

	type AccoindIdOf<T> = <T as frame_system::Config>::AccountId;
	type RouteBoundedVec<T> = BoundedVec<
		(ChainInfo, BoundedVec<u8, <T as Config>::ChainNameVecLimit>),
		<T as Config>::MaxMultihopCount,
	>;
	type ListChainNameAddress = Vec<(ChainInfo, Vec<u8>, [u8; 32])>;
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

		/// The maximum length of chain name
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
			reason: MultihopEventReason,
		},
		MultihopXcmMemo {
			reason: MultihopEventReason,
			from: T::AccountId,
			to: T::AccountId,
			amount: u128,
			asset_id: u128,
			is_error: bool,
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
		FailedToDecodeAccountId,
	}

	#[derive(
		Copy,
		Clone,
		PartialEq,
		Eq,
		Hash,
		codec::Encode,
		codec::Decode,
		scale_info::TypeInfo,
		Ord,
		PartialOrd,
		MaxEncodedLen,
		Debug,
	)]
	pub enum MultihopEventReason {
		FailedToConvertAddressToBytes,
		XcmTransferInitiated,
		IncorrectPalletId,
		MultiHopRouteDoesNotExist,
		MultiHopRouteExistButNotConfigured,
		IncorrectCountOfAddresses,
		FailedToDeriveCosmosAddressFromBytes,
		FailedToDeriveChainNameFromUtf8,
		FailedToEncodeBech32Address,
		FailedToDecodeDestAccountId,
		FailedToDecodeSenderAccountId,
		DoesNotSupportNonFungible,
		FailedCreateMemo,
		FailedToConvertMemoIntoPalletIbcMemoMessageType,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn route_id_to_miltihop_path)]
	pub type RouteIdToRoutePath<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u128,               /* route id */
		RouteBoundedVec<T>, /* route to forward */
		OptionQuery,
	>;

	// The pallet's dispatchable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(100_000)]
		pub fn add_route(
			origin: OriginFor<T>,
			route_id: u128,
			route: RouteBoundedVec<T>,
		) -> DispatchResult {
			ensure_root(origin)?;
			RouteIdToRoutePath::<T>::insert(route_id, route);
			Ok(())
		}
	}

	impl<T: Config> pallet_ibc::ics20::SubstrateMultihopXcmHandler for Pallet<T>
	where
		u128: Into<<T as orml_xtokens::Config>::Balance>,
		u128: Into<<T as orml_xtokens::Config>::CurrencyId>,
	{
		type AccountId = T::AccountId;
		/// Transfer asset from current chain to another chain(relay-chain or parachain).
		/// The function is part of multihop execution.
		/// In case when ibc memo includes `Xcm` hop, the function will be called from pallet-ibc
		/// 'execute memo' function.
		///
		/// This function return Some(()) if transfer is successful and None if transfer failed.
		/// The failure of `transfer_xcm` does not affect the current IBC transfer process. This is
		/// why we return None instead of Result.
		///
		/// This function emit event for front-end to track the process because this is the only way
		/// to give some feedback to the user about Multihop execution.
		///
		/// There is no reason to return Result with error because `process-memo' in pallet-ibc is
		/// the part of execution of `deliver` extrinsic called by relayer. Because of origin of the
		/// extrinsic is relayer, the error will be returned to relayer and not to the user. This is
		/// why we emit event and return None instead of Result. Event needs to notify front-end
		/// about the execution.
		///
		/// This function aslo emit event about failed execution with `reason` to provide why it
		/// failed. This events with a reason also help to trace the multihop execution and find
		/// where it failed without looking into the logs.
		///
		/// # Arguments
		/// * `from` - account id of the sender. This is account thet recieved IBC transfer and need
		///   to initiate XCM hop accuding to multihop logic
		/// * `to` - account id of the receiver. This is an destination account of the XCM hop that
		///   was specified in ibc memo
		/// * `para_id` - para id of the receiver. If para id is none then xcm to relay-chain. if
		///   para is some then xcm to parachain
		/// * `amount` - amount of the asset to transfer
		/// * `currency` - currency id of the asset to transfer
		fn transfer_xcm(
			from: T::AccountId,
			to: T::AccountId,
			para_id: Option<u32>,
			amount: u128,
			currency: u128,
		) -> Option<()> {
			let signed_account_id = RawOrigin::Signed(from.clone());
			let acc_bytes = T::AccountId::encode(&to);
			let Ok(id) = acc_bytes.try_into() else{
				//we need to emit event when error or succseed for front-end becase 
				//this function is called from pallet-ibc 
				//deliver extrinsic and only relayer will get the error. 
				<Pallet<T>>::deposit_event(crate::Event::<T>::MultihopXcmMemo {
					reason: MultihopEventReason::FailedToConvertAddressToBytes,
					from: from.clone(),
					to: to.clone(),
					amount: amount,
					asset_id: currency,
					is_error: true,
				});
				return None;
			};
			//if para id is none then parent is 1 if para id is some then parent is 0
			let parent = if para_id.is_some() { 0 } else { 1 };
			let result = orml_xtokens::Pallet::<T>::transfer(
				signed_account_id.into(),
				currency.into(),
				amount.into(),
				Box::new(
					xcm::latest::MultiLocation::new(
						parent, //parent
						xcm::latest::Junctions::X1(xcm::latest::Junction::AccountId32 {
							id,
							network: None,
						}),
					)
					.into(),
				),
				WeightLimit::Unlimited,
			);

			if let Err(e) = result {
				frame_support::log::info!(
					"transfer_xcm from: {:?}, to: {:?}, para_id: {:?}, amount: {:?}, currency: {:?}, error: {:?}",
					from,
					to,
					para_id,
					amount,
					currency,
					e
				);
			}

			//track the error as event and return none
			<Pallet<T>>::deposit_event(crate::Event::<T>::MultihopXcmMemo {
				reason: MultihopEventReason::XcmTransferInitiated,
				from: from.clone(),
				to: to.clone(),
				amount,
				asset_id: currency,
				is_error: result.is_err(),
			});

			result.ok()
		}
	}

	impl<T: Config> Pallet<T> {
		/// Create memo from the route configuration using chain info, name and address
		///
		/// This function is called from deposit_asset XCM callback function in
		/// pallet-multihop-xcm-ibc
		///
		/// # Arguments
		/// - `list_chain_name_address` - list of chain info, name and address to create memo
		///
		/// # Returns
		/// * `Option<MemoData>` - return none if failed to create memo
		///
		/// # Errors
		/// 	* `DispatchError` - if failed to create memo. Failed to create memo can happen if:
		/// 	* Failed to convert u8 into u5
		/// 	* Failed to convert chain name from utf8
		/// 	* Failed to convert chain name and address into bech32
		/// 	* Timeout is none
		/// 	* Retries is none
		///
		/// # Example
		/// ```ignore
		/// Pallet::create_memo(
		/// 	list_chain_name_address: ListChainNameAddress,
		/// )
		/// ```
		///
		/// # Note
		/// Return Ok(None) if `list_chain_name_address` is empty
		pub fn create_memo(
			mut list_chain_name_address: ListChainNameAddress,
		) -> Result<Option<MemoData>, DispatchError> {
			list_chain_name_address.reverse(); // reverse to create memo from the end

			//osmosis(ibc) <- huahua(ibc) <- centauri(ibc)  =  ibc transfer from composable to
			//picasso with memo substrate(xcm) hop can be only the last one
			//moonriver(xcm) = ibc transfer from composable to picasso
			//polkadot(xcm) = ibc transfer from picasso to composable

			let mut last_memo_data: Option<MemoData> = None;

			for (i, name, address) in list_chain_name_address {
				let mut forward = if i.chain_hop == ChainHop::Xcm {
					let memo_receiver = scale_info::prelude::format!("0x{}", hex::encode(&address));
					Forward::new_xcm_memo(memo_receiver, IbcSubstrate::new(i.para_id))
				} else {
					let memo_receiver = if i.chain_hop == ChainHop::SubstrateIbc {
						scale_info::prelude::format!("0x{}", hex::encode(&address))
					} else {
						let data: Vec<u5> = address
							.into_iter()
							.map(|byte| {
								u5::try_from_u8(byte).map_err(|_| {
									DispatchError::Other("Failed to convert u8 into u5")
								})
							})
							.collect::<Result<_, _>>()?;

						let name = String::from_utf8(name.into()).map_err(|_| {
							DispatchError::Other("Failed to convert chain name from utf8")
						})?;
						bech32_no_std::encode(&name, data.clone()).map_err(|_| {
							DispatchError::Other(
								"Failed to convert chain name and address into bech32",
							)
						})?
					};

					Forward::new_ibc_memo(
						memo_receiver,
						PortId::transfer(),
						ChannelId::new(i.channel_id),
						i.timeout.ok_or(DispatchError::Other("Timeout is none"))?.to_string(),
						i.retries.ok_or(DispatchError::Other("Retries is none"))?,
					)
				};
				if let Some(memo_memo) = last_memo_data {
					forward.next = Some(Box::new(memo_memo));
				};
				let new_memo = MemoData::forward(forward);
				last_memo_data = Some(new_memo);
			}
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

		/// Create memo if need and call pallet-ibc `transfer` function to initiate IBC transfer
		///
		/// This function is called from `MultiCurrencyAdapter` (xcmp.rs) deposit function after XCM
		/// execution. The function does not return error to revert XCM deposit execution because
		/// the failure of multihop deposit should not affect the current XCM deposit process. This
		/// is why we return None instead of Result
		///
		/// This function emit event for front-end to track the error because this is the only way
		/// to give some feedback to the user about Multihop execution
		///
		/// This function aslo emit event about failed execution with `reason` to provide why it
		/// failed. This events with a reason also help to trace the multihop execution and find
		/// where it failed without looking into the logs
		///
		/// # Arguments
		/// * `asset` - asset to deposit. Support only fungible assets
		/// * `location` - location of the chain to deposit, should be in format: X4, X5, X6, X7 or
		///   X8
		/// Example:
		/// 			X4(
		///					PalletInstance(pallet_id),
		///					GeneralIndex(route_id),
		///					AccountId32 { id: current_network_address, network: _ },
		///					AccountId32 { id: ibc1, network: _ },
		///				)
		/// * `context` - context of the chain to deposit. ingnored
		/// * `deposit_result` - result of the deposit, should be ok to continue
		/// * `asset_id` - asset id, asset id of the asset to deposit
		///
		/// # Returns
		/// * `Option<()>` - return none if deposit failed. There is no reason to return Result.
		///   because the failure of multihop deposit should not affect the current XCM deposit
		///   process that is why we return None
		///
		/// # Errors
		/// * `None` - if deposit failed. Deposit can fail if:
		/// 	* `location` is not in format X4, X5, X6, X7 or X8 for example X.(PalletInsance,
		///    GeneralIndex, AccountId32, AccountId32, ......)
		/// 	* `location` does not match with the route
		/// 	* `location` does not match with the pallet id
		/// 	* `deposit_result` is not ok
		/// 	* `asset_id` is not supported
		/// 	* incorrect count of addresses compared to count of chain in route configuration
		/// 	* `location` does not match with the chain name
		/// 	* failed to create memo from the route
		///
		/// # Example
		/// ```ignore
		/// MultiCurrencyCallback::deposit_asset(
		/// 	asset: &xcm::latest::MultiAsset,
		/// 	location: &xcm::latest::MultiLocation,
		/// 	context: &xcm::latest::XcmContext,
		/// 	deposit_result: xcm::latest::Result,
		/// 	asset_id: Self::AssetId,
		/// )
		/// ```
		fn deposit_asset(
			asset: &xcm::latest::MultiAsset,
			location: &xcm::latest::MultiLocation,
			_context: &xcm::latest::XcmContext,
			deposit_result: xcm::latest::Result,
			asset_id: Self::AssetId,
		) -> Option<()> {
			deposit_result.ok()?;
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
				} => (pallet_id, *current_network_address, *route_id, sp_std::vec![*ibc1]),
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
				} => (pallet_id, *current_network_address, *route_id, sp_std::vec![*ibc1, *ibc2]),
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
				} => (
					pallet_id,
					*current_network_address,
					*route_id,
					sp_std::vec![*ibc1, *ibc2, *ibc3],
				),
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
				} => (
					pallet_id,
					*current_network_address,
					*route_id,
					sp_std::vec![*ibc1, *ibc2, *ibc3, *ibc4],
				),
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
				} => (
					pallet_id,
					*current_network_address,
					*route_id,
					sp_std::vec![*ibc1, *ibc2, *ibc3, *ibc4, *ibc5],
				),
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
					reason: MultihopEventReason::IncorrectPalletId,
				});
				return None
			}

			//route does not exist
			let Ok(mut route) = RouteIdToRoutePath::<T>::try_get(route_id) else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: MultihopEventReason::MultiHopRouteDoesNotExist,
				});
				return None;
			};

			let route_len = route.len();
			//order route by chain_id
			route.sort_by_key(|item| item.0.order);
			let mut chain_info_iter = route.into_iter();

			//route does not exist
			let Some((next_chain_info, chain_name)) = chain_info_iter.next() else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: MultihopEventReason::MultiHopRouteExistButNotConfigured,
				});
				return None;
			};

			if addresses.len() != route_len {
				//wrong XCM MultiLocation. route len does not match addresses list in XCM call.
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: MultihopEventReason::IncorrectCountOfAddresses,
				});
				return None
			}

			let raw_address_to = addresses.remove(0); //remove first element and put into transfer_params.
			let account_id = if next_chain_info.chain_hop == ChainHop::CosmosIbc {
				let result: core::result::Result<Vec<bech32_no_std::u5>, bech32_no_std::Error> =
					raw_address_to.into_iter().map(bech32_no_std::u5::try_from_u8).collect();

				let Ok(data) = result else{
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
						origin_address: address_from,
						route_id,
						reason: MultihopEventReason::FailedToDeriveCosmosAddressFromBytes,
					});
					return None;
				};

				let Ok(name) = String::from_utf8(chain_name.into()) else {
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
						origin_address: address_from,
						route_id,
						reason: MultihopEventReason::FailedToDeriveChainNameFromUtf8,
					});
					return None;
				};

				let Ok(name) = bech32_no_std::encode(&name, data.clone()) else {
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
						origin_address: address_from,
						route_id,
						reason: MultihopEventReason::FailedToEncodeBech32Address,
					});
					return None;
				};

				MultiAddress::<AccoindIdOf<T>>::Raw(name.into_bytes())
			} else {
				let account_from = sp_runtime::AccountId32::new(raw_address_to);
				let mut account_from_32: &[u8] = sp_runtime::AccountId32::as_ref(&account_from);
				let Ok(account_id_from) = T::AccountId::decode(&mut account_from_32) else{
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
						origin_address: address_from,
						route_id,
						reason: MultihopEventReason::FailedToDecodeDestAccountId,
					});
					return None;
				};
				MultiAddress::<AccoindIdOf<T>>::Id(account_id_from)
			};
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
					reason: MultihopEventReason::FailedToDecodeSenderAccountId,
				});
				return None;
			};
			let signed_account_id = RawOrigin::Signed(account_id_from.clone());

			//do not support non fungible.
			let Fungibility::Fungible(ref amount) = asset.fun else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: MultihopEventReason::DoesNotSupportNonFungible,
				});
				return None;
			};

			let mut memo: Option<<T as pallet_ibc::Config>::MemoMessage> = None;

			// chain_info_iter does not contains the first IBC chain in the route, addresses does
			// not contain first ibc address as well.

			let vec: sp_std::vec::Vec<_> = chain_info_iter
				.zip(addresses.into_iter())
				.map(|(i, address)| (i.0, i.1.into_inner(), address.clone()))
				.collect();

			let memo_data = Pallet::<T>::create_memo(vec);
			let Ok(memo_data) = memo_data else{
				<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
					origin_address: address_from,
					route_id,
					reason: MultihopEventReason::FailedCreateMemo,
				});
				return None;
			};
			match memo_data {
				Some(memo_data) => {
					let memo_result =
						<T as pallet_ibc::Config>::MemoMessage::try_from(Map::from_cw(memo_data));

					let Ok(memo_result) = memo_result else{
						<Pallet<T>>::deposit_event(crate::Event::<T>::FailedCallback {
							origin_address: address_from,
							route_id,
							reason: MultihopEventReason::FailedToConvertMemoIntoPalletIbcMemoMessageType,
						});
						return None;
					};
					memo = Some(memo_result)
				},
				_ => {},
			}

			let result = pallet_ibc::Pallet::<T>::transfer(
				signed_account_id.into(),
				transfer_params,
				asset_id,
				(*amount).into(),
				memo.clone(),
			);

			match result {
				Ok(_) => {
					<Pallet<T>>::deposit_event(crate::Event::<T>::SuccessXcmToIbc {
						origin_address: account_id_from,
						to: raw_address_to.clone(),
						amount: *amount,
						asset_id,
						memo,
					});
				},
				Err(e) => {
					<Pallet<T>>::deposit_event(crate::Event::<T>::FailedXcmToIbc {
						origin_address: account_id_from,
						to: raw_address_to.clone(),
						amount: *amount,
						asset_id,
						memo,
					});
					frame_support::log::error!(
						"Failed to deposit asset: {:?} to location: {:?}, with error: {:?}",
						asset,
						location,
						e
					);
					return None
				},
			}
			Some(())
		}
	}
}
