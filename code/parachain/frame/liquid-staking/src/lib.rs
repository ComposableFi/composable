// Copyright 2021 Composable Finance Developer.
// This file is part of Composable Finance

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Liquid staking pallet
//!
//! ## Overview
//!
//! This pallet manages the NPoS operations for relay chain asset.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{tokens::Balance as BalanceT, Get};
use sp_runtime::{
	traits::{One, Zero},
	FixedPointNumber, FixedPointOperand,
};

pub use pallet::*;
use pallet_xcm_helper::ump::RewardDestination;

// mod benchmarking;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod distribution;
// pub mod migrations;
pub mod types;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
		ensure,
		error::BadOrigin,
		log,
		pallet_prelude::*,
		require_transactional,
		storage::{storage_prefix, with_transaction},
		traits::{
			fungibles::{Inspect, Mutate},
			tokens::{Fortitude, Precision, Preservation},
			IsType, SortedMembers,
		},
		transactional, PalletId, StorageHasher,
	};
	use frame_system::{
		ensure_signed,
		pallet_prelude::{BlockNumberFor, OriginFor},
	};
	use pallet_xcm::ensure_response;
	use parity_scale_codec::Encode;
	use polkadot_parachain::primitives::Id as ParaId;
	use sp_runtime::{
		traits::{
			AccountIdConversion, BlakeTwo256, BlockNumberProvider, CheckedDiv, CheckedSub,
			Saturating, StaticLookup,
		},
		ArithmeticError, FixedPointNumber, TransactionOutcome,
	};
	use sp_std::{borrow::Borrow, boxed::Box, result::Result, vec::Vec};
	use sp_trie::StorageProof;
	use xcm::latest::prelude::*;

	use crate::distribution::*;

	use primitives::currency::CurrencyId;
	pub type Balance = u128;

	use pallet_xcm_helper::XcmHelper;

	use super::{types::*, *};

	pub const MAX_UNLOCKING_CHUNKS: usize = 32;

	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type AssetIdOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
	pub type BalanceOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	/// Utility type for managing upgrades/migrations.
	#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub enum Versions {
		V1,
		V2,
		V3,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_utility::Config + pallet_xcm::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type RuntimeOrigin: IsType<<Self as frame_system::Config>::RuntimeOrigin>
			+ Into<Result<pallet_xcm::Origin, <Self as Config>::RuntimeOrigin>>;

		type RuntimeCall: IsType<<Self as pallet_xcm::Config>::RuntimeCall> + From<Call<Self>>;

		/// Assets for deposit/withdraw assets to/from pallet account
		type Assets: Mutate<Self::AccountId, AssetId = CurrencyId, Balance = Balance>
			+ Inspect<Self::AccountId, AssetId = CurrencyId, Balance = Balance>;

		/// The origin which can do operation on relaychain using parachain's sovereign account
		type RelayOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;

		/// The origin which can update liquid currency, staking currency and other parameters
		type UpdateOrigin: EnsureOrigin<<Self as frame_system::Config>::RuntimeOrigin>;

		/// Approved accouts which can call `withdraw_unbonded` and `settlement`
		type Members: SortedMembers<Self::AccountId>;

		/// The pallet id of liquid staking, keeps all the staking assets
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Returns the parachain ID we are running with.
		#[pallet::constant]
		type SelfParaId: Get<ParaId>;

		/// Derivative index list
		#[pallet::constant]
		type DerivativeIndexList: Get<Vec<DerivativeIndex>>;

		/// Xcm fees
		#[pallet::constant]
		type XcmFees: Get<BalanceOf<Self>>;

		/// MatchingPool fast unstake fee
		#[pallet::constant]
		type MatchingPoolFastUnstakeFee: Get<Rate>;

		/// Staking currency
		#[pallet::constant]
		type StakingCurrency: Get<AssetIdOf<Self>>;

		/// Liquid currency
		#[pallet::constant]
		type LiquidCurrency: Get<AssetIdOf<Self>>;

		/// Minimum stake amount
		#[pallet::constant]
		type MinStake: Get<BalanceOf<Self>>;

		/// Minimum unstake amount
		#[pallet::constant]
		type MinUnstake: Get<BalanceOf<Self>>;

		/// Weight information
		type WeightInfo: WeightInfo;

		/// Number of unbond indexes for unlocking.
		#[pallet::constant]
		type BondingDuration: Get<EraIndex>;

		/// The minimum active bond to become and maintain the role of a nominator.
		#[pallet::constant]
		type MinNominatorBond: Get<BalanceOf<Self>>;

		/// Number of blocknumbers that each period contains.
		/// SessionsPerEra * EpochDuration / MILLISECS_PER_BLOCK
		#[pallet::constant]
		type EraLength: Get<BlockNumberFor<Self>>;

		#[pallet::constant]
		type NumSlashingSpans: Get<u32>;

		/// The relay's validation data provider
		type RelayChainValidationDataProvider: ValidationDataProvider
			+ BlockNumberProvider<BlockNumber = BlockNumberFor<Self>>;

		/// To expose XCM helper functions
		type XCM: XcmHelper<Self, BalanceOf<Self>, Self::AccountId>;

		/// Current strategy for distributing assets to multi-accounts
		type DistributionStrategy: DistributionStrategy<BalanceOf<Self>>;

		/// Number of blocknumbers that do_matching after each era updated.
		/// Need to do_bond before relaychain store npos solution
		#[pallet::constant]
		type ElectionSolutionStoredOffset: Get<BlockNumberFor<Self>>;

		/// Who/where to send the protocol fees
		#[pallet::constant]
		type ProtocolFeeReceiver: Get<Self::AccountId>;

		/// Decimal provider.
		type Decimal: DecimalProvider<CurrencyId>;

		/// The asset id for native currency.
		#[pallet::constant]
		type NativeCurrency: Get<AssetIdOf<Self>>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The assets get staked successfully
		Staked(T::AccountId, BalanceOf<T>),
		/// The derivative get unstaked successfully
		Unstaked(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Staking ledger updated
		StakingLedgerUpdated(DerivativeIndex, StakingLedger<T::AccountId, BalanceOf<T>>),
		/// Sent staking.bond call to relaychain
		Bonding(DerivativeIndex, T::AccountId, BalanceOf<T>, RewardDestination<T::AccountId>),
		/// Sent staking.bond_extra call to relaychain
		BondingExtra(DerivativeIndex, BalanceOf<T>),
		/// Sent staking.unbond call to relaychain
		Unbonding(DerivativeIndex, BalanceOf<T>),
		/// Sent staking.rebond call to relaychain
		Rebonding(DerivativeIndex, BalanceOf<T>),
		/// Sent staking.withdraw_unbonded call to relaychain
		WithdrawingUnbonded(DerivativeIndex, u32),
		/// Sent staking.nominate call to relaychain
		Nominating(DerivativeIndex, Vec<T::AccountId>),
		/// Staking ledger's cap was updated
		StakingLedgerCapUpdated(BalanceOf<T>),
		/// Reserve_factor was updated
		ReserveFactorUpdated(Ratio),
		/// Exchange rate was updated
		ExchangeRateUpdated(Rate),
		/// Notification received
		/// [multi_location, query_id, res]
		NotificationReceived(Box<MultiLocation>, QueryId, Option<(u32, XcmError)>),
		/// Claim user's unbonded staking assets
		/// [account_id, amount]
		ClaimedFor(T::AccountId, BalanceOf<T>),
		/// New era
		/// [era_index]
		NewEra(EraIndex),
		/// Matching stakes & unstakes for optimizing operations to be done
		/// on relay chain
		/// [bond_amount, rebond_amount, unbond_amount]
		Matching(BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
		/// Event emitted when the reserves are reduced
		/// [receiver, reduced_amount]
		ReservesReduced(T::AccountId, BalanceOf<T>),
		/// Unstake cancelled
		/// [account_id, amount, liquid_amount]
		UnstakeCancelled(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Commission rate was updated
		CommissionRateUpdated(Rate),
		/// Fast Unstake Matched
		/// [unstaker, received_staking_amount, matched_liquid_amount, fee_in_liquid_currency]
		FastUnstakeMatched(T::AccountId, BalanceOf<T>, BalanceOf<T>, BalanceOf<T>),
		/// Incentive amount was updated
		IncentiveUpdated(BalanceOf<T>),
		/// Not the ideal staking ledger
		NonIdealStakingLedger(DerivativeIndex),

		RelaychainStorageProofKey(DerivativeIndex, Vec<u8>, T::AccountId),
		SetStakingLedgerTry {
			origin: T::AccountId,
			derivative_index: DerivativeIndex,
			staking_ledger: StakingLedger<T::AccountId, BalanceOf<T>>,
			proof: Vec<Vec<u8>>,
		},

		OnInitializeHook {
			relay_block_number: BlockNumberFor<T>,
			era: u32,
		},

		SetMembers {
			members: Vec<T::AccountId>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Exchange rate is invalid.
		InvalidExchangeRate,
		/// The stake was below the minimum, `MinStake`.
		StakeTooSmall,
		/// The unstake was below the minimum, `MinUnstake`.
		UnstakeTooSmall,
		/// Invalid liquid currency
		InvalidLiquidCurrency,
		/// Invalid staking currency
		InvalidStakingCurrency,
		/// Invalid derivative index
		InvalidDerivativeIndex,
		/// Invalid staking ledger
		InvalidStakingLedger,
		/// Exceeded liquid currency's market cap
		CapExceeded,
		/// Invalid market cap
		InvalidCap,
		/// The factor should be bigger than 0% and smaller than 100%
		InvalidFactor,
		/// Nothing to claim yet
		NothingToClaim,
		/// Stash wasn't bonded yet
		NotBonded,
		/// Stash is already bonded.
		AlreadyBonded,
		/// Can not schedule more unlock chunks.
		NoMoreChunks,
		/// Staking ledger is locked due to mutation in notification_received
		StakingLedgerLocked,
		/// Not withdrawn unbonded yet
		NotWithdrawn,
		/// Cannot have a nominator role with value less than the minimum defined by
		/// `MinNominatorBond`
		InsufficientBond,
		/// The merkle proof is invalid
		InvalidProof,
		/// No unlocking items
		NoUnlockings,
		/// Invalid commission rate
		InvalidCommissionRate,

		InvalidOrigin,
	}

	/// The exchange rate between relaychain native asset and the voucher.
	#[pallet::storage]
	#[pallet::getter(fn exchange_rate)]
	#[allow(clippy::disallowed_types)]
	pub type ExchangeRate<T: Config> = StorageValue<_, Rate, ValueQuery>;

	/// The commission rate charge for staking total rewards.
	#[pallet::storage]
	#[pallet::getter(fn commission_rate)]
	#[allow(clippy::disallowed_types)]
	pub type CommissionRate<T: Config> = StorageValue<_, Rate, ValueQuery>;

	/// ValidationData of previous block
	///
	/// This is needed since validation data from cumulus_pallet_parachain_system
	/// will be updated in set_validation_data Inherent which happens before external
	/// extrinsics
	#[pallet::storage]
	#[pallet::getter(fn validation_data)]
	pub type ValidationData<T: Config> = StorageValue<_, PersistedValidationData, OptionQuery>;

	/// Fraction of reward currently set aside for reserves.
	#[pallet::storage]
	#[pallet::getter(fn reserve_factor)]
	#[allow(clippy::disallowed_types)]
	pub type ReserveFactor<T: Config> = StorageValue<_, Ratio, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_reserves)]
	#[allow(clippy::disallowed_types)]
	pub type TotalReserves<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Store total stake amount and unstake amount in each era,
	/// And will update when stake/unstake occurred.
	#[pallet::storage]
	#[pallet::getter(fn matching_pool)]
	#[allow(clippy::disallowed_types)]
	pub type MatchingPool<T: Config> = StorageValue<_, MatchingLedger<BalanceOf<T>>, ValueQuery>;

	/// Staking ledger's cap
	#[pallet::storage]
	#[pallet::getter(fn staking_ledger_cap)]
	#[allow(clippy::disallowed_types)]
	pub type StakingLedgerCap<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Flying & failed xcm requests
	#[pallet::storage]
	#[pallet::getter(fn xcm_request)]
	pub type XcmRequests<T> = StorageMap<_, Blake2_128Concat, QueryId, XcmRequest<T>, OptionQuery>;

	/// Users' fast unstake requests in liquid currency
	#[pallet::storage]
	#[pallet::getter(fn fast_unstake_requests)]
	#[allow(clippy::disallowed_types)]
	pub type FastUnstakeRequests<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

	/// Current era index
	/// Users can come to claim their unbonded staking assets back once this value arrived
	/// at certain height decided by `BondingDuration` and `EraLength`
	#[pallet::storage]
	#[pallet::getter(fn current_era)]
	#[allow(clippy::disallowed_types)]
	pub type CurrentEra<T: Config> = StorageValue<_, EraIndex, ValueQuery>;

	/// Current era's start relaychain block
	#[pallet::storage]
	#[pallet::getter(fn era_start_block)]
	#[allow(clippy::disallowed_types)]
	pub type EraStartBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

	/// Unbonding requests to be handled after arriving at target era
	#[pallet::storage]
	#[pallet::getter(fn unlockings)]
	pub type Unlockings<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<UnlockChunk<BalanceOf<T>>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn members)]
	#[allow(clippy::disallowed_types)]
	pub type Members<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	/// Platform's staking ledgers
	#[pallet::storage]
	#[pallet::getter(fn staking_ledger)]
	pub type StakingLedgers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		DerivativeIndex,
		StakingLedger<T::AccountId, BalanceOf<T>>,
		OptionQuery,
	>;

	/// Set to true if staking ledger has been modified in this block
	#[pallet::storage]
	#[pallet::getter(fn is_updated)]
	#[allow(clippy::disallowed_types)]
	pub type IsUpdated<T: Config> = StorageMap<_, Twox64Concat, DerivativeIndex, bool, ValueQuery>;

	/// DefaultVersion is using for initialize the StorageVersion
	#[pallet::type_value]
	pub(super) fn DefaultVersion() -> Versions {
		Versions::V2
	}

	/// Storage version of the pallet.
	#[pallet::storage]
	#[allow(clippy::disallowed_types)]
	pub(crate) type StorageVersion<T: Config> =
		StorageValue<_, Versions, ValueQuery, DefaultVersion>;

	/// Set to true if already do matching in current era
	/// clear after arriving at next era
	#[pallet::storage]
	#[pallet::getter(fn is_matched)]
	#[allow(clippy::disallowed_types)]
	pub type IsMatched<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Incentive for users who successfully update era/ledger
	#[pallet::storage]
	#[pallet::getter(fn incentive)]
	#[allow(clippy::disallowed_types)]
	pub type Incentive<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[derive(Default)]
	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub exchange_rate: Rate,
		pub reserve_factor: Ratio,
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			ExchangeRate::<T>::put(self.exchange_rate);
			ReserveFactor::<T>::put(self.reserve_factor);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Put assets under staking, the native assets will be transferred to the account
		/// owned by the pallet, user receive derivative in return, such derivative can be
		/// further used as collateral for lending.
		///
		/// - `amount`: the amount of staking assets
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::stake())]
		#[transactional]
		pub fn stake(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(amount >= T::MinStake::get(), Error::<T>::StakeTooSmall);

			let reserves = Self::reserve_factor().mul_floor(amount);

			let xcm_fees = T::XcmFees::get();
			let amount = amount.checked_sub(xcm_fees).ok_or(ArithmeticError::Underflow)?;

			use frame_support::traits::tokens::Preservation;
			let keep_alive = false;
			let keep_alive =
				if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
			T::Assets::transfer(
				Self::staking_currency()?,
				&who,
				&Self::account_id(),
				amount,
				keep_alive,
			)?;
			T::XCM::add_xcm_fees(&who, xcm_fees)?;

			let amount = amount.checked_sub(reserves).ok_or(ArithmeticError::Underflow)?;
			let liquid_amount =
				Self::staking_to_liquid(amount).ok_or(Error::<T>::InvalidExchangeRate)?;
			let liquid_currency = Self::liquid_currency()?;
			Self::ensure_market_cap(amount)?;

			T::Assets::mint_into(liquid_currency, &who, liquid_amount)?;

			log::trace!(
				target: "liquidStaking::stake",
				"stake_amount: {:?}, liquid_amount: {:?}, reserved: {:?}",
				&amount,
				&liquid_amount,
				&reserves
			);

			MatchingPool::<T>::try_mutate(|p| -> DispatchResult { p.add_stake_amount(amount) })?;
			TotalReserves::<T>::try_mutate(|b| -> DispatchResult {
				*b = b.checked_add(reserves).ok_or(ArithmeticError::Overflow)?;
				Ok(())
			})?;

			Self::deposit_event(Event::<T>::Staked(who, amount));
			Ok(().into())
		}

		/// Unstake by exchange derivative for assets, the assets will not be available immediately.
		/// Instead, the request is recorded and pending for the nomination accounts on relaychain
		/// chain to do the `unbond` operation.
		///
		/// - `amount`: the amount of derivative
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::unstake())]
		#[transactional]
		pub fn unstake(
			origin: OriginFor<T>,
			#[pallet::compact] liquid_amount: BalanceOf<T>,
			unstake_provider: UnstakeProvider,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(liquid_amount >= T::MinUnstake::get(), Error::<T>::UnstakeTooSmall);

			if unstake_provider.is_matching_pool() {
				use frame_support::traits::tokens::{Fortitude, Preservation};
				let keep_alive = false;
				let keep_alive =
					if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
				FastUnstakeRequests::<T>::try_mutate(&who, |b| -> DispatchResult {
					let balance = T::Assets::reducible_balance(
						Self::liquid_currency()?,
						&who,
						keep_alive,
						Fortitude::Polite,
					);
					*b = b.saturating_add(liquid_amount).min(balance);
					Ok(())
				})?;
				return Ok(().into());
			}

			let amount =
				Self::liquid_to_staking(liquid_amount).ok_or(Error::<T>::InvalidExchangeRate)?;
			let unlockings_key = who.clone();

			Unlockings::<T>::try_mutate(&unlockings_key, |b| -> DispatchResult {
				let mut chunks = b.take().unwrap_or_default();
				let target_era = Self::target_era();
				if let Some(mut chunk) = chunks.last_mut().filter(|chunk| chunk.era == target_era) {
					chunk.value = chunk.value.saturating_add(amount);
				} else {
					chunks.push(UnlockChunk { value: amount, era: target_era });
				}
				ensure!(chunks.len() <= MAX_UNLOCKING_CHUNKS, Error::<T>::NoMoreChunks);
				*b = Some(chunks);
				Ok(())
			})?;

			T::Assets::burn_from(
				Self::liquid_currency()?,
				&who,
				liquid_amount,
				Precision::Exact,
				Fortitude::Polite,
			)?;

			MatchingPool::<T>::try_mutate(|p| p.add_unstake_amount(amount))?;

			log::trace!(
				target: "liquidStaking::unstake",
				"unstake_amount: {:?}, liquid_amount: {:?}",
				&amount,
				&liquid_amount,
			);

			Self::deposit_event(Event::<T>::Unstaked(who, liquid_amount, amount));
			Ok(().into())
		}

		/// Update insurance pool's reserve_factor
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::update_reserve_factor())]
		#[transactional]
		pub fn update_reserve_factor(
			origin: OriginFor<T>,
			reserve_factor: Ratio,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			ensure!(
				reserve_factor > Ratio::zero() && reserve_factor < Ratio::one(),
				Error::<T>::InvalidFactor,
			);

			log::trace!(
				target: "liquidStaking::update_reserve_factor",
				 "reserve_factor: {:?}",
				&reserve_factor,
			);

			ReserveFactor::<T>::mutate(|v| *v = reserve_factor);
			Self::deposit_event(Event::<T>::ReserveFactorUpdated(reserve_factor));
			Ok(().into())
		}

		/// Update ledger's max bonded cap
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::update_staking_ledger_cap())]
		#[transactional]
		pub fn update_staking_ledger_cap(
			origin: OriginFor<T>,
			#[pallet::compact] cap: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			ensure!(!cap.is_zero(), Error::<T>::InvalidCap);

			log::trace!(
				target: "liquidStaking::update_staking_ledger_cap",
				"cap: {:?}",
				&cap,
			);
			StakingLedgerCap::<T>::mutate(|v| *v = cap);
			Self::deposit_event(Event::<T>::StakingLedgerCapUpdated(cap));
			Ok(().into())
		}

		/// Bond on relaychain via xcm.transact
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::bond())]
		#[transactional]
		pub fn bond(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			#[pallet::compact] amount: BalanceOf<T>,
			payee: RewardDestination<T::AccountId>,
		) -> DispatchResult {
			T::RelayOrigin::ensure_origin(origin)?;
			Self::do_bond(derivative_index, amount, payee)?;
			Ok(())
		}

		/// Bond_extra on relaychain via xcm.transact
		#[pallet::call_index(5)]
		#[pallet::weight(<T as Config>::WeightInfo::bond_extra())]
		#[transactional]
		pub fn bond_extra(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			T::RelayOrigin::ensure_origin(origin)?;
			Self::do_bond_extra(derivative_index, amount)?;
			Ok(())
		}

		/// Unbond on relaychain via xcm.transact
		#[pallet::call_index(6)]
		#[pallet::weight(<T as Config>::WeightInfo::unbond())]
		#[transactional]
		pub fn unbond(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			T::RelayOrigin::ensure_origin(origin)?;
			Self::do_unbond(derivative_index, amount)?;
			Ok(())
		}

		/// Rebond on relaychain via xcm.transact
		#[pallet::call_index(7)]
		#[pallet::weight(<T as Config>::WeightInfo::rebond())]
		#[transactional]
		pub fn rebond(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			T::RelayOrigin::ensure_origin(origin)?;
			Self::do_rebond(derivative_index, amount)?;
			Ok(())
		}

		/// Withdraw unbonded on relaychain via xcm.transact
		#[pallet::call_index(8)]
		#[pallet::weight(<T as Config>::WeightInfo::withdraw_unbonded())]
		#[transactional]
		pub fn withdraw_unbonded(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			num_slashing_spans: u32,
		) -> DispatchResult {
			Self::ensure_origin(origin)?;
			Self::do_withdraw_unbonded(derivative_index, num_slashing_spans)?;
			Ok(())
		}

		/// Nominate on relaychain via xcm.transact
		#[pallet::call_index(9)]
		#[pallet::weight(<T as Config>::WeightInfo::nominate())]
		#[transactional]
		pub fn nominate(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			targets: Vec<T::AccountId>,
		) -> DispatchResult {
			Self::ensure_origin(origin)?;
			Self::do_nominate(derivative_index, targets)?;
			Ok(())
		}

		/// Internal call which is expected to be triggered only by xcm instruction
		#[pallet::call_index(10)]
		#[pallet::weight(<T as Config>::WeightInfo::notification_received())]
		#[transactional]
		pub fn notification_received(
			origin: OriginFor<T>,
			query_id: QueryId,
			response: Response,
		) -> DispatchResultWithPostInfo {
			let responder = ensure_response(<T as Config>::RuntimeOrigin::from(origin.clone()))
				.or_else(|_| {
					T::UpdateOrigin::ensure_origin(origin).map(|_| MultiLocation::here())
				})?;
			if let Response::ExecutionResult(res) = response {
				Self::deposit_event(Event::<T>::NotificationReceived(
					Box::new(responder),
					query_id,
					res,
				));

				if let Some(request) = Self::xcm_request(query_id) {
					Self::do_notification_received(query_id, request, res)?;
				}

				Self::deposit_event(Event::<T>::NotificationReceived(
					Box::new(responder),
					query_id,
					res,
				));
			}
			Ok(().into())
		}

		/// Claim assets back when current era index arrived
		/// at target era
		#[pallet::call_index(11)]
		#[pallet::weight(<T as Config>::WeightInfo::claim_for())]
		#[transactional]
		pub fn claim_for(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			Self::ensure_origin(origin)?;
			let who = T::Lookup::lookup(dest)?;
			let current_era = Self::current_era();

			Unlockings::<T>::try_mutate_exists(&who, |b| -> DispatchResult {
				let mut amount: BalanceOf<T> = Zero::zero();
				let chunks = b.as_mut().ok_or(Error::<T>::NoUnlockings)?;
				chunks.retain(|chunk| {
					if chunk.era > current_era {
						true
					} else {
						amount += chunk.value;
						false
					}
				});

				let total_unclaimed = Self::get_total_unclaimed(Self::staking_currency()?);

				log::trace!(
					target: "liquidStaking::claim_for",
					"current_era: {:?}, beneficiary: {:?}, total_unclaimed: {:?}, amount: {:?}",
					&current_era,
					&who,
					&total_unclaimed,
					amount
				);

				if amount.is_zero() {
					return Err(Error::<T>::NothingToClaim.into());
				}

				if total_unclaimed < amount {
					return Err(Error::<T>::NotWithdrawn.into());
				}

				Self::do_claim_for(&who, amount)?;

				if chunks.is_empty() {
					*b = None;
				}

				Self::deposit_event(Event::<T>::ClaimedFor(who.clone(), amount));
				Ok(())
			})?;
			Ok(().into())
		}

		/// Force set era start block
		#[pallet::call_index(12)]
		#[pallet::weight(<T as Config>::WeightInfo::force_set_era_start_block())]
		#[transactional]
		pub fn force_set_era_start_block(
			origin: OriginFor<T>,
			block_number: BlockNumberFor<T>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			EraStartBlock::<T>::put(block_number);
			Ok(())
		}

		/// Force set current era
		#[pallet::call_index(13)]
		#[pallet::weight(<T as Config>::WeightInfo::force_set_current_era())]
		#[transactional]
		pub fn force_set_current_era(origin: OriginFor<T>, era: EraIndex) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			IsMatched::<T>::put(false);
			CurrentEra::<T>::put(era);
			Ok(())
		}

		/// Force advance era
		#[pallet::call_index(14)]
		#[pallet::weight(<T as Config>::WeightInfo::force_advance_era())]
		#[transactional]
		pub fn force_advance_era(
			origin: OriginFor<T>,
			offset: EraIndex,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			Self::do_advance_era(offset)?;

			Ok(().into())
		}

		/// Force matching
		#[pallet::call_index(15)]
		#[pallet::weight(<T as Config>::WeightInfo::force_matching())]
		#[transactional]
		pub fn force_matching(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			Self::do_matching()?;

			Ok(().into())
		}

		/// Force set staking_ledger
		#[pallet::call_index(16)]
		#[pallet::weight(<T as Config>::WeightInfo::force_set_staking_ledger())]
		#[transactional]
		pub fn force_set_staking_ledger(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			staking_ledger: StakingLedger<T::AccountId, BalanceOf<T>>,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			Self::do_update_ledger(derivative_index, |ledger| {
				ensure!(
					!Self::is_updated(derivative_index) &&
						XcmRequests::<T>::iter().count().is_zero(),
					Error::<T>::StakingLedgerLocked
				);
				*ledger = staking_ledger;
				Ok(())
			})?;

			Ok(().into())
		}

		/// Set current era by providing storage proof
		#[pallet::call_index(17)]
		#[pallet::weight(<T as Config>::WeightInfo::force_set_current_era())]
		#[transactional]
		pub fn set_current_era(
			origin: OriginFor<T>,
			era: EraIndex,
			proof: Vec<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let offset = era.saturating_sub(Self::current_era());

			let key = Self::get_current_era_key();
			let value = era.encode();
			ensure!(Self::verify_merkle_proof(key, value, proof), Error::<T>::InvalidProof);

			Self::do_advance_era(offset)?;
			if !offset.is_zero() {
				use frame_support::traits::tokens::Preservation;
				let keep_alive = false;
				let keep_alive =
					if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
				let _ = T::Assets::transfer(
					T::NativeCurrency::get(),
					&Self::account_id(),
					&who,
					Self::incentive(),
					keep_alive,
				);
			}

			Ok(().into())
		}

		/// Set staking_ledger by providing storage proof
		#[pallet::call_index(18)]
		#[pallet::weight(<T as Config>::WeightInfo::force_set_staking_ledger())]
		#[transactional]
		pub fn set_staking_ledger(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
			staking_ledger: StakingLedger<T::AccountId, BalanceOf<T>>,
			proof: Vec<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Self::deposit_event(Event::<T>::SetStakingLedgerTry {
				origin: who.clone(),
				derivative_index,
				staking_ledger: staking_ledger.clone(),
				proof: proof.clone(),
			});

			Self::do_update_ledger(derivative_index, |ledger| {
				ensure!(!Self::is_updated(derivative_index), Error::<T>::StakingLedgerLocked);
				let requests = XcmRequests::<T>::iter().count();
				if staking_ledger.total < ledger.total ||
					staking_ledger.active < ledger.active ||
					staking_ledger.unlocking != ledger.unlocking ||
					!requests.is_zero()
				{
					log::trace!(
						target: "liquidStaking::set_staking_ledger::invalidStakingLedger",
						"index: {:?}, staking_ledger: {:?}, xcm_request: {:?}",
						&derivative_index,
						&staking_ledger,
						requests,
					);
					Self::deposit_event(Event::<T>::NonIdealStakingLedger(derivative_index));
				}
				let key = Self::get_staking_ledger_key(derivative_index);
				let value = staking_ledger.encode();
				ensure!(Self::verify_merkle_proof(key, value, proof), Error::<T>::InvalidProof);
				let rewards = staking_ledger.total.saturating_sub(ledger.total);

				let inflate_liquid_amount = Self::get_inflate_liquid_amount(rewards)?;
				if !inflate_liquid_amount.is_zero() {
					T::Assets::mint_into(
						Self::liquid_currency()?,
						&T::ProtocolFeeReceiver::get(),
						inflate_liquid_amount,
					)?;
				}

				log::trace!(
					target: "liquidStaking::set_staking_ledger",
					"index: {:?}, staking_ledger: {:?}, inflate_liquid_amount: {:?}",
					&derivative_index,
					&staking_ledger,
					inflate_liquid_amount,
				);
				use frame_support::traits::tokens::Preservation;
				let keep_alive = false;
				let keep_alive =
					if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
				let _ = T::Assets::transfer(
					T::NativeCurrency::get(),
					&Self::account_id(),
					&who,
					Self::incentive(),
					keep_alive,
				);
				*ledger = staking_ledger;
				Ok(())
			})?;

			Ok(().into())
		}

		/// Reduces reserves by transferring to receiver.
		#[pallet::call_index(19)]
		#[pallet::weight(<T as Config>::WeightInfo::reduce_reserves())]
		#[transactional]
		pub fn reduce_reserves(
			origin: OriginFor<T>,
			receiver: <T::Lookup as StaticLookup>::Source,
			#[pallet::compact] reduce_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			let receiver = T::Lookup::lookup(receiver)?;

			TotalReserves::<T>::try_mutate(|b| -> DispatchResult {
				*b = b.checked_sub(reduce_amount).ok_or(ArithmeticError::Underflow)?;
				Ok(())
			})?;

			use frame_support::traits::tokens::Preservation;
			let keep_alive = false;
			let keep_alive =
				if keep_alive { Preservation::Preserve } else { Preservation::Expendable };

			T::Assets::transfer(
				Self::staking_currency()?,
				&Self::account_id(),
				&receiver,
				reduce_amount,
				keep_alive,
			)?;

			Self::deposit_event(Event::<T>::ReservesReduced(receiver, reduce_amount));

			Ok(().into())
		}

		/// Cancel unstake
		#[pallet::call_index(20)]
		#[pallet::weight(<T as Config>::WeightInfo::cancel_unstake())]
		#[transactional]
		pub fn cancel_unstake(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			FastUnstakeRequests::<T>::try_mutate(&who, |b| -> DispatchResultWithPostInfo {
				use frame_support::traits::tokens::{Fortitude, Preservation};
				let keep_alive = false;
				let keep_alive =
					if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
				let balance = T::Assets::reducible_balance(
					Self::liquid_currency()?,
					&who,
					keep_alive,
					Fortitude::Polite,
				);
				*b = (*b).min(balance).saturating_sub(amount);

				// reserve two amounts in event
				Self::deposit_event(Event::<T>::UnstakeCancelled(who.clone(), amount, amount));

				Ok(().into())
			})
		}

		/// Update commission rate
		#[pallet::call_index(21)]
		#[pallet::weight(<T as Config>::WeightInfo::update_commission_rate())]
		#[transactional]
		pub fn update_commission_rate(
			origin: OriginFor<T>,
			commission_rate: Rate,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;

			ensure!(
				commission_rate > Rate::zero() && commission_rate < Rate::one(),
				Error::<T>::InvalidCommissionRate,
			);

			log::trace!(
				target: "liquidStaking::update_commission_rate",
				 "commission_rate: {:?}",
				&commission_rate,
			);

			CommissionRate::<T>::put(commission_rate);
			Self::deposit_event(Event::<T>::CommissionRateUpdated(commission_rate));
			Ok(())
		}

		/// Fast match unstake through matching pool
		#[pallet::call_index(22)]
		#[pallet::weight(<T as Config>::WeightInfo::fast_match_unstake(unstaker_list.len() as u32))]
		#[transactional]
		pub fn fast_match_unstake(
			origin: OriginFor<T>,
			unstaker_list: Vec<T::AccountId>,
		) -> DispatchResult {
			Self::ensure_origin(origin)?;
			for unstaker in unstaker_list {
				Self::do_fast_match_unstake(&unstaker)?;
			}
			Ok(())
		}

		/// Update incentive amount
		#[pallet::call_index(23)]
		#[pallet::weight(<T as Config>::WeightInfo::update_incentive())]
		#[transactional]
		pub fn update_incentive(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			Incentive::<T>::put(amount);
			Self::deposit_event(Event::<T>::IncentiveUpdated(amount));
			Ok(())
		}

		#[pallet::call_index(24)]
		#[pallet::weight(<T as Config>::WeightInfo::update_incentive())]
		#[transactional]
		pub fn initiate_exchange_rate(origin: OriginFor<T>) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			if Self::exchange_rate() == Rate::zero() {
				ExchangeRate::<T>::put(Rate::one());
				Self::deposit_event(Event::<T>::ExchangeRateUpdated(Rate::one()));
			}
			Ok(())
		}

		#[pallet::call_index(25)]
		#[pallet::weight(<T as Config>::WeightInfo::update_incentive())]
		#[transactional]
		pub fn relaychain_storage_proof_key(
			origin: OriginFor<T>,
			derivative_index: DerivativeIndex,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let key = Self::get_staking_ledger_key(derivative_index);
			let derivative_account_id = Self::derivative_sovereign_account_id(derivative_index);
			Self::deposit_event(Event::<T>::RelaychainStorageProofKey(
				derivative_index,
				key,
				derivative_account_id,
			));
			Ok(())
		}

		#[pallet::call_index(26)]
		#[pallet::weight(<T as Config>::WeightInfo::update_incentive())]
		#[transactional]
		pub fn set_members(origin: OriginFor<T>, members: Vec<T::AccountId>) -> DispatchResult {
			if T::UpdateOrigin::ensure_origin(origin).is_err() {
				return Err(BadOrigin.into());
			}
			Members::<T>::put(members.clone());
			Self::deposit_event(Event::<T>::SetMembers { members });
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(_block_number: T::BlockNumber) -> frame_support::weights::Weight {
			let mut weight = <T as Config>::WeightInfo::on_initialize();
			// let relaychain_block_number =
			// 	T::RelayChainValidationDataProvider::current_block_number();

			let relaychain_block_number =
				ValidationData::<T>::get().map(|i| i.relay_parent_number).unwrap_or(0).into();
			let mut do_on_initialize = || -> DispatchResult {
				if !Self::is_matched() &&
					T::ElectionSolutionStoredOffset::get()
						.saturating_add(Self::era_start_block()) <=
						relaychain_block_number
				{
					weight += <T as Config>::WeightInfo::force_matching();
					Self::do_matching()?;
				}

				let offset = Self::offset(relaychain_block_number);
				Self::deposit_event(Event::<T>::OnInitializeHook {
					relay_block_number: relaychain_block_number,
					era: offset,
				});
				if offset.is_zero() {
					return Ok(());
				}
				weight += <T as Config>::WeightInfo::force_advance_era();
				Self::do_advance_era(offset)
			};
			let _ = with_transaction(|| match do_on_initialize() {
				Ok(()) => TransactionOutcome::Commit(Ok(())),
				Err(err) => TransactionOutcome::Rollback(Err(err)),
			});
			weight
		}

		fn on_finalize(_n: T::BlockNumber) {
			let _ = IsUpdated::<T>::clear(u32::max_value(), None);
			if let Some(data) = T::RelayChainValidationDataProvider::validation_data() {
				ValidationData::<T>::put(data);
			}
		}
	}

	impl<T: Config> Pallet<T> {
		/// Staking pool account
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		/// Parachain's sovereign account
		pub fn sovereign_account_id() -> T::AccountId {
			T::SelfParaId::get().into_account_truncating()
		}

		/// Target era_index if users unstake in current_era
		pub fn target_era() -> EraIndex {
			// TODO: check if we can bond before the next era
			// so that the one era's delay can be removed
			Self::current_era() + T::BondingDuration::get() + 1
		}

		/// Get staking currency or return back an error
		pub fn staking_currency() -> Result<AssetIdOf<T>, DispatchError> {
			Self::get_staking_currency()
				.ok_or(Error::<T>::InvalidStakingCurrency)
				.map_err(Into::into)
		}

		/// Get liquid currency or return back an error
		pub fn liquid_currency() -> Result<AssetIdOf<T>, DispatchError> {
			Self::get_liquid_currency()
				.ok_or(Error::<T>::InvalidLiquidCurrency)
				.map_err(Into::into)
		}

		/// Get total unclaimed
		pub fn get_total_unclaimed(staking_currency: AssetIdOf<T>) -> BalanceOf<T> {
			use frame_support::traits::tokens::{Fortitude, Preservation};
			let keep_alive = false;
			let keep_alive =
				if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
			T::Assets::reducible_balance(
				staking_currency,
				&Self::account_id(),
				keep_alive,
				Fortitude::Polite,
			)
			.saturating_sub(Self::total_reserves())
			.saturating_sub(Self::matching_pool().total_stake_amount.total)
		}

		/// Derivative of parachain's account
		pub fn derivative_sovereign_account_id(index: DerivativeIndex) -> T::AccountId {
			let para_account = Self::sovereign_account_id();
			pallet_utility::Pallet::<T>::derivative_account_id(para_account, index)
		}

		fn offset(relaychain_block_number: BlockNumberFor<T>) -> EraIndex {
			relaychain_block_number
				.checked_sub(&Self::era_start_block())
				.and_then(|r| r.checked_div(&T::EraLength::get()))
				.and_then(|r| TryInto::<EraIndex>::try_into(r).ok())
				.unwrap_or_else(Zero::zero)
		}

		fn total_bonded_of(index: DerivativeIndex) -> BalanceOf<T> {
			Self::staking_ledger(index).map_or(Zero::zero(), |ledger| ledger.total)
		}

		fn active_bonded_of(index: DerivativeIndex) -> BalanceOf<T> {
			Self::staking_ledger(index).map_or(Zero::zero(), |ledger| ledger.active)
		}

		fn unbonding_of(index: DerivativeIndex) -> BalanceOf<T> {
			Self::staking_ledger(index)
				.map_or(Zero::zero(), |ledger| ledger.total.saturating_sub(ledger.active))
		}

		fn unbonded_of(index: DerivativeIndex) -> BalanceOf<T> {
			let current_era = Self::current_era();
			Self::staking_ledger(index).map_or(Zero::zero(), |ledger| {
				ledger.unlocking.iter().fold(Zero::zero(), |acc, chunk| {
					if chunk.era <= current_era {
						acc.saturating_add(chunk.value)
					} else {
						acc
					}
				})
			})
		}

		fn get_total_unbonding() -> BalanceOf<T> {
			StakingLedgers::<T>::iter_values().fold(Zero::zero(), |acc, ledger| {
				acc.saturating_add(ledger.total.saturating_sub(ledger.active))
			})
		}

		fn get_total_bonded() -> BalanceOf<T> {
			StakingLedgers::<T>::iter_values()
				.fold(Zero::zero(), |acc, ledger| acc.saturating_add(ledger.total))
		}

		fn get_total_active_bonded() -> BalanceOf<T> {
			StakingLedgers::<T>::iter_values()
				.fold(Zero::zero(), |acc, ledger| acc.saturating_add(ledger.active))
		}

		fn get_market_cap() -> BalanceOf<T> {
			Self::staking_ledger_cap()
				.saturating_mul(T::DerivativeIndexList::get().len() as BalanceOf<T>)
		}

		#[require_transactional]
		fn do_bond(
			derivative_index: DerivativeIndex,
			amount: BalanceOf<T>,
			payee: RewardDestination<T::AccountId>,
		) -> DispatchResult {
			if amount.is_zero() {
				return Ok(());
			}

			if StakingLedgers::<T>::contains_key(derivative_index) {
				return Self::do_bond_extra(derivative_index, amount);
			}

			ensure!(
				T::DerivativeIndexList::get().contains(&derivative_index),
				Error::<T>::InvalidDerivativeIndex
			);
			ensure!(amount >= T::MinNominatorBond::get(), Error::<T>::InsufficientBond);
			Self::ensure_staking_ledger_cap(derivative_index, amount)?;

			log::trace!(
				target: "liquidStaking::bond",
				"index: {:?}, amount: {:?}",
				&derivative_index,
				&amount,
			);

			MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
				p.set_stake_amount_lock(amount)
			})?;

			let derivative_account_id = Self::derivative_sovereign_account_id(derivative_index);
			let query_id = T::XCM::do_bond(
				amount,
				payee.clone(),
				derivative_account_id.clone(),
				derivative_index,
				Self::notify_placeholder(),
			)?;

			XcmRequests::<T>::insert(
				query_id,
				XcmRequest::Bond { index: derivative_index, amount },
			);

			Self::deposit_event(Event::<T>::Bonding(
				derivative_index,
				derivative_account_id,
				amount,
				payee,
			));

			Ok(())
		}

		#[require_transactional]
		fn do_bond_extra(
			derivative_index: DerivativeIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			if amount.is_zero() {
				return Ok(());
			}

			ensure!(
				T::DerivativeIndexList::get().contains(&derivative_index),
				Error::<T>::InvalidDerivativeIndex
			);
			ensure!(StakingLedgers::<T>::contains_key(derivative_index), Error::<T>::NotBonded);
			Self::ensure_staking_ledger_cap(derivative_index, amount)?;

			log::trace!(
				target: "liquidStaking::bond_extra",
				"index: {:?}, amount: {:?}",
				&derivative_index,
				&amount,
			);

			MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
				p.set_stake_amount_lock(amount)
			})?;

			let query_id = T::XCM::do_bond_extra(
				amount,
				Self::derivative_sovereign_account_id(derivative_index),
				derivative_index,
				Self::notify_placeholder(),
			)?;

			XcmRequests::<T>::insert(
				query_id,
				XcmRequest::BondExtra { index: derivative_index, amount },
			);

			Self::deposit_event(Event::<T>::BondingExtra(derivative_index, amount));

			Ok(())
		}

		#[require_transactional]
		fn do_unbond(derivative_index: DerivativeIndex, amount: BalanceOf<T>) -> DispatchResult {
			if amount.is_zero() {
				return Ok(());
			}

			ensure!(
				T::DerivativeIndexList::get().contains(&derivative_index),
				Error::<T>::InvalidDerivativeIndex
			);

			let ledger: StakingLedger<T::AccountId, BalanceOf<T>> =
				Self::staking_ledger(derivative_index).ok_or(Error::<T>::NotBonded)?;
			ensure!(ledger.unlocking.len() < MAX_UNLOCKING_CHUNKS, Error::<T>::NoMoreChunks);
			ensure!(
				ledger.active.saturating_sub(amount) >= T::MinNominatorBond::get(),
				Error::<T>::InsufficientBond
			);

			MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
				p.set_unstake_amount_lock(amount)
			})?;

			log::trace!(
				target: "liquidStaking::unbond",
				"index: {:?} , amount: {:?}",
				&derivative_index,
				&amount,
			);

			let query_id = T::XCM::do_unbond(amount, derivative_index, Self::notify_placeholder())?;

			XcmRequests::<T>::insert(
				query_id,
				XcmRequest::Unbond { index: derivative_index, amount },
			);

			Self::deposit_event(Event::<T>::Unbonding(derivative_index, amount));

			Ok(())
		}

		#[require_transactional]
		fn do_rebond(derivative_index: DerivativeIndex, amount: BalanceOf<T>) -> DispatchResult {
			if amount.is_zero() {
				return Ok(());
			}

			ensure!(
				T::DerivativeIndexList::get().contains(&derivative_index),
				Error::<T>::InvalidDerivativeIndex
			);
			ensure!(StakingLedgers::<T>::contains_key(derivative_index), Error::<T>::NotBonded);

			log::trace!(
				target: "liquidStaking::rebond",
				"index: {:?}, amount: {:?}",
				&derivative_index,
				&amount,
			);

			MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
				p.set_stake_amount_lock(amount)
			})?;

			let query_id = T::XCM::do_rebond(amount, derivative_index, Self::notify_placeholder())?;

			XcmRequests::<T>::insert(
				query_id,
				XcmRequest::Rebond { index: derivative_index, amount },
			);

			Self::deposit_event(Event::<T>::Rebonding(derivative_index, amount));

			Ok(())
		}

		#[require_transactional]
		fn do_withdraw_unbonded(
			derivative_index: DerivativeIndex,
			num_slashing_spans: u32,
		) -> DispatchResult {
			if Self::unbonded_of(derivative_index).is_zero() {
				return Ok(());
			}

			ensure!(
				T::DerivativeIndexList::get().contains(&derivative_index),
				Error::<T>::InvalidDerivativeIndex
			);
			ensure!(StakingLedgers::<T>::contains_key(derivative_index), Error::<T>::NotBonded);

			log::trace!(
				target: "liquidStaking::withdraw_unbonded",
				"index: {:?}, num_slashing_spans: {:?}",
				&derivative_index,
				&num_slashing_spans,
			);

			let query_id = T::XCM::do_withdraw_unbonded(
				num_slashing_spans,
				Self::sovereign_account_id(),
				derivative_index,
				Self::notify_placeholder(),
			)?;

			XcmRequests::<T>::insert(
				query_id,
				XcmRequest::WithdrawUnbonded { index: derivative_index, num_slashing_spans },
			);

			Self::deposit_event(Event::<T>::WithdrawingUnbonded(
				derivative_index,
				num_slashing_spans,
			));

			Ok(())
		}

		#[require_transactional]
		fn do_nominate(
			derivative_index: DerivativeIndex,
			targets: Vec<T::AccountId>,
		) -> DispatchResult {
			ensure!(
				T::DerivativeIndexList::get().contains(&derivative_index),
				Error::<T>::InvalidDerivativeIndex
			);
			ensure!(StakingLedgers::<T>::contains_key(derivative_index), Error::<T>::NotBonded);

			log::trace!(
				target: "liquidStaking::nominate",
				"index: {:?}",
				&derivative_index,
			);

			let query_id =
				T::XCM::do_nominate(targets.clone(), derivative_index, Self::notify_placeholder())?;

			XcmRequests::<T>::insert(
				query_id,
				XcmRequest::Nominate { index: derivative_index, targets: targets.clone() },
			);

			Self::deposit_event(Event::<T>::Nominating(derivative_index, targets));

			Ok(())
		}

		#[require_transactional]
		fn do_multi_bond(
			total_amount: BalanceOf<T>,
			payee: RewardDestination<T::AccountId>,
		) -> DispatchResult {
			if total_amount.is_zero() {
				return Ok(());
			}

			let amounts: Vec<(DerivativeIndex, BalanceOf<T>, BalanceOf<T>)> =
				T::DerivativeIndexList::get()
					.iter()
					.map(|&index| {
						(index, Self::active_bonded_of(index), Self::total_bonded_of(index))
					})
					.collect();
			let distributions = T::DistributionStrategy::get_bond_distributions(
				amounts,
				total_amount,
				Self::staking_ledger_cap(),
				T::MinNominatorBond::get(),
			);

			for (index, amount) in distributions.into_iter() {
				Self::do_bond(index, amount, payee.clone())?;
			}

			Ok(())
		}

		#[require_transactional]
		fn do_multi_unbond(total_amount: BalanceOf<T>) -> DispatchResult {
			if total_amount.is_zero() {
				return Ok(());
			}

			let amounts: Vec<(DerivativeIndex, BalanceOf<T>)> = T::DerivativeIndexList::get()
				.iter()
				.map(|&index| (index, Self::active_bonded_of(index)))
				.collect();
			let distributions = T::DistributionStrategy::get_unbond_distributions(
				amounts,
				total_amount,
				T::MinNominatorBond::get(),
			);

			for (index, amount) in distributions.into_iter() {
				Self::do_unbond(index, amount)?;
			}

			Ok(())
		}

		#[require_transactional]
		fn do_multi_rebond(total_amount: BalanceOf<T>) -> DispatchResult {
			if total_amount.is_zero() {
				return Ok(());
			}

			let amounts: Vec<(DerivativeIndex, BalanceOf<T>)> = T::DerivativeIndexList::get()
				.iter()
				.map(|&index| (index, Self::unbonding_of(index)))
				.collect();
			let distributions =
				T::DistributionStrategy::get_rebond_distributions(amounts, total_amount);

			for (index, amount) in distributions.into_iter() {
				Self::do_rebond(index, amount)?;
			}

			Ok(())
		}

		#[require_transactional]
		fn do_multi_withdraw_unbonded(num_slashing_spans: u32) -> DispatchResult {
			for derivative_index in StakingLedgers::<T>::iter_keys() {
				Self::do_withdraw_unbonded(derivative_index, num_slashing_spans)?;
			}

			Ok(())
		}

		#[require_transactional]
		fn do_notification_received(
			query_id: QueryId,
			req: XcmRequest<T>,
			res: Option<(u32, XcmError)>,
		) -> DispatchResult {
			use XcmRequest::*;

			log::info!(
				target: "liquidStaking::notification_received",
				"query_id: {:?}, response: {:?}",
				&query_id,
				&res
			);

			let executed = res.is_none();
			if !executed {
				return Ok(());
			}

			match req {
				Bond { index: derivative_index, amount } => {
					ensure!(
						!StakingLedgers::<T>::contains_key(derivative_index),
						Error::<T>::AlreadyBonded
					);
					let staking_ledger = <StakingLedger<T::AccountId, BalanceOf<T>>>::new(
						Self::derivative_sovereign_account_id(derivative_index),
						amount,
					);
					StakingLedgers::<T>::insert(derivative_index, staking_ledger);
					MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
						p.consolidate_stake(amount)
					})?;
					T::Assets::burn_from(
						Self::staking_currency()?,
						&Self::account_id(),
						amount,
						Precision::Exact,
						Fortitude::Polite,
					)?;
				},
				BondExtra { index: derivative_index, amount } => {
					Self::do_update_ledger(derivative_index, |ledger| {
						ledger.bond_extra(amount);
						Ok(())
					})?;
					MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
						p.consolidate_stake(amount)
					})?;
					T::Assets::burn_from(
						Self::staking_currency()?,
						&Self::account_id(),
						amount,
						Precision::Exact,
						Fortitude::Polite,
					)?;
				},
				Unbond { index: derivative_index, amount } => {
					let target_era = Self::current_era() + T::BondingDuration::get();
					Self::do_update_ledger(derivative_index, |ledger| {
						ledger.unbond(amount, target_era);
						Ok(())
					})?;
					MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
						p.consolidate_unstake(amount)
					})?;
				},
				Rebond { index: derivative_index, amount } => {
					Self::do_update_ledger(derivative_index, |ledger| {
						ledger.rebond(amount);
						Ok(())
					})?;
					MatchingPool::<T>::try_mutate(|p| -> DispatchResult {
						p.consolidate_stake(amount)
					})?;
				},
				WithdrawUnbonded { index: derivative_index, num_slashing_spans: _ } => {
					Self::do_update_ledger(derivative_index, |ledger| {
						let current_era = Self::current_era();
						let total = ledger.total;
						let staking_currency = Self::staking_currency()?;
						let account_id = Self::account_id();
						ledger.consolidate_unlocked(current_era);
						let amount = total.saturating_sub(ledger.total);
						T::Assets::mint_into(staking_currency, &account_id, amount)?;
						Ok(())
					})?;
				},
				Nominate { targets: _, .. } => {},
			}
			XcmRequests::<T>::remove(query_id);
			Ok(())
		}

		#[require_transactional]
		fn do_update_exchange_rate() -> DispatchResult {
			let matching_ledger = Self::matching_pool();
			let total_active_bonded = Self::get_total_active_bonded();
			let issuance = T::Assets::total_issuance(Self::liquid_currency()?);
			if issuance.is_zero() {
				return Ok(());
			}
			// TODO: when one era has big amount of stakes, the exchange rate
			// will not look great
			let new_exchange_rate = Rate::checked_from_rational(
				total_active_bonded
					.checked_add(matching_ledger.total_stake_amount.total)
					.and_then(|r| r.checked_sub(matching_ledger.total_unstake_amount.total))
					.ok_or(ArithmeticError::Overflow)?,
				issuance,
			)
			.ok_or(Error::<T>::InvalidExchangeRate)?;
			// slashes should be handled properly offchain
			// by doing `bond_extra` using OrmlXcm or PolkadotXcm
			if new_exchange_rate > Self::exchange_rate() {
				ExchangeRate::<T>::put(new_exchange_rate);
				Self::deposit_event(Event::<T>::ExchangeRateUpdated(new_exchange_rate));
			}
			Ok(())
		}

		#[require_transactional]
		fn do_update_ledger(
			derivative_index: DerivativeIndex,
			cb: impl FnOnce(&mut StakingLedger<T::AccountId, BalanceOf<T>>) -> DispatchResult,
		) -> DispatchResult {
			StakingLedgers::<T>::try_mutate(derivative_index, |ledger| -> DispatchResult {
				let ledger = ledger.as_mut().ok_or(Error::<T>::NotBonded)?;
				cb(ledger)?;
				IsUpdated::<T>::insert(derivative_index, true);
				Self::deposit_event(Event::<T>::StakingLedgerUpdated(
					derivative_index,
					ledger.clone(),
				));
				Ok(())
			})
		}

		#[require_transactional]
		pub fn do_matching() -> DispatchResult {
			let total_unbonding = Self::get_total_unbonding();
			let (bond_amount, rebond_amount, unbond_amount) =
				Self::matching_pool().matching(total_unbonding)?;

			log::trace!(
				target: "liquidStaking::do_matching",
				"bond_amount: {:?}, rebond_amount: {:?}, unbond_amount: {:?}",
				&bond_amount,
				&rebond_amount,
				&unbond_amount
			);

			IsMatched::<T>::put(true);

			Self::do_multi_bond(bond_amount, RewardDestination::Staked)?;
			Self::do_multi_rebond(rebond_amount)?;

			Self::do_multi_unbond(unbond_amount)?;

			Self::do_multi_withdraw_unbonded(T::NumSlashingSpans::get())?;

			Self::deposit_event(Event::<T>::Matching(bond_amount, rebond_amount, unbond_amount));

			Ok(())
		}

		#[require_transactional]
		pub fn do_advance_era(offset: EraIndex) -> DispatchResult {
			if offset.is_zero() {
				return Ok(());
			}

			log::trace!(
				target: "liquidStaking::do_advance_era",
				"offset: {:?}",
				&offset,
			);

			EraStartBlock::<T>::put(T::RelayChainValidationDataProvider::current_block_number());
			CurrentEra::<T>::mutate(|e| *e = e.saturating_add(offset));

			// ignore error
			if let Err(e) = Self::do_update_exchange_rate() {
				log::error!(target: "liquidStaking::do_advance_era", "advance era error caught: {:?}", &e);
			}

			IsMatched::<T>::put(false);
			Self::deposit_event(Event::<T>::NewEra(Self::current_era()));
			Ok(())
		}

		#[require_transactional]
		fn do_claim_for(who: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
			let module_id = Self::account_id();
			let staking_currency = Self::staking_currency()?;

			use frame_support::traits::tokens::Preservation;
			let keep_alive = false;
			let keep_alive =
				if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
			T::Assets::transfer(staking_currency, &module_id, who, amount, keep_alive)?;

			Ok(())
		}

		// liquid_amount_to_fee=TotalLiquidCurrency *
		// (commission_rate*total_rewards/(TotalStakeCurrency+(1-commission_rate)*total_rewards))
		fn get_inflate_liquid_amount(rewards: BalanceOf<T>) -> Result<BalanceOf<T>, DispatchError> {
			let issuance = T::Assets::total_issuance(Self::liquid_currency()?);
			let commission_rate = Self::commission_rate();
			if issuance.is_zero() || commission_rate.is_zero() || rewards.is_zero() {
				return Ok(Zero::zero());
			}

			let matching_ledger = Self::matching_pool();
			let total_active_bonded = Self::get_total_active_bonded();
			let total_stake_amount = total_active_bonded
				.checked_add(matching_ledger.total_stake_amount.total)
				.and_then(|r| r.checked_sub(matching_ledger.total_unstake_amount.total))
				.ok_or(ArithmeticError::Overflow)?;

			let commission_staking_amount = commission_rate.saturating_mul_int(rewards);

			let inflate_rate = Rate::checked_from_rational(
				commission_staking_amount,
				total_stake_amount
					.saturating_add(rewards)
					.saturating_sub(commission_staking_amount),
			)
			.unwrap_or_else(Rate::zero);
			let inflate_liquid_amount = inflate_rate.saturating_mul_int(issuance);
			Ok(inflate_liquid_amount)
		}

		#[require_transactional]
		fn do_fast_match_unstake(unstaker: &T::AccountId) -> DispatchResult {
			FastUnstakeRequests::<T>::try_mutate_exists(unstaker, |b| -> DispatchResult {
				if b.is_none() {
					return Ok(());
				}
				let keep_alive = false;
				let keep_alive =
					if keep_alive { Preservation::Preserve } else { Preservation::Expendable };
				let current_liquid_amount = T::Assets::reducible_balance(
					Self::liquid_currency()?,
					unstaker,
					keep_alive,
					Fortitude::Polite,
				);
				let request_liquid_amount =
					b.take().expect("Could not be none, qed;").min(current_liquid_amount);

				let available_liquid_amount =
					Self::staking_to_liquid(Self::matching_pool().total_stake_amount.free()?)
						.ok_or(Error::<T>::InvalidExchangeRate)?;

				let matched_liquid_amount = request_liquid_amount.min(available_liquid_amount);

				if !matched_liquid_amount.is_zero() {
					let matched_fee = T::MatchingPoolFastUnstakeFee::get()
						.saturating_mul_int(matched_liquid_amount);
					let liquid_to_burn = matched_liquid_amount.saturating_sub(matched_fee);
					T::Assets::burn_from(
						Self::liquid_currency()?,
						unstaker,
						liquid_to_burn,
						Precision::Exact,
						Fortitude::Polite,
					)?;
					T::Assets::transfer(
						Self::liquid_currency()?,
						unstaker,
						&T::ProtocolFeeReceiver::get(),
						matched_fee,
						keep_alive,
					)?;

					let staking_to_receive = Self::liquid_to_staking(liquid_to_burn)
						.ok_or(Error::<T>::InvalidExchangeRate)?;

					MatchingPool::<T>::try_mutate(|p| p.sub_stake_amount(staking_to_receive))?;
					T::Assets::transfer(
						Self::staking_currency()?,
						&Self::account_id(),
						unstaker,
						staking_to_receive,
						keep_alive,
					)?;

					Self::deposit_event(Event::<T>::FastUnstakeMatched(
						unstaker.clone(),
						staking_to_receive,
						matched_liquid_amount,
						matched_fee,
					));
				}

				let unmatched_amount = request_liquid_amount.saturating_sub(matched_liquid_amount);
				if !unmatched_amount.is_zero() {
					*b = Some(unmatched_amount);
				}

				log::trace!(
					target: "liquidStaking::do_fast_match_unstake",
					"unstaker: {:?}, request_liquid_amount: {:?}, unmatched_amount: {:?}",
					unstaker,
					request_liquid_amount,
					unmatched_amount,
				);

				Ok(())
			})
		}

		fn ensure_origin(origin: OriginFor<T>) -> DispatchResult {
			if T::RelayOrigin::ensure_origin(origin.clone()).is_ok() {
				return Ok(());
			}
			let who = ensure_signed(origin)?;
			if !T::Members::contains(&who) && !Members::<T>::get().contains(&who) {
				return Err(BadOrigin.into());
			}
			Ok(())
		}

		fn ensure_market_cap(amount: BalanceOf<T>) -> DispatchResult {
			ensure!(
				Self::get_total_bonded().saturating_add(amount) <= Self::get_market_cap(),
				Error::<T>::CapExceeded
			);
			Ok(())
		}

		fn ensure_staking_ledger_cap(
			derivative_index: DerivativeIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure!(
				Self::total_bonded_of(derivative_index).saturating_add(amount) <=
					Self::staking_ledger_cap(),
				Error::<T>::CapExceeded
			);
			Ok(())
		}

		fn notify_placeholder() -> <T as Config>::RuntimeCall {
			<T as Config>::RuntimeCall::from(Call::<T>::notification_received {
				query_id: Default::default(),
				response: Default::default(),
			})
		}

		pub(crate) fn verify_merkle_proof(
			key: Vec<u8>,
			value: Vec<u8>,
			proof: Vec<Vec<u8>>,
		) -> bool {
			let validation_data = Self::validation_data();
			if validation_data.is_none() {
				return false;
			}
			let PersistedValidationData { relay_parent_number, relay_parent_storage_root, .. } =
				validation_data.expect("Could not be none, qed;");
			log::trace!(
				target: "liquidStaking::verify_merkle_proof",
				"relay_parent_number: {:?}, relay_parent_storage_root: {:?}",
				&relay_parent_number, &relay_parent_storage_root,
			);
			let relay_proof = StorageProof::new(proof);
			let db = relay_proof.into_memory_db();
			if let Ok(Some(result)) = sp_trie::read_trie_value::<sp_trie::LayoutV1<BlakeTwo256>, _>(
				&db,
				&relay_parent_storage_root,
				&key,
				None,
				None,
			) {
				return result == value;
			}
			false
		}

		pub(crate) fn get_staking_ledger_key(derivative_index: DerivativeIndex) -> Vec<u8> {
			let storage_prefix = storage_prefix("Staking".as_bytes(), "Ledger".as_bytes());
			let key = Self::derivative_sovereign_account_id(derivative_index);
			let key_hashed = key.borrow().using_encoded(Blake2_128Concat::hash);
			let mut final_key =
				Vec::with_capacity(storage_prefix.len() + (key_hashed.as_ref() as &[u8]).len());

			final_key.extend_from_slice(&storage_prefix);
			final_key.extend_from_slice(key_hashed.as_ref() as &[u8]);

			final_key
		}

		pub(crate) fn get_current_era_key() -> Vec<u8> {
			storage_prefix("Staking".as_bytes(), "CurrentEra".as_bytes()).to_vec()
		}
	}
}

