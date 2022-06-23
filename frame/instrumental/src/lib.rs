//! # Instrumental Pallet
//!
//! This pallet will house the logic required by [`Instrumental Finance`](https://www.instrumental.finance/);
//! Instrumental will speak to this pallet through the Mosaic Pallet. This pallet will be
//! responsible for sending assets into their associated vaults and specifying which strategies
//! should be set for each vault.
//!
//! ## Overview
//!
//! The following API functions will be exposed through this pallet:
//!
//! - [`create`](Pallet::create)
//! - [`add_liquidity`](Pallet::add_liquidity)
//! - [`remove_liquidity`](Pallet::remove_liquidity)
//!
//! ### Terminology
//!
//! ### Goals
//!
//! ### Actors
//!
//! - users: Instrumentals users lie in the Ethereum ecosystem and interact (indirectly) with this
//!   pallet through the [`Instrumental Finance`](https://www.instrumental.finance/) frontend.
//!
//! - Instrumental: The Ethereum-native smart contracts provide the core functionality for
//!   Instrumental.
//!
//! - Mosaic Pallet: Instrumental speaks to the Mosaic pallet which then redirects calls to the
//!   Instrumental pallet.
//!
//! - [`Vault Pallet`](../pallet_vault/index.html): Each asset supported by this pallet will have an
//!   underlying vault. Each vault will have an associated strategy that will dictate where those
//!   assets will go in order to earn yield.
//!
//! ### Implementations
//!
//! ## Interface
//!
//! ### Extrinsics
//!
//! - [`create`](Pallet::create): Creates a Cubic vault that is responsible for housing the
//!   specified asset and enforcing its strategy.
//!
//! - [`add_liquidity`](Pallet::add_liquidity): Adds assets to its associated vault.
//!
//! - [`remove_liquidity`](Pallet::remove_liquidity): Removes assets from its associated vault.
//!
//! ### Runtime Storage Objects
//!
//! - [`AssetVault`]: Mapping of an [`AssetId`](Config::AssetId) to the underlying Cubic Vault's
//!   [`VaultId`](Config::VaultId) that is responsible for enforcing the asset's strategy.
//!
//! ## Usage
//!
//! ### Example
//!
//! ## Related Modules
//!
//! - [`Vault Pallet`](../pallet_vault/index.html)

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	clippy::indexing_slicing,
	clippy::panic,
	clippy::todo,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_used
)]
#![cfg_attr(
	test,
	allow(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::panic,
		clippy::unwrap_used,
	)
)]

