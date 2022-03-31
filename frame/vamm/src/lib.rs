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
//! * [`pallet::Pallet::create`]
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

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use codec::FullCodec;
	use composable_traits::vamm::Vamm;
	use frame_support::{pallet_prelude::*, transactional, Blake2_128Concat};
	use sp_runtime::{
		traits::{CheckedAdd, One},
		ArithmeticError,
	};
	use std::fmt::Debug;

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

		/// The vamm id type for this pallet.
		type VammId: CheckedAdd
			+ One
			+ Default
			+ FullCodec
			+ MaxEncodedLen
			+ TypeInfo
			+ Clone
			+ PartialEq
			+ Debug;

		/// Timestamp to be used for twap calculations and market deprecation.
		type Timestamp: Default
			+ FullCodec
			+ MaxEncodedLen
			+ TypeInfo
			+ Clone
			+ Copy
			+ PartialEq
			+ Debug;
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
	pub struct VammState<Timestamp> {
		/// The total amount of base asset present in the virtual market.
		base_asset_reserves: u128,

		/// The total amount of quote asset present in the virtual market.
		quote_asset_reserves: u128,

		/// The magnitude of the quote asset reserve.
		peg_multiplier: u128,

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

	type TimestampOf<T> = <T as Config>::Timestamp;
	type VammStateOf<T> = VammState<TimestampOf<T>>;

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
	pub type VammsCount<T: Config> = StorageValue<_, T::VammId, ValueQuery>;

	/// Maps [VammId](Config::VammId) to the corresponding virtual [VammState] specs
	#[pallet::storage]
	#[pallet::getter(fn get_vamm)]
	pub type Vamms<T: Config> = StorageMap<_, Blake2_128Concat, T::VammId, VammStateOf<T>>;

	// ----------------------------------------------------------------------------------------------------
	//                                            Runtime Events
	// ----------------------------------------------------------------------------------------------------

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New vamm successfully created
		VammCreated(VammStateOf<T>),
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		BaseAssetReserveIsZero,
		QuoteAssetReserveIsZero,
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

	// ----------------------------------------------------------------------------------------------------
	//                                           Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Vamm for Pallet<T> {
		type VammId = T::VammId;

		/// Creates a new virtual market.
		///
		/// # Overview
		/// In order for the caller to create new markets, it has to request it
		/// to the VAMM, which is responsible to keep track of all active
		/// markets. The VAMM creates the market, inserts it in storage,
		/// deposits a [`VammCreated`](Event::<T>::VammCreated) event on the
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
		/// * [`VammCreated`](Event::<T>::VammCreated)
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
			base_asset_reserves: u128,
			quote_asset_reserves: u128,
			peg_multiplier: u128,
		) -> Result<Self::VammId, DispatchError> {
			ensure!(base_asset_reserves != 0, Error::<T>::BaseAssetReserveIsZero);
			ensure!(quote_asset_reserves != 0, Error::<T>::QuoteAssetReserveIsZero);
			ensure!(peg_multiplier != 0, Error::<T>::PegMultiplierIsZero);

			VammsCount::<T>::try_mutate(|id| {
				let old_id = id.clone();
				let vamm_state = VammState {
					base_asset_reserves,
					quote_asset_reserves,
					peg_multiplier,
					deprecated: Default::default(),
				};

				Vamms::<T>::insert(&old_id, vamm_state);
				*id = id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;
				Self::deposit_event(Event::<T>::VammCreated(vamm_state));
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

#[cfg(test)]
mod tests {}
