//! # VAMM Pallet
//!
//! ## Overview
//!
//! ![](http://www.plantuml.com/plantuml/svg/NSqz2W91343XtbFe0TpqLWk2zyXcuyv09XdoWzTNB2oi7b_rraZqh26dIrUIshbSpYrpnWt0yRKSFLjj5Unacgova0susvWMk0a_Ej0Fm46D7PwEWu64qRiUrsOL_roce7xFA-l-wHi0)
//!
//! TODO
//!
//! ### Terminology
//!
//! * **VAAM**: Acronym for Virtual Automated Market Maker. (TODO: expand definition)
//!
//! TODO
//!
//! ### Goals
//!
//! TODO
//!
//! ### Actors
//!
//! TODO
//!
//! (clearing house)
//!
//! ### Implementations
//!
//! The VAMM pallet provides implementations for the following traits:
//!
//! - [`Vamm`](composable_traits::vamm::Vamm): Exposes functionality for
//! creating, managing and deprecating virtual automated market makers.
//!
//! ## Interface
//!
//! TODO
//!
//! ### Extrinsics
//!
//! TODO
//!
//! not applicable
//!
//! ### Implemented Functions
//!
//! * [`create`](<Pallet as Vamm>::create)
//!
//! TODO
//!
//! ### Public Functions
//!
//! TODO
//!
//! ### Public Storage Objects
//!
//! TODO
//!
//! ## Usage
//!
//! TODO
//!
//! ### Example
//!
//! TODO
//!
//! ## Related Modules
//!
//! TODO
//!
//! (clearing house)
//!
//! <!-- Original author: @Cardosaum -->

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
	use composable_traits::vamm::*;
	use frame_support::{pallet_prelude::*, sp_std::fmt::Debug, transactional, Blake2_128Concat};
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

		/// The `VammId` used by the pallet. Corresponds to the Ids used by the Vamm pallet.
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
			+ Zero;

		/// Timestamp to be used for twap calculations and market deprecation.
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
			+ Zero;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types
	// ----------------------------------------------------------------------------------------------------

	/// Represents the direction a of a position.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum SwapDirection {
		Add,
		Remove,
	}

	/// Data relating to the state of a virtual market.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug)]
	pub struct VammState<Balance, Timestamp> {
		/// The total amount of base asset present in the virtual market.
		base_asset_reserves: Balance,

		/// The total amount of quote asset present in the virtual market.
		quote_asset_reserves: Balance,

		/// The magnitude of the quote asset reserve.
		peg_multiplier: Balance,

		/// Whether this market is deprecated or not.
		///
		/// This variable function as a signal to allow pallets who uses the
		/// VAMM to set a market as "operating as normal" or "not to be used
		/// anymore".  If the value is `None` it means the market is operating
		/// as normal, but if the value is `Some(timestamp)` it means the market
		/// is deprecated and the deprecation will take effect at the time
		/// `timestamp`.
		deprecated: Option<Timestamp>,
	}

	type BalanceOf<T> = <T as Config>::Balance;
	type TimestampOf<T> = <T as Config>::Timestamp;
	type VammIdOf<T> = <T as Config>::VammId;
	type VammStateOf<T> = VammState<BalanceOf<T>, TimestampOf<T>>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	/// The number of virtual markets, also used to generate the next market
	/// identifier.
	///
	/// # Note
	///
	/// Frozen markets do not decrement the counter.
	#[pallet::storage]
	#[pallet::getter(fn vamm_count)]
	#[allow(clippy::disallowed_types)]
	pub type VammCounter<T: Config> = StorageValue<_, T::VammId, ValueQuery>;

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
		/// This error is thrown when the caller tries to set `base_asset_reserves` to zero.
		BaseAssetReserveIsZero,
		/// This error is thrown when the caller tries to set `quote_asset_reserves` to zero.
		QuoteAssetReserveIsZero,
		/// This error is thrown when the caller tries to set `peg_multiplier` to zero.
		PegMultiplierIsZero,
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

		/// Creates a new virtual market.
		///
		/// # Overview
		/// In order for the caller to create new markets, it has to request it
		/// to the VAMM, which is responsible to keep track of all active
		/// markets. The VAMM creates the market, inserts it in storage,
		/// deposits a [`Created`](Event::<T>::Created) event on the
		/// blockchain and returns the new market ID to the caller.
		///
		/// In the diagram below the Clearing House is depicted as the caller.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/TP3FQi904CRl-nHZAbIHWCMRs81_3kt1Ndo2SJ8cYybEkZk9hzziIYeMtDF2z_j-0r-uMjUWnneyXqPSu2E7W0Nlk9BRrdkvWVgMZLgj6BhjyGWr-Yiha6TKAmvIEBL4VGqkVSUyOgk2fBP3PH1d9bfopR4MpAJnGfotdc5wGLlrdzcq3iNS06mkudgjLEBVanUYPV-IR7FE8c0cxFA_yeCd_5v_ubckjlktrJEFQT2h9TjWNqds5QEthe0FQGCdW06eV4JY0aFGOLsR71NF61YIauh7Wc4MWVb0X7_8hXAwKedM3V6PZA4IqcnGBHPhsCT5UTpNytVBGKrC8pNe7g4nJ3FfTMiuS2F1Aj30vAE9EtPt5AXCq_LzjkIBRoFvUKZcvWS0)
		///
		/// ## Parameters:
		/// - `base_asset_reserves`: The amount of base asset
		/// - `quote_asset_reserves`: The amount of quote asset
		/// - `peg_multiplier`: The constant multiplier responsible to balance quote and base asset
		///
		/// ## Returns
		/// The new virtual market id, if successful.
		///
		/// ## Assumptions or Requirements
		/// TODO
		///
		/// ## Emits
		/// * [`Created`](Event::<T>::Created)
		///
		/// ## State Changes
		/// Updates [`Vamms`] storage map and [`VammsCount`] storage value.
		///
		/// ## Errors
		/// * [`ArithmeticError::Overflow`](sp_runtime::ArithmeticError)
		///
		/// # Runtime
		/// `O(1)`

		#[transactional]
		fn create(
			base_asset_reserves: Self::Balance,
			quote_asset_reserves: Self::Balance,
			peg_multiplier: Self::Balance,
		) -> Result<Self::VammId, DispatchError> {
			ensure!(!base_asset_reserves.is_zero(), Error::<T>::BaseAssetReserveIsZero);
			ensure!(!quote_asset_reserves.is_zero(), Error::<T>::QuoteAssetReserveIsZero);
			ensure!(!peg_multiplier.is_zero(), Error::<T>::PegMultiplierIsZero);

			VammCounter::<T>::try_mutate(|id| {
				let old_id = id.clone();
				let vamm_state = VammStateOf::<T> {
					base_asset_reserves,
					quote_asset_reserves,
					peg_multiplier,
					deprecated: Default::default(),
				};

				VammMap::<T>::insert(&old_id, vamm_state);
				*id = id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;

				Self::deposit_event(Event::<T>::Created {
					vamm_id: old_id.clone(),
					state: vamm_state,
				});

				Ok(old_id)
			})
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
