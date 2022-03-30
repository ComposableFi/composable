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
//! TODO
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
//! * [create_market](pallet::create_market)
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

	use frame_support::pallet_prelude::*;
	// use frame_system::{ensure_signed, pallet_prelude::*};

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
		#[allow(missing_docs)]
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types
	// ----------------------------------------------------------------------------------------------------

	pub type VammId = u128;

	// ----------------------------------------------------------------------------------------------------
	//                                            Runtime Events
	// ----------------------------------------------------------------------------------------------------

	#[pallet::event]
	pub enum Event<T: Config> {
		MarketCreated,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		MarketAlreadyExists,
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	// ----------------------------------------------------------------------------------------------------
	//                                                Hooks
	// ----------------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// ----------------------------------------------------------------------------------------------------
	//                                              Extrinsics
	// ----------------------------------------------------------------------------------------------------

	// ----------------------------------------------------------------------------------------------------
	//                              Trait Implementations
	// ----------------------------------------------------------------------------------------------------

	/// Creates a new virtual market.
	///
	/// # Overview
	/// In order for the caller to create new markets, it has to request it to
	/// the VAMM, which is responsible to keep track of all active markets. The
	/// VAMM creates the market, inserts it in storage, deposits an
	/// `MarketCreated` event on the blockchain and returns the new market ID to
	/// the caller.
	///
	/// In the diagram below the Clearing House is depicted as the caller.
	///
	/// ![](https://www.plantuml.com/plantuml/svg/TP7TQi9048Nlzod6LAWY0OjtOWL_eEs5tlW4ayqaMKnsqfrDVFrcDR4ecAl0-RupS5REnjRei097KhCLEf08vhIbaYyRv_W2tZMalF4bGRiv7A12ToOFsX5wmnPYE3LQTnfadQFUiV6CsfiMFG5RDV85LHGqKQFQiQQqSGJZ34tP_Kp6bUHJ1R3As7QrNBBxZDwqddTVj5ubwm2e4TdqBZ_qoEtwYvxvOhFxMBpgKJKwRxIfiLEYNKdeZCtXcnkX25105uz8ME0Qj5XLAaMF2Gf1TFBiC2UO161w9UG9sPQipxf0OwznefNI1SHh2DnijeHTM9I6VHtzDYHUXJHGCc1Vv7j6oItiE-9u2KVsa75ZqqVlgLmD7aEYbx2dciNqJqff3m6rPF_HRefmu3vtTAJBGFxiewdw0m00)
	///
	/// ## Parameters:
	/// - `base_asset_amount`: The amount of base asset
	/// - `quote_asset_amount`: The amount of quote asset
	/// - `peg_multiplier`: The constant multiplier responsible to balance quote and base asset
	///
	/// ## Assumptions or Requirements
	/// TODO
	///
	/// ## Emits
	/// * [`MarketCreated`](Event::<T>::MarketCreated)
	///
	/// ## State Changes
	/// Updates the [`AccountsMargin`] storage map. If an account does not exist in
	/// [`AccountsMargin`], it is created and initialized with 0 margin.
	///
	/// ## Errors
	/// * [`MarketAlreadyExists`](Error::<T>::MarketAlreadyExists)
	///
	/// # Weight/Runtime
	/// `O(1)`

	pub fn create_market(
		_base_asset_amount: u128,
		_quote_asset_amount: u128,
		_peg_multiplier: u128,
	) -> VammId {
		unimplemented!()
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
