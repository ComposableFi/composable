//! # VAMM Pallet
//!
//! The VAMM Pallet provides functionality to manage virtual automated market makers.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The VAMM Pallet allows other Pallets to leverage it's functions in order to
//! manage virtual automated market makers, abstracting away complexity. It's
//! important to note that currently just one type of constant function market
//! maker is supported, namely the `x * y = k`.
//!
//! Below is a diagram showing how the trait and runtime storage looks like and
//! interact with each other:
//!
//! ![](https://www.plantuml.com/plantuml/svg/hLD1pzCm3BtdLvXT0EdR9y49LQNI92uS9Wv3N2kJcYvQisQssnxG-EqqxIZjEWaXVJbKYUtt4Z-_lj8ZUGAFBCO4j2Si2JO1gufqsphM1gijUh-1dmRwvSvA_0DjN_Hj2AD0k_CUqbGPdMRPhJ2kNs2PKEdDTnJAKOGqHrytPqsWUVV-mnDScbeVPmALkMygTQ5on6FqxOoveC1a8tcBtYSGN_EvU8BkIES4lZfFekZ375AIve6TlOSCru_7NTpUOxJ3i83C2wHFvd_xkwDUzbGu9gkkxXzuw66V_XnNV56MboBqTKiFs_xleQnOjNv8hCYJr7FaTVWMg1YlXasxs-_Xe3LZIPkPRJo6qLqosKiWJM-LUmmBayKrtWmViobwdNQsGXb93efAFP4eDrPN9F6XSr6OXBWbMHT5WVvT9HVM_BIEBvszpdlaq-2vHUfBY8DD9sDdu2IVVe9YqNZyNeqnhwF29hOEswBkpjli9cO27JibCvwUdzcLHyrc8YLn2F8R)
//!
//! ### Terminology
//!
//! * **VAMM:** Acronym for Virtual Automated Market Maker.
//! * **CFMM:** Acronym for Constant Function Market Maker.
//!
//! ### Goals
//!
//! ### Actors
//!
//! ### Implementations
//!
//! The VAMM Pallet provides implementations for the following traits:
//!
//! - [`Vamm`](composable_traits::vamm::Vamm): Exposes functionality for
//! creating, managing and closing virtual automated market makers.
//!
//! ## Interface
//!
//! ### Extrinsics
//!
//! The current implementation doesn't deal with external calls to the pallet,
//! so there is no extrisic defined.
//!
//! ### Public Functions
//!
//! * [`create`](pallet/struct.Pallet.html#method.create): Creates a new vamm,
//! returning it's Id.
//!
//! ### Runtime Storage Objects
//!
//! - [`VammCounter`](VammCounter): The number of created vamms.
//! - [`VammMap`](VammMap): Mapping of a [VammId](Config::VammId) to it's
//! corresponding [VammState].
//!
//! ## Usage
//!
//! ### Example
//!
//! ## Related Modules
//!
//! - [`Clearing House Pallet`](../clearing_house/index.html)
//!
//! <!-- Original author: @Cardosaum -->
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic
	)
)] // allow in tests
#![warn(clippy::unseparated_literal_suffix)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
	dead_code,
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

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use codec::{Codec, FullCodec};
	use composable_traits::vamm::{AssetType, Vamm, VammConfig};
	use frame_support::{pallet_prelude::*, sp_std::fmt::Debug, transactional, Blake2_128Concat};
	use sp_arithmetic::traits::Unsigned;
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero},
		ArithmeticError,
	};

	// ----------------------------------------------------------------------------------------------------
	//                                    Declaration Of The Pallet Type
	// ----------------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ----------------------------------------------------------------------------------------------------
	//                                             Config Trait
	// ----------------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Event type emitted by this pallet. Depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Ids used by the pallet to index each virtual automated market maker created.
		type VammId: Default
			+ CheckedAdd
			+ Clone
			+ Copy
			+ Debug
			+ FullCodec
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ One
			+ Parameter
			+ PartialEq
			+ TypeInfo
			+ Unsigned
			+ Zero;

		/// Timestamp to be used for twap calculations and market closing.
		type Timestamp: Default
			+ Clone
			+ Copy
			+ Debug
			+ FullCodec
			+ MaxEncodedLen
			+ PartialEq
			+ TypeInfo;

		/// The Balance type used by the pallet for bookkeeping. `Config::Convert` is used for
		/// conversions to `u128`, which are used in the computations.
		type Balance: Default
			+ AtLeast32BitUnsigned
			+ CheckedAdd
			+ CheckedDiv
			+ CheckedMul
			+ CheckedSub
			+ Codec
			+ Copy
			+ MaxEncodedLen
			+ Ord
			+ Parameter
			+ Unsigned
			+ Zero;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types
	// ----------------------------------------------------------------------------------------------------

	type BalanceOf<T> = <T as Config>::Balance;
	type TimestampOf<T> = <T as Config>::Timestamp;
	type VammIdOf<T> = <T as Config>::VammId;
	type VammStateOf<T> = VammState<BalanceOf<T>, TimestampOf<T>>;

	/// Represents the direction a of a position.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum SwapDirection {
		Add,
		Remove,
	}

	/// Data relating to the state of a virtual market.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug)]
	pub struct VammState<Balance, Timestamp> {
		/// The total amount of base asset present in the vamm.
		pub base_asset_reserves: Balance,

		/// The total amount of quote asset present in the vamm.
		pub quote_asset_reserves: Balance,

		/// The magnitude of the quote asset reserve.
		pub peg_multiplier: Balance,

		/// Whether this market is closed or not.
		///
		/// This variable function as a signal to allow pallets who uses the
		/// Vamm to set a market as "operating as normal" or "not to be used
		/// anymore".  If the value is `None` it means the market is operating
		/// as normal, but if the value is `Some(timestamp)` it means the market
		/// is flaged to be closed and the closing action will take (or took)
		/// effect at the time `timestamp`.
		pub closed: Option<Timestamp>,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	/// The number of created vamms, also used to generate the next market
	/// identifier.
	///
	/// # Note
	///
	/// Frozen markets do not decrement the counter.
	#[pallet::storage]
	#[pallet::getter(fn vamm_count)]
	#[allow(clippy::disallowed_types)]
	pub type VammCounter<T: Config> = StorageValue<_, VammIdOf<T>, ValueQuery>;

	/// Maps [VammId](Config::VammId) to the corresponding virtual
	/// [VammState] specs
	#[pallet::storage]
	#[pallet::getter(fn get_vamm)]
	pub type VammMap<T: Config> = StorageMap<_, Blake2_128Concat, VammIdOf<T>, VammStateOf<T>>;

	// ----------------------------------------------------------------------------------------------------
	//                                            Runtime Events
	// ----------------------------------------------------------------------------------------------------

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted after a successful call to the [`create`](Pallet::create) function.
		Created { vamm_id: VammIdOf<T>, state: VammStateOf<T> },
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Tried to set `base_asset_reserves` to zero.
		BaseAssetReserveIsZero,
		/// Tried to set `quote_asset_reserves` to zero.
		QuoteAssetReserveIsZero,
		/// Tried to set `peg_multiplier` to zero.
		PegMultiplierIsZero,
		/// Tried to access an invalid [VammId](Config::VammId).
		VammDoesNotExist,
		/// Tried to retrieve a Vamm but the function failed.
		FailToRetrieveVamm,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                                Hooks
	// ----------------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// ----------------------------------------------------------------------------------------------------
	//                                         Genesis Configuration
	// ----------------------------------------------------------------------------------------------------

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub vamm_count: VammIdOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { vamm_count: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			VammCounter::<T>::put(self.vamm_count);
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Vamm Trait
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Vamm for Pallet<T> {
		type VammId = VammIdOf<T>;
		type Balance = BalanceOf<T>;

		/// Creates a new virtual automated market maker.
		///
		/// # Overview
		/// In order for the caller to create new vamms, it has to request it to
		/// the Vamm Pallet, which is responsible to keep track of and update
		/// when requested all active virtual automated market makers. The Vamm
		/// Pallet creates a new vamm, inserts it into storage, deposits a
		/// [`Created`](Event::<T>::Created) event on the blockchain and returns
		/// the new [`VammId`](Config::VammId) to the caller.
		///
		/// In the diagram below the [`Clearing House
		/// Pallet`](../../clearing_house/index.html) is depicted as the caller.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/TP3FQi904CRl-nHZAbIHWCMRs81_3kt1Ndo2SJ8cYybEkZk9hzziIYeMtDF2z_j-0r-uMjUWnneyXqPSu2E7W0Nlk9BRrdkvWVgMZLgj6BhjyGWr-Yiha6TKAmvIEBL4VGqkVSUyOgk2fBP3PH1d9bfopR4MpAJnGfotdc5wGLlrdzcq3iNS06mkudgjLEBVanUYPV-IR7FE8c0cxFA_yeCd_5v_ubckjlktrJEFQT2h9TjWNqds5QEthe0FQGCdW06eV4JY0aFGOLsR71NF61YIauh7Wc4MWVb0X7_8hXAwKedM3V6PZA4IqcnGBHPhsCT5UTpNytVBGKrC8pNe7g4nJ3FfTMiuS2F1Aj30vAE9EtPt5AXCq_LzjkIBRoFvUKZcvWS0)
		///
		/// ## Parameters:
		/// - `base_asset_reserves`: The amount of base asset
		/// - `quote_asset_reserves`: The amount of quote asset
		/// - `peg_multiplier`: The constant multiplier responsible to balance quote and base asset
		///
		/// ## Returns
		/// The new vamm's id, if successful.
		///
		/// ## Assumptions or Requirements
		/// In order to create a valid vamm, we need to ensure that both base and quote asset
		/// reserves, as well as the peg_multiplier, are non-zero. Every parameter must be greater
		/// than zero.
		///
		/// ## Emits
		/// * [`Created`](Event::<T>::Created)
		///
		/// ## State Changes
		/// Updates [`VammMap`] storage map and [`VammCounter`] storage value.
		///
		/// ## Errors
		/// * [`ArithmeticError::Overflow`](sp_runtime::ArithmeticError)
		///
		/// # Runtime
		/// `O(1)`
		#[transactional]
		fn create(config: VammConfig<BalanceOf<T>>) -> Result<Self::VammId, DispatchError> {
			// TODO: (Matheus)
			// How to ensure that the caller has the right privileges?
			// (eg. How to ensure the caller is the Clearing House, and not anyone else?)
			ensure!(!config.base_asset_reserves.is_zero(), Error::<T>::BaseAssetReserveIsZero);
			ensure!(!config.quote_asset_reserves.is_zero(), Error::<T>::QuoteAssetReserveIsZero);
			ensure!(!config.peg_multiplier.is_zero(), Error::<T>::PegMultiplierIsZero);

			VammCounter::<T>::try_mutate(|next_id| {
				let id = *next_id;
				let vamm_state = VammStateOf::<T> {
					base_asset_reserves: config.base_asset_reserves,
					quote_asset_reserves: config.quote_asset_reserves,
					peg_multiplier: config.peg_multiplier,
					closed: Default::default(),
				};

				VammMap::<T>::insert(&id, vamm_state);
				*next_id = id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;

				Self::deposit_event(Event::<T>::Created { vamm_id: id, state: vamm_state });

				Ok(id)
			})
		}

		/// Get the current price of asset in vamm. One can query either quote or base asset.
		fn get_price(
			vamm_id: VammIdOf<T>,
			asset_type: AssetType,
		) -> Result<BalanceOf<T>, DispatchError> {
			// Requested vamm must exist.
			ensure!(VammMap::<T>::contains_key(vamm_id), Error::<T>::VammDoesNotExist);

			let vamm_state = VammMap::<T>::get(vamm_id).ok_or(Error::<T>::FailToRetrieveVamm)?;

			match asset_type {
				AssetType::BaseAsset => Ok(vamm_state
					.base_asset_reserves
					.checked_mul(&vamm_state.peg_multiplier)
					.ok_or(ArithmeticError::Overflow)?
					.checked_div(&vamm_state.quote_asset_reserves)
					.ok_or(ArithmeticError::Overflow)?),

				AssetType::QuoteAsset => Ok(vamm_state
					.quote_asset_reserves
					.checked_mul(&vamm_state.peg_multiplier)
					.ok_or(ArithmeticError::Overflow)?
					.checked_div(&vamm_state.base_asset_reserves)
					.ok_or(ArithmeticError::Overflow)?),
			}
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                              Helper Functions
	// ----------------------------------------------------------------------------------------------------

	// Helper functions - core functionality
	impl<T: Config> Pallet<T> {}

	// Helper functions - validity checks
	impl<T: Config> Pallet<T> {}

	// Helper functions - low-level functionality
	impl<T: Config> Pallet<T> {}
}

// ----------------------------------------------------------------------------------------------------
//                                              Unit Tests
// ----------------------------------------------------------------------------------------------------
