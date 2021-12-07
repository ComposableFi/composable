#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use crate::weights::WeightInfo;
	use codec::{Codec, FullCodec};
	use composable_traits::oracle::{Oracle, Price as LastPrice};
	use core::ops::{Div, Mul};
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo},
		pallet_prelude::*,
		traits::{
			Currency, EnsureOrigin,
			ExistenceRequirement::{AllowDeath, KeepAlive},
			ReservableCurrency,
		},
		weights::{DispatchClass::Operational, Pays},
	};
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendSignedTransaction, SignedPayload, Signer,
			SigningTypes,
		},
		pallet_prelude::*,
		Config as SystemConfig,
	};
	use lite_json::json::JsonValue;
	use scale_info::TypeInfo;
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		offchain::{http, Duration},
		traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Saturating, Zero},
		AccountId32, KeyTypeId as CryptoKeyTypeId, PerThing, Percent, RuntimeDebug,
	};
	use sp_std::{borrow::ToOwned, fmt::Debug, str, vec, vec::Vec};

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
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;
		type AssetId: FullCodec
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
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
			+ Zero;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// The Min stake for an oracle
		type MinStake: Get<BalanceOf<Self>>;
		/// The delay to withdraw stake as an oracle
		type StakeLock: Get<Self::BlockNumber>;
		/// Blocks until price is considered stale
		type StalePrice: Get<Self::BlockNumber>;
		/// Origin to add new price types
		type AddOracle: EnsureOrigin<Self::Origin>;
		/// Slash for an incorrect answer
		type SlashAmount: Get<BalanceOf<Self>>;
		/// Upper bound for max answers for a price
		type MaxAnswerBound: Get<u32>;
		/// Upper bound for total assets available for the oracle
		type MaxAssetsCount: Get<u32>;

		type MaxHistory: Get<u32>;

		/// The weight information of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
	pub struct Withdraw<Balance, BlockNumber> {
		pub stake: Balance,
		pub unlock_block: BlockNumber,
	}

	#[derive(Encode, Decode, Clone, Copy, Default, Debug, PartialEq, TypeInfo)]
	pub struct PrePrice<PriceValue, BlockNumber, AccountId> {
		pub price: PriceValue,
		pub block: BlockNumber,
		pub who: AccountId,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, TypeInfo, Clone)]
	pub struct Price<PriceValue, BlockNumber> {
		pub price: PriceValue,
		pub block: BlockNumber,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone, TypeInfo)]
	pub struct AssetInfo<Percent, BlockNumber, Balance> {
		pub threshold: Percent,
		pub min_answers: u32,
		pub max_answers: u32,
		pub block_interval: BlockNumber,
		pub reward: Balance,
		pub slash: Balance,
	}

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn assets_count)]
	/// Total amount of assets
	pub type AssetsCount<T: Config> = StorageValue<_, u32, ValueQuery>;

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
	/// Tracking withdrawl requests
	pub type DeclaredWithdraws<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Withdraw<BalanceOf<T>, T::BlockNumber>>;

	#[pallet::storage]
	#[pallet::getter(fn oracle_stake)]
	/// Mapping of signing key to stake
	pub type OracleStake<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn prices)]
	/// Price for an asset and blocknumber asset was updated at
	pub type Prices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Price<T::PriceValue, T::BlockNumber>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn price_history)]
	/// Price for an asset and blocknumber asset was updated at
	pub type PriceHistory<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<Price<T::PriceValue, T::BlockNumber>>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pre_prices)]
	/// Temporary prices before aggregated
	pub type PrePrices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
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
		ValueQuery,
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
		UserRewarded(T::AccountId, T::AssetId, BalanceOf<T>),
		/// Answer from oracle removed for staleness. \[oracle_address, price\]
		AnswerPruned(T::AccountId, T::PriceValue),
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
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block: T::BlockNumber) -> Weight {
			Self::update_prices(block)
		}

		fn offchain_worker(_block_number: T::BlockNumber) {
			log::info!("Offchain worker triggered");
			Self::check_requests();
		}
	}

	impl<T: Config> Oracle for Pallet<T> {
		type Balance = T::PriceValue;
		type AssetId = T::AssetId;
		type Timestamp = <T as frame_system::Config>::BlockNumber;

		fn get_price(
			of: Self::AssetId,
		) -> Result<LastPrice<Self::Balance, Self::Timestamp>, DispatchError> {
			let Price { price, block } =
				Prices::<T>::try_get(of).map_err(|_| Error::<T>::PriceNotFound)?;
			Ok(LastPrice { price, block })
		}

		fn get_twap(
			of: Self::AssetId,
			weighting: Vec<Self::Balance>,
		) -> Result<Self::Balance, DispatchError> {
			Self::get_twap(of, weighting)
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
		///
		/// Emits `DepositEvent` event when successful.
		#[pallet::weight(T::WeightInfo::add_asset_and_info())]
		pub fn add_asset_and_info(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			threshold: Percent,
			min_answers: u32,
			max_answers: u32,
			block_interval: T::BlockNumber,
			reward: BalanceOf<T>,
			slash: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::AddOracle::ensure_origin(origin)?;
			ensure!(min_answers > 0, Error::<T>::InvalidMinAnswers);
			ensure!(max_answers >= min_answers, Error::<T>::MaxAnswersLessThanMinAnswers);
			ensure!(threshold < Percent::from_percent(100), Error::<T>::ExceedThreshold);
			ensure!(max_answers <= T::MaxAnswerBound::get(), Error::<T>::ExceedMaxAnswers);
			ensure!(block_interval > T::StalePrice::get(), Error::<T>::BlockIntervalLength);
			ensure!(
				AssetsCount::<T>::get() < T::MaxAssetsCount::get(),
				Error::<T>::ExceedAssetsCount
			);
			let asset_info =
				AssetInfo { threshold, min_answers, max_answers, block_interval, reward, slash };
			AssetsInfo::<T>::insert(asset_id, asset_info);
			AssetsCount::<T>::mutate(|a| *a += 1);
			Self::deposit_event(Event::AssetInfoChange(
				asset_id,
				threshold,
				min_answers,
				max_answers,
				block_interval,
				reward,
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

			ensure!(current_controller == None, Error::<T>::ControllerUsed);
			ensure!(current_signer == None, Error::<T>::SignerUsed);

			Self::do_add_stake(who.clone(), signer.clone(), T::MinStake::get())?;

			ControllerToSigner::<T>::insert(&who, signer.clone());
			SignerToController::<T>::insert(signer.clone(), who.clone());

			Self::deposit_event(Event::SignerSet(signer, who));
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
			let _ = T::Currency::transfer(&signer, &who, withdrawal.stake, AllowDeath);

			ControllerToSigner::<T>::remove(&who);
			SignerToController::<T>::remove(&signer);

			Self::deposit_event(Event::StakeReclaimed(signer, withdrawal.stake));
			Ok(().into())
		}
		/// Call to submit a price, gas is returned if all logic gates passed
		/// Should be called from offchain worker but can be called manually too
		/// Operational transaction
		///
		/// - `price`: price to submit
		/// - `asset_id`: Id for the asset
		///
		/// Emits `PriceSubmitted` event when successful.
		#[pallet::weight((T::WeightInfo::submit_price(T::MaxAnswerBound::get()), Operational))]
		pub fn submit_price(
			origin: OriginFor<T>,
			price: T::PriceValue,
			asset_id: T::AssetId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let author_stake = OracleStake::<T>::get(&who).unwrap_or_else(Zero::zero);
			ensure!(Self::is_requested(&asset_id), Error::<T>::PriceNotRequested);
			ensure!(author_stake >= T::MinStake::get(), Error::<T>::NotEnoughStake);

			let set_price = PrePrice {
				price,
				block: frame_system::Pallet::<T>::block_number(),
				who: who.clone(),
			};
			PrePrices::<T>::try_mutate(asset_id, |current_prices| -> Result<(), DispatchError> {
				// There can convert current_prices.len() to u32 safely
				// because current_prices.len() limited by u32
				// (type of AssetsInfo::<T>::get(asset_id).max_answers).
				if current_prices.len() as u32 >= AssetsInfo::<T>::get(asset_id).max_answers {
					return Err(Error::<T>::MaxPrices.into())
				}
				if current_prices.iter().any(|candidate| candidate.who == who) {
					return Err(Error::<T>::AlreadySubmitted.into())
				}
				current_prices.push(set_price);
				Ok(())
			})?;
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
				.unwrap_or_else(|| 0u32.into())
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
		) {
			let asset_info = Self::asset_info(asset_id);
			for answer in pre_prices {
				let accuracy: Percent;
				if answer.price < price {
					accuracy = PerThing::from_rational(answer.price, price);
				} else {
					let adjusted_number = price.saturating_sub(answer.price - price);
					accuracy = PerThing::from_rational(adjusted_number, price);
				}
				let min_accuracy = AssetsInfo::<T>::get(asset_id).threshold;
				if accuracy < min_accuracy {
					let slash_amount = asset_info.slash;
					let try_slash = T::Currency::can_slash(&answer.who, slash_amount);
					if !try_slash {
						log::warn!("Failed to slash {:?}", answer.who);
					}
					T::Currency::slash(&answer.who, slash_amount);
					Self::deposit_event(Event::UserSlashed(
						answer.who.clone(),
						asset_id,
						slash_amount,
					));
				} else {
					let reward_amount = asset_info.reward;
					let controller = SignerToController::<T>::get(&answer.who)
						.unwrap_or_else(|| answer.who.clone());

					let _ = T::Currency::deposit_into_existing(&controller, reward_amount);
					Self::deposit_event(Event::UserRewarded(
						answer.who.clone(),
						asset_id,
						reward_amount,
					));
				}
			}
		}

		pub fn update_prices(block: T::BlockNumber) -> Weight {
			let mut total_weight: Weight = Zero::zero();
			let one_read = T::DbWeight::get().reads(1);
			for (asset_id, asset_info) in AssetsInfo::<T>::iter() {
				total_weight += one_read;
				let (removed_pre_prices_len, pre_prices) =
					Self::update_pre_prices(asset_id, asset_info.clone(), block);
				total_weight += T::WeightInfo::update_pre_prices(removed_pre_prices_len as u32);
				let pre_prices_len = pre_prices.len();
				Self::update_price(asset_id, asset_info.clone(), block, pre_prices);
				total_weight += T::WeightInfo::update_price(pre_prices_len as u32);
			}
			total_weight
		}

		#[allow(clippy::type_complexity)]
		pub fn update_pre_prices(
			asset_id: T::AssetId,
			asset_info: AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
			block: T::BlockNumber,
		) -> (usize, Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>) {
			// TODO maybe add a check if price is requested, is less operations?
			let pre_pruned_prices = PrePrices::<T>::get(asset_id);
			let prev_pre_prices_len = pre_pruned_prices.len();
			let mut pre_prices = Vec::new();

			// There can convert pre_pruned_prices.len() to u32 safely
			// because pre_pruned_prices.len() limited by u32
			// (type of AssetsInfo::<T>::get(asset_id).max_answers).
			if pre_pruned_prices.len() as u32 >= asset_info.min_answers {
				let res = Self::prune_old_pre_prices(asset_info, pre_pruned_prices, block);
				let staled_prices = res.0;
				pre_prices = res.1;
				for p in staled_prices {
					Self::deposit_event(Event::AnswerPruned(p.who.clone(), p.price));
				}
				PrePrices::<T>::insert(asset_id, pre_prices.clone());
			}

			(prev_pre_prices_len - pre_prices.len(), pre_prices)
		}

		pub fn update_price(
			asset_id: T::AssetId,
			asset_info: AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
			block: T::BlockNumber,
			pre_prices: Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
		) {
			// There can convert pre_prices.len() to u32 safely
			// because pre_prices.len() limited by u32
			// (type of AssetsInfo::<T>::get(asset_id).max_answers).
			if pre_prices.len() as u32 >= asset_info.min_answers {
				if let Some(price) = Self::get_median_price(&pre_prices) {
					Prices::<T>::insert(asset_id, Price { price, block });
					let historical = Self::price_history(asset_id);
					if (historical.len() as u32) < T::MaxHistory::get() {
						PriceHistory::<T>::mutate(asset_id, |prices| {
							if Self::prices(asset_id).block != 0u32.into() {
								prices.push(Price { price, block });
							}
						})
					} else {
						PriceHistory::<T>::mutate(asset_id, |prices| {
							prices.remove(0);
							prices.push(Price { price, block });
						})
					}
					PrePrices::<T>::remove(asset_id);

					Self::handle_payout(&pre_prices, price, asset_id);
				}
			}
		}

		#[allow(clippy::type_complexity)]
		pub fn prune_old_pre_prices(
			asset_info: AssetInfo<Percent, T::BlockNumber, BalanceOf<T>>,
			mut pre_prices: Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
			block: T::BlockNumber,
		) -> (
			Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
			Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
		) {
			let stale_block = block.saturating_sub(T::StalePrice::get());
			let (staled_prices, mut fresh_prices) =
				match pre_prices.iter().position(|p| p.block >= stale_block) {
					Some(index) => {
						let fresh_prices = pre_prices.split_off(index);
						(pre_prices, fresh_prices)
					},
					None => (pre_prices, vec![]),
				};

			// check max answer
			if fresh_prices.len() as u32 > asset_info.max_answers {
				fresh_prices = fresh_prices[0..asset_info.max_answers as usize].to_vec();
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
				Some(numbers[mid - 1].saturating_add(numbers[mid]) / 2u32.into())
			} else {
				Some(numbers[mid])
			}
		}

		pub fn check_requests() {
			for (i, _) in AssetsInfo::<T>::iter() {
				if Self::is_requested(&i) {
					let _ = Self::fetch_price_and_send_signed(&i);
				}
			}
		}

		pub fn is_requested(price_id: &T::AssetId) -> bool {
			let last_update = Self::prices(price_id);
			let current_block = frame_system::Pallet::<T>::block_number();
			let asset_info = Self::asset_info(price_id);
			last_update.block + asset_info.block_interval < current_block
		}

		pub fn get_twap(
			asset_id: T::AssetId,
			mut price_weights: Vec<T::PriceValue>,
		) -> Result<T::PriceValue, DispatchError> {
			let precision: T::PriceValue = 100u128.into();
			let historical_prices = Self::price_history(asset_id);
			// add an extra to account for current price not stored in history
			ensure!(historical_prices.len() + 1 >= price_weights.len(), Error::<T>::DepthTooLarge);
			let sum = Self::price_values_sum(&price_weights);
			ensure!(sum == precision, Error::<T>::MustSumTo100);
			let last_weight = price_weights.pop().unwrap_or_else(|| 0u128.into());
			ensure!(last_weight != 0u128.into(), Error::<T>::ArithmeticError);
			let mut weighted_prices = price_weights
				.iter()
				.enumerate()
				.map(|(i, weight)| {
					weight
						.mul(
							historical_prices[historical_prices.len() - price_weights.len() + i]
								.price,
						)
						.div(precision)
				})
				.collect::<Vec<_>>();
			let current_price = Self::prices(asset_id);
			let current_weighted_price = last_weight.mul(current_price.price).div(precision);
			weighted_prices.push(current_weighted_price);
			let weighted_average = Self::price_values_sum(&weighted_prices);
			ensure!(weighted_average != 0u128.into(), Error::<T>::ArithmeticError);
			Ok(weighted_average)
		}

		fn price_values_sum(price_values: &[T::PriceValue]) -> T::PriceValue {
			price_values
				.iter()
				.fold(T::PriceValue::from(0u128), |acc, b| acc.saturating_add(*b))
		}

		pub fn fetch_price_and_send_signed(price_id: &T::AssetId) -> Result<(), &'static str> {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			if !signer.can_sign() {
				log::info!("no signer");
				return Err(
					"No local accounts available. Consider adding one via `author_insertKey` RPC.",
				)
			}
			// checks to make sure key from keystore has not already submitted price
			let prices = PrePrices::<T>::get(*price_id);
			let public_key = sp_io::crypto::sr25519_public_keys(CRYPTO_KEY_TYPE);
			let account = AccountId32::new(public_key[0].0);
			let mut to32 = AccountId32::as_ref(&account);
			let address: T::AccountId = T::AccountId::decode(&mut to32).unwrap_or_default();

			if prices.into_iter().any(|price| price.who == address) {
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
			// You can also wait idefinitely for the response, however you may still get a timeout
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
	}
}
