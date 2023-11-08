// Copyright 2021 Composable Finance Developer.
// This file is part of Composable Finance.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Common XCM Helper pallet
//!
//! ## Overview
//! This pallet should be in charge of everything XCM related including callbacks and sending XCM
//! calls.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::{DispatchResult, GetDispatchInfo},
	pallet_prelude::*,
	traits::fungibles::{Inspect, Mutate},
	transactional, PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::{AccountIdConversion, BlockNumberProvider, Convert, StaticLookup};
use sp_std::{boxed::Box, prelude::*, vec, vec::Vec};
use xcm::{latest::prelude::*, DoubleEncoded};

pub use pallet::*;
pub mod ump;

use crate::ump::*;

use frame_support::traits::tokens::{Fortitude, Precision, Preservation};

// mod benchmarking;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod weights;
pub use weights::WeightInfo;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CallIdOf<T> = <T as pallet_xcm::Config>::RuntimeCall;
pub type AssetIdOf<T> =
	<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
pub type BalanceOf<T> =
	<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

pub type CurrencyId = primitives::currency::CurrencyId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
	use sp_runtime::traits::{Convert, Zero};

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_xcm::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Assets for deposit/withdraw assets to/from crowdloan account
		type Assets: Inspect<AccountIdOf<Self>, AssetId = CurrencyId, Balance = Balance>
			+ Mutate<AccountIdOf<Self>, AssetId = CurrencyId, Balance = Balance>;

		/// XCM message sender
		type XcmSender: SendXcm;

		/// Relay network
		#[pallet::constant]
		type RelayNetwork: Get<NetworkId>;

		/// Pallet account for collecting xcm fees
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Account on relaychain for receiving refunded fees
		#[pallet::constant]
		type RefundLocation: Get<Self::AccountId>;

		/// Convert `T::AccountId` to `MultiLocation`.
		type AccountIdToMultiLocation: Convert<Self::AccountId, MultiLocation>;

		/// Notify call timeout
		#[pallet::constant]
		type NotifyTimeout: Get<BlockNumberFor<Self>>;

		/// The block number provider
		type BlockNumberProvider: BlockNumberProvider<BlockNumber = BlockNumberFor<Self>>;

		/// The origin which can update reserve_factor, xcm_fees etc
		type UpdateOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;

		/// Weight information
		type WeightInfo: WeightInfo;

		/// Relay currency
		#[pallet::constant]
		type RelayCurrency: Get<AssetIdOf<Self>>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Xcm fee and weight updated
		XcmWeightFeeUpdated(XcmWeightFeeMisc<Weight, BalanceOf<T>>),
	}

	#[pallet::storage]
	#[pallet::getter(fn xcm_weight_fee)]
	#[allow(clippy::disallowed_types)]
	pub type XcmWeightFee<T: Config> =
		StorageMap<_, Twox64Concat, XcmCall, XcmWeightFeeMisc<Weight, BalanceOf<T>>, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::error]
	pub enum Error<T> {
		/// `MultiLocation` value ascend more parents than known ancestors of local location.
		MultiLocationNotInvertible,
		/// XcmWeightMisc cannot have zero value
		ZeroXcmWeightMisc,
		/// Xcm fees cannot be zero
		ZeroXcmFees,
		/// Insufficient xcm fees
		InsufficientXcmFees,
		/// The message and destination was recognized as being reachable but
		/// the operation could not be completed.
		SendFailure,
		/// Can not convert account success
		ConvertAccountError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Update xcm fees amount to be used in xcm.Withdraw message
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::update_xcm_weight_fee())]
		#[transactional]
		pub fn update_xcm_weight_fee(
			origin: OriginFor<T>,
			xcm_call: XcmCall,
			xcm_weight_fee_misc: XcmWeightFeeMisc<Weight, BalanceOf<T>>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;

			ensure!(!xcm_weight_fee_misc.fee.is_zero(), Error::<T>::ZeroXcmFees);
			ensure!(!xcm_weight_fee_misc.weight.is_zero(), Error::<T>::ZeroXcmWeightMisc);

			XcmWeightFee::<T>::mutate(xcm_call, |v| *v = xcm_weight_fee_misc);
			Self::deposit_event(Event::<T>::XcmWeightFeeUpdated(xcm_weight_fee_misc));
			Ok(())
		}
	}
}