mod mock;
#[cfg(test)]
mod tests;
mod validation;
mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ---------------------------------------------------------------------------------------------
	//                                     Imports and Dependencies
	// ---------------------------------------------------------------------------------------------

	use std::collections::BTreeMap;

	use codec::{Codec, FullCodec};
	use composable_support::validation::Validated;
	use composable_traits::{
		instrumental::{
			Instrumental, InstrumentalDynamicStrategy, InstrumentalProtocolStrategy,
			InstrumentalVaultConfig,
		},
		vault::{Deposit as Duration, FundsAvailability, StrategicVault, Vault, VaultConfig},
	};
	use frame_support::{pallet_prelude::*, transactional, PalletId};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor};
	use sp_runtime::{
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Zero,
		},
		ArithmeticError, Perquintill,
	};
	use sp_std::fmt::Debug;

	use crate::{
		validation::{ValidateVaultDoesNotExist, ValidateVaultExists},
		weights::WeightInfo,
	};

	// ---------------------------------------------------------------------------------------------
	//                                  Declaration Of The Pallet Type
	// ---------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ---------------------------------------------------------------------------------------------
	//                                           Config Trait
	// ---------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		/// The [`Balance`](Config::Balance) type used by the pallet for bookkeeping.
		type Balance: Default
			+ Parameter
			+ Codec
			+ MaxEncodedLen
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ Zero;

		/// The [`AssetId`](Config::AssetId) used by the pallet. Corresponds to the Ids used by the
		/// Currency pallet.
		type AssetId: FullCodec
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ TypeInfo;

		/// The [`VaultId`](Config::VaultId) used by the pallet. Corresponds to the Ids used by the
		/// Vault pallet.
		type VaultId: Clone + Codec + MaxEncodedLen + Debug + PartialEq + Default + Parameter;

		type Vault: StrategicVault<
			AssetId = Self::AssetId,
			Balance = Self::Balance,
			AccountId = Self::AccountId,
			VaultId = Self::VaultId,
		>;

		type InstrumentalStrategy: InstrumentalDynamicStrategy<AssetId = Self::AssetId, AccountId = Self::AccountId>
			+ InstrumentalProtocolStrategy<
				AssetId = Self::AssetId,
				AccountId = Self::AccountId,
				VaultId = Self::VaultId,
			>;

		/// The id used as the
		/// [`AccountId`](composable_traits::instrumental::Instrumental::AccountId) of the vault.
		/// This should be unique across all pallets to avoid name collisions with other pallets and
		/// vaults.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	// ---------------------------------------------------------------------------------------------
	//                                           Pallet Types
	// ---------------------------------------------------------------------------------------------

	pub type InstrumentalVaultConfigFor<T> =
		InstrumentalVaultConfig<<T as Config>::AssetId, Perquintill>;

	// -------------------------------------------------------------------------------------------------
	//                                          Runtime Storage
	// -------------------------------------------------------------------------------------------------

	/// Stores the [`VaultId`](Config::VaultId) that corresponds to a specific
	/// [`AssetId`](Config::AssetId).
	#[pallet::storage]
	#[pallet::getter(fn asset_vault)]
	pub type AssetVault<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, T::VaultId>;

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Events
	// ---------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after a successful call to the [`create`](Pallet::create) extrinsic.
		Created { vault_id: T::VaultId, config: InstrumentalVaultConfigFor<T> },

		/// Emitted after a successful call to the [`add_liquidity`](Pallet::add_liquidity)
		/// extrinsic.
		AddedLiquidity { asset: T::AssetId, amount: T::Balance },

		/// Emitted after a successful call to the [`remove_liquidity`](Pallet::remove_liquidity)
		/// extrinsic.
		RemovedLiquidity { asset: T::AssetId, amount: T::Balance },
	}

	// ---------------------------------------------------------------------------------------------
	//                                          Runtime Errors
	// ---------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		/// This error is thrown when a vault is trying to be created for an asset that already has
		/// an associated vault.
		VaultAlreadyExists,

		/// This error is thrown when a vault is trying to be created with a `strategies`
		/// [`Perquintill`](sp_runtime::Perquintill) value outside of the range [0, 1].
		InvalidDeployablePercent,

		/// This error is thrown when a user tries to call [`add_liquidity`](Pallet::add_liquidity)
		/// or [`remove_liquidity`](Pallet::remove_liquidity) on an asset that does not have an
		/// associated vault (yet).
		AssetDoesNotHaveAnAssociatedVault,

		/// This error is thrown if a user tries to withdraw an amount of assets that is currently
		/// not held in the specified vault.
		NotEnoughLiquidity,
	}

	// ---------------------------------------------------------------------------------------------
	//                                               Hooks
	// ---------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// ---------------------------------------------------------------------------------------------
	//                                            Extrinsics
	// ---------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create an underlying vault and save a reference to its [`VaultId`](Config::VaultId).
		///
		/// # Overview
		///
		/// ## Parameters
		/// - `origin`: [`Origin`](frame_system::pallet::Config::Origin) type representing the
		///   origin of this dispatch.
		/// - `config`: the [`InstrumentalVaultConfig`] of the underlying vault to create.
		///
		/// ## Requirements
		///
		/// 1. the call must have been signed by the issuer.
		/// 2. [`config.asset_id`](InstrumentalVaultConfig) must not correspond to a
		/// preexisting Instrumental vault.
		///
		/// ## Emits
		///
		/// - [`Event::Created`]
		///
		/// ## State Changes
		///
		/// - [`AssetVault`]: a mapping between the parameter `asset` and the created vault's
		///   [`VaultId`](Config::VaultId) is stored.
		///
		/// ## Errors
		///
		/// - [`VaultAlreadyExists`](Error::VaultAlreadyExists): there already exists an underlying
		///   vault for `asset`.
		///
		/// # Examples
		///
		/// # Weight: O(TBD)
		#[pallet::weight(<T as Config>::WeightInfo::create())]
		pub fn create(
			origin: OriginFor<T>,
			config: InstrumentalVaultConfigFor<T>,
		) -> DispatchResultWithPostInfo {
			// TODO: (Nevin)
			//  - (potentially) enforce that the issuer must have privileged rights

			// Requirement 1) This extrinsic must be signed
			let _from = ensure_signed(origin)?;

			let vault_id = <Self as Instrumental>::create(config)?;
			Self::deposit_event(Event::Created { vault_id, config });

			Ok(().into())
		}

		/// Add assets into its underlying vault.
		///
		/// # Overview
		///
		/// ## Parameters
		///
		/// - `origin`: [`Origin`](frame_system::pallet::Config::Origin) type representing the
		///   origin of this dispatch.
		/// - `asset`: the [`AssetId`](Config::AssetId) of the asset to deposit.
		/// - `amount`: the amount of `asset` to deposit.
		///
		/// ## Requirements
		///
		/// 1. The call must have been signed by the issuer.
		/// 2. There must be a vault associated with `asset`.
		///
		/// ## Emits
		///
		/// - [`Event::AddedLiquidity`]
		///
		/// ## Errors
		///
		/// - [`AssetDoesNotHaveAnAssociatedVault`](Error::AssetDoesNotHaveAnAssociatedVault): no
		///   vault has been created for `asset`.
		///
		/// # Examples
		///
		/// # Weight: O(TBD)
		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			// Requirement 1) This extrinsic must be signed
			let issuer = ensure_signed(origin)?;

			<Self as Instrumental>::add_liquidity(&issuer, &asset, amount)?;

			Self::deposit_event(Event::AddedLiquidity { asset, amount });

			Ok(().into())
		}

		/// Remove assets from its underlying vault.
		///
		/// # Overview
		///
		/// ## Parameters
		///
		/// - `origin`: [`Origin`](frame_system::pallet::Config::Origin) type representing the
		///   origin of this dispatch.
		/// - `asset`: the [`AssetId`](Config::AssetId) of the asset to withdraw.
		/// - `amount`: the amount of `asset` to withdraw.
		///
		/// ## Requirements
		///
		/// 1. The call must have been signed by the issuer.
		/// 2. There must be a vault associated with `asset`.
		///
		/// ## Emits
		///
		/// - [`Event::RemovedLiquidity`]
		///
		/// ## Errors
		///
		/// - [`AssetDoesNotHaveAnAssociatedVault`](Error::AssetDoesNotHaveAnAssociatedVault): no
		///   vault has been created for `asset`.
		///
		/// # Examples
		///
		/// # Weight: O(TBD)
		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		pub fn remove_liquidity(
			origin: OriginFor<T>,
			asset: T::AssetId,
			amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			// Requirement 1) This extrinsic must be signed
			let issuer = ensure_signed(origin)?;

			<Self as Instrumental>::remove_liquidity(&issuer, &asset, amount)?;

			Self::deposit_event(Event::RemovedLiquidity { asset, amount });

			Ok(().into())
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                        Instrumental Trait
	// ---------------------------------------------------------------------------------------------

	impl<T: Config> Instrumental for Pallet<T> {
		type AccountId = T::AccountId;
		type AssetId = T::AssetId;
		type Balance = T::Balance;
		type VaultId = T::VaultId;

		// TODO: (Nevin)
		//  - should each asset have its own account?
		fn account_id() -> Self::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		/// Create an underlying vault and save a reference to its [`VaultId`](Config::VaultId).
		///
		/// # Overview
		///
		/// ## Parameters
		///
		/// - `config`: the [`InstrumentalVaultConfig`] of the underlying vault to create.
		///
		/// ## Requirements
		///
		/// 1. [`config.asset_id`](InstrumentalVaultConfig) must not correspond to a preexisting
		/// Instrumental vault.
		///
		/// ## State Changes
		///
		/// - [`AssetVault`]: a mapping between the parameter `asset` and the created vault's
		///   [`VaultId`](Config::VaultId) is stored.
		///
		/// ## Errors
		///
		/// - [`VaultAlreadyExists`](Error::VaultAlreadyExists): their already exists an underlying
		///   vault for `asset`.
		///
		/// # Runtime: O(TBD)
		fn create(config: InstrumentalVaultConfigFor<T>) -> Result<Self::VaultId, DispatchError> {
			match Validated::new(config) {
				Ok(validated_config) => Self::do_create(validated_config),
				Err(_) => Err(Error::<T>::VaultAlreadyExists.into()),
			}
		}

		/// Add assets into its underlying vault.
		///
		/// # Overview
		///
		/// ## Parameters
		///
		/// - `issuer`: the [`AccountId`](composable_traits::instrumental::Instrumental::AccountId)
		///   of the user who issued the request
		/// - `asset`: the [`AssetId`](composable_traits::instrumental::Instrumental::AssetId) of
		///   the asset to deposit.
		/// - `amount`: the amount of `asset` to deposit.
		///
		/// ## Requirements
		///
		/// 1. There must be a vault associated with `asset`.
		///
		/// ## Errors
		///
		/// - [`AssetDoesNotHaveAnAssociatedVault`](Error::AssetDoesNotHaveAnAssociatedVault): no
		///   vault has been created for `asset`.
		///
		/// # Runtime: O(TBD)
		fn add_liquidity(
			issuer: &Self::AccountId,
			asset: &Self::AssetId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			// Requirement 1) The asset must have an associated vault
			match Validated::new(asset) {
				Ok(validated_asset) => Self::do_add_liquidity(issuer, validated_asset, amount),
				Err(_) => Err(Error::<T>::AssetDoesNotHaveAnAssociatedVault.into()),
			}
		}

		/// Remove assets from its underlying vault.
		///
		/// # Overview
		///
		/// ## Parameters
		/// - `issuer`: the [`AccountId`](composable_traits::instrumental::Instrumental::AccountId)
		///   of the user who issued the request
		/// - `asset`: the [`AssetId`](composable_traits::instrumental::Instrumental::AssetId) of
		///   the asset to withdraw.
		/// - `amount`: the amount of `asset` to withdraw.
		///
		/// ## Requirements
		/// 1. There must be a vault associated with `asset`.
		///
		/// ## Errors
		/// - [`AssetDoesNotHaveAnAssociatedVault`](Error::AssetDoesNotHaveAnAssociatedVault): no
		///   vault has been created for `asset`.
		///
		/// # Runtime: O(TBD)
		fn remove_liquidity(
			issuer: &Self::AccountId,
			asset: &Self::AssetId,
			amount: Self::Balance,
		) -> Result<(), DispatchError> {
			// Requirement 1) The asset must have an associated vault
			match Validated::new(asset) {
				Ok(validated_asset) => Self::do_remove_liquidity(issuer, validated_asset, amount),
				Err(_) => Err(Error::<T>::AssetDoesNotHaveAnAssociatedVault.into()),
			}
		}
	}

	// ---------------------------------------------------------------------------------------------
	//                                      Low Level Functionality
	// ---------------------------------------------------------------------------------------------

	impl<T: Config> Pallet<T> {
		#[transactional]
		fn do_create(
			config: Validated<InstrumentalVaultConfigFor<T>, ValidateVaultDoesNotExist<T>>,
		) -> Result<T::VaultId, DispatchError> {
			// Requirement 1) Obtain each required field for the VaultConfig struct
			let asset_id = config.asset_id;
			let manager = Self::account_id();

			let reserved = Perquintill::one()
				.checked_sub(&config.percent_deployable)
				.ok_or(ArithmeticError::Overflow)?;

			// TODO: (Nevin)
			//  - obtain the optimum strategies account_id
			let strategy_account_id = T::InstrumentalStrategy::get_optimum_strategy_for(asset_id)?;
			let strategies: BTreeMap<T::AccountId, Perquintill> =
				BTreeMap::from([(strategy_account_id, config.percent_deployable)]);

			// Requirement 2) Create the underlying vault
			let vault_id: T::VaultId = T::Vault::create(
				Duration::Existential,
				VaultConfig { asset_id, manager, reserved, strategies },
			)?;

			AssetVault::<T>::insert(asset_id, &vault_id);

			Ok(vault_id)
		}

		#[transactional]
		fn do_add_liquidity(
			issuer: &T::AccountId,
			asset: Validated<&T::AssetId, ValidateVaultExists<T>>,
			amount: T::Balance,
		) -> Result<(), DispatchError> {
			let vault_id: T::VaultId = Self::asset_vault(asset.value())
				.ok_or(Error::<T>::AssetDoesNotHaveAnAssociatedVault)?;

			<T::Vault as StrategicVault>::deposit(&vault_id, issuer, amount)?;

			Ok(())
		}

		#[transactional]
		fn do_remove_liquidity(
			issuer: &T::AccountId,
			asset: Validated<&T::AssetId, ValidateVaultExists<T>>,
			amount: T::Balance,
		) -> Result<(), DispatchError> {
			let vault_id: T::VaultId = Self::asset_vault(asset.value())
				.ok_or(Error::<T>::AssetDoesNotHaveAnAssociatedVault)?;

			// TODO: (Nevin)
			//  - this can be done in a better way
			let vault_account = T::Vault::account_id(&vault_id);
			match <T::Vault as StrategicVault>::available_funds(&vault_id, &vault_account)? {
				FundsAvailability::Withdrawable(balance) if balance >= amount =>
					<T::Vault as StrategicVault>::withdraw(&vault_id, issuer, amount),
				FundsAvailability::MustLiquidate =>
					<T::Vault as StrategicVault>::withdraw(&vault_id, issuer, amount),
				_ => Err(Error::<T>::NotEnoughLiquidity.into()),
			}
			.map_err(|_| Error::<T>::NotEnoughLiquidity)?;

			Ok(())
		}
	}
}

// -------------------------------------------------------------------------------------------------
//                                            Unit Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {}
