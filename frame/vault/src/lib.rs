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
    use crate::models::{Vault, VaultConfig};
    use crate::traits;
    use crate::traits::Assets;
    use codec::Codec;
    use frame_support::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, CheckedSub};

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
        type Balance: Parameter + Codec + Copy + Ord + CheckedAdd + CheckedSub;
        type Assets: traits::Assets<Self::AssetId, Self::Balance, Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn vault_count)]
    pub type VaultCount<T: Config> = StorageValue<_, VaultIndex, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn accuracy_threshold)]
    pub type Vaults<T: Config> =
        StorageMap<_, Blake2_128Concat, VaultIndex, Vault<T::AssetId>, ValueQuery>;

    /// Key type for the vaults. `VaultIndex` uniquely identifies a vault.
    // TODO: should probably be a new type
    pub type VaultIndex = u64;

    #[pallet::error]
    pub enum Error<T> {
        InsufficientBalance,
        CannotCreateAsset,
    }

    impl<T: Config> Pallet<T> {
        fn do_create_vault(config: VaultConfig) -> Result<VaultIndex, Error<T>> {
            log::info!("Hello from do_create_vault :)");

            // 1. check config
            // 2. lock endowment
            // 3. mint LP token
            // 4. insert vault (do we check if the strategy addresses even exists?)
            VaultCount::<T>::try_mutate(|id| {
                let id = {
                    *id += 1;
                    *id
                };

                let lp_token_id = {
                    T::Assets::create(id).map_err(|e| {
                        log::debug!("failed to create asset: {:?}", e);
                        Error::<T>::CannotCreateAsset
                    })?
                };

                Vaults::<T>::insert(
                    id,
                    Vault {
                        config,
                        lp_token_id,
                    },
                );

                Ok(id)
            })
        }
    }
}
