//!
//!
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    bad_style,
    bare_trait_objects,
    const_err,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates
)]
// TODO remove me!
#![allow(missing_docs)]

mod models;
mod traits;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::models::{StrategyOverview, Vault, VaultConfig};
    use crate::traits::{
        self, CurrencyFactory, FundsAvailability, ReportableStrategicVault, StrategicVault,
    };
    use codec::{Codec, FullCodec};
    use frame_support::pallet_prelude::*;
    use frame_support::PalletId;
    use num_traits::SaturatingSub;
    use orml_traits::MultiCurrency;
    use sp_runtime::traits::{
        AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
        StaticLookup, Zero,
    };
    use sp_runtime::Perquintill;
    use sp_std::fmt::Debug;

    // TODO(kaiserkarel) name this better
    pub const PALLET_ID: PalletId = PalletId(*b"Vaults!!");

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emitted after a vault has been succesfully created.
        VaultCreated,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Balance: Default
            + Parameter
            + Codec
            + Copy
            + Ord
            + CheckedAdd
            + CheckedSub
            + CheckedMul
            + SaturatingSub
            + AtLeast32BitUnsigned
            + Zero
            + From<u64>;
        type CurrencyFactory: CurrencyFactory<Self::CurrencyId>;
        type CurrencyId: FullCodec
            + Eq
            + PartialEq
            + Copy
            + MaybeSerializeDeserialize
            + Debug
            + Default;
        type Currency: MultiCurrency<
            Self::AccountId,
            Balance = Self::Balance,
            CurrencyId = Self::CurrencyId,
        >;
        type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;
        type MaxStrategies: Get<usize>;
        type StrategyReport: FullCodec + Default;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn vault_count)]
    pub type VaultCount<T: Config> = StorageValue<_, VaultIndex, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn vault_data)]
    pub type Vaults<T: Config> =
        StorageMap<_, Twox64Concat, VaultIndex, Vault<T::CurrencyId, T::Balance>, ValueQuery>;

    /// Amounts which each strategy is allowed to access, including the amount reserved for quick
    /// withdrawals for the pallet.
    #[pallet::storage]
    #[pallet::getter(fn allocations)]
    pub type Allocations<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        VaultIndex,
        Blake2_128Concat,
        T::AccountId,
        Perquintill,
        ValueQuery,
    >;

    /// Overview of the balances at each strategy. Does not contain the balance held by the vault
    /// itself.
    #[pallet::storage]
    #[pallet::getter(fn capital_structure)]
    pub type CapitalStructure<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        VaultIndex,
        Blake2_128Concat,
        T::AccountId,
        StrategyOverview<T::Balance>,
        ValueQuery,
    >;

    /// Key type for the vaults. `VaultIndex` uniquely identifies a vault.
    // TODO: should probably be a new type
    pub type VaultIndex = u64;

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        CannotCreateAsset,
        InconsistentStorage,
        TransferFromFailed,
        MintFailed,
        LookupError,
        VaultDoesNotExist,
        NoFreeVaultAllocation,
        AllocationMustSumToOne,
        TooManyStrategies,
        OverflowError,
        InsufficientFunds,
    }

    impl<T: Config> Pallet<T>
    where
        <T as frame_system::Config>::AccountId: core::hash::Hash,
    {
        fn do_create_vault(
            config: VaultConfig<T::AccountId, T::CurrencyId>,
        ) -> Result<VaultIndex, Error<T>> {
            // 1. check config
            // 2. lock endowment
            // 3. mint LP token
            // 4. insert vault (do we check if the strategy addresses even exists?)
            VaultCount::<T>::try_mutate(|id| {
                let id = {
                    *id += 1;
                    *id
                };

                // Perhaps later on, we'll make this configurable per creator account id, if we want
                // to allow special projects to create more complex vaults.
                ensure!(
                    config.strategies.len() <= T::MaxStrategies::get(),
                    Error::<T>::TooManyStrategies
                );

                // We do allow vaults without strategies, since strategies can be decided on later
                // through governance. If strategies are present, their allocations must sum up to 1.
                let sum = config
                    .strategies
                    .iter()
                    .fold(
                        Some(config.reserved.deconstruct()),
                        |sum, (_, allocation)| {
                            sum.map(|sum| sum.checked_add(allocation.deconstruct()))
                                .flatten()
                        },
                    )
                    .ok_or(Error::<T>::AllocationMustSumToOne)?;

                ensure!(
                    sum == Perquintill::one().deconstruct(),
                    Error::<T>::AllocationMustSumToOne
                );

                let lp_token_id = {
                    T::CurrencyFactory::create().map_err(|e| {
                        log::debug!("failed to create asset: {:?}", e);
                        Error::<T>::CannotCreateAsset
                    })?
                };

                config
                    .strategies
                    .into_iter()
                    .for_each(|(account_id, allocation)| {
                        Allocations::<T>::insert(id, account_id, allocation);
                    });

                Allocations::<T>::insert(id, Self::account_id(), config.reserved);

                Vaults::<T>::insert(
                    id,
                    Vault {
                        lp_token_id,
                        ..Default::default()
                    },
                );

                Ok(id)
            })
        }

        fn account_id() -> T::AccountId {
            PALLET_ID.into_account()
        }

        /// Computes the sum of all the assets that the vault currently controls.
        fn assets_under_management(vault_id: VaultIndex) -> Result<T::Balance, Error<T>> {
            let vault =
                Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
            let owned = T::Currency::total_balance(vault.asset_id, &Self::account_id());
            let outstanding = CapitalStructure::<T>::iter_prefix_values(vault_id)
                .fold(T::Balance::zero(), |sum, item| sum + item.balance);
            Ok(owned + outstanding)
        }

        fn do_deposit(
            vault: VaultIndex,
            from: <T::Lookup as StaticLookup>::Source,
            amount: T::Balance,
        ) -> Result<(), Error<T>> {
            let from = T::Lookup::lookup(from).map_err(|_| Error::<T>::LookupError)?;
            let vault = Vaults::<T>::try_get(&vault).map_err(|_| Error::<T>::VaultDoesNotExist)?;

            if vault.assets_under_management.is_zero() {
                // No assets in the vault means we should have no outstanding LP tokens, we can thus
                // freely mint new tokens without performing the calculation.
                T::Currency::transfer(vault.asset_id, &from, &Self::account_id(), amount)
                    .map_err(|_| Error::<T>::TransferFromFailed)?;
                T::Currency::deposit(vault.lp_token_id, &from, amount)
                    .map_err(|_| Error::<T>::MintFailed)?;
            } else {
                // Compute how much of the underlying assets are deposited. LP tokens are allocated such
                // that, if the deposit is N% of the `aum`, N% of the LP token supply are minted to
                // the depositor.
                //
                // TODO(kaiserkarel): Get this reviewed, integer arithmetic is hard after all.
                // reference:
                // https://medium.com/coinmonks/programming-defi-uniswap-part-2-13a6428bf892
                let deposit = <T::Convert as Convert<T::Balance, u128>>::convert(amount);
                let outstanding = T::Currency::total_issuance(vault.lp_token_id);
                let outstanding = <T::Convert as Convert<T::Balance, u128>>::convert(outstanding);
                let aum = <T::Convert as Convert<T::Balance, u128>>::convert(
                    vault.assets_under_management,
                );
                let lp = (|| deposit.checked_mul(outstanding)?.checked_div(aum))()
                    .ok_or(Error::<T>::NoFreeVaultAllocation)?;
                let lp = <T::Convert as Convert<u128, T::Balance>>::convert(lp);

                T::Currency::transfer(vault.asset_id, &from, &Self::account_id(), amount)
                    .map_err(|_| Error::<T>::TransferFromFailed)?;
                T::Currency::deposit(vault.lp_token_id, &from, lp)
                    .map_err(|_| Error::<T>::MintFailed)?;
            }
            Ok(())
        }
    }

    impl<T: Config> StrategicVault for Pallet<T>
    where
        <T as frame_system::Config>::AccountId: core::hash::Hash,
    {
        type AccountId = T::AccountId;
        type Balance = T::Balance;
        type Error = Error<T>;
        type VaultId = VaultIndex;

        fn available_funds(
            vault_id: &Self::VaultId,
            account: &Self::AccountId,
        ) -> Result<FundsAvailability<Self::Balance>, Self::Error> {
            let allocation = match Allocations::<T>::try_get(vault_id, &account) {
                Ok(allocation) => allocation,
                // The strategy was removed by the fund manager or governance, so all funds should be
                // returned.
                Err(_) => return Ok(FundsAvailability::MustLiquidate),
            };

            let aum = Self::assets_under_management(*vault_id)?;
            let max_allowed = allocation.mul_floor(aum);

            // if a strategy has an allocation, it must have an associated capital structure too.
            let state = CapitalStructure::<T>::try_get(vault_id, &account)
                .map_err(|_| Error::<T>::InconsistentStorage)?;
            if state.balance >= max_allowed {
                Ok(FundsAvailability::Depositable(state.balance - max_allowed))
            } else {
                Ok(FundsAvailability::Withdrawable(max_allowed - state.balance))
            }
        }

        fn withdraw(
            vault_id: &Self::VaultId,
            to: &Self::AccountId,
            amount: Self::Balance,
        ) -> Result<(), Self::Error> {
            // TODO: should we check the allocation here? Pallets are technically trusted, so it would
            // only add unnecessary overhead. The extrinsic/ChainExtension interface should check however
            let vault =
                Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
            CapitalStructure::<T>::try_mutate(vault_id, to, |state| {
                // I do not thing balance can actually overflow, since the total_issuance <= T::Balance::Max
                state
                    .balance
                    .checked_add(&amount)
                    .ok_or(Error::<T>::OverflowError)?;
                // This can definitely overflow. Perhaps it should be a BigUint?
                state
                    .lifetime_withdrawn
                    .checked_add(&amount)
                    .ok_or(Error::<T>::OverflowError)?;
                T::Currency::transfer(vault.asset_id, &Self::account_id(), to, amount)
                    .map_err(|_| Error::<T>::InsufficientFunds)
            })?;
            Ok(())
        }

        fn deposit(
            vault_id: &Self::VaultId,
            from: &Self::AccountId,
            amount: Self::Balance,
        ) -> Result<(), Self::Error> {
            let vault =
                Vaults::<T>::try_get(&vault_id).map_err(|_| Error::<T>::VaultDoesNotExist)?;
            CapitalStructure::<T>::try_mutate(vault_id, from, |state| {
                // A strategy can return more than it has withdrawn through profits.
                state.balance.saturating_sub(&amount);
                // This can definitely overflow. Perhaps it should be a BigUint?
                state
                    .lifetime_deposited
                    .checked_add(&amount)
                    .ok_or(Error::<T>::OverflowError)?;
                T::Currency::transfer(vault.asset_id, from, &Self::account_id(), amount)
                    .map_err(|_| Error::<T>::InsufficientFunds)
            })?;
            Ok(())
        }
    }

    impl<T: Config> ReportableStrategicVault for Pallet<T>
    where
        <T as frame_system::Config>::AccountId: core::hash::Hash,
    {
        type Report = T::Balance;

        fn update_strategy_report(
            vault: &Self::VaultId,
            strategy: &Self::AccountId,
            report: &Self::Report,
        ) -> Result<(), Self::Error> {
            CapitalStructure::<T>::mutate(vault, strategy, |state| state.balance = *report);
            Ok(())
        }
    }
}
