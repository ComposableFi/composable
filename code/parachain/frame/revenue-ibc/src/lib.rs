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
)]
#![deny(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![warn(bad_style, trivial_numeric_casts)]
#![allow(clippy::let_unit_value)]
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
	// unused_imports,
	unused_parens,
	while_true
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(more_qualified_paths)]
pub use codec::{Decode, Encode, FullCodec};
pub use pallet::*;

pub use sp_std::str::FromStr;

#[frame_support::pallet]
pub mod pallet {
	use bech32_no_std::u5;
	use common::ibc::RawMemo;
	use composable_support::{
		collections::vec::bounded::{bi_bounded_vec::BiBoundedVecOutOfBounds, BiBoundedVec},
		math::safe::{SafeArithmetic, SafeSub},
	};
	use composable_traits::{
		assets::AssetInfo,
		currency::{AssetExistentialDepositInspect, BalanceLike},
		dex::{AssetAmount, FeeConfig, SwapResult},
		prelude::{String, Vec},
		xcm::assets::RemoteAssetRegistryInspect,
	};
	use core::fmt::Debug;
	use frame_support::{
		pallet_prelude::*,
		storage::with_transaction,
		traits::{
			fungibles::{Inspect, Mutate},
			tokens::Preservation,
			Time,
		},
		transactional, BoundedBTreeMap, PalletId, RuntimeDebug,
	};
	use frame_system::{ensure_root, ensure_signed, pallet_prelude::OriginFor, RawOrigin};
	use ibc_primitives::Timeout as IbcTimeout;
	use ibc_rs_scale::{
		applications::transfer::TracePrefix,
		core::ics24_host::identifier::{ChannelId, PortId},
	};
	use pallet_ibc::{
		ics20::ValidateMemo, ics20_fee::FlatFeeConverter, DenomToAssetId, MultiAddress,
		TransferParams,
	};
	use primitives::currency::ForeignAssetId;
	use sp_arithmetic::FixedPointOperand;
	use sp_runtime::{
		traits::{AccountIdConversion, Convert, IdentifyAccount, One, Saturating, Zero},
		AccountId32, ArithmeticError, BoundedBTreeSet, FixedPointNumber, Perbill, Permill,
		TransactionOutcome,
	};
	use sp_std::{boxed::Box, collections::btree_map::BTreeMap};
	pub use sp_std::{prelude::*, str::FromStr, vec};

	pub(crate) type AssetIdOf<T> = <T as pallet_ibc::Config>::AssetId;
	pub(crate) type BalanceOf<T> = <T as pallet_ibc::Config>::Balance;

	pub(crate) type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_ibc::Config {
		#[allow(missing_docs)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// treasury account
		#[pallet::constant]
		type FromPalletId: Get<PalletId>;

		// every period, funds to transfer are sent to IntermediatePalletId, then from it they are
		// send further In case of failure funds come back to this account and revenue calculation
		// for treasury will stay untouched
		#[pallet::constant]
		type IntermediatePalletId: Get<PalletId>;

		// token locationtype
		type ForeignAssetId: codec::FullCodec
			+ Eq
			+ PartialEq
			+ MaybeSerializeDeserialize
			+ Debug
			+ Clone
			+ TypeInfo
			+ MaxEncodedLen;
		type AssetId: codec::FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ Clone
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo
			+ From<u128>
			+ Into<u128>
			+ Ord;
		// get access to tokens location
		type AssetsRegistry: RemoteAssetRegistryInspect<
				AssetId = AssetIdOf<Self>,
				AssetNativeLocation = Self::ForeignAssetId,
			> + AssetExistentialDepositInspect<AssetId = AssetIdOf<Self>, Balance = BalanceOf<Self>>;

		type Assets: Mutate<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>
			+ Inspect<AccountIdOf<Self>, Balance = BalanceOf<Self>, AssetId = AssetIdOf<Self>>;

		#[pallet::constant]
		type MaxStringSize: Get<u32>
			+ TypeInfo
			+ core::fmt::Debug
			+ MaxEncodedLen
			+ Copy
			+ Clone
			+ PartialEq
			+ Eq;

		// root and council
		type Admin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;
	}

