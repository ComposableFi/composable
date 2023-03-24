//! Oracle tells prices.
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
#![warn(clippy::unseparated_literal_suffix, clippy::disallowed_types)]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
pub use pallet::*;

mod validation;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::validation::{ValidBlockInterval, ValidMaxAnswer, ValidMinAnswers, ValidThreshold};
	pub use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_support::{
		abstractions::{
			nonce::Nonce,
			utils::{
				increment::{Increment, SafeIncrement},
				start_at::ZeroInit,
			},
		},
		math::safe::{safe_multiply_by_rational, SafeDiv},
		validation::Validated,
	};
	use composable_traits::{
		currency::{BalanceLike, LocalAssets},
		oracle::{Oracle, Price, RewardTracker},
		time::MS_PER_YEAR_NAIVE,
	};
	use frame_support::{
		dispatch::{DispatchClass, DispatchResult, DispatchResultWithPostInfo, Pays},
		pallet_prelude::*,
		traits::{
			BalanceStatus, Currency, EnsureOrigin,
			ExistenceRequirement::{AllowDeath, KeepAlive},
			ReservableCurrency, Time,
		},
		transactional, PalletId,
	};
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendSignedTransaction, SignedPayload, Signer,
			SigningTypes,
		},
		pallet_prelude::*,
	};
	use lite_json::json::JsonValue;
	use scale_info::TypeInfo;
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		offchain::{http, Duration},
		traits::{
			AccountIdConversion, AtLeast32Bit, AtLeast32BitUnsigned, CheckedAdd, CheckedDiv,
			CheckedMul, CheckedSub, Saturating, UniqueSaturatedInto as _, Zero,
		},
		AccountId32, ArithmeticError, FixedPointNumber, FixedU128, KeyTypeId as CryptoKeyTypeId,
		PerThing, Percent, RuntimeDebug,
	};
	use sp_std::{
		borrow::ToOwned, collections::btree_set::BTreeSet, fmt::Debug, str, vec, vec::Vec,
	};

	// Key Id for location of signer key in keystore
	pub const KEY_ID: [u8; 4] = *b"orac";
	pub const KEY_TYPE: KeyTypeId = KeyTypeId(KEY_ID);
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(KEY_ID);

	pub mod crypto {
		use super::KEY_TYPE;
		use sp_core::sr25519::Signature as Sr25519Signature;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify,
			MultiSignature, MultiSigner,
		};
		app_crypto!(sr25519, KEY_TYPE);

		pub struct BathurstStId;
		// implemented for ocw-runtime
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for BathurstStId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}

		impl
			frame_system::offchain::AppCrypto<
				<Sr25519Signature as Verify>::Signer,
				Sr25519Signature,
			> for BathurstStId
		{
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Balance: BalanceLike + From<u128>;
		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId, Balance = Self::Balance>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ From<u128>
			+ Into<u128>
			+ Debug
			+ Default
			+ TypeInfo;
		type PriceValue: Default
			+ Parameter
			+ Codec
			+ Copy
			+ Ord
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ AtLeast32BitUnsigned
			+ From<u64>
			+ From<u128>
			+ Into<u128>
			+ MaxEncodedLen
			+ Zero;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// The Min stake for an oracle
		type MinStake: Get<BalanceOf<Self>>;
		/// The delay to withdraw stake as an oracle
		type StakeLock: Get<Self::BlockNumber>;
		/// Blocks until price is considered stale
		type StalePrice: Get<Self::BlockNumber>;
		/// Origin to add new price types
		type AddOracle: EnsureOrigin<Self::RuntimeOrigin>;
		/// Origin to manage rewards
		type RewardOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Upper bound for max answers for a price
		type MaxAnswerBound: Get<u32>;
		/// Upper bound for total assets available for the oracle
		type MaxAssetsCount: Get<u32>;
		/// Slashed stakes are transferred to treasury.
		type TreasuryAccount: Get<Self::AccountId>;

		#[pallet::constant]
		type MaxHistory: Get<u32>;

		#[pallet::constant]
		type TwapWindow: Get<u16>;

		#[pallet::constant]
		type MaxPrePrices: Get<u32>;

		#[pallet::constant]
		type MsPerBlock: Get<u64>;

		/// The weight information of this pallet.
		type WeightInfo: WeightInfo;
		type LocalAssets: LocalAssets<Self::AssetId>;

		/// Type for timestamps
		type Moment: AtLeast32Bit + Parameter + Default + Copy + MaxEncodedLen + FullCodec;

		/// The time provider.
		type Time: Time<Moment = Self::Moment>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[derive(Encode, Decode, MaxEncodedLen, Default, Debug, PartialEq, Eq, TypeInfo, Clone)]
	pub struct Withdraw<Balance, BlockNumber> {
		pub stake: Balance,
		pub unlock_block: BlockNumber,
	}

	#[derive(
		Encode, Decode, MaxEncodedLen, Clone, Copy, Default, Debug, PartialEq, Eq, TypeInfo,
	)]
	pub struct PrePrice<PriceValue, BlockNumber, AccountId> {
		/// The price of an asset, normalized to 12 decimals.
		pub price: PriceValue,
		/// The block the price was submitted at.
		pub block: BlockNumber,
		/// The account that submitted the price.
		pub who: AccountId,
	}

	#[derive(Encode, Decode, MaxEncodedLen, Default, Debug, PartialEq, Eq, Clone, TypeInfo)]
	pub struct AssetInfo<Percent, BlockNumber, Balance> {
		pub threshold: Percent,
		pub min_answers: u32,
		pub max_answers: u32,
		pub block_interval: BlockNumber,
		/// Reward allocation weight for this asset type out of the total block reward.
		pub reward_weight: Balance,
		pub slash: Balance,
		pub emit_price_changes: bool,
	}

	type BalanceOf<T> = <T as Config>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn assets_count)]
	#[allow(clippy::disallowed_types)] // Default asset count of 0 is valid in this context
	/// Total amount of assets
	// TODO: Replace SafeIncrement with IncrementToMax
	pub type AssetsCount<T: Config> =
		StorageValue<_, u32, ValueQuery, Nonce<ZeroInit, SafeIncrement>>;

	#[pallet::storage]
	#[pallet::getter(fn reward_tracker_store)]
	#[allow(clippy::disallowed_types)]
	/// Rewarding history for Oracles. Used for calculating the current block reward.
	pub type RewardTrackerStore<T: Config> =
		StorageValue<_, RewardTracker<BalanceOf<T>, T::Moment>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn signer_to_controller)]
	/// Mapping signing key to controller key
	pub type SignerToController<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn controller_to_signer)]
	/// Mapping Controller key to signer key
	pub type ControllerToSigner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn declared_withdraws)]
	/// Tracking withdrawal requests
	pub type DeclaredWithdraws<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Withdraw<BalanceOf<T>, T::BlockNumber>>;

	#[pallet::storage]
	#[pallet::getter(fn oracle_stake)]
	/// Mapping of signing key to stake
	pub type OracleStake<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn answer_in_transit)]
	/// Mapping of slash amounts currently in transit
	pub type AnswerInTransit<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn prices)]
	// Price<_, _> has a default value which is checked against in a few places.
	// REVIEW: (benluelo) I think there's probably a better way to use this with an OptionQuery,
	// instead of checking against defaults.
	/// Price for an asset and blocknumber asset was updated at
	#[allow(clippy::disallowed_types)]
	pub type Prices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Price<T::PriceValue, T::BlockNumber>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn price_history)]
	#[allow(clippy::disallowed_types)] // default history for an asset is an empty list, which is valid in this context.
	/// Price for an asset and blocknumber asset was updated at
	pub type PriceHistory<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		BoundedVec<Price<T::PriceValue, T::BlockNumber>, T::MaxHistory>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pre_prices)]
	#[allow(clippy::disallowed_types)] // default history for an asset is an empty list, which is valid in this context.
	/// Temporary prices before aggregated
	pub type PrePrices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		BoundedVec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>, T::MaxPrePrices>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn asset_info)]
	/// Information about asset, including precision threshold and max/min answers
	pub type AssetsInfo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Asset info created or changed. \[asset_id, threshold, min_answers, max_answers,
		/// block_interval, reward, slash\]
		AssetInfoChange(T::AssetId, Percent, u32, u32, T::BlockNumber, BalanceOf<T>, BalanceOf<T>),
		/// Signer was set. \[signer, controller\]
		SignerSet(T::AccountId, T::AccountId),
		/// Stake was added. \[added_by, amount_added, total_amount\]
		StakeAdded(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// Stake removed. \[removed_by, amount, block_number\]
		StakeRemoved(T::AccountId, BalanceOf<T>, T::BlockNumber),
		/// Stake reclaimed. \[reclaimed_by, amount\]
		StakeReclaimed(T::AccountId, BalanceOf<T>),
		/// Price submitted by oracle. \[oracle_address, asset_id, price\]
		PriceSubmitted(T::AccountId, T::AssetId, T::PriceValue),
		/// Oracle slashed. \[oracle_address, asset_id, amount\]
		UserSlashed(T::AccountId, T::AssetId, BalanceOf<T>),
		/// Oracle rewarded. \[oracle_address, asset_id, price\]
		OracleRewarded(T::AccountId, T::AssetId, BalanceOf<T>),
		/// Rewarding Started \[rewarding start timestamp]
		RewardingAdjustment(T::Moment),
		/// Answer from oracle removed for staleness. \[oracle_address, price\]
		AnswerPruned(T::AccountId, T::PriceValue),
		/// Price changed by oracle \[asset_id, price\]
		PriceChanged(T::AssetId, T::PriceValue),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Unknown
		Unknown,
		/// No Permission
		NoPermission,
		/// No stake for oracle
		NoStake,
		/// Stake is locked try again later
		StakeLocked,
		/// Not enough oracle stake for action
		NotEnoughStake,
		/// Not Enough Funds to complete action
		NotEnoughFunds,
		/// Invalid asset id
		InvalidAssetId,
		/// Price already submitted
		AlreadySubmitted,
		/// Max prices already reached
		MaxPrices,
		/// Price has not been requested
		PriceNotRequested,
		/// Signer has not been set
		UnsetSigner,
		/// Signer has already been set
		AlreadySet,
		/// No controller has been set
		UnsetController,
		/// This controller is already in use
		ControllerUsed,
		/// This signer is already in use
		SignerUsed,
		/// Error avoids a panic
		AvoidPanic,
		/// Max answers have been exceeded
		ExceedMaxAnswers,
		/// Invalid min answers
		InvalidMinAnswers,
		// max answers less than min answers
		MaxAnswersLessThanMinAnswers,
		/// Threshold exceeded
		ExceedThreshold,
		/// Asset count exceeded
		ExceedAssetsCount,
		/// Price not found
		PriceNotFound,
		/// Stake exceeded
		ExceedStake,
		/// Price weight must sum to 100
		MustSumTo100,
		/// Too many weighted averages requested
		DepthTooLarge,
		ArithmeticError,
		/// Block interval is less then stale price
		BlockIntervalLength,
		/// There was an error transferring
		TransferError,
		MaxHistory,
		MaxPrePrices,
		/// Rewarding has not started
		NoRewardTrackerSet,
		/// Annual rewarding cost too high
		AnnualRewardLessThanAlreadyRewarded,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block: T::BlockNumber) -> Weight {
			Self::reset_reward_tracker_if_expired();
			Self::update_prices(block)
		}

		fn offchain_worker(_block_number: T::BlockNumber) {
			log::info!("Offchain worker triggered");
			Self::check_requests();
		}
	}

	impl<T: Config> Oracle for Pallet<T> {
		type AssetId = T::AssetId;
		type Balance = T::PriceValue;
		type Timestamp = <T as frame_system::Config>::BlockNumber;
		type LocalAssets = T::LocalAssets;
		type MaxAnswerBound = T::MaxAnswerBound;
		type TwapWindow = T::TwapWindow;

		fn get_price(
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Price<Self::Balance, Self::Timestamp>, DispatchError> {
			let Price { price, block } =
				Prices::<T>::try_get(asset_id).map_err(|_| Error::<T>::PriceNotFound)?;
			// dbg!(&price);
			let price = Self::quote(asset_id, price, amount)?;
			// dbg!(&price);
			Ok(Price { price, block })
		}

		/// Currently using a flat distribution of weights.
		fn get_twap_for_amount(
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			let prices_length = Self::price_history(asset_id).len();
			let twap_window: usize = <Self as Oracle>::TwapWindow::get().into();
			if twap_window > prices_length + 1 {
				Self::get_price(asset_id, amount).map(|p| p.price)
			} else {
				let price = Self::get_twap(asset_id, twap_window)?;
				Self::quote(asset_id, price, amount)
			}
		}

		fn get_ratio(
			pair: composable_traits::defi::CurrencyPair<Self::AssetId>,
		) -> Result<sp_runtime::FixedU128, DispatchError> {
			let base: u128 =
				Self::get_price(pair.base, T::LocalAssets::unit(pair.base)?)?.price.into();
			let quote: u128 =
				Self::get_price(pair.quote, T::LocalAssets::unit(pair.base)?)?.price.into();

			let base = FixedU128::saturating_from_integer(base);
			let quote = FixedU128::saturating_from_integer(quote);
			Ok(base.safe_div(&quote)?)
		}

		fn get_price_inverse(
			asset_id: Self::AssetId,
			amount: Self::Balance,
		) -> Result<Self::Balance, DispatchError> {
			// assume the following parameters:
			//
			// asset A has 3 decimal places
			// 1 unit of asset ACOIN costs 4 units of default quote asset (4_000_000_000_000)
			// the price for 1/1_000th of asset A_COIN (the smallest unit possible, we'll call it
			// ACENT) is then: 4_000_000_000_000/1_000 == 4_000_000_000; or 4/1000ths (1/250th) of 1
			// unit of default quote asset.
			//
			// if we want 10 units of default quote asset:
			// 10_000_000_000_000 * (1_000 / 4_000_000_000_000) = 2_500
			// 10^12 * (10^3 / 4^12) = 2_500
			//
			// 2_500 ACENTS are needed to pay for 10 default quote asset.
			let unit = T::LocalAssets::unit(asset_id)?;
			let asset_price_per_unit: u128 = Self::get_price(asset_id, unit)?.price.into();

			let amount: u128 = amount.into();
			let result = safe_multiply_by_rational(amount, unit.into(), asset_price_per_unit)?;
			let result: u64 = result.try_into().map_err(|_| ArithmeticError::Overflow)?;

			Ok(result.into())
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Permissioned call to add an asset
		///
		/// - `asset_id`: Id for the asset
		/// - `threshold`: Percent close to mean to be rewarded
		/// - `min_answers`: Min answers before aggregation
		/// - `max_answers`: Max answers to aggregate
		/// - `block_interval`: blocks until oracle triggered
		/// - `reward`: reward amount for correct answer
		/// - `slash`: slash amount for bad answer
		/// - `emit_price_changes`: emit PriceChanged event when asset price changes
		///
		/// Emits `DepositEvent` event when successful.
		#[pallet::weight(T::WeightInfo::add_asset_and_info())]
		pub fn add_asset_and_info(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			threshold: Validated<Percent, ValidThreshold>,
			min_answers: Validated<u32, ValidMinAnswers>,
			max_answers: Validated<u32, ValidMaxAnswer<T::MaxAnswerBound>>,
			block_interval: Validated<T::BlockNumber, ValidBlockInterval<T::StalePrice>>,
			reward_weight: BalanceOf<T>,
			slash: BalanceOf<T>,
			emit_price_changes: bool,
		) -> DispatchResultWithPostInfo {
			T::AddOracle::ensure_origin(origin)?;

			ensure!(*max_answers >= *min_answers, Error::<T>::MaxAnswersLessThanMinAnswers);

			ensure!(
				AssetsCount::<T>::get() < T::MaxAssetsCount::get(),
				Error::<T>::ExceedAssetsCount
			);

			let asset_info = AssetInfo {
				threshold: *threshold,
				min_answers: *min_answers,
				max_answers: *max_answers,
				block_interval: *block_interval,
				reward_weight,
				slash,
				emit_price_changes,
			};
			// track reward total weight for all assets
			let mut reward_tracker = RewardTrackerStore::<T>::get().unwrap_or_default();
			if let Some(current_asset_info) = Self::asset_info(asset_id) {
				reward_tracker.total_reward_weight = reward_tracker.total_reward_weight +
					reward_weight - current_asset_info
					.reward_weight;
			} else {
				AssetsCount::<T>::increment()?;
				reward_tracker.total_reward_weight += reward_weight;
			}
			RewardTrackerStore::<T>::set(Option::from(reward_tracker));

			AssetsInfo::<T>::insert(asset_id, asset_info);
			Self::deposit_event(Event::AssetInfoChange(
				asset_id,
				*threshold,
				*min_answers,
				*max_answers,
				*block_interval,
				reward_weight,
				slash,
			));
			Ok(().into())
		}

		/// Call for a signer to be set, called from controller, adds stake.
		///
		/// - `signer`: signer to tie controller to
		///
		/// Emits `SignerSet` and `StakeAdded` events when successful.
		#[pallet::weight(T::WeightInfo::set_signer())]
		pub fn set_signer(
			origin: OriginFor<T>,
			signer: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let current_controller = ControllerToSigner::<T>::get(&who);
			let current_signer = SignerToController::<T>::get(&signer);

			ensure!(current_controller.is_none(), Error::<T>::ControllerUsed);
			ensure!(current_signer.is_none(), Error::<T>::SignerUsed);

			Self::do_add_stake(who.clone(), signer.clone(), T::MinStake::get())?;

			ControllerToSigner::<T>::insert(&who, signer.clone());
			SignerToController::<T>::insert(signer.clone(), who.clone());

			Self::deposit_event(Event::SignerSet(signer, who));
			Ok(().into())
		}

		/// Call to start rewarding Oracles.
		/// - `annual_cost_per_oracle`: Annual cost of an Oracle.
		/// - `num_ideal_oracles`: Number of ideal Oracles. This in fact should be higher than the
		///   actual ideal number so that the Oracles make a profit under ideal conditions.
		///
		/// Emits `RewardRateSet` event when successful.
		#[pallet::weight(T::WeightInfo::adjust_rewards())]
		pub fn adjust_rewards(
			origin: OriginFor<T>,
			annual_cost_per_oracle: BalanceOf<T>,
			num_ideal_oracles: u8,
		) -> DispatchResultWithPostInfo {
			T::RewardOrigin::ensure_origin(origin)?;
			let now = T::Time::now();
			let period: T::Moment = MS_PER_YEAR_NAIVE.unique_saturated_into();
			let mut reward_tracker = RewardTrackerStore::<T>::get().unwrap_or_default();
			if reward_tracker.start == Zero::zero() {
				reward_tracker.start = now;
				reward_tracker.period = period;
			}
			// calculate the current block reward by dividing the total possible reward by the
			// remaining number of blocks in the year.
			let elapsed_period = (now - reward_tracker.start) % period;
			let period_left: u64 = (period - elapsed_period).unique_saturated_into();
			let remaining_blocks_in_year: T::Balance = period_left
				.checked_div(T::MsPerBlock::get())
				.ok_or(ArithmeticError::DivisionByZero)?
				.unique_saturated_into();
			let new_annual_reward = annual_cost_per_oracle.saturating_mul(num_ideal_oracles.into());
			ensure!(
				new_annual_reward > reward_tracker.total_already_rewarded,
				Error::<T>::AnnualRewardLessThanAlreadyRewarded
			);
			reward_tracker.current_block_reward = (new_annual_reward -
				reward_tracker.total_already_rewarded)
				.checked_div(&remaining_blocks_in_year)
				.ok_or(ArithmeticError::Overflow)?;
			RewardTrackerStore::<T>::set(Option::from(reward_tracker));
			Self::deposit_event(Event::RewardingAdjustment(T::Time::now()));
			Ok(().into())
		}

		/// call to add more stake from a controller
		///
		/// - `stake`: amount to add to stake
		///
		/// Emits `StakeAdded` event when successful.
		#[pallet::weight(T::WeightInfo::add_stake())]
		pub fn add_stake(origin: OriginFor<T>, stake: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;

			Self::do_add_stake(who, signer, stake)?;

			Ok(().into())
		}

		/// Call to put in a claim to remove stake, called from controller
		///
		/// Emits `StakeRemoved` event when successful.
		#[pallet::weight(T::WeightInfo::remove_stake())]
		pub fn remove_stake(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;
			let block = frame_system::Pallet::<T>::block_number();
			let unlock_block = block + T::StakeLock::get();
			let stake = Self::oracle_stake(signer.clone()).ok_or(Error::<T>::NoStake)?;
			ensure!(stake > BalanceOf::<T>::zero(), Error::<T>::NoStake);
			let withdrawal = Withdraw { stake, unlock_block };
			OracleStake::<T>::remove(&signer);
			DeclaredWithdraws::<T>::insert(signer.clone(), withdrawal);
			Self::deposit_event(Event::StakeRemoved(signer, stake, unlock_block));
			Ok(().into())
		}

		/// Call to reclaim stake after proper time has passed, called from controller
		///
		/// Emits `StakeReclaimed` event when successful.
		#[pallet::weight(T::WeightInfo::reclaim_stake())]
		pub fn reclaim_stake(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;
			let block = frame_system::Pallet::<T>::block_number();
			let withdrawal = DeclaredWithdraws::<T>::get(&signer).ok_or(Error::<T>::Unknown)?;
			ensure!(block > withdrawal.unlock_block, Error::<T>::StakeLocked);
			DeclaredWithdraws::<T>::remove(&signer);
			T::Currency::unreserve(&signer, withdrawal.stake);
			let result = T::Currency::transfer(&signer, &who, withdrawal.stake, AllowDeath);
			ensure!(result.is_ok(), Error::<T>::TransferError);

			ControllerToSigner::<T>::remove(&who);
			SignerToController::<T>::remove(&signer);

			Self::deposit_event(Event::StakeReclaimed(signer, withdrawal.stake));
			Ok(().into())
		}
		/// Call to submit a price, gas is returned if extrinsic is successful.
		/// Should be called from offchain worker but can be called manually too.
		///
		/// This is an operational transaction.
		///
		/// - `price`: price to submit, normalized to 12 decimals
		/// - `asset_id`: id for the asset
		///
		/// Emits `PriceSubmitted` event when successful.
		#[pallet::weight((T::WeightInfo::submit_price(T::MaxAnswerBound::get()), DispatchClass::Operational))]
		pub fn submit_price(
			origin: OriginFor<T>,
			price: T::PriceValue,
			asset_id: T::AssetId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let author_stake = OracleStake::<T>::get(&who).unwrap_or_else(Zero::zero);
			ensure!(Self::is_requested(&asset_id), Error::<T>::PriceNotRequested);
			ensure!(
				author_stake >=
					T::MinStake::get().saturating_add(
						Self::answer_in_transit(&who).unwrap_or_else(Zero::zero)
					),
				Error::<T>::NotEnoughStake
			);
			let asset_info = Self::asset_info(asset_id).ok_or(Error::<T>::InvalidAssetId)?;
			ensure!(author_stake >= asset_info.slash, Error::<T>::NotEnoughStake);

			PrePrices::<T>::try_mutate(asset_id, |current_prices| -> Result<(), DispatchError> {
				// current_prices.len() can be casted to u32 safely because current_prices.len() is
				// limited to u32::MAX (see AssetsInfo::<T>::get(asset_id).max_answers).
				if current_prices.len() as u32 >= asset_info.max_answers {
					return Err(Error::<T>::MaxPrices.into())
				}
				if current_prices.iter().any(|candidate| candidate.who == who) {
					return Err(Error::<T>::AlreadySubmitted.into())
				}
				current_prices
					.try_push(PrePrice {
						price,
						block: frame_system::Pallet::<T>::block_number(),
						who: who.clone(),
					})
					.map_err(|_| Error::<T>::MaxPrePrices)?;

				Ok(())
			})?;

			AnswerInTransit::<T>::mutate(&who, |transit| {
				*transit =
					Some(transit.unwrap_or_else(Zero::zero).saturating_add(asset_info.slash));
			});

			Self::deposit_event(Event::PriceSubmitted(who, asset_id, price));
			Ok(Pays::No.into())
		}
	}

	/// Payload used by this example crate to hold price
	/// data required to submit a transaction.
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
	pub struct PricePayload<Public, BlockNumber> {
		block_number: BlockNumber,
		price: u32,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for PricePayload<T::Public, T::BlockNumber> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn do_add_stake(
			who: T::AccountId,
			signer: T::AccountId,
			stake: BalanceOf<T>,
		) -> DispatchResult {
			let amount_staked = Self::oracle_stake(signer.clone())
				.unwrap_or_else(|| 0_u32.into())
				.checked_add(&stake)
				.ok_or(Error::<T>::ExceedStake)?;
			T::Currency::transfer(&who, &signer, stake, KeepAlive)?;
			T::Currency::reserve(&signer, stake)?;
			OracleStake::<T>::insert(&signer, amount_staked);
			Self::deposit_event(Event::StakeAdded(signer, stake, amount_staked));
			Ok(())
		}

		pub fn handle_payout(
			pre_prices: &[PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>],
			price: T::PriceValue,
			asset_id: T::AssetId,
			asset_info: &AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
		) -> DispatchResult {
			let mut rewarded_oracles = BTreeSet::new();
			for answer in pre_prices {
				// TODO vim: duplicated code could be refactored to do these accuracy calculations
				// once
				let accuracy: Percent = if answer.price < price {
					PerThing::from_rational(answer.price, price)
				} else {
					let adjusted_number = price.saturating_sub(answer.price - price);
					PerThing::from_rational(adjusted_number, price)
				};
				let min_accuracy = asset_info.threshold;
				if accuracy < min_accuracy {
					let slash_amount = asset_info.slash;
					let new_amount_staked = Self::oracle_stake(answer.who.clone())
						.unwrap_or_else(|| 0_u32.into())
						.saturating_sub(slash_amount);
					OracleStake::<T>::insert(&answer.who, new_amount_staked);
					let result = T::Currency::repatriate_reserved(
						&answer.who,
						&T::TreasuryAccount::get(),
						slash_amount,
						BalanceStatus::Free,
					);
					match result {
						Ok(remaining_val) =>
							if remaining_val > BalanceOf::<T>::zero() {
								log::warn!("Only slashed {:?}", slash_amount - remaining_val);
							},
						Err(e) => {
							log::warn!("Failed to slash {:?} due to {:?}", answer.who, e);
						},
					}
					Self::deposit_event(Event::UserSlashed(
						answer.who.clone(),
						asset_id,
						slash_amount,
					));
				} else {
					let controller = SignerToController::<T>::get(&answer.who)
						.unwrap_or_else(|| answer.who.clone());
					rewarded_oracles.insert((answer.who.clone(), controller.clone()));
				}
				Self::remove_price_in_transit(&answer.who, asset_info)
			}
			if let Some(mut reward_tracker) = Self::get_reward_tracker_if_enabled() {
				// divide the per asset reward(by weight) by the number of oracles
				let reward_amount_per_oracle: T::Balance = safe_multiply_by_rational(
					reward_tracker.current_block_reward.unique_saturated_into(),
					asset_info.reward_weight.unique_saturated_into(),
					reward_tracker.total_reward_weight.unique_saturated_into(),
				)?
				.checked_div(rewarded_oracles.len() as u128)
				.ok_or(ArithmeticError::DivisionByZero)?
				.into();
				if !reward_amount_per_oracle.is_zero() {
					for accounts in rewarded_oracles {
						Self::transfer_reward(
							accounts.0,
							accounts.1,
							asset_id,
							reward_amount_per_oracle,
						)?;
						// track the total being rewarded
						reward_tracker.total_already_rewarded = reward_tracker
							.total_already_rewarded
							.saturating_add(reward_amount_per_oracle);
					}
					RewardTrackerStore::<T>::mutate(|r| *r = Some(reward_tracker));
				}
			} else {
				log::warn!("Oracle rewarding not enabled");
			}
			Ok(())
		}

		fn get_reward_tracker_if_enabled(
		) -> Option<RewardTracker<<T as Config>::Balance, <T as Config>::Moment>> {
			RewardTrackerStore::<T>::get().and_then(|r| {
				if r.start != Zero::zero() {
					Some(r)
				} else {
					None
				}
			})
		}

		fn transfer_reward(
			who: T::AccountId,
			controller: T::AccountId,
			asset_id: T::AssetId,
			reward_amount: BalanceOf<T>,
		) -> DispatchResult {
			T::Currency::transfer(&Self::account_id(), &controller, reward_amount, KeepAlive)?;
			Self::deposit_event(Event::OracleRewarded(who, asset_id, reward_amount));
			Ok(())
		}

		pub fn reset_reward_tracker_if_expired() {
			RewardTrackerStore::<T>::mutate(|reward_tracker_opt| match reward_tracker_opt {
				Some(reward_tracker) => {
					let now = T::Time::now();
					if now - reward_tracker.start >= reward_tracker.period {
						reward_tracker.start = now;
						reward_tracker.total_already_rewarded = Zero::zero();
					}
				},
				None => {},
			});
		}

		pub fn update_prices(block: T::BlockNumber) -> Weight {
			let mut total_weight: Weight = Zero::zero();
			let one_read = T::DbWeight::get().reads(1);
			for (asset_id, asset_info) in AssetsInfo::<T>::iter() {
				total_weight += one_read;
				if let Ok((removed_pre_prices_len, pre_prices)) =
					Self::update_pre_prices(asset_id, &asset_info, block)
				{
					total_weight += T::WeightInfo::update_pre_prices(removed_pre_prices_len as u32);
					let pre_prices_len = pre_prices.len();

					// We can ignore `Error::<T>::MaxHistory` because it can not be
					// because we control the length of items of `PriceHistory`.
					let _ = Self::update_price(asset_id, asset_info.clone(), block, pre_prices);
					total_weight += T::WeightInfo::update_price(pre_prices_len as u32);
				};
			}
			total_weight
		}

		#[allow(clippy::type_complexity)]
		pub fn update_pre_prices(
			asset_id: T::AssetId,
			asset_info: &AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
			block: T::BlockNumber,
		) -> Result<
			(usize, Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>),
			DispatchError,
		> {
			// TODO maybe add a check if price is requested, is less operations?
			let pre_pruned_prices = PrePrices::<T>::get(asset_id);
			let prev_pre_prices_len = pre_pruned_prices.len();
			let mut pre_prices = Vec::new();

			// There can convert pre_pruned_prices.len() to u32 safely
			// because pre_pruned_prices.len() limited by u32
			// (type of AssetsInfo::<T>::get(asset_id).max_answers).
			if pre_pruned_prices.len() as u32 >= asset_info.min_answers {
				let res =
					Self::prune_old_pre_prices(asset_info, pre_pruned_prices.into_inner(), block);
				let staled_prices = res.0;
				pre_prices = res.1;
				for p in staled_prices {
					Self::deposit_event(Event::AnswerPruned(p.who.clone(), p.price));
				}
				PrePrices::<T>::insert(
					asset_id,
					BoundedVec::try_from(pre_prices.clone())
						.map_err(|_| Error::<T>::MaxPrePrices)?,
				);
			}

			Ok((prev_pre_prices_len - pre_prices.len(), pre_prices))
		}

		#[transactional]
		pub fn update_price(
			asset_id: T::AssetId,
			asset_info: AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
			block: T::BlockNumber,
			pre_prices: Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
		) -> DispatchResult {
			// There can convert pre_prices.len() to u32 safely
			// because pre_prices.len() limited by u32
			// (type of AssetsInfo::<T>::get(asset_id).max_answers).
			if pre_prices.len() as u32 >= asset_info.min_answers {
				if let Some(price) = Self::calculate_price(&pre_prices, &asset_info) {
					let last_price = match pre_prices.last() {
						Some(pre_price) => pre_price.price,
						_ => Zero::zero(),
					};

					Prices::<T>::insert(asset_id, Price { price, block });
					PriceHistory::<T>::try_mutate(asset_id, |prices| -> DispatchResult {
						if prices.len() as u32 >= T::MaxHistory::get() {
							prices.remove(0);
						}
						if block != 0_u32.into() {
							prices
								.try_push(Price { price, block })
								.map_err(|_| Error::<T>::MaxHistory)?;
						}
						Ok(())
					})?;
					PrePrices::<T>::remove(asset_id);

					Self::handle_payout(&pre_prices, price, asset_id, &asset_info)?;

					// Emit `PriceChanged` event when prices have changed, if required.
					if price != last_price && asset_info.emit_price_changes {
						Self::deposit_event(Event::PriceChanged(asset_id, price));
					}
				}
			}
			Ok(())
		}

		#[allow(clippy::type_complexity)]
		pub fn prune_old_pre_prices(
			asset_info: &AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
			mut pre_prices: Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
			block: T::BlockNumber,
		) -> (
			Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
			Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
		) {
			let stale_block = block.saturating_sub(T::StalePrice::get());
			let (staled_prices, mut fresh_prices) =
				match pre_prices.iter().enumerate().find(|(_, p)| p.block >= stale_block) {
					Some((index, pre_price)) => {
						Self::remove_price_in_transit(&pre_price.who, asset_info);
						let fresh_prices = pre_prices.split_off(index);
						(pre_prices, fresh_prices)
					},
					None => (pre_prices, vec![]),
				};

			// check max answer
			let max_answers = asset_info.max_answers;
			if fresh_prices.len() as u32 > max_answers {
				let pruned = fresh_prices.len() - max_answers as usize;
				for price in fresh_prices.iter().skip(pruned) {
					Self::remove_price_in_transit(&price.who, asset_info);
				}
				#[allow(clippy::indexing_slicing)]
				// max_answers is confirmed to be less than the len in the condition of the if
				// block (in a block due to https://github.com/rust-lang/rust/issues/15701)
				{
					fresh_prices = fresh_prices[0..max_answers as usize].to_vec();
				};
			}

			(staled_prices, fresh_prices)
		}

		pub fn get_median_price(
			prices: &[PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>],
		) -> Option<T::PriceValue> {
			if prices.is_empty() {
				return None
			}

			let mut numbers: Vec<T::PriceValue> =
				prices.iter().map(|current_prices| current_prices.price).collect();

			numbers.sort_unstable();

			let mid = numbers.len() / 2;
			if numbers.len() % 2 == 0 {
				#[allow(clippy::indexing_slicing)] // mid is less than the len (len/2)
				Some(numbers[mid - 1].saturating_add(numbers[mid]) / 2_u32.into())
			} else {
				#[allow(clippy::indexing_slicing)] // mid is less than the len (len/2)
				Some(numbers[mid])
			}
		}

		pub fn calculate_price(
			prices: &[PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>],
			asset_info: &AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
		) -> Option<T::PriceValue> {
			let median_price = Self::get_median_price(prices)?;
			let mut sum_of_price = T::PriceValue::zero();
			let mut number_of_prices = 0_u32;
			for answer in prices {
				let accuracy: Percent = if answer.price < median_price {
					PerThing::from_rational(answer.price, median_price)
				} else {
					let adjusted_number = median_price.saturating_sub(answer.price - median_price);
					PerThing::from_rational(adjusted_number, median_price)
				};
				let min_accuracy = asset_info.threshold;
				// consider all prices which are with in threshold of median_price
				if accuracy >= min_accuracy {
					sum_of_price += answer.price;
					number_of_prices += 1;
				}
			}
			if number_of_prices == 0 {
				return None
			}
			Some(sum_of_price / number_of_prices.into())
		}

		pub fn check_requests() {
			for (i, asset_info) in AssetsInfo::<T>::iter() {
				if Self::is_requested(&i) {
					let _ = Self::fetch_price_and_send_signed(&i, asset_info);
				}
			}
		}

		pub fn is_requested(price_id: &T::AssetId) -> bool {
			let last_update = Self::prices(price_id);
			let current_block = frame_system::Pallet::<T>::block_number();
			let asset_info = Self::asset_info(price_id);
			if asset_info.is_none() {
				false
			} else {
				last_update.block + asset_info.unwrap_or_default().block_interval < current_block
			}
		}

		pub fn remove_price_in_transit(
			who: &T::AccountId,
			asset_info: &AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
		) {
			AnswerInTransit::<T>::mutate(who, |transit| {
				*transit = Some(transit.unwrap_or_else(Zero::zero).saturating_sub(asset_info.slash))
			});
		}

		pub fn get_twap(
			asset_id: T::AssetId,
			twap_window: usize,
		) -> Result<T::PriceValue, DispatchError> {
			let historical_prices = Self::price_history(asset_id);

			ensure!(twap_window <= historical_prices.len() + 1, Error::<T>::DepthTooLarge);

			let mut prices: Vec<_> =
				historical_prices.iter().rev().take(twap_window).rev().collect();
			let current_price =
				Prices::<T>::try_get(asset_id).map_err(|_| Error::<T>::PriceNotFound)?;
			prices.push(&current_price);

			let weights: Vec<u128> = prices
				.windows(2)
				.map(|window| {
					window
						.get(0)
						.and_then(|first| window.get(1).map(|second| (first, second)))
						.and_then(|(first, second)| second.block.checked_sub(&first.block))
						.unwrap_or_default()
						.unique_saturated_into()
				})
				.collect();

			let weights_sum: u128 = weights.iter().sum();

			let prices_sum: u128 = prices
				.iter()
				// skip 1 for diff calculation
				.skip(1)
				.map(|e| -> u128 { e.price.into() })
				.zip(weights)
				.filter_map(|(price, weight)| {
					if weights_sum.is_zero() {
						// if all weights is eq 0 then just return price.
						Some(price)
					} else if weight.is_zero() {
						// if weight is eq 0 then skip it.
						// it happens if few prices associated with same block number.
						None
					} else {
						// otherwise multiply price and weight.
						Some(price * weight)
					}
				})
				.sum();

			let twap = if weights_sum.is_zero() {
				prices_sum / ((prices.len() - 1) as u128)
			} else {
				prices_sum / weights_sum
			};

			Ok(twap.into())
		}

		fn quote(
			asset_id: T::AssetId,
			price: T::PriceValue,
			amount: T::PriceValue,
		) -> Result<T::PriceValue, DispatchError> {
			let unit = <Self as Oracle>::LocalAssets::unit(asset_id)?;
			let price = safe_multiply_by_rational(price.into(), amount.into(), unit)?;
			Ok(price.into())
		}

		// REVIEW: indexing
		pub fn fetch_price_and_send_signed(
			price_id: &T::AssetId,
			asset_info: AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
		) -> Result<(), &'static str> {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			if !signer.can_sign() {
				log::info!("no signer");
				return Err(
					"No local accounts available. Consider adding one via `author_insertKey` RPC.",
				)
			}
			// checks to make sure key from keystore has not already submitted price
			let prices = PrePrices::<T>::get(*price_id);
			let public_keys: Vec<sp_core::sr25519::Public> =
				sp_io::crypto::sr25519_public_keys(CRYPTO_KEY_TYPE);
			let account = AccountId32::new(
				public_keys.first().ok_or("No public keys for crypto key type `orac`")?.0,
			);
			let mut to32 = AccountId32::as_ref(&account);
			let address: T::AccountId =
				T::AccountId::decode(&mut to32).map_err(|_| "Could not decode account")?;

			if prices.len() as u32 >= asset_info.max_answers {
				log::info!("Max answers reached");
				return Err("Max answers reached")
			}

			if prices.into_iter().any(|price| price.who == address) {
				log::info!("Tx already submitted");
				return Err("Tx already submitted")
			}
			// Make an external HTTP request to fetch the current price.
			// Note this call will block until response is received.
			let price = Self::fetch_price(price_id).map_err(|_| "Failed to fetch price")?;
			log::info!("price {:#?}", price);

			// Using `send_signed_transaction` associated type we create and submit a transaction
			// representing the call, we've just created.
			// Submit signed will return a vector of results for all accounts that were found in the
			// local keystore with expected `KEY_TYPE`.

			let results = signer.send_signed_transaction(|_account| {
				// Received price is wrapped into a call to `submit_price` public function of this
				// pallet. This means that the transaction, when executed, will simply call that
				// function passing `price` as an argument.
				Call::submit_price { price: price.into(), asset_id: *price_id }
			});

			for (acc, res) in &results {
				match res {
					Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
					Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
				}
			}

			Ok(())
		}

		pub fn fetch_price(price_id: &T::AssetId) -> Result<u64, http::Error> {
			// We want to keep the offchain worker execution time reasonable, so we set a hard-coded
			// deadline to 2s to complete the external call.
			// You can also wait indefinitely for the response, however you may still get a timeout
			// coming from the host machine.
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));

			// Check if the node has another endpoint to call if not fall back to localhost:3001
			// Then build the endpoint
			let kind = sp_core::offchain::StorageKind::PERSISTENT;
			let from_local = sp_io::offchain::local_storage_get(kind, b"ocw-url")
				.unwrap_or_else(|| b"http://localhost:3001/price/".to_vec());
			let base = str::from_utf8(&from_local).unwrap_or("http://localhost:3001/price/");
			let string_id =
				serde_json::to_string(&(*price_id).into()).map_err(|_| http::Error::IoError)?;
			let url = base.to_owned() + &string_id;

			// Initiate an external HTTP GET request.
			let request = http::Request::get(&url);

			log::info!("request incoming {:#?}", request);

			// set the deadline for sending of the request
			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			// Let's check the status code before we proceed to reading the response.
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}

			let body = response.body().collect::<Vec<u8>>();

			// Create a str slice from the body.
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;

			let price = match Self::parse_price(body_str, &string_id) {
				Some(price) => Ok(price),
				None => {
					log::warn!("Unable to extract price from the response: {:?}", body_str);
					Err(http::Error::Unknown)
				},
			}?;

			log::warn!("Got price: {} cents", price);

			Ok(price)
		}

		pub fn parse_price(price_str: &str, asset_id: &str) -> Option<u64> {
			let val = lite_json::parse_json(price_str);
			let price = match val.ok()? {
				JsonValue::Object(obj) => {
					let (_, v) =
						obj.into_iter().find(|(k, _)| k.iter().copied().eq(asset_id.chars()))?;
					match v {
						JsonValue::Number(number) => number,
						_ => return None,
					}
				},
				_ => return None,
			};
			Some(price.integer as u64)
		}

		/// The AccountId of this pallet.
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}
