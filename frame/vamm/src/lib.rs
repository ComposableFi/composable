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
//! ![](https://www.plantuml.com/plantuml/svg/ZLJDZjCm4BxdAKnFYzJk0qGXscMvS408kk8Q3SbiQX7_u9cqGgWyEx4JR6zJfFgGsZFV_7_J1s9mFAgXUCC7L2WKE2eA2-qFw55iB0m3yku8Ict4xq9CHsf6zm8jYc-JL5GLEv1Srrwzd3-YTGYCTwtHBx8l0_8ftD_ceC4GtddVC-9ZjnMd0-fIF4k5nA1i3k-H6-jaEviqiajMG8HSYaV_y_pBugKPdy2-2fG3Q5B6JFVJOvsfCaVCOgV0tu6m2T4RK6RKN8htC81kSIj-ZeR_erpvPdFLFOEBLLyOdyEt0mQVWzY4OUpPEEXnayr2WGtkQ9hKelu4DX-NFqj4yQwEqdEyjGCG1SIUWN5oHEp6bbTEbWJphZWaT4UagpZVePk05lj6ZGDBEqXqho2VBKkZgyYOUgPLbzSHlkT8wwLPJoEnKSBpXNp7Kgc9hgjQRwZpXXflgEzSf8GIAzS9vTDRzYAAupxC2x8AAxKT5sucvGVfiFKz5Ts_syhGZ9micq4goNdIg4UL1QygBxZe865yVF4jMjcdF2xi7xjk6ovVqUzE6cyHnhhhp4dlweNqfJWvoLZCh_jx9_i3rncPIxyXL3oWxlpVu5y0)
//!
//! ### Terminology
//!
//! * **VAMM:** Acronym for Virtual Automated Market Maker.
//! * **CFMM:** Acronym for Constant Function Market Maker.
//! * **TWAP:** Acronym for Time Weighted Average Price.
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
//! so there is no extrinsic defined.
//!
//! ### Public Functions
//!
//! * [`create`](pallet/struct.Pallet.html#method.create): Creates a new vamm,
//! returning it's Id.
//! * [`swap`](pallet/struct.Pallet.html#method.swap): Performs swap of a
//! * [`move_price`](pallet/struct.Pallet.html#method.move_price): Changes
//! amount of [`base`](VammState::base_asset_reserves) and
//! [`quote`](VammState::quote_asset_reserves) assets in reserve, essentially
//! changing the invariant.
//! * [`get_price`](pallet/struct.Pallet.html#method.get_price): Gets the
//! current price of the [`base`](VammState::base_asset_reserves) or
//! [`quote`](VammState::quote_asset_reserves) asset in a vamm.
//! * [`get_twap`](pallet/struct.Pallet.html#method.get_twap): Gets the time
//! weighted average price of the desired asset.
//! * [`update_twap`](pallet/struct.Pallet.html#method.update_twap): Updates the
//! time weighted average price of the desired asset.
//!
//! ### Runtime Storage Objects
//!
//! - [`VammCounter`](VammCounter): The number of created vamms.
//! - [`VammMap`](VammMap): Mapping of a [`VammId`](Config::VammId) to it's
//! corresponding [`VammState`].
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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// Allow some linters for tests.
#![cfg_attr(
	not(test),
	warn(
		clippy::disallowed_methods,
		clippy::disallowed_types,
		clippy::indexing_slicing,
		clippy::todo,
		clippy::unwrap_used,
		clippy::panic,
		clippy::doc_markdown
	)
)]
// Specify linters to VAMM Pallet.
#![warn(clippy::unseparated_literal_suffix)]
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
	use composable_traits::vamm::{
		AssetType, Direction, MovePriceConfig, SwapConfig, SwapOutput, SwapSimulationConfig, Vamm,
		VammConfig, MINIMUM_FUNDING_PERIOD,
	};
	use frame_support::{
		pallet_prelude::*, sp_std::fmt::Debug, traits::UnixTime, transactional, Blake2_128Concat,
	};
	use num_integer::Integer;
	use sp_arithmetic::traits::Unsigned;
	use sp_core::U256;
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero},
		ArithmeticError, FixedPointNumber,
	};
	use std::cmp::Ordering;

	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};

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

		/// The Balance type used by the pallet for bookkeeping.
		type Balance: Default
			+ AtLeast32BitUnsigned
			+ CheckedAdd
			+ CheckedDiv
			+ CheckedMul
			+ CheckedSub
			+ Codec
			+ Copy
			+ From<u64>
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Ord
			+ Parameter
			+ Unsigned
			+ Zero;

		/// Signed decimal fixed point number.
		type Decimal: FullCodec + MaxEncodedLen + TypeInfo + FixedPointNumber<Inner = Self::Balance>;

		/// The Integer type used by the pallet for computing swaps.
		type Integer: Integer;

		/// Type representing the current time.
		type Moment: Default
			+ AtLeast32BitUnsigned
			+ Clone
			+ Codec
			+ Copy
			+ Debug
			+ From<u64>
			+ Into<u64>
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ TypeInfo;

		/// Implementation for querying the current Unix timestamp
		type TimeProvider: UnixTime;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types
	// ----------------------------------------------------------------------------------------------------

	type BalanceOf<T> = <T as Config>::Balance;
	type DecimalOf<T> = <T as Config>::Decimal;
	type VammIdOf<T> = <T as Config>::VammId;
	type MomentOf<T> = <T as Config>::Moment;
	type SwapOutputOf<T> = SwapOutput<BalanceOf<T>>;
	type SwapConfigOf<T> = SwapConfig<VammIdOf<T>, BalanceOf<T>>;
	type SwapSimulationConfigOf<T> = SwapSimulationConfig<VammIdOf<T>, BalanceOf<T>>;
	type MovePriceConfigOf<T> = MovePriceConfig<VammIdOf<T>, BalanceOf<T>>;
	type VammConfigOf<T> = VammConfig<BalanceOf<T>, MomentOf<T>>;
	type VammStateOf<T> = VammState<BalanceOf<T>, MomentOf<T>>;

	/// Represents the direction a of a position.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum SwapDirection {
		Add,
		Remove,
	}

	/// Data relating to the state of a virtual market.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug, Default)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub struct VammState<Balance, Moment> {
		/// The total amount of base asset present in the vamm.
		pub base_asset_reserves: Balance,

		/// The total amount of quote asset present in the vamm.
		pub quote_asset_reserves: Balance,

		/// The magnitude of the quote asset reserve.
		pub peg_multiplier: Balance,

		/// The invariant `K`.
		pub invariant: U256,

		/// Whether this market is closed or not.
		///
		/// This variable function as a signal to allow pallets who uses the
		/// Vamm to set a market as "operating as normal" or "not to be used
		/// anymore".  If the value is `None` it means the market is operating
		/// as normal, but if the value is `Some(timestamp)` it means the market
		/// is flagged to be closed and the closing action will take (or took)
		/// effect at the time `timestamp`.
		pub closed: Option<Moment>,

		/// The time weighted average price for [`base_asset_reserves`](VammState::base_asset_reserves).
		pub base_asset_twap: Balance,

		/// The time weighted average price timestamp for [`base_asset_reserves`](VammState::base_asset_reserves).
		pub base_asset_twap_timestamp: Moment,

		/// The time weighted average price for [`quote_asset_reserves`](VammState::quote_asset_reserves).
		pub quote_asset_twap: Balance,

		/// The time weighted average price timestamp for [`quote_asset_reserves`](VammState::quote_asset_reserves).
		pub quote_asset_twap_timestamp: Moment,

		/// The frequency with which the vamm must have it's funding rebalance.
		/// (Used only for twap calculations.)
		pub funding_period: Moment,
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
		/// Emitted after a successful call to the [`create`](Pallet::create)
		/// function.
		Created { vamm_id: VammIdOf<T>, state: VammStateOf<T> },
		/// Emitted after a successful call to the [`swap`](Pallet::swap)
		/// function.
		Swapped {
			vamm_id: VammIdOf<T>,
			input_amount: BalanceOf<T>,
			output_amount: SwapOutputOf<T>,
			input_asset_type: AssetType,
			direction: Direction,
		},
		/// Emitted after a successful call to the
		/// [`move_price`](Pallet::move_price) function.
		PriceMoved {
			vamm_id: VammIdOf<T>,
			base_asset_reserves: BalanceOf<T>,
			quote_asset_reserves: BalanceOf<T>,
			invariant: U256,
		},
		/// Emitted after a successful call to the
		/// [`update_twap`](Pallet::update_twap) function.
		UpdatedTwap { vamm_id: VammIdOf<T>, asset_type: AssetType, value: DecimalOf<T> },
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Runtime  Errors
	// ----------------------------------------------------------------------------------------------------

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Tried to set [`base_asset_reserves`](VammState::base_asset_reserves)
		/// to zero.
		BaseAssetReserveIsZero,
		/// Tried to set
		/// [`quote_asset_reserves`](VammState::quote_asset_reserves) to zero.
		QuoteAssetReserveIsZero,
		/// Computed Invariant is zero.
		InvariantIsZero,
		/// Tried to set [`peg_multiplier`](VammState) to zero.
		PegMultiplierIsZero,
		/// Tried to access an invalid [`VammId`](Config::VammId).
		VammDoesNotExist,
		/// Tried to retrieve a Vamm but the function failed.
		FailToRetrieveVamm,
		/// Tried to execute a trade but the Vamm didn't have enough funds to
		/// fulfill it.
		InsufficientFundsForTrade,
		/// Tried to add some amount of asset to Vamm but it would exceeds the
		/// supported maximum value.
		TradeExtrapolatesMaximumSupportedAmount,
		/// Tried to perform operation against a closed Vamm.
		VammIsClosed,
		/// Tried to swap assets but the amount returned was less than the minimum expected.
		SwappedAmountLessThanMinimumLimit,
		/// Tried to derive invariant from [`base`](VammState::base_asset_reserves) and
		/// [`quote`](VammState::quote_asset_reserves) asset, but the
		/// computation was not successful.
		FailedToDeriveInvariantFromBaseAndQuoteAsset,
		/// Tried to perform swap operation but it would drain all
		/// [`base`](VammState::base_asset_reserves) asset reserves.
		BaseAssetReservesWouldBeCompletelyDrained,
		/// Tried to perform swap operation but it would drain all
		/// [`quote`](VammState::quote_asset_reserves) asset reserves.
		QuoteAssetReservesWouldBeCompletelyDrained,
		/// Tried to update twap for an asset, but it's last twap update was
		/// more recent than the current time.
		AssetTwapTimestampIsMoreRecent,
		/// Tried to update twap for an asset, but the desired new twap value is
		/// zero.
		NewTwapValueIsZero,
		/// Tried to compute last twap weight. This is due to
		/// [`funding_period`](VammState::funding_period) being less than `now -
		/// last_twap_timestamp`.
		FailedToComputeLastTwapWeight,
		/// Tried to create a vamm with a
		/// [`funding_period`](VammState::funding_period) smaller than the
		/// minimum allowed one specified by
		/// [`MINIMUM_FUNDING_PERIOD`](composable_traits::vamm::MINIMUM_FUNDING_PERIOD).
		FundingPeriodTooSmall,
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
		pub vamms: Vec<(VammIdOf<T>, VammStateOf<T>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { vamm_count: Default::default(), vamms: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			VammCounter::<T>::put(self.vamm_count);
			self.vamms.iter().for_each(|(vamm_id, vamm_state)| {
				VammMap::<T>::insert(vamm_id, vamm_state);
			})
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                                           Vamm Trait
	// ----------------------------------------------------------------------------------------------------

	impl<T: Config> Vamm for Pallet<T> {
		type Balance = BalanceOf<T>;
		type Moment = MomentOf<T>;
		type Decimal = T::Decimal;
		type SwapConfig = SwapConfigOf<T>;
		type SwapSimulationConfig = SwapSimulationConfigOf<T>;
		type VammConfig = VammConfigOf<T>;
		type MovePriceConfig = MovePriceConfigOf<T>;
		type VammId = VammIdOf<T>;

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
		/// ![](https://www.plantuml.com/plantuml/svg/NP2nJiCm48PtFyNH1L2L5yXGbROB0on8x5VdLsibjiFT9NbzQaE4odRIz-dp-VPgB3R5mMaVqiZ2aGGwvgHuWofVSC2GbnUHl93916V11j0dnqXUm1PoSeyyMMPlOMO3vUGUx8e8YYpgtCXYmOUHaz7cE0Gasn0h-JhUuzAjSBuDhcFZCojeys5P-09wAi9pDVIVSXYox_sLGwhux9txUO6QNSrjjoqToyfriHv6Wgy9QgxGOjNalRJ2PfTloPPE6BC68r-TRYrXHlfJVx_MD2szOrcTrvFR8tNbsjy0)
		///
		/// ## Parameters:
		/// - `base_asset_reserves`: The amount of
		/// [`base`](VammState::base_asset_reserves) asset
		/// - `quote_asset_reserves`: The amount of
		/// [`quote`](VammState::quote_asset_reserves) asset
		/// - `peg_multiplier`: The constant multiplier responsible to balance
		/// [`quote`](VammState::quote_asset_reserves) and
		/// [`base`](VammState::base_asset_reserves)
		/// asset
		///
		/// ## Returns
		/// The new vamm's id, if successful.
		///
		/// ## Assumptions or Requirements
		/// In order to create a valid vamm, we need to ensure that both
		/// [`base`](VammState::base_asset_reserves) and
		/// [`quote`](VammState::quote_asset_reserves) asset reserves, as well
		/// as the peg_multiplier, are non-zero. Every parameter must be greater
		/// than zero.
		///
		/// ## Emits
		/// * [`Created`](Event::<T>::Created)
		///
		/// ## State Changes
		/// Updates [`VammMap`] storage map and [`VammCounter`] storage value.
		///
		/// ## Errors
		/// * [`Error::<T>::BaseAssetReserveIsZero`]
		/// * [`Error::<T>::QuoteAssetReserveIsZero`]
		/// * [`Error::<T>::InvariantIsZero`]
		/// * [`Error::<T>::FailedToDeriveInvariantFromBaseAndQuoteAsset`]
		/// * [`Error::<T>::FundingPeriodTooSmall`]
		/// * [`ArithmeticError::Overflow`](sp_runtime::ArithmeticError)
		///
		/// # Runtime
		/// `O(1)`
		#[transactional]
		fn create(config: &VammConfigOf<T>) -> Result<VammIdOf<T>, DispatchError> {
			// TODO(Cardosaum)
			// How to ensure that the caller has the right privileges?
			// (eg. How to ensure the caller is the Clearing House, and not anyone else?)

			ensure!(!config.peg_multiplier.is_zero(), Error::<T>::PegMultiplierIsZero);
			ensure!(
				config.funding_period >= MINIMUM_FUNDING_PERIOD.into(),
				Error::<T>::FundingPeriodTooSmall
			);

			let invariant =
				Self::compute_invariant(config.base_asset_reserves, config.quote_asset_reserves)?;
			let now = Self::now(&None);
			let tmp_vamm_state = VammStateOf::<T> {
				base_asset_reserves: config.base_asset_reserves,
				quote_asset_reserves: config.quote_asset_reserves,
				peg_multiplier: config.peg_multiplier,
				..Default::default()
			};
			let base_twap = Self::do_get_price(&tmp_vamm_state, AssetType::Base)?;
			let quote_twap = Self::do_get_price(&tmp_vamm_state, AssetType::Quote)?;

			VammCounter::<T>::try_mutate(|next_id| {
				let id = *next_id;
				let vamm_state = VammStateOf::<T> {
					base_asset_reserves: config.base_asset_reserves,
					quote_asset_reserves: config.quote_asset_reserves,
					base_asset_twap: base_twap.into_inner(),
					quote_asset_twap: quote_twap.into_inner(),
					base_asset_twap_timestamp: now,
					quote_asset_twap_timestamp: now,
					peg_multiplier: config.peg_multiplier,
					invariant,
					funding_period: config.funding_period,
					closed: None,
				};

				VammMap::<T>::insert(&id, vamm_state);
				*next_id = id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;

				Self::deposit_event(Event::<T>::Created { vamm_id: id, state: vamm_state });

				Ok(id)
			})
		}

		/// Gets the current price of the
		/// [`base`](VammState::base_asset_reserves) or
		/// [`quote`](VammState::quote_asset_reserves) asset in a vamm.
		///
		/// # Overview
		/// In order for the caller to know what the current price of an asset
		/// in a specific vamm is, it has to request it to the Vamm Pallet. The
		/// Vamm Pallet consults the runtime storage for the desired vamm,
		/// computes the current price and returns it to the caller.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/PP0zJWCn44PxdsBO1b2q5qY14b9GKI7H3vkFOB7-OURRvFfWhm0XEillpHlBEwSQbpG7Vu-vgcaIWzUI7OzmrnFkCPVBtgnSXBOWC7A6F82Yxg1KYnFajPYeF6jAuLeN5fqOpqf8oU6ARqYGfEOXL3N6ALRDbE4mHsGEeYvJF_x5BTVXkNMFIdrHXmnFBAOdo4qJRhlXNGbhHSQxFhBPRFyzrF2nm1aQRruVNBL-vLJYXwxmK59TY5xuPbzmNJQEMzd_BWWxv6Fxq4y0)
		///
		/// ## Parameters
		///  - `vamm_id`: The ID of the desired vamm to query.
		///  - `asset_type`: The desired asset type to get info about. (either
		///  [`base`](VammState::base_asset_reserves) or
		///  [`quote`](VammState::quote_asset_reserves))
		///
		/// ## Returns
		/// The price of [`base`](VammState::base_asset_reserves) asset in
		/// relation to [`quote`](VammState::quote_asset_reserves) (or
		/// vice-versa).
		///
		/// ## Assumptions or Requirements
		/// In order to consult the current price for an asset, we need to
		/// ensure that the desired vamm_id exists.
		///
		/// ## Emits
		/// No event is emitted for this function.
		///
		/// ## State Changes
		/// This function does not mutate runtime storage.
		///
		/// ## Errors
		/// * [`Error::<T>::VammDoesNotExist`]
		/// * [`Error::<T>::FailToRetrieveVamm`]
		/// * [`ArithmeticError::Overflow`](sp_runtime::ArithmeticError)
		/// * [`ArithmeticError::DivisionByZero`](sp_runtime::ArithmeticError)
		///
		/// # Runtime
		/// `O(1)`
		fn get_price(
			vamm_id: VammIdOf<T>,
			asset_type: AssetType,
		) -> Result<DecimalOf<T>, DispatchError> {
			// Get Vamm state.
			let vamm_state = Self::get_vamm_state(&vamm_id)?;

			Self::do_get_price(&vamm_state, asset_type)
		}

		/// Returns the time weighted average price of the desired asset.
		///
		/// # Overview
		/// In order for the caller to know which is the time weighted average
		/// price of the desired asset, it has to request it to the Vamm Pallet.
		/// The pallet will query the runtime storage and return the desired
		/// twap.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/FSqz3i8m343XdLF01UgTgH8IrwXSrsqYnKxa7tfzAWQcfszwimTQfBJReogrt3YjtKl4y2U0uJaTDKgkwMpKDLXZeYxmwZAwuzhuNO7-07OgRB0R2iC7HM2hU5nos5CfQjVbu5ZYn36DXlfxpwpRrIy0)
		///
		/// ## Parameters
		///  - [`vamm_id`](Config::VammId): The ID of the desired vamm to query.
		///  - [`asset_type`](composable_traits::vamm::AssetType): The desired
		///  asset type to get info about.
		///
		/// ## Returns
		/// The twap for the specified asset.
		///
		/// ## Assumptions or Requirements
		/// * The requested [`VammId`](Config::VammId) must exist.
		/// * The requested Vamm must be open.
		///
		/// For more information about how to know if a Vamm is open or not,
		/// please have a look in the variable [`closed`](VammState::closed).
		///
		/// ## Emits
		/// No event is emitted for this function.
		///
		/// ## State Changes
		/// This function does not mutate runtime storage.
		///
		/// ## Errors
		/// * [`Error::<T>::VammDoesNotExist`]
		/// * [`Error::<T>::FailToRetrieveVamm`]
		/// * [`Error::<T>::VammIsClosed`]
		///
		/// # Runtime
		/// `O(1)`
		#[transactional]
		fn get_twap(
			vamm_id: VammIdOf<T>,
			asset_type: AssetType,
		) -> Result<DecimalOf<T>, DispatchError> {
			// Sanity Checks
			// 1) Vamm must exist
			let vamm_state = Self::get_vamm_state(&vamm_id)?;

			// 2) Vamm must be open
			ensure!(!Self::is_vamm_closed(&vamm_state, &None), Error::<T>::VammIsClosed);

			match asset_type {
				AssetType::Base => Ok(DecimalOf::<T>::from_inner(vamm_state.base_asset_twap)),
				AssetType::Quote => Ok(DecimalOf::<T>::from_inner(vamm_state.quote_asset_twap)),
			}
		}

		/// Updates the time weighted average price of the desired asset.
		///
		/// # Overview
		/// In order for the caller to update the time weighted average price of
		/// the desired asset, it has to request it to the Vamm Pallet. The
		/// pallet will perform the needed sanity checks and update the runtime
		/// storage with the desired twap value, returning it in case of
		/// success.
		///
		/// This function can also compute the new twap value using an
		/// Exponential Moving Average algorithm rather than blindly seting it
		/// to the value passed by the caller. In that case, the following
		/// algorithm will be used:
		///
		/// $$
		/// twap = \frac{(x_t \cdot w_t) + (x_{t-1} \cdot w_{t-1})}{w_t + w_{t-1}}
		/// $$
		///
		/// Where:
		/// * $twap$: Is the new calculated twap.
		/// * $x_t$: Is the current price of the asset.
		/// * $x_{t-1}$: Is the last twap price of the asset.
		/// * $w_t$: $max(1, T_{now} - T_{last\_update})$.
		/// * $w_{t-1}$: $max(1, $[`funding_period`](VammState::funding_period)$ - w_t)$.
		/// * $T_{now}$: current unix timestamp (ie. seconds since the Unix epoch).
		/// * $T_{last\_update}$: timestamp from last twap price update.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/FSqz3i8m343XdLF01UgTgH8IrwXSnsqZnKxa7tfzAWQcfszwimTQfBJReogrB9pMxaV4y2U0uJdjDOvSqzceQx36H5tWrMLqnxNnkmBz0UnqiC5cA0mV585ISR_aiALIrAvBZeB1Ivmufj5GV_kPjLpz0W00)
		///
		/// ## Parameters
		///  - [`vamm_id`](Config::VammId): The ID of the desired vamm to update.
		///  - [`asset_type`](composable_traits::vamm::AssetType): The desired
		///  asset type to update.
		///  - `new_twap`: The optional desired value for the new asset's twap.
		///  If the value is `None`, than the Vamm will update the twap using an
		///  exponential moving average algorithm.
		///
		/// ## Returns
		/// The new twap value for the specified asset.
		///
		/// ## Assumptions or Requirements
		/// * The requested [`VammId`](Config::VammId) must exists
		/// * The requested Vamm must be open.
		/// * The `new_twap` value can't be zero.
		///
		/// For more information about how to know if a Vamm is open or not,
		/// please have a look in the variable [`closed`](VammState::closed).
		///
		/// ## Emits
		/// * [`UpdatedTwap`](Event::<T>::UpdatedTwap)
		///
		/// ## State Changes
		/// Updates [`VammMap`] storage map.
		///
		/// ## Errors
		/// * [`Error::<T>::VammDoesNotExist`]
		/// * [`Error::<T>::FailToRetrieveVamm`]
		/// * [`Error::<T>::VammIsClosed`]
		/// * [`Error::<T>::NewTwapValueIsZero`]
		/// * [`Error::<T>::AssetTwapTimestampIsMoreRecent`]
		/// * [`ArithmeticError::Overflow`](sp_runtime::ArithmeticError)
		/// * [`ArithmeticError::Underflow`](sp_runtime::ArithmeticError)
		/// * [`ArithmeticError::DivisionByZero`](sp_runtime::ArithmeticError)
		///
		/// # Runtime
		/// `O(1)`
		fn update_twap(
			vamm_id: VammIdOf<T>,
			asset_type: AssetType,
			new_twap: Option<DecimalOf<T>>,
		) -> Result<DecimalOf<T>, DispatchError> {
			// Sanity Checks
			// Vamm must exist.
			let mut vamm_state = Self::get_vamm_state(&vamm_id)?;

			// Delegate update twap to internal functions.
			match new_twap {
				Some(new_twap) => {
					Self::do_update_twap(vamm_id, &mut vamm_state, asset_type, new_twap, &None)
				},
				None => Self::update_vamm_twap(vamm_id, &mut vamm_state, asset_type, &None),
			}
		}

		/// Performs the swap of the desired asset against the vamm.
		///
		/// # Overview
		/// In order for the caller be able to swap assets in the vamm, it has
		/// to request it to the Vamm Pallet. The pallet will perform all needed
		/// checks to ensure the swap is a valid one and then, using the
		/// corresponding function it was configured to, will compute the amount
		/// of assets the caller will receive.
		///
		/// In the current state the only function available to perform these
		/// computations is the CFMM `x * y = k`.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/FSq_giCm383n_PtYzGBHtYbGw3MA8YknmPAD_ZJNR-ZGwUCtVQi7MgJqlrjJwbauhV_NYEbt0CDpELhKtDBPQ6Ymna93u35a3iUjyxC1_G3iLDbWDnI6Duf0QNXSSjXJAThGbvyubzbHlz-LjLpz0000)
		///
		/// ## Parameters
		///  - `config`: Specification for swaps.
		///
		/// ## Returns
		/// The amount of the other asset the caller will receive as a
		/// result of the swap.
		///
		/// E.g. If the caller swaps [`quote`](VammState::quote_asset_reserves)
		/// asset, it will receive some amount of
		/// [`base`](VammState::base_asset_reserves) asset (and vice-versa).
		///
		/// ## Assumptions or Requirements
		/// * The requested [`VammId`](Config::VammId) must exists
		/// * The desired swap amount can not exceed the maximum supported value
		/// for the Vamm
		/// * The desired swap amount must result in at least
		/// [`output_amount_limit`](composable_traits::vamm::SwapConfig)
		///
		/// ## Emits
		/// * [`Swapped`](Event::<T>::Swapped)
		///
		/// ## State Changes
		/// Updates [`VammMap`] storage map.
		///
		/// ## Errors
		/// * [`Error::<T>::VammDoesNotExist`]
		/// * [`Error::<T>::FailToRetrieveVamm`]
		/// * [`Error::<T>::VammIsClosed`]
		/// * [`Error::<T>::InsufficientFundsForTrade`]
		/// * [`Error::<T>::TradeExtrapolatesMaximumSupportedAmount`]
		/// * [`ArithmeticError::Overflow`](sp_runtime::ArithmeticError)
		/// * [`ArithmeticError::Underflow`](sp_runtime::ArithmeticError)
		/// * [`ArithmeticError::DivisionByZero`](sp_runtime::ArithmeticError)
		///
		/// # Runtime
		/// `O(1)`
		#[transactional]
		fn swap(config: &SwapConfigOf<T>) -> Result<SwapOutputOf<T>, DispatchError> {
			// Get Vamm state.
			let mut vamm_state = Self::get_vamm_state(&config.vamm_id)?;

			// Perform twap update before swapping assets.
			Self::update_vamm_twap(config.vamm_id, &mut vamm_state, config.asset, &None)?;

			// Perform required sanity checks.
			Self::swap_sanity_check(config, &vamm_state)?;

			// Delegate swap to helper functions.
			let amount_swapped = match config.asset {
				AssetType::Quote => Self::swap_quote_asset(config, &mut vamm_state),
				AssetType::Base => Self::swap_base_asset(config, &mut vamm_state),
			}?;

			// Ensure swapped amount is valid.
			ensure!(
				amount_swapped.output >= config.output_amount_limit,
				Error::<T>::SwappedAmountLessThanMinimumLimit
			);

			// Ensure both quote and base assets weren't completely drained from vamm.
			ensure!(
				!vamm_state.base_asset_reserves.is_zero(),
				Error::<T>::BaseAssetReservesWouldBeCompletelyDrained
			);
			ensure!(
				!vamm_state.quote_asset_reserves.is_zero(),
				Error::<T>::QuoteAssetReservesWouldBeCompletelyDrained
			);

			// TODO(Cardosaum): Write one more `ensure!` block regarding
			// amount_swapped negative or positive?

			// Update runtime storage
			VammMap::<T>::insert(&config.vamm_id, vamm_state);

			// Deposit swap event into blockchain
			Self::deposit_event(Event::<T>::Swapped {
				vamm_id: config.vamm_id,
				input_amount: config.input_amount,
				output_amount: amount_swapped,
				input_asset_type: config.asset,
				direction: config.direction,
			});

			// Return total swapped asset
			Ok(amount_swapped)
		}

		#[allow(unused_variables)]
		fn swap_simulation(
			config: &SwapSimulationConfigOf<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			todo!()
		}

		/// Moves the price of a vamm to the desired values of
		/// [`base`](VammState::base_asset_reserves) and
		/// [`quote`](VammState::quote_asset_reserves) asset reserves.
		///
		/// # Overview
		/// In order for the caller to modify the
		/// [`base`](VammState::base_asset_reserves) and
		/// [`quote`](VammState::quote_asset_reserves) asset reserves,
		/// essentially modifying the invariant `k` of the function `x * y = k`,
		/// it has to request it to the Vamm Pallet. The pallet will perform the
		/// needed validity checks and, if everything succeeds, a
		/// [`PriceMoved`](Event::<T>::PriceMoved) event will be deposited on
		/// the blockchain warning the state change for the vamm and the asset
		/// reserves of the vamm and it's invariant will change accordingly.
		///
		/// ![](https://www.plantuml.com/plantuml/svg/FSqz3i8m343XdLF01UgTgH8IrwXSrsqYnKxadt9zAWQcfszwimTQfBJReogrt3YjtKl4y2U0uMSwQfHSqzceQx36H5tWrMLqnxNnkmBz0UnKs60t58OJHM2hU5nos5CfQjT5-idBi4eyZORwky-iszKl)
		///
		/// ## Parameters:
		/// * [`config`](composable_traits::vamm::MovePriceConfig):
		/// Specification for moving the price of the vamm.
		///
		/// ## Returns
		/// This function returns the calculated invariant `K` if successful.
		///
		/// ## Assumptions or Requirements
		/// In order to move the price of a vamm we need to ensure that some properties hold:
		/// * The passed [`VammId`](Config::VammId) must be valid
		/// * The desired vamm must be open. (See the [`closed`](VammState)
		/// field for more information).
		/// * Both [`base`](VammState::base_asset_reserves) and
		/// [`quote`](VammState::quote_asset_reserves) must be greater than
		/// zero.
		///
		/// ## Emits
		/// * [`PriceMoved`](Event::<T>::PriceMoved)
		///
		/// ## State Changes
		/// Updates:
		/// * [`VammMap`], modifying both
		/// [`base`](VammState::base_asset_reserves) and
		/// [`quote`](VammState::quote_asset_reserves) asset reserves as well as
		/// the invariant.
		///
		/// ## Errors
		/// * [`Error::<T>::VammDoesNotExist`]
		/// * [`Error::<T>::FailToRetrieveVamm`]
		/// * [`Error::<T>::VammIsClosed`]
		/// * [`Error::<T>::BaseAssetReserveIsZero`]
		/// * [`Error::<T>::QuoteAssetReserveIsZero`]
		/// * [`Error::<T>::InvariantIsZero`]
		/// * [`Error::<T>::FailedToDeriveInvariantFromBaseAndQuoteAsset`]
		///
		/// # Runtime
		/// `O(1)`
		#[transactional]
		fn move_price(config: &Self::MovePriceConfig) -> Result<U256, DispatchError> {
			// Get Vamm state.
			let mut vamm_state = Self::get_vamm_state(&config.vamm_id)?;
			// TODO(Cardosaum): Try to move from using function
			// Self::is_vamm_closed to Vamm.is_closed method
			ensure!(!Self::is_vamm_closed(&vamm_state, &None), Error::<T>::VammIsClosed);

			let invariant =
				Self::compute_invariant(config.base_asset_reserves, config.quote_asset_reserves)?;

			vamm_state.base_asset_reserves = config.base_asset_reserves;
			vamm_state.quote_asset_reserves = config.quote_asset_reserves;
			vamm_state.invariant = invariant;

			// Update runtime storage.
			VammMap::<T>::insert(&config.vamm_id, vamm_state);

			// Deposit price moved event into blockchain.
			Self::deposit_event(Event::<T>::PriceMoved {
				vamm_id: config.vamm_id,
				base_asset_reserves: config.base_asset_reserves,
				quote_asset_reserves: config.quote_asset_reserves,
				invariant,
			});

			// Return new invariant.
			Ok(invariant)
		}
	}

	// ----------------------------------------------------------------------------------------------------
	//                              Helper Functions
	// ----------------------------------------------------------------------------------------------------

	// Helper types - core functionality
	struct CalculateSwapAsset<T: Config> {
		output_amount: BalanceOf<T>,
		input_amount: BalanceOf<T>,
	}

	// Helper functions - core functionality
	impl<T: Config> Pallet<T> {
		pub fn do_get_price(
			vamm_state: &VammStateOf<T>,
			asset_type: AssetType,
		) -> Result<DecimalOf<T>, DispatchError> {
			let base_asset_reserves_decimal =
				DecimalOf::<T>::from_inner(vamm_state.base_asset_reserves);
			let quote_asset_reserves_decimal =
				DecimalOf::<T>::from_inner(vamm_state.quote_asset_reserves);
			let peg_multiplier_decimal = DecimalOf::<T>::from_inner(vamm_state.peg_multiplier);

			match asset_type {
				AssetType::Base => Ok(quote_asset_reserves_decimal
					.checked_mul(&peg_multiplier_decimal)
					.ok_or(ArithmeticError::Overflow)?
					.checked_div(&base_asset_reserves_decimal)
					.ok_or(ArithmeticError::DivisionByZero)?),

				AssetType::Quote => Ok(base_asset_reserves_decimal
					.checked_mul(&peg_multiplier_decimal)
					.ok_or(ArithmeticError::Overflow)?
					.checked_div(&quote_asset_reserves_decimal)
					.ok_or(ArithmeticError::DivisionByZero)?),
			}
		}

		fn do_update_twap(
			vamm_id: VammIdOf<T>,
			vamm_state: &mut VammStateOf<T>,
			asset_type: AssetType,
			new_twap: DecimalOf<T>,
			now: &Option<MomentOf<T>>,
		) -> Result<DecimalOf<T>, DispatchError> {
			// Sanity checks
			Self::update_twap_sanity_check(vamm_state, asset_type, Some(new_twap), now)?;

			let now = Self::now(now);
			match asset_type {
				AssetType::Base => {
					vamm_state.base_asset_twap = new_twap.into_inner();
					vamm_state.base_asset_twap_timestamp = now;
				},
				AssetType::Quote => {
					vamm_state.quote_asset_twap = new_twap.into_inner();
					vamm_state.quote_asset_twap_timestamp = now;
				},
			};

			// Update runtime storage
			VammMap::<T>::insert(&vamm_id, vamm_state);

			// TODO(Cardosaum): This function will execute quite frequently,
			// isn't it a problem to emit one new event for each function call?`
			//
			// Deposit swap event into blockchain
			Self::deposit_event(Event::<T>::UpdatedTwap { vamm_id, asset_type, value: new_twap });

			// Return total swapped asset
			Ok(new_twap)
		}

		fn swap_quote_asset(
			config: &SwapConfigOf<T>,
			vamm_state: &mut VammStateOf<T>,
		) -> Result<SwapOutputOf<T>, DispatchError> {
			let quote_asset_reserve_amount = config
				.input_amount
				.checked_div(&vamm_state.peg_multiplier)
				.ok_or(ArithmeticError::DivisionByZero)?;

			let initial_base_asset_reserve = vamm_state.base_asset_reserves;
			let swap_amount = Self::calculate_swap_asset(
				&quote_asset_reserve_amount,
				&vamm_state.quote_asset_reserves,
				&config.direction,
				vamm_state,
			)?;

			vamm_state.base_asset_reserves = swap_amount.output_amount;
			vamm_state.quote_asset_reserves = swap_amount.input_amount;

			match initial_base_asset_reserve.cmp(&swap_amount.output_amount) {
				Ordering::Less => Ok(SwapOutput {
					output: swap_amount
						.output_amount
						.checked_sub(&initial_base_asset_reserve)
						.ok_or(ArithmeticError::Underflow)?,
					negative: true,
				}),
				_ => Ok(SwapOutput {
					output: initial_base_asset_reserve
						.checked_sub(&swap_amount.output_amount)
						.ok_or(ArithmeticError::Underflow)?,
					negative: false,
				}),
			}
		}

		fn swap_base_asset(
			config: &SwapConfigOf<T>,
			vamm_state: &mut VammStateOf<T>,
		) -> Result<SwapOutputOf<T>, DispatchError> {
			let initial_quote_asset_reserve = vamm_state.quote_asset_reserves;
			let swap_amount = Self::calculate_swap_asset(
				&config.input_amount,
				&vamm_state.base_asset_reserves,
				&config.direction,
				vamm_state,
			)?;

			vamm_state.base_asset_reserves = swap_amount.input_amount;
			vamm_state.quote_asset_reserves = swap_amount.output_amount;

			Ok(SwapOutput {
				output: Self::calculate_quote_asset_amount_swapped(
					&initial_quote_asset_reserve,
					&swap_amount.output_amount,
					&config.direction,
					vamm_state,
				)?,
				negative: false,
			})
		}

		fn calculate_swap_asset(
			swap_amount: &BalanceOf<T>,
			input_asset_amount: &BalanceOf<T>,
			direction: &Direction,
			vamm_state: &VammStateOf<T>,
		) -> Result<CalculateSwapAsset<T>, DispatchError> {
			let new_input_amount = match direction {
				Direction::Add => {
					input_asset_amount.checked_add(swap_amount).ok_or(ArithmeticError::Overflow)?
				},

				Direction::Remove => {
					input_asset_amount.checked_sub(swap_amount).ok_or(ArithmeticError::Underflow)?
				},
			};
			let new_input_amount_u256 = Self::balance_to_u256(new_input_amount)?;

			// TODO(Cardosaum): Maybe it would be worth to create another sanity
			// check in the helper function tracking the inputs and verify if
			// they would result in a division by zero? (Doing this we could
			// present a better error message for the caller).
			let new_output_amount_u256 = vamm_state
				.invariant
				.checked_div(new_input_amount_u256)
				.ok_or(ArithmeticError::DivisionByZero)?;
			let new_output_amount = Self::u256_to_balance(new_output_amount_u256)?;

			Ok(CalculateSwapAsset {
				input_amount: new_input_amount,
				output_amount: new_output_amount,
			})
		}

		fn calculate_quote_asset_amount_swapped(
			quote_asset_reserve_before: &BalanceOf<T>,
			quote_asset_reserve_after: &BalanceOf<T>,
			direction: &Direction,
			vamm_state: &VammStateOf<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			let quote_asset_reserve_change = match direction {
				Direction::Add => quote_asset_reserve_before
					.checked_sub(quote_asset_reserve_after)
					.ok_or(ArithmeticError::Underflow)?,
				Direction::Remove => quote_asset_reserve_after
					.checked_sub(quote_asset_reserve_before)
					.ok_or(ArithmeticError::Underflow)?,
			};

			let quote_asset_amount = quote_asset_reserve_change
				.checked_mul(&vamm_state.peg_multiplier)
				.ok_or(ArithmeticError::Overflow)?;

			Ok(quote_asset_amount)
		}
	}

	// Helper functions - validity checks
	impl<T: Config> Pallet<T> {
		fn update_twap_sanity_check(
			vamm_state: &VammStateOf<T>,
			asset_type: AssetType,
			new_twap: Option<DecimalOf<T>>,
			now: &Option<MomentOf<T>>,
		) -> Result<(), DispatchError> {
			// Sanity Checks
			// 1) New desired twap value can't be zero.
			if let Some(new_twap) = new_twap {
				ensure!(!new_twap.is_zero(), Error::<T>::NewTwapValueIsZero);
			}

			// 2) Vamm must be open.
			ensure!(!Self::is_vamm_closed(vamm_state, now), Error::<T>::VammIsClosed);

			// 3) Only update asset's twap if time has passed since last update.
			let now = Self::now(now);
			let asset_twap_timestamp =
				Self::asset_twap_timestamp_from_vamm_state(vamm_state, asset_type);
			ensure!(now > asset_twap_timestamp, Error::<T>::AssetTwapTimestampIsMoreRecent);

			Ok(())
		}
		fn swap_sanity_check(
			config: &SwapConfigOf<T>,
			vamm_state: &VammStateOf<T>,
		) -> Result<(), DispatchError> {
			// We must ensure that the vamm is not closed before performing any swap.
			ensure!(!Self::is_vamm_closed(vamm_state, &None), Error::<T>::VammIsClosed);

			match config.direction {
				// If we intend to remove some asset amount from vamm, we must
				// have sufficient funds for it.
				Direction::Remove => match config.asset {
					AssetType::Base => ensure!(
						config.input_amount <= vamm_state.base_asset_reserves,
						Error::<T>::InsufficientFundsForTrade
					),
					AssetType::Quote => ensure!(
						config.input_amount <= vamm_state.quote_asset_reserves,
						Error::<T>::InsufficientFundsForTrade
					),
				},

				// If we intend to add some asset amount to the vamm, the
				// final amount must not overflow.
				Direction::Add => match config.asset {
					AssetType::Base => ensure!(
						config.input_amount.checked_add(&vamm_state.base_asset_reserves).is_some(),
						Error::<T>::TradeExtrapolatesMaximumSupportedAmount
					),
					AssetType::Quote => ensure!(
						config.input_amount.checked_add(&vamm_state.quote_asset_reserves).is_some(),
						Error::<T>::TradeExtrapolatesMaximumSupportedAmount
					),
				},
			};

			Ok(())
		}
	}

	// Helper functions - low-level functionality
	impl<T: Config> Pallet<T> {
		fn update_vamm_twap(
			vamm_id: VammIdOf<T>,
			vamm_state: &mut VammStateOf<T>,
			asset_type: AssetType,
			now: &Option<MomentOf<T>>,
		) -> Result<DecimalOf<T>, DispatchError> {
			// Sanity checks
			Self::update_twap_sanity_check(vamm_state, asset_type, None, now)?;

			let asset_twap_timestamp =
				Self::asset_twap_timestamp_from_vamm_state(vamm_state, asset_type);
			let asset_twap = Self::asset_twap_from_vamm_state(vamm_state, asset_type);
			let new_twap = Self::calculate_twap(
				now,
				asset_twap_timestamp,
				vamm_state.funding_period,
				Self::do_get_price(vamm_state, asset_type)?,
				Self::balance_to_decimal(asset_twap),
			)?;

			Self::do_update_twap(vamm_id, vamm_state, asset_type, new_twap, now)
		}

		fn calculate_twap(
			now: &Option<MomentOf<T>>,
			last_twap_timestamp: MomentOf<T>,
			funding_period: MomentOf<T>,
			new_price: DecimalOf<T>,
			old_price: DecimalOf<T>,
		) -> Result<DecimalOf<T>, DispatchError> {
			let now = Self::now(now);
			let weight_now: MomentOf<T> = std::cmp::max(
				1_u64.into(),
				now.checked_sub(&last_twap_timestamp)
					.ok_or(Error::<T>::AssetTwapTimestampIsMoreRecent)?,
			);

			// TODO(Cardosaum): Won't this subtraction cause a failure everytime
			// if we don't update twap for a long period of time?  for example,
			// if `funding_period = 1 hour`, and we pass 2 or more hours without
			// updating the twap, doesn't it mean we will throw an error each
			// time we try to subtract `funding_period -  weight_now`?
			let weight_last_twap: MomentOf<T> = std::cmp::max(
				1_u64.into(),
				funding_period
					.checked_sub(&weight_now)
					.ok_or(Error::<T>::FailedToComputeLastTwapWeight)?,
			);

			Self::calculate_exponential_moving_average(
				new_price,
				weight_now,
				old_price,
				weight_last_twap,
			)
		}

		fn calculate_exponential_moving_average(
			x1: DecimalOf<T>,
			w1: MomentOf<T>,
			x2: DecimalOf<T>,
			w2: MomentOf<T>,
		) -> Result<DecimalOf<T>, DispatchError> {
			let w1_u256 = U256::from(w1.into());
			let w2_u256 = U256::from(w2.into());
			let denominator = w1_u256.checked_add(w2_u256).ok_or(ArithmeticError::Overflow)?;
			let xw1 = Self::balance_to_u256(x1.into_inner())?
				.checked_mul(w1_u256)
				.ok_or(ArithmeticError::Overflow)?;
			let xw2 = Self::balance_to_u256(x2.into_inner())?
				.checked_mul(w2_u256)
				.ok_or(ArithmeticError::Overflow)?;

			let twap_u256 = xw1
				.checked_add(xw2)
				.ok_or(ArithmeticError::Overflow)?
				.checked_div(denominator)
				.ok_or(ArithmeticError::DivisionByZero)?;
			let twap = Self::balance_to_decimal(Self::u256_to_balance(twap_u256)?);
			Ok(twap)
		}

		fn get_vamm_state(vamm_id: &VammIdOf<T>) -> Result<VammStateOf<T>, DispatchError> {
			// Requested vamm must exists and be retrievable.
			ensure!(VammMap::<T>::contains_key(vamm_id), Error::<T>::VammDoesNotExist);
			let vamm_state = VammMap::<T>::get(vamm_id).ok_or(Error::<T>::FailToRetrieveVamm)?;
			Ok(vamm_state)
		}

		fn is_vamm_closed(vamm_state: &VammStateOf<T>, now: &Option<MomentOf<T>>) -> bool {
			let now = Self::now(now);
			match vamm_state.closed {
				Some(timestamp) => now >= timestamp,
				None => false,
			}
		}

		fn asset_twap_from_vamm_state(
			vamm_state: &VammStateOf<T>,
			asset_type: AssetType,
		) -> BalanceOf<T> {
			match asset_type {
				AssetType::Base => vamm_state.base_asset_twap,
				AssetType::Quote => vamm_state.quote_asset_twap,
			}
		}

		fn asset_twap_timestamp_from_vamm_state(
			vamm_state: &VammStateOf<T>,
			asset_type: AssetType,
		) -> MomentOf<T> {
			match asset_type {
				AssetType::Base => vamm_state.base_asset_twap_timestamp,
				AssetType::Quote => vamm_state.quote_asset_twap_timestamp,
			}
		}

		fn now(now: &Option<MomentOf<T>>) -> MomentOf<T> {
			match now {
				Some(now) => *now,
				None => T::TimeProvider::now().as_secs().into(),
			}
		}

		fn balance_to_decimal(value: BalanceOf<T>) -> DecimalOf<T> {
			DecimalOf::<T>::from_inner(value)
		}

		fn moment_to_decimal(value: MomentOf<T>) -> DecimalOf<T> {
			DecimalOf::<T>::from_inner(value.into().into())
		}

		fn balance_to_u128(value: BalanceOf<T>) -> Result<u128, DispatchError> {
			Ok(TryInto::<u128>::try_into(value).ok().ok_or(ArithmeticError::Overflow)?)
		}

		fn balance_to_u256(value: BalanceOf<T>) -> Result<U256, DispatchError> {
			Ok(U256::from(Self::balance_to_u128(value)?))
		}

		fn u256_to_u128(value: U256) -> Result<u128, DispatchError> {
			Ok(TryInto::<u128>::try_into(value).ok().ok_or(ArithmeticError::Overflow)?)
		}

		fn u256_to_balance(value: U256) -> Result<BalanceOf<T>, DispatchError> {
			Ok(Self::u256_to_u128(value)?.try_into().ok().ok_or(ArithmeticError::Overflow)?)
		}

		pub fn compute_invariant(
			base: BalanceOf<T>,
			quote: BalanceOf<T>,
		) -> Result<U256, DispatchError> {
			// Neither base nor quote asset are allowed to be zero since it
			// would mean the invariant would also be zero.
			ensure!(!base.is_zero(), Error::<T>::BaseAssetReserveIsZero);
			ensure!(!quote.is_zero(), Error::<T>::QuoteAssetReserveIsZero);

			let base_u256 = Self::balance_to_u256(base)?;
			let quote_u256 = Self::balance_to_u256(quote)?;
			let invariant = base_u256
				.checked_mul(quote_u256)
				.ok_or(Error::<T>::FailedToDeriveInvariantFromBaseAndQuoteAsset)?;

			ensure!(!invariant.is_zero(), Error::<T>::InvariantIsZero);

			Ok(invariant)
		}
	}
}