	// The pallet's events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PeriodSet {
			period: T::BlockNumber,
		},
		MemoSet {
			memo: BoundedVec<u8, T::MaxStringSize>,
		},
		RevenueTransferred {
			amount: BalanceOf<T>,
			asset_id: AssetIdOf<T>,
			memo: BoundedVec<u8, T::MaxStringSize>,
		},
		TransferFailed {
			asset_id: AssetIdOf<T>,
		},
		SkipAsset {
			asset_id: AssetIdOf<T>,
		},
		TransferSuccess {
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		TransferFail {
			asset_id: AssetIdOf<T>,
			amount: BalanceOf<T>,
		},
		RevenueCalcutions,
		SetAllowed,
		AddAllowed,
		RemoveAllowed,
		SetDisallowed,
		AddDisallowed,
		RemoveDisallowed,
		TransferTriggered,
		IntermediateTransferFail,
	}

	#[pallet::error]
	pub enum Error<T> {
		ChannelNotSet,
		CentauriAddressNotSet
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn allowed_assets)]
	pub type AllowedAssets<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, (), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn disallowed_assets)]
	pub type DisallowedAssets<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, (), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tokens_prev_amount)]
	pub type TokenPrevPeriodBalance<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn period)]
	pub type Period<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn centauri_address)]
	pub type CentauriAddress<T: Config> =
		StorageValue<_, BoundedVec<u8, T::MaxStringSize>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn centauri_channel)]
	pub type CentauriChannel<T: Config> = StorageValue<_, u64, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn memo)]
	pub type ForwardMemo<T: Config> =
		StorageValue<_, BoundedVec<u8, T::MaxStringSize>, OptionQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T>
	where
		T: Send + Sync,
		AccountId32: From<<T as frame_system::Config>::AccountId>,
		u32: From<<T as frame_system::Config>::BlockNumber>,
	{
		// on every period, for every ibc asset from CentauriChannel - Disallowed Assets + Allowed Asset
		// if 20 percent of new treasury balance for a token - old balance of token >= token's ED
		// this amount is transferred to the pallets account
		// from this account tokens are sent to osmosis. We have intermediate account so that tokens can be
		// resend by trigger and transfer failure wont affect revenue calculations
		fn on_initialize(now: T::BlockNumber) -> Weight {
			if Self::period() != sp_runtime::traits::Zero::zero() &&
				now % Self::period() == Zero::zero()
			{
				let mut count: u32 = 0;
				Self::deposit_event(Event::<T>::RevenueCalcutions);
				Self::get_ibc_assets().iter().for_each(|asset_id| {
					let percentage = Perbill::from_rational(200_u32, 1000_u32);
					let new_balance = T::Assets::reducible_balance(
						asset_id.clone(),
						&Self::treasury_account_id(),
						Preservation::Expendable,
						frame_support::traits::tokens::Fortitude::Polite,
					);
					let old_balance = TokenPrevPeriodBalance::<T>::get(asset_id);
					let asset_ed_res = T::AssetsRegistry::existential_deposit(asset_id.clone());
					if let Ok(asset_ed) = asset_ed_res {
						if new_balance > old_balance &&
							percentage * (new_balance - old_balance) >= asset_ed
						{
							let amount = percentage * (new_balance - old_balance);
							match T::Assets::transfer(
								asset_id.clone(),
								&Self::treasury_account_id(),
								&Self::pallet_account_id(),
								percentage * (new_balance - old_balance),
								Preservation::Expendable,
							) {
								Ok(_) => Self::deposit_event(Event::<T>::TransferSuccess {
									asset_id: asset_id.clone(),
									amount,
								}),
								Err(_) => Self::deposit_event(Event::<T>::TransferFail {
									asset_id: asset_id.clone(),
									amount,
								}),
							};
							TokenPrevPeriodBalance::<T>::insert(asset_id, new_balance - amount);
						}
					}
					Self::deposit_event(Event::<T>::SkipAsset { asset_id: asset_id.clone() });
				});
				if let Err(_) = Self::transfer_from_intermediate() {
					Self::deposit_event(Event::<T>::IntermediateTransferFail);
				}

				// T::WeightInfo::on_initialize(count)
				Weight::zero()
			} else {
				Weight::zero()
			}
		}
	}

	// The pallet's dispatchable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: Send + Sync,
		AccountId32: From<<T as frame_system::Config>::AccountId>,
		u32: From<<T as frame_system::Config>::BlockNumber>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(100_000)]
		pub fn set_period(origin: OriginFor<T>, period: T::BlockNumber) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			// stop sharing
			if period == Zero::zero() {
				TokenPrevPeriodBalance::<T>::remove_all(None);
			}
			// save current values
			if Self::period() == Zero::zero() {
				Self::get_ibc_assets().iter().for_each(|asset_id| {
					TokenPrevPeriodBalance::<T>::insert(
						asset_id,
						T::Assets::reducible_balance(
							asset_id.clone(),
							&Self::treasury_account_id(),
							Preservation::Expendable,
							frame_support::traits::tokens::Fortitude::Polite,
						),
					);
				})
			}
			Period::<T>::set(period);
			Self::deposit_event(Event::<T>::PeriodSet { period });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(100_000)]
		pub fn set_memo(
			origin: OriginFor<T>,
			memo: BoundedVec<u8, T::MaxStringSize>,
		) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			ForwardMemo::<T>::set(Some(memo.clone()));
			Self::deposit_event(Event::<T>::MemoSet { memo });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(100_000)]
		pub fn trigger_transfer(origin: OriginFor<T>) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			Self::transfer_from_intermediate()?;
			Self::deposit_event(Event::<T>::TransferTriggered);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(100_000)]
		pub fn set_allowed(origin: OriginFor<T>, assets: Vec<AssetIdOf<T>>) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			AllowedAssets::<T>::drain().for_each(|(asset, _val)|{
				TokenPrevPeriodBalance::<T>::remove(asset);
			});
			
			assets.iter().for_each(|asset| {
				AllowedAssets::<T>::insert(asset, ());
				TokenPrevPeriodBalance::<T>::insert(
					asset,
					T::Assets::reducible_balance(
						asset.clone(),
						&Self::treasury_account_id(),
						Preservation::Expendable,
						frame_support::traits::tokens::Fortitude::Polite,
					),
				);
			});
			Self::deposit_event(Event::<T>::SetAllowed);
			Ok(())
		}

		// add a new allowed asset or reset TokenPrevPeriodBalance to the current balance for asset 
		#[pallet::call_index(4)]
		#[pallet::weight(100_000)]
		pub fn add_allowed(origin: OriginFor<T>, asset: AssetIdOf<T>) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			AllowedAssets::<T>::insert(&asset, ());
			TokenPrevPeriodBalance::<T>::insert(
				&asset,
				T::Assets::reducible_balance(
					asset.clone(),
					&Self::treasury_account_id(),
					Preservation::Expendable,
					frame_support::traits::tokens::Fortitude::Polite,
				),
			);
			Self::deposit_event(Event::<T>::AddAllowed);
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(100_000)]
		pub fn remove_allowed(origin: OriginFor<T>, asset: AssetIdOf<T>) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			AllowedAssets::<T>::remove(&asset);
			TokenPrevPeriodBalance::<T>::remove(&asset);
			Self::deposit_event(Event::<T>::RemoveAllowed);
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(100_000)]
		pub fn set_disallowed(origin: OriginFor<T>, assets: Vec<AssetIdOf<T>>) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			DisallowedAssets::<T>::remove_all(None);
			assets.iter().for_each(|asset| DisallowedAssets::<T>::insert(asset, ()));
			Self::deposit_event(Event::<T>::SetDisallowed);
			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(100_000)]
		pub fn add_disallowed(origin: OriginFor<T>, asset: AssetIdOf<T>) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			DisallowedAssets::<T>::insert(asset, ());
			Self::deposit_event(Event::<T>::AddDisallowed);
			Ok(())
		}

		#[pallet::call_index(8)]
		#[pallet::weight(100_000)]
		pub fn remove_disallowed(origin: OriginFor<T>, asset: AssetIdOf<T>) -> DispatchResult {
			T::Admin::ensure_origin(origin)?;
			DisallowedAssets::<T>::remove(asset);
			Self::deposit_event(Event::<T>::RemoveDisallowed);
			Ok(())
		}
	}

	impl<T: Config> Pallet<T>
	where
		T: Send + Sync,
		AccountId32: From<<T as frame_system::Config>::AccountId>,
		u32: From<<T as frame_system::Config>::BlockNumber>,
	{
		pub fn treasury_account_id() -> T::AccountId {
			T::FromPalletId::get().into_account_truncating()
		}

		pub fn pallet_account_id() -> T::AccountId {
			T::IntermediatePalletId::get().into_account_truncating()
		}

		fn transfer_from_intermediate() -> DispatchResult {
			if let Some(channel_id) = Self::centauri_channel() {
				if let Some(centauri_address) = Self::centauri_address() {
					let transfer_params: TransferParams<T::AccountId> =
						TransferParams::<T::AccountId> {
							to: MultiAddress::<T::AccountId>::Raw(centauri_address.into()),
							source_channel: channel_id,
							timeout: IbcTimeout::Offset {
								timestamp: Some(6_000_000_000_000),
								height: Some(1000),
							},
						};
					let memo = match Self::memo().clone() {
						Some(m) => match String::from_utf8(m.into()) {
							Ok(m) => <T as pallet_ibc::Config>::MemoMessage::from_str(&m).ok(),
							_ => None,
						},
						_ => None,
					};
					Self::get_ibc_assets().into_iter().for_each(|asset_id| {
						let result = pallet_ibc::Pallet::<T>::transfer(
							frame_system::RawOrigin::Signed(Self::pallet_account_id()).into(),
							transfer_params.clone(),
							asset_id.clone(),
							T::Assets::reducible_balance(
								asset_id.clone(),
								&Self::pallet_account_id(),
								Preservation::Expendable,
								frame_support::traits::tokens::Fortitude::Polite,
							),
							memo.clone(),
						);
						if let Err(e) = result {
							Self::deposit_event(Event::<T>::TransferFailed { asset_id });
						}
					});
					Ok(())
				} else {
					return Err(Error::<T>::CentauriAddressNotSet.into())
				}
			} else {
				return Err(Error::<T>::ChannelNotSet.into())
			}
		}

		fn get_ibc_assets() -> Vec<AssetIdOf<T>> {
			let allowed = AllowedAssets::<T>::iter_keys().collect::<Vec<AssetIdOf<T>>>();
			// TODO: send all ibc tokens to specified address
			// let disallowed = DisallowedAssets::<T>::iter_keys().collect::<Vec<AssetIdOf<T>>>();
			// <T::AssetsRegistry as RemoteAssetRegistryInspect>::get_foreign_assets_list()
			// 	.iter()
			// 	.for_each(|asset| {
			// 		if let Ok(ForeignAssetId::IbcIcs20(denom)) =
			// 			ForeignAssetId::decode(&mut asset.foreign_id.clone().encode().as_slice())
			// 		{
			// 			if denom
			// 				.0
			// 				.trace_path
			// 				.starts_with(&TracePrefix::new(PortId::transfer(), ChannelId::new(15))) &&
			// 				!disallowed.contains(&asset.id)
			// 			{
			// 				allowed.push(asset.id.clone())
			// 			}
			// 		}
			// 	});
			allowed
		}
	}
}
