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
		AssetType, Direction, SwapConfig, SwapSimulationConfig, Vamm, VammConfig,
	};
	use frame_support::{pallet_prelude::*, sp_std::fmt::Debug, transactional, Blake2_128Concat};
	use num_integer::Integer;
	use sp_arithmetic::traits::Unsigned;
	use sp_core::U256;
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, One, Zero},
		ArithmeticError, FixedPointNumber,
	};

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

		/// Timestamp to be used for twap calculations and market closing.
		type Timestamp: Default
			+ Clone
			+ Copy
			+ Debug
			+ FullCodec
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
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
			+ MaybeSerializeDeserialize
			+ Ord
			+ Parameter
			+ Unsigned
			+ Zero;

		/// Signed decimal fixed point number.
		type Decimal: FullCodec + MaxEncodedLen + TypeInfo + FixedPointNumber;

		/// The Integer type used by the pallet for computing swaps.
		type Integer: Integer;
	}

	// ----------------------------------------------------------------------------------------------------
	//                                             Pallet Types
	// ----------------------------------------------------------------------------------------------------

	type BalanceOf<T> = <T as Config>::Balance;
	type IntegerOf<T> = <T as Config>::Integer;
	type TimestampOf<T> = <T as Config>::Timestamp;
	type VammIdOf<T> = <T as Config>::VammId;
	type SwapConfigOf<T> = SwapConfig<VammIdOf<T>, BalanceOf<T>>;
	type SwapSimulationConfigOf<T> = SwapSimulationConfig<VammIdOf<T>, BalanceOf<T>>;
	type VammConfigOf<T> = VammConfig<BalanceOf<T>>;
	type VammStateOf<T> = VammState<BalanceOf<T>, TimestampOf<T>>;

	/// Represents the direction a of a position.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	pub enum SwapDirection {
		Add,
		Remove,
	}

	/// Data relating to the state of a virtual market.
	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Copy, PartialEq, Debug)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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
		/// Tried to set [`base_asset_reserves`](VammState) to zero.
		BaseAssetReserveIsZero,
		/// Tried to set [`quote_asset_reserves`](VammState) to zero.
		QuoteAssetReserveIsZero,
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
		pub vamms: Vec<(VammIdOf<T>, VammState<BalanceOf<T>, TimestampOf<T>>)>,
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
		type Decimal = T::Decimal;
		type Integer = IntegerOf<T>;
		type SwapConfig = SwapConfigOf<T>;
		type SwapSimulationConfig = SwapSimulationConfigOf<T>;
		type VammConfig = VammConfigOf<T>;
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
		fn create(config: &VammConfigOf<T>) -> Result<VammIdOf<T>, DispatchError> {
			// TODO(Cardosaum)
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

		/// Gets the current price of the __base__ or __quote__ asset in a vamm.
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
		///  __base__ or __quote__)
		///
		/// ## Returns
		/// The price of __base__ asset in relation to __quote__
		/// (or vice-versa).
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
		) -> Result<BalanceOf<T>, DispatchError> {
			// Requested vamm must exist.
			ensure!(VammMap::<T>::contains_key(vamm_id), Error::<T>::VammDoesNotExist);

			let vamm_state = VammMap::<T>::get(vamm_id).ok_or(Error::<T>::FailToRetrieveVamm)?;

			match asset_type {
				AssetType::Base => Ok(vamm_state
					.quote_asset_reserves
					.checked_mul(&vamm_state.peg_multiplier)
					.ok_or(ArithmeticError::Overflow)?
					.checked_div(&vamm_state.base_asset_reserves)
					.ok_or(ArithmeticError::DivisionByZero)?),

				AssetType::Quote => Ok(vamm_state
					.base_asset_reserves
					.checked_mul(&vamm_state.peg_multiplier)
					.ok_or(ArithmeticError::Overflow)?
					.checked_div(&vamm_state.quote_asset_reserves)
					.ok_or(ArithmeticError::DivisionByZero)?),
			}
		}

		#[allow(unused_variables)]
		fn get_twap(vamm_id: &VammIdOf<T>) -> Result<Self::Decimal, DispatchError> {
			todo!()
		}

		fn swap(config: &SwapConfigOf<T>) -> Result<Self::Balance, DispatchError> {
			// Get Vamm state.
			let mut vamm_state = Self::get_vamm_state(&config.vamm_id)?;

			// Perform required sanity checks.
			Self::swap_sanity_check(config, &vamm_state)?;

			// Delegate swap to helper functions.
			match config.asset {
				AssetType::Quote => Self::swap_quote_asset(config, &mut vamm_state),
				AssetType::Base => Self::swap_base_asset(config, &mut vamm_state),
			}
		}

		#[allow(unused_variables)]
		fn swap_simulation(
			config: &SwapSimulationConfigOf<T>,
		) -> Result<IntegerOf<T>, DispatchError> {
			todo!()
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
		fn swap_quote_asset(
			config: &SwapConfigOf<T>,
			vamm_state: &mut VammStateOf<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			// TODO(Cardosaum): Is it needed to divide the quote asset by the
			// peg_multiplier the way Drift does?
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

			let base_asset_amount = initial_base_asset_reserve
				.checked_sub(&swap_amount.output_amount)
				.ok_or(ArithmeticError::Underflow)?;

			// TODO(Cardosaum): Return an `IntegerOf<T>` rather than Balance
			Ok(base_asset_amount)
		}

		fn swap_base_asset(
			config: &SwapConfigOf<T>,
			vamm_state: &mut VammStateOf<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			let initial_quote_asset_reserve = vamm_state.quote_asset_reserves;
			let swap_amount = Self::calculate_swap_asset(
				&config.input_amount,
				&vamm_state.base_asset_reserves,
				&config.direction,
				vamm_state,
			)?;

			vamm_state.base_asset_reserves = swap_amount.input_amount;
			vamm_state.quote_asset_reserves = swap_amount.output_amount;

			Self::calculate_quote_asset_amount_swapped(
				&initial_quote_asset_reserve,
				&swap_amount.output_amount,
				&config.direction,
				&vamm_state,
			)
		}

		fn calculate_swap_asset(
			swap_amount: &BalanceOf<T>,
			input_asset_amount: &BalanceOf<T>,
			direction: &Direction,
			vamm_state: &VammStateOf<T>,
		) -> Result<CalculateSwapAsset<T>, DispatchError> {
			// NOTE(Cardosaum): There is still the problem that, if Balance is
			// U256 or greater, this would overflow. (does it matter?)
			let base_asset_reserves_u256 = Self::balance_to_u256(vamm_state.base_asset_reserves)?;
			let quote_asset_reserves_u256 = Self::balance_to_u256(vamm_state.quote_asset_reserves)?;
			let invariant = base_asset_reserves_u256
				.checked_mul(quote_asset_reserves_u256)
				.ok_or(ArithmeticError::Overflow)?;

			let new_input_amount = match direction {
				Direction::Add =>
					input_asset_amount.checked_add(swap_amount).ok_or(ArithmeticError::Overflow)?,

				Direction::Remove =>
					input_asset_amount.checked_sub(swap_amount).ok_or(ArithmeticError::Overflow)?,
			};
			let new_input_amount_u256 = Self::balance_to_u256(new_input_amount)?;

			let new_output_amount_u256 = invariant
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
		fn swap_sanity_check(
			config: &SwapConfigOf<T>,
			vamm_state: &VammStateOf<T>,
		) -> Result<(), DispatchError> {
			// TODO(Cardosaum): Implement check based on time to assess if vamm
			// is operational. Essentialy, check if `time.now() <
			// vamm_state.closed`, in case vamm_state is Some(time).
			//
			// The vamm must not be closed

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
		fn get_vamm_state(vamm_id: &VammIdOf<T>) -> Result<VammStateOf<T>, DispatchError> {
			// Requested vamm must exists and be retrievable.
			ensure!(VammMap::<T>::contains_key(vamm_id), Error::<T>::VammDoesNotExist);
			let vamm_state = VammMap::<T>::get(vamm_id).ok_or(Error::<T>::FailToRetrieveVamm)?;
			Ok(vamm_state)
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
	}
}

// ----------------------------------------------------------------------------------------------------
//                                              Unit Tests
// ----------------------------------------------------------------------------------------------------
