// Copyright 2021 Parallel Finance Developer.
// This file is part of Parallel Finance.

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
// use pallet_traits::{
//     DecimalProvider, DistributionStrategy, ExchangeRateProvider, LiquidStakingConvert,
//     LiquidStakingCurrenciesProvider, Loans, LoansMarketDataProvider, LoansPositionDataProvider,
//     ValidationDataProvider,
// };
// use primitives::{PersistedValidationData, Rate};

// mod benchmarking;

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod distribution;
// pub mod migrations;
pub mod types;
// pub mod weights;
// pub use weights::WeightInfo;


#[frame_support::pallet]
pub mod pallet {
    use parity_scale_codec::Encode;
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        ensure,
        error::BadOrigin,
        log,
        pallet_prelude::*,
        require_transactional,
        storage::{storage_prefix, with_transaction},
        traits::{
            fungibles::{Inspect, Mutate, /*Transfer*/ },
            IsType, SortedMembers,
        },
        transactional, PalletId, StorageHasher,
    };
    use frame_system::{
        ensure_signed,
        pallet_prelude::{BlockNumberFor, OriginFor},
    };
    use pallet_xcm::ensure_response;
    use sp_runtime::{
        traits::{
            AccountIdConversion, BlakeTwo256, BlockNumberProvider, CheckedDiv, CheckedSub,
            Saturating, StaticLookup,
        },
        ArithmeticError, FixedPointNumber, TransactionOutcome,
    };
    use sp_std::{borrow::Borrow, boxed::Box, cmp::min, result::Result, vec::Vec};
    use sp_trie::StorageProof;
    use xcm::latest::prelude::*;
    use polkadot_parachain::primitives::Id as ParaId;

    use crate::distribution::*;

    pub type CurrencyId = u32;
    pub type Balance = u128;

    // use pallet_traits::ump::*;
    // use pallet_xcm_helper::XcmHelper;
    // use primitives::{Balance, CurrencyId, DerivativeIndex, EraIndex, ParaId, Rate, Ratio};

    use super::{types::*, *};
    // use frame_support::traits::fungibles::Transfer;

    pub const MAX_UNLOCKING_CHUNKS: usize = 32;

    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type AssetIdOf<T> =
        <<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
    pub type BalanceOf<T> =
        <<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

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
        type Assets: /* Transfer<Self::AccountId, AssetId = CurrencyId> + */
            Mutate<Self::AccountId, AssetId = CurrencyId, Balance = Balance>
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

        /// The pallet id of loans used for fast unstake
        #[pallet::constant]
        type LoansPalletId: Get<PalletId>;

        /// Returns the parachain ID we are running with.
        #[pallet::constant]
        type SelfParaId: Get<ParaId>;

        /// Derivative index list
        #[pallet::constant]
        type DerivativeIndexList: Get<Vec<DerivativeIndex>>;

        /// Xcm fees
        #[pallet::constant]
        type XcmFees: Get<BalanceOf<Self>>;

        /// Loans instant unstake fee
        #[pallet::constant]
        type LoansInstantUnstakeFee: Get<Rate>;

        /// MatchingPool fast unstake fee
        #[pallet::constant]
        type MatchingPoolFastUnstakeFee: Get<Rate>;

        /// Staking currency
        #[pallet::constant]
        type StakingCurrency: Get<AssetIdOf<Self>>;

        /// Liquid currency
        #[pallet::constant]
        type LiquidCurrency: Get<AssetIdOf<Self>>;

        /// Collateral currency
        #[pallet::constant]
        type CollateralCurrency: Get<AssetIdOf<Self>>;

        /// Minimum stake amount
        #[pallet::constant]
        type MinStake: Get<BalanceOf<Self>>;

        /// Minimum unstake amount
        #[pallet::constant]
        type MinUnstake: Get<BalanceOf<Self>>;

        /// Weight information
        // type WeightInfo: WeightInfo;

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

        // /// Loans
        // type Loans: Loans<AssetIdOf<Self>, Self::AccountId, BalanceOf<Self>>
        //     + LoansPositionDataProvider<AssetIdOf<Self>, Self::AccountId, BalanceOf<Self>>
        //     + LoansMarketDataProvider<AssetIdOf<Self>, BalanceOf<Self>>;

        /// To expose XCM helper functions
        // type XCM: XcmHelper<Self, BalanceOf<Self>, Self::AccountId>;

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
        Bonding(
            DerivativeIndex,
            T::AccountId,
            BalanceOf<T>,
            RewardDestination<T::AccountId>,
        ),
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
    }

    /// The exchange rate between relaychain native asset and the voucher.
    #[pallet::storage]
    #[pallet::getter(fn exchange_rate)]
    pub type ExchangeRate<T: Config> = StorageValue<_, Rate, ValueQuery>;

    /// The commission rate charge for staking total rewards.
    #[pallet::storage]
    #[pallet::getter(fn commission_rate)]
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
    pub type ReserveFactor<T: Config> = StorageValue<_, Ratio, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total_reserves)]
    pub type TotalReserves<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Store total stake amount and unstake amount in each era,
    /// And will update when stake/unstake occurred.
    #[pallet::storage]
    #[pallet::getter(fn matching_pool)]
    pub type MatchingPool<T: Config> = StorageValue<_, MatchingLedger<BalanceOf<T>>, ValueQuery>;

    /// Staking ledger's cap
    #[pallet::storage]
    #[pallet::getter(fn staking_ledger_cap)]
    pub type StakingLedgerCap<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Flying & failed xcm requests
    #[pallet::storage]
    #[pallet::getter(fn xcm_request)]
    pub type XcmRequests<T> = StorageMap<_, Blake2_128Concat, QueryId, XcmRequest<T>, OptionQuery>;

    /// Users' fast unstake requests in liquid currency
    #[pallet::storage]
    #[pallet::getter(fn fast_unstake_requests)]
    pub type FastUnstakeRequests<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    /// Current era index
    /// Users can come to claim their unbonded staking assets back once this value arrived
    /// at certain height decided by `BondingDuration` and `EraLength`
    #[pallet::storage]
    #[pallet::getter(fn current_era)]
    pub type CurrentEra<T: Config> = StorageValue<_, EraIndex, ValueQuery>;

    /// Current era's start relaychain block
    #[pallet::storage]
    #[pallet::getter(fn era_start_block)]
    pub type EraStartBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    /// Unbonding requests to be handled after arriving at target era
    #[pallet::storage]
    #[pallet::getter(fn unlockings)]
    pub type Unlockings<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<UnlockChunk<BalanceOf<T>>>, OptionQuery>;

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
    pub type IsUpdated<T: Config> = StorageMap<_, Twox64Concat, DerivativeIndex, bool, ValueQuery>;

    /// DefaultVersion is using for initialize the StorageVersion
    #[pallet::type_value]
    pub(super) fn DefaultVersion<T: Config>() -> Versions {
        Versions::V2
    }

    /// Storage version of the pallet.
    #[pallet::storage]
    pub(crate) type StorageVersion<T: Config> =
        StorageValue<_, Versions, ValueQuery, DefaultVersion<T>>;

    /// Set to true if already do matching in current era
    /// clear after arriving at next era
    #[pallet::storage]
    #[pallet::getter(fn is_matched)]
    pub type IsMatched<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Incentive for users who successfully update era/ledger
    #[pallet::storage]
    #[pallet::getter(fn incentive)]
    pub type Incentive<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;


    #[pallet::call]
    impl<T: Config> Pallet<T> {

        

    }
}