#![cfg_attr(not(feature = "std"), no_std)]

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
	use codec::{Codec, FullCodec};
	use composable_traits::oracle::Oracle;
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo, Vec},
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
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		offchain::{http, Duration},
		traits::{AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Saturating, Zero},
		AccountId32, KeyTypeId as CryptoKeyTypeId, PerThing, Percent, RuntimeDebug,
	};
	use sp_std::{borrow::ToOwned, fmt::Debug, str, vec};

	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"orac");
	pub const CRYPTO_KEY_TYPE: CryptoKeyTypeId = CryptoKeyTypeId(*b"orac");
	pub use crate::weights::WeightInfo;

	pub mod crypto {
		use super::KEY_TYPE;
		use sp_core::sr25519::Signature as Sr25519Signature;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			traits::Verify,
			MultiSignature, MultiSigner,
		};
		app_crypto!(sr25519, KEY_TYPE);

		pub struct TestAuthId;
		// implemented for ocw-runtime
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}

		impl
			frame_system::offchain::AppCrypto<
				<Sr25519Signature as Verify>::Signer,
				Sr25519Signature,
			> for TestAuthId
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
			+ Default;
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
		type MinStake: Get<BalanceOf<Self>>;
		type StakeLock: Get<Self::BlockNumber>;
		type StalePrice: Get<Self::BlockNumber>;
		type AddOracle: EnsureOrigin<Self::Origin>;
		type RequestCost: Get<BalanceOf<Self>>;
		type RewardAmount: Get<BalanceOf<Self>>;
		type SlashAmount: Get<BalanceOf<Self>>;
		type MaxAnswerBound: Get<u32>;
		type MaxAssetsCount: Get<u32>;
		/// The weight information of this pallet.
		type WeightInfo: WeightInfo;
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq)]
	pub struct Withdraw<Balance, BlockNumber> {
		pub stake: Balance,
		pub unlock_block: BlockNumber,
	}

	#[derive(Encode, Decode, Clone, Copy, Default, Debug, PartialEq)]
	pub struct PrePrice<PriceValue, BlockNumber, AccountId> {
		pub price: PriceValue,
		pub block: BlockNumber,
		pub who: AccountId,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq)]
	pub struct Price<PriceValue, BlockNumber> {
		pub price: PriceValue,
		pub block: BlockNumber,
	}

	#[derive(Encode, Decode, Default, Debug, PartialEq, Clone)]
	pub struct AssetInfo<Percent> {
		pub threshold: Percent,
		pub min_answers: u32,
		pub max_answers: u32,
	}

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn assets_count)]
	pub type AssetsCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn signer_to_controller)]
	pub type SignerToController<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn controller_to_signer)]
	pub type ControllerToSigner<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn declared_withdraws)]
	pub type DeclaredWithdraws<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Withdraw<BalanceOf<T>, T::BlockNumber>>;

	#[pallet::storage]
	#[pallet::getter(fn oracle_stake)]
	pub type OracleStake<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn prices)]
	pub type Prices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Price<T::PriceValue, T::BlockNumber>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pre_prices)]
	pub type PrePrices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AssetId,
		Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn accuracy_threshold)]
	pub type AssetsInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, AssetInfo<Percent>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn requested)]
	pub type Requested<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn request_id)]
	pub type RequestedId<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, u128, ValueQuery>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId",  BalanceOf<T> = "Balance", T::BlockNumber = "BlockNumber", Percent = "Percent")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewAsset(u128),
		AssetInfoChange(T::AssetId, Percent, u32, u32),
		PriceRequested(T::AccountId, T::AssetId),
		/// Who added it, the amount added and the total cumulative amount
		StakeAdded(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		StakeRemoved(T::AccountId, BalanceOf<T>, T::BlockNumber),
		StakeReclaimed(T::AccountId, BalanceOf<T>),
		PriceSubmitted(T::AccountId, T::AssetId, T::PriceValue),
		UserSlashed(T::AccountId, T::AssetId, BalanceOf<T>),
		UserRewarded(T::AccountId, T::AssetId, BalanceOf<T>),
		AnswerPruned(T::AccountId, T::PriceValue),
	}

	#[pallet::error]
	pub enum Error<T> {
		Unknown,
		NoPermission,
		NoStake,
		StakeLocked,
		NotEnoughStake,
		NotEnoughFunds,
		InvalidAssetId,
		AlreadySubmitted,
		MaxPrices,
		PriceNotRequested,
		UnsetSigner,
		AlreadySet,
		UnsetController,
		ControllerUsed,
		SignerUsed,
		AvoidPanic,
		ExceedMaxAnswers,
		InvalidMinAnswers,
		MaxAnswersLessThanMinAnswers,
		ExceedThreshold,
		ExceedAssetsCount,
		PriceNotFound,
		ExceedStake,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(block: T::BlockNumber) -> Weight {
			Self::update_prices(block)
		}

		fn offchain_worker(_block_number: T::BlockNumber) {
			log::info!("Hello World from offchain workers!");
			Self::check_requests();
		}
	}

	impl<T: Config> Oracle for Pallet<T> {
		type Balance = T::PriceValue;
		type AssetId = T::AssetId;
		type Timestamp = <T as frame_system::Config>::BlockNumber;

		fn get_price(
			of: &Self::AssetId,
		) -> Result<(Self::Balance, Self::Timestamp), DispatchError> {
			let price = Prices::<T>::try_get(of).map_err(|_| Error::<T>::PriceNotFound)?;
			Ok((price.price, price.block))
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_asset_and_info())]
		pub fn add_asset_and_info(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			threshold: Percent,
			min_answers: u32,
			max_answers: u32,
		) -> DispatchResultWithPostInfo {
			T::AddOracle::ensure_origin(origin)?;
			ensure!(min_answers > 0, Error::<T>::InvalidMinAnswers);
			ensure!(max_answers >= min_answers, Error::<T>::MaxAnswersLessThanMinAnswers);
			ensure!(threshold < Percent::from_percent(100), Error::<T>::ExceedThreshold);
			ensure!(max_answers <= T::MaxAnswerBound::get(), Error::<T>::ExceedMaxAnswers);
			ensure!(
				AssetsCount::<T>::get() < T::MaxAssetsCount::get(),
				Error::<T>::ExceedAssetsCount
			);
			let asset_info = AssetInfo { threshold, min_answers, max_answers };
			AssetsInfo::<T>::insert(asset_id, asset_info);
			AssetsCount::<T>::mutate(|a| *a += 1);
			Self::deposit_event(Event::AssetInfoChange(
				asset_id,
				threshold,
				min_answers,
				max_answers,
			));
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::request_price())]
		pub fn request_price(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
		) -> DispatchResultWithPostInfo {
			//TODO talk about the security and if this should be protected
			let who = ensure_signed(origin)?;
			Self::do_request_price(&who, asset_id)?;
			Ok(().into())
		}

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
			SignerToController::<T>::insert(signer, who);

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::add_stake())]
		pub fn add_stake(origin: OriginFor<T>, stake: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;

			Self::do_add_stake(who, signer, stake)?;

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::remove_stake())]
		pub fn remove_stake(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let signer = ControllerToSigner::<T>::get(&who).ok_or(Error::<T>::UnsetSigner)?;
			let block = frame_system::Pallet::<T>::block_number();
			let unlock_block = block + T::StakeLock::get(); //TODO check type of add
			let stake = Self::oracle_stake(signer.clone()).ok_or(Error::<T>::NoStake)?;
			let withdrawal = Withdraw { stake, unlock_block };
			OracleStake::<T>::remove(&signer);
			DeclaredWithdraws::<T>::insert(signer.clone(), withdrawal);
			Self::deposit_event(Event::StakeRemoved(signer, stake, unlock_block));
			Ok(().into())
		}

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

		#[pallet::weight((T::WeightInfo::submit_price(T::MaxAnswerBound::get()), Operational))]
		pub fn submit_price(
			origin: OriginFor<T>,
			price: T::PriceValue,
			asset_id: T::AssetId,
		) -> DispatchResultWithPostInfo {
			log::info!("inside submit {:#?}, {:#?}", asset_id, price);
			let who = ensure_signed(origin)?;
			let author_stake = OracleStake::<T>::get(&who).unwrap_or_else(Zero::zero);
			ensure!(Requested::<T>::get(asset_id), Error::<T>::PriceNotRequested);
			ensure!(author_stake >= T::MinStake::get(), Error::<T>::NotEnoughStake);

			let set_price = PrePrice {
				price,
				block: frame_system::Pallet::<T>::block_number(),
				who: who.clone(),
			};
			log::info!("inside submit 2 {:#?}, {:#?}", set_price, asset_id);
			PrePrices::<T>::try_mutate(asset_id, |current_prices| -> Result<(), DispatchError> {
				// There can convert current_prices.len() to u32 safely
				// because current_prices.len() limited by u32
				// (type of AssetsInfo::<T>::get(asset_id).max_answers).
				if current_prices.len() as u32 >= AssetsInfo::<T>::get(asset_id).max_answers {
					return Err(Error::<T>::MaxPrices.into());
				}
				if current_prices.iter().any(|candidate| candidate.who == who) {
					return Err(Error::<T>::AlreadySubmitted.into());
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
			// TODO only take prices up to max prices
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
					let slash_amount = T::SlashAmount::get();
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
					let reward_amount = T::RewardAmount::get();
					let controller = SignerToController::<T>::get(&answer.who)
						.unwrap_or_else(|| answer.who.clone());

					// TODO: since inlflationary, burn a portion of tx fees of the chain to account
					// for this
					let _ = T::Currency::deposit_into_existing(&controller, reward_amount);
					Self::deposit_event(Event::UserRewarded(
						answer.who.clone(),
						asset_id,
						reward_amount,
					));
				}
			}
		}

		pub fn do_request_price(who: &T::AccountId, asset_id: T::AssetId) -> DispatchResult {
			ensure!(AssetsInfo::<T>::contains_key(asset_id), Error::<T>::InvalidAssetId);
			if !Self::requested(asset_id) {
				ensure!(
					T::Currency::can_slash(who, T::RequestCost::get()),
					Error::<T>::NotEnoughFunds
				);
				T::Currency::slash(who, T::RequestCost::get());
				RequestedId::<T>::mutate(asset_id, |request_id| *request_id += 1);
				Requested::<T>::insert(asset_id, true);
			}
			Self::deposit_event(Event::PriceRequested(who.clone(), asset_id));
			Ok(())
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
			asset_info: AssetInfo<Percent>,
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
			asset_info: AssetInfo<Percent>,
			block: T::BlockNumber,
			pre_prices: Vec<PrePrice<T::PriceValue, T::BlockNumber, T::AccountId>>,
		) {
			// There can convert pre_prices.len() to u32 safely
			// because pre_prices.len() limited by u32
			// (type of AssetsInfo::<T>::get(asset_id).max_answers).
			if pre_prices.len() as u32 >= asset_info.min_answers {
				if let Some(price) = Self::get_median_price(&pre_prices) {
					Prices::<T>::insert(asset_id, Price { price, block });
					Requested::<T>::insert(asset_id, false);
					PrePrices::<T>::remove(asset_id);

					Self::handle_payout(&pre_prices, price, asset_id);
				}
			}
		}

		#[allow(clippy::type_complexity)]
		pub fn prune_old_pre_prices(
			asset_info: AssetInfo<Percent>,
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
					}
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
			let mut numbers: Vec<T::PriceValue> =
				prices.iter().map(|current_prices| current_prices.price).collect();
			numbers.sort();
			let mid = numbers.len() / 2;
			numbers.get(mid).cloned()
		}

		pub fn check_requests() {
			for (i, _) in AssetsInfo::<T>::iter() {
				if Requested::<T>::get(i) {
					let _ = Self::fetch_price_and_send_signed(&i);
				}
			}
		}

		pub fn fetch_price_and_send_signed(price_id: &T::AssetId) -> Result<(), &'static str> {
			let signer = Signer::<T, T::AuthorityId>::all_accounts();
			log::info!("signer");
			if !signer.can_sign() {
				log::info!("no signer");
				return Err(
					"No local accounts available. Consider adding one via `author_insertKey` RPC.",
				);
			}
			// Make an external HTTP request to fetch the current price.
			// Note this call will block until response is received.
			let prices = PrePrices::<T>::get(*price_id);
			let public_key = sp_io::crypto::sr25519_public_keys(CRYPTO_KEY_TYPE);
			let account = AccountId32::new(public_key[0].0);
			let mut to32 = AccountId32::as_ref(&account);
			let address: T::AccountId = T::AccountId::decode(&mut to32).unwrap_or_default();

			if prices.into_iter().any(|price| price.who == address) {
				return Err("Tx already submitted");
			}

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
				Call::submit_price(price.into(), *price_id)
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
			// Initiate an external HTTP GET request.
			// This is using high-level wrappers from `sp_runtime`, for the low-level calls that
			// you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
			// since we are running in a custom WASM execution environment we can't simply
			// import the library here.]

			let kind = sp_core::offchain::StorageKind::PERSISTENT;
			let from_local = sp_io::offchain::local_storage_get(kind, b"ocw-url")
				.unwrap_or_else(|| b"http://localhost:3001/price/".to_vec());
			let base = str::from_utf8(&from_local).unwrap_or("http://localhost:3001/price/");
			let string_id =
				serde_json::to_string(&(*price_id).into()).map_err(|_| http::Error::IoError)?;
			let request_id = RequestedId::<T>::get(price_id);
			let string_request_id =
				serde_json::to_string(&request_id).map_err(|_| http::Error::IoError)?;
			let url = base.to_owned() + &string_id + "/" + &string_request_id;
			let request = http::Request::get(&url);

			log::info!("request incoming {:#?}", request);

			// We set the deadline for sending of the request, note that awaiting response can
			// have a separate deadline. Next we send the request, before that it's also possible
			// to alter request headers or stream body content in case of non-GET requests.
			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

			// The request is already being processed by the host, we are free to do anything
			// else in the worker (we can send multiple concurrent requests too).
			// At some point however we probably want to check the response though,
			// so we can block current thread and wait for it to finish.
			// Note that since the request is being driven by the host, we don't have to wait
			// for the request to have it complete, we will just not read the response.
			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			// Let's check the status code before we proceed to reading the response.
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown);
			}

			// Next we want to fully read the response body and collect it to a vector of bytes.
			// Note that the return object allows you to read the body in chunks as well
			// with a way to control the deadline.
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
				}
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
				}
				_ => return None,
			};
			Some(price.integer as u64)
		}
	}
}