use crate::types::DecimalProvider;
impl<T: Config> LiquidStakingCurrenciesProvider<AssetIdOf<T>> for Pallet<T> {
	fn get_staking_currency() -> Option<AssetIdOf<T>> {
		let asset_id = T::StakingCurrency::get();
		if T::Decimal::get_decimal(&asset_id).is_some() {
			Some(asset_id)
		} else {
			None
		}
	}

	fn get_liquid_currency() -> Option<AssetIdOf<T>> {
		let asset_id = T::LiquidCurrency::get();
		if T::Decimal::get_decimal(&asset_id).is_some() {
			Some(asset_id)
		} else {
			None
		}
	}
}

pub trait LiquidStakingCurrenciesProvider<CurrencyId> {
	fn get_staking_currency() -> Option<CurrencyId>;
	fn get_liquid_currency() -> Option<CurrencyId>;
}

pub trait LiquidStakingConvert<Balance> {
	fn staking_to_liquid(amount: Balance) -> Option<Balance>;
	fn liquid_to_staking(liquid_amount: Balance) -> Option<Balance>;
}

impl<T: Config, Balance: BalanceT + FixedPointOperand> LiquidStakingConvert<Balance> for Pallet<T> {
	fn staking_to_liquid(amount: Balance) -> Option<Balance> {
		Self::exchange_rate().reciprocal().and_then(|r| r.checked_mul_int(amount))
	}

	fn liquid_to_staking(liquid_amount: Balance) -> Option<Balance> {
		Self::exchange_rate().checked_mul_int(liquid_amount)
	}
}
