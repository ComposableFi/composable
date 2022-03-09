#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies
	// ----------------------------------------------------------------------------------------------------

	use codec::FullCodec;
	use composable_traits::defi::DeFiComposableConfig;
	use frame_support::{pallet_prelude::*, Blake2_128Concat, Twox64Concat};
	use frame_system::pallet_prelude::OriginFor;
	use sp_runtime::FixedPointNumber;

	// ----------------------------------------------------------------------------------------------------
	//                                    Declaration Of The Pallet Type
	// ----------------------------------------------------------------------------------------------------

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// ----------------------------------------------------------------------------------------------------
	//                                             Config Trait
	// ----------------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: DeFiComposableConfig + frame_system::Config {
		// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The market ID type for this pallet.
		type MarketId: FullCodec + MaxEncodedLen + TypeInfo;
		/// Signed decimal fixed point number.
		type Decimal: FullCodec + MaxEncodedLen + TypeInfo + FixedPointNumber;
		/// Timestamp to be used for funding rate updates
		type Timestamp: FullCodec + MaxEncodedLen + TypeInfo;
		/// Duration type for funding rate periodicity
		type Duration: FullCodec + MaxEncodedLen + TypeInfo;
		/// The virtual AMM ID type for this pallet. `pallet-virtual-amm` should implement a trait
		/// VAMM with an associated type 'VAMMId' compatible with this one.
		type VAMMId: FullCodec + MaxEncodedLen + TypeInfo;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types
	// ----------------------------------------------------------------------------------------------------

	/// Stores the user's position in a particular market
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Position<MarketId, Decimal> {
		market_id: MarketId,
		base_asset_amount: Decimal,
		quote_asset_notional_amount: Decimal,
		last_cum_funding: Decimal,
	}

	/// Data relating to a perpetual contracts market
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub struct Market<AssetId, Decimal, Duration, Timestamp, VAMMId> {
		vamm_id: VAMMId,
		asset_id: AssetId,
		cum_funding_rate: Decimal,
		funding_rate_ts: Timestamp,
		periodicity: Duration,
	}

	pub type AssetIdOf<T> = <T as DeFiComposableConfig>::MayBeAssetId;
	pub type MarketIdOf<T> = <T as Config>::MarketId;
	pub type DecimalOf<T> = <T as Config>::Decimal;
	pub type TimestampOf<T> = <T as Config>::Timestamp;
	pub type DurationOf<T> = <T as Config>::Duration;
	pub type VAMMIdOf<T> = <T as Config>::VAMMId;
	pub type PositionOf<T> = Position<MarketIdOf<T>, DecimalOf<T>>;
	pub type MarketOf<T> =
		Market<AssetIdOf<T>, DecimalOf<T>, DurationOf<T>, TimestampOf<T>, VAMMIdOf<T>>;

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Storage
	// ----------------------------------------------------------------------------------------------------

	#[pallet::storage]
	#[pallet::getter(fn get_initial_margin_ratio)]
	#[allow(clippy::disallowed_types)]
	/// Minimum margin ratio for opening a new position
	type InitialMarginRatio<T: Config> = StorageValue<_, T::Decimal, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_maintenance_margin_ratio)]
	#[allow(clippy::disallowed_types)]
	/// Minimum margin ratio, below which liquidations can occur
	type MaintenanceMarginRatio<T: Config> = StorageValue<_, T::Decimal, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_margin)]
	pub type AccountsMargin<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance>;

	#[pallet::storage]
	#[pallet::getter(fn get_position)]
	pub type Positions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Twox64Concat,
		T::MarketId,
		PositionOf<T>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_market)]
	pub type Markets<T: Config> = StorageMap<_, Twox64Concat, T::MarketId, MarketOf<T>>;

	// ----------------------------------------------------------------------------------------------------
	//                                            Runtime Events
	// ----------------------------------------------------------------------------------------------------

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// ----------------------------------------------------------------------------------------------------
	//                              Extrinsics
	// ----------------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds margin to a user's account. A user has to have enough margin to open new positions
		/// and can be liquidated if its margin ratio falls bellow maintenance. Deposited collateral
		/// backs all the positions of an account accross multiple markets (cross-margining).
		///
		/// If an account does not exist in `AccountsMargin`, it is created and initialized with 0
		/// margin. Checks that the collateral type is supported.
		#[pallet::weight(0)]
		#[allow(unused_variables)]
		pub fn add_margin(
			origin: OriginFor<T>,
			asset: AssetIdOf<T>,
			amount: T::Balance,
		) -> DispatchResult {
			Ok(())
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                              Trait Implementations
	// ----------------------------------------------------------------------------------------------------

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