pub trait XcmHelper<T: pallet_xcm::Config, Balance, TAccountId> {
	fn add_xcm_fees(payer: &TAccountId, amount: Balance) -> DispatchResult;

	fn do_ump_transact(
		call: DoubleEncoded<()>,
		weight: Weight,
		beneficiary: MultiLocation,
		fees: Balance,
	) -> Result<Xcm<()>, DispatchError>;

	fn do_bond(
		value: Balance,
		payee: RewardDestination<TAccountId>,
		stash: TAccountId,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError>;

	fn do_bond_extra(
		value: Balance,
		stash: TAccountId,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError>;

	fn do_unbond(
		value: Balance,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError>;

	fn do_rebond(
		value: Balance,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError>;

	fn do_withdraw_unbonded(
		num_slashing_spans: u32,
		para_account_id: TAccountId,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError>;

	fn do_nominate(
		targets: Vec<TAccountId>,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError>;
}

impl<T: Config> Pallet<T> {
	pub fn account_id() -> AccountIdOf<T> {
		T::PalletId::get().into_account_truncating()
	}

	pub fn refund_location() -> MultiLocation {
		T::AccountIdToMultiLocation::convert(T::RefundLocation::get())
	}

	pub fn report_outcome_notify(
		message: &mut Xcm<()>,
		responder: impl Into<MultiLocation>,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
		timeout: BlockNumberFor<T>,
	) -> Result<QueryId, DispatchError> {
		let responder = responder.into();
		let destination = <T as pallet_xcm::Config>::UniversalLocation::get()
			.invert_target(&responder)
			.map_err(|()| Error::<T>::MultiLocationNotInvertible)?;
		let notify: <T as pallet_xcm::Config>::RuntimeCall = notify.into();
		let max_weight = notify.get_dispatch_info().weight;
		let query_id = pallet_xcm::Pallet::<T>::new_notify_query(responder, notify, timeout, Here);
		let report_error =
			Xcm(vec![ReportError(QueryResponseInfo { destination, query_id, max_weight })]);
		// Prepend SetAppendix(Xcm(vec![ReportError])) wont be able to pass barrier check
		// so we need to insert it after Withdraw, BuyExecution
		message.0.insert(2, SetAppendix(report_error));
		Ok(query_id)
	}

	// Since xcm v3 doesn't support utility.batch_all
	// instead, here append one more transact msg
	//
	// NOTE: index here is 3,
	// must append before 'report_outcome_notify' that index is 2
	pub fn append_transact(message: &mut Xcm<()>, call: DoubleEncoded<()>, weight: Weight) {
		message.0.insert(
			3,
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: weight,
				call,
			},
		);
	}
}

impl<T: Config> XcmHelper<T, BalanceOf<T>, AccountIdOf<T>> for Pallet<T> {
	fn add_xcm_fees(payer: &AccountIdOf<T>, amount: BalanceOf<T>) -> DispatchResult {
		let keep_alive = false;
		let keep_alive = if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
		T::Assets::transfer(
			T::RelayCurrency::get(),
			payer,
			&Self::account_id(),
			amount,
			keep_alive,
		)?;
		Ok(())
	}

	fn do_ump_transact(
		call: DoubleEncoded<()>,
		weight: Weight,
		beneficiary: MultiLocation,
		fees: BalanceOf<T>,
	) -> Result<Xcm<()>, DispatchError> {
		let asset: MultiAsset = (MultiLocation::here(), fees).into();
		T::Assets::burn_from(
			T::RelayCurrency::get(),
			&Self::account_id(),
			fees,
			Precision::Exact,
			Fortitude::Polite,
		)
		.map_err(|_| Error::<T>::InsufficientXcmFees)?;

		Ok(Xcm(vec![
			WithdrawAsset(MultiAssets::from(asset.clone())),
			BuyExecution { fees: asset.clone(), weight_limit: Unlimited },
			Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: weight,
				call,
			},
			RefundSurplus,
			DepositAsset { assets: asset.into(), beneficiary },
		]))
	}

	fn do_bond(
		value: BalanceOf<T>,
		payee: RewardDestination<AccountIdOf<T>>,
		stash: AccountIdOf<T>,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError> {
		let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Bond);
		Ok(switch_relay!({
			let call = RelaychainCall::<T>::Balances(BalancesCall::TransferKeepAlive(
				BalancesTransferKeepAliveCall { dest: T::Lookup::unlookup(stash), value },
			));
			let mut msg = Self::do_ump_transact(
				call.encode().into(),
				xcm_weight_fee_misc.weight,
				Self::refund_location(),
				xcm_weight_fee_misc.fee,
			)?;
			let call = RelaychainCall::<T>::Utility(Box::new(UtilityCall::AsDerivative(
				UtilityAsDerivativeCall {
					index,
					call: RelaychainCall::Staking::<T>(StakingCall::Bond(StakingBondCall {
						// controller: T::Lookup::unlookup(controller),
						value,
						payee,
					})),
				},
			)));

			Self::append_transact(&mut msg, call.encode().into(), xcm_weight_fee_misc.weight);

			let query_id = Self::report_outcome_notify(
				&mut msg,
				MultiLocation::parent(),
				notify,
				T::NotifyTimeout::get(),
			)?;

			if let Err(_err) = send_xcm::<T::XcmSender>(MultiLocation::parent(), msg) {
				return Err(Error::<T>::SendFailure.into());
			}

			query_id
		}))
	}

	fn do_bond_extra(
		value: BalanceOf<T>,
		stash: AccountIdOf<T>,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError> {
		let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::BondExtra);
		Ok(switch_relay!({
			let call = RelaychainCall::<T>::Balances(BalancesCall::TransferKeepAlive(
				BalancesTransferKeepAliveCall { dest: T::Lookup::unlookup(stash), value },
			));

			let mut msg = Self::do_ump_transact(
				call.encode().into(),
				xcm_weight_fee_misc.weight,
				Self::refund_location(),
				xcm_weight_fee_misc.fee,
			)?;

			let call = RelaychainCall::<T>::Utility(Box::new(UtilityCall::AsDerivative(
				UtilityAsDerivativeCall {
					index,
					call: RelaychainCall::Staking::<T>(StakingCall::BondExtra(
						StakingBondExtraCall { value },
					)),
				},
			)));
			Self::append_transact(&mut msg, call.encode().into(), xcm_weight_fee_misc.weight);

			let query_id = Self::report_outcome_notify(
				&mut msg,
				MultiLocation::parent(),
				notify,
				T::NotifyTimeout::get(),
			)?;

			if let Err(_err) = send_xcm::<T::XcmSender>(MultiLocation::parent(), msg) {
				return Err(Error::<T>::SendFailure.into());
			}

			query_id
		}))
	}

	fn do_unbond(
		value: BalanceOf<T>,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError> {
		let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Unbond);
		Ok(switch_relay!({
			let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
				UtilityAsDerivativeCall {
					index,
					call: RelaychainCall::Staking::<T>(StakingCall::Unbond(StakingUnbondCall {
						value,
					})),
				},
			)));

			let mut msg = Self::do_ump_transact(
				call.encode().into(),
				xcm_weight_fee_misc.weight,
				Self::refund_location(),
				xcm_weight_fee_misc.fee,
			)?;

			let query_id = Self::report_outcome_notify(
				&mut msg,
				MultiLocation::parent(),
				notify,
				T::NotifyTimeout::get(),
			)?;

			if let Err(_err) = send_xcm::<T::XcmSender>(MultiLocation::parent(), msg) {
				return Err(Error::<T>::SendFailure.into());
			}

			query_id
		}))
	}

	fn do_rebond(
		value: BalanceOf<T>,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError> {
		let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Rebond);
		Ok(switch_relay!({
			let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
				UtilityAsDerivativeCall {
					index,
					call: RelaychainCall::Staking::<T>(StakingCall::Rebond(StakingRebondCall {
						value,
					})),
				},
			)));

			let mut msg = Self::do_ump_transact(
				call.encode().into(),
				xcm_weight_fee_misc.weight,
				Self::refund_location(),
				xcm_weight_fee_misc.fee,
			)?;

			let query_id = Self::report_outcome_notify(
				&mut msg,
				MultiLocation::parent(),
				notify,
				T::NotifyTimeout::get(),
			)?;

			if let Err(_err) = send_xcm::<T::XcmSender>(MultiLocation::parent(), msg) {
				return Err(Error::<T>::SendFailure.into());
			}

			query_id
		}))
	}

	fn do_withdraw_unbonded(
		num_slashing_spans: u32,
		para_account_id: AccountIdOf<T>,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError> {
		let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::WithdrawUnbonded);
		Ok(switch_relay!({
			let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
				UtilityAsDerivativeCall {
					index,
					call: RelaychainCall::Staking::<T>(StakingCall::WithdrawUnbonded(
						StakingWithdrawUnbondedCall { num_slashing_spans },
					)),
				},
			)));

			let mut msg = Self::do_ump_transact(
				call.encode().into(),
				xcm_weight_fee_misc.weight,
				Self::refund_location(),
				xcm_weight_fee_misc.fee,
			)?;

			let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
				UtilityAsDerivativeCall {
					index,
					call: RelaychainCall::Balances::<T>(BalancesCall::TransferAll(
						BalancesTransferAllCall {
							dest: T::Lookup::unlookup(para_account_id),
							keep_alive: true,
						},
					)),
				},
			)));
			Self::append_transact(&mut msg, call.encode().into(), xcm_weight_fee_misc.weight);

			let query_id = Self::report_outcome_notify(
				&mut msg,
				MultiLocation::parent(),
				notify,
				T::NotifyTimeout::get(),
			)?;

			if let Err(_err) = send_xcm::<T::XcmSender>(MultiLocation::parent(), msg) {
				return Err(Error::<T>::SendFailure.into());
			}

			query_id
		}))
	}

	fn do_nominate(
		targets: Vec<AccountIdOf<T>>,
		index: u16,
		notify: impl Into<<T as pallet_xcm::Config>::RuntimeCall>,
	) -> Result<QueryId, DispatchError> {
		let targets_source = targets.into_iter().map(T::Lookup::unlookup).collect();
		let xcm_weight_fee_misc = Self::xcm_weight_fee(XcmCall::Nominate);
		Ok(switch_relay!({
			let call = RelaychainCall::Utility(Box::new(UtilityCall::AsDerivative(
				UtilityAsDerivativeCall {
					index,
					call: RelaychainCall::Staking::<T>(StakingCall::Nominate(
						StakingNominateCall { targets: targets_source },
					)),
				},
			)));

			let mut msg = Self::do_ump_transact(
				call.encode().into(),
				xcm_weight_fee_misc.weight,
				Self::refund_location(),
				xcm_weight_fee_misc.fee,
			)?;

			let query_id = Self::report_outcome_notify(
				&mut msg,
				MultiLocation::parent(),
				notify,
				T::NotifyTimeout::get(),
			)?;

			if let Err(_err) = send_xcm::<T::XcmSender>(MultiLocation::parent(), msg) {
				return Err(Error::<T>::SendFailure.into());
			}

			query_id
		}))
	}
}
