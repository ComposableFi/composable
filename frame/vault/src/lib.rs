//!
//!
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    missing_docs,
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

mod models;
mod traits;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::models::{StrategyOverview, Vault, VaultConfig};
    use crate::traits;
    use crate::traits::Assets;
    use codec::Codec;
    use frame_support::pallet_prelude::*;
    use frame_support::PalletId;
    use num_traits::SaturatingAdd;
    use sp_runtime::traits::{
        AccountIdConversion, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Convert, StaticLookup,
        Zero,
    };
    use sp_runtime::{Perbill, Perquintill};
    use sp_std::convert::TryInto;

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
        type AssetId: Parameter + Ord + Copy + core::default::Default;
        type Balance: Default
            + Parameter
            + Codec
            + Copy
            + Ord
            + CheckedAdd
            + CheckedSub
            + Zero
            + SaturatingAdd;
        type Assets: traits::Assets<Self::AssetId, Self::Balance, Self::AccountId>;

        type Convert: Convert<Self::Balance, u128> + Convert<u128, Self::Balance>;

        type MaxStrategies: Get<usize>;
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
        StorageMap<_, Twox64Concat, VaultIndex, Vault<T::AssetId, T::Balance>, ValueQuery>;

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
    }

    impl<T: Config> Pallet<T>
    where
        <T as frame_system::Config>::AccountId: core::hash::Hash,
    {
        fn do_create_vault(
            config: VaultConfig<T::AccountId, T::AssetId>,
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
                    T::Assets::create(id).map_err(|e| {
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
            let owned = T::Assets::balance_of(&vault.asset_id, &Self::account_id())
                .ok_or(Error::<T>::InconsistentStorage)?;
            let outstanding = CapitalStructure::<T>::iter_prefix_values(vault_id)
                .fold(T::Balance::zero(), |sum, item| sum + item.withdrawn);
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
                T::Assets::transfer_from(&vault.asset_id, &from, &Self::account_id(), amount)
                    .map_err(|_| Error::<T>::TransferFromFailed)?;
                T::Assets::mint_to(&vault.lp_token_id, &from, amount)
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
                let outstanding = T::Assets::total_supply(&vault.lp_token_id)
                    .ok_or(Error::<T>::InconsistentStorage)?;
                let outstanding = <T::Convert as Convert<T::Balance, u128>>::convert(outstanding);
                let aum = <T::Convert as Convert<T::Balance, u128>>::convert(
                    vault.assets_under_management,
                );
                let lp = (|| deposit.checked_mul(outstanding)?.checked_div(aum))()
                    .ok_or(Error::<T>::NoFreeVaultAllocation)?;
                let lp = <T::Convert as Convert<u128, T::Balance>>::convert(lp);

                T::Assets::transfer_from(&vault.asset_id, &from, &Self::account_id(), amount)
                    .map_err(|_| Error::<T>::TransferFromFailed)?;
                T::Assets::mint_to(&vault.lp_token_id, &from, lp)
                    .map_err(|_| Error::<T>::MintFailed)?;
            }

            Ok(())
        }
    }
}
